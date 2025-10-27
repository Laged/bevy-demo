//! Cross-domain events using Bevy 0.17 Trigger/Observer pattern

use bevy::prelude::*;
use crate::core::choice_system::{ChoiceOption, ChoiceMetadata, ChoiceContext};

/// Request to trigger a player choice pause
#[derive(Event, Clone)]
pub struct TriggerChoiceEvent {
    pub context: ChoiceContext,
    pub metadata: ChoiceMetadata,
}

/// Notify UI to display choice options
#[derive(Event, Clone)]
pub struct ShowChoiceUIEvent {
    pub context: ChoiceContext,
    pub options: Vec<ChoiceOption>,
}

/// Player has selected a choice option
#[derive(Event, Clone)]
pub struct ChoiceSelectedEvent {
    pub choice: ChoiceOption,
}

/// Hide the choice UI
#[derive(Event)]
pub struct HideChoiceUIEvent;

/// Pause menu toggled (pause/unpause)
#[derive(Event, Clone)]
pub struct PauseMenuToggled;

/// Settings changed in settings panel
#[derive(Event, Clone)]
pub struct SettingsChanged {
    pub volume: f32,
    pub difficulty: f32,
    pub particle_count: u32,
}

/// Minimap camera is ready and positioned
#[derive(Event, Clone)]
pub struct MinimapCameraReady {
    pub camera_entity: Entity,
}
