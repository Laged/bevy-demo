//! Minimap system using Bevy 0.17 ViewportNode - Owned by UI Agent
//!
//! Renders a zoomed-out camera view into a UI corner for tactical overview

use bevy::prelude::*;
use bevy::ui::widget::ViewportNode;
use crate::core::state::GameState;
use crate::core::events::MinimapCameraReady;
use crate::entities::player::Player;
use crate::plugin_mode::PluginMode;

pub struct MinimapPlugin {
    mode: PluginMode,
}

impl MinimapPlugin {
    pub fn new(mode: PluginMode) -> Self {
        Self { mode }
    }
}

impl Default for MinimapPlugin {
    fn default() -> Self {
        Self::new(PluginMode::default())
    }
}

#[derive(Component)]
pub struct MinimapCamera;

#[derive(Component)]
pub struct MinimapUI;

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MinimapCameraReady>();

        // Both modes: create minimap camera entity (works in headless)
        app.add_systems(OnEnter(GameState::InGame), setup_minimap_camera);

        // Visual mode only: create UI rendering
        if self.mode == PluginMode::Standard {
            app.add_systems(OnEnter(GameState::InGame), setup_minimap_ui)
                .add_systems(Update, update_minimap_camera_position.run_if(in_state(GameState::InGame)));
        }
    }
}

fn setup_minimap_camera(mut commands: Commands) {
    // Create minimap camera (zoomed out 5x)
    let camera_entity = commands
        .spawn((
            Camera2d,
            Camera {
                // Lower order than main camera
                order: -1,
                // Don't clear, we want to see through to main camera
                clear_color: ClearColorConfig::None,
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, 100.0))
                .with_scale(Vec3::splat(0.2)), // 5x zoom out
            MinimapCamera,
        ))
        .id();

    // Emit event that minimap camera is ready
    commands.trigger(MinimapCameraReady { camera_entity });
}

fn setup_minimap_ui(
    mut commands: Commands,
    minimap_camera_query: Query<Entity, With<MinimapCamera>>,
) {
    let Ok(camera_entity) = minimap_camera_query.single() else {
        warn!("Minimap camera not found during UI setup");
        return;
    };

    // Create minimap UI node in bottom-right corner
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(10.0),
            bottom: Val::Px(10.0),
            width: Val::Px(200.0),
            height: Val::Px(200.0),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BorderColor::all(Color::srgb(0.8, 0.8, 0.8)),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        MinimapUI,
        // ViewportNode connects this UI node to the camera
        ViewportNode::new(camera_entity),
    ));
}

fn update_minimap_camera_position(
    player_query: Query<&Transform, With<Player>>,
    mut minimap_camera_query: Query<&mut Transform, (With<MinimapCamera>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let Ok(mut camera_transform) = minimap_camera_query.single_mut() else {
        return;
    };

    // Follow player position (keep Z and scale the same)
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
