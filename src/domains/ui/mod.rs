//! UI Domain - Owned by UI Agent
//!
//! HUD, menus, choice UI, camera

pub mod hud;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Placeholder - will be populated during migration
    }
}
