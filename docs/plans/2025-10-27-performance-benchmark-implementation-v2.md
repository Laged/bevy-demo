# Performance Benchmark System Implementation Plan (v2 - Using Headless Infrastructure)

> **STATUS: ✅ COMPLETED - Merged to dev branch**

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build wave-based performance benchmark using the new headless testing infrastructure from the restructuring work.

**Architecture:** Integration test using `test_utils` for app creation. Much simpler than original plan since headless infrastructure handles plugin setup, entity spawning, and state management.

**Tech Stack:** Bevy 0.17.2, bevy_hanabi 0.17.0, test_utils module from restructured codebase

---

## Prerequisites

- **CRITICAL:** Headless testing refactor MUST be merged to main first ✅ COMPLETED
- `test_utils` module exists with all utilities ✅ AVAILABLE
- Plugins support headless mode ✅ AVAILABLE
- Current working directory: `/home/laged/Codings/laged/bevy-demo` ✅
- **Execute in worktree:** `feature/performance-benchmark` ✅ COMPLETED & MERGED

## Task Overview

1. Create benchmark config module
2. Create benchmark state and metrics structs
3. Implement wave controller system
4. Implement 8-direction auto-fire system
5. Implement metrics collector system
6. Create test harness using test_utils
7. Add console output formatting
8. Test with single wave (100 enemies)
9. Run full benchmark suite
10. Document results

---

## Task 1: Create Benchmark Config Module

**Files:**
- Create: `src/benchmark_config.rs`
- Modify: `src/lib.rs`

**Step 1: Create benchmark config**

Create `src/benchmark_config.rs`:

```rust
use crate::config_loader::GameConfig;

impl GameConfig {
    /// Returns a config optimized for performance benchmarking
    pub fn benchmark_mode() -> Self {
        let mut config = Self::default();

        // Overpowered player
        config.gun.bullet_damage = 999999.0;
        config.gun.num_bullets_per_shot = 50;
        config.gun.bullet_spawn_interval = 0.0; // Fire every frame
        config.gun.bullet_speed = 30.0;
        config.gun.bullet_time_secs = 2.0;

        // No spawn rate limiting
        config.enemy.spawn_interval = 0.0;
        config.enemy.max_num_enemies = 100_000;

        config
    }
}
```

**Step 2: Export from lib.rs**

Add to `src/lib.rs`:

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

## Task 2: Create Benchmark State and Metrics Structs

**Files:**
- Create: `tests/integration/performance_benchmark.rs`
- Modify: `tests/integration/mod.rs`

**Step 1: Update test module**

Edit `tests/integration/mod.rs` and add:

```rust
pub mod performance_benchmark;
```

**Step 2: Create benchmark test file with structs**

Create `tests/integration/performance_benchmark.rs`:

```rust
use bevy::prelude::*;
use hell_game::test_utils::*;
use hell_game::config_loader::GameConfig;
use std::time::Duration;

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

#[test]
fn performance_benchmark() {
    println!("Benchmark test placeholder");
}
```

**Step 3: Build and verify**

Run: `cargo test --test performance_benchmark`
Expected: Test passes (placeholder)

**Step 4: Commit**

```bash
git add tests/integration/
git commit -m "test: add benchmark state and metrics structures"
```

---

## Task 3: Implement Wave Controller System

**Files:**
- Modify: `tests/integration/performance_benchmark.rs`

**Step 1: Add wave controller**

Add before the test function:

```rust
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

fn print_wave_results(metrics: &WaveMetrics) {
    let spawn_fps = calculate_avg_fps(&metrics.spawn_frame_times);
    let combat_fps = calculate_avg_fps(&metrics.combat_frame_times);
    let cleanup_fps = calculate_avg_fps(&metrics.cleanup_frame_times);

    println!("  Spawn:   {:.0} FPS avg", spawn_fps);
    println!("  Combat:  {:.0} FPS avg", combat_fps);
    println!("  Cleanup: {:.0} FPS avg", cleanup_fps);
    println!("  Peak: {} enemies, {} bullets, {} particles",
        metrics.peak_enemy_count, metrics.peak_bullet_count, metrics.peak_particle_count);
}

fn calculate_avg_fps(frame_times_ms: &[f32]) -> f32 {
    if frame_times_ms.is_empty() {
        return 0.0;
    }
    let fps_samples: Vec<f32> = frame_times_ms.iter().map(|&ms| 1000.0 / ms).collect();
    fps_samples.iter().sum::<f32>() / fps_samples.len() as f32
}
```

**Step 2: Build and verify**

Run: `cargo test --test performance_benchmark --no-run`
Expected: Compiles successfully

**Step 3: Commit**

```bash
git add tests/integration/performance_benchmark.rs
git commit -m "test: add wave controller system for benchmark"
```

---

## Task 4: Implement 8-Direction Auto-Fire System

**Files:**
- Modify: `tests/integration/performance_benchmark.rs`

**Step 1: Add auto-fire system**

Add before the test function:

```rust
use bevy_hanabi::prelude::*;
use rand::Rng;
use std::time::Instant;

fn benchmark_auto_fire_8_directions(
    state: Res<BenchmarkState>,
    player_query: Query<&Transform, With<hell_game::player::Player>>,
    mut commands: Commands,
    handle: Res<hell_game::resources::GlobalTextureAtlas>,
    config: Res<GameConfig>,
    particle_assets: Res<hell_game::particle_effects::ParticleEffectAssets>,
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
```

