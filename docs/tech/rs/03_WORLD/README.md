# World & Environments (Tech)

## Assets & Data
- Hex tile definitions and modifiers stored in `assets/world/tiles.ron`.
- Scene templates for arena sectors (mesh references, spawn points).
- Environmental effect definitions (hazard timelines, broadcast props).

## Systems
- `WorldPlugin` responsibilities: map loading, tile occupancy, dynamic events.
- Streaming/spawning logic for sectors as the player travels.
- Integration with collision and pathfinding subsystems.

## Tooling
- Editor or script support for authoring hex layouts.
- Visualization commands (`cargo run --features world_debug`) for debugging.
- Automated tests validating tile adjacency and encounter placement rules.
