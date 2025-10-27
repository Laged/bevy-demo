# Player Forces (Tech)

## Modules
- `PlayerPlugin` orchestrates commander input, drone squad composition, and loadout management.
- `GunPlugin` (or future combat modules) handle weapon firing patterns and cooldowns.
- Progression data sourced from `configs.rs` and player profile resources.

## Components & Resources
- ECS components for drones (stats, loadout slots, AI behaviour toggles).
- Player commander resource storing tech tree selections, sponsorship perks.
- Save/load hooks for persisting builds between sessions.

## Implementation Focus
- Build formation editing tools surfaced through UI events.
- Validate stat modifications flow into combat calculations deterministically.
- Unit tests around loadout application, respec rules, and fail-safes.
