//! Gameplay Domain - Owned by Gameplay Agent
//!
//! Balance, spawning, combat, state machine

pub mod config;
pub mod combat;

use bevy::prelude::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        // Placeholder
    }
}
