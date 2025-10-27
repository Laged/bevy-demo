//! Testing Domain - Owned by Testing Agent
//!
//! Test harnesses, benchmarks, utilities

pub mod harness;
pub mod helpers;
pub mod simulation;
pub mod benchmarks;

// Re-export for backward compatibility with old test_utils structure
pub mod app {
    pub use super::harness::*;
}

pub mod entities {
    pub use super::helpers::*;
}

// Re-export commonly used functions
pub use harness::create_headless_app;
pub use helpers::{spawn_test_player, spawn_test_enemy, spawn_test_bullet};
pub use simulation::{run_frames, set_state, get_state};
