# Particle Effects Design: Bullet Trails and Impact Effects

**Date:** 2025-10-27
**Status:** Design Approved
**Implementation:** Pending

## Overview

Add GPU-accelerated particle effects to the bullet system using bevy_hanabi. This design maintains the existing sprite-based bullet collision system while adding visual polish through particle trails and impact effects.

## Goals

- **Primary:** Add bullet trails and impact burst effects for visual feedback
- **Visual Style:** Mixed approach - pixel-style sprite bullets with smooth particle effects
- **Architecture:** Hybrid system keeping sprite bullets for collision, particles for visuals only
- **Performance:** Leverage GPU compute shaders to handle thousands of particles efficiently

## Non-Goals

- Particle-based collision detection (bullets handle all collision logic)
- Replacing sprite bullets entirely
- CPU-based particle simulation

## Design Decisions

### Architecture: Hybrid Bullet + Particle System

**Keep existing sprite bullet system:**
- Bullet entities remain unchanged (`Bullet`, `BulletDirection`, `SpawnInstant` components)
- Collision detection stays in `collision.rs` using KD-tree
- Sprite bullets are the authoritative source for game logic

**Add particle effects as visual layer:**
- Particle emitters spawn as child entities of bullets
- Trails follow bullet movement via parent-child transform hierarchy
- Impact effects spawn at collision points
- Particles are purely decorative (no gameplay logic)

**Benefits:**
- Minimal changes to existing gun.rs and collision.rs
- Easy to debug (toggle particles on/off)
- Collision code doesn't need particle awareness
- GPU particles scale to thousands without performance impact

### Technology Choice: bevy_hanabi

**Selected:** `bevy_hanabi` - GPU-based particle system with compute shaders

**Rationale:**
- Best performance for large particle counts (supports 100k+ particles)
- Single draw call per effect type (all bullet trails batched together)
- High visual quality with smooth gradients and blending
- Scales well with 10k enemy spawn rate and multiple simultaneous shots

**Alternatives considered:**
- `bevy_particle_systems` (CPU-based) - simpler but doesn't scale as well
- `bevy_enoki` (particle editor) - less documentation, newer
- Custom system - reinventing the wheel, unnecessary complexity

### Lighting and Future Extensions

**Current design:**
- Particles use emissive (self-lit) rendering
- No collision detection on particles themselves
- Bullet sprites handle all damage and collision

**Future extensibility:**
- Dynamic lighting: Can add point lights at impact/muzzle flash positions
- Particles can receive scene lighting if needed (material configuration)
- No changes needed to particle system for lighting additions

## Component Structure

### New Components

```rust
// Marker for bullet trail particle emitters
#[derive(Component)]
pub struct BulletTrailEmitter;

// Impact particle effects with lifetime tracking
#[derive(Component)]
pub struct ImpactEffect {
    pub lifetime: Timer,
}

// Optional: Point light for impact flash
#[derive(Component)]
pub struct ImpactLight(pub Timer);
```

### New Resource

```rust
#[derive(Resource)]
pub struct ParticleEffectAssets {
    pub bullet_trail: Handle<EffectAsset>,
    pub impact_burst: Handle<EffectAsset>,
}
```

## Particle Effect Specifications

### Bullet Trail Effect

**Behavior:** Continuous emission following bullet
- **Emission rate:** 200-500 particles/second
- **Particle size:** 2-4 pixels
- **Lifetime:** 0.2-0.3 seconds (creates short trail)
- **Color gradient:** Bright color → fade to transparent over lifetime
- **Velocity:** Inherit bullet velocity + slight random spread
- **Blending:** Additive (creates glow effect)

**Visual result:** Glowing streak behind each bullet that fades quickly

### Impact Burst Effect

**Behavior:** One-shot radial explosion on collision
- **Spawn count:** 20-50 particles instantly
- **Particle size:** 2-4 pixels, slight growth over lifetime
- **Lifetime:** 0.3-0.5 seconds
- **Color:** Match bullet trail, slightly brighter
- **Velocity:** Radiate outward from impact point with random spread
- **Optional:** Slight upward bias for "pop" effect
- **Blending:** Additive

**Visual result:** Quick flash/pop when bullet hits enemy

## Data Flow

### 1. Bullet Spawn (gun.rs)

**Current code location:** `handle_gun_input()` line 114-128

**Modification:**
```rust
commands.spawn((
    // ... existing bullet components ...
))
.with_children(|parent| {
    // NEW: Spawn trail emitter as child
    parent.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(particle_assets.bullet_trail.clone()),
            transform: Transform::default(), // Inherits parent transform
            ..default()
        },
        BulletTrailEmitter,
    ));
});
```

