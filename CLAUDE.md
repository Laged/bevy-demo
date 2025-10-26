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

## Notes

- This codebase prioritizes clarity and iterability for agent learning over production optimization
- The `main.rs` initializes all plugins in order; plugin order can matter for system scheduling
- Window is resizable; some UI elements may need responsive adjustments
- Bevy version: 0.13.1 (check `Cargo.toml` for current dependencies)
- The project uses `bevy::utils` instead of `std::time` for WASM compatibility (see commit 10cce3d)
