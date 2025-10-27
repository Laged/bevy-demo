# Core Gameplay (Tech)

## Systems Inventory
- Overworld navigation plugin (hex grid pathing, encounter triggers).
- Game state transitions (`GameState`, sub-states for overworld, encounter, results).
- Encounter scheduling and difficulty scaling resources.

## Data & Resources
- Hex map definitions (`assets/maps/*.ron`) and runtime cache components.
- Encounter templates with roster configuration and reward tables.
- Telemetry resource to persist pre/post battle summaries.

## Implementation Targets
- Ensure `WorldPlugin` exposes navigation APIs used by player and AI systems.
- Wire deterministic sequencing for encounter resolution (tick rate, ordering).
- Integrate analytics hooks to feed post-match summaries in UI.
