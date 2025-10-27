# Combat System (Tech)

## Core Modules
- Combat resolution engine (tick scheduler, initiative, damage application).
- `collision.rs` for spatial checks; ensure it plays nicely with hex positioning.
- Shared math utilities for DPS/EHP/Power Level calculations.

## Data & Resources
- Stat definitions and scaling curves defined in `configs.rs`.
- Status effect registry (duration, stacking rules, visuals).
- Combat log resource for UI replay, analytics, and debugging.

## Testing & Validation
- Deterministic simulations with known seeds to validate balance changes.
- Unit tests covering formulas, resistances, and edge cases.
- Benchmarks for large-scale encounters to maintain frame pacing.
