# Bevy Shooter Autocode Lab

This repo is a training ground for autonomous coding agents working on a minimalist Bevy survival shooter. The project emphasizes documentation-driven development: every gameplay system is mirrored by design docs and matching technical notes so contributors can reason about intent before writing code.

![screenshot](/screenshot.png)

## Current Gameplay Snapshot
- One-click main menu drops you into an open arena.
- Pilot a single drone with `W/A/S/D` or arrow keys; aim with the mouse.
- Hold the left mouse button to fire rapid volleys of ten bullets with mild spread.
- Enemies spawn every second around the player (up to 20,000 total) and rush straight toward you.
- Collisions chip away at your 100 HP; when health hits zero the run ends and you return to the menu.

## Quick Start

### With Nix (Recommended)

```bash
# Run directly from GitHub (no clone needed!)
nix run github:Laged/bevy-demo/main
```

### Without Nix

```bash
# Prerequisites: Rust 1.70+, see full installation guide below
cargo run
```

---

## Documentation Map
- `docs/game/` – technology-agnostic design bible (lore, gameplay pillars, world, UI, combat, progression).
- `docs/tech/rs/` – Bevy/Rust implementation notes that mirror the game docs chapter-for-chapter.
- `docs-shooter/game/` – detailed documentation of the shooter as it exists today.
- `docs-shooter/tech/rs/` – system-level breakdown of the live codebase (plugins, components, assets, known issues).

Start with `docs-shooter/game/00_INDEX.md` to understand the shipped experience, then dive into the corresponding tech documents before touching code.

## Full Installation Guide

### Quick Start with Nix (Recommended)

If you have [Nix installed](https://docs.determinate.systems/determinate-nix/):

```bash
# Run directly from GitHub (no clone needed)
nix run github:Laged/bevy-demo/main

# Or clone and run locally
git clone https://github.com/Laged/bevy-demo.git
cd bevy-demo
nix run

# Enter development shell with all tools
nix develop
cargo run
```

### Installation without Nix

**Required:**
- Rust 1.70+ (check: `rustc --version`)
- Cargo (typically bundled with Rust)
- System dependencies for Bevy/graphics:
  - **Linux:** `alsa-lib`, `udev`, `libxkbcommon`, `wayland`, `vulkan-loader`, `pkg-config`
    ```bash
    # Ubuntu/Debian
    sudo apt install libasound2-dev libudev-dev libxkbcommon-dev wayland-protocols libwayland-dev vulkan-tools libvulkan-dev pkg-config

    # Fedora
    sudo dnf install alsa-lib-devel libudev-devel libxkbcommon-devel wayland-devel vulkan-loader-devel pkg-config

    # macOS (with Homebrew)
    brew install pkg-config
    ```
  - **macOS:** Xcode Command Line Tools (`xcode-select --install`)
  - **Windows:** Install Visual Studio Build Tools or MinGW

### Running the Game

```bash
# Build and run with cargo
cargo run

# Run with optimizations (recommended for performance testing)
cargo run --release

# Run tests
cargo test

# Code quality checks
cargo fmt --all
cargo clippy --all-targets --all-features
```

### Development Workflow

```bash
# Enter Nix dev shell (all dependencies pre-configured)
nix develop

# Watch for changes and recompile
cargo watch -x "run"

# Format before committing
cargo fmt --all

# Run linter and address warnings
cargo clippy --all-targets --all-features
```

## Contribution Workflow
1. Read the relevant chapters in `docs-shooter` (for today’s behaviour) and `docs/` + `docs/tech/rs` (for future-facing structure).
2. Update design docs if you change intent; update tech docs if you change implementation.
3. Implement or refactor the Bevy systems under `src/`.
4. Run tests and the game locally, then capture outcomes back in the docs for the next agent.

## Controls & Credits
- Movement: `W/A/S/D` or arrow keys  
- Fire: Hold left mouse button  
- Camera: Automatically follows the player (PanCam assist)

Assets: [0x72 Dungeon Tileset II](https://0x72.itch.io/dungeontileset-ii)  
Font: [Monogram](https://datagoblin.itch.io/monogram)
