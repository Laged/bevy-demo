use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::Rng;

use crate::animation::AnimationTimer;
use crate::gun::{Gun, GunTimer};
use crate::player::{Health, Player, PlayerState};
use crate::{state::GameState, GlobalTextureAtlas};

pub struct WorldPlugin;

#[derive(Component)]
pub struct GameEntity;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameInit),
            (init_world, spawn_world_decorations),
        )
        .add_systems(OnExit(GameState::InGame), despawn_all_game_entities);
    }
}

fn init_world(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
    config: Res<crate::config_loader::GameConfig>,
) {
    commands.spawn((
        Sprite {
            image: handle.image.clone().unwrap(),
            texture_atlas: Some(TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(config.sprites.sprite_scale_factor)),
        Player,
        Health(config.player.health),
        PlayerState::default(),
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        GameEntity,
    ));
    commands.spawn((
        Sprite {
            image: handle.image.clone().unwrap(),
            texture_atlas: Some(TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 17,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(config.sprites.sprite_scale_factor)),
        Gun,
        GunTimer(Stopwatch::new()),
        GameEntity,
    ));

    next_state.set(GameState::InGame);
}

fn spawn_world_decorations(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    config: Res<crate::config_loader::GameConfig>,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..config.world.num_world_decorations {
        let x = rng.gen_range(-config.world.world_width..config.world.world_width);
        let y = rng.gen_range(-config.world.world_height..config.world.world_height);
        commands.spawn((
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: rng.gen_range(24..=25),
                }),
                ..default()
            },
            Transform::from_translation(vec3(x, y, 0.0))
                .with_scale(Vec3::splat(config.sprites.sprite_scale_factor)),
            GameEntity,
        ));
    }
}

fn despawn_all_game_entities(
    mut commands: Commands,
    all_entities: Query<Entity, With<GameEntity>>,
) {
    for e in all_entities.iter() {
        commands.entity(e).despawn();
    }
}
