use bevy::prelude::*;

use crate::{
    enemy::{Enemy, EnemyType},
    gun::Gun,
    player::{Player, PlayerState},
    state::GameState,
    CursorPosition,
};

pub struct AnimationPlugin;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animation_timer_tick,
                animate_player,
                animate_enemy,
                flip_gun_sprite_y,
                flip_player_sprite_x,
                flip_enemy_sprite_x,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn animation_timer_tick(
    time: Res<Time>,
    mut query: Query<&mut AnimationTimer, With<AnimationTimer>>,
) {
    for mut timer in query.iter_mut() {
        timer.tick(time.delta());
    }
}

fn animate_player(
    mut player_query: Query<(&mut Sprite, &PlayerState, &AnimationTimer), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let Ok((mut sprite, state, timer)) = player_query.single_mut() else { return; };
    if timer.just_finished() {
        if let Some(ref mut atlas) = sprite.texture_atlas {
            let base_sprite_index = match state {
                PlayerState::Idle => 0,
                PlayerState::Run => 4,
            };
            atlas.index = base_sprite_index + (atlas.index + 1) % 4;
        }
    }
}

fn animate_enemy(
    mut enemy_query: Query<(&mut Sprite, &AnimationTimer, &EnemyType), With<Enemy>>,
) {
    if enemy_query.is_empty() {
        return;
    }

    for (mut sprite, timer, enemy_type) in enemy_query.iter_mut() {
        if timer.just_finished() {
            if let Some(ref mut atlas) = sprite.texture_atlas {
                atlas.index = enemy_type.get_base_sprite_index() + (atlas.index + 1) % 4;
            }
        }
    }
}

fn flip_player_sprite_x(
    cursor_position: Res<CursorPosition>,
    mut player_query: Query<(&mut Sprite, &Transform), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let Ok((mut sprite, transform)) = player_query.single_mut() else { return; };
    if let Some(cursor_position) = cursor_position.0 {
        if cursor_position.x > transform.translation.x {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}

fn flip_enemy_sprite_x(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Sprite, &Transform), With<Enemy>>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let Ok(player_transform) = player_query.single() else { return; };
    let player_pos = player_transform.translation;
    for (mut sprite, transform) in enemy_query.iter_mut() {
        if transform.translation.x < player_pos.x {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}

fn flip_gun_sprite_y(
    cursor_position: Res<CursorPosition>,
    mut gun_query: Query<(&mut Sprite, &Transform), With<Gun>>,
) {
    if gun_query.is_empty() {
        return;
    }

    let Ok((mut sprite, transform)) = gun_query.single_mut() else { return; };
    if let Some(cursor_position) = cursor_position.0 {
        if cursor_position.x > transform.translation.x {
            sprite.flip_y = false;
        } else {
            sprite.flip_y = true;
        }
    }
}
