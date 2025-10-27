# Lore (Tech)

## Data Sources
- Narrative text, VO scripts, and broadcast overlays live under `assets/narrative/` (to be created as needed).
- Localized strings should route through a resource or asset loader rather than hard-coded literals.
- Codex entries map to Bevy assets that can be hot-reloaded for iteration.

## Systems & Components
- Lore surfacing via UI: HUD banners, codex panels, encounter intros.
- Event system to trigger narrative beats (e.g., `GameEvent::EncounterIntro`).
- Resource for session metadata (current faction rivalry, sponsorship modifiers).

## Implementation Notes
- Keep story data in structured formats (RON/JSON/TOML) for easy parsing.
- Provide schema docs or Rust types for narrative assets.
- Log TODOs for missing narrative hooks referenced in `docs/game/01_LORE`.
