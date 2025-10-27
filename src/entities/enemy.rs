//! Enemy entity definition - Owned by Gameplay Agent
//!
//! Enemy component, types (Green/Red/Skin), spawning logic, AI movement, and death effects

use std::time::Duration;
use std::f32::consts::PI;

use bevy::math::vec3;
use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::Rng;

use crate::animation::AnimationTimer;
use crate::player::Player;
use crate::state::GameState;
use crate::world::GameEntity;
use crate::particle_effects::{ParticleEffectAssets, DeathLingerEffect, ImpactEffect, find_closest_effect_variant};
use bevy_hanabi::prelude::*;
use crate::*;
use crate::plugin_mode::PluginMode;

pub struct EnemyPlugin {
    mode: PluginMode,
}

impl EnemyPlugin {
    pub fn new(mode: PluginMode) -> Self {
        Self { mode }
    }
}

impl Default for EnemyPlugin {
    fn default() -> Self {
        Self::new(PluginMode::default())
    }
}

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
}

#[derive(Component)]
pub enum EnemyType {
    Green,
    Red,
    Skin,
}

#[derive(Component)]
pub struct EnemyColor(pub Color);

/// Marker for the base sprite child (black/white details)
#[derive(Component)]
pub struct EnemyBaseSprite;

/// Marker for the tint sprite child (colored layer)
#[derive(Component)]
pub struct EnemyTintSprite;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // Get config for spawn interval
        let config = app.world().get_resource::<crate::config_loader::GameConfig>()
            .expect("GameConfig must be inserted before EnemyPlugin");
        let spawn_interval = config.enemy.spawn_interval;

        // Logic systems (always run in both modes)
        app.add_systems(
            Update,
            (
                update_enemy_transform,
                despawn_dead_enemies_logic,
            )
                .run_if(in_state(GameState::InGame)),
        );

        // Rendering systems (only run in Standard mode)
        if self.mode == PluginMode::Standard {
            app.add_systems(
                Update,
                (
                    spawn_enemies.run_if(on_timer(Duration::from_secs_f32(spawn_interval))),
                    sync_enemy_colors,
                    despawn_dead_enemies_visual,
                )
                    .run_if(in_state(GameState::InGame)),
            );
        }
    }
}

/// Logic-only despawn system - removes dead enemies without visual effects
fn despawn_dead_enemies_logic(
    mut commands: Commands,
    enemy_query: Query<(&Enemy, Entity), With<Enemy>>,
) {
    if enemy_query.is_empty() {
        return;
    }

    for (enemy, entity) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Visual-only despawn system - spawns particle effects for dead enemies
/// Runs BEFORE despawn_dead_enemies_logic to spawn effects before entity is removed
fn despawn_dead_enemies_visual(
    mut commands: Commands,
    enemy_query: Query<(&Enemy, &EnemyColor, &Transform, Entity), With<Enemy>>,
    particle_assets: Res<ParticleEffectAssets>,
    config: Res<crate::config_loader::GameConfig>,
) {
    if enemy_query.is_empty() {
        return;
    }

    for (enemy, enemy_color, transform, entity) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            let pos = transform.translation;

            // Spawn colored death burst (large explosion)
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

            // Spawn lingering smoke effect (white/gray, slow upward)
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

            // Mark for despawn - the logic system will handle the actual despawn
            // This avoids double-despawn issues with children
            // We only spawn the particle effects here, let the logic system handle removal
        }
    }
}

