//! Configuration subsystem - Owned by Gameplay Agent

pub mod constants;
pub mod loader;

// Re-export for backward compatibility
pub use constants::*;
pub use loader::*;
