# Performance Benchmark System Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a wave-based performance benchmark that measures FPS under realistic combat loads (100 to 100k enemies) with deterministic 5-second waves.

**Architecture:** Integration test in `tests/integration/` using headless Bevy app. Wave controller spawns enemies, auto-fire system shoots in 8 directions, metrics collector tracks FPS per phase. JSON output for regression tracking.

**Tech Stack:** Bevy 0.17.2, bevy_hanabi 0.17.0, Rust integration tests, headless ECS testing

---

## Prerequisites

- Particle effects system implemented (bullet trails, colored impacts, death explosions)
- Enemy spawning system functional
- Collision detection working (KD-tree based)
- Current working directory: `/home/laged/Codings/laged/bevy-demo`

## Task Overview

1. Create benchmark config module
2. Add test directory structure
3. Create benchmark state and metrics structs
4. Implement wave controller system
5. Implement 8-direction auto-fire system
6. Implement metrics collector system
7. Create test harness and Bevy app setup
8. Add results output (JSON + console)
9. Test with small wave (100 enemies)
10. Run full benchmark suite and document results

---

## Task 1: Create Benchmark Config Module

**Files:**
- Create: `src/benchmark_config.rs`
- Modify: `src/lib.rs`

**Step 1: Create benchmark config file**

Create `src/benchmark_config.rs`:

```rust
use crate::config_loader::GameConfig;

impl GameConfig {
    /// Returns a config optimized for performance benchmarking.
    /// Overpowered player to clear enemies quickly, no spawn rate limits.
    pub fn benchmark_mode() -> Self {
        let mut config = Self::default();

        // Overpowered player configuration
        config.gun.bullet_damage = 999999.0; // One-shot any enemy
        config.gun.num_bullets_per_shot = 50; // Dense bullet spread
        config.gun.bullet_spawn_interval = 0.0; // Fire every frame
        config.gun.bullet_speed = 30.0;
        config.gun.bullet_time_secs = 2.0;

        // Disable enemy spawn rate limiting
        config.enemy.spawn_interval = 0.0;
        config.enemy.max_num_enemies = 100_000;
        config.enemy.speed = 1.0;

        // Keep particle effects at current settings (what we're testing)
        // No changes to particle_effects config

        config
    }
}
```

**Step 2: Export from lib.rs**

Add to `src/lib.rs` at the end of module declarations:

```rust
pub mod benchmark_config;
```

**Step 3: Build and verify**

Run: `cargo build`
Expected: Success

**Step 4: Commit**

```bash
git add src/benchmark_config.rs src/lib.rs
git commit -m "feat: add benchmark config with overpowered player settings"
```

---

## Task 2: Add Test Directory Structure

**Files:**
- Create: `tests/integration/mod.rs`
- Create: `tests/integration/performance_benchmark.rs`

**Step 1: Create test module file**

Create `tests/integration/mod.rs`:

```rust
// Integration tests module
pub mod performance_benchmark;
```

**Step 2: Create benchmark test skeleton**

Create `tests/integration/performance_benchmark.rs`:

```rust
use bevy::prelude::*;

#[test]
fn performance_benchmark() {
    println!("Performance benchmark test placeholder");
    assert!(true);
}
```

**Step 3: Test structure**

Run: `cargo test --test performance_benchmark`
Expected: Test passes with "performance benchmark test placeholder" output

**Step 4: Commit**

```bash
git add tests/integration/
git commit -m "test: add performance benchmark test structure"
```

---

## Task 3: Create Benchmark State and Metrics Structs

**Files:**
- Modify: `tests/integration/performance_benchmark.rs`

**Step 1: Add benchmark state structs**

Add to `tests/integration/performance_benchmark.rs` before the test function:

```rust
use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource)]
struct BenchmarkState {
    current_wave: usize,
    wave_timer: Timer,
    phase: BenchmarkPhase,
    metrics: WaveMetrics,
    enemies_spawned: bool,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum BenchmarkPhase {
    Spawn,      // 0-0.5s
    Combat,     // 0.5-4.5s
    Cleanup,    // 4.5-5s
    Rest,       // 5s marker
    Complete,   // All waves done
}

struct WaveConfig {
    enemy_count: u32,
    duration_secs: f32,
}

const BENCHMARK_WAVES: &[WaveConfig] = &[
    WaveConfig { enemy_count: 100, duration_secs: 5.0 },
    WaveConfig { enemy_count: 1_000, duration_secs: 5.0 },
    WaveConfig { enemy_count: 10_000, duration_secs: 5.0 },
    WaveConfig { enemy_count: 20_000, duration_secs: 5.0 },
    WaveConfig { enemy_count: 100_000, duration_secs: 5.0 },
];

impl BenchmarkState {
    fn new() -> Self {
        Self {
            current_wave: 0,
            wave_timer: Timer::from_seconds(5.0, TimerMode::Once),
            phase: BenchmarkPhase::Spawn,
            metrics: WaveMetrics::new(0, 100),
            enemies_spawned: false,
        }
    }
}
```

