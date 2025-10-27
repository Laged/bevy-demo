//! Tilemap arena background system - Owned by Graphics Agent
//!
//! Creates colored arena zones for visual distinction:
//! - Safe zone (green tint)
//! - Danger zone (red tint)
//! - Neutral zone (default/gray)
//!
//! Uses sprite grid approach for WASM compatibility and simplicity.
//! No destructible tiles - purely visual background layer.

use bevy::prelude::*;
use crate::core::state::GameState;
use crate::plugin_mode::PluginMode;

/// Plugin for arena tilemap background rendering
pub struct TilemapPlugin {
    mode: PluginMode,
}

impl TilemapPlugin {
    pub fn new(mode: PluginMode) -> Self {
        Self { mode }
    }
}

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        match self.mode {
            PluginMode::Standard => {
                app.add_systems(OnEnter(GameState::GameInit), setup_tilemap_zones);
            }
            PluginMode::Headless => {
                // Tilemap zones exist in headless for logic testing (no rendering)
                app.add_systems(OnEnter(GameState::GameInit), setup_tilemap_zones_headless);
            }
        }
    }
}

/// Marker component for tilemap zone entities
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArenaZone {
    Safe,
    Danger,
    Neutral,
}

/// Tilemap tile component (for tracking grid position)
#[derive(Component, Debug, Clone, Copy)]
pub struct TilemapTile {
    pub zone: ArenaZone,
    pub grid_x: i32,
    pub grid_y: i32,
}

/// Configuration for tilemap zones
#[derive(Resource, Debug, Clone)]
pub struct TilemapConfig {
    pub tile_size: f32,
    pub safe_zone_radius: f32,
    pub danger_zone_outer_radius: f32,
    pub safe_zone_color: Color,
    pub danger_zone_color: Color,
    pub neutral_zone_color: Color,
}

impl Default for TilemapConfig {
    fn default() -> Self {
        Self {
            tile_size: 48.0, // 16px sprite * 3.0 scale factor
            safe_zone_radius: 300.0,
            danger_zone_outer_radius: 800.0,
            safe_zone_color: Color::srgba(0.5, 1.0, 0.5, 0.3), // Green tint
            danger_zone_color: Color::srgba(1.0, 0.5, 0.5, 0.3), // Red tint
            neutral_zone_color: Color::srgba(0.8, 0.8, 0.8, 0.2), // Gray tint
        }
    }
}

/// Setup tilemap zones in visual mode (with rendering)
fn setup_tilemap_zones(
    mut commands: Commands,
    texture_atlas: Res<crate::core::resources::GlobalTextureAtlas>,
    config: Res<crate::domains::gameplay::config::loader::GameConfig>,
) {
    let tilemap_config = TilemapConfig::default();
    commands.insert_resource(tilemap_config.clone());

    let tile_size = tilemap_config.tile_size;
    let world_width = config.world.world_width;
    let world_height = config.world.world_height;

    // Calculate grid dimensions
    let grid_cols = (world_width * 2.0 / tile_size).ceil() as i32;
    let grid_rows = (world_height * 2.0 / tile_size).ceil() as i32;

    // Spawn tiles in a grid pattern
    for grid_x in -grid_cols / 2..=grid_cols / 2 {
        for grid_y in -grid_rows / 2..=grid_rows / 2 {
            let world_x = grid_x as f32 * tile_size;
            let world_y = grid_y as f32 * tile_size;
            let distance = (world_x * world_x + world_y * world_y).sqrt();

            // Determine zone based on distance from center
            let (zone, color) = if distance < tilemap_config.safe_zone_radius {
                (ArenaZone::Safe, tilemap_config.safe_zone_color)
            } else if distance < tilemap_config.danger_zone_outer_radius {
                (ArenaZone::Danger, tilemap_config.danger_zone_color)
            } else {
                (ArenaZone::Neutral, tilemap_config.neutral_zone_color)
            };

            // Use tile index 24 (grass/floor tile from sprite sheet)
            commands.spawn((
                Sprite {
                    image: texture_atlas.image.clone().unwrap(),
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas.layout.clone().unwrap(),
                        index: 24,
                    }),
                    color,
                    ..default()
                },
                Transform::from_translation(Vec3::new(world_x, world_y, -2.0))
                    .with_scale(Vec3::splat(config.sprites.sprite_scale_factor)),
                TilemapTile { zone, grid_x, grid_y },
                crate::entities::world::GameEntity,
            ));
        }
    }
}

/// Setup tilemap zones in headless mode (no rendering, just entities for testing)
fn setup_tilemap_zones_headless(
    mut commands: Commands,
    config: Res<crate::domains::gameplay::config::loader::GameConfig>,
) {
    let tilemap_config = TilemapConfig::default();
    commands.insert_resource(tilemap_config.clone());

    let tile_size = tilemap_config.tile_size;
    let world_width = config.world.world_width;
    let world_height = config.world.world_height;

    // Calculate grid dimensions
    let grid_cols = (world_width * 2.0 / tile_size).ceil() as i32;
    let grid_rows = (world_height * 2.0 / tile_size).ceil() as i32;

    // Spawn minimal tile entities (no sprites, just markers for testing)
    for grid_x in -grid_cols / 2..=grid_cols / 2 {
        for grid_y in -grid_rows / 2..=grid_rows / 2 {
            let world_x = grid_x as f32 * tile_size;
            let world_y = grid_y as f32 * tile_size;
            let distance = (world_x * world_x + world_y * world_y).sqrt();

            // Determine zone based on distance from center
            let zone = if distance < tilemap_config.safe_zone_radius {
                ArenaZone::Safe
            } else if distance < tilemap_config.danger_zone_outer_radius {
                ArenaZone::Danger
            } else {
                ArenaZone::Neutral
            };

            commands.spawn((
                Transform::from_translation(Vec3::new(world_x, world_y, -2.0)),
                TilemapTile { zone, grid_x, grid_y },
                crate::entities::world::GameEntity,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tilemap_config_defaults() {
        let config = TilemapConfig::default();
        assert_eq!(config.tile_size, 48.0);
        assert_eq!(config.safe_zone_radius, 300.0);
        assert_eq!(config.danger_zone_outer_radius, 800.0);
    }

    #[test]
    fn test_arena_zone_types() {
        let safe = ArenaZone::Safe;
        let danger = ArenaZone::Danger;
        let neutral = ArenaZone::Neutral;

        assert_ne!(safe, danger);
        assert_ne!(danger, neutral);
        assert_ne!(safe, neutral);
    }
}
