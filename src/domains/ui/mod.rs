//! UI Domain - Owned by UI Agent
//!
//! HUD, menus, choice UI, camera

pub mod hud;
pub mod camera;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Placeholder - will be populated during migration
    }
}
