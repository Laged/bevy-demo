//! Game state machine - Owned by Gameplay Agent
//!
//! Defines the core game states including choice/pause states

use bevy::prelude::*;
use crate::core::ChoiceContext;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    GameInit,
    InGame,
    PlayerChoice(ChoiceContext),  // NEW: Pause for player decision
    ApplyingChoice,                // NEW: Process selection
}

/// Check if game is paused for a player choice
pub fn is_paused_for_choice(state: Res<State<GameState>>) -> bool {
    matches!(state.get(), GameState::PlayerChoice(_))
}