fn update_enemy_transform(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    config: Res<crate::config_loader::GameConfig>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let Ok(player_transform) = player_query.single() else { return; };
    let player_pos = player_transform.translation;
    for mut transform in enemy_query.iter_mut() {
        let dir = (player_pos - transform.translation).normalize();
        transform.translation += dir * config.enemy.speed;

        // Rotate sprite to face movement direction
        // Sprite default orientation is facing DOWN, so add π/2 to correct
        let angle = dir.y.atan2(dir.x) + PI / 2.0;
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn spawn_enemies(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    config: Res<crate::config_loader::GameConfig>,
) {
    let num_enemies = enemy_query.iter().len();
    let enemy_spawn_count = (config.enemy.max_num_enemies - num_enemies).min(config.enemy.spawn_rate_per_second);

    if num_enemies >= config.enemy.max_num_enemies || player_query.is_empty() {
        return;
    }

    let Ok(player_transform) = player_query.single() else { return; };
    let player_pos = player_transform.translation.truncate();
    for _ in 0..enemy_spawn_count {
        let (x, y) = get_random_position_around(player_pos, &config);
        let enemy_type = EnemyType::get_rand_enemy();

        // Start at a random frame in the animation cycle (0-7)
        let mut rng = rand::thread_rng();
        let start_frame = rng.gen_range(0..config.enemy_sprites.enemy_animation_frames);

        // Generate random color for this enemy
        let random_color = Color::srgb(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        );

        // Spawn parent entity with two sprite children (base + tint layer)
        commands.spawn((
            Transform::from_translation(vec3(x, y, 1.0))
                .with_scale(Vec3::splat(enemy_type.get_scale_factor())),
            Enemy {
                health: config.enemy.health,
            },
            enemy_type,
            EnemyColor(random_color),
            AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
            GameEntity,
            Visibility::default(),
            InheritedVisibility::default(),
        ))
        .with_children(|parent| {
            // Base sprite (black/white details, no tint)
            parent.spawn((
                Sprite {
                    image: handle.enemy_bg_image.clone().unwrap(),
                    texture_atlas: Some(TextureAtlas {
                        layout: handle.enemy_layout.clone().unwrap(),
                        index: start_frame,
                    }),
                    ..default()
                },
                Transform::default(),
                InheritedVisibility::default(),
                Visibility::default(),
                EnemyBaseSprite,
            ));

            // Tint sprite (white silhouette, will be colored)
            parent.spawn((
                Sprite {
                    image: handle.enemy_tint_image.clone().unwrap(),
                    texture_atlas: Some(TextureAtlas {
                        layout: handle.enemy_layout.clone().unwrap(),
                        index: start_frame,
                    }),
                    color: random_color, // Apply tint color
                    ..default()
                },
                Transform::from_translation(vec3(0.0, 0.0, 0.1)), // Slightly in front
                InheritedVisibility::default(),
                Visibility::default(),
                EnemyTintSprite,
            ));
        });
    }
}

fn get_random_position_around(pos: Vec2, config: &crate::config_loader::GameConfig) -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..PI * 2.0);
    let dist = rng.gen_range(config.enemy.spawn_distance_min..config.enemy.spawn_distance_max);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    (random_x, random_y)
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: ENEMY_HEALTH,
        }
    }
}

impl EnemyType {
    fn get_rand_enemy() -> Self {
        let mut rng = rand::thread_rng();
        let rand_index = rng.gen_range(0..3);
        return match rand_index {
            0 => Self::Green,
            1 => Self::Red,
            _ => Self::Skin,
        };
    }

    pub fn get_base_sprite_index(&self) -> usize {
        match self {
            EnemyType::Green => 8,
            EnemyType::Red => 12,
            EnemyType::Skin => 20,
        }
    }

    pub fn get_scale_factor(&self) -> f32 {
        match self {
            EnemyType::Green => SPRITE_SCALE_FACTOR,
            EnemyType::Red => SPRITE_SCALE_FACTOR,
            EnemyType::Skin => SPRITE_SCALE_FACTOR,
        }
    }
}

/// Syncs the tint sprite color with the parent EnemyColor component
fn sync_enemy_colors(
    enemy_query: Query<(&Children, &EnemyColor), With<Enemy>>,
    mut sprite_query: Query<&mut Sprite, With<EnemyTintSprite>>,
) {
    for (children, enemy_color) in enemy_query.iter() {
        for child in children.iter() {
            if let Ok(mut sprite) = sprite_query.get_mut(child) {
                sprite.color = enemy_color.0;
            }
        }
    }
}
