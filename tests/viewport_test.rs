//! Headless tests for ViewportNode minimap system

use bevy::prelude::*;
use hell_game::test_utils::*;
use hell_game::domains::ui::minimap::{MinimapPlugin, MinimapCamera, MinimapUI};
use hell_game::core::events::MinimapCameraReady;
use hell_game::entities::player::Player;
use hell_game::plugin_mode::PluginMode;
use hell_game::GameConfig;

#[test]
fn test_minimap_camera_created_in_headless() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    app.add_plugins(MinimapPlugin::new(PluginMode::Headless));
    init_for_testing(&mut app, &config);

    // Update to trigger minimap camera setup
    app.update();

    // Verify minimap camera exists
    let camera_count = app.world()
        .query::<&MinimapCamera>()
        .iter(app.world())
        .count();

    assert_eq!(camera_count, 1, "Minimap camera should be created in headless mode");
}

#[test]
fn test_minimap_camera_ready_event_fired() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    app.add_plugins(MinimapPlugin::new(PluginMode::Headless));

    let mut event_fired = false;
    let mut camera_entity_from_event = None;

    // Add event observer
    app.add_systems(Update, move |mut events: EventReader<MinimapCameraReady>| {
        for event in events.read() {
            event_fired = true;
            camera_entity_from_event = Some(event.camera_entity);
        }
    });

    init_for_testing(&mut app, &config);
    app.update();

    // Verify event was fired with valid entity
    assert!(event_fired, "MinimapCameraReady event should be fired");
    assert!(camera_entity_from_event.is_some(), "Event should contain camera entity");
}

#[test]
fn test_minimap_camera_follows_player() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    app.add_plugins(MinimapPlugin::new(PluginMode::Standard)); // Standard mode has update system
    init_for_testing(&mut app, &config);

    // Spawn player at specific position
    let player_entity = app.world_mut().spawn((
        Player,
        Transform::from_translation(Vec3::new(500.0, 300.0, 0.0)),
    )).id();

    app.update();

    // Get minimap camera position
    let camera_query = app.world().query::<(&Transform, &MinimapCamera)>();
    let camera_transform = camera_query
        .iter(app.world())
        .next()
        .map(|(t, _)| t.translation);

    if let Some(camera_pos) = camera_transform {
        assert!(
            (camera_pos.x - 500.0).abs() < 1.0,
            "Minimap camera should follow player X position"
        );
        assert!(
            (camera_pos.y - 300.0).abs() < 1.0,
            "Minimap camera should follow player Y position"
        );
    }

    // Move player
    if let Some(mut player_transform) = app.world_mut().get_mut::<Transform>(player_entity) {
        player_transform.translation = Vec3::new(1000.0, 800.0, 0.0);
    }

    app.update();

    // Camera should follow
    let camera_query = app.world().query::<(&Transform, &MinimapCamera)>();
    let camera_transform = camera_query
        .iter(app.world())
        .next()
        .map(|(t, _)| t.translation);

    if let Some(camera_pos) = camera_transform {
        assert!(
            (camera_pos.x - 1000.0).abs() < 1.0,
            "Minimap camera should track player movement X"
        );
        assert!(
            (camera_pos.y - 800.0).abs() < 1.0,
            "Minimap camera should track player movement Y"
        );
    }
}

#[test]
fn test_viewport_system_performance_baseline() {
    let config = GameConfig::benchmark_mode();
    let mut app = create_headless_app(config.clone());
    app.add_plugins(MinimapPlugin::new(PluginMode::Headless));
    init_for_testing(&mut app, &config);

    // Run 1000 frames with viewport system active
    let start = std::time::Instant::now();
    run_frames(&mut app, 1000);
    let elapsed = start.elapsed();

    let fps = 1000.0 / elapsed.as_secs_f32();
    println!("Viewport System FPS: {:.2}", fps);

    // Must maintain baseline
    assert!(fps >= 15000.0, "Viewport system must maintain ≥15k fps baseline, got {:.2}", fps);
}
