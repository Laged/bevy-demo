//! Test utilities for headless Bevy app testing
//!
//! This module provides helpers for creating and testing Bevy systems in a headless
//! environment (no rendering, minimal plugins). Use these utilities to:
//! - Create test apps with MinimalPlugins + custom plugins
//! - Spawn entities with specific component configurations
//! - Step through frames and verify game logic without GPU overhead

pub mod app;
pub mod entities;
pub mod simulation;
