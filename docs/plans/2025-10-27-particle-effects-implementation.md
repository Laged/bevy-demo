# Particle Effects Implementation Plan

> **STATUS: ✅ COMPLETED - Merged to dev branch**

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add GPU-accelerated bullet trails and impact particle effects using bevy_hanabi while keeping sprite-based collision detection.

**Architecture:** Hybrid system - sprite bullets remain unchanged for collision logic. Particle emitters spawn as child entities for trails (continuous emission) and as standalone entities for impacts (one-shot bursts). GPU compute shaders handle all particle simulation.

**Tech Stack:** Bevy 0.17.2, bevy_hanabi 0.17 (GPU particle system), existing sprite-based bullet system

---

## Task 1: Add bevy_hanabi Dependency

**Files:**
- Modify: `Cargo.toml`

**Step 1: Add bevy_hanabi dependency**

Edit `Cargo.toml` dependencies section:

```toml
[dependencies]
bevy = "0.17.2"
bevy_pancam = "0.19.0"
kd-tree = "0.5.3"
rand = "0.8.5"
typenum = "1.17.0"
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
bevy_hanabi = "0.17"
```

**Step 2: Build to verify dependency resolves**

Run: `cargo build`

Expected: Successfully downloads and compiles bevy_hanabi

**Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "deps: add bevy_hanabi for GPU particle effects"
```

---

## Task 2: Create Particle Effects Module

**Files:**
- Create: `src/particle_effects.rs`
- Modify: `src/lib.rs`

**Step 1: Create particle effects module file**

Create `src/particle_effects.rs`:

```rust
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::state::GameState;

pub struct ParticleEffectsPlugin;

impl Plugin for ParticleEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup_particle_assets);
    }
}

#[derive(Resource)]
pub struct ParticleEffectAssets {
    pub bullet_trail: Handle<EffectAsset>,
    pub impact_burst: Handle<EffectAsset>,
}

#[derive(Component)]
pub struct BulletTrailEmitter;

#[derive(Component)]
pub struct ImpactEffect {
    pub lifetime: Timer,
}

