# Performance Benchmark System Design

**Date:** 2025-10-27
**Status:** Design Approved
**Implementation:** Pending

## Overview

Build a deterministic, wave-based performance benchmark to measure FPS under realistic combat loads. Target: 200+ FPS in release mode across all entity count scales (100 to 100k enemies).

## Goals

- **Scalable testing:** Benchmark at 100 / 1k / 10k / 20k / 100k enemy counts
- **Deterministic waves:** Each wave is exactly 5 seconds with fixed phases
- **Realistic load:** Test actual gameplay systems (bullets, collisions, particles)
- **Regression detection:** JSON output for tracking performance over time
- **Target performance:** 200+ FPS in release mode, identify bottlenecks below target

## Non-Goals

- Interactive gameplay testing (this is automated stress testing)
- GPU rendering benchmarks (headless mode, CPU-only ECS testing)
- Memory profiling (focus is frame timing and FPS)
- Network/multiplayer performance

## Design Decisions

### Architecture: Hybrid Test Harness

**Chosen approach:** Integration test in `tests/integration/performance_benchmark.rs`

**Why hybrid over alternatives:**
- **vs Separate binary (criterion):** Need access to full Bevy ECS and game systems, not just isolated functions
- **vs In-game benchmark mode:** Keeps game code clean, no test-specific code polluting production
- **vs Manual testing:** Automated, reproducible, works in CI/CD

**Benefits:**
- Runs in headless mode (no window, no GPU rendering)
- Uses real game systems (collision, particles, bullets)
- Deterministic results (fixed timestep, seeded RNG)
- Can run in CI to catch regressions

### Wave Structure: 5-Second Phases

**Each wave lasts exactly 5 seconds:**

```
0.0s - 0.5s: Spawn Phase
  - Spawn all enemies at once in circle around player
  - Measure spawn system performance

0.5s - 4.5s: Combat Phase
  - Player auto-fires in 8 directions simultaneously
  - Bullets hit enemies, particles spawn from impacts/deaths
  - Measure peak load performance (collision + particles)

4.5s - 5.0s: Cleanup Phase
  - Enemies should be dead, measure despawn/cleanup performance
  - Particle effects finish and despawn

5.0s: Rest Phase
  - Record metrics (min/max/avg/p95/p99 FPS per phase)
  - Despawn any remaining entities
  - Prepare for next wave
```

**Why this structure:**
- **Deterministic:** Same timeline every run, comparable results
- **Realistic:** Tests actual gameplay (not programmatic entity deletion)
- **Phased metrics:** Identifies which phase is the bottleneck
- **Scales well:** Works from 100 to 100k enemies

### Auto-Fire System: 8-Direction Overpowered Player

**Player configuration for benchmark:**
```rust
bullet_damage: 999999.0       // One-shot any enemy
num_bullets_per_shot: 50      // Dense bullet spread
bullet_spawn_interval: 0.0    // Fire every frame
bullet_speed: 30.0            // Fast enough to clear enemies quickly
```

**Firing pattern:**
- Fires in 8 directions: N, NE, E, SE, S, SW, W, NW
- 50 bullets per direction = 400 bullets per frame
- At 60 FPS × 4 seconds combat = ~96,000 bullets total per wave

**Why 8-direction auto-fire:**
- Tests realistic collision detection load
- Generates realistic particle spawn load (hits + deaths)
- No programmatic cheating (enemies die from actual bullets)
- Simulates "surrounded by enemies" gameplay scenario

### Headless Mode: CPU-Only Testing

**Bevy app configuration:**
```rust
App::new()
    .add_plugins(MinimalPlugins)           // Time, scheduling only
    .add_plugins(AssetPlugin::default())   // Need for config loading
    .add_plugins(bevy_hanabi::HanabiPlugin) // Particle CPU logic
    .add_plugins((
        EnemyPlugin,
        GunPlugin,
        CollisionPlugin,
        ParticleEffectsPlugin,
        // ... all game plugins
    ))
```

