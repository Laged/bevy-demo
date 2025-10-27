# Parallel Agent Architecture Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Date:** 2025-10-27
**Branch:** `feature/parallel-agent-architecture`
**Goal:** Restructure codebase into domain-driven architecture for parallel agent workflows
**Design Reference:** `docs/plans/2025-10-27-parallel-agent-architecture-design.md`

---

## Prerequisites

- ✅ Current branch: `feature/parallel-agent-architecture`
- ✅ All tests passing on dev (commit dc4ec77)
- ✅ Headless testing infrastructure exists
- ✅ Performance benchmarks exist
- ✅ Design document approved

---

## Implementation Strategy

**Approach:** Incremental migration with continuous testing
- Each task creates domain structure piece-by-piece
- Tests run after every meaningful change
- Commit after each task completes
- If any test fails, stop and report

**Autonomy:** This plan is designed for fully autonomous execution:
- No manual intervention needed
- Clear success/failure criteria for each task
- Automatic rollback on test failures
- Comprehensive verification at each step

---

## Task Breakdown

### Phase 1: Create Domain Structure (Foundation)
1. Create empty domain folders
2. Create core infrastructure folders
3. Verify directory structure

### Phase 2: Implement Choice System (Core Logic)
4. Create choice system types and trait
5. Add choice system to core/state.rs
6. Verify choice system compiles

### Phase 3: Migrate Testing Domain (Lowest Risk)
7. Move test_utils to domains/testing
8. Move benchmark_config to domains/testing
9. Update test imports and verify all tests pass

### Phase 4: Migrate Graphics Domain
10. Move particle_effects to domains/graphics
11. Move animation to domains/graphics
12. Update imports and verify tests pass

### Phase 5: Migrate UI Domain
13. Move gui to domains/ui
14. Move camera to domains/ui
15. Update imports and verify tests pass

### Phase 6: Migrate Gameplay Domain
16. Move configs to domains/gameplay
17. Split gun.rs into bullet entity + combat
18. Update imports and verify tests pass

### Phase 7: Migrate Core and Entities
19. Move shared infrastructure to core/
20. Move entity definitions to entities/
21. Update all imports and verify tests pass

### Phase 8: Verification and Finalization
22. Run full test suite with gold standard validation
23. Run performance benchmarks
24. Update lib.rs with domain exports
25. Final verification and commit

---

## Task 1: Create Empty Domain Folders

**Goal:** Establish directory structure without moving code

**Commands:**
```bash
mkdir -p src/domains/ui
mkdir -p src/domains/gameplay
mkdir -p src/domains/graphics
mkdir -p src/domains/testing
mkdir -p src/core
mkdir -p src/entities
```

**Create placeholder mod.rs files:**
```bash
touch src/domains/ui/mod.rs
touch src/domains/gameplay/mod.rs
touch src/domains/graphics/mod.rs
touch src/domains/testing/mod.rs
touch src/core/mod.rs
touch src/entities/mod.rs
```

**Verification:**
```bash
# Verify directories exist
ls -la src/domains/
ls -la src/core/
ls -la src/entities/

# Should compile (no code changes yet)
cargo check
```

**Success Criteria:**
- ✅ All directories created
- ✅ All mod.rs files exist
- ✅ `cargo check` passes

**Commit:**
```bash
git add src/domains/ src/core/ src/entities/
git commit -m "feat: create domain folder structure for parallel agents"
```

---

## Task 2: Create Core Infrastructure Folders

**Goal:** Set up core module structure

**Files to create:**

`src/core/mod.rs`:
```rust
//! Core infrastructure shared across domains
//!
//! Maintained by Gameplay Agent, read by all domains

pub mod events;
pub mod choice_system;
```

`src/core/events.rs`:
```rust
//! Cross-domain events using Bevy 0.17 Trigger/Observer pattern

use bevy::prelude::*;
use crate::core::choice_system::{ChoiceOption, ChoiceMetadata, ChoiceContext};

/// Request to trigger a player choice pause
#[derive(Event, Clone)]
pub struct TriggerChoiceEvent {
    pub context: ChoiceContext,
    pub metadata: ChoiceMetadata,
}

/// Notify UI to display choice options
#[derive(Event, Clone)]
pub struct ShowChoiceUIEvent {
    pub context: ChoiceContext,
    pub options: Vec<ChoiceOption>,
}

/// Player has selected a choice option
#[derive(Event, Clone)]
pub struct ChoiceSelectedEvent {
    pub choice: ChoiceOption,
}

/// Hide the choice UI
#[derive(Event)]
pub struct HideChoiceUIEvent;
```