**Step 2: Add metrics structs**

Add after BenchmarkState:

```rust
#[derive(Default)]
struct WaveMetrics {
    wave_number: usize,
    enemy_count: u32,

    // Frame timing samples
    spawn_frame_times: Vec<f32>,
    combat_frame_times: Vec<f32>,
    cleanup_frame_times: Vec<f32>,

    // Peak entity counts
    peak_particle_count: u32,
    peak_bullet_count: u32,
    peak_enemy_count: u32,
}

#[derive(Default, Debug)]
struct PhaseMetrics {
    min_fps: f32,
    max_fps: f32,
    avg_fps: f32,
    p95_fps: f32,
    p99_fps: f32,
}

impl WaveMetrics {
    fn new(wave_number: usize, enemy_count: u32) -> Self {
        Self {
            wave_number,
            enemy_count,
            ..Default::default()
        }
    }

    fn calculate_phase_metrics(&self, frame_times_ms: &[f32]) -> PhaseMetrics {
        if frame_times_ms.is_empty() {
            return PhaseMetrics::default();
        }

        let fps_samples: Vec<f32> = frame_times_ms
            .iter()
            .map(|&ms| 1000.0 / ms)
            .collect();

        let mut sorted = fps_samples.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let p95_idx = (sorted.len() as f32 * 0.95) as usize;
        let p99_idx = (sorted.len() as f32 * 0.99) as usize;

        PhaseMetrics {
            min_fps: *sorted.first().unwrap(),
            max_fps: *sorted.last().unwrap(),
            avg_fps: fps_samples.iter().sum::<f32>() / fps_samples.len() as f32,
            p95_fps: sorted[p95_idx.min(sorted.len() - 1)],
            p99_fps: sorted[p99_idx.min(sorted.len() - 1)],
        }
    }
}
```

**Step 3: Build and verify**

Run: `cargo test --test performance_benchmark`
Expected: Test compiles and passes

**Step 4: Commit**

```bash
git add tests/integration/performance_benchmark.rs
git commit -m "test: add benchmark state and metrics structures"
```

---

## Task 4: Implement Wave Controller System

**Files:**
- Modify: `tests/integration/performance_benchmark.rs`

**Step 1: Add wave controller system**

Add before the test function:

```rust
fn benchmark_wave_controller(
    mut state: ResMut<BenchmarkState>,
    time: Res<Time>,
    mut commands: Commands,
    handle: Res<hell_game::GlobalTextureAtlas>,
    config: Res<hell_game::GameConfig>,
) {
    state.wave_timer.tick(time.delta());
    let elapsed = state.wave_timer.elapsed_secs();

    match state.phase {
        BenchmarkPhase::Spawn if elapsed < 0.5 => {
            if !state.enemies_spawned {
                spawn_wave_enemies(
                    &mut commands,
                    &handle,
                    &config,
                    BENCHMARK_WAVES[state.current_wave].enemy_count,
                );
                state.enemies_spawned = true;
                state.phase = BenchmarkPhase::Combat;
            }
        }
        BenchmarkPhase::Combat if elapsed >= 0.5 && elapsed < 4.5 => {
            // Auto-fire handled by separate system
        }
        BenchmarkPhase::Cleanup if elapsed >= 4.5 && elapsed < 5.0 => {
            // Just measuring cleanup performance
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
                state.enemies_spawned = false;
                state.metrics = WaveMetrics::new(
                    state.current_wave,
                    BENCHMARK_WAVES[state.current_wave].enemy_count,
                );
            }
        }
        _ => {}
    }
}
```

**Step 2: Add enemy spawning helper**

Add after wave controller:

