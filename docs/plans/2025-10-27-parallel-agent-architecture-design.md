# Parallel Agent Architecture Design

**Date:** 2025-10-27
**Status:** Design Approved
**Goal:** Restructure codebase to support 4 parallel agents (UI, Gameplay, Graphics, Testing) working independently with minimal merge conflicts.

---

## Overview

This design transforms the flat `src/` structure into a domain-driven architecture where each agent owns specific folders and can work independently. The key innovation is an extensible **choice system** that allows gameplay to pause for player decisions (upgrades, loot, level-ups) while maintaining clean separation between agents.

**Context:** This design builds on the recently completed headless testing infrastructure (commit 20b32bb) and performance benchmark system (commit dc4ec77), which provide the foundation for Testing Agent workflows.

---

## High-Level Structure

```
src/
├── domains/              # Agent-specific code
│   ├── ui/              # UI Agent owns
│   │   ├── mod.rs
│   │   ├── hud.rs
│   │   ├── choices.rs   # Generic choice UI
│   │   ├── camera.rs
│   │   └── menu.rs
│   ├── gameplay/        # Gameplay Agent owns
│   │   ├── mod.rs
│   │   ├── balance.rs   # Game constants (from configs.rs)
│   │   ├── combat.rs
│   │   ├── spawning.rs
│   │   └── loop.rs      # State machine control
│   ├── graphics/        # Graphics Agent owns
│   │   ├── mod.rs
│   │   ├── particles.rs # From particle_effects.rs
│   │   ├── animation.rs
│   │   └── sprites.rs
│   └── testing/         # Testing Agent owns
│       ├── mod.rs
│       ├── harness.rs   # From test_utils/app.rs
│       ├── benchmarks.rs # From benchmark_config.rs
│       └── helpers.rs   # From test_utils/entities.rs
├── core/                # Shared infrastructure (Gameplay Agent maintains)
│   ├── mod.rs
│   ├── state.rs         # Game state machine + choice system
│   ├── choice_system.rs # ChoiceHandler trait + registry
│   ├── config_loader.rs # Existing
│   ├── collision.rs     # Existing
│   ├── resources.rs     # Existing
│   └── events.rs        # Cross-domain events
├── entities/            # Entity definitions (shared, read-mostly)
│   ├── mod.rs
│   ├── player.rs
│   ├── enemy.rs
│   ├── bullet.rs        # Split from gun.rs
│   └── world.rs
├── main.rs
└── lib.rs
```

---

## Current Codebase Status (Commit dc4ec77)

**Completed Infrastructure:**
- ✅ Headless testing framework (`src/test_utils/`)
- ✅ Performance benchmarks (`src/benchmark_config.rs`, `tests/integration/performance_benchmark.rs`)
- ✅ Gold standard baselines (≥15k fps, ≤67µs frame time)
- ✅ Visibility hierarchy fixes for GPU rendering

**Files that exist and need migration:**

| Current File | Lines | Status |
|--------------|-------|--------|
| `src/test_utils/app.rs` | 63 | Move to domains/testing/harness.rs |
| `src/test_utils/entities.rs` | 94 | Move to domains/testing/helpers.rs |
| `src/benchmark_config.rs` | 31 | Move to domains/testing/benchmarks.rs |
| `src/particle_effects.rs` | ~200 | Move to domains/graphics/particles.rs |
| `src/animation.rs` | ~100 | Move to domains/graphics/animation.rs |
| `src/gui.rs` | ~150 | Move to domains/ui/hud.rs |
| `src/camera.rs` | ~80 | Move to domains/ui/camera.rs |
| `src/configs.rs` | ~100 | Move to domains/gameplay/balance.rs |
| `src/gun.rs` | ~150 | Split: entities/bullet.rs + domains/gameplay/combat.rs |

---

## Agent Ownership Matrix