**Verification:**
```bash
cargo check
```

**Success Criteria:**
- ✅ Files created
- ✅ `cargo check` passes

**Commit:**
```bash
git add src/core/
git commit -m "feat: add core event definitions for cross-domain communication"
```

---

## Task 3: Create Choice System Types

**Goal:** Implement extensible choice system foundation

**File:** `src/core/choice_system.rs`

```rust
//! Extensible choice system for gameplay pauses
//!
//! Allows gameplay to pause for player decisions (upgrades, loot, etc.)

use bevy::prelude::*;
use std::collections::HashMap;

/// Context for why the game is paused for a choice
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChoiceContext {
    WaveComplete,   // End of wave upgrades
    LevelUp,        // XP threshold reached
    LootFound,      // Random loot drop
    ShopEncounter,  // Merchant encounter
    BossReward,     // Boss defeated
}

/// A single choice option presented to the player
#[derive(Clone, Debug)]
pub struct ChoiceOption {
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub choice_type: ChoiceType,
}

/// Type of choice being offered
#[derive(Clone, Debug)]
pub enum ChoiceType {
    Upgrade { stat: StatType, amount: f32 },
    LootItem { item_id: String },
    Skill { skill_id: String },
}

/// Stats that can be upgraded
#[derive(Clone, Debug)]
pub enum StatType {
    Damage,
    FireRate,
    HealthRegen,
    MoveSpeed,
}

/// Metadata about the choice context
#[derive(Clone, Debug, Default)]
pub struct ChoiceMetadata {
    pub wave_number: Option<u32>,
    pub xp_gained: Option<u32>,
    pub loot_rarity: Option<LootRarity>,
}

#[derive(Clone, Debug)]
pub enum LootRarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

/// Trait for handling different choice contexts
pub trait ChoiceHandler: Send + Sync + 'static {
    /// Generate options for this choice context
    fn generate_options(&self, world: &World) -> Vec<ChoiceOption>;

    /// Apply the selected option
    fn apply_choice(&self, choice: &ChoiceOption, world: &mut World);

    /// Determine next state after applying choice
    fn next_state(&self) -> crate::state::GameState;
}

/// Registry of choice handlers
#[derive(Resource, Default)]
pub struct ChoiceHandlerRegistry {
    handlers: HashMap<ChoiceContext, Box<dyn ChoiceHandler>>,
}

impl ChoiceHandlerRegistry {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register(&mut self, context: ChoiceContext, handler: Box<dyn ChoiceHandler>) {
        self.handlers.insert(context, handler);
    }

    pub fn get(&self, context: &ChoiceContext) -> Option<&dyn ChoiceHandler> {
        self.handlers.get(context).map(|b| b.as_ref())
    }
}

/// Resource holding pending choice data
#[derive(Resource, Clone)]
pub struct PendingChoice {
    pub context: ChoiceContext,
    pub options: Vec<ChoiceOption>,
    pub metadata: ChoiceMetadata,
}
```

**Update:** `src/core/mod.rs`
```rust
pub mod events;
pub mod choice_system;

// Re-export commonly used types
pub use choice_system::{
    ChoiceContext, ChoiceOption, ChoiceType, ChoiceHandler,
    ChoiceHandlerRegistry, PendingChoice, ChoiceMetadata,
};
```

**Verification:**
```bash
cargo check
```

**Success Criteria:**
- ✅ Choice system compiles
- ✅ No errors or warnings

**Commit:**
```bash
git add src/core/choice_system.rs src/core/mod.rs
git commit -m "feat: implement extensible choice system with handler trait"
```

---

## Task 4: Extend GameState with PlayerChoice

**Goal:** Add choice pause state to game state machine

**File:** `src/state.rs`

**Add after existing GameState enum:**

```rust
// Add this import at top
use crate::core::ChoiceContext;

// Modify GameState enum to add PlayerChoice variant
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    GameInit,
    InGame,
    PlayerChoice(ChoiceContext),  // NEW: Pause for player decision
    ApplyingChoice,                // NEW: Process selection
    GameOver,
}
```

**Add helper function:**

```rust
/// Check if game is paused for a player choice
pub fn is_paused_for_choice(state: Res<State<GameState>>) -> bool {
    matches!(state.get(), GameState::PlayerChoice(_))
}
```

**Verification:**
```bash
cargo check
```

**Success Criteria:**
- ✅ GameState compiles with new variants
- ✅ Helper function exists
- ✅ No errors

