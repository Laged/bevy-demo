# Core Gameplay

## Loop Breakdown
- **Preparation**: None. Press `Play` and drop directly into the arena.
- **Action**: Survive waves of melee enemies that beeline toward the player.
- **Resolution**: Run ends instantly when health drops to 0, returning to the main menu.

## Objectives
- Stay alive as long as possible; there is no scoring yet, so survival time and enemy count act as soft goals.
- Maintain situational awareness by watching the debug HUD for health and swarm size.

## Controls
- `W/A/S/D` or arrow keys: Move the drone on the XY plane.
- Mouse movement: Aim the gun (the weapon rotates to face the cursor).
- Left mouse button: Continuous fire with light spread.

## Difficulty Ramp
- Enemies spawn around the player every second, up to 500 new units per tick, with a maximum population of 20,000.
- Enemy speed (1.0 units/frame) closely matches player speed (2.0 units/frame), forcing constant motion.
- Collisions inflict steady chip damage (1 HP per tick while touching).

## Failure Conditions
- Health starts at 100 and only decreases through enemy contact; there are no heals or shields.
- Falling to 0 HP triggers a state reset to the main menu; run data is not persisted.
