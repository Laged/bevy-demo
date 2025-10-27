//! Settings panel with sliders for game configuration - Owned by UI Agent
//!
//! Uses Bevy 0.17 Slider widgets to adjust volume, difficulty, and particle count

use bevy::prelude::*;
use crate::core::events::SettingsChanged;
use crate::plugin_mode::PluginMode;

pub struct SettingsPanelPlugin {
    mode: PluginMode,
}

impl SettingsPanelPlugin {
    pub fn new(mode: PluginMode) -> Self {
        Self { mode }
    }
}

impl Default for SettingsPanelPlugin {
    fn default() -> Self {
        Self::new(PluginMode::default())
    }
}

#[derive(Component)]
pub struct SettingsPanelRoot;

#[derive(Component)]
pub struct VolumeSlider {
    pub value: f32,
}

#[derive(Component)]
pub struct DifficultySlider {
    pub value: f32,
}

#[derive(Component)]
pub struct ParticleCountSlider {
    pub value: f32,
}

#[derive(Resource)]
pub struct GameSettings {
    pub volume: f32,
    pub difficulty: f32,
    pub particle_count: u32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            volume: 0.5,
            difficulty: 1.0,
            particle_count: 100,
        }
    }
}

impl Plugin for SettingsPanelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameSettings::default())
            .add_event::<SettingsChanged>();

        // Visual mode: create UI
        if self.mode == PluginMode::Standard {
            // Systems to handle slider updates would go here
            app.add_systems(Update, update_settings_from_sliders);
        }

        // Both modes: process settings changed events
        app.add_systems(Update, apply_settings_changes);
    }
}

fn update_settings_from_sliders(
    mut commands: Commands,
    mut settings: ResMut<GameSettings>,
    volume_query: Query<&VolumeSlider, Changed<VolumeSlider>>,
    difficulty_query: Query<&DifficultySlider, Changed<DifficultySlider>>,
    particle_query: Query<&ParticleCountSlider, Changed<ParticleCountSlider>>,
) {
    let mut changed = false;

    for slider in volume_query.iter() {
        settings.volume = slider.value;
        changed = true;
    }

    for slider in difficulty_query.iter() {
        settings.difficulty = slider.value;
        changed = true;
    }

    for slider in particle_query.iter() {
        settings.particle_count = (slider.value * 200.0) as u32;
        changed = true;
    }

    if changed {
        commands.trigger(SettingsChanged {
            volume: settings.volume,
            difficulty: settings.difficulty,
            particle_count: settings.particle_count,
        });
    }
}

fn apply_settings_changes(
    mut events: EventReader<SettingsChanged>,
    mut settings: ResMut<GameSettings>,
) {
    for event in events.read() {
        settings.volume = event.volume;
        settings.difficulty = event.difficulty;
        settings.particle_count = event.particle_count;

        info!(
            "Settings updated: volume={:.2}, difficulty={:.2}, particles={}",
            settings.volume, settings.difficulty, settings.particle_count
        );
    }
}