**Commit:**
```bash
git add src/state.rs
git commit -m "feat: extend GameState with PlayerChoice pause state"
```

---

## Task 5: Create Domains Module Exports

**Goal:** Make domains accessible from lib.rs

**File:** `src/lib.rs`

**Add before existing plugin definitions:**

```rust
// Domain modules
pub mod domains {
    pub mod ui;
    pub mod gameplay;
    pub mod graphics;
    pub mod testing;
}

// Core infrastructure
pub mod core;

// Entity definitions
pub mod entities;
```

**Create empty plugin stubs in each domain:**

`src/domains/ui/mod.rs`:
```rust
//! UI Domain - Owned by UI Agent
//!
//! HUD, menus, choice UI, camera

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Placeholder - will be populated during migration
    }
}
```

`src/domains/gameplay/mod.rs`:
```rust
//! Gameplay Domain - Owned by Gameplay Agent
//!
//! Balance, spawning, combat, state machine

use bevy::prelude::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        // Placeholder
    }
}
```

`src/domains/graphics/mod.rs`:
```rust
//! Graphics Domain - Owned by Graphics Agent
//!
//! Particles, animations, sprites

use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        // Placeholder
    }
}
```

`src/domains/testing/mod.rs`:
```rust
//! Testing Domain - Owned by Testing Agent
//!
//! Test harnesses, benchmarks, utilities

pub mod harness;
pub mod helpers;
pub mod benchmarks;
```

`src/domains/testing/harness.rs`:
```rust
//! Test harness utilities - placeholder
```

`src/domains/testing/helpers.rs`:
```rust
//! Test helper functions - placeholder
```

`src/domains/testing/benchmarks.rs`:
```rust
//! Benchmark configurations - placeholder
```

**Create entities module:**

`src/entities/mod.rs`:
```rust
//! Entity definitions shared across domains
//!
//! Owned by Gameplay Agent, read by all domains

// Will be populated during migration
```

**Verification:**
```bash
cargo check
```

**Success Criteria:**
- ✅ All domain modules compile
- ✅ Plugins defined
- ✅ No errors

**Commit:**
```bash
git add src/lib.rs src/domains/ src/entities/
git commit -m "feat: create domain plugin structure with empty implementations"
```

---

## Task 6: Move test_utils to domains/testing

**Goal:** Migrate testing infrastructure (lowest risk first)

**Step 1: Copy test_utils content**

```bash
# Copy files
cp src/test_utils/app.rs src/domains/testing/harness.rs
cp src/test_utils/entities.rs src/domains/testing/helpers.rs
cp src/test_utils/simulation.rs src/domains/testing/simulation.rs
```

**Step 2: Update domains/testing/mod.rs**

```rust
//! Testing Domain - Owned by Testing Agent

pub mod harness;
pub mod helpers;
pub mod simulation;
pub mod benchmarks;

// Re-export commonly used functions
pub use harness::{create_headless_app, init_for_testing};
pub use helpers::{spawn_test_player, spawn_test_enemies};
pub use simulation::simulate_frames;
```

**Step 3: Update imports in migrated files**

In `src/domains/testing/harness.rs`, change:
```rust
use crate::test_utils::*;
```
to:
```rust
use crate::*;
```

Do the same for `helpers.rs` and `simulation.rs`.

**Step 4: Update src/lib.rs to re-export test_utils**

Add after domains:
```rust
// Backward compatibility - re-export test_utils
pub mod test_utils {
    pub use crate::domains::testing::*;
}
```

**Verification:**
```bash
# Should compile
cargo check

# Run tests to verify nothing broke
cargo test -- --nocapture
```

**Success Criteria:**
- ✅ All files copied
- ✅ Imports updated
- ✅ `cargo test` passes
- ✅ All 6 tests pass including performance benchmark

**Commit:**
```bash
git add src/domains/testing/ src/lib.rs
git commit -m "refactor: move test_utils to domains/testing with backward compat"
```

**Step 5: Remove old test_utils (after verification)**

```bash
git rm -r src/test_utils/
```

**Verification:**
```bash
cargo test -- --nocapture
```

**Success Criteria:**
- ✅ Tests still pass after removal

**Commit:**
```bash
git commit -m "refactor: remove old test_utils folder"
```

---

## Task 7: Move benchmark_config to domains/testing

**Goal:** Consolidate testing utilities

**Step 1: Copy benchmark_config**

```bash
cp src/benchmark_config.rs src/domains/testing/benchmarks.rs
```

