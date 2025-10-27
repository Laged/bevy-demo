///! Tests for tilemap arena zone system
///!
///! Validates:
///! - Tilemap zone creation in headless mode
///! - Zone assignment based on distance from center
///! - Performance impact (fps ≥ 15k baseline)

use bevy::prelude::*;
use hell_game::domains::graphics::tilemap::{ArenaZone, TilemapConfig, TilemapPlugin, TilemapTile};
use hell_game::domains::testing::harness::create_headless_app;
use hell_game::domains::testing::simulation::{run_frames, set_state};
use hell_game::plugin_mode::PluginMode;
use hell_game::state::GameState;

/// Test that tilemap zones are created in headless mode
#[test]
fn test_tilemap_zones_headless_creation() {
    let mut app = create_headless_app();

    // Add required plugins (skip WorldPlugin to avoid texture dependencies)
    app.add_plugins(TilemapPlugin::new(PluginMode::Headless));

    // Transition to GameInit to trigger tilemap setup
    set_state(&mut app, GameState::GameInit);
    app.update(); // Additional frame for OnEnter systems

    // Verify TilemapConfig resource exists
    assert!(app.world().get_resource::<TilemapConfig>().is_some());

    // Query for tilemap tiles
    let tile_count = app
        .world_mut()
        .query::<&TilemapTile>()
        .iter(app.world())
        .count();

    println!("Tilemap tiles created: {}", tile_count);

    // Verify tiles were created (should be hundreds based on world size)
    assert!(
        tile_count > 100,
        "Expected >100 tiles, got {}",
        tile_count
    );
}

/// Test that zone types are correctly assigned based on distance
#[test]
fn test_zone_assignment_by_distance() {
    let mut app = create_headless_app();

    app.add_plugins(TilemapPlugin::new(PluginMode::Headless));

    set_state(&mut app, GameState::GameInit);
    app.update();

    let tilemap_config = app
        .world()
        .get_resource::<TilemapConfig>()
        .expect("TilemapConfig should exist");

    let safe_radius = tilemap_config.safe_zone_radius;
    let danger_radius = tilemap_config.danger_zone_outer_radius;

    // Count tiles by zone
    let mut safe_count = 0;
    let mut danger_count = 0;
    let mut neutral_count = 0;

    for (tile, transform) in app
        .world_mut()
        .query::<(&TilemapTile, &Transform)>()
        .iter(app.world())
    {
        let distance = transform.translation.truncate().length();

        match tile.zone {
            ArenaZone::Safe => {
                assert!(distance < safe_radius, "Safe zone tile outside radius");
                safe_count += 1;
            }
            ArenaZone::Danger => {
                assert!(
                    distance >= safe_radius && distance < danger_radius,
                    "Danger zone tile outside radius range"
                );
                danger_count += 1;
            }
            ArenaZone::Neutral => {
                assert!(distance >= danger_radius, "Neutral zone tile inside danger radius");
                neutral_count += 1;
            }
        }
    }

    println!("Zone distribution:");
    println!("  Safe: {}", safe_count);
    println!("  Danger: {}", danger_count);
    println!("  Neutral: {}", neutral_count);

    // Verify all three zones have tiles
    assert!(safe_count > 0, "Safe zone should have tiles");
    assert!(danger_count > 0, "Danger zone should have tiles");
    assert!(neutral_count > 0, "Neutral zone should have tiles");
}

/// Test that tiles have GameEntity component for cleanup integration
#[test]
fn test_tilemap_cleanup_on_state_exit() {
    let mut app = create_headless_app();

    app.add_plugins(TilemapPlugin::new(PluginMode::Headless));

    set_state(&mut app, GameState::GameInit);
    app.update();

    // Verify tiles exist
    let tile_count = app
        .world_mut()
        .query::<&TilemapTile>()
        .iter(app.world())
        .count();
    assert!(tile_count > 0, "Tiles should exist after GameInit");

    // Verify all tiles have GameEntity component (enables WorldPlugin cleanup)
    let tiles_with_game_entity = app
        .world_mut()
        .query::<(&TilemapTile, &hell_game::entities::world::GameEntity)>()
        .iter(app.world())
        .count();

    println!(
        "Total tiles: {}, tiles with GameEntity: {}",
        tile_count, tiles_with_game_entity
    );

    assert_eq!(
        tiles_with_game_entity, tile_count,
        "All tilemap tiles should have GameEntity component for automatic cleanup"
    );
}

/// Performance benchmark: Verify tilemap doesn't break fps baseline
#[test]
fn test_tilemap_performance_impact() {
    let mut app = create_headless_app();

    app.add_plugins(TilemapPlugin::new(PluginMode::Headless));

    set_state(&mut app, GameState::GameInit);
    app.update();

    // Run 1000-frame simulation to measure performance
    let start = std::time::Instant::now();
    run_frames(&mut app, 1000);
    let elapsed = start.elapsed();

    let fps = 1000.0 / elapsed.as_secs_f64();
    let frame_time_us = elapsed.as_micros() as f64 / 1000.0;

    println!("=== Tilemap Performance Baseline ===");
    println!("1000 frames completed in {:?}", elapsed);
    println!("Effective FPS: {:.0}", fps);
    println!("Average frame time: {:.2}µs", frame_time_us);

    // Gold standard: ≥15,000 fps (≤67µs per frame)
    assert!(
        fps >= 15000.0,
        "Performance regression: fps={:.0} (required ≥15,000)",
        fps
    );
    assert!(
        frame_time_us <= 67.0,
        "Performance regression: frame_time={:.2}µs (required ≤67µs)",
        frame_time_us
    );

    println!("✅ Performance baseline maintained");
}

/// Verify grid alignment and positioning
#[test]
fn test_tile_grid_alignment() {
    let mut app = create_headless_app();

    app.add_plugins(TilemapPlugin::new(PluginMode::Headless));

    set_state(&mut app, GameState::GameInit);
    app.update();

    let tilemap_config = app
        .world()
        .get_resource::<TilemapConfig>()
        .expect("TilemapConfig should exist");

    let tile_size = tilemap_config.tile_size;

    // Check that all tiles are on grid points
    for (tile, transform) in app
        .world_mut()
        .query::<(&TilemapTile, &Transform)>()
        .iter(app.world())
    {
        let pos = transform.translation;

        // Verify position matches grid coordinates
        let expected_x = tile.grid_x as f32 * tile_size;
        let expected_y = tile.grid_y as f32 * tile_size;

        assert!(
            (pos.x - expected_x).abs() < 0.1,
            "Tile X position mismatch: expected {}, got {}",
            expected_x,
            pos.x
        );
        assert!(
            (pos.y - expected_y).abs() < 0.1,
            "Tile Y position mismatch: expected {}, got {}",
            expected_y,
            pos.y
        );

        // Verify Z-order (should be -2.0 to stay behind entities)
        assert_eq!(pos.z, -2.0, "Tile should be at Z=-2.0 (behind entities)");
    }

    println!("✅ All tiles properly aligned on grid");
}