fn setup_particle_assets(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    // Placeholder - we'll implement effect creation in next task
    let bullet_trail = effects.add(EffectAsset::default());
    let impact_burst = effects.add(EffectAsset::default());

    commands.insert_resource(ParticleEffectAssets {
        bullet_trail,
        impact_burst,
    });
}
```

**Step 2: Register module in lib.rs**

Edit `src/lib.rs`, add to module declarations:

```rust
pub mod animation;
pub mod camera;
pub mod collision;
pub mod config_loader;
pub mod configs;
pub mod enemy;
pub mod gui;
pub mod gun;
pub mod particle_effects;  // NEW
pub mod player;
pub mod resources;
pub mod state;
pub mod world;
```

**Step 3: Build to verify module compiles**

Run: `cargo build`

Expected: Compiles successfully with no errors

**Step 4: Commit**

```bash
git add src/particle_effects.rs src/lib.rs
git commit -m "feat: add particle effects module skeleton"
```

---

## Task 3: Register Hanabi Plugin in Main

**Files:**
- Modify: `src/main.rs`

**Step 1: Import and add HanabiPlugin**

Edit `src/main.rs`, add import at top:

```rust
use bevy_hanabi::HanabiPlugin;
use hell_game::particle_effects::ParticleEffectsPlugin;
```

Then in `main()` function, add plugins after DefaultPlugins:

```rust
.add_plugins(
    DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            // ... existing config ...
        }),
)
.add_plugins(HanabiPlugin)  // NEW: Add after DefaultPlugins
.init_state::<GameState>()
.insert_resource(ClearColor(Color::srgb_u8(
    config.colors.bg_color[0],
    config.colors.bg_color[1],
    config.colors.bg_color[2],
)))
.insert_resource(config)
.add_plugins(FollowCameraPlugin)
.add_plugins(GuiPlugin)
.add_plugins(GunPlugin)
.add_plugins(PlayerPlugin)
.add_plugins(AnimationPlugin)
.add_plugins(ResourcesPlugin)
.add_plugins(WorldPlugin)
.add_plugins(EnemyPlugin)
.add_plugins(CollisionPlugin)
.add_plugins(ParticleEffectsPlugin)  // NEW: Add our particle plugin
```

**Step 2: Build and run to verify plugins load**

Run: `cargo run`

Expected: Game runs with no errors (particles not visible yet, just loading system)

**Step 3: Commit**

```bash
git add src/main.rs
git commit -m "feat: register HanabiPlugin and ParticleEffectsPlugin"
```

---

## Task 4: Implement Bullet Trail Effect Asset

**Files:**
- Modify: `src/particle_effects.rs`

**Step 1: Create bullet trail effect configuration**

Replace the `setup_particle_assets` function in `src/particle_effects.rs`:

```rust
fn setup_particle_assets(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    // Bullet trail effect - continuous emission
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.0, 0.8, 1.0, 1.0)); // Bright cyan at spawn
    gradient.add_key(1.0, Vec4::new(0.0, 0.8, 1.0, 0.0)); // Fade to transparent

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(0.25).expr(); // 0.25 second trail
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(2.0).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(10.0).expr(),
    };

    let bullet_trail = effects.add(
        EffectAsset::new(vec![32768], Spawner::rate(300.0.into()), writer.finish())
            .with_name("bullet_trail")
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .render(ColorOverLifetimeModifier { gradient })
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(3.0)),
                screen_space_size: false,
            }),
    );

    // Placeholder for impact burst (next task)
    let impact_burst = effects.add(EffectAsset::default());

    commands.insert_resource(ParticleEffectAssets {
        bullet_trail,
        impact_burst,
    });
}
```

**Step 2: Build to verify effect compiles**

Run: `cargo build`

Expected: Compiles successfully

**Step 3: Commit**

```bash
git add src/particle_effects.rs
git commit -m "feat: implement bullet trail particle effect asset"
```

---

## Task 5: Implement Impact Burst Effect Asset

**Files:**
- Modify: `src/particle_effects.rs`

**Step 1: Create impact burst effect configuration**

Replace the `impact_burst` placeholder in `setup_particle_assets`:

```rust
fn setup_particle_assets(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    // ... bullet_trail code from previous task ...

    // Impact burst effect - one-shot radial explosion
    let mut impact_gradient = Gradient::new();
    impact_gradient.add_key(0.0, Vec4::new(1.0, 0.9, 0.3, 1.0)); // Bright yellow-white
    impact_gradient.add_key(0.5, Vec4::new(1.0, 0.5, 0.0, 0.8)); // Orange
    impact_gradient.add_key(1.0, Vec4::new(1.0, 0.0, 0.0, 0.0)); // Fade to red transparent

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(0.4).expr(); // 0.4 second burst
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(5.0).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(50.0).expr(),
    };

    let impact_burst = effects.add(
        EffectAsset::new(vec![2048], Spawner::once(30.0.into(), true), writer.finish())
            .with_name("impact_burst")
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .render(ColorOverLifetimeModifier { gradient: impact_gradient })
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(4.0)),
                screen_space_size: false,
            }),
    );

    commands.insert_resource(ParticleEffectAssets {
        bullet_trail,
        impact_burst,
    });
}
```

**Step 2: Build to verify effect compiles**

Run: `cargo build`

Expected: Compiles successfully

**Step 3: Commit**

```bash
git add src/particle_effects.rs
git commit -m "feat: implement impact burst particle effect asset"
```

---

## Task 6: Add Trail Emitters to Bullets

**Files:**
- Modify: `src/gun.rs`

**Step 1: Import particle types**

Add imports at top of `src/gun.rs`:

```rust
use crate::particle_effects::{BulletTrailEmitter, ParticleEffectAssets};
use bevy_hanabi::prelude::*;
```

**Step 2: Modify bullet spawn to add trail emitter child**

In `handle_gun_input()` function, modify the bullet spawn code (around line 114-128):

```rust
// Current code spawns bullet like this:
commands.spawn((
    Sprite {
        image: handle.image.clone().unwrap(),
        texture_atlas: Some(TextureAtlas {
            layout: handle.layout.clone().unwrap(),
            index: 16,
        }),
        ..default()
    },
    Transform::from_translation(vec3(gun_pos.x, gun_pos.y, 1.0))
        .with_scale(Vec3::splat(config.sprites.sprite_scale_factor)),
    Bullet,
    BulletDirection(dir),
    SpawnInstant(Instant::now()),
));

