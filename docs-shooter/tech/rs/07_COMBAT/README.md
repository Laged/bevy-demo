# Combat System (Tech)

## Shooting Pipeline
- `GunPlugin::handle_gun_input`
  - Uses `GunTimer` (`Stopwatch`) to gate volleys every `BULLET_SPAWN_INTERVAL = 0.1s`.
  - Spawns `NUM_BULLETS_PER_SHOT = 10` entities per volley with `Bullet` marker and `BulletDirection`.
  - Applies random spread (`-0.5..0.5`) to X/Y components of the gun's local X axis.
- `update_bullets` advances bullets along normalized direction × `BULLET_SPEED = 15.0`.
- `despawn_old_bullets` removes bullets after `BULLET_TIME_SECS = 0.5`.

## Collision Handling
- `EnemyKdTree` rebuilt every `KD_TREE_REFRESH_RATE = 0.1s`.
- `handle_enemy_bullet_collision`
  - Queries kd-tree within 50 units of each bullet.
  - Subtracts `BULLET_DAMAGE = 15.0` from enemy health.
- `handle_enemy_player_collision`
  - Queries kd-tree within 50 units of the player.
  - Reduces `Health` by `ENEMY_DAMAGE = 1.0` when any enemy is nearby.

## Death & Cleanup
- Enemies despawn when health ≤ 0; bullets are not removed on hit (decay via lifetime only).
- `WorldPlugin::despawn_all_game_entities` clears bullets, enemies, props, and player when leaving `InGame`.

## Testing & Debugging
- No automated combat tests; manual verification via live play.
- Consider writing unit tests for kd-tree radius queries or mocking collisions by injecting deterministic positions.
