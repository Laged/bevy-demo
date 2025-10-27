# UI Domain

**Maintainer:** UI Agent

## Responsibilities
- HUD rendering (health, FPS, debug info)
- Main menu
- Camera controls
- Future: Choice dialogs, pause menus

## Modules
- `hud.rs` - HUD and menu rendering (GuiPlugin)
- `camera.rs` - Camera follow system (FollowCameraPlugin)

## Dependencies
**Reads from:**
- `core::state::GameState`
- `entities::player::Health`
- `entities::enemy::Enemy`

**Writes to:**
- None (read-only)

## Contact Points
For inter-domain communication, use events in `core::events`.
