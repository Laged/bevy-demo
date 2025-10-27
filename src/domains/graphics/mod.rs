//! Graphics Domain - Owned by Graphics Agent
//!
//! Particles, animations, sprites

pub mod particles;
pub mod animation;

use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        // Placeholder
    }
}
