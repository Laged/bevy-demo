use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use kd_tree::{KdPoint, KdTree};

use crate::player::{Player, Health};
use crate::{enemy::{Enemy, EnemyColor}, gun::Bullet, state::GameState};
use crate::particle_effects::{ImpactEffect, ParticleEffectAssets, find_closest_effect_variant};
use bevy_hanabi::prelude::*;

pub struct CollisionPlugin;

#[derive(Component)]
struct Collidable {
    pos: Vec2,
    entity: Entity,
}
#[derive(Resource)]
struct EnemyKdTree(KdTree<Collidable>);

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        // Get config for kd-tree refresh rate
        let config = app.world().get_resource::<crate::config_loader::GameConfig>()
            .expect("GameConfig must be inserted before CollisionPlugin");
        let refresh_rate = config.kd_tree.refresh_rate;

        app.insert_resource(EnemyKdTree::default()).add_systems(
            Update,
            (
                handle_enemy_bullet_collision,
                handle_enemy_player_collision,
                update_enemy_kd_tree
                    .run_if(on_timer(Duration::from_secs_f32(refresh_rate))),
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_enemy_player_collision(
    player_query: Query<&Transform, With<Player>>,
    tree: Res<EnemyKdTree>,
    mut player_health: Query<&mut Health, With<Player>>,
    config: Res<crate::config_loader::GameConfig>,
) {
    if player_query.is_empty() {
        return;
    }

    let Ok(player_transform) = player_query.single() else { return; };
    let player_pos = player_transform.translation;
    let enemies = tree.0.within_radius(&[player_pos.x, player_pos.y], 50.0);
    if !enemies.is_empty() {
        if let Ok(mut health) = player_health.single_mut() {
            health.0 -= config.enemy.damage;
        }
    }
}

fn update_enemy_kd_tree(
    mut tree: ResMut<EnemyKdTree>,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
) {
    let mut items = Vec::new();
    for (t, e) in enemy_query.iter() {
        items.push(Collidable {
            entity: e,
            pos: t.translation.truncate(),
        })
    }

    tree.0 = KdTree::build_by_ordered_float(items);
}

fn handle_enemy_bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    tree: Res<EnemyKdTree>,
    mut enemy_query: Query<(&Transform, &mut Enemy, &EnemyColor), With<Enemy>>,
    config: Res<crate::config_loader::GameConfig>,
    particle_assets: Res<ParticleEffectAssets>,
) {
    if bullet_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    for (_bullet_entity, bullet_transform) in bullet_query.iter() {
        let pos = bullet_transform.translation;
        let enemies = tree.0.within_radius(&[pos.x, pos.y], 50.0);

        for e in enemies {
            if let Ok((_, mut enemy, enemy_color)) = enemy_query.get_mut(e.entity) {
                enemy.health -= config.gun.bullet_damage;

                // Find closest matching color variant from palette
                let impact_effect = find_closest_effect_variant(
                    enemy_color.0,
                    &particle_assets.impact_variants,
                );

                // Spawn colored impact burst
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

impl KdPoint for Collidable {
    type Scalar = f32;
    type Dim = typenum::U2;
    fn at(&self, k: usize) -> f32 {
        if k == 0 {
            return self.pos.x;
        }

        self.pos.y
    }
}

impl Default for EnemyKdTree {
    fn default() -> Self {
        Self(KdTree::build_by_ordered_float(vec![]))
    }
}