**What's tested:**
- ECS update performance (entity spawning, queries, system scheduling)
- Collision detection (KD-tree lookups, bullet-enemy intersections)
- Particle spawning/despawning (bevy_hanabi CPU-side logic)
- Component updates (transform, health, timers)

**What's NOT tested:**
- GPU rendering (no window, no draw calls)
- Shader compilation
- Texture loading/atlasing
- GPU particle rendering

**Why headless:**
- Isolates CPU bottlenecks (ECS, collision, particle spawning)
- Runs in CI environments (no display required)
- Faster iteration (no GPU overhead)
- Deterministic (no GPU driver variance)

**Trade-off:** If GPU rendering is the bottleneck, this won't detect it. Solution: Add optional windowed benchmark mode later if needed.

## Component Structure

### New Resource

```rust
#[derive(Resource)]
struct BenchmarkState {
    current_wave: usize,
    wave_timer: Timer,
    phase: BenchmarkPhase,
    metrics: WaveMetrics,
}

#[derive(PartialEq)]
enum BenchmarkPhase {
    Spawn,      // 0-0.5s
    Combat,     // 0.5-4.5s
    Cleanup,    // 4.5-5s
    Rest,       // 5s marker
    Complete,   // All waves done
}
```

### Metrics Structures

```rust
#[derive(Default)]
struct WaveMetrics {
    wave_number: usize,
    enemy_count: u32,

    // Frame timing (collected every frame)
    frame_times_ms: Vec<f32>,

    // Phase-specific FPS metrics
    spawn_phase_fps: PhaseMetrics,
    combat_phase_fps: PhaseMetrics,
    cleanup_phase_fps: PhaseMetrics,

    // Peak entity counts
    peak_particle_count: u32,
    peak_bullet_count: u32,
    peak_enemy_count: u32,
}

struct PhaseMetrics {
    min_fps: f32,
    max_fps: f32,
    avg_fps: f32,
    p95_fps: f32,  // 95th percentile
    p99_fps: f32,  // 99th percentile
}
```

## Data Flow

### 1. Test Initialization

**Location:** `tests/integration/performance_benchmark.rs`

**Setup:**
```rust
#[test]
fn performance_benchmark() {
    let mut app = App::new();

    // Minimal plugins (headless)
    app.add_plugins(MinimalPlugins)
        .add_plugins(AssetPlugin::default())
        .add_plugins(bevy_hanabi::HanabiPlugin);

    // Game plugins
    app.add_plugins((
        ResourcesPlugin,
        StatePlugin,
        EnemyPlugin,
        GunPlugin,
        CollisionPlugin,
        ParticleEffectsPlugin,
        PlayerPlugin,
        WorldPlugin,
    ));

    // Inject benchmark config
    app.insert_resource(GameConfig::benchmark_mode());
    app.insert_resource(BenchmarkState::new(BENCHMARK_WAVES));

    // Add benchmark systems
    app.add_systems(Update, (
        benchmark_wave_controller,
        benchmark_auto_fire_8_directions,
        benchmark_metrics_collector,
    ));

    // Run until complete
    while app.world().resource::<BenchmarkState>().phase != BenchmarkPhase::Complete {
        app.update();
    }

    // Output results
    save_benchmark_results(app.world().resource::<BenchmarkState>());
}
```

### 2. Wave Controller System

**Function:** `benchmark_wave_controller()`

**Responsibilities:**
- Track wave timer and current phase
- Spawn enemies at wave start (phase = Spawn)
- Transition between phases based on elapsed time
- Record metrics at wave end
- Advance to next wave or complete