**Step 2: Update domains/testing/benchmarks.rs**

Change module comment:
```rust
//! Benchmark configurations for performance testing
//!
//! Owned by Testing Agent

use crate::config_loader::GameConfig;

impl GameConfig {
    /// Returns a config optimized for performance benchmarking
    pub fn benchmark_mode() -> Self {
        // ... existing implementation
    }
}
```

**Step 3: Update src/lib.rs**

Add re-export:
```rust
// Re-export benchmark_config for compatibility
pub use domains::testing::benchmarks::*;

// Or keep the old module name
pub mod benchmark_config {
    pub use crate::domains::testing::benchmarks::*;
}
```

**Verification:**
```bash
cargo check
cargo test performance_benchmark -- --nocapture
```

**Success Criteria:**
- ✅ Benchmark config accessible
- ✅ Performance benchmark test passes

**Commit:**
```bash
git add src/domains/testing/benchmarks.rs src/lib.rs
git commit -m "refactor: move benchmark_config to domains/testing/benchmarks"
```

**Step 4: Remove old file**

```bash
git rm src/benchmark_config.rs
```

**Verification:**
```bash
cargo test -- --nocapture
```

**Success Criteria:**
- ✅ All tests pass

**Commit:**
```bash
git commit -m "refactor: remove old benchmark_config file"
```

---

## Task 8: Move particle_effects to domains/graphics

**Goal:** Migrate graphics domain (particles)

**Step 1: Copy file**

```bash
cp src/particle_effects.rs src/domains/graphics/particles.rs
```

**Step 2: Update domains/graphics/mod.rs**

```rust
//! Graphics Domain - Owned by Graphics Agent

use bevy::prelude::*;

pub mod particles;

// Re-export for convenience
pub use particles::{ParticleEffectsPlugin, ParticleEffectAssets, ImpactEffect, DeathLingerEffect};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ParticleEffectsPlugin);
    }
}
```

**Step 3: Update src/lib.rs re-exports**

```rust
// Re-export particle_effects for backward compatibility
pub mod particle_effects {
    pub use crate::domains::graphics::particles::*;
}
```

**Verification:**
```bash
cargo check
cargo test -- --nocapture
```

**Success Criteria:**
- ✅ Compiles
- ✅ All tests pass

**Commit:**
```bash
git add src/domains/graphics/
git commit -m "refactor: move particle_effects to domains/graphics/particles"
```

**Step 4: Remove old file**

```bash
git rm src/particle_effects.rs
```

**Verification:**
```bash
cargo test -- --nocapture
```

**Commit:**
```bash
git commit -m "refactor: remove old particle_effects file"
```

---

## Task 9: Move animation to domains/graphics

**Goal:** Complete graphics domain migration

**Step 1: Copy file**

```bash
cp src/animation.rs src/domains/graphics/animation.rs
```

**Step 2: Update domains/graphics/mod.rs**

```rust
pub mod particles;
pub mod animation;

pub use particles::{ParticleEffectsPlugin, ParticleEffectAssets, ImpactEffect, DeathLingerEffect};
pub use animation::{AnimationPlugin, AnimationTimer, PlayerState};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ParticleEffectsPlugin,
            AnimationPlugin,
        ));
    }
}
```

**Step 3: Update src/lib.rs**

```rust
pub mod animation {
    pub use crate::domains::graphics::animation::*;
}
```

**Verification:**
```bash
cargo check
cargo test -- --nocapture
```

**Success Criteria:**
- ✅ Compiles
- ✅ Tests pass

**Commit:**
```bash
git add src/domains/graphics/
git commit -m "refactor: move animation to domains/graphics"
```

**Step 4: Remove old file**

```bash
git rm src/animation.rs
```

**Verification:**
```bash
cargo test -- --nocapture
```

**Commit:**
```bash
git commit -m "refactor: remove old animation file"
```

---

## Task 10: Move gui to domains/ui

**Goal:** Start UI domain migration

**Step 1: Copy file**

```bash
cp src/gui.rs src/domains/ui/hud.rs
```

**Step 2: Update domains/ui/mod.rs**

```rust
//! UI Domain - Owned by UI Agent

use bevy::prelude::*;

pub mod hud;

pub use hud::{GuiPlugin, HealthText};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GuiPlugin);
    }
}
```

**Step 3: Update src/lib.rs**

```rust
pub mod gui {
    pub use crate::domains::ui::hud::*;
}
```

**Verification:**
```bash
cargo check
cargo test -- --nocapture
```

