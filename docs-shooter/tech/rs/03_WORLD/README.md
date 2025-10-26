# World & Environments (Tech)

## Data Sources
- Decorations use sprite indices 24–25 from `assets.png`, selected randomly per prop.
- No serialized map data; arena generation is procedural each run.

## Systems
- `WorldPlugin::spawn_world_decorations` spawns `NUM_WORLD_DECORATIONS = 500` props with random XY coordinates inside `WORLD_W/WORLD_H` bounds.
- Props receive `GameEntity` so `despawn_all_game_entities` can clean them up when leaving `InGame`.
- No collision components are attached; they are visual only.

## Camera
- `FollowCameraPlugin::camera_follow_player` lerps camera translation toward the player (`0.1` factor), keeping Z at 0.
- Camera entity gets a `PanCam` component with empty grab buttons, disabling manual panning but enabling smooth follow logic.

## Improvements
- Consider storing prop positions in a resource for deterministic seeding and replay.
- Add spatial partitioning for decorations if physics interactions are added later.
