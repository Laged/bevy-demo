# Enemy Forces (Tech)

## Entities & Components
- Spawned via `EnemyPlugin::spawn_enemies` with:
  - `Sprite` (atlas index determined by `EnemyType`)
  - `Transform` seeded to random XY around the player
  - `Enemy { health: 100.0 }`
  - `EnemyType` enum (Green, Red, Skin)
  - `AnimationTimer` (0.08s repeating)
  - `GameEntity`

## Spawn Scheduling
- `spawn_enemies` gated by `on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))`.
- Calculates new spawn count as `min(MAX_NUM_ENEMIES - existing_count, SPAWN_RATE_PER_SECOND)`.
- Uses `get_random_position_around` to pick a polar coordinate 1,000–5,000 units from the player.

## Behaviour Systems
- `update_enemy_transform` moves each enemy toward the player using normalized direction × `ENEMY_SPEED`.
- `despawn_dead_enemies` removes entities when `Enemy.health <= 0`.
- Animation system flips sprites to face the player and cycles frames.

## Data Structures
- `EnemyKdTree` resource stores positions for efficient collision queries.
- `Collidable` struct implements `KdPoint` for kd-tree node storage.

## Issues & Improvements
- Lerp-free movement means enemies can overlap heavily; consider separation or avoidance.
- Health stored on the component; there is no visual feedback when reduced.
- Spawn timer is global; there is no difficulty scaling beyond raw unit count.
