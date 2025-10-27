//! Player entity definition - Owned by Gameplay Agent
//!
//! Player component, health, state (Idle/Run), movement, and collision handling

use bevy::math::vec3;
use bevy::prelude::*;

use crate::state::GameState;
use crate::plugin_mode::PluginMode;

pub struct PlayerPlugin {
    mode: PluginMode,
}

impl PlayerPlugin {
    pub fn new(mode: PluginMode) -> Self {
        Self { mode }
    }
}

impl Default for PlayerPlugin {
    fn default() -> Self {
        Self::new(PluginMode::default())
    }
}

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Run,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Logic systems (always run)
        app.add_systems(
            Update,
            handle_player_death.run_if(in_state(GameState::InGame)),
        );

        // Input handling (only in Standard mode - requires keyboard)
        if self.mode == PluginMode::Standard {
            app.add_systems(
                Update,
                handle_player_input.run_if(in_state(GameState::InGame)),
            );
        }
    }
}


fn handle_player_death(
    player_query: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player_query.is_empty() {
        return;
    }
    let Ok(health) = player_query.single() else { return; };
    if health.0 <= 0.0 {
        next_state.set(GameState::MainMenu);
    }
}

fn handle_player_input(
    mut player_query: Query<(&mut Transform, &mut PlayerState), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    config: Res<crate::config_loader::GameConfig>,
) {
    if player_query.is_empty() {
        return;
    }

    let Ok((mut transform, mut player_state)) = player_query.single_mut() else { return; };
    let w_key = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let a_key = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    let s_key = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let d_key =
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);

    let mut delta = Vec2::ZERO;
    if w_key {
        delta.y += 1.0;
    }
    if s_key {
        delta.y -= 1.0;
    }
    if a_key {
        delta.x -= 1.0;
    }
    if d_key {
        delta.x += 1.0;
    }
    delta = delta.normalize();

    if delta.is_finite() && (w_key || a_key || s_key || d_key) {
        transform.translation += vec3(delta.x, delta.y, 0.0) * config.player.speed;
        transform.translation.z = 10.0;
        *player_state = PlayerState::Run;
    } else {
        *player_state = PlayerState::Idle;
    }
}
