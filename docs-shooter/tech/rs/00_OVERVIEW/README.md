# Overview (Tech)

## App Wiring
- `src/main.rs` initializes `GameState` and registers plugins in this order:
  1. `FollowCameraPlugin`
  2. `GuiPlugin`
  3. `GunPlugin`
  4. `PlayerPlugin`
  5. `AnimationPlugin`
  6. `ResourcesPlugin`
  7. `WorldPlugin`
  8. `EnemyPlugin`
  9. `CollisionPlugin`
- Default plugins are configured with nearest-neighbour sampling and a resizable window (`1200x900`).
- MSAA is disabled to match pixel art.

## State Machine
- `Loading` → asset load and camera spawn.
- `MainMenu` → waits for button press.
- `GameInit` → spawns player, gun, and world props, then transitions to `InGame`.
- `InGame` → runs combat systems; death returns to `MainMenu`.

## Assets
- Single atlas (`assets.png`) with an 8×8 grid of 16×16 sprites; scaled ×3 for on-screen rendering.
- No audio or additional asset types are loaded.

## Tooling & Libraries
- External crates: `rand` (spawn offsets), `kd-tree` (collision queries), `bevy_pancam` (camera follow helper).
- `cargo run` hot-reloads assets; `cargo fmt`, `cargo clippy`, and `cargo test` operate normally.