**Pseudocode:**
```rust
fn benchmark_wave_controller(
    mut state: ResMut<BenchmarkState>,
    time: Res<Time>,
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
) {
    state.wave_timer.tick(time.delta());
    let elapsed = state.wave_timer.elapsed_secs();

    match (state.phase, elapsed) {
        (BenchmarkPhase::Spawn, t) if t < 0.5 => {
            if !state.enemies_spawned {
                spawn_wave_enemies(
                    &mut commands,
                    &handle,
                    WAVES[state.current_wave].enemy_count,
                );
                state.enemies_spawned = true;
                state.phase = BenchmarkPhase::Combat;
            }
        }
        (BenchmarkPhase::Combat, t) if t >= 0.5 && t < 4.5 => {
            // Auto-fire handled by separate system
        }
        (BenchmarkPhase::Cleanup, t) if t >= 4.5 && t < 5.0 => {
            // Just measuring cleanup performance
        }
        (_, t) if t >= 5.0 => {
            // Wave complete - record and advance
            state.metrics.finalize_wave();
            state.current_wave += 1;

            if state.current_wave >= WAVES.len() {
                state.phase = BenchmarkPhase::Complete;
            } else {
                state.wave_timer.reset();
                state.phase = BenchmarkPhase::Spawn;
                state.enemies_spawned = false;
            }
        }
        _ => {}
    }
}
```

### 3. Auto-Fire System

**Function:** `benchmark_auto_fire_8_directions()`

**Fires bullets only during combat phase:**
```rust
fn benchmark_auto_fire_8_directions(
    state: Res<BenchmarkState>,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    config: Res<GameConfig>,
    particle_assets: Res<ParticleEffectAssets>,
) {
    // Only fire during combat phase
    if state.phase != BenchmarkPhase::Combat {
        return;
    }

    let Ok(player_transform) = player_query.get_single() else { return };
    let player_pos = player_transform.translation;

    // 8 cardinal/diagonal directions
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

    for direction in directions {
        // Spawn 50 bullets per direction (400 total per frame)
        for _ in 0..config.gun.num_bullets_per_shot {
            // Reuse existing bullet spawn logic from gun.rs
            spawn_bullet(&mut commands, &handle, &particle_assets, player_pos, direction, &config);
        }
    }
}
```

### 4. Metrics Collection

**Function:** `benchmark_metrics_collector()`

**Collects data every frame:**
```rust
fn benchmark_metrics_collector(
    mut state: ResMut<BenchmarkState>,
    time: Res<Time>,
    bullets: Query<(), With<Bullet>>,
    particles: Query<(), Or<(With<ImpactEffect>, With<DeathLingerEffect>, With<BulletTrailEmitter>)>>,
    enemies: Query<(), With<Enemy>>,
) {
    // Record frame time
    let frame_time_ms = time.delta_secs() * 1000.0;
    state.metrics.frame_times_ms.push(frame_time_ms);

    // Track peak entity counts
    let particle_count = particles.iter().count() as u32;
    let bullet_count = bullets.iter().count() as u32;
    let enemy_count = enemies.iter().count() as u32;

    state.metrics.peak_particle_count = state.metrics.peak_particle_count.max(particle_count);
    state.metrics.peak_bullet_count = state.metrics.peak_bullet_count.max(bullet_count);
    state.metrics.peak_enemy_count = state.metrics.peak_enemy_count.max(enemy_count);

    // Categorize frame times by phase for later analysis
    state.metrics.record_frame_for_phase(state.phase, frame_time_ms);
}
```

### 5. Results Output

**Function:** `save_benchmark_results()`

**JSON output format:**
```json
{
  "benchmark_date": "2025-10-27T23:30:00Z",
  "build_type": "release",
  "bevy_version": "0.17.2",
  "hanabi_version": "0.17.0",
  "waves": [
    {
      "wave": 1,
      "enemy_count": 100,
      "spawn_phase": {
        "min_fps": 240.5,
        "max_fps": 310.2,
        "avg_fps": 280.1,
        "p95_fps": 260.0,
        "p99_fps": 250.0
      },
      "combat_phase": {
        "min_fps": 180.3,
        "max_fps": 250.1,
        "avg_fps": 220.5,
        "p95_fps": 200.0,
        "p99_fps": 190.0
      },
      "cleanup_phase": {
        "min_fps": 250.0,
        "max_fps": 320.0,
        "avg_fps": 290.0,
        "p95_fps": 270.0,
        "p99_fps": 260.0
      },
      "peak_particles": 1200,
      "peak_bullets": 8000,
      "peak_enemies": 100,
      "duration_secs": 5.0
    },
    {
      "wave": 2,
      "enemy_count": 1000,
      "spawn_phase": { /* ... */ },
      "combat_phase": {
        "min_fps": 140.0,  // ⚠️ Below 200 FPS target
        "avg_fps": 185.0
      },
      "peak_particles": 3500,
      "peak_bullets": 15000,
      "peak_enemies": 1000
    }
    // ... waves 3, 4, 5
  ],
  "summary": {
    "lowest_fps_overall": 140.0,
    "lowest_fps_wave": 2,
    "lowest_fps_phase": "combat",
    "target_fps": 200.0,
    "passes_target": false
  }
}
```

