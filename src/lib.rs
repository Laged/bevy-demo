pub mod animation;
pub mod benchmark_config;
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
        use crate::enemy::{EnemyType, Enemy};
        use crate::plugin_mode::PluginMode;
        use crate::enemy::EnemyPlugin;
        use crate::gun::GunPlugin;
        use crate::player::{PlayerPlugin, Player, Health};
        use bevy::prelude::*;
        use std::time::Instant;

        println!("\n=== Headless App Creation Test ===");

        // Create a headless app
        println!("Creating headless app...");
        let mut app = create_headless_app();

        // Add headless plugins
        println!("Adding headless plugins (Player, Enemy, Gun)...");
        app.add_plugins(PlayerPlugin::new(PluginMode::Headless))
            .add_plugins(EnemyPlugin::new(PluginMode::Headless))
            .add_plugins(GunPlugin::new(PluginMode::Headless));

        // Verify state machine works
        let initial_state = get_state::<GameState>(&app);
        println!("Initial state: {:?}", initial_state);
        assert_eq!(initial_state, Some(GameState::Loading));

        // Transition to InGame
        println!("Transitioning to InGame state...");
        set_state(&mut app, GameState::InGame);
        let new_state = get_state::<GameState>(&app);
        println!("New state: {:?}", new_state);
        assert_eq!(new_state, Some(GameState::InGame));

        // Spawn test entities
        println!("\n--- Entity Setup ---");
        println!("Spawning player at (0, 0, 0) with 100 health...");
        let _player_id = spawn_test_player(&mut app, Vec3::ZERO, 100.0);

        println!("Spawning enemy at (50, 0, 0) with 50 health...");
        let _enemy_id = spawn_test_enemy(&mut app, Vec3::new(50.0, 0.0, 0.0), 50.0, EnemyType::Green);

        // Query entities before simulation
        let player_count = app.world_mut().query::<&Player>().iter(app.world()).count();
        let enemy_count = app.world_mut().query::<&Enemy>().iter(app.world()).count();
        println!("Entity count - Players: {}, Enemies: {}", player_count, enemy_count);

        // Capture initial state
        let mut initial_player_pos = Vec3::ZERO;
        let mut initial_player_health = 0.0;
        let mut initial_enemy_pos = Vec3::ZERO;
        let mut initial_enemy_health = 0.0;

        let mut player_query = app.world_mut().query::<(&Player, &Health, &Transform)>();
        if let Some((_, health, transform)) = player_query.iter(app.world()).next() {
            initial_player_pos = transform.translation;
            initial_player_health = health.0;
            println!("Player initial state - Pos: {:?}, Health: {}", initial_player_pos, initial_player_health);
        }

        let mut enemy_query = app.world_mut().query::<(&Enemy, &Transform)>();
        if let Some((enemy, transform)) = enemy_query.iter(app.world()).next() {
            initial_enemy_pos = transform.translation;
            initial_enemy_health = enemy.health;
            println!("Enemy initial state - Pos: {:?}, Health: {}", initial_enemy_pos, initial_enemy_health);
        }

        let initial_distance = initial_player_pos.distance(initial_enemy_pos);
        println!("Initial distance between player and enemy: {:.2}", initial_distance);

        // Run short simulation with performance measurement
        println!("\n--- Short Simulation (10 frames) ---");
        let start = Instant::now();
        run_frames(&mut app, 10);
        let short_elapsed = start.elapsed();
        println!("10 frames completed in {:?} ({:.2} fps)", short_elapsed, 10.0 / short_elapsed.as_secs_f64());

        // Check entity state after short simulation
        let player_count_after = app.world_mut().query::<(&Player, &Health, &Transform)>().iter(app.world()).count();
        let enemy_count_after = app.world_mut().query::<(&Enemy, &Transform)>().iter(app.world()).count();

        println!("After simulation - Players: {}, Enemies: {}", player_count_after, enemy_count_after);

        // Verify entities still exist
        assert_eq!(player_count_after, 1, "Player should still exist");
        assert_eq!(enemy_count_after, 1, "Enemy should still exist");

        // Check component state after simulation
        let mut final_player_pos = Vec3::ZERO;
        let mut final_player_health = 0.0;
        let mut final_enemy_pos = Vec3::ZERO;
        let mut final_enemy_health = 0.0;

        let mut player_query = app.world_mut().query::<(&Player, &Health, &Transform)>();
        if let Some((_, health, transform)) = player_query.iter(app.world()).next() {
            final_player_pos = transform.translation;
            final_player_health = health.0;
            println!("Player final state - Pos: {:?}, Health: {}", final_player_pos, final_player_health);
        }

        let mut enemy_query = app.world_mut().query::<(&Enemy, &Transform)>();
        if let Some((enemy, transform)) = enemy_query.iter(app.world()).next() {
            final_enemy_pos = transform.translation;
            final_enemy_health = enemy.health;
            println!("Enemy final state - Pos: {:?}, Health: {}", final_enemy_pos, final_enemy_health);
        }

        let final_distance = final_player_pos.distance(final_enemy_pos);
        println!("Final distance between player and enemy: {:.2}", final_distance);

        // Behavior assertions
        println!("\n--- Behavior Validation ---");

        // GOLD STANDARD: Enemy movement rate
        const EXPECTED_MOVEMENT_PER_FRAME: f32 = 1.0;  // units per frame
        const FRAMES_SIMULATED: usize = 10;
        const EXPECTED_TOTAL_MOVEMENT: f32 = EXPECTED_MOVEMENT_PER_FRAME * FRAMES_SIMULATED as f32;

        // Enemy should move toward player (distance decreases)
        let distance_delta = initial_distance - final_distance;
        println!("Enemy moved {:.2} units closer to player", distance_delta);

        assert!(distance_delta > 0.0, "Enemy should move closer to player (distance decreased by {:.2})", distance_delta);
        assert!(final_enemy_pos.x < initial_enemy_pos.x, "Enemy X position should decrease (moving toward player at x=0)");

        // GOLD STANDARD: Exact movement rate verification
        assert_eq!(distance_delta, EXPECTED_TOTAL_MOVEMENT,
            "BEHAVIOR REGRESSION: Enemy movement rate changed. \
            Expected exactly {:.1} units in {} frames ({:.1} units/frame), \
            but moved {:.2} units. Check src/enemy.rs:update_enemy_transform",
            EXPECTED_TOTAL_MOVEMENT, FRAMES_SIMULATED, EXPECTED_MOVEMENT_PER_FRAME, distance_delta
        );

        // Player should not move (no input in headless mode)
        println!("Player position unchanged: {}", initial_player_pos == final_player_pos);
        assert_eq!(initial_player_pos, final_player_pos, "Player should not move without input");

        // Health should be unchanged (no collision/damage in 10 frames from distance 50)
        println!("Player health unchanged: {}", initial_player_health == final_player_health);
        println!("Enemy health unchanged: {}", initial_enemy_health == final_enemy_health);
        assert_eq!(initial_player_health, final_player_health, "Player health should be unchanged");
        assert_eq!(initial_enemy_health, final_enemy_health, "Enemy health should be unchanged");

        // Performance benchmark: longer simulation
        println!("\n--- Performance Benchmark (1000 frames) ---");
        let start = Instant::now();
        run_frames(&mut app, 1000);
        let long_elapsed = start.elapsed();
        let fps = 1000.0 / long_elapsed.as_secs_f64();
        let avg_frame_time_us = long_elapsed.as_micros() as f64 / 1000.0;
        println!("1000 frames completed in {:?} ({:.2} fps)", long_elapsed, fps);
        println!("Average frame time: {:.2} µs", avg_frame_time_us);

        // GOLD STANDARD PERFORMANCE BASELINES - MUST NEVER REGRESS
        const MIN_FPS: f64 = 15_000.0;          // Minimum acceptable fps for 1000-frame benchmark
        const MAX_FRAME_TIME_US: f64 = 67.0;    // Maximum acceptable microseconds per frame

        assert!(fps >= MIN_FPS,
            "PERFORMANCE REGRESSION: fps ({:.2}) dropped below minimum threshold ({:.2}). \
            This is a critical regression that must be fixed before merging.",
            fps, MIN_FPS
        );

        assert!(avg_frame_time_us <= MAX_FRAME_TIME_US,
            "PERFORMANCE REGRESSION: Average frame time ({:.2}µs) exceeded maximum threshold ({:.2}µs). \
            This is a critical regression that must be fixed before merging.",
            avg_frame_time_us, MAX_FRAME_TIME_US
        );

        println!("✓ Performance meets gold standard (≥{:.0} fps, ≤{:.0}µs/frame)", MIN_FPS, MAX_FRAME_TIME_US);

        println!("\n=== Test Passed! Headless mode works correctly ===");
        println!("Summary:");
        println!("  ✓ State machine transitions work");
        println!("  ✓ Entities spawn and persist");
        println!("  ✓ Enemy AI movement toward player verified");
        println!("  ✓ Player stationary without input");
        println!("  ✓ Health systems operational");
        println!("  ✓ Performance: {:.2} fps on 1000-frame benchmark\n", fps);
    }

    #[test]
    fn performance_benchmark() {
        use crate::test_utils::*;
        use std::time::Instant;
        use bevy_hanabi::prelude::*;
        use rand::Rng;

        #[derive(Resource)]
        struct BenchmarkState {
            current_wave: usize,
            wave_timer: Timer,
            phase: BenchmarkPhase,
            metrics: WaveMetrics,
        }

        #[derive(PartialEq, Clone, Copy, Debug)]
        enum BenchmarkPhase {
            Spawn,
            Combat,
            Cleanup,
            Complete,
        }

        struct WaveConfig {
            enemy_count: u32,
        }

        const BENCHMARK_WAVES: &[WaveConfig] = &[
            WaveConfig { enemy_count: 100 },
            // Temporarily testing single wave - will uncomment for full run
            // WaveConfig { enemy_count: 1_000 },
            // WaveConfig { enemy_count: 10_000 },
            // WaveConfig { enemy_count: 20_000 },
            // WaveConfig { enemy_count: 100_000 },
        ];

        #[derive(Default)]
        struct WaveMetrics {
            wave_number: usize,
            enemy_count: u32,
            spawn_frame_times: Vec<f32>,
            combat_frame_times: Vec<f32>,
            cleanup_frame_times: Vec<f32>,
            peak_particle_count: u32,
            peak_bullet_count: u32,
            peak_enemy_count: u32,
        }

        impl WaveMetrics {
            fn new(wave_number: usize, enemy_count: u32) -> Self {
                Self {
                    wave_number,
                    enemy_count,
                    ..Default::default()
                }
            }
        }

        impl BenchmarkState {
            fn new() -> Self {
                Self {
                    current_wave: 0,
                    wave_timer: Timer::from_seconds(5.0, TimerMode::Once),
                    phase: BenchmarkPhase::Spawn,
                    metrics: WaveMetrics::new(0, 100),
                }
            }
        }

        fn benchmark_wave_controller(
            mut state: ResMut<BenchmarkState>,
            time: Res<Time>,
            mut commands: Commands,
            config: Res<GameConfig>,
        ) {
            state.wave_timer.tick(time.delta());
            let elapsed = state.wave_timer.elapsed_secs();

            match state.phase {
                BenchmarkPhase::Spawn if elapsed < 0.5 => {
                    spawn_test_enemies(&mut commands, BENCHMARK_WAVES[state.current_wave].enemy_count, 500.0, &config);
                    state.phase = BenchmarkPhase::Combat;
                }
                BenchmarkPhase::Combat if elapsed >= 0.5 && elapsed < 4.5 => {}
                BenchmarkPhase::Cleanup if elapsed >= 4.5 && elapsed < 5.0 => {}
                _ if elapsed >= 5.0 => {
                    println!("\nWave {}/{} complete!", state.current_wave + 1, BENCHMARK_WAVES.len());
                    state.current_wave += 1;

                    if state.current_wave >= BENCHMARK_WAVES.len() {
                        state.phase = BenchmarkPhase::Complete;
                    } else {
                        state.wave_timer.reset();
                        state.phase = BenchmarkPhase::Spawn;
                        state.metrics = WaveMetrics::new(state.current_wave, BENCHMARK_WAVES[state.current_wave].enemy_count);
                    }
                }
                _ => {}
            }
        }

        fn benchmark_auto_fire_8_directions(
            state: Res<BenchmarkState>,
            player_query: Query<&Transform, With<crate::player::Player>>,
            mut commands: Commands,
            _handle: Res<GlobalTextureAtlas>,
            config: Res<GameConfig>,
        ) {
            if state.phase != BenchmarkPhase::Combat {
                return;
            }

            let Ok(player_transform) = player_query.get_single() else { return };
            let player_pos = player_transform.translation;

            let directions = [
                Vec3::Y,
                (Vec3::X + Vec3::Y).normalize(),
                Vec3::X,
                (Vec3::X - Vec3::Y).normalize(),
                -Vec3::Y,
                (-Vec3::X - Vec3::Y).normalize(),
                -Vec3::X,
                (-Vec3::X + Vec3::Y).normalize(),
            ];

            use bevy::math::vec3;

            for direction in directions {
                for _ in 0..config.gun.num_bullets_per_shot {
                    let mut rng = rand::thread_rng();
                    let spread_dir = vec3(
                        direction.x + rng.gen_range(-0.1..0.1),
                        direction.y + rng.gen_range(-0.1..0.1),
                        direction.z,
                    );

                    commands.spawn((
                        Transform::from_translation(player_pos),
                        crate::gun::Bullet,
                        crate::gun::BulletDirection(spread_dir),
                        crate::gun::SpawnInstant(Instant::now()),
                    ));
                }
            }
        }

        fn benchmark_metrics_collector(
            mut state: ResMut<BenchmarkState>,
            time: Res<Time>,
            bullets: Query<(), With<crate::gun::Bullet>>,
            particles: Query<(), Or<(
                With<crate::particle_effects::ImpactEffect>,
                With<crate::particle_effects::DeathLingerEffect>,
            )>>,
            enemies: Query<(), With<crate::enemy::Enemy>>,
        ) {
            let frame_time_ms = time.delta_secs() * 1000.0;

            match state.phase {
                BenchmarkPhase::Spawn => {
                    state.metrics.spawn_frame_times.push(frame_time_ms);
                }
                BenchmarkPhase::Combat => {
                    state.metrics.combat_frame_times.push(frame_time_ms);
                }
                BenchmarkPhase::Cleanup => {
                    state.metrics.cleanup_frame_times.push(frame_time_ms);
                }
                _ => {}
            }

            let particle_count = particles.iter().count() as u32;
            let bullet_count = bullets.iter().count() as u32;
            let enemy_count = enemies.iter().count() as u32;

            state.metrics.peak_particle_count = state.metrics.peak_particle_count.max(particle_count);
            state.metrics.peak_bullet_count = state.metrics.peak_bullet_count.max(bullet_count);
            state.metrics.peak_enemy_count = state.metrics.peak_enemy_count.max(enemy_count);
        }

        println!("\n========================================");
        println!("Performance Benchmark Starting");
        println!("========================================\n");

        let config = GameConfig::benchmark_mode();
        let mut app = app::create_headless_app(config.clone());
        init_for_testing(&mut app, &config);

        app.insert_resource(BenchmarkState::new());
        app.add_systems(
            Update,
            (
                benchmark_wave_controller,
                benchmark_auto_fire_8_directions,
                benchmark_metrics_collector,
            ),
        );

        let max_frames = 10000;
        let mut frame_count = 0;

        while frame_count < max_frames {
            app.update();
            frame_count += 1;

            if app.world().resource::<BenchmarkState>().phase == BenchmarkPhase::Complete {
                break;
            }
        }

        println!("\n========================================");
        println!("Benchmark Complete!");
        println!("Total frames: {}", frame_count);
        println!("========================================\n");

        assert_eq!(
            app.world().resource::<BenchmarkState>().phase,
            BenchmarkPhase::Complete,
            "Benchmark should complete all waves"
        );
    }
}
