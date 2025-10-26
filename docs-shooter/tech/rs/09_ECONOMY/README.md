# Economy (Tech)

## Current State
- No economy systems exist; there are no resources, components, or data files for currencies.
- Enemy deaths do not emit events, so there is no hook for loot drops.

## Extension Points
- Define a `Currency` resource and award amounts via combat events.
- Track loot tables in external data (RON/JSON) and load them through `AssetServer`.
- Add shop or upgrade UI by extending `GuiPlugin` with additional nodes and state transitions.

## Risks
- Introducing pickups will require collision layers beyond the current kd-tree approach.
- Ensure new economy systems respect run resets to avoid stale state between sessions.
