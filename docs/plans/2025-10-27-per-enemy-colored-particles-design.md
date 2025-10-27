# Per-Enemy Colored Particle Effects Design

**Date:** 2025-10-27
**Status:** Design Approved
**Implementation:** Pending

## Overview

Extend the existing particle effects system to use enemy-specific colors for hit impacts and death explosions. This design uses an effect asset pooling strategy to balance visual variety with GPU performance.

## Goals

- **Visual Feedback:** Hit impacts and death explosions match each enemy's random tint color
- **Color Variety:** Each enemy's particles feel unique despite using a shared palette
- **Performance:** Maintain GPU batching efficiency with minimal memory overhead
- **Death Polish:** Death explosions more satisfying than hit impacts (larger + secondary lingering effect)

## Non-Goals

- Perfect per-enemy color matching (8-color palette provides perceived variety)
- Particle colors for bullet trails (bullets remain cyan, unchanged)
- Enemy movement trails (only death and hit effects)

## Design Decisions

### Architecture: Effect Asset Pooling

**Core Strategy:**
Pre-generate 8-12 EffectAssets with different color gradients covering the color wheel. When an enemy is hit or dies, find the closest matching color variant and spawn that effect.

**Why pooling over alternatives:**
- **vs Runtime gradient modification:** Simpler implementation, no need to access effect internals
- **vs Custom particle attributes:** Less complex setup, good enough batching for 8 variants
- **vs Single shared effect:** Provides visual variety matching enemy colors

**Benefits:**
- GPU batching: All enemies using "red variant" batch together
- Memory efficient: 18 EffectAssets total vs thousands (one per enemy)
- Perceived variety: 8-12 colors appear "per-enemy" to human perception
- Simple implementation: Just color distance matching, no runtime manipulation

### Color Palette

**8-color palette evenly spaced on color wheel:**
- Red (0°)
- Orange (45°)
- Yellow (90°)
- Green (135°)
- Cyan (180°)
- Blue (225°)
- Purple (270°)
- Magenta (315°)

**Coverage:** Any random enemy color within ~22° of a palette color

**Gradient structure for each variant:**
```
Key 0.0: Base color (full saturation/brightness)
Key 0.5: White (flash effect)
Key 1.0: Base color at 0 alpha (fade to transparent)
```

Example: Red variant = `rgb(1.0, 0.0, 0.0)` → `rgb(1.0, 1.0, 1.0)` → `rgba(1.0, 0.0, 0.0, 0.0)`

### Hit Impact vs Death Explosion

**Hit Impact (on bullet collision):**
- Particle count: 30
- Lifetime: 0.4s
- Size: 4.0
- Velocity: 50.0 (radial spread)
- Color: Closest match to enemy color from `impact_variants` pool

**Death Explosion (when enemy health <= 0):**

*Primary burst:*
- Particle count: 60 (2× hit impact)
- Lifetime: 0.4s
- Size: 4.0
- Velocity: 100.0 (spreads farther)
- Color: Closest match to enemy color from `death_burst_variants` pool

*Secondary lingering effect:*
- Particle count: 25
- Lifetime: 1.2s
- Size: 2.0
- Velocity: 20.0 (slow upward drift)
- Color: White/gray gradient (shared effect, no color variation)
- Visual: "Smoke rising" after explosion

## Component Structure

### Modified Resource

```rust
#[derive(Resource)]
pub struct ParticleEffectAssets {
    pub bullet_trail: Handle<EffectAsset>, // Unchanged

    // NEW: Colored hit impact variants
    pub impact_variants: Vec<(Color, Handle<EffectAsset>)>,

    // NEW: Colored death burst variants
    pub death_burst_variants: Vec<(Color, Handle<EffectAsset>)>,

    // NEW: Shared lingering death effect
    pub death_linger: Handle<EffectAsset>,
}
```

### New Component

```rust
#[derive(Component)]
pub struct DeathLingerEffect {
    pub lifetime: Timer,
}
```

### Existing Components (unchanged)
- `ImpactEffect` - Used for both hit impacts and death bursts
- `BulletTrailEmitter` - Unchanged

## Data Flow

### 1. Asset Setup (src/particle_effects.rs)

**Location:** `setup_particle_assets()` system (OnEnter GameState::Loading)

