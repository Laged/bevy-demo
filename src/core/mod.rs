//! Core infrastructure shared across domains
//!
//! Maintained by Gameplay Agent, read by all domains

pub mod events;
pub mod choice_system;

// Re-export commonly used types
pub use choice_system::{
    ChoiceContext, ChoiceOption, ChoiceType, ChoiceHandler,
    ChoiceHandlerRegistry, PendingChoice, ChoiceMetadata,
};
