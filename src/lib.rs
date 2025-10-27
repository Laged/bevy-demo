pub mod animation;
pub mod camera;
pub mod collision;
pub mod config_loader;
pub mod configs;
pub mod enemy;
pub mod gui;
pub mod gun;
pub mod particle_effects;
pub mod player;
pub mod plugin_mode;
pub mod resources;
pub mod state;
pub mod world;

pub use config_loader::GameConfig;
pub use configs::*;
pub use resources::*;

#[cfg(test)]
pub mod test_utils;

#[cfg(test)]
mod tests {
    use crate::state::GameState;

    #[test]
    fn test_game_state_enum_exists() {
        // Smoke test: verify GameState enum has expected variants
        let _loading = GameState::Loading;
        let _menu = GameState::MainMenu;
        let _init = GameState::GameInit;
        let _in_game = GameState::InGame;
        assert!(true);
    }

    #[test]
    fn test_configs_are_valid() {
        use crate::*;

        // Verify window configuration is positive
        assert!(WW > 0.0, "Window width should be positive");
        assert!(WH > 0.0, "Window height should be positive");

        // Verify game configuration constants are reasonable
        assert!(PLAYER_HEALTH > 0.0, "Player health should be positive");
        assert!(PLAYER_SPEED > 0.0, "Player speed should be positive");
        assert!(ENEMY_HEALTH > 0.0, "Enemy health should be positive");
        assert!(BULLET_DAMAGE > 0.0, "Bullet damage should be positive");
        assert!(MAX_NUM_ENEMIES > 0, "Max enemies should be positive");
        assert!(SPRITE_SCALE_FACTOR > 0.0, "Sprite scale should be positive");
    }

    #[test]
    fn test_tile_configuration() {
        use crate::*;

        // Verify sprite sheet configuration
        assert!(TILE_W > 0, "Tile width should be positive");
        assert!(TILE_H > 0, "Tile height should be positive");
        assert!(SPRITE_SHEET_W > 0, "Sprite sheet width should be positive");
        assert!(SPRITE_SHEET_H > 0, "Sprite sheet height should be positive");

        // Verify reasonable dimensions
        assert!(TILE_W <= 256, "Tile width seems unreasonable");
        assert!(TILE_H <= 256, "Tile height seems unreasonable");
    }

    #[test]
    fn test_timing_configuration() {
        use crate::*;

        // Verify timing values are reasonable (not zero or negative)
        assert!(BULLET_TIME_SECS > 0.0, "Bullet lifetime should be positive");
        assert!(BULLET_SPAWN_INTERVAL > 0.0, "Bullet spawn interval should be positive");
        assert!(ENEMY_SPAWN_INTERVAL > 0.0, "Enemy spawn interval should be positive");
        assert!(KD_TREE_REFRESH_RATE > 0.0, "KD-tree refresh rate should be positive");
    }

    #[test]
    fn test_headless_app_creation() {
        use crate::test_utils::app::create_headless_app;
        use crate::test_utils::simulation::{set_state, get_state, run_frames};
        use crate::test_utils::entities::{spawn_test_player, spawn_test_enemy};
        use crate::enemy::EnemyType;
        use crate::plugin_mode::PluginMode;
        use crate::enemy::EnemyPlugin;
        use crate::gun::GunPlugin;
        use crate::player::PlayerPlugin;
        use bevy::prelude::*;

        // Create a headless app
        let mut app = create_headless_app();

        // Add headless plugins
        app.add_plugins(PlayerPlugin::new(PluginMode::Headless))
            .add_plugins(EnemyPlugin::new(PluginMode::Headless))
            .add_plugins(GunPlugin::new(PluginMode::Headless));

        // Verify state machine works
        assert_eq!(get_state::<GameState>(&app), Some(GameState::Loading));

        // Transition to InGame
        set_state(&mut app, GameState::InGame);
        assert_eq!(get_state::<GameState>(&app), Some(GameState::InGame));

        // Spawn test entities
        let _player = spawn_test_player(&mut app, Vec3::ZERO, 100.0);
        let _enemy = spawn_test_enemy(&mut app, Vec3::new(50.0, 0.0, 0.0), 50.0, EnemyType::Green);

        // Run simulation frames without crashing
        run_frames(&mut app, 10);

        // If we got here, headless mode works!
        assert!(true);
    }
}
