# Progression (Tech)

## Current State
- No resources or components track XP, levels, or upgrades.
- `configs.rs` hosts all tunable values (speed, health, spawn rates) as constants.

## Extension Points
- Introduce a `Progression` resource to hold run-time unlocks or persistent meta stats.
- Convert constants (e.g., `PLAYER_SPEED`, `BULLET_DAMAGE`) into data assets to support scaling.
- Add save/load routines to `ResourcesPlugin` for persisting progression between sessions.

## Testing Ideas
- Unit tests ensuring upgrade modifiers combine correctly with base constants.
- Integration tests simulating run resets to verify progression persistence.
