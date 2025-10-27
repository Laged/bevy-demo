# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Star Streams** is a Y2K-futurist auto-battler game built on Bevy (Rust game engine). This codebase serves as both a game implementation and a training ground for LLM agents to learn full-stack game development by translating design documents into Bevy systems.

The project is documentation-driven: design decisions live in `docs/` (see `docs/00_INDEX.md` for the full index) and implementation is in `src/` using Bevy plugins.

## Essential Commands

```bash
# Build and run the game
cargo run

# Run with optimizations for development (faster than release, still debuggable)
cargo run

# Build for release (slower to compile but runs fast)
cargo build --release

# Run tests (currently minimal coverage)
cargo test

# Run a specific test
cargo test <test_name> -- --nocapture

# Check code without compiling
cargo check

# Lint and format code
cargo clippy
cargo fmt

# Development environment (NixOS)
nix flake update
nix develop
```

## Architecture Overview

### Plugin-Based System
The game uses Bevy's plugin architecture where each major subsystem is a plugin:

- **PlayerPlugin** (`src/player.rs`) – Player movement, health, collision events
- **EnemyPlugin** (`src/enemy.rs`) – Enemy spawning, health, movement toward player
- **GunPlugin** (`src/gun.rs`) – Bullet spawning and firing logic
- **CollisionPlugin** (`src/collision.rs`) – KD-tree based collision detection using `kd-tree` crate
- **AnimationPlugin** (`src/animation.rs`) – Sprite sheet animations with timer-based frames
- **FollowCameraPlugin** (`src/camera.rs`) – Camera that follows the player
- **WorldPlugin** (`src/world.rs`) – World decoration generation and management
- **GuiPlugin** (`src/gui.rs`) – HUD rendering (health bars, debug info)
- **ResourcesPlugin** (`src/resources.rs`) – Asset loading (texture atlases, cursor position tracking)

Each plugin registers its systems in the `build()` method, typically filtering by `GameState` using `run_if(in_state(...))`.

### Game State Machine
Defined in `src/state.rs`:
- **Loading** – Initial asset loading phase
- **MainMenu** – Menu state (currently minimal)
- **GameInit** – Setup before gameplay starts
- **InGame** – Active gameplay

Systems typically run only in the `InGame` state to avoid running unnecessary updates.

### Key Data Structures

**Components** (entity-local data):
- `Player` – Marks the player entity
- `Health(f32)` – Health value; used by Player and Enemy
- `Enemy { health: f32 }` – Marks enemy entities with health
- `EnemyType` – Enum (Green, Red, Skin) for different enemy sprites
- `PlayerState` – Enum (Idle, Run) for animation state
- `AnimationTimer` – Timer for sprite frame cycling

**Resources** (global/shared data):
- `GlobalTextureAtlas` – Cached texture atlas handle and layout (loaded in `src/resources.rs`)
- `CursorPosition(Option<Vec2>)` – Current mouse position in world coordinates

**Events**:
- `PlayerEnemyCollisionEvent` – Fired when enemy touches player (see `src/player.rs:38`)

### Configuration & Tweaking
All game constants are in `src/configs.rs`:
- Window dimensions: `WW` (1200), `WH` (900)
- Sprite settings: `SPRITE_SCALE_FACTOR` (3.0), `TILE_W`/`TILE_H` (16 pixels)
- Player: `PLAYER_SPEED`, `PLAYER_HEALTH`
- Enemies: `MAX_NUM_ENEMIES`, `ENEMY_DAMAGE`, `ENEMY_SPEED`, `SPAWN_RATE_PER_SECOND`
- Bullets: `BULLET_SPEED`, `BULLET_DAMAGE`, `BULLET_TIME_SECS`, `NUM_BULLETS_PER_SHOT`
- KD-tree refresh rate for collision optimization
- Background color and other visual tweaks

Modify these constants to balance gameplay or prototype changes without rewriting systems.

### Collision Detection
Uses the `kd-tree` crate for spatial indexing. Collision checks happen in `src/collision.rs` and are refreshed at `KD_TREE_REFRESH_RATE` (0.1 seconds by default) to avoid per-frame overhead.

### Sprite Sheet & Rendering
- Single sprite sheet (`assets.png`) loaded as `GlobalTextureAtlas`
- 8x8 grid of 16x16 pixel tiles, scaled 3x in-game (48x48 pixels on screen)
- TextureAtlasLayout created in `src/resources.rs:37` during Loading state
- Each entity references the layout and an index into the grid for its sprite

### Asset Loading
Assets are loaded in `src/resources.rs::load_assets()` during the Loading state and stored in the `GlobalTextureAtlas` resource. The game transitions to MainMenu once assets are loaded.

## Design Documents
The `docs/` directory is the single source of truth for game design. Key sections:
- `docs/00_GAME_DESIGN/` – Core game concept and visual design
- `docs/02_COMBAT_SYSTEM/` – Combat formulas (DPS, EHP, Power Level)
- `docs/04_GAMEPLAY_SYSTEMS/` – Hex grid, AI behavior, auto-battler rules
- `docs/05_ENCOUNTER_DESIGN/` – Enemy archetypes, wave composition, difficulty scaling
- `docs/06_GAME_MVP/06_03_IMPLEMENTATION_ROADMAP.md` – Week-by-week showcase roadmap
- `docs/09_UNIT_DESIGN/` – Player drone and Kitsune enemy specifications

When implementing features, start by reading the relevant design doc, then translate its rules into a Bevy system or component.

## Development Workflow for Agents