// Change to:
commands.spawn((
    Sprite {
        image: handle.image.clone().unwrap(),
        texture_atlas: Some(TextureAtlas {
            layout: handle.layout.clone().unwrap(),
            index: 16,
        }),
        ..default()
    },
    Transform::from_translation(vec3(gun_pos.x, gun_pos.y, 1.0))
        .with_scale(Vec3::splat(config.sprites.sprite_scale_factor)),
    Bullet,
    BulletDirection(dir),
    SpawnInstant(Instant::now()),
))
.with_children(|parent| {
    // NEW: Spawn trail emitter as child
    parent.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(particle_assets.bullet_trail.clone()),
            transform: Transform::default(),
            ..default()
        },
        BulletTrailEmitter,
    ));
});
```

**Step 3: Add particle_assets parameter to handle_gun_input**

Update the function signature (around line 83):

```rust
fn handle_gun_input(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut GunTimer), With<Gun>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    handle: Res<GlobalTextureAtlas>,
    config: Res<crate::config_loader::GameConfig>,
    particle_assets: Res<ParticleEffectAssets>,  // NEW
) {
```

**Step 4: Build and run to test trail effects**

Run: `cargo run`

Expected: Bullets now have glowing cyan trails following them

**Step 5: Commit**

```bash
git add src/gun.rs
git commit -m "feat: add particle trail emitters to bullets"
```

---

## Task 7: Add Impact Effects to Collisions

**Files:**
- Modify: `src/collision.rs`

**Step 1: Import particle types**

Add imports at top of `src/collision.rs`:

```rust
use crate::particle_effects::{ImpactEffect, ParticleEffectAssets};
use bevy_hanabi::prelude::*;
```

**Step 2: Modify collision handler to spawn impact effects**

In `handle_enemy_bullet_collision()` function (around line 75-95), modify to spawn impacts:

```rust
fn handle_enemy_bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,  // NEW: Add Entity to query
    tree: Res<EnemyKdTree>,
    mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>,
    config: Res<crate::config_loader::GameConfig>,
    particle_assets: Res<ParticleEffectAssets>,  // NEW
) {
    if bullet_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    for (bullet_entity, bullet_transform) in bullet_query.iter() {  // NEW: Get entity
        let pos = bullet_transform.translation;
        let enemies = tree.0.within_radius(&[pos.x, pos.y], 50.0);

        for e in enemies {
            if let Ok((_, mut enemy)) = enemy_query.get_mut(e.entity) {
                enemy.health -= config.gun.bullet_damage;

                // NEW: Spawn impact effect at bullet position
                commands.spawn((
                    ParticleEffectBundle {
                        effect: ParticleEffect::new(particle_assets.impact_burst.clone()),
                        transform: Transform::from_translation(pos),
                        ..default()
                    },
                    ImpactEffect {
                        lifetime: Timer::from_seconds(0.5, TimerMode::Once),
                    },
                ));
            }
        }
    }
}
```

**Step 3: Build and run to test impact effects**

Run: `cargo run`

Expected: Yellow-orange bursts appear when bullets hit enemies

**Step 4: Commit**

```bash
git add src/collision.rs
git commit -m "feat: add particle impact effects on bullet-enemy collision"
```

---

## Task 8: Add Impact Effect Cleanup System

**Files:**
- Modify: `src/particle_effects.rs`

**Step 1: Add cleanup system**

Add new system to `src/particle_effects.rs`:

```rust
fn despawn_finished_impacts(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ImpactEffect)>,
) {
    for (entity, mut impact) in query.iter_mut() {
        impact.lifetime.tick(time.delta());
        if impact.lifetime.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
```

**Step 2: Register cleanup system in plugin**

Modify `ParticleEffectsPlugin::build()`:

```rust
impl Plugin for ParticleEffectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Loading), setup_particle_assets)
            .add_systems(
                Update,
                despawn_finished_impacts.run_if(in_state(GameState::InGame)),
            );
    }
}
```

**Step 3: Build and run to verify cleanup**

Run: `cargo run`

Expected: Impact effects disappear after ~0.5 seconds, no entity leak

**Step 4: Monitor entity count to verify no leaks**

Run game for 30 seconds, check entity count in debug overlay doesn't grow unbounded

**Step 5: Commit**

```bash
git add src/particle_effects.rs
git commit -m "feat: add cleanup system for finished impact effects"
```

---

## Task 9: Add Particle Configuration to config.toml

**Files:**
- Modify: `config.toml`
- Modify: `src/config_loader.rs`

**Step 1: Add particle config section to config.toml**

Add at end of `config.toml`:

```toml
[particle_effects]
bullet_trail_emission_rate = 300.0
bullet_trail_lifetime = 0.25
bullet_trail_size = 3.0
bullet_trail_color_r = 0.0
bullet_trail_color_g = 0.8
bullet_trail_color_b = 1.0

