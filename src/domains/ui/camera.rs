//! Camera system - Owned by UI Agent
//!
//! Manages camera behavior:
//! - Following player
//! - Pan controls via bevy_pancam

use bevy::{math::vec3, prelude::*};
use bevy_pancam::{PanCam, PanCamPlugin};

use crate::player::Player;
use crate::state::GameState;

pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default())
            .add_systems(OnEnter(GameState::Loading), setup_camera)
            .add_systems(
                Update,
                camera_follow_player.run_if(in_state(GameState::InGame)),
            );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default()).insert(PanCam {
        grab_buttons: vec![],
        ..default()
    });
}

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if camera_query.is_empty() || player_query.is_empty() {
        return;
    }

    let Ok(mut camera_transform) = camera_query.single_mut() else { return; };
    let Ok(player_transform) = player_query.single() else { return; };
    let player_pos = player_transform.translation;
    let (x, y) = (player_pos.x, player_pos.y);

    camera_transform.translation = camera_transform.translation.lerp(vec3(x, y, 0.0), 0.1);
}
