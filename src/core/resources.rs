//! Shared resources and asset loading - Owned by Gameplay Agent (read by all domains)
//!
//! Global texture atlases, cursor position tracking, and asset loading systems

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::state::GameState;

pub struct ResourcesPlugin;

#[derive(Resource)]
pub struct GlobalTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
    pub enemy_layout: Option<Handle<TextureAtlasLayout>>,
    pub enemy_image: Option<Handle<Image>>,
    pub enemy_bg_image: Option<Handle<Image>>,
    pub enemy_tint_image: Option<Handle<Image>>,
}
#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalTextureAtlas::default())
            .insert_resource(CursorPosition(None))
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(
                Update,
                update_cursor_position.run_if(in_state(GameState::InGame)),
            );
    }
}

fn load_assets(
    mut handle: ResMut<GlobalTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
    config: Res<crate::config_loader::GameConfig>,
) {
    // Load main sprite sheet
    handle.image = Some(asset_server.load(&config.sprites.sprite_sheet_path));

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(config.sprites.tile_width as u32, config.sprites.tile_height as u32),
        config.sprites.sprite_sheet_width as u32,
        config.sprites.sprite_sheet_height as u32,
        None,
        None,
    );
    handle.layout = Some(texture_atlas_layouts.add(layout));

    // Load enemy sprite sheets (legacy single file + new dual layer)
    handle.enemy_image = Some(asset_server.load(&config.enemy_sprites.enemy_sprite_sheet_path));
    handle.enemy_bg_image = Some(asset_server.load(&config.enemy_sprites.enemy_bg_path));
    handle.enemy_tint_image = Some(asset_server.load(&config.enemy_sprites.enemy_tint_path));

    let enemy_layout = TextureAtlasLayout::from_grid(
        UVec2::new(config.enemy_sprites.enemy_tile_width as u32, config.enemy_sprites.enemy_tile_height as u32),
        config.enemy_sprites.enemy_sprite_sheet_width as u32,
        config.enemy_sprites.enemy_sprite_sheet_height as u32,
        None,
        None,
    );
    handle.enemy_layout = Some(texture_atlas_layouts.add(enemy_layout));

    next_state.set(GameState::MainMenu);
}

fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if window_query.is_empty() || camera_query.is_empty() {
        cursor_pos.0 = None;
        return;
    }

    let Ok((camera, camera_transform)) = camera_query.single() else { return; };
    let Ok(window) = window_query.single() else { return; };
    cursor_pos.0 = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate());
}

impl Default for GlobalTextureAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
            enemy_layout: None,
            enemy_image: None,
            enemy_bg_image: None,
            enemy_tint_image: None,
        }
    }
}
