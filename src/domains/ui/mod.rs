//! UI Domain - Owned by UI Agent
//!
//! HUD, menus, choice UI, camera

pub mod hud;
pub mod camera;
pub mod pause_menu;
pub mod settings_panel;
pub mod minimap;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Placeholder - will be populated during migration
    }
}
