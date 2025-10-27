# UI & UX (Tech)

## Rendering Stack
- `GuiPlugin` owns Bevy UI nodes for HUD, combat overlays, and meta menus.
- Font and icon assets located under `assets/ui/`.
- Consider using bevy_egui or custom widgets where default Bevy UI falls short.

## Data Flow
- Resources feeding UI: player telemetry, combat metrics, economy snapshot.
- Event channels for notifications, broadcast pop-ups, and commentary cues.
- Localization pipeline and formatting helpers.

## Implementation Tasks
- Establish UI state machines (e.g., encounter summary -> loot selection).
- Build component styling guidelines (padding, colors, animation timing).
- Instrument UI for accessibility toggles (colorblind palettes, text size).
