# Bevy 0.17 Event System Clarification

## What I Got Wrong

I incorrectly tried to use `Message` and `MessageReader` thinking they were the Bevy 0.17 replacements for `Event` and `EventReader`. **This was wrong.**

## Actual Bevy 0.17 Event System

### Standard Pattern (Still Valid)
```rust
// Define event
#[derive(Event, Clone)]
pub struct MyEvent;

// Register event
app.add_event::<MyEvent>();

// Read events in system
fn my_system(mut events: EventReader<MyEvent>) {
    for event in events.read() {
        // handle event
    }
}
```

**This pattern still works and is the standard approach.**

### New Trigger/Observer Pattern (Alternative)
```rust
// Define event (same as before)
#[derive(Event, Clone)]
pub struct MyEvent;

// Observe events on entities
commands.entity(e).observe(|trigger: Trigger<MyEvent>| {
    // handle event for this entity
});

// Trigger events
commands.trigger(MyEvent);
```

**This is an ALTERNATIVE pattern, not a replacement.**

## What MessageReader Actually Is

`MessageReader` appears to be an internal Bevy type or a deprecated experimental API. It's NOT the standard way to handle events in Bevy 0.17.

## Correct Implementation for UI Widgets

### Events Definition (core/events.rs)
```rust
#[derive(Event, Clone)]
pub struct PauseMenuToggled;

#[derive(Event, Clone)]
pub struct SettingsChanged {
    pub volume: f32,
    pub difficulty: f32,
    pub particle_count: u32,
}
```

### Plugin Registration
```rust
app.add_event::<PauseMenuToggled>();
app.add_event::<SettingsChanged>();
```

### System Handlers
```rust
fn process_pause_toggle(
    mut events: EventReader<PauseMenuToggled>,
    // ... other params
) {
    for _ in events.read() {
        // handle toggle
    }
}

fn apply_settings_changes(
    mut events: EventReader<SettingsChanged>,
    // ... other params
) {
    for event in events.read() {
        // handle settings change
    }
}
```

## Files Corrected

1. `.worktrees/ui-widgets/src/domains/ui/pause_menu.rs`
   - Reverted `MessageReader` → `EventReader`
   - Reverted `add_message` → `add_event`

2. `.worktrees/ui-widgets/src/domains/ui/settings_panel.rs`
   - Reverted `MessageReader` → `EventReader`
   - Reverted `add_message` → `add_event`

3. `.worktrees/ui-widgets/src/core/events.rs`
   - Events correctly defined with `#[derive(Event, Clone)]`

## Expected Result

UI widgets branch should now compile successfully with the standard Bevy 0.17 event pattern.

## References

- Bevy 0.17 Event System: https://docs.rs/bevy/0.17.2/bevy/ecs/event/index.html
- Trigger/Observer Pattern: https://bevyengine.org/news/bevy-0-17/ (optional alternative)
