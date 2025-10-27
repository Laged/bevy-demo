//! Headless app creation and configuration for testing

use bevy::prelude::*;

/// Creates a minimal headless Bevy app for testing game logic without rendering.
///
/// Returns an App configured with:
/// - MinimalPlugins (no rendering, windowing, or input)
/// - TimePlugin for frame timing
/// - Game state machine
///
/// # Example
/// ```ignore
/// let mut app = create_headless_app();
/// app.add_plugins(PlayerPlugin::new(PluginMode::Headless));
/// ```
pub fn create_headless_app() -> App {
    // TODO: Implement in later tasks
    todo!("Will be implemented in task 7")
}