impact_particle_count = 30
impact_lifetime = 0.4
impact_size = 4.0
```

**Step 2: Add config struct to config_loader.rs**

Add new struct in `src/config_loader.rs`:

```rust
#[derive(Debug, Deserialize, Clone)]
pub struct ParticleEffectsConfig {
    pub bullet_trail_emission_rate: f32,
    pub bullet_trail_lifetime: f32,
    pub bullet_trail_size: f32,
    pub bullet_trail_color_r: f32,
    pub bullet_trail_color_g: f32,
    pub bullet_trail_color_b: f32,
    pub impact_particle_count: u32,
    pub impact_lifetime: f32,
    pub impact_size: f32,
}
```

**Step 3: Add field to GameConfig**

Add to `GameConfig` struct:

```rust
#[derive(Debug, Deserialize, Clone, Resource)]
pub struct GameConfig {
    pub window: WindowConfig,
    pub sprites: SpritesConfig,
    pub enemy_sprites: EnemySpritesConfig,
    pub world: WorldConfig,
    pub player: PlayerConfig,
    pub enemy: EnemyConfig,
    pub kd_tree: KdTreeConfig,
    pub gun: GunConfig,
    pub colors: ColorsConfig,
    pub particle_effects: ParticleEffectsConfig,  // NEW
}
```

**Step 4: Add default values**

Add to `GameConfig::default()`:

```rust
particle_effects: ParticleEffectsConfig {
    bullet_trail_emission_rate: 300.0,
    bullet_trail_lifetime: 0.25,
    bullet_trail_size: 3.0,
    bullet_trail_color_r: 0.0,
    bullet_trail_color_g: 0.8,
    bullet_trail_color_b: 1.0,
    impact_particle_count: 30,
    impact_lifetime: 0.4,
    impact_size: 4.0,
},
```

**Step 5: Build to verify config loads**

Run: `cargo build`

Expected: Compiles successfully

**Step 6: Commit**

```bash
git add config.toml src/config_loader.rs
git commit -m "feat: add particle effects configuration to config system"
```

---

## Task 10: Use Config Values in Particle Effects

**Files:**
- Modify: `src/particle_effects.rs`

**Step 1: Update setup_particle_assets to use config**

Modify function signature and implementation:

```rust
fn setup_particle_assets(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    config: Res<crate::config_loader::GameConfig>,  // NEW
) {
    // Bullet trail effect - use config values
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(
        config.particle_effects.bullet_trail_color_r,
        config.particle_effects.bullet_trail_color_g,
        config.particle_effects.bullet_trail_color_b,
        1.0
    ));
    gradient.add_key(1.0, Vec4::new(
        config.particle_effects.bullet_trail_color_r,
        config.particle_effects.bullet_trail_color_g,
        config.particle_effects.bullet_trail_color_b,
        0.0
    ));

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(config.particle_effects.bullet_trail_lifetime).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(2.0).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(10.0).expr(),
    };

    let bullet_trail = effects.add(
        EffectAsset::new(
            vec![32768],
            Spawner::rate(config.particle_effects.bullet_trail_emission_rate.into()),
            writer.finish()
        )
            .with_name("bullet_trail")
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .render(ColorOverLifetimeModifier { gradient })
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(config.particle_effects.bullet_trail_size)),
                screen_space_size: false,
            }),
    );

    // Impact burst - use config values
    let mut impact_gradient = Gradient::new();
    impact_gradient.add_key(0.0, Vec4::new(1.0, 0.9, 0.3, 1.0));
    impact_gradient.add_key(0.5, Vec4::new(1.0, 0.5, 0.0, 0.8));
    impact_gradient.add_key(1.0, Vec4::new(1.0, 0.0, 0.0, 0.0));

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(config.particle_effects.impact_lifetime).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(5.0).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(50.0).expr(),
    };

    let impact_burst = effects.add(
        EffectAsset::new(
            vec![2048],
            Spawner::once(config.particle_effects.impact_particle_count.into(), true),
            writer.finish()
        )
            .with_name("impact_burst")
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .render(ColorOverLifetimeModifier { gradient: impact_gradient })
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(config.particle_effects.impact_size)),
                screen_space_size: false,
            }),
    );

    commands.insert_resource(ParticleEffectAssets {
        bullet_trail,
        impact_burst,
    });
}
```

**Step 2: Update ImpactEffect lifetime to use config**

Modify `despawn_finished_impacts` and impact spawn in collision.rs to use config value

In `src/collision.rs`, update impact spawn:

```rust
ImpactEffect {
    lifetime: Timer::from_seconds(config.particle_effects.impact_lifetime, TimerMode::Once),
}
```

**Step 3: Build and test with different config values**

Run: `cargo run`

Expected: Particle effects use config values

**Step 4: Test tweaking config values**

Edit config.toml, change `bullet_trail_color_g` to 0.2, restart game

Expected: Trail color changes from cyan to more blue/purple

**Step 5: Commit**

```bash
git add src/particle_effects.rs src/collision.rs
git commit -m "feat: use config values for particle effect parameters"
```

---

## Task 11: Final Testing and Performance Verification

**Files:**
- None (testing only)

**Step 1: Visual verification**

Run: `cargo run`

Test checklist:
- [ ] Bullet trails visible behind bullets (cyan glow)
- [ ] Trails follow bullets smoothly
- [ ] Impact bursts appear when bullets hit enemies (yellow-orange flash)
- [ ] No visual glitches or flickering
- [ ] Particles fade out smoothly

**Step 2: Performance testing**

Run game for 2 minutes with active combat:
- [ ] FPS stays at 60 (or target framerate)
- [ ] No stuttering when many particles on screen
- [ ] Entity count doesn't grow unbounded

Check with debug overlay or add temporary FPS counter

**Step 3: Memory leak check**

Run game for 5 minutes, monitor memory usage:
- [ ] Memory usage stable (no continuous growth)
- [ ] Impact effects despawn after lifetime
- [ ] Trail emitters despawn with bullets

**Step 4: Configuration flexibility**

Edit config.toml values:
- [ ] Changing `bullet_trail_emission_rate` affects trail density
- [ ] Changing trail colors works correctly
- [ ] Changing `impact_particle_count` affects burst intensity

**Step 5: Document any issues**

If issues found, create GitHub issues or notes for future work

**Step 6: Final commit (if any fixes needed)**

```bash
git add <any-final-files>
git commit -m "fix: address particle effect edge cases"
```

---

## Completion

All tasks complete! The particle effects system is now integrated:

✅ bevy_hanabi dependency added
✅ Particle effects module created
✅ Bullet trail effect implemented (continuous emission)
✅ Impact burst effect implemented (one-shot radial)
✅ Trail emitters attached to bullets as children
✅ Impact effects spawn on collision
✅ Cleanup system prevents entity leaks
✅ Configuration system integrated
✅ Visual and performance testing complete

**Next steps (optional enhancements):**
- Add muzzle flash particles at gun barrel
- Add dynamic point lights for impact flashes
- Vary particle colors based on weapon/bullet type
- Add screen shake on impact (non-particle feature)