**Process:**
```rust
fn setup_particle_assets(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    config: Res<GameConfig>,
) {
    // Define 8-color palette
    let palette = vec![
        ("Red", Color::srgb(1.0, 0.0, 0.0)),
        ("Orange", Color::srgb(1.0, 0.5, 0.0)),
        ("Yellow", Color::srgb(1.0, 1.0, 0.0)),
        ("Green", Color::srgb(0.0, 1.0, 0.0)),
        ("Cyan", Color::srgb(0.0, 1.0, 1.0)),
        ("Blue", Color::srgb(0.0, 0.0, 1.0)),
        ("Purple", Color::srgb(0.5, 0.0, 1.0)),
        ("Magenta", Color::srgb(1.0, 0.0, 1.0)),
    ];

    // Generate hit impact variants
    let mut impact_variants = Vec::new();
    for (name, base_color) in palette.iter() {
        let gradient = create_color_gradient(*base_color);
        let effect = effects.add(create_impact_effect(gradient, config));
        impact_variants.push((*base_color, effect));
    }

    // Generate death burst variants (same colors, different particle count/velocity)
    let mut death_burst_variants = Vec::new();
    for (name, base_color) in palette.iter() {
        let gradient = create_color_gradient(*base_color);
        let effect = effects.add(create_death_burst_effect(gradient, config));
        death_burst_variants.push((*base_color, effect));
    }

    // Create shared lingering effect (white/gray)
    let death_linger = effects.add(create_death_linger_effect(config));

    // Existing bullet trail creation...

    commands.insert_resource(ParticleEffectAssets {
        bullet_trail,
        impact_variants,
        death_burst_variants,
        death_linger,
    });
}
```

**Helper function:**
```rust
fn create_color_gradient(base_color: Color) -> bevy_hanabi::Gradient {
    let mut gradient = bevy_hanabi::Gradient::new();

    // Start: Base color (vibrant)
    gradient.add_key(0.0, Vec4::new(
        base_color.r(),
        base_color.g(),
        base_color.b(),
        1.0
    ));

    // Middle: White flash
    gradient.add_key(0.5, Vec4::new(1.0, 1.0, 1.0, 1.0));

    // End: Base color fading to transparent
    gradient.add_key(1.0, Vec4::new(
        base_color.r(),
        base_color.g(),
        base_color.b(),
        0.0
    ));

    gradient
}
```

### 2. Color Matching (src/particle_effects.rs)

**New function:**
```rust
pub fn find_closest_effect_variant(
    enemy_color: Color,
    variants: &Vec<(Color, Handle<EffectAsset>)>,
) -> Handle<EffectAsset> {
    variants
        .iter()
        .min_by_key(|(palette_color, _)| {
            // Simple RGB distance
            let dr = (palette_color.r() - enemy_color.r()).abs();
            let dg = (palette_color.g() - enemy_color.g()).abs();
            let db = (palette_color.b() - enemy_color.b()).abs();
            ((dr + dg + db) * 1000.0) as u32
        })
        .map(|(_, handle)| handle.clone())
        .unwrap()
}
```

### 3. Hit Impact Integration (src/collision.rs)

**Modify:** `handle_enemy_bullet_collision()` function

**Changes:**
```rust
fn handle_enemy_bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    tree: Res<EnemyKdTree>,
    mut enemy_query: Query<(&Transform, &mut Enemy, &EnemyColor), With<Enemy>>, // ADD EnemyColor
    config: Res<GameConfig>,
    particle_assets: Res<ParticleEffectAssets>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        let pos = bullet_transform.translation;
        let enemies = tree.0.within_radius(&[pos.x, pos.y], 50.0);

        for e in enemies {
            if let Ok((_, mut enemy, enemy_color)) = enemy_query.get_mut(e.entity) {
                enemy.health -= config.gun.bullet_damage;

                // NEW: Find matching color variant
                let impact_effect = find_closest_effect_variant(
                    enemy_color.0,
                    &particle_assets.impact_variants,
                );

                // Spawn colored impact
                commands.spawn((
                    ParticleEffect::new(impact_effect),
                    Transform::from_translation(pos),
                    ImpactEffect {
                        lifetime: Timer::from_seconds(
                            config.particle_effects.impact_lifetime,
                            TimerMode::Once
                        ),
                    },
                ));
            }
        }
    }
}
```

### 4. Death Explosion Integration (src/enemy.rs)

**Modify:** `despawn_dead_enemies()` function