**File saved to:** `target/benchmark_results_TIMESTAMP.json`

## Configuration

### Benchmark-Specific Config

**New file:** `src/benchmark_config.rs`

```rust
impl GameConfig {
    pub fn benchmark_mode() -> Self {
        let mut config = Self::default();

        // Overpowered player for fast enemy clearing
        config.gun.bullet_damage = 999999.0;
        config.gun.num_bullets_per_shot = 50;
        config.gun.bullet_spawn_interval = 0.0;  // Fire every frame
        config.gun.bullet_speed = 30.0;
        config.gun.bullet_time_secs = 2.0;  // Bullets live 2 seconds

        // Disable enemy spawn rate limiting
        config.enemy.spawn_interval = 0.0;
        config.enemy.max_num_enemies = 100_000;
        config.enemy.speed = 1.0;  // Normal enemy speed

        // Keep particle effects at current settings (what we're testing)
        // No changes to particle_effects config

        config
    }
}
```

### Wave Definitions

```rust
const BENCHMARK_WAVES: &[WaveConfig] = &[
    WaveConfig { enemy_count: 100, duration_secs: 5.0 },
    WaveConfig { enemy_count: 1_000, duration_secs: 5.0 },
    WaveConfig { enemy_count: 10_000, duration_secs: 5.0 },
    WaveConfig { enemy_count: 20_000, duration_secs: 5.0 },
    WaveConfig { enemy_count: 100_000, duration_secs: 5.0 },
];
```

## Module Structure

### New Files

- **tests/integration/performance_benchmark.rs** - Main benchmark test
- **tests/integration/mod.rs** - Test module exports
- **src/benchmark_config.rs** - Benchmark-specific config overrides

### Modified Files

- **src/lib.rs** - Export benchmark utilities (`pub use benchmark_config::*;`)
- **Cargo.toml** - Add `[[test]]` section for integration test

### File Organization

```
bevy-demo/
├── src/
│   ├── benchmark_config.rs     # NEW
│   ├── lib.rs                  # Export benchmark_config
│   └── ...
├── tests/
│   └── integration/
│       ├── mod.rs              # NEW
│       └── performance_benchmark.rs  # NEW
├── target/
│   └── benchmark_results_*.json  # Generated output
└── Cargo.toml                  # Add [[test]] section
```

## Running the Benchmark

### Basic Usage

```bash
# Run full benchmark suite (all 5 waves)
cargo test --release --test performance_benchmark -- --nocapture

# Run with timing details
cargo test --release --test performance_benchmark -- --nocapture --show-output

# Single wave for quick testing (if we add per-wave tests)
cargo test --release --test performance_benchmark -- --nocapture wave_1000_enemies
```

### Expected Output (Console)

