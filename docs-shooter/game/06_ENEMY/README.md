# Enemy Forces

## Roster
- Three visual variants pulled from the sprite atlas:
  - **Green Slime** (base index 8)
  - **Red Slime** (base index 12)
  - **Skin-Toned Slime** (base index 20)
- All variants share identical stats: 100 HP, melee-only attacks.

## Spawn Logic
- Every second, up to 500 enemies spawn around the player, limited by a global cap of 20,000.
- Spawn radius ranges from 1,000 to 5,000 units, ensuring enemies emerge off-screen before closing in.
- Enemy population resets whenever the run ends.

## Behaviour
- Each enemy continuously moves toward the player at `ENEMY_SPEED = 1.0`.
- Rotation is not rendered; sprites flip horizontally to face the player.
- No pathfinding or obstacle avoidance—movement is straight-line pursuit.

## Threat Profile
- Contact damage: 1 HP per collision tick.
- No ranged attacks, special abilities, or phase changes.
- Time-to-kill is determined solely by how long the player maintains distance and fires effectively.