**Changes:**
```rust
fn despawn_dead_enemies(
    mut commands: Commands,
    enemy_query: Query<(&Enemy, &EnemyColor, &Transform, Entity), With<Enemy>>, // ADD EnemyColor, Transform
    particle_assets: Res<ParticleEffectAssets>, // ADD
    config: Res<GameConfig>, // ADD
) {
    for (enemy, enemy_color, transform, entity) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            let pos = transform.translation;

            // Spawn colored death burst
            let death_effect = find_closest_effect_variant(
                enemy_color.0,
                &particle_assets.death_burst_variants,
            );

            commands.spawn((
                ParticleEffect::new(death_effect),
                Transform::from_translation(pos),
                ImpactEffect {
                    lifetime: Timer::from_seconds(
                        config.particle_effects.death_burst_lifetime,
                        TimerMode::Once
                    ),
                },
            ));

            // Spawn lingering secondary effect
            commands.spawn((
                ParticleEffect::new(particle_assets.death_linger.clone()),
                Transform::from_translation(pos),
                DeathLingerEffect {
                    lifetime: Timer::from_seconds(
                        config.particle_effects.death_linger_lifetime,
                        TimerMode::Once
                    ),
                },
            ));

            // Despawn enemy with children
            let mut entity_commands = commands.entity(entity);
            entity_commands.despawn_children();
            entity_commands.despawn();
        }
    }
}
```

### 5. Cleanup (src/particle_effects.rs)

**Add new system:**
```rust
fn despawn_finished_death_linger(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DeathLingerEffect)>,
) {
    for (entity, mut linger) in query.iter_mut() {
        linger.lifetime.tick(time.delta());
        if linger.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
```

**Register in plugin:**
```rust
impl Plugin for ParticleEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup_particle_assets)
            .add_systems(
                Update,
                (
                    despawn_finished_impacts,
                    despawn_finished_death_linger, // NEW
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
```

## Configuration

### config.toml additions

```toml
[particle_effects]
# Existing bullet trail config (unchanged)
bullet_trail_emission_rate = 300.0
bullet_trail_lifetime = 0.25
bullet_trail_size = 3.0
bullet_trail_color_r = 0.0
bullet_trail_color_g = 0.8
bullet_trail_color_b = 1.0

# Hit impact config
impact_particle_count = 30
impact_lifetime = 0.4
impact_size = 4.0
impact_velocity = 50.0

# Death burst config
death_burst_particle_count = 60
death_burst_lifetime = 0.4
death_burst_size = 4.0
death_burst_velocity = 100.0

# Death lingering effect config
death_linger_particle_count = 25
death_linger_lifetime = 1.2
death_linger_size = 2.0
death_linger_velocity = 20.0

# Color palette
color_variant_count = 8
```

### Config struct additions (src/config_loader.rs)

```rust
#[derive(Debug, Deserialize, Clone)]
pub struct ParticleEffectsConfig {
    // Existing bullet trail fields...

    // Hit impact
    pub impact_particle_count: u32,
    pub impact_lifetime: f32,
    pub impact_size: f32,
    pub impact_velocity: f32,

    // Death burst
    pub death_burst_particle_count: u32,
    pub death_burst_lifetime: f32,
    pub death_burst_size: f32,
    pub death_burst_velocity: f32,

    // Death linger
    pub death_linger_particle_count: u32,
    pub death_linger_lifetime: f32,
    pub death_linger_size: f32,
    pub death_linger_velocity: f32,

    // Palette
    pub color_variant_count: u32,
}
```

## Performance Analysis

### Memory Overhead

**Before (current system):**
- 2 EffectAssets (bullet_trail, impact_burst)

**After (per-enemy colored system):**
- 1 bullet_trail (unchanged)
- 8 impact_variants
- 8 death_burst_variants
- 1 death_linger
- **Total: 18 EffectAssets** (9× increase, but still minimal GPU memory)

### Particle Count Estimate

**Steady state during gameplay:**
- Bullet trails: 3,000 particles (unchanged)
- Hit impacts: 30 particles × 40 active × 0.4s duration = 1,200 particles
- Death bursts: 60 particles × 20 active × 0.4s duration = 1,200 particles
- Death linger: 25 particles × 20 active × 1.2s duration = 1,500 particles
- **Total: ~7,000 particles** (comparable to current system)

### Draw Call Analysis

