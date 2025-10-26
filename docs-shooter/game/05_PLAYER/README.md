# Player Forces

## Core Kit
- **Avatar**: Single drone sprite (atlas index 0) animated across four frames.
- **Health**: 100 HP; no regeneration, shields, or consumables.
- **Mobility**: Flat movement speed of `PLAYER_SPEED = 2.0`. Movement is constrained to eight directions via WASD/arrow keys with instant acceleration.
- **Weapon**: Shoulder-mounted gun that tracks the cursor and fires 10 bullets per salvo.

## Controls Feel
- Movement is continuous while keys are held; releasing all inputs switches to an idle animation.
- Player sprite flips horizontally based on cursor position, preserving facing context.
- Gun floats beside the player with a 20-unit offset, rotating smoothly toward the cursor.

## Survivability
- Collision with any enemy deducts `1` HP per collision tick; health is otherwise static.
- Player death immediately despawns all game entities and returns to the main menu.

## Growth Hooks
- No upgrades, loadouts, or unlocks exist today.
- Debug HUD is the only source of stat awareness; there is no in-world indication of speed or fire rate buffs.