| Domain/Folder | Primary Owner | Can Modify Without PR | Responsibilities |
|---------------|---------------|----------------------|------------------|
| **domains/ui/** | UI Agent | ✅ Yes | HUD, menus, choice UI, camera follow |
| **domains/gameplay/** | Gameplay Agent | ✅ Yes | Balance, spawning, combat, state machine |
| **domains/graphics/** | Graphics Agent | ✅ Yes | Particles, animations, sprite generation |
| **domains/testing/** | Testing Agent | ✅ Yes | Benchmarks, test harnesses, utilities |
| **core/** | Gameplay Agent | ❌ Requires discussion | State machine, config system, collision |
| **entities/** | Gameplay Agent | ❌ Propose changes | Player, enemy, bullet components |
| **tests/** | Testing Agent | ✅ Yes (maintain gold standards) | Integration tests |

**Shared Code Rule:** `core/` and `entities/` owned by Gameplay Agent but other agents can read. Modifications require discussion to avoid breaking dependencies.

---

## Game State Machine (Auto-Battler Pattern)

### States

```rust
// src/core/state.rs

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    WaveSpawning,         // Spawn wave, auto-transition
    Combat,               // Auto-battle
    PlayerChoice(ChoiceContext),  // ⏸️ PAUSE - Wait for player input
    ApplyingChoice,       // Process selection, resume
    GameOver,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChoiceContext {
    WaveComplete,   // End-of-wave upgrades
    LevelUp,        // XP threshold reached
    LootFound,      // Random loot drop
    ShopEncounter,  // Merchant encounter
    BossReward,     // Boss defeated
    // Extensible: add more as gameplay evolves
}
```

### State Flow

```
Loading → MainMenu → WaveSpawning
                          ↓
                       Combat ←────────────┐
                          ↓                │
    ┌─────────────────────┴────────────────┤
    │                                      │
    ├─→ Wave cleared  → PlayerChoice(WaveComplete) ⏸️
    ├─→ XP threshold  → PlayerChoice(LevelUp) ⏸️
    ├─→ Loot dropped  → PlayerChoice(LootFound) ⏸️
    └─→ Boss killed   → PlayerChoice(BossReward) ⏸️
              ↓
        [Player clicks option]
              ↓
        ApplyingChoice
              ↓
        WaveSpawning (or Combat) ─────────┘
              ↓
          GameOver
```

**Key Behavior:** During `PlayerChoice(*)`, ALL gameplay systems pause (enemies freeze, bullets stop, spawning halts). Only UI systems remain active.

---

## Extensible Choice System

### Core Trait

```rust
// src/core/choice_system.rs

pub trait ChoiceHandler: Send + Sync + 'static {
    /// Generate options for this choice context
    fn generate_options(&self, world: &World) -> Vec<ChoiceOption>;

    /// Apply the selected option
    fn apply_choice(&self, choice: &ChoiceOption, world: &mut World);

    /// Determine next state after applying choice
    fn next_state(&self) -> GameState;
}

#[derive(Resource)]
pub struct ChoiceHandlerRegistry {
    handlers: HashMap<ChoiceContext, Box<dyn ChoiceHandler>>,
}

#[derive(Clone)]
pub struct ChoiceOption {
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub choice_type: ChoiceType,
}

#[derive(Clone)]
pub enum ChoiceType {
    Upgrade { stat: StatType, amount: f32 },
    LootItem { item_id: String },
    Skill { skill_id: String },
    // Extensible
}
```

### Example: Wave Complete Handler

```rust
// domains/gameplay/handlers/wave_complete.rs

pub struct WaveCompleteHandler;

impl ChoiceHandler for WaveCompleteHandler {
    fn generate_options(&self, world: &World) -> Vec<ChoiceOption> {
        vec![
            ChoiceOption {
                id: "damage_boost".into(),
                display_name: "Damage Boost".into(),
                description: "+20% damage".into(),
                choice_type: ChoiceType::Upgrade {
                    stat: StatType::Damage,
                    amount: 1.2,
                },
            },
            ChoiceOption {
                id: "fire_rate".into(),
                display_name: "Faster Firing".into(),
                description: "-0.1s reload time".into(),
                choice_type: ChoiceType::Upgrade {
                    stat: StatType::FireRate,
                    amount: 0.1,
                },
            },
            ChoiceOption {
                id: "health_regen".into(),
                display_name: "Regeneration".into(),
                description: "+5 HP/sec".into(),
                choice_type: ChoiceType::Upgrade {
                    stat: StatType::HealthRegen,
                    amount: 5.0,
                },
            },
        ]
    }

    fn apply_choice(&self, choice: &ChoiceOption, world: &mut World) {
        match &choice.choice_type {
            ChoiceType::Upgrade { stat, amount } => {
                let mut config = world.resource_mut::<GameConfig>();
                match stat {
                    StatType::Damage => config.gun.bullet_damage *= amount,
                    StatType::FireRate => config.gun.bullet_spawn_interval -= amount,
                    StatType::HealthRegen => config.player.health_regen += amount,
                }
            }
            _ => {}
        }
    }

    fn next_state(&self) -> GameState {
        GameState::WaveSpawning  // Start next wave
    }
}
```

---

## Cross-Domain Communication (Bevy 0.17 Patterns)

### Event-Driven Architecture

Domains communicate via **Bevy 0.17 Trigger/Observer pattern** (not old EventReader):

```rust
// src/core/events.rs

#[derive(Event)]
pub struct TriggerChoiceEvent {
    pub context: ChoiceContext,
    pub metadata: ChoiceMetadata,
}

#[derive(Event)]
pub struct ShowChoiceUIEvent {
    pub context: ChoiceContext,
    pub options: Vec<ChoiceOption>,
}

#[derive(Event)]
pub struct ChoiceSelectedEvent {
    pub choice: ChoiceOption,
}

#[derive(Event)]
pub struct HideChoiceUIEvent;
```

### System Scheduling

All domains use `run_if()` to pause during player choices:

```rust
// domains/gameplay/mod.rs
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            // State management (always runs)
            .add_systems(Update, (
                detect_choice_triggers,
                handle_choice_trigger,
            ))

            // Gameplay systems (pause during choices)
            .add_systems(Update, (
                spawn_enemies,
                run_combat,
                update_bullets,
                move_enemies,
            ).run_if(not(is_paused_for_choice)))

            // Observers
            .observe(apply_choice);
    }
}

fn is_paused_for_choice(state: Res<State<GameState>>) -> bool {
    matches!(state.get(), GameState::PlayerChoice(_))
}
```

---

## Migration Strategy

### Phase 1: Create Domain Structure (No Code Movement)

```bash
mkdir -p src/domains/{ui,gameplay,graphics,testing}
mkdir -p src/core
mkdir -p src/entities
touch src/domains/{ui,gameplay,graphics,testing}/mod.rs
touch src/core/{mod.rs,choice_system.rs}
```

### Phase 2: Move Files by Domain

| Current File | New Location | Domain |
|--------------|--------------|--------|
| `src/gui.rs` | `src/domains/ui/hud.rs` | UI |
| `src/camera.rs` | `src/domains/ui/camera.rs` | UI |
| `src/configs.rs` | `src/domains/gameplay/balance.rs` | Gameplay |
| `src/particle_effects.rs` | `src/domains/graphics/particles.rs` | Graphics |
| `src/animation.rs` | `src/domains/graphics/animation.rs` | Graphics |
| `src/test_utils/app.rs` | `src/domains/testing/harness.rs` | Testing |
| `src/test_utils/entities.rs` | `src/domains/testing/helpers.rs` | Testing |
| `src/test_utils/simulation.rs` | `src/domains/testing/simulation.rs` | Testing |
| `src/benchmark_config.rs` | `src/domains/testing/benchmarks.rs` | Testing |
| `src/state.rs` | `src/core/state.rs` | Core (Gameplay maintains) |
| `src/config_loader.rs` | `src/core/config_loader.rs` | Core |
| `src/collision.rs` | `src/core/collision.rs` | Core |
| `src/resources.rs` | `src/core/resources.rs` | Core |
| `src/player.rs` | `src/entities/player.rs` | Entities |
| `src/enemy.rs` | `src/entities/enemy.rs` | Entities |
| `src/gun.rs` | Split: `src/entities/bullet.rs` + `src/domains/gameplay/combat.rs` | Mixed |
| `src/world.rs` | `src/entities/world.rs` | Entities |

**Note:** `src/plugin_mode.rs` stays at root (used by all plugins for headless mode detection).

### Phase 3: Implement Choice System

1. Add `ChoiceContext` enum to `src/core/state.rs`
2. Create `src/core/choice_system.rs` with `ChoiceHandler` trait
3. Implement `WaveCompleteHandler` in `domains/gameplay/`
4. Create `domains/ui/choices.rs` for generic choice UI
5. Wire up observers in both plugins

### Phase 4: Update Plugin Exports

```rust
// src/lib.rs

pub mod domains {
    pub mod ui;
    pub mod gameplay;
    pub mod graphics;
    pub mod testing;
}

pub mod core {
    pub mod state;
    pub mod choice_system;
    pub mod config_loader;
    pub mod collision;
    pub mod resources;
    pub mod events;
}

pub mod entities {
    pub mod player;
    pub mod enemy;
    pub mod bullet;
    pub mod world;
}

// Re-exports
pub use domains::{
    ui::UiPlugin,
    gameplay::GameplayPlugin,
    graphics::GraphicsPlugin,
};

// Keep plugin_mode at root
pub mod plugin_mode;
```

### Phase 5: Verify Tests Pass (MANDATORY)

Following README.md gold standard protocol:

```bash
# 1. Run all tests
cargo test -- --nocapture

# 2. Run headless performance baseline
cargo test test_headless_app_creation -- --nocapture

# 3. Run performance benchmark (new!)
cargo test performance_benchmark -- --nocapture

# 4. Verify compilation
cargo check; cargo build
```

**Gold Standard Baselines (MUST NOT REGRESS):**

| Metric | Baseline | Threshold |
|--------|----------|-----------|
| 1000-frame simulation | ~19,000 fps | ≥15,000 fps |
| Average frame time | ~52 µs | ≤67 µs |
| Enemy movement | 1.0 units/frame | Exact |
| Entity persistence | 100% | 100% |

**Hand-off Report Template:**

```markdown
✅ ALL AUTOMATED TESTS PASSED

Performance Metrics:
- 1000-frame simulation: [ACTUAL] fps (baseline: ≥15,000) [✓/✗]
- Average frame time: [ACTUAL]µs (baseline: ≤67µs) [✓/✗]

Benchmark Results:
- Wave 1 (100 enemies): [ACTUAL] fps [✓/✗]
- Wave 2 (1k enemies): [ACTUAL] fps [✓/✗]
- Wave 3 (10k enemies): [ACTUAL] fps [✓/✗]

Domain Structure:
- domains/ui/ exports UiPlugin ✓
- domains/gameplay/ exports GameplayPlugin ✓
- domains/graphics/ exports GraphicsPlugin ✓
- domains/testing/ contains test_utils + benchmarks ✓
- core/choice_system.rs implemented ✓

Recommendation: [Ready for parallel agent work / Issues detected]
```

---

## Testing Agent Infrastructure (Already Completed)

The Testing Agent already has a strong foundation:

**Existing Tools:**
- `src/test_utils/app.rs` - `create_headless_app()` for test setup
- `src/test_utils/entities.rs` - `spawn_test_enemies()`, `spawn_test_player()`
- `src/test_utils/simulation.rs` - `simulate_frames()` helper
- `src/benchmark_config.rs` - `GameConfig::benchmark_mode()`
- `tests/integration/performance_benchmark.rs` - Full 5-wave benchmark

**After Migration:**
```
domains/testing/
├── mod.rs              # TestingPlugin
├── harness.rs          # create_headless_app, init_for_testing
├── helpers.rs          # spawn_test_enemies, spawn_test_player
├── simulation.rs       # simulate_frames
└── benchmarks.rs       # GameConfig::benchmark_mode
```

**Testing Agent Responsibilities:**
1. Maintain gold standard baselines
2. Add new benchmark scenarios
3. Create domain-specific test utilities
4. Verify no cross-domain pollution
5. Ensure all tests pass before agent hand-offs

---

## Typical Agent Workflows

### UI Agent

**Task:** Add minimap using ViewportNode

**Files touched:** `domains/ui/minimap.rs`

**Dependencies:**
- Read: `entities/player.rs` (player position)
- Read: `core/state.rs` (GameState)
- Event: None (minimap is read-only)

**Systems:**
```rust
.add_systems(Update, update_minimap_camera.run_if(in_state(GameState::Combat)))
```

**Testing:** UI changes shouldn't affect performance benchmarks

---

### Gameplay Agent

**Task:** Add new enemy type "Rusher"

**Files touched:**
- `entities/enemy.rs` (add RusherEnemy component)
- `domains/gameplay/spawning.rs` (spawn logic)
- `domains/gameplay/balance.rs` (rusher stats)

**Dependencies:**
- Trigger: `EnemySpawnedEvent` (for UI/Graphics to observe)

**Testing requirement:** Update benchmarks to test with rushers, verify gold standards hold

---

### Graphics Agent

**Task:** Add impact particles on bullet hit

**Files touched:** `domains/graphics/particles.rs`

**Dependencies:**
- Read: `core/events.rs` (observe `BulletHitEvent`)
- Read: `core/resources.rs` (particle assets)

**Testing:** Run benchmarks to verify particle FPS impact within thresholds

---

### Testing Agent

**Task:** Add boss wave benchmark

**Files touched:**
- `domains/testing/benchmarks.rs` (boss config)
- `tests/integration/boss_benchmark.rs` (new test)

**Dependencies:**
- Read: ALL domains (validates entire system)
- Use: `domains/testing/harness.rs` helpers

---

## Benefits of This Architecture

1. **Parallel Work:** Each agent owns folders, minimal file overlap
2. **Clean Boundaries:** Events enforce one-way communication
3. **Extensible:** Add new choice contexts without touching UI code
4. **Testable:** Each domain can be tested independently
5. **Pauseable:** Single `PlayerChoice` state pauses ALL gameplay
6. **Type-Safe:** Rust ensures cross-domain contracts via events
7. **Proven Infrastructure:** Testing tools already validate this works
8. **Maintainable:** Ownership matrix prevents ambiguity

---

## References

- **Bevy 0.17 Features:** See `latest-rs-bevy-features` skill
- **Event/Observer Pattern:** Bevy 0.17 Trigger system (not EventReader)
- **Gold Standard Tests:** `README.md` testing section
- **Performance Baselines:** Commit dc4ec77 (benchmark infrastructure)
- **Headless Testing:** Commit 20b32bb (test_utils infrastructure)
- **ViewportNode:** For minimap/tactical view (stable in 0.17)

---

## Success Criteria

- [ ] All 4 domains created with proper exports
- [ ] Choice system implemented with ≥2 handlers
- [ ] All tests pass (gold standard maintained)
- [ ] Performance benchmarks still run and pass
- [ ] No agent can accidentally break another agent's domain
- [ ] New choice contexts can be added in <30 minutes
- [ ] Parallel agents can work on separate branches without conflicts

---

## Notes

- **Ownership discipline:** Agents MUST respect ownership boundaries
- **Testing first:** Changes that break gold standards MUST be reverted
- **Event-driven:** NO direct function calls between domains
- **YAGNI:** Don't implement all choice contexts upfront, add as needed
- **Bevy 0.17:** Use new Trigger/Observer, not old EventReader
- **Build on existing:** Leverage test_utils and benchmark_config already in place
