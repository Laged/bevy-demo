# Content & Roadmap (Tech)

## Content Inventory
- 1 player sprite animation (4 frames).
- 3 enemy sprite variants sharing animation timing.
- 2 decorative prop sprites.
- 1 weapon sprite; bullets reuse the same atlas slot (index 16).

## Build Process
- No CI scripts; manual workflow is `cargo fmt`, `cargo clippy`, `cargo run`.
- Assets are hot-reloaded via Bevy defaults during development.
- Release builds use `cargo build --release`; no additional packaging.

## Expansion Mechanics
- Add feature flags or plugin groups to isolate experimental content (e.g., `EnemyPlugin::new_wave()`).
- Move content definitions (enemy stats, spawn waves) into data files for iteration without recompiling.
- Consider snapshotting atlas usage to prevent index drift as art expands.
