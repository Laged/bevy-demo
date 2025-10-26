# World & Environments

## Arena Layout
- Infinite-feeling open plane; technically a large coordinate space with no walls.
- The camera follows the player, so only a slice of the world is visible at any time.
- There are no collision boundaries—players and enemies can kite indefinitely.

## Set Dressing
- 500 environmental props spawn at run start:
  - Randomly scattered between `(-3000..3000, -2500..2500)`.
  - Drawn from two sprite indices (radar dishes and crates), providing light variety.
- Props are purely cosmetic and do not block movement or projectiles.

## Spatial Rules
- Enemies spawn 1,000–5,000 units away from the player’s current position, ensuring they enter from off-screen.
- No dynamic events (weather, hazards, fog) are implemented.

## Visual Language
- Muted background color (`RGB(197,204,184)`) reinforces a dusty battlefield.
- Parallax and depth layering are minimal; sprites sit on a single plane with simple z-ordering.
