use bevy::prelude::Resource;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone, Resource)]
pub struct GameConfig {
    pub window: WindowConfig,
    pub sprites: SpritesConfig,
    pub enemy_sprites: EnemySpritesConfig,
    pub world: WorldConfig,
    pub player: PlayerConfig,
    pub enemy: EnemyConfig,
    pub kd_tree: KdTreeConfig,
    pub gun: GunConfig,
    pub colors: ColorsConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpritesConfig {
    pub sprite_sheet_path: String,
    pub sprite_scale_factor: f32,
    pub tile_width: usize,
    pub tile_height: usize,
    pub sprite_sheet_width: usize,
    pub sprite_sheet_height: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EnemySpritesConfig {
    pub enemy_sprite_sheet_path: String,
    pub enemy_bg_path: String,
    pub enemy_tint_path: String,
    pub enemy_tile_width: usize,
    pub enemy_tile_height: usize,
    pub enemy_sprite_sheet_width: usize,
    pub enemy_sprite_sheet_height: usize,
    pub enemy_animation_frames: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WorldConfig {
    pub num_world_decorations: usize,
    pub world_width: f32,
    pub world_height: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PlayerConfig {
    pub speed: f32,
    pub health: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EnemyConfig {
    pub max_num_enemies: usize,
    pub damage: f32,
    pub spawn_rate_per_second: usize,
    pub health: f32,
    pub spawn_interval: f32,
    pub speed: f32,
    pub spawn_distance_min: f32,
    pub spawn_distance_max: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KdTreeConfig {
    pub refresh_rate: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GunConfig {
    pub bullet_spawn_interval: f32,
    pub bullet_time_secs: f32,
    pub bullet_speed: f32,
    pub bullet_damage: f32,
    pub num_bullets_per_shot: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ColorsConfig {
    pub bg_color: [u8; 3],
}

impl GameConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string("config.toml")?;
        let config: GameConfig = toml::from_str(&config_str)?;
        Ok(config)
    }

    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_else(|e| {
            eprintln!("Failed to load config.toml: {}. Using defaults.", e);
            Self::default()
        })
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig {
                width: 1200.0,
                height: 900.0,
            },
            sprites: SpritesConfig {
                sprite_sheet_path: "assets.png".to_string(),
                sprite_scale_factor: 3.0,
                tile_width: 16,
                tile_height: 16,
                sprite_sheet_width: 8,
                sprite_sheet_height: 8,
            },
            enemy_sprites: EnemySpritesConfig {
                enemy_sprite_sheet_path: "enemy.png".to_string(),
                enemy_bg_path: "enemy_bg.png".to_string(),
                enemy_tint_path: "enemy_tint.png".to_string(),
                enemy_tile_width: 32,
                enemy_tile_height: 32,
                enemy_sprite_sheet_width: 8,
                enemy_sprite_sheet_height: 1,
                enemy_animation_frames: 8,
            },
            world: WorldConfig {
                num_world_decorations: 500,
                world_width: 3000.0,
                world_height: 2500.0,
            },
            player: PlayerConfig {
                speed: 2.0,
                health: 100.0,
            },
            enemy: EnemyConfig {
                max_num_enemies: 20000,
                damage: 1.0,
                spawn_rate_per_second: 10000,
                health: 100.0,
                spawn_interval: 1.0,
                speed: 1.0,
                spawn_distance_min: 3000.0,
                spawn_distance_max: 8000.0,
            },
            kd_tree: KdTreeConfig {
                refresh_rate: 0.1,
            },
            gun: GunConfig {
                bullet_spawn_interval: 0.1,
                bullet_time_secs: 0.5,
                bullet_speed: 15.0,
                bullet_damage: 15.0,
                num_bullets_per_shot: 10,
            },
            colors: ColorsConfig {
                bg_color: [197, 204, 184],
            },
        }
    }
}
