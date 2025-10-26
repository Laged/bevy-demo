# UI & UX (Tech)

## Main Menu
- Built in `GuiPlugin::setup_main_menu`.
- Root `Node` fills the screen; child `Button` entity shows the `Play` label.
- `MainMenuItem` marker ensures all menu entities despawn on exit.
- Interaction system listens for `Interaction::Pressed` to queue state change.

## In-Game HUD
- `spawn_debug_text` creates a HUD root node tagged with `GameEntity`.
- Text uses `monogram.ttf` at 40px; content replaced each frame by `update_debug_text`.
- FPS values come from `DiagnosticsStore` via `FrameTimeDiagnosticsPlugin`.
- Enemy count uses `Query::iter().count()`, which scales poorly once the cap is increased.

## Cursor & Sprite Orientation
- `CursorPosition` resource updated every frame using `viewport_to_world`.
- `AnimationPlugin` flips player/gun sprites based on cursor X relative to entity translation.

## Known Issues
- Debug HUD assumes at least one enemy in the world; when zero, text never updates.
- No hover/pressed visual feedback on the menu button.
- HUD lacks run timer or other metrics; extend `Text` formatting as needed.
