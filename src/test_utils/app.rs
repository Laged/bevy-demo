//! Headless app creation and configuration for testing

use bevy::prelude::*;
use crate::state::GameState;
use crate::config_loader::GameConfig;

/// Creates a minimal headless Bevy app for testing game logic without rendering.
///
/// Returns an App configured with:
/// - MinimalPlugins (no rendering, windowing, or input)
/// - TimePlugin for frame timing (included in MinimalPlugins)
/// - Game state machine initialized to Loading state
/// - GameConfig resource loaded from config or defaults
///
/// # Example
/// ```ignore
/// use hell_game::test_utils::app::create_headless_app;
/// use hell_game::plugin_mode::PluginMode;
/// use hell_game::player::PlayerPlugin;
///
/// let mut app = create_headless_app();
/// app.add_plugins(PlayerPlugin::new(PluginMode::Headless));
/// ```
pub fn create_headless_app() -> App {
    let config = GameConfig::load_or_default();

    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_state::<GameState>()
        .insert_resource(config);

    app
}
