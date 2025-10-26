# Player Forces (Tech)

## Entities & Components
- Player entity spawned in `WorldPlugin::init_world` with:
  - `Sprite` (atlas index 0, scale ×3)
  - `Transform` (origin)
  - `Player`, `Health(100.0)`, `PlayerState::Idle`
  - `AnimationTimer` (0.15s repeating)
- Gun entity spawned separately with `Gun`, `GunTimer(Stopwatch)`, and shared atlas sprite (index 17).
  - Note: Gun sprite does not animate; bullets spawned by `GunPlugin` use index 16.

## Systems
- `handle_player_input`
  - Reads `ButtonInput<KeyCode>` for WASD/arrow keys.
  - Builds a directional vector, normalizes, and translates the player by `PLAYER_SPEED` each frame.
  - Updates `PlayerState` for animation selection.
- `handle_player_death`
  - Checks for player absence or HP ≤ 0; schedules transition to `MainMenu`.

## Animation & Orientation
- `AnimationPlugin::animate_player` cycles sprite index every tick based on `PlayerState`.
- `flip_player_sprite_x` flips `Sprite::flip_x` according to cursor X position.
- Gun rotation handled in `GunPlugin::update_gun_transform`, aligning weapon with aim.

## Known Issues
- No acceleration or friction; movement is instant on/off, which may feel abrupt.
- Health is shared via `Query<&mut Health, With<Player>>`; consider moving to resource if multiple player entities are added later.
- Collision damage is applied every frame when enemies are within radius; no invulnerability window or knockback.
