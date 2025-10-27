# Enemy Forces (Tech)

## Modules
- `EnemyPlugin` covers spawn tables, AI behaviours, and encounter scripting.
- Behaviour trees or state machines defined per archetype (skirmisher, artillery, disruptor, support).
- Integration with animation and audio cues for spectacle moments.

## Data
- Enemy roster definitions (`assets/enemies/*.ron`) with stat blocks and abilities.
- Encounter composition templates referencing roster IDs.
- Broadcast personality data (taunts, camera actions) tied to AI events.

## Implementation Targets
- Deterministic AI decision-making to align with combat sim expectations.
- Hooks for difficulty scaling (modifier stacks, elite mutations).
- Debug visualizers for enemy intent and threat zones.
