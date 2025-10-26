use std::time::Instant;
use std::f32::consts::PI;

use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::Rng;

use crate::particle_effects::{BulletTrailEmitter, ParticleEffectAssets};
use crate::player::Player;
use crate::state::GameState;
use crate::*;
use bevy_hanabi::prelude::*;

pub struct GunPlugin;

#[derive(Component)]
pub struct Gun;
#[derive(Component)]
pub struct GunTimer(pub Stopwatch);
#[derive(Component)]
pub struct Bullet;
#[derive(Component)]
pub struct SpawnInstant(Instant);
#[derive(Component)]
struct BulletDirection(Vec3);

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_gun_transform,
                update_bullets,
                handle_gun_input,
                despawn_old_bullets,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn despawn_old_bullets(
    mut commands: Commands,
    bullet_query: Query<(&SpawnInstant, Entity), With<Bullet>>,
    config: Res<crate::config_loader::GameConfig>,
) {
    for (instant, e) in bullet_query.iter() {
        if instant.0.elapsed().as_secs_f32() > config.gun.bullet_time_secs {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn update_gun_transform(
    cursor_pos: Res<CursorPosition>,
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }

    let Ok(player_transform) = player_query.single() else { return; };
    let player_pos = player_transform.translation.truncate();
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => player_pos,
    };
    let Ok(mut gun_transform) = gun_query.single_mut() else { return; };

    let angle = (player_pos.y - cursor_pos.y).atan2(player_pos.x - cursor_pos.x) + PI;
    gun_transform.rotation = Quat::from_rotation_z(angle);

    let offset = 20.0;
    let new_gun_pos = vec2(
        player_pos.x + offset * angle.cos() - 5.0,
        player_pos.y + offset * angle.sin() - 10.0,
    );

    gun_transform.translation = vec3(new_gun_pos.x, new_gun_pos.y, gun_transform.translation.z);
    gun_transform.translation.z = 15.0;
}

fn handle_gun_input(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut GunTimer), With<Gun>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    handle: Res<GlobalTextureAtlas>,
    config: Res<crate::config_loader::GameConfig>,
    particle_assets: Res<ParticleEffectAssets>,
) {
    if gun_query.is_empty() {
        return;
    }

    let Ok((gun_transform, mut gun_timer)) = gun_query.single_mut() else { return; };
    let gun_pos = gun_transform.translation.truncate();
    gun_timer.0.tick(time.delta());

    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    let mut rng = rand::thread_rng();
    let bullet_direction = gun_transform.local_x();
    if gun_timer.0.elapsed_secs() >= config.gun.bullet_spawn_interval {
        gun_timer.0.reset();

        for _ in 0..config.gun.num_bullets_per_shot {
            let dir = vec3(
                bullet_direction.x + rng.gen_range(-0.5..0.5),
                bullet_direction.y + rng.gen_range(-0.5..0.5),
                bullet_direction.z,
            );
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
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
                Bullet,
                BulletDirection(dir),
                SpawnInstant(Instant::now()),
            ))
            .with_children(|parent| {
                // Spawn trail emitter as child
                parent.spawn((
                    ParticleEffect::new(particle_assets.bullet_trail.clone()),
                    Transform::default(),
                    BulletTrailEmitter,
                ));
            });
        }
    }
}

fn update_bullets(
    mut bullet_query: Query<(&mut Transform, &BulletDirection), With<Bullet>>,
    config: Res<crate::config_loader::GameConfig>,
) {
    if bullet_query.is_empty() {
        return;
    }

    for (mut t, dir) in bullet_query.iter_mut() {
        t.translation += dir.0.normalize() * Vec3::splat(config.gun.bullet_speed);
        t.translation.z = 10.0;
    }
}
