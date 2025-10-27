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

## Testing

This project includes headless testing infrastructure that validates game logic without GPU rendering. This is critical for:
- **LLM monitoring**: Automated agents can verify changes don't break gameplay
- **Performance benchmarking**: Establish and maintain performance baselines
- **CI/CD integration**: Fast, deterministic tests that run in any environment

### Running Tests (For Humans)

```bash
# Run all tests
cargo test

# Run specific test with detailed output
cargo test test_headless_app_creation -- --nocapture

# Expected output shows:
#   - State machine transitions
#   - Entity spawning and persistence
#   - Enemy AI behavior (movement toward player)
#   - Performance metrics (17,000+ fps on headless simulation)
#   - Component state inspection
```

### Performance Baselines (Gold Standards)

**These performance criteria must NEVER regress:**

| Metric | Baseline | Threshold | Description |
|--------|----------|-----------|-------------|
| 1000-frame simulation | ~19,000 fps | ≥15,000 fps | Full game logic execution speed |
| Average frame time | ~52 µs | ≤67 µs | Per-frame processing time |
| Enemy movement rate | 1.0 units/frame | Exact | AI behavior consistency |
| Entity persistence | 100% | 100% | No entities lost during simulation |

**Breaking these thresholds indicates a critical regression.**

### For LLM Tools: Using Tests to Monitor Changes

When modifying game logic, **always run the headless test** to verify:

```bash
cargo test test_headless_app_creation -- --nocapture
```

**What to check in the output:**

1. **Performance Metrics**:
   ```
   1000 frames completed in ~50ms (≥15,000 fps required)
   Average frame time: ≤67µs
   ```
   If fps drops below 15,000 or frame time exceeds 67µs → **performance regression detected**

2. **Behavior Validation**:
   ```
   Enemy moved 10.00 units closer to player
   Player position unchanged: true
   ```
   If enemy doesn't move exactly 10 units → **AI logic broken**
   If player moves without input → **headless mode broken**

3. **System Health Checklist**:
   ```
   ✓ State machine transitions work
   ✓ Entities spawn and persist
   ✓ Enemy AI movement toward player verified
   ✓ Player stationary without input
   ✓ Health systems operational
   ```
   Any ✗ indicates a broken subsystem → **investigate immediately**

**Example: Detecting a Performance Regression**
```diff
# Before your change:
1000 frames completed in 51.5ms (19,400 fps) ✓

# After your change:
1000 frames completed in 150ms (6,666 fps) ✗
```
**Action**: Your change introduced a 3x slowdown. Revert or optimize.

**Example: Detecting Broken Logic**
```diff
# Before your change:
Enemy moved 10.00 units closer to player ✓

# After your change:
Enemy moved 0.00 units closer to player ✗
```
**Action**: Enemy movement system is broken. Check `src/enemy.rs:update_enemy_transform`

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
