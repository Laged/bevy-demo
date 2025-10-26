# Lore (Tech)

## Current Implementation
- No narrative assets are loaded; `ResourcesPlugin::load_assets` only handles the sprite atlas.
- There are no dialogue components, codex resources, or VO placeholders.

## Extension Hooks
- Add structured lore files under `assets/narrative/` and load them alongside the atlas.
- Emit narrative beats via Bevy events when transitioning between `GameState` values.
- Bind lore text to UI nodes in `GuiPlugin` (main menu, HUD, death screen).

## TODOs
- Define a resource for run metadata (mission name, run timer) to expose story context later.
- Instrument state transitions with logging for easier future cutscene triggers.