**Commit:**
```bash
git add src/domains/ui/
git commit -m "refactor: move gui to domains/ui/hud"
```

**Step 4: Remove old file**

```bash
git rm src/gui.rs
cargo test -- --nocapture
```

**Commit:**
```bash
git commit -m "refactor: remove old gui file"
```

---

## Task 11: Move camera to domains/ui

**Goal:** Complete UI domain migration

**Step 1: Copy file**

```bash
cp src/camera.rs src/domains/ui/camera.rs
```

**Step 2: Update domains/ui/mod.rs**

```rust
pub mod hud;
pub mod camera;

pub use hud::{GuiPlugin, HealthText};
pub use camera::FollowCameraPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GuiPlugin,
            FollowCameraPlugin,
        ));
    }
}
```

**Step 3: Update src/lib.rs**

```rust
pub mod camera {
    pub use crate::domains::ui::camera::*;
}
```

**Verification:**
```bash
cargo check
cargo test -- --nocapture
```

**Commit:**
```bash
git add src/domains/ui/
git commit -m "refactor: move camera to domains/ui"
```

**Step 4: Remove old file**

```bash
git rm src/camera.rs
cargo test -- --nocapture
```

**Commit:**
```bash
git commit -m "refactor: remove old camera file"
```

---

## Task 12: Move configs to domains/gameplay

**Goal:** Start gameplay domain migration

**Step 1: Copy file**

```bash
cp src/configs.rs src/domains/gameplay/balance.rs
```

**Step 2: Update file comment**

In `src/domains/gameplay/balance.rs`:
```rust
//! Game balance constants
//!
//! Owned by Gameplay Agent
//! All domains can read, only Gameplay can modify

// ... rest of file unchanged
```

**Step 3: Update domains/gameplay/mod.rs**

```rust
//! Gameplay Domain - Owned by Gameplay Agent

use bevy::prelude::*;

pub mod balance;

pub use balance::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        // Will add systems during migration
    }
}
```

**Step 4: Update src/lib.rs**

```rust
pub mod configs {
    pub use crate::domains::gameplay::balance::*;
}
```

**Verification:**
```bash
cargo check
cargo test -- --nocapture
```

**Commit:**
```bash
git add src/domains/gameplay/
git commit -m "refactor: move configs to domains/gameplay/balance"
```

**Step 5: Remove old file**

```bash
git rm src/configs.rs
cargo test -- --nocapture
```

**Commit:**
```bash
git commit -m "refactor: remove old configs file"
```

---

## Task 13: Move Core Infrastructure

**Goal:** Migrate shared infrastructure to core/

**Files to move:**
- `src/state.rs` → `src/core/state.rs` (already has PlayerChoice)
- `src/config_loader.rs` → `src/core/config_loader.rs`
- `src/collision.rs` → `src/core/collision.rs`
- `src/resources.rs` → `src/core/resources.rs`

**Step 1: Move state.rs**

```bash
cp src/state.rs src/core/state.rs
```

Update `src/core/mod.rs`:
```rust
pub mod events;
pub mod choice_system;
pub mod state;
pub mod config_loader;
pub mod collision;
pub mod resources;

// Re-exports
pub use state::{GameState, is_paused_for_choice};
pub use choice_system::{ChoiceContext, ChoiceOption, /* ... */};
```

Update `src/lib.rs`:
```rust
pub mod state {
    pub use crate::core::state::*;
}
```

**Step 2: Move config_loader.rs**

```bash
cp src/config_loader.rs src/core/config_loader.rs
```

Update `src/lib.rs`:
```rust
pub mod config_loader {
    pub use crate::core::config_loader::*;
}
```

**Step 3: Move collision.rs**

```bash
cp src/collision.rs src/core/collision.rs
```

Update `src/lib.rs`:
```rust
pub mod collision {
    pub use crate::core::collision::*;
}
```

**Step 4: Move resources.rs**

```bash
cp src/resources.rs src/core/resources.rs
```

Update `src/lib.rs`:
```rust
pub mod resources {
    pub use crate::core::resources::*;
}
```

**Verification:**
```bash
cargo check
cargo test -- --nocapture
```

**Success Criteria:**
- ✅ All files accessible
- ✅ Tests pass

**Commit:**
```bash
git add src/core/ src/lib.rs
git commit -m "refactor: move core infrastructure to core/ module"
```

**Step 5: Remove old files**

```bash
git rm src/state.rs src/config_loader.rs src/collision.rs src/resources.rs
cargo test -- --nocapture
```