**Batching efficiency:**
- 1 draw call for bullet trails (all cyan)
- Up to 8 draw calls for hit impacts (one per color variant in use)
- Up to 8 draw calls for death bursts (one per color variant in use)
- 1 draw call for death linger (shared white/gray)
- **Worst case: 18 draw calls** (when all 8 colors active simultaneously)
- **Typical case: 5-8 draw calls** (3-4 dominant enemy colors on screen)

**GPU overhead:** Negligible - modern GPUs handle 18 batched draw calls trivially

### Comparison to Alternatives

| Approach | EffectAssets | Draw Calls | Flexibility | Complexity |
|----------|--------------|------------|-------------|------------|
| Current (shared) | 2 | 2 | None | Simple |
| **Pooling (chosen)** | **18** | **5-18** | **Good** | **Moderate** |
| Runtime modification | 2 | 2 | Perfect | High |
| Custom attributes | 2 | 2 | Perfect | Very High |

## Module Changes

### Files Modified

- **src/particle_effects.rs**
  - Add `impact_variants`, `death_burst_variants`, `death_linger` to `ParticleEffectAssets`
  - Add `DeathLingerEffect` component
  - Modify `setup_particle_assets()` to generate 8 color variants
  - Add `find_closest_effect_variant()` helper function
  - Add `create_color_gradient()` helper function
  - Add `despawn_finished_death_linger()` system

- **src/collision.rs**
  - Modify `handle_enemy_bullet_collision()` query to include `&EnemyColor`
  - Change impact spawn to use `find_closest_effect_variant()`

- **src/enemy.rs**
  - Modify `despawn_dead_enemies()` query to include `&EnemyColor` and `&Transform`
  - Add `particle_assets: Res<ParticleEffectAssets>` parameter
  - Add `config: Res<GameConfig>` parameter
  - Spawn death burst with color matching
  - Spawn death linger effect

- **src/config_loader.rs**
  - Add 12 new fields to `ParticleEffectsConfig`

- **config.toml**
  - Add 12 new particle effect configuration values

### Files Unchanged

- src/gun.rs (bullet trails remain cyan)
- src/world.rs
- src/player.rs
- src/animation.rs

## Testing Plan

### Visual Verification

1. **Color variety:**
   - Spawn 20+ enemies
   - Shoot each enemy
   - Verify impacts use different colors matching enemy tints (within palette)

2. **Death effects:**
   - Kill 10 enemies
   - Verify death burst larger than hit impact
   - Verify lingering smoke effect appears and drifts upward

3. **Color matching:**
   - Spawn red-tinted enemy → impact should use red variant
   - Spawn blue-tinted enemy → impact should use blue variant
   - Verify closest match for intermediate colors (e.g., orange enemy uses orange or red)

### Performance Testing

1. **FPS stability:**
   - Spawn 100 enemies
   - Rapid-fire bullets hitting multiple enemies
   - Measure FPS (target: maintain 60 FPS)

2. **Memory stability:**
   - Play for 5 minutes with continuous enemy spawning/death
   - Monitor entity count (should not leak)
   - Check particle effect instances cleanup correctly

3. **Draw call count:**
   - Use GPU profiler to verify batching
   - Verify worst-case draw calls < 20

### Edge Cases

1. **All enemies same color:**
   - Verify batching works (all impacts in one draw call)

2. **Rapid enemy death:**
   - Kill 20 enemies simultaneously
   - Verify all death effects spawn correctly
   - No crashes or performance spikes

3. **Config changes:**
   - Modify `death_burst_particle_count` to 120
   - Restart game
   - Verify larger death explosions

## Future Enhancements (Out of Scope)

- Enemy movement trails (continuous particle emission while moving)
- Per-enemy-type effect variations (larger explosions for bigger enemies)
- HSV color matching instead of RGB (better perceptual matching)
- Dynamic palette generation from actual enemy color distribution
- Screen shake on death explosion
- Sound effects tied to particle spawns

## References

- Current particle system: `src/particle_effects.rs`
- Enemy color system: `src/enemy.rs:117-122` (random color generation)
- Enemy tint rendering: `src/enemy.rs:152-164` (dual-layer sprites)
- Collision detection: `src/collision.rs:handle_enemy_bullet_collision()`
- bevy_hanabi 0.17.0 gradient API: https://docs.rs/bevy_hanabi/0.17.0
