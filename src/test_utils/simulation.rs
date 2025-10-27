//! Simulation control utilities for stepping through frames and managing state

use bevy::prelude::*;

/// Runs the app for the specified number of frames.
///
/// Each frame executes one full ECS update cycle.
///
/// # Example
/// ```ignore
/// run_frames(&mut app, 60); // Simulate 60 frames (~1 second at 60 FPS)
/// ```
pub fn run_frames(app: &mut App, num_frames: usize) {
    // TODO: Implement in task 9
    todo!("Will be implemented in task 9")
}

/// Transitions the app to the specified game state and runs one frame.
///
/// This inserts a `NextState<S>` resource to trigger the state transition,
/// then runs one update to apply the transition.
///
/// # Example
/// ```ignore
/// set_state(&mut app, GameState::InGame);
/// ```
pub fn set_state<S: States>(app: &mut App, state: S) {
    app.world_mut().insert_resource(NextState::Pending(state));
    app.update();
}

/// Gets the current game state from the app.
///
/// Returns None if the state resource doesn't exist.
///
/// # Example
/// ```ignore
/// let current = get_state::<GameState>(&app);
/// assert_eq!(current, Some(GameState::InGame));
/// ```
pub fn get_state<S: States + Copy>(app: &App) -> Option<S> {
    app.world().get_resource::<State<S>>().map(|s| s.get().clone())
}