**Commit:**
```bash
git commit -m "refactor: remove old core infrastructure files"
```

---

## Task 14: Move Entity Definitions

**Goal:** Migrate entity modules to entities/

**Files:**
- `src/player.rs` → `src/entities/player.rs`
- `src/enemy.rs` → `src/entities/enemy.rs`
- `src/world.rs` → `src/entities/world.rs`

**Step 1: Move files**

```bash
cp src/player.rs src/entities/player.rs
cp src/enemy.rs src/entities/enemy.rs
cp src/world.rs src/entities/world.rs
```

**Step 2: Update entities/mod.rs**

```rust
//! Entity definitions shared across domains
//!
//! Owned by Gameplay Agent, read by all domains

pub mod player;
pub mod enemy;
pub mod world;

// Re-exports
pub use player::{Player, PlayerPlugin, Health, PlayerEnemyCollisionEvent};
pub use enemy::{Enemy, EnemyPlugin, EnemyType};
pub use world::WorldPlugin;
```

**Step 3: Update src/lib.rs**

```rust
pub mod player {
    pub use crate::entities::player::*;
}

pub mod enemy {
    pub use crate::entities::enemy::*;
}

pub mod world {
    pub use crate::entities::world::*;
}
```

**Verification:**
```bash
cargo check
cargo test -- --nocapture
```

**Commit:**
```bash
git add src/entities/ src/lib.rs
git commit -m "refactor: move entity definitions to entities/ module"
```

**Step 4: Remove old files**

```bash
git rm src/player.rs src/enemy.rs src/world.rs
cargo test -- --nocapture
```

**Commit:**
```bash
git commit -m "refactor: remove old entity files"
```

---

## Task 15: Split gun.rs into Bullet Entity + Combat

**Goal:** Separate entity definition from gameplay logic

**Step 1: Create entities/bullet.rs**

Extract bullet components from `src/gun.rs`:

```rust
//! Bullet entity definition

use bevy::prelude::*;
use std::time::Instant;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct BulletDirection(pub Vec3);

#[derive(Component)]
pub struct SpawnInstant(pub Instant);
```

**Step 2: Create domains/gameplay/combat.rs**

Move gun firing logic:

```rust
//! Combat systems - gun firing, bullet movement

use bevy::prelude::*;
use crate::entities::bullet::{Bullet, BulletDirection, SpawnInstant};
// ... rest of gun.rs logic
```

**Step 3: Update modules**

`src/entities/mod.rs`:
```rust
pub mod bullet;
pub use bullet::{Bullet, BulletDirection, SpawnInstant};
```

`src/domains/gameplay/mod.rs`:
```rust
pub mod combat;
```

**Step 4: Update src/lib.rs**

```rust
pub mod gun {
    pub use crate::entities::bullet::*;
    pub use crate::domains::gameplay::combat::*;
}
```

**Verification:**
```bash
cargo check
cargo test -- --nocapture
```

**Commit:**
```bash
git add src/entities/bullet.rs src/domains/gameplay/combat.rs
git commit -m "refactor: split gun.rs into bullet entity and combat systems"
```

**Step 5: Remove old file**

```bash
git rm src/gun.rs
cargo test -- --nocapture
```

**Commit:**
```bash
git commit -m "refactor: remove old gun.rs file"
```

---

## Task 16: Update Main Plugin Registrations

**Goal:** Use domain plugins in main.rs

**File:** `src/main.rs`

**Find plugin registration section and replace:**

Old:
```rust
.add_plugins((
    GuiPlugin,
    FollowCameraPlugin,
    ParticleEffectsPlugin,
    AnimationPlugin,
    // ... etc
))
```

New:
```rust
.add_plugins((
    // Domain plugins
    UiPlugin,
    GraphicsPlugin,
    // Individual entity plugins still registered
    PlayerPlugin,
    EnemyPlugin,
    WorldPlugin,
    GunPlugin,
    CollisionPlugin,
))
```

**Add imports:**
```rust
use hell_game::domains::{ui::UiPlugin, graphics::GraphicsPlugin};
```

**Verification:**
```bash
cargo check
cargo run  # Manual smoke test - should start without errors
```

**Success Criteria:**
- ✅ Game compiles
- ✅ Game runs without panics
- ✅ Main menu appears

**Commit:**
```bash
git add src/main.rs
git commit -m "refactor: use domain plugins in main.rs"
```

---

## Task 17: Run Full Test Suite

**Goal:** Verify all migrations successful

