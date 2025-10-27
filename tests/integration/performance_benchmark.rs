use bevy::prelude::*;
use hell_game::test_utils::*;
use hell_game::config_loader::GameConfig;
use bevy_hanabi::prelude::*;
use rand::Rng;
use std::time::Instant;

#[derive(Resource)]
struct BenchmarkState {
    current_wave: usize,
    wave_timer: Timer,
    phase: BenchmarkPhase,
    metrics: WaveMetrics,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum BenchmarkPhase {
    Spawn,      // 0-0.5s
    Combat,     // 0.5-4.5s
    Cleanup,    // 4.5-5s
    Complete,   // All waves done
}

struct WaveConfig {
    enemy_count: u32,
}

const BENCHMARK_WAVES: &[WaveConfig] = &[
    WaveConfig { enemy_count: 100 },
    WaveConfig { enemy_count: 1_000 },
    WaveConfig { enemy_count: 10_000 },
    WaveConfig { enemy_count: 20_000 },
    WaveConfig { enemy_count: 100_000 },
];

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

// ===== WAVE CONTROLLER =====

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
            // Spawn enemies using test_utils
            spawn_test_enemies(
                &mut commands,
                BENCHMARK_WAVES[state.current_wave].enemy_count,
                500.0,
                &config,
            );
            state.phase = BenchmarkPhase::Combat;
        }
        BenchmarkPhase::Combat if elapsed >= 0.5 && elapsed < 4.5 => {
            // Auto-fire handled by separate system
        }
        BenchmarkPhase::Cleanup if elapsed >= 4.5 && elapsed < 5.0 => {
            // Measuring cleanup
        }
        _ if elapsed >= 5.0 => {
            // Wave complete
            println!("\nWave {}/{} complete!", state.current_wave + 1, BENCHMARK_WAVES.len());
            print_wave_results(&state.metrics);

            state.current_wave += 1;

            if state.current_wave >= BENCHMARK_WAVES.len() {
                state.phase = BenchmarkPhase::Complete;
            } else {
                state.wave_timer.reset();
                state.phase = BenchmarkPhase::Spawn;
                state.metrics = WaveMetrics::new(
                    state.current_wave,
                    BENCHMARK_WAVES[state.current_wave].enemy_count,
                );
            }
        }
        _ => {}
    }
}

// ===== AUTO-FIRE SYSTEM =====

fn benchmark_auto_fire_8_directions(
    state: Res<BenchmarkState>,
    player_query: Query<&Transform, With<hell_game::player::Player>>,
    mut commands: Commands,
    _handle: Res<hell_game::resources::GlobalTextureAtlas>,
    config: Res<GameConfig>,
) {
    if state.phase != BenchmarkPhase::Combat {
        return;
    }

    let Ok(player_transform) = player_query.get_single() else { return };
    let player_pos = player_transform.translation;

    // 8 directions
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
                hell_game::gun::Bullet,
                hell_game::gun::BulletDirection(spread_dir),
                hell_game::gun::SpawnInstant(Instant::now()),
            ));
        }
    }
}

// ===== METRICS COLLECTOR =====

fn benchmark_metrics_collector(
    mut state: ResMut<BenchmarkState>,
    time: Res<Time>,
    bullets: Query<(), With<hell_game::gun::Bullet>>,
    particles: Query<(), Or<(
        With<hell_game::particle_effects::ImpactEffect>,
        With<hell_game::particle_effects::DeathLingerEffect>,
    )>>,
    enemies: Query<(), With<hell_game::enemy::Enemy>>,
) {
    let frame_time_ms = time.delta_secs() * 1000.0;

    // Record for current phase
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

    // Track peaks
    let particle_count = particles.iter().count() as u32;
    let bullet_count = bullets.iter().count() as u32;
    let enemy_count = enemies.iter().count() as u32;

    state.metrics.peak_particle_count = state.metrics.peak_particle_count.max(particle_count);
    state.metrics.peak_bullet_count = state.metrics.peak_bullet_count.max(bullet_count);
    state.metrics.peak_enemy_count = state.metrics.peak_enemy_count.max(enemy_count);
}

// ===== OUTPUT FORMATTING =====

fn print_wave_results(metrics: &WaveMetrics) {
    let spawn_fps = calculate_avg_fps(&metrics.spawn_frame_times);
    let combat_fps = calculate_avg_fps(&metrics.combat_frame_times);
    let cleanup_fps = calculate_avg_fps(&metrics.cleanup_frame_times);

    println!("  Spawn:   {:.0} FPS avg", spawn_fps);
    println!("  Combat:  {:.0} FPS avg", combat_fps);
    println!("  Cleanup: {:.0} FPS avg", cleanup_fps);
    println!(
        "  Peak: {} enemies, {} bullets, {} particles",
        metrics.peak_enemy_count, metrics.peak_bullet_count, metrics.peak_particle_count
    );
}

fn calculate_avg_fps(frame_times_ms: &[f32]) -> f32 {
    if frame_times_ms.is_empty() {
        return 0.0;
    }
    let fps_samples: Vec<f32> = frame_times_ms.iter().map(|&ms| 1000.0 / ms).collect();
    fps_samples.iter().sum::<f32>() / fps_samples.len() as f32
}

// ===== TEST =====

#[test]
fn performance_benchmark() {
    println!("\n========================================");
    println!("Performance Benchmark Starting");
    println!("========================================\n");

    // Create headless app using test_utils (one line!)
    let config = GameConfig::benchmark_mode();
    let mut app = create_headless_app(config.clone());

    // Initialize to InGame state (one line!)
    init_for_testing(&mut app, &config);

    // Add benchmark-specific systems
    app.insert_resource(BenchmarkState::new());
    app.add_systems(
        Update,
        (
            benchmark_wave_controller,
            benchmark_auto_fire_8_directions,
            benchmark_metrics_collector,
        ),
    );

    // Run until complete
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
