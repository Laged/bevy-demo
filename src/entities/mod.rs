//! Entity definitions - Owned by Gameplay Agent
//!
//! Player, Enemy, World entities and their associated components

pub mod player;
pub mod enemy;
pub mod world;

// Re-export commonly used types
pub use player::{PlayerPlugin, Player, PlayerState, Health};
pub use enemy::{EnemyPlugin, Enemy, EnemyType, EnemyColor};
pub use world::{WorldPlugin, GameEntity};
