# Core Gameplay (Tech)

## State Coordination
- `GameState` (in `state.rs`) defines four variants: `Loading`, `MainMenu`, `GameInit`, `InGame`.
- `ResourcesPlugin::load_assets` advances `Loading → MainMenu`.
- `GuiPlugin::handle_main_menu_buttons` moves `MainMenu → GameInit` on button press.
- `WorldPlugin::init_world` handles `GameInit → InGame` once entities are spawned.
- `PlayerPlugin::handle_player_death` sets `InGame → MainMenu` when HP ≤ 0.

## Systems Overview
- `WorldPlugin` seeds the run: player, gun, and decorations tagged with `GameEntity` for cleanup.
- `FollowCameraPlugin` spawns the camera on `Loading` and lerps toward the player each frame.
- `ResourcesPlugin::update_cursor_position` tracks world-space cursor coordinates for aiming.

## Data Flow
- Cursor position resource feeds both gun rotation and sprite flipping.
- `GlobalTextureAtlas` holds handles to the shared image/layout used by every sprite.
- `GameEntity` marker ensures everything spawned for a run despawns on exit.

## Known Issues
- No guard against starting a new run while assets fail to load; missing handles would panic.
- Collision radius (50 units) is hardcoded in collision systems; consider moving to `src/configs.rs` for tuning.