```
Running performance benchmark...

Wave 1/5: 100 enemies
  Spawn phase:   280.1 FPS avg (240 min, 310 max, p99: 250)
  Combat phase:  220.5 FPS avg (180 min, 250 max, p99: 190) ✓
  Cleanup phase: 290.0 FPS avg (250 min, 320 max, p99: 260)
  Peak entities: 100 enemies, 8000 bullets, 1200 particles
  ✓ Passed 200 FPS target

Wave 2/5: 1000 enemies
  Spawn phase:   265.0 FPS avg (220 min, 290 max, p99: 235)
  Combat phase:  185.0 FPS avg (140 min, 210 max, p99: 160) ⚠️ BELOW TARGET
  Cleanup phase: 270.0 FPS avg (230 min, 300 max, p99: 250)
  Peak entities: 1000 enemies, 15000 bullets, 3500 particles
  ✗ Failed 200 FPS target (lowest: 140 FPS in combat phase)

Wave 3/5: 10000 enemies
  Spawn phase:   180.0 FPS avg (150 min, 210 max, p99: 165)
  Combat phase:  120.0 FPS avg (80 min, 150 max, p99: 95) ⚠️ BELOW TARGET
  Cleanup phase: 200.0 FPS avg (170 min, 230 max, p99: 185)
  Peak entities: 10000 enemies, 25000 bullets, 8000 particles
  ✗ Failed 200 FPS target (lowest: 80 FPS in combat phase)

...

Benchmark complete!
Results saved to: target/benchmark_results_2025-10-27_233045.json

Summary:
  Overall lowest FPS: 80.0 (wave 3, combat phase)
  Waves passing target (200 FPS): 1/5
  Bottleneck phase: Combat (collision + particles)
  Recommended action: Profile wave 2-3 combat phase
```

## Performance Analysis Workflow

### 1. Run Benchmark

```bash
cargo test --release --test performance_benchmark -- --nocapture
```

### 2. Identify Bottleneck Wave

Look for first wave that drops below 200 FPS:
- Wave 1 (100 enemies): ✓ Passes
- Wave 2 (1000 enemies): ✗ 185 FPS avg, 140 FPS min ← **Start here**

### 3. Identify Bottleneck Phase

Check which phase is slowest in that wave:
- Spawn: 265 FPS ✓
- Combat: 185 FPS ✗ ← **Bottleneck**
- Cleanup: 270 FPS ✓

### 4. Check Entity Counts

Correlate FPS drop with entity counts:
- 15,000 bullets active
- 3,500 particles active
- 1,000 enemies active

**Hypothesis:** Collision detection or particle spawning is bottleneck.

### 5. Profile with Cargo Flamegraph

```bash
# Profile just wave 2 (if we add per-wave tests)
cargo flamegraph --test performance_benchmark -- --nocapture wave_1000_enemies

# Analyze flamegraph.svg to find hot functions
```

### 6. Optimize and Re-run

Make targeted optimization → Re-run benchmark → Compare JSON results.

## Success Criteria

- [ ] Benchmark runs in headless mode (no window)
- [ ] All 5 waves complete deterministically (5 seconds each)
- [ ] JSON output generated with per-phase FPS metrics
- [ ] Console output shows clear pass/fail vs 200 FPS target
- [ ] Peak entity counts tracked (bullets, particles, enemies)
- [ ] Reproducible results (same input = same output)
- [ ] Identifies bottleneck wave and phase
- [ ] Can run in CI environment

## Future Enhancements (Out of Scope)

### Optional Windowed Benchmark

If GPU rendering is suspected bottleneck:
- Add `--windowed` flag to run with DefaultPlugins
- Measure GPU draw calls and shader overhead
- Compare headless vs windowed results

### Memory Profiling

- Track heap allocations per frame
- Identify allocation hotspots
- Measure ECS archetype fragmentation

### CI Integration

- Run benchmark on every commit
- Compare against baseline
- Fail CI if FPS drops >10% from baseline

### Profiler Integration

- Auto-generate flamegraphs for failed waves
- Integration with `pprof` or `samply`
- Automated bottleneck detection

## References

- Bevy testing docs: https://bevyengine.org/learn/book/contributing/testing/
- bevy_hanabi particle system: https://docs.rs/bevy_hanabi/0.17.0
- Current collision system: `src/collision.rs` (KD-tree based)
- Current particle system: `src/particle_effects.rs`
- Game config system: `src/config_loader.rs`
