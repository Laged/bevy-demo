//! Entity spawning utilities for tests

use bevy::prelude::*;

/// Spawns a test player entity with specified position and health.
///
/// Only spawns logical components (Player, Health, Transform).
/// No sprites or visual components.
///
/// # Example
/// ```ignore
/// let player_id = spawn_test_player(&mut app, Vec3::ZERO, 100.0);
/// ```
pub fn spawn_test_player(app: &mut App, position: Vec3, health: f32) -> Entity {
    // TODO: Implement in task 8
    todo!("Will be implemented in task 8")
}

/// Spawns a test enemy entity with specified position, health, and type.
///
/// Only spawns logical components (Enemy, Health, Transform).
/// No sprites or visual components.
pub fn spawn_test_enemy(
    app: &mut App,
    position: Vec3,
    health: f32,
    enemy_type: crate::enemy::EnemyType,
) -> Entity {
    // TODO: Implement in task 8
    todo!("Will be implemented in task 8")
}

/// Spawns a test bullet entity with specified position and velocity.
///
/// Only spawns logical components (Bullet, Transform, velocity).
/// No sprites or visual components.
pub fn spawn_test_bullet(app: &mut App, position: Vec3, velocity: Vec3) -> Entity {
    // TODO: Implement in task 8
    todo!("Will be implemented in task 8")
}