1. **Study the design doc** – Read the relevant section in `docs/` to understand requirements
2. **Understand the plugin** – Find or create the matching plugin in `src/`
3. **Implement or extend** – Add/modify systems, components, or resources
4. **Test locally** – Run `cargo run` and verify behavior visually, or use `cargo test` for unit tests
5. **Document gaps** – Add comments or notes to the code if ambiguities remain for the next agent

## Nix Development Environment
A `flake.nix` is provided for reproducible development. It includes:
- Rust toolchain (stable, via fenix)
- Build tools: gcc, pkg-config, cargo-watch, cargo-deny, cargo-edit
- Graphics libraries for Bevy: alsa-lib, udev, libxkbcommon, wayland, vulkan-loader
- rust-analyzer for IDE support

Enter the environment with `nix develop` (or `nix flake update` to refresh lockfile first).

---

## MANDATORY: Local Bevy Skills First (Strong Enforcement)

**BEFORE ANY BEVY 0.17 RESEARCH OR IMPLEMENTATION:**

You MUST check local skills FIRST. No exceptions. This is not negotiable.

### Available Local Bevy Skills

1. **`latest-rs-bevy-features`** (Production-ready, TDD-validated)
   - Use for: Stability assessment, production readiness, deployment decisions, FPS impact
   - Time: < 2 minutes per decision
   - Trigger: Any question about Bevy 0.17+ features

2. **`crates-rs-bevy`** (Comprehensive reference)
   - Use for: Full API documentation, detailed method signatures
   - Time: Reference material for deep dives
   - Trigger: Specific API questions (e.g., "What methods does Camera have?")

### The Rule (STRONG ENFORCEMENT)

**If you're working on ANY of these, you MUST use local skills first:**
- ✅ Headless widgets, ViewportNode, TilemapChunk
- ✅ Event system (Trigger pattern, lifecycle events)
- ✅ Component propagation, UI gradients, Val helpers
- ✅ Breaking changes assessment (0.16 → 0.17)
- ✅ Third-party crate compatibility (bevy_ecs_tilemap, bevy_egui, etc.)
- ✅ Production readiness decisions
- ✅ FPS impact estimates
- ✅ WASM deployment constraints

### DO NOT do this:
- ❌ Search GitHub without checking skills first
- ❌ Ask Discord/Reddit before reading skill
- ❌ Assume feature stability without checking skill stability matrix
- ❌ Pick third-party crates without verifying skill's version table
- ❌ Estimate FPS impact without checking skill's performance reference

### Workflow (Non-Negotiable)

1. **Ask yourself:** "Is this about Bevy 0.17 features?"
   - YES → Go to step 2
   - NO → Proceed normally

2. **Check local skills:**
   ```bash
   # latest-rs-bevy-features: decisions in < 2 minutes
   # crates-rs-bevy: deep API reference
   ```
   - Found answer? → Use it immediately
   - Still unclear? → Go to step 3

3. **Only then, research network sources** (if needed)
   - Official release notes (bevy.org/news)
   - Bevy examples (github.com/bevyengine/bevy/examples)
   - Migration guide (bevyengine.org/learn/migration-guides)
   - API docs (docs.rs/bevy)

### Why This Matters

- **Speed:** 12-18x faster than scattered research
- **Confidence:** 95%+ on deployment decisions vs. 50% baseline
- **Correctness:** Skills are TDD-validated, not guesswork
- **Consistency:** Every agent uses same authoritative reference

---

## AUTOMATED TESTING BEFORE MANUAL VERIFICATION

**Before asking for manual testing, the LLM MUST verify:**

1. **Run README checks** - Validate against README.md testing requirements
2. **Run cargo test** - All tests pass, no regressions
3. **Run cargo check** - No compilation errors
4. **Run cargo build** - Builds successfully
5. **Report results** - Document findings before handing to human

### What Tests Verify (From README.md)

See README.md "For LLM Tools" section for detailed test verification checklist. Key metrics:

| Metric | Baseline | Threshold |
|--------|----------|-----------|
| 1000-frame simulation | ~19,000 fps | ≥15,000 fps |
| Average frame time | ~52 µs | ≤67 µs |
| Enemy movement | 1.0 units/frame | Exact |
| Entity persistence | 100% | 100% |

If ANY metric fails → LLM must investigate and report issues, not hand to human.

### Testing Workflow (LLM)

```bash
# 1. Run comprehensive tests
cargo test -- --nocapture

# 2. Run performance baseline
cargo test test_headless_app_creation -- --nocapture

# 3. Verify against README thresholds
# - Check: fps ≥ 15,000
# - Check: frame time ≤ 67µs
# - Check: enemy movement = 10.0 units
# - Check: no entities lost

# 4. Report findings
# - If all pass: "Testing verified, manual testing OK"
# - If any fail: "Regression detected at [metric], investigating..."
```

### Hand-off to Human

Only after ALL of the above pass can you ask for manual testing. Your report should include:
- ✅ All cargo test results (pass/fail)
- ✅ Performance metrics vs. baselines
- ✅ Any regressions detected
- ✅ Recommendation: "Ready for manual testing" or "Issues to resolve"

## Notes

- This codebase prioritizes clarity and iterability for agent learning over production optimization
- The `main.rs` initializes all plugins in order; plugin order can matter for system scheduling
- Window is resizable; some UI elements may need responsive adjustments
- Bevy version: 0.13.1 (check `Cargo.toml` for current dependencies)
- The project uses `bevy::utils` instead of `std::time` for WASM compatibility (see commit 10cce3d)
- Do both, always remove worktree and branch once merged to dev and tests ok