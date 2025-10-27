# Architecture: Domain-Driven Design for Parallel Agent Development

**Status**: ✅ Active (Implemented 2025-10-27)
**Last Updated**: 2025-10-27
**Maintainer**: All Agents (coordination required)

## Overview

This codebase uses a **domain-driven architecture** that enables 4 autonomous agents to work in parallel with minimal merge conflicts. Each agent owns a specific domain and is responsible for maintaining code quality within their territory.

## Critical Rules for ALL Agents

### 🚨 MANDATORY: Respect Domain Boundaries

1. **NEVER directly edit another agent's domain without coordination**
2. **ALWAYS use events in `core/events.rs` for cross-domain communication**
3. **ALWAYS read the domain README before making changes**
4. **ALWAYS verify tests pass after changes**
5. **ALWAYS maintain the gold standard performance baseline (≥15k fps)**

### 🚨 Breaking These Rules = Merge Conflicts + Broken Build

## Directory Structure

```
src/
├── core/                    # GAMEPLAY AGENT OWNS (shared read-only for others)
│   ├── events.rs            # Cross-domain events (Trigger/Observer pattern)
│   ├── choice_system.rs     # Extensible choice/pause system
│   ├── state.rs             # Game state machine (Loading, MainMenu, InGame, PlayerChoice)
│   ├── collision.rs         # KD-tree spatial indexing
│   └── resources.rs         # Asset loading, GlobalTextureAtlas, CursorPosition
│
├── entities/                # GAMEPLAY AGENT OWNS (shared read-only for others)
│   ├── player.rs            # Player component, Health, PlayerState, movement
│   ├── enemy.rs             # Enemy spawning, AI, types (Green/Red/Skin)
│   └── world.rs             # World decoration, GameEntity marker
│
├── domains/
│   ├── ui/                  # UI AGENT OWNS
│   │   ├── hud.rs           # GuiPlugin (HUD, menus, debug overlay, FPS)
│   │   ├── camera.rs        # FollowCameraPlugin (camera follow + pan controls)
│   │   └── README.md        # Domain documentation
│   │
│   ├── gameplay/            # GAMEPLAY AGENT OWNS
│   │   ├── config/          # Game configuration
│   │   │   ├── constants.rs # Game balance constants (WW, WH, speeds, etc.)
│   │   │   ├── loader.rs    # GameConfig TOML loader
│   │   │   └── mod.rs       # Config re-exports
│   │   ├── combat.rs        # GunPlugin (bullets, shooting, Gun component)
│   │   └── README.md        # Domain documentation
│   │
│   ├── graphics/            # GRAPHICS AGENT OWNS
│   │   ├── particles.rs     # ParticleEffectsPlugin (trails, impacts, death)
│   │   ├── animation.rs     # AnimationPlugin (sprite animations, flipping)
│   │   └── README.md        # Domain documentation
│   │
│   └── testing/             # TESTING AGENT OWNS
│       ├── harness.rs       # create_headless_app() for tests
│       ├── helpers.rs       # spawn_test_player/enemy/bullet()
│       ├── simulation.rs    # run_frames(), set_state(), get_state()
│       ├── benchmarks.rs    # GameConfig::benchmark_mode()
│       └── README.md        # Domain documentation
│
├── main.rs                  # Entry point (uses all domains)
├── plugin_mode.rs           # PluginMode::Headless / Visual switching
└── lib.rs                   # Module exports + backward compatibility re-exports
```

## Agent Ownership Matrix

