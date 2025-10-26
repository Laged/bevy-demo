# Combat System

## Shooting Model
- Holding the left mouse button fires volleys every `0.1` seconds.
- Each volley spawns `10` bullets with light random spread to create a cone.
- Bullets travel in straight lines at `15` units per frame and expire after `0.5` seconds.

## Damage Rules
- Bullets deal `15` damage on hit; enemies have `100` HP, requiring 7 hits on average.
- Damage is applied instantly when bullets overlap an enemy within a 50-unit radius (KD-tree lookup).
- Player contact with any enemy reduces health by `1` per collision tick; enemies do not take contact damage.

## Hit Feedback
- No hit flashes or sounds; feedback comes from enemies disappearing when HP ≤ 0.
- Player death is silent and instant; the sprite despawns and state resets.

## Encounter Pace
- Ever-growing enemy counts put emphasis on strafing and aiming efficiency.
- Lack of bullet cooldown beyond the 0.1-second volley window encourages constant firing.
- No environmental hazards, power-ups, or boss phases in the current build.