```rust
fn spawn_wave_enemies(
    commands: &mut Commands,
    handle: &hell_game::GlobalTextureAtlas,
    config: &hell_game::GameConfig,
    count: u32,
) {
    use std::f32::consts::PI;
    use bevy::math::vec3;

    let radius = 500.0; // Spawn in circle around player
    let angle_step = (2.0 * PI) / count as f32;

    for i in 0..count {
        let angle = angle_step * i as f32;
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;

        // Spawn enemy (reusing game's enemy spawn logic)
        commands.spawn((
            Sprite {
                image: handle.enemy_image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.enemy_layout.clone().unwrap(),
                    index: 0,
                }),
                ..default()
            },
            Transform::from_translation(vec3(x, y, 1.0))
                .with_scale(Vec3::splat(config.sprites.sprite_scale_factor)),
            hell_game::enemy::Enemy {
                health: config.enemy.health,
            },
            hell_game::enemy::EnemyType::Green,
            hell_game::enemy::EnemyColor(Color::srgb(0.5, 0.5, 0.5)),
            hell_game::animation::AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
            hell_game::world::GameEntity,
        ))
        .with_children(|parent| {
            // Base sprite
            parent.spawn((
                Sprite {
                    image: handle.enemy_bg_image.clone().unwrap(),
                    texture_atlas: Some(TextureAtlas {
                        layout: handle.enemy_layout.clone().unwrap(),
                        index: 0,
                    }),
                    ..default()
                },
                Transform::default(),
                hell_game::enemy::EnemyBaseSprite,
            ));

            // Tint sprite
            parent.spawn((
                Sprite {
                    image: handle.enemy_tint_image.clone().unwrap(),
                    texture_atlas: Some(TextureAtlas {
                        layout: handle.enemy_layout.clone().unwrap(),
                        index: 0,
                    }),
                    color: Color::srgb(0.5, 0.5, 0.5),
                    ..default()
                },
                Transform::from_translation(vec3(0.0, 0.0, 0.1)),
                hell_game::enemy::EnemyTintSprite,
            ));
        });
    }
}

fn print_wave_results(metrics: &WaveMetrics) {
    let spawn_metrics = metrics.calculate_phase_metrics(&metrics.spawn_frame_times);
    let combat_metrics = metrics.calculate_phase_metrics(&metrics.combat_frame_times);
    let cleanup_metrics = metrics.calculate_phase_metrics(&metrics.cleanup_frame_times);

    println!("  Spawn phase:   {:.1} FPS avg ({:.0} min, {:.0} max, p99: {:.0})",
        spawn_metrics.avg_fps, spawn_metrics.min_fps, spawn_metrics.max_fps, spawn_metrics.p99_fps);
    println!("  Combat phase:  {:.1} FPS avg ({:.0} min, {:.0} max, p99: {:.0})",
        combat_metrics.avg_fps, combat_metrics.min_fps, combat_metrics.max_fps, combat_metrics.p99_fps);
    println!("  Cleanup phase: {:.1} FPS avg ({:.0} min, {:.0} max, p99: {:.0})",
        cleanup_metrics.avg_fps, cleanup_metrics.min_fps, cleanup_metrics.max_fps, cleanup_metrics.p99_fps);
    println!("  Peak entities: {} enemies, {} bullets, {} particles",
        metrics.peak_enemy_count, metrics.peak_bullet_count, metrics.peak_particle_count);
}
```

**Step 3: Build and verify**

