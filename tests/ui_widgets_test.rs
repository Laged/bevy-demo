//! Headless tests for UI widgets (pause menu and settings panel)

use bevy::prelude::*;
use hell_game::test_utils::*;
use hell_game::core::events::{PauseMenuToggled, SettingsChanged};
use hell_game::core::state::GameState;
use hell_game::domains::ui::pause_menu::PauseMenuPlugin;
use hell_game::domains::ui::settings_panel::{SettingsPanelPlugin, GameSettings};
use hell_game::plugin_mode::PluginMode;
use hell_game::GameConfig;

#[test]
fn test_pause_menu_toggle_event_headless() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    app.add_plugins(PauseMenuPlugin::new(PluginMode::Headless));

    // Initialize to InGame state
    app.world_mut().resource_mut::<NextState<GameState>>().set(GameState::InGame);
    app.update();

    // Trigger pause toggle
    app.world_mut().send_event(PauseMenuToggled);
    app.update();

    // Should transition to MainMenu (pause state)
    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state.get(), GameState::MainMenu, "Pause should transition to MainMenu");

    // Toggle again
    app.world_mut().send_event(PauseMenuToggled);
    app.update();

    // Should transition back to InGame
    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state.get(), GameState::InGame, "Unpause should transition back to InGame");
}

#[test]
fn test_settings_changed_event_updates_resource() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    app.add_plugins(SettingsPanelPlugin::new(PluginMode::Headless));

    init_for_testing(&mut app, &config);

    // Initial settings should be default
    {
        let settings = app.world().resource::<GameSettings>();
        assert_eq!(settings.volume, 0.5);
        assert_eq!(settings.difficulty, 1.0);
        assert_eq!(settings.particle_count, 100);
    }

    // Trigger settings change
    app.world_mut().send_event(SettingsChanged {
        volume: 0.8,
        difficulty: 1.5,
        particle_count: 150,
    });
    app.update();

    // Verify settings resource updated
    {
        let settings = app.world().resource::<GameSettings>();
        assert_eq!(settings.volume, 0.8, "Volume should update");
        assert_eq!(settings.difficulty, 1.5, "Difficulty should update");
        assert_eq!(settings.particle_count, 150, "Particle count should update");
    }
}

#[test]
fn test_ui_widgets_performance_baseline() {
    let config = GameConfig::benchmark_mode();
    let mut app = create_headless_app(config.clone());
    app.add_plugins((
        PauseMenuPlugin::new(PluginMode::Headless),
        SettingsPanelPlugin::new(PluginMode::Headless),
    ));
    init_for_testing(&mut app, &config);

    // Run 1000 frames with UI widget systems active
    let start = std::time::Instant::now();
    run_frames(&mut app, 1000);
    let elapsed = start.elapsed();

    let fps = 1000.0 / elapsed.as_secs_f32();
    println!("UI Widgets FPS: {:.2}", fps);

    // Must maintain baseline
    assert!(fps >= 15000.0, "UI widgets must maintain ≥15k fps baseline, got {:.2}", fps);
}