| Domain | Owner | Responsibilities | Dependencies (Read) | Dependencies (Write) |
|--------|-------|-----------------|-------------------|---------------------|
| **core/** | Gameplay Agent | State machine, events, collision, resources | N/A | GameState, events |
| **entities/** | Gameplay Agent | Player, Enemy, World entities | core/ | Entity spawning |
| **domains/ui/** | UI Agent | HUD, menus, camera | core/, entities/ | Camera transform |
| **domains/gameplay/** | Gameplay Agent | Balance, combat, config | core/, entities/ | Bullets, config |
| **domains/graphics/** | Graphics Agent | Particles, animations | core/, entities/ | Visual components |
| **domains/testing/** | Testing Agent | Test harness, benchmarks | All (for testing) | Test state only |

## Communication Protocol

### Cross-Domain Events (Bevy 0.17 Pattern)

All cross-domain communication **MUST** use the event system defined in `core/events.rs`:

```rust
// core/events.rs - Define your events here
#[derive(Event, Clone)]
pub struct TriggerChoiceEvent {
    pub context: ChoiceContext,
    pub metadata: ChoiceMetadata,
}

// Sender (any domain)
commands.trigger(TriggerChoiceEvent { ... });

// Receiver (any domain)
app.add_observer(handle_choice_event);
```

**Why Events?**
- Decouples domains (no direct function calls)
- Prevents circular dependencies
- Makes communication explicit and traceable
- Enables parallel development

### Shared State (Read-Only)

Domains can READ from `core/` and `entities/` but should NEVER write directly:

```rust
// ✅ OK: Read shared state
fn system(state: Res<State<GameState>>, players: Query<&Player>) { ... }

// ❌ BAD: Write to shared state directly
fn system(mut state: ResMut<NextState<GameState>>) { ... }  // Only Gameplay Agent!

// ✅ OK: Request state change via event
fn system(mut commands: Commands) {
    commands.trigger(RequestStateChangeEvent { ... });
}
```

## Adding New Features

### Step 1: Identify the Correct Domain

| Feature Type | Domain | Example |
|-------------|--------|---------|
| HUD elements, menus, camera | UI Agent | Health bar, pause menu |
| Game balance, spawning, AI | Gameplay Agent | Enemy spawn rate, bullet damage |
| Visual effects, animations | Graphics Agent | Explosion particles, sprite wobble |
| Test infrastructure | Testing Agent | New test helpers |

### Step 2: Read the Domain README

**BEFORE touching any code**, read `src/domains/{domain}/README.md` to understand:
- Current responsibilities
- Existing modules
- Dependencies
- Communication patterns

### Step 3: Follow Domain Conventions

Each domain has its own conventions. Examples:

**UI Domain:**
- All UI systems run in `InGame` state via `.run_if(in_state(GameState::InGame))`
- Camera follows player via `Transform` component updates
- HUD reads from `Health`, `Enemy` queries (never writes)

**Gameplay Domain:**
- Owns all state transitions via `NextState<GameState>`
- Manages entity spawning/despawning
- Coordinates combat logic

**Graphics Domain:**
- Purely visual (no gameplay logic)
- Uses particle asset handles from `ParticleEffectAssets`
- Sprite flipping based on player/cursor position

### Step 4: Add Cross-Domain Events if Needed

If your feature needs to communicate with another domain:

1. Define the event in `core/events.rs`
2. Document the event purpose and payload
3. Add sender in your domain
4. Coordinate with other agent to add receiver

## File Organization Rules

### Where to Put New Files

| File Type | Location | Example |
|----------|----------|---------|
| New UI component | `domains/ui/` | `choice_dialog.rs` |
| New gameplay system | `domains/gameplay/` | `wave_system.rs` |
| New visual effect | `domains/graphics/` | `screen_shake.rs` |
| New entity type | `entities/` | `projectile.rs` |
| New shared resource | `core/` | Discuss with team first! |
| New test utility | `domains/testing/` | `perf_profiler.rs` |

### Module Export Rules

When adding a new module:

1. Add `pub mod your_module;` to the domain's `mod.rs`
2. Re-export important types if needed: `pub use your_module::YourPlugin;`
3. Update domain README with module description
4. Update this ARCHITECTURE.md if it's a major change

## Backward Compatibility

The `lib.rs` maintains backward compatibility with old flat imports:

```rust
// Old code still works:
use hell_game::animation::AnimationPlugin;  // ✅ Still works
use hell_game::player::PlayerPlugin;        // ✅ Still works

// New code should use domain structure:
use hell_game::domains::graphics::animation::AnimationPlugin;  // ✅ Preferred
use hell_game::entities::player::PlayerPlugin;                 // ✅ Preferred
```

**Never remove backward compat re-exports without team discussion!**

## Testing Requirements

### Gold Standard Performance Baseline

**ALL changes MUST maintain these baselines:**

```bash
cargo test test_headless_app_creation -- --nocapture
```

**Required Metrics:**
- ✅ 1000-frame simulation: **≥15,000 fps**
- ✅ Average frame time: **≤67 µs**
- ✅ Enemy movement: **10.0 units in 10 frames** (exactly)
- ✅ All 6 tests passing

**If ANY metric fails after your changes:**
1. ❌ Do NOT merge
2. ❌ Do NOT ask for manual testing
3. ✅ Investigate and fix the regression
4. ✅ Report findings before hand-off

### Domain-Specific Tests

Each domain should maintain its own tests:

```bash
# UI Domain
cargo test domains::ui::

# Gameplay Domain
cargo test domains::gameplay::

# Graphics Domain
cargo test domains::graphics::

# Testing Domain
cargo test domains::testing::
```

## Common Pitfalls

### ❌ Anti-Pattern: Direct Domain Coupling

```rust
// ❌ BAD: UI Agent directly calling Gameplay Agent code
use crate::domains::gameplay::combat::spawn_bullet;
spawn_bullet(&mut commands, position);  // Creates tight coupling!

// ✅ GOOD: Use events for cross-domain communication
commands.trigger(SpawnBulletEvent { position });
```

### ❌ Anti-Pattern: Circular Dependencies

```rust
// ❌ BAD: domains/ui depends on domains/gameplay
use crate::domains::gameplay::SomeType;

// ✅ GOOD: Both depend on shared core/entities
use crate::core::SomeType;
use crate::entities::SomeEntity;
```

### ❌ Anti-Pattern: Modifying Shared State

```rust
// ❌ BAD: UI Agent changing game state
fn ui_system(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::MainMenu);  // Only Gameplay Agent should do this!
}

// ✅ GOOD: UI Agent requests state change via event
fn ui_system(mut commands: Commands) {
    commands.trigger(RequestStateChangeEvent { target: GameState::MainMenu });
}
```

## Migration from Old Structure

The old flat structure is still accessible via backward compatibility:

```rust
// Old paths (still work, but deprecated)
src/animation.rs       → lib.rs re-exports from domains/graphics/animation.rs
src/camera.rs          → lib.rs re-exports from domains/ui/camera.rs
src/collision.rs       → lib.rs re-exports from core/collision.rs
src/configs.rs         → lib.rs re-exports from domains/gameplay/config/constants.rs
src/config_loader.rs   → lib.rs re-exports from domains/gameplay/config/loader.rs
src/enemy.rs           → lib.rs re-exports from entities/enemy.rs
src/gui.rs             → lib.rs re-exports from domains/ui/hud.rs
src/gun.rs             → lib.rs re-exports from domains/gameplay/combat.rs
src/particle_effects.rs→ lib.rs re-exports from domains/graphics/particles.rs
src/player.rs          → lib.rs re-exports from entities/player.rs
src/resources.rs       → lib.rs re-exports from core/resources.rs
src/state.rs           → lib.rs re-exports from core/state.rs
src/world.rs           → lib.rs re-exports from entities/world.rs
```

**For New Code:** Use the new domain paths directly (see main.rs for examples).

## Questions?

- Read the domain README: `src/domains/{domain}/README.md`
- Check this document: `docs/ARCHITECTURE.md`
- Review the implementation plan: `docs/plans/2025-10-27-parallel-agent-architecture-implementation.md`
- Check CLAUDE.md for agent-specific guidance

## Version History

- **2025-10-27**: Initial domain-driven architecture implemented
  - 15 files migrated to domains
  - All tests passing (16204 fps baseline)
  - Zero breaking changes
  - Full backward compatibility maintained
