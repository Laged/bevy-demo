//! Core infrastructure shared across domains
//!
//! Maintained by Gameplay Agent, read by all domains

pub mod events;
pub mod choice_system;
pub mod state;
pub mod collision;
pub mod resources;

// Re-export commonly used types
pub use choice_system::{
    ChoiceContext, ChoiceOption, ChoiceType, ChoiceHandler,
    ChoiceHandlerRegistry, PendingChoice, ChoiceMetadata,
};
pub use state::{GameState, is_paused_for_choice};
pub use collision::CollisionPlugin;
pub use resources::{ResourcesPlugin, GlobalTextureAtlas, CursorPosition};
