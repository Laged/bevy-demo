// Starting point: https://github.com/bones-ai/bevy-2d-shooter
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_hanabi::HanabiPlugin;

// Core infrastructure
use hell_game::core::{GameState, CollisionPlugin, ResourcesPlugin};

// Entity plugins
use hell_game::entities::{PlayerPlugin, EnemyPlugin, WorldPlugin};

// Domain plugins
use hell_game::domains::ui::{
    hud::GuiPlugin,
    camera::FollowCameraPlugin,
    minimap::MinimapPlugin,
    pause_menu::PauseMenuPlugin,
    settings_panel::SettingsPanelPlugin,
};
use hell_game::domains::gameplay::combat::GunPlugin;
use hell_game::domains::graphics::{
    animation::AnimationPlugin,
    particles::ParticleEffectsPlugin,
    tilemap::TilemapPlugin,
};
use hell_game::plugin_mode::PluginMode;

// Re-export config for convenience
use hell_game::GameConfig;

fn main() {
    // Load configuration
    let config = GameConfig::load_or_default();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // mode: bevy::window::WindowMode::Fullscreen,
                        resizable: true,
                        focused: true,
                        resolution: WindowResolution::new(
                            config.window.width as u32,
                            config.window.height as u32
                        ),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(HanabiPlugin)
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb_u8(
            config.colors.bg_color[0],
            config.colors.bg_color[1],
            config.colors.bg_color[2],
        )))
        .insert_resource(config)
        // Core systems
        .add_plugins(ResourcesPlugin)
        .add_plugins(CollisionPlugin)
        // Entities
        .add_plugins(PlayerPlugin::default())
        .add_plugins(EnemyPlugin::default())
        .add_plugins(WorldPlugin)
        // Graphics
        .add_plugins(AnimationPlugin)
        .add_plugins(ParticleEffectsPlugin)
        .add_plugins(TilemapPlugin::new(PluginMode::Standard))  // Bevy 0.17: Colored arena zones
        // Gameplay
        .add_plugins(GunPlugin::default())
        // UI
        .add_plugins(FollowCameraPlugin)
        .add_plugins(GuiPlugin)
        .add_plugins(MinimapPlugin::new(PluginMode::Standard))  // Bevy 0.17: ViewportNode minimap
        .add_plugins(PauseMenuPlugin::new(PluginMode::Standard))  // Bevy 0.17: Pause menu
        .add_plugins(SettingsPanelPlugin::new(PluginMode::Standard))  // Bevy 0.17: Settings panel
        .run();
}