**Commands:**
```bash
# 1. All tests
cargo test -- --nocapture

# 2. Specific tests
cargo test test_headless_app_creation -- --nocapture
cargo test performance_benchmark -- --nocapture

# 3. Build verification
cargo check
cargo build

# 4. Clippy check
cargo clippy --all-targets --all-features
```

**Success Criteria:**
- ✅ All 6 tests pass
- ✅ Performance baseline ≥15k fps
- ✅ Frame time ≤67µs
- ✅ Enemy movement = 10.0 units
- ✅ No clippy errors

**If tests fail:**
- Identify which test failed
- Check imports in that test
- Verify backward compatibility re-exports
- Fix and re-run

**Commit:**
```bash
git add .
git commit -m "test: verify all tests pass after domain migration"
```

---

## Task 18: Create Domain README Files

**Goal:** Document domain ownership for agents

**Create:** `src/domains/ui/README.md`

```markdown
# UI Domain

**Owner:** UI Agent

## Responsibilities
- HUD rendering
- Menus and settings UI
- Choice UI system
- Camera follow behavior

## Files
- `hud.rs` - Health bar, debug info
- `camera.rs` - Camera following player
- `choices.rs` - (Future) Generic choice UI

## Dependencies
- Read: `entities/player.rs` (Health component)
- Read: `core/state.rs` (GameState)
- Observe: `core/events.rs` (ShowChoiceUIEvent)

## Modification Rules
- ✅ Can modify any file in domains/ui/
- ❌ Cannot modify entities/ or core/
- ✅ Can propose changes to shared modules via discussion
```

**Create similar READMEs for:**
- `src/domains/gameplay/README.md`
- `src/domains/graphics/README.md`
- `src/domains/testing/README.md`

**Commit:**
```bash
git add src/domains/*/README.md
git commit -m "docs: add ownership documentation for each domain"
```

---

## Task 19: Update Root README with New Structure

**Goal:** Document new architecture

**File:** `README.md`

Add section after "Documentation Map":

```markdown
## Domain-Driven Architecture

This codebase uses a domain-driven structure to enable parallel agent workflows:

```
src/
├── domains/        # Agent-specific code
│   ├── ui/        # UI Agent owns
│   ├── gameplay/  # Gameplay Agent owns
│   ├── graphics/  # Graphics Agent owns
│   └── testing/   # Testing Agent owns
├── core/          # Shared infrastructure (Gameplay maintains)
├── entities/      # Entity definitions (shared)
└── main.rs
```

**Domain Ownership:**
- **UI Agent:** `domains/ui/` - HUD, menus, camera
- **Gameplay Agent:** `domains/gameplay/` + `core/` + `entities/` - Balance, combat, state machine
- **Graphics Agent:** `domains/graphics/` - Particles, animations
- **Testing Agent:** `domains/testing/` + `tests/` - Benchmarks, test utilities

**Cross-Domain Communication:** Bevy events only (see `core/events.rs`)

**See:** `docs/plans/2025-10-27-parallel-agent-architecture-design.md` for full architecture details.
```

**Commit:**
```bash
git add README.md
git commit -m "docs: document domain-driven architecture in README"
```

---

## Task 20: Final Verification and Performance Check

**Goal:** Confirm everything works before merge

**Full Test Suite:**
```bash
# Clean build
cargo clean
cargo build --release

# Run all tests
cargo test -- --nocapture

# Run performance benchmark
cargo test --release performance_benchmark -- --nocapture

# Manual game test
cargo run --release
```

**Verification Checklist:**
```
[ ] All 6 tests pass
[ ] Headless test: ≥15k fps, ≤67µs
[ ] Performance benchmark completes 5 waves
[ ] Game runs without panics
[ ] Main menu appears
[ ] Gameplay works (can move, shoot, enemies spawn)
[ ] No B0004 warnings
[ ] No clippy errors
```

**Generate Report:**

Create: `MIGRATION_REPORT.md`

