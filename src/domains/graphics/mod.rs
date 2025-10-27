//! Graphics Domain - Owned by Graphics Agent
//!
//! Particles, animations, sprites, tilemap backgrounds

pub mod particles;
pub mod animation;
pub mod tilemap;

use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        // Placeholder
    }
}