**Step 2: Build and verify**

Run: `cargo test --test performance_benchmark --no-run`
Expected: Compiles

**Step 3: Commit**

```bash
git add tests/integration/performance_benchmark.rs
git commit -m "test: add 8-direction auto-fire system for benchmark"
```

---

## Task 5: Implement Metrics Collector System

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
```

**Step 2: Build and verify**

Run: `cargo test --test performance_benchmark --no-run`
Expected: Compiles

**Step 3: Commit**

```bash
git add tests/integration/performance_benchmark.rs
git commit -m "test: add metrics collector for frame timing and entity counts"
```

---

## Task 6: Create Test Harness Using Test Utils

**Files:**
- Modify: `tests/integration/performance_benchmark.rs`

**Step 1: Replace placeholder test with full harness**

Replace the `performance_benchmark()` function with:

```rust
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
    app.add_systems(Update, (
        benchmark_wave_controller,
        benchmark_auto_fire_8_directions,
        benchmark_metrics_collector,
    ));

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
```

**Step 2: Build and verify**

Run: `cargo test --test performance_benchmark --no-run`
Expected: Compiles successfully

**Step 3: Commit**

```bash
git add tests/integration/performance_benchmark.rs
git commit -m "test: add complete benchmark harness using test_utils"
```

---

## Task 7: Test with Single Wave (100 Enemies)

**Files:**
- Modify: `tests/integration/performance_benchmark.rs` (temporary)

**Step 1: Reduce to single wave for testing**

Temporarily change `BENCHMARK_WAVES`:

```rust
const BENCHMARK_WAVES: &[WaveConfig] = &[
    WaveConfig { enemy_count: 100 },
    // Comment others for initial test
];
```

**Step 2: Run test in release mode**

Run: `cargo test --release --test performance_benchmark -- --nocapture`

Expected output:
```
Performance Benchmark Starting

Wave 1/1 complete!
  Spawn:   XXX FPS avg
  Combat:  XXX FPS avg
  Cleanup: XXX FPS avg
  Peak: 100 enemies, XXXX bullets, XXX particles

Benchmark Complete!
```

**Step 3: Verify metrics are reasonable**

Check:
- FPS > 60 for 100 enemies
- No crashes
- Metrics printed

**Step 4: Restore all waves**

Uncomment all waves in `BENCHMARK_WAVES`

**Step 5: Commit**

```bash
git add tests/integration/performance_benchmark.rs
git commit -m "test: verify benchmark works with single wave"
```

---

## Task 8: Run Full Benchmark Suite

**Files:**
- None (testing only)

**Step 1: Run full benchmark**

Run: `cargo test --release --test performance_benchmark -- --nocapture > benchmark_results.txt 2>&1`

**Step 2: Review results**

Check `benchmark_results.txt` for:
1. Which wave drops below 200 FPS?
2. Which phase is slowest?
3. Peak entity counts

**Step 3: Document findings**

No commit - results are ephemeral documentation

---

## Task 9: Document and Finalize

**Files:**
- Update: `docs/plans/2025-10-27-performance-benchmark-implementation-v2.md` (this file)

**Step 1: Add results section**

At end of this file, document:

```markdown
## Benchmark Results

**Date:** [YOUR DATE]
**Build:** Release mode
**Hardware:** [YOUR SPECS]

### Results Summary

| Wave | Enemies | Spawn FPS | Combat FPS | Cleanup FPS |
|------|---------|-----------|------------|-------------|
| 1    | 100     | XXX       | XXX        | XXX         |
| 2    | 1k      | XXX       | XXX        | XXX         |
| 3    | 10k     | XXX       | XXX        | XXX         |
| 4    | 20k     | XXX       | XXX        | XXX         |
| 5    | 100k    | XXX       | XXX        | XXX         |

**Bottleneck:** Wave X, Combat phase at YYY FPS
```

**Step 2: Final commit and merge**

```bash
git add tests/integration/performance_benchmark.rs
git commit -m "test: finalize performance benchmark implementation"

# Merge to main
git checkout main
git merge feature/performance-benchmark
git push origin main
```

---

## Success Criteria

- [ ] Benchmark uses test_utils for all setup
- [ ] All 5 waves complete successfully
- [ ] Console output shows FPS metrics
- [ ] Release mode performance measured
- [ ] Results documented
- [ ] Zero merge conflicts (only touched tests/ and src/benchmark_config.rs)

---

## Comparison to Original Plan

**Original plan (without restructuring):**
- 10 tasks
- 50+ lines of manual Bevy app setup
- Manual plugin configuration
- Manual state management
- Manual entity spawning with sprites

**This plan (with restructuring):**
- 9 tasks
- 3 lines for app setup (`create_headless_app` + `init_for_testing`)
- Uses test_utils utilities
- Clean, maintainable code

**Lines of code saved:** ~100+ lines by using restructured infrastructure

---

## Notes for Engineer

- **Depends on restructure:** Must merge headless refactor BEFORE starting this
- **Much simpler:** Restructuring did the hard work, benchmark is now trivial
- **Reusable:** test_utils will benefit future tests too
- **Sequential workflow:** Restructure → main → Benchmark branch → main
- **Zero conflicts:** Restructure touched src/, benchmark touches tests/