**Result:** Bullet sprite + trail emitter move together automatically

### 2. Bullet Movement (gun.rs)

**Current code location:** `update_bullets()` line 133-145

**No changes needed:** Parent-child hierarchy automatically propagates transform updates to trail emitter

### 3. Bullet Collision (collision.rs)

**Current code location:** `handle_enemy_bullet_collision()` line 75-95

**Modification:** Add impact effect spawn before bullet despawn
```rust
for e in enemies {
    if let Ok((transform, mut enemy)) = enemy_query.get_mut(e.entity) {
        enemy.health -= config.gun.bullet_damage;

        // NEW: Spawn impact effect at collision point
        if let Ok(bullet_transform) = bullet_query.get(bullet_entity) {
            commands.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(particle_assets.impact_burst.clone()),
                    transform: Transform::from_translation(bullet_transform.translation),
                    ..default()
                },
                ImpactEffect {
                    lifetime: Timer::from_seconds(0.5, TimerMode::Once),
                },
            ));
        }
    }
}
```

### 4. Cleanup

**Trail emitters:** Despawn automatically with parent bullet (Bevy hierarchy)

**Impact effects:** New system despawns after lifetime expires
```rust
fn despawn_finished_impacts(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ImpactEffect)>,
) {
    for (entity, mut impact) in query.iter_mut() {
        impact.lifetime.tick(time.delta());
        if impact.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
```

## Module Structure

### New file: `src/particle_effects.rs`

```rust
pub struct ParticleEffectsPlugin;

impl Plugin for ParticleEffectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Loading), setup_particle_assets)
            .add_systems(Update, despawn_finished_impacts.run_if(in_state(GameState::InGame)));
    }
}

fn setup_particle_assets(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    // Create bullet trail effect
    let bullet_trail = effects.add(/* EffectAsset configuration */);

    // Create impact burst effect
    let impact_burst = effects.add(/* EffectAsset configuration */);

    commands.insert_resource(ParticleEffectAssets {
        bullet_trail,
        impact_burst,
    });
}
```

### Modified files:
- `src/lib.rs` - Add `pub mod particle_effects;`
- `src/main.rs` - Add `HanabiPlugin` and `ParticleEffectsPlugin`
- `src/gun.rs` - Modify `handle_gun_input()` to spawn trail emitters
- `src/collision.rs` - Modify `handle_enemy_bullet_collision()` to spawn impact effects
- `Cargo.toml` - Add `bevy_hanabi = "0.17"`

## Performance Considerations

**Particle counts:**
- 10 bullets/shot × 500 particles/trail = 5,000 active trail particles
- 100 impact bursts/sec × 30 particles = 3,000 impact particles
- **Total:** ~8,000 particles active simultaneously (trivial for GPU)

**Rendering:**
- 1 draw call for all bullet trails (batched)
- 1 draw call for all impact effects (batched)
- GPU compute shader handles simulation (zero CPU cost)

**Memory:**
- EffectAsset templates loaded once (2 assets total)
- Minimal per-particle memory (position, velocity, color, lifetime)

## Configuration

Add to `config.toml`:

```toml
[particle_effects]
bullet_trail_emission_rate = 300
bullet_trail_lifetime = 0.25
bullet_trail_size = 3.0

impact_particle_count = 30
impact_lifetime = 0.4
impact_size = 3.0
```

## Testing Plan

1. **Visual verification:**
   - Trails visible behind bullets
   - Impacts appear on enemy hits
   - No visual glitches or flickering

2. **Performance testing:**
   - Measure FPS with 10k enemies + bullets + particles
   - Verify 60 FPS maintained on target hardware
   - Profile GPU usage (should be minimal)

3. **Cleanup verification:**
   - No particle entities leak after bullets despawn
   - Impact effects despawn after lifetime
   - No memory growth over time

## Future Enhancements (Out of Scope)

- Muzzle flash particles at gun barrel
- Smoke/steam continuous emission from gun
- Particle colors based on bullet type/damage
- Dynamic point lights for impact flashes
- Screen shake on impact (non-particle feature)

## References

- [bevy_hanabi documentation](https://docs.rs/bevy_hanabi)
- [Bevy 0.17.2 hierarchy system](https://docs.rs/bevy/0.17.2/bevy/hierarchy/)
- Current gun implementation: `src/gun.rs`
- Current collision system: `src/collision.rs`