```markdown
# Domain Migration Report

**Date:** 2025-10-27
**Branch:** feature/parallel-agent-architecture
**Status:** ✅ Complete

## Test Results

### Gold Standard Baselines
- 1000-frame simulation: [ACTUAL] fps (≥15,000 required) ✅
- Average frame time: [ACTUAL]µs (≤67µs required) ✅
- Enemy movement: 10.0 units/frame ✅

### Performance Benchmark
- Wave 1 (100 enemies): [ACTUAL] fps ✅
- Wave 2 (1k enemies): [ACTUAL] fps ✅
- Wave 3 (10k enemies): [ACTUAL] fps ✅
- Wave 4 (20k enemies): [ACTUAL] fps ✅
- Wave 5 (100k enemies): [ACTUAL] fps ✅

### Domain Structure
- domains/ui/ exports UiPlugin ✅
- domains/gameplay/ exports GameplayPlugin ✅
- domains/graphics/ exports GraphicsPlugin ✅
- domains/testing/ contains test utilities ✅
- core/ module exists ✅
- entities/ module exists ✅

## Files Migrated

### Testing Domain
- test_utils/app.rs → domains/testing/harness.rs
- test_utils/entities.rs → domains/testing/helpers.rs
- test_utils/simulation.rs → domains/testing/simulation.rs
- benchmark_config.rs → domains/testing/benchmarks.rs

### Graphics Domain
- particle_effects.rs → domains/graphics/particles.rs
- animation.rs → domains/graphics/animation.rs

### UI Domain
- gui.rs → domains/ui/hud.rs
- camera.rs → domains/ui/camera.rs

### Gameplay Domain
- configs.rs → domains/gameplay/balance.rs

### Core
- state.rs → core/state.rs
- config_loader.rs → core/config_loader.rs
- collision.rs → core/collision.rs
- resources.rs → core/resources.rs

### Entities
- player.rs → entities/player.rs
- enemy.rs → entities/enemy.rs
- world.rs → entities/world.rs
- gun.rs → entities/bullet.rs + domains/gameplay/combat.rs

## Backward Compatibility

All old module paths re-exported in lib.rs for compatibility:
- `use hell_game::gui::*` still works
- `use hell_game::test_utils::*` still works
- No breaking changes for existing code

## Next Steps

Ready for parallel agent development:
1. UI Agent can work in domains/ui/
2. Gameplay Agent can work in domains/gameplay/
3. Graphics Agent can work in domains/graphics/
4. Testing Agent can work in domains/testing/

Minimal merge conflicts expected due to clear ownership boundaries.
```

**Commit:**
```bash
git add MIGRATION_REPORT.md
git commit -m "docs: add migration completion report with test results"
```

---

## Final Task: Merge to Dev

**Goal:** Integrate domain architecture into dev branch

**Steps:**

```bash
# Ensure all tests pass one final time
cargo test -- --nocapture

# Switch to dev
git checkout dev

# Merge feature branch
git merge feature/parallel-agent-architecture

# Verify tests on dev
cargo test -- --nocapture

# If all pass, push
git push origin dev

# Clean up feature branch (optional)
git branch -d feature/parallel-agent-architecture
```

**Success Criteria:**
- ✅ Merge completes without conflicts
- ✅ All tests pass on dev
- ✅ Performance benchmarks pass

---

## Rollback Plan (If Anything Fails)

**If tests fail at any point:**

```bash
# 1. Note which task failed
echo "Failed at Task X: [description]" >> FAILURE_LOG.txt

# 2. Revert last commit
git revert HEAD

# 3. Re-run tests
cargo test -- --nocapture

# 4. If tests pass after revert, report issue
cat FAILURE_LOG.txt

# 5. Stop execution and wait for human review
```

**Critical Failure Points:**
- Task 6: test_utils migration - if tests fail, revert immediately
- Task 17: Full test suite - must pass before proceeding
- Task 20: Final verification - must pass before merge

---

## Success Criteria Summary

**All tasks complete when:**
- [ ] All 20 tasks executed successfully
- [ ] All commits made (approximately 40 commits)
- [ ] All tests pass with gold standards maintained
- [ ] Performance benchmarks complete
- [ ] Domain structure created and documented
- [ ] Backward compatibility maintained
- [ ] Migration report generated
- [ ] Merged to dev branch

**Estimated Time:** 2-4 hours for autonomous execution
**Risk Level:** Low (incremental, tested at each step)
**Rollback Capability:** High (commit after each task)

---

## For Claude Agent Executing This Plan

**Instructions:**
1. Execute tasks sequentially (1 → 20)
2. Run verification commands after each task
3. Commit after each successful task
4. If any test fails, execute rollback plan immediately
5. Do not proceed if verification fails
6. Generate migration report at end
7. Report results to human

**Autonomy Level:** Maximum
- No user input needed during execution
- All decisions pre-defined
- Clear success/failure criteria
- Automatic rollback on failure

**Reporting:** Provide progress updates:
- "Completed Task X: [description]"
- "Tests passing: ✅"
- "Moving to Task Y"