Run: `cargo test --test performance_benchmark`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add tests/integration/performance_benchmark.rs
git commit -m "test: add wave controller and enemy spawning systems"
```

---

## Task 5: Implement 8-Direction Auto-Fire System

**Files:**
- Modify: `tests/integration/performance_benchmark.rs`

**Step 1: Add auto-fire system**

Add before the test function:

```rust
fn benchmark_auto_fire_8_directions(
    state: Res<BenchmarkState>,
    player_query: Query<&Transform, With<hell_game::player::Player>>,
    mut commands: Commands,
    handle: Res<hell_game::GlobalTextureAtlas>,
    config: Res<hell_game::GameConfig>,
    particle_assets: Res<hell_game::particle_effects::ParticleEffectAssets>,
) {
    // Only fire during combat phase
    if state.phase != BenchmarkPhase::Combat {
        return;
    }

    let Ok(player_transform) = player_query.get_single() else { return };
    let player_pos = player_transform.translation;

    // 8 cardinal and diagonal directions
    let directions = [
        Vec3::Y,                            // N
        (Vec3::X + Vec3::Y).normalize(),    // NE
        Vec3::X,                            // E
        (Vec3::X - Vec3::Y).normalize(),    // SE
        -Vec3::Y,                           // S
        (-Vec3::X - Vec3::Y).normalize(),   // SW
        -Vec3::X,                           // W
        (-Vec3::X + Vec3::Y).normalize(),   // NW
    ];

    use bevy::math::vec3;
    use bevy_hanabi::prelude::*;
    use std::time::Instant;

    for direction in directions {
        // Spawn 50 bullets per direction (400 total per frame)
        for _ in 0..config.gun.num_bullets_per_shot {
            // Add small random spread
            let mut rng = rand::thread_rng();
            let spread_dir = vec3(
                direction.x + rng.gen_range(-0.1..0.1),
                direction.y + rng.gen_range(-0.1..0.1),
                direction.z,
            );

            commands.spawn((
                Sprite {
                    image: handle.image.clone().unwrap(),
                    texture_atlas: Some(TextureAtlas {
                        layout: handle.layout.clone().unwrap(),
                        index: 16,
                    }),
                    ..default()
                },
                Transform::from_translation(player_pos)
                    .with_scale(Vec3::splat(config.sprites.sprite_scale_factor)),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
                hell_game::gun::Bullet,
                hell_game::gun::BulletDirection(spread_dir),
                hell_game::gun::SpawnInstant(Instant::now()),
            ))
            .with_children(|parent| {
                parent.spawn((
                    ParticleEffect::new(particle_assets.bullet_trail.clone()),
                    Transform::default(),
                    hell_game::particle_effects::BulletTrailEmitter,
                ));
            });
        }
    }
}
```

**Step 2: Build and verify**

Run: `cargo test --test performance_benchmark`
Expected: Compiles successfully

**Step 3: Commit**

```bash
git add tests/integration/performance_benchmark.rs
git commit -m "test: add 8-direction auto-fire system for benchmark"
```

---

## Task 6: Implement Metrics Collector System

**Files:**
- Modify: `tests/integration/performance_benchmark.rs`

**Step 1: Add metrics collector**

Add before the test function:

```rust
fn benchmark_metrics_collector(
    mut state: ResMut<BenchmarkState>,
    time: Res<Time>,
    bullets: Query<(), With<hell_game::gun::Bullet>>,
    particles: Query<(), Or<(
        With<hell_game::particle_effects::ImpactEffect>,
        With<hell_game::particle_effects::DeathLingerEffect>,
        With<hell_game::particle_effects::BulletTrailEmitter>,
    )>>,
    enemies: Query<(), With<hell_game::enemy::Enemy>>,
) {
    let frame_time_ms = time.delta_secs() * 1000.0;

    // Record frame time for current phase
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

    // Track peak entity counts
    let particle_count = particles.iter().count() as u32;
    let bullet_count = bullets.iter().count() as u32;
    let enemy_count = enemies.iter().count() as u32;

    state.metrics.peak_particle_count = state.metrics.peak_particle_count.max(particle_count);
    state.metrics.peak_bullet_count = state.metrics.peak_bullet_count.max(bullet_count);
    state.metrics.peak_enemy_count = state.metrics.peak_enemy_count.max(enemy_count);
}
```

**Step 2: Build and verify**

Run: `cargo test --test performance_benchmark`
Expected: Compiles successfully

**Step 3: Commit**

```bash
git add tests/integration/performance_benchmark.rs
git commit -m "test: add metrics collector for frame timing and entity counts"
```

---

## Task 7: Create Test Harness and Bevy App Setup

**Files:**
- Modify: `tests/integration/performance_benchmark.rs`

**Step 1: Update test function with Bevy app setup**

Replace the placeholder test function with:

```rust
#[test]
fn performance_benchmark() {
    use hell_game::*;

    println!("\n========================================");
    println!("Performance Benchmark Starting");
    println!("========================================\n");

    let mut app = App::new();

    // Minimal plugins (headless - no rendering)
    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::asset::AssetPlugin::default());
    app.add_plugins(bevy_hanabi::HanabiPlugin);

    // Add game plugins
    app.add_plugins((
        resources::ResourcesPlugin,
        state::StatePlugin,
        config_loader::ConfigLoaderPlugin,
        enemy::EnemyPlugin,
        gun::GunPlugin,
        collision::CollisionPlugin,
        particle_effects::ParticleEffectsPlugin,
        player::PlayerPlugin,
        world::WorldPlugin,
        animation::AnimationPlugin,
    ));

    // Override with benchmark config
    app.insert_resource(config_loader::GameConfig::benchmark_mode());

    // Initialize benchmark state
    app.insert_resource(BenchmarkState::new());

    // Set initial game state
    app.insert_resource(State::new(state::GameState::Loading));
    app.insert_resource(NextState::<state::GameState>(Some(state::GameState::GameInit)));

    // Add benchmark systems
    app.add_systems(Update, (
        benchmark_wave_controller,
        benchmark_auto_fire_8_directions,
        benchmark_metrics_collector,
    ));

    // Run until benchmark complete
    let mut frame_count = 0;
    let max_frames = 5000; // Safety limit (5 waves × 5 sec × 200 FPS = ~5000 frames)

    while frame_count < max_frames {
        app.update();
        frame_count += 1;

        // Check if benchmark is complete
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
```

**Step 2: Build and verify**

Run: `cargo test --test performance_benchmark`
Expected: May fail if plugins have initialization issues, but should compile

**Step 3: Commit**

```bash
git add tests/integration/performance_benchmark.rs
git commit -m "test: add complete Bevy app test harness for benchmark"
```

---

## Task 8: Add Imports and Fix Compilation Issues

**Files:**
- Modify: `tests/integration/performance_benchmark.rs`

**Step 1: Add all necessary imports at top of file**

Add at the very top of `tests/integration/performance_benchmark.rs`:

```rust
use bevy::prelude::*;
use bevy::time::TimerMode;
use bevy_hanabi::prelude::*;
use rand::Rng;
use std::time::Instant;
```

**Step 2: Build and fix any remaining errors**

Run: `cargo test --test performance_benchmark --no-run`
Expected: Compilation errors about missing imports or visibility

Fix each error by:
- Adding `pub` to structs/enums in main codebase if needed
- Adjusting import paths
- Checking that all game modules are properly exported from `lib.rs`

**Step 3: Final build check**

Run: `cargo test --test performance_benchmark --no-run`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add tests/integration/performance_benchmark.rs
git commit -m "test: fix imports and compilation issues in benchmark"
```

---

## Task 9: Test with Small Wave (100 Enemies)

**Files:**
- None (testing only)

**Step 1: Temporarily reduce to single wave**

Edit `BENCHMARK_WAVES` to test just first wave:

```rust
const BENCHMARK_WAVES: &[WaveConfig] = &[
    WaveConfig { enemy_count: 100, duration_secs: 5.0 },
    // Comment out others for initial testing
    // WaveConfig { enemy_count: 1_000, duration_secs: 5.0 },
    // ...
];
```

**Step 2: Run test in release mode**

Run: `cargo test --release --test performance_benchmark -- --nocapture`

Expected output:
```
========================================
Performance Benchmark Starting
========================================

Wave 1/1 complete!
  Spawn phase:   XXX FPS avg (XXX min, XXX max, p99: XXX)
  Combat phase:  XXX FPS avg (XXX min, XXX max, p99: XXX)
  Cleanup phase: XXX FPS avg (XXX min, XXX max, p99: XXX)
  Peak entities: 100 enemies, XXXX bullets, XXXX particles

========================================
Benchmark Complete!
Total frames: XXX
========================================
```

**Step 3: Verify metrics make sense**

Check:
- [ ] FPS values are reasonable (>60 FPS for 100 enemies)
- [ ] Combat phase FPS < Spawn/Cleanup (expected)
- [ ] Peak bullet count is high (thousands)
- [ ] Peak particle count is non-zero
- [ ] Test completes without crashes

**Step 4: Restore all waves**

Uncomment all waves in `BENCHMARK_WAVES`

**Step 5: Commit**

```bash
git add tests/integration/performance_benchmark.rs
git commit -m "test: verify benchmark works with single wave test"
```

---

## Task 10: Run Full Benchmark and Document Results

**Files:**
- Create: `target/benchmark_results_TIMESTAMP.txt` (manual output capture)
- None to commit (results are documentation)

**Step 1: Run full benchmark suite in release**

Run: `cargo test --release --test performance_benchmark -- --nocapture > benchmark_results.txt 2>&1`

This saves all output to `benchmark_results.txt`

**Step 2: Analyze results**

Review output and answer:
1. Which wave first drops below 200 FPS?
2. Which phase is the bottleneck (spawn/combat/cleanup)?
3. What are the peak entity counts at bottleneck?
4. Is the test deterministic (run twice, same results)?

**Step 3: Add results summary to implementation plan**

At the end of this file (below "Task 10"), add:

```markdown
## Benchmark Results

**Test Date:** [DATE]
**Build:** Release
**Hardware:** [CPU/GPU info]

### Wave Results

| Wave | Enemies | Spawn FPS | Combat FPS | Cleanup FPS | Peak Bullets | Peak Particles |
|------|---------|-----------|------------|-------------|--------------|----------------|
| 1    | 100     | XXX       | XXX        | XXX         | XXX          | XXX            |
| 2    | 1k      | XXX       | XXX        | XXX         | XXX          | XXX            |
| 3    | 10k     | XXX       | XXX        | XXX         | XXX          | XXX            |
| 4    | 20k     | XXX       | XXX        | XXX         | XXX          | XXX            |
| 5    | 100k    | XXX       | XXX        | XXX         | XXX          | XXX            |

### Bottleneck Analysis

- **First failure:** Wave X at YYY FPS (target: 200 FPS)
- **Bottleneck phase:** [Spawn/Combat/Cleanup]
- **Suspected cause:** [Collision detection / Particle spawning / Entity iteration]
- **Recommended next step:** [Profile wave X with cargo flamegraph]
```

**Step 4: No commit (results are ephemeral)**

Results go in documentation, not in git

---

## Success Criteria

- [ ] Benchmark compiles and runs in headless mode
- [ ] All 5 waves complete successfully (5 seconds each)
- [ ] Console output shows per-phase FPS metrics
- [ ] Entity counts tracked (bullets, particles, enemies)
- [ ] Release mode performance measured
- [ ] Bottleneck wave identified (<200 FPS)
- [ ] Results reproducible (run twice, similar numbers)
- [ ] No crashes or panics during test

---

## Troubleshooting

### Issue: Test hangs or doesn't progress

**Cause:** Game state not transitioning properly

**Fix:** Check that `StatePlugin` properly transitions from Loading → MainMenu → GameInit → InGame

### Issue: No enemies spawn

**Cause:** Enemy spawn system not running or config override issue

**Fix:**
- Verify `EnemyPlugin` is added to app
- Check `benchmark_mode()` config is applied
- Ensure `GameState::InGame` is active

### Issue: No bullets spawn

**Cause:** Player entity doesn't exist or gun system not running

**Fix:**
- Verify `WorldPlugin` spawns player entity
- Check `GunPlugin` systems are registered
- Ensure `benchmark_auto_fire_8_directions` is in Update schedule

### Issue: FPS metrics show 0 or NaN

**Cause:** Frame time collection not working

**Fix:**
- Check `Time` resource is available
- Verify `benchmark_metrics_collector` runs every frame
- Ensure frame times are being pushed to correct vec

---

## Future Enhancements (Out of Scope)

### JSON Output

Add structured results for regression tracking:

```rust
#[derive(serde::Serialize)]
struct BenchmarkResults {
    timestamp: String,
    waves: Vec<WaveResult>,
}

fn save_results(state: &BenchmarkState) {
    let json = serde_json::to_string_pretty(&results).unwrap();
    std::fs::write("target/benchmark_results.json", json).unwrap();
}
```

### Per-Wave Tests

Add individual tests for faster iteration:

```rust
#[test]
fn benchmark_wave_100_enemies() { /* ... */ }

#[test]
fn benchmark_wave_1000_enemies() { /* ... */ }
```

### Windowed Benchmark

Add optional GPU rendering test:

```rust
// Add --features windowed-benchmark flag
#[cfg(feature = "windowed-benchmark")]
app.add_plugins(DefaultPlugins);
```

### Flamegraph Integration

Auto-generate profiling data for failed waves:

```bash
cargo flamegraph --test performance_benchmark -- --nocapture
```

---

## Notes for Engineer

- **Headless mode:** No window appears, this is intentional (CPU-only testing)
- **Release mode:** Always run in `--release` for realistic performance
- **Determinism:** Results should be reproducible (±5% variance acceptable)
- **Entity cleanup:** Bevy automatically cleans up entities when test ends
- **200 FPS target:** Ambitious goal, may need optimization to hit on all waves
- **Profiling:** Use `cargo flamegraph` on specific bottleneck waves for detailed analysis
