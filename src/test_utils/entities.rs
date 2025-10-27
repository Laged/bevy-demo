//! Entity spawning utilities for tests

use bevy::prelude::*;
use crate::player::{Player, Health};
use crate::enemy::{Enemy, EnemyType, EnemyColor};
use crate::gun::Bullet;

/// Spawns a test player entity with specified position and health.
///
/// Only spawns logical components (Player, Health, Transform).
/// No sprites or visual components.
///
/// # Example
/// ```ignore
/// use hell_game::test_utils::entities::spawn_test_player;
/// let player_id = spawn_test_player(&mut app, Vec3::ZERO, 100.0);
/// ```
pub fn spawn_test_player(app: &mut App, position: Vec3, health: f32) -> Entity {
    app.world_mut().spawn((
        Player,
        Health(health),
        Transform::from_translation(position),
    )).id()
}

/// Spawns a test enemy entity with specified position, health, and type.
///
/// Only spawns logical components (Enemy, Health, Transform, EnemyType, EnemyColor).
/// No sprites or visual components.
///
/// # Example
/// ```ignore
/// use hell_game::test_utils::entities::spawn_test_enemy;
/// use hell_game::enemy::EnemyType;
/// let enemy_id = spawn_test_enemy(&mut app, Vec3::new(100.0, 0.0, 0.0), 50.0, EnemyType::Green);
/// ```
pub fn spawn_test_enemy(
    app: &mut App,
    position: Vec3,
    health: f32,
    enemy_type: EnemyType,
) -> Entity {
    app.world_mut().spawn((
        Enemy { health },
        Transform::from_translation(position),
        enemy_type,
        EnemyColor(Color::srgb(1.0, 0.0, 0.0)), // Default red color for tests
    )).id()
}

/// Spawns a test bullet entity with specified position and direction.
///
/// Only spawns logical components (Bullet, Transform).
/// No sprites or visual components.
///
/// Note: Velocity is handled by the BulletDirection component in the gun system.
/// This simplified version just spawns a Bullet marker with a Transform.
///
/// # Example
/// ```ignore
/// use hell_game::test_utils::entities::spawn_test_bullet;
/// let bullet_id = spawn_test_bullet(&mut app, Vec3::ZERO, Vec3::X);
/// ```
pub fn spawn_test_bullet(app: &mut App, position: Vec3, _direction: Vec3) -> Entity {
    app.world_mut().spawn((
        Bullet,
        Transform::from_translation(position),
    )).id()
}
