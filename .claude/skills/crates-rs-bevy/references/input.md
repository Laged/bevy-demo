# Crates-Rs-Bevy - Input

**Pages:** 32

---

## Struct UntypedAssetLoadFailedEvent Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.UntypedAssetLoadFailedEvent.html

**Contents:**
- Struct UntypedAssetLoadFailedEvent Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for UntypedAssetLoadFailedEvent
    - fn clone(&self) -> UntypedAssetLoadFailedEvent
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for UntypedAssetLoadFailedEvent
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<A> From<&AssetLoadFailedEvent<A>> for UntypedAssetLoadFailedEventwhere A: Asset,
    - fn from(value: &AssetLoadFailedEvent<A>) -> UntypedAssetLoadFailedEvent

An untyped version of AssetLoadFailedEvent.

The stable identifier of the asset that failed to load.

The asset path that was attempted.

Why the asset failed to load.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct UntypedAssetLoadFailedEvent {
    pub id: UntypedAssetId,
    pub path: AssetPath<'static>,
    pub error: AssetLoadError,
}
```

---

## Struct ButtonInput Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input/struct.ButtonInput.html

**Contents:**
- Struct ButtonInput Copy item path
  - §Usage
  - §Multiple systems
  - §Performance
  - §Window focus
  - §Examples
  - §Note
- Implementations§
  - impl<T> ButtonInput<T>where T: Clone + Eq + Hash + Send + Sync + 'static,
    - pub fn press(&mut self, input: T)

A “press-able” input of type T.

This type can be used as a resource to keep the current state of an input, by reacting to events from the input. For a given input value:

In case multiple systems are checking for ButtonInput::just_pressed or ButtonInput::just_released but only one should react, for example when modifying a Resource, you should consider clearing the input state, either by:

For all operations, the following conventions are used:

See Rust’s std::collections doc on performance for more details on the conventions used here.

ButtonInput<KeyCode> is tied to window focus. For example, if the user holds a button while the window loses focus, ButtonInput::just_released will be triggered. Similarly if the window regains focus, ButtonInput::just_pressed will be triggered.

ButtonInput<GamepadButton> is independent of window focus.

Reading and checking against the current set of pressed buttons:

When adding this resource for a new input type, you should:

Note: Calling clear from a ResMut will trigger change detection. It may be preferable to use DetectChangesMut::bypass_change_detection to avoid causing the resource to always be marked as changed.

Registers a press for the given input.

Returns true if the input has been pressed.

Returns true if any item in inputs has been pressed.

Returns true if all items in inputs have been pressed.

Registers a release for the given input.

Registers a release for all currently pressed inputs.

Returns true if the input has been pressed during the current frame.

Note: This function does not imply information regarding the current state of ButtonInput::pressed or ButtonInput::just_released.

Returns true if any item in inputs has been pressed during the current frame.

Clears the just_pressed state of the input and returns true if the input has just been pressed.

Future calls to ButtonInput::just_pressed for the given input will return false until a new press event occurs.

Returns true if the input has been released during the current frame.

Note: This function does not imply information regarding the current state of ButtonInput::pressed or ButtonInput::just_pressed.

Returns true if any item in inputs has just been released.

Returns true if all items in inputs have just been released.

Returns true if all items in inputs have been just pressed.

Clears the just_released state of the input and returns true if the input has just been released.

Future calls to ButtonInput::just_released for the given 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct ButtonInput<T>where
    T: Clone + Eq + Hash + Send + Sync + 'static,{ /* private fields */ }
```

Example 2 (unknown):
```unknown
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Update,
            print_mouse.run_if(resource_changed::<ButtonInput<MouseButton>>),
        )
        .add_systems(
            Update,
            print_keyboard.run_if(resource_changed::<ButtonInput<KeyCode>>),
        )
        .run();
}

fn print_mouse(mouse: Res<ButtonInput<MouseButton>>) {
    println!("Mouse: {:?}", mouse.get_pressed().collect::<Vec<_>>());
}

fn print_keyboard(keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]
...
```

Example 3 (unknown):
```unknown
74fn transition_to_in_game_system(
75    mut next_state: ResMut<NextState<AppState>>,
76    keyboard_input: Res<ButtonInput<KeyCode>>,
77) {
78    if keyboard_input.pressed(KeyCode::Space) {
79        next_state.set(AppState::InGame);
80    }
81}
```

Example 4 (unknown):
```unknown
136fn trigger_hooks(
137    mut commands: Commands,
138    keys: Res<ButtonInput<KeyCode>>,
139    index: Res<MyComponentIndex>,
140) {
141    for (key, entity) in index.iter() {
142        if !keys.pressed(*key) {
143            commands.entity(*entity).remove::<MyComponent>();
144        }
145    }
146    for key in keys.get_just_pressed() {
147        commands.spawn(MyComponent(*key));
148    }
149}
```

---

## Crate input_focus Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input_focus/index.html

**Contents:**
- Crate input_focus Copy item path
- Modules§
- Structs§
- Enums§
- Traits§
- Functions§
- Type Aliases§

A UI-centric focus system for Bevy.

This crate provides a system for managing input focus in Bevy applications, including:

This crate does not provide any integration with UI widgets: this is the responsibility of the widget crate, which should depend on bevy_input_focus.

---

## Function set_initial_focus Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input_focus/fn.set_initial_focus.html

**Contents:**
- Function set_initial_focus Copy item path

If no entity is focused, sets the focus to the primary window, if any.

**Examples:**

Example 1 (unknown):
```unknown
pub fn set_initial_focus(
    input_focus: ResMut<'_, InputFocus>,
    window: Single<'_, '_, Entity, With<PrimaryWindow>>,
)
```

---

## Struct AcquireFocus Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input_focus/struct.AcquireFocus.html

**Contents:**
- Struct AcquireFocus Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for AcquireFocus
    - fn clone(&self) -> AcquireFocus
    - fn clone_from(&mut self, source: &Self)
  - impl EntityEvent for AcquireFocuswhere AcquireFocus: Send + Sync + 'static,
    - fn event_target(&self) -> Entity
    - fn event_target_mut(&mut self) -> &mut Entity
  - impl Event for AcquireFocuswhere AcquireFocus: Send + Sync + 'static,

An event which is used to set input focus. Trigger this on an entity, and it will bubble until it finds a focusable entity, and then set focus to it.

The entity that has acquired focus.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AcquireFocus {
    pub focused_entity: Entity,
    /* private fields */
}
```

---

## Struct KeyboardInput Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input/keyboard/struct.KeyboardInput.html

**Contents:**
- Struct KeyboardInput Copy item path
  - §Usage
- Fields§
- Trait Implementations§
  - impl Clone for KeyboardInput
    - fn clone(&self) -> KeyboardInput
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for KeyboardInput
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for KeyboardInput

A keyboard input event.

This event is the translated version of the WindowEvent::KeyboardInput from the winit crate. It is available to the end user and can be used for game logic.

The event is consumed inside of the keyboard_input_system to update the ButtonInput<KeyCode> and ButtonInput<Key> resources.

The physical key code of the key.

This corresponds to the location of the key independent of the keyboard layout.

The logical key of the input.

This corresponds to the actual key taking keyboard layout into account.

The press state of the key.

Contains the text produced by this keypress.

In most cases this is identical to the content of the Character variant of logical_key. However, on Windows when a dead key was pressed earlier but cannot be combined with the character from this keypress, the produced text will consist of two characters: the dead-key-character followed by the character resulting from this keypress.

This is None if the current keypress cannot be interpreted as text.

On some systems, holding down a key for some period of time causes that key to be repeated as though it were being pressed and released repeatedly. This field is true if this event is the result of one of those repeats.

Window that received the input.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct KeyboardInput {
    pub key_code: KeyCode,
    pub logical_key: Key,
    pub state: ButtonState,
    pub text: Option<SmolStr>,
    pub repeat: bool,
    pub window: Entity,
}
```

---

## Struct InputFocusVisible Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input_focus/struct.InputFocusVisible.html

**Contents:**
- Struct InputFocusVisible Copy item path
- Tuple Fields§
- Trait Implementations§
  - impl Clone for InputFocusVisible
    - fn clone(&self) -> InputFocusVisible
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for InputFocusVisible
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for InputFocusVisible
    - fn default() -> InputFocusVisible

Resource representing whether the input focus indicator should be visible on UI elements.

Note that this resource is not used by bevy_input_focus itself, but is provided for convenience to UI widgets or frameworks that want to display a focus indicator. InputFocus may still be Some even if the focus indicator is not visible.

The value of this resource should be set by your focus navigation solution. For a desktop/web style of user interface this would be set to true when the user presses the tab key, and set to false when the user clicks on a different element. By contrast, a console-style UI intended to be navigated with a gamepad may always have the focus indicator visible.

To easily access information about whether focus indicators should be shown for a given entity, use the IsFocused trait.

By default, this resource is set to false.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct InputFocusVisible(pub bool);
```

---

## Struct CursorOptions Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.CursorOptions.html

**Contents:**
- Struct CursorOptions Copy item path
- Fields§
    - §Platform-specific
    - §Platform-specific
    - §Platform-specific
- Trait Implementations§
  - impl Clone for CursorOptions
    - fn clone(&self) -> CursorOptions
    - fn clone_from(&mut self, source: &Self)
  - impl Component for CursorOptionswhere CursorOptions: Send + Sync + 'static,

Cursor data for a Window.

Whether the cursor is visible or not.

Whether or not the cursor is locked by or confined within the window.

Since macOS and X11 don’t have full CursorGrabMode support, we first try to set the grab mode that was asked for. If it doesn’t work then use the alternate grab mode.

Set whether or not mouse events within this window are captured or fall through to the Window below.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CursorOptions {
    pub visible: bool,
    pub grab_mode: CursorGrabMode,
    pub hit_test: bool,
}
```

---

## Struct AssetLoadFailedEvent Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.AssetLoadFailedEvent.html

**Contents:**
- Struct AssetLoadFailedEvent Copy item path
- Fields§
- Implementations§
  - impl<A> AssetLoadFailedEvent<A>where A: Asset,
    - pub fn untyped(&self) -> UntypedAssetLoadFailedEvent
- Trait Implementations§
  - impl<A> Clone for AssetLoadFailedEvent<A>where A: Clone + Asset,
    - fn clone(&self) -> AssetLoadFailedEvent<A>
    - fn clone_from(&mut self, source: &Self)
  - impl<A> Debug for AssetLoadFailedEvent<A>where A: Debug + Asset,

A Message emitted when a specific Asset fails to load.

For an untyped equivalent, see UntypedAssetLoadFailedEvent.

The stable identifier of the asset that failed to load.

The asset path that was attempted.

Why the asset failed to load.

Converts this to an “untyped” / “generic-less” asset error event that stores the type information.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AssetLoadFailedEvent<A>where
    A: Asset,{
    pub id: AssetId<A>,
    pub path: AssetPath<'static>,
    pub error: AssetLoadError,
}
```

---

## Struct WindowCloseRequested Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.WindowCloseRequested.html

**Contents:**
- Struct WindowCloseRequested Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for WindowCloseRequested
    - fn clone(&self) -> WindowCloseRequested
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for WindowCloseRequested
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for WindowCloseRequested
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<WindowCloseRequested, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,

An event that is sent whenever the operating systems requests that a window be closed. This will be sent when the close button of the window is pressed.

If the default WindowPlugin is used, these events are handled by closing the corresponding Window. To disable this behavior, set close_when_requested on the WindowPlugin to false.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WindowCloseRequested {
    pub window: Entity,
}
```

---

## Struct Touch Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input/touch/struct.Touch.html

**Contents:**
- Struct Touch Copy item path
  - §Usage
- Implementations§
  - impl Touch
    - pub fn delta(&self) -> Vec2
    - pub fn distance(&self) -> Vec2
    - pub fn id(&self) -> u64
      - Examples found in repository?
    - pub fn start_position(&self) -> Vec2
    - pub fn start_force(&self) -> Option<ForceTouch>

It is used to store the position and force of a touch input and also the id of the finger. The data of the touch input comes from the TouchInput event and is being stored inside of the Touches bevy resource.

The delta of the current position and the previous_position.

The distance of the start_position and the current position.

Returns the id of the touch.

Returns the start_position of the touch.

Returns the start_force of the touch.

Returns the previous_position of the touch.

Returns the previous_force of the touch.

Returns the current position of the touch.

Returns the current force of the touch.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Touch { /* private fields */ }
```

Example 2 (unknown):
```unknown
12fn touch_system(touches: Res<Touches>) {
13    for touch in touches.iter_just_pressed() {
14        info!(
15            "just pressed touch with id: {}, at: {}",
16            touch.id(),
17            touch.position()
18        );
19    }
20
21    for touch in touches.iter_just_released() {
22        info!(
23            "just released touch with id: {}, at: {}",
24            touch.id(),
25            touch.position()
26        );
27    }
28
29    for touch in touches.iter_just_canceled() {
30        info!("canceled touch with id: {}", touch.id());
31    }
32
33    // you can also iterate
...
```

Example 3 (unknown):
```unknown
12fn touch_system(touches: Res<Touches>) {
13    for touch in touches.iter_just_pressed() {
14        info!(
15            "just pressed touch with id: {}, at: {}",
16            touch.id(),
17            touch.position()
18        );
19    }
20
21    for touch in touches.iter_just_released() {
22        info!(
23            "just released touch with id: {}, at: {}",
24            touch.id(),
25            touch.position()
26        );
27    }
28
29    for touch in touches.iter_just_canceled() {
30        info!("canceled touch with id: {}", touch.id());
31    }
32
33    // you can also iterate
...
```

---

## Derive Macro AnimationEvent Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/derive.AnimationEvent.html

**Contents:**
- Derive Macro AnimationEvent Copy item path

Implements the AnimationEvent trait for a type - see the trait docs for an example usage.

**Examples:**

Example 1 (unknown):
```unknown
#[derive(AnimationEvent)]
```

---

## Module touch Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input/touch/index.html

**Contents:**
- Module touch Copy item path
- Structs§
- Enums§
- Functions§

The touch input functionality.

---

## Enum Ime Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.Ime.html

**Contents:**
- Enum Ime Copy item path
- Variants§
  - Preedit
    - Fields
  - Commit
    - Fields
  - Enabled
    - Fields
  - Disabled
    - Fields

An Input Method Editor event.

This event is the translated version of the WindowEvent::Ime from the winit crate.

It is only sent if IME was enabled on the window with Window::ime_enabled.

Notifies when a new composing text should be set at the cursor position.

Window that received the event.

Cursor begin and end position.

None indicated the cursor should be hidden

Notifies when text should be inserted into the editor widget.

Window that received the event.

Notifies when the IME was enabled.

After this event, you will receive events Ime::Preedit and Ime::Commit.

Window that received the event.

Notifies when the IME was disabled.

Window that received the event.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum Ime {
    Preedit {
        window: Entity,
        value: String,
        cursor: Option<(usize, usize)>,
    },
    Commit {
        window: Entity,
        value: String,
    },
    Enabled {
        window: Entity,
    },
    Disabled {
        window: Entity,
    },
}
```

---

## Module gamepad Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input/gamepad/index.html

**Contents:**
- Module gamepad Copy item path
- Structs§
- Enums§
- Functions§

The gamepad input functionality.

---

## Enum ButtonState Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input/enum.ButtonState.html

**Contents:**
- Enum ButtonState Copy item path
- Variants§
  - Pressed
  - Released
- Implementations§
  - impl ButtonState
    - pub fn is_pressed(&self) -> bool
      - Examples found in repository?
- Trait Implementations§
  - impl Clone for ButtonState

The current “press” state of an element

The button is pressed.

The button is not pressed.

Is this button pressed?

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ButtonState {
    Pressed,
    Released,
}
```

Example 2 (javascript):
```javascript
16fn print_char_event_system(mut keyboard_inputs: MessageReader<KeyboardInput>) {
17    for event in keyboard_inputs.read() {
18        // Only check for characters when the key is pressed.
19        if !event.state.is_pressed() {
20            continue;
21        }
22        if let Key::Character(character) = &event.logical_key {
23            info!("{:?}: '{}'", event, character);
24        }
25    }
26}
```

Example 3 (javascript):
```javascript
191fn handle_mouse(
192    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
193    mut mouse_button_inputs: MessageReader<MouseButtonInput>,
194    mut camera_transform: Single<&mut Transform, With<Camera>>,
195    mut mouse_pressed: ResMut<MousePressed>,
196) {
197    // Store left-pressed state in the MousePressed resource
198    for mouse_button_input in mouse_button_inputs.read() {
199        if mouse_button_input.button != MouseButton::Left {
200            continue;
201        }
202        *mouse_pressed = MousePressed(mouse_button_input.state.is_pressed());
203    }
204
205    // 
...
```

Example 4 (javascript):
```javascript
221fn handle_mouse(
222    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
223    mut mouse_button_inputs: MessageReader<MouseButtonInput>,
224    mut camera_transform: Single<&mut Transform, With<Camera>>,
225    mut mouse_pressed: ResMut<MousePressed>,
226) {
227    // Store left-pressed state in the MousePressed resource
228    for mouse_button_input in mouse_button_inputs.read() {
229        if mouse_button_input.button != MouseButton::Left {
230            continue;
231        }
232        *mouse_pressed = MousePressed(mouse_button_input.state.is_pressed());
233    }
234
235    // 
...
```

---

## Enum WindowEvent Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.WindowEvent.html

**Contents:**
- Enum WindowEvent Copy item path
- Variants§
  - AppLifecycle(AppLifecycle)
  - CursorEntered(CursorEntered)
  - CursorLeft(CursorLeft)
  - CursorMoved(CursorMoved)
  - FileDragAndDrop(FileDragAndDrop)
  - Ime(Ime)
  - RequestRedraw(RequestRedraw)
  - WindowBackendScaleFactorChanged(WindowBackendScaleFactorChanged)

Wraps all bevy_window and bevy_input events in a common enum.

Read these events with MessageReader<WindowEvent> if you need to access window events in the order they were received from the operating system. Otherwise, the event types are individually readable with MessageReader<E> (e.g. MessageReader<KeyboardInput>).

An application lifecycle event.

The user’s cursor has entered a window.

The user’s cursor has left a window.

The user’s cursor has moved inside a window.

A file drag and drop event.

An Input Method Editor event.

A redraw of all of the application’s windows has been requested.

The window’s OS-reported scale factor has changed.

The OS has requested that a window be closed.

A new window has been created.

A window has been destroyed by the underlying windowing system.

A window has received or lost focus.

A window has been moved.

A window has started or stopped being occluded.

A window’s logical size has changed.

A window’s scale factor has changed.

Sent for windows that are using the system theme when the system theme changes.

The state of a mouse button has changed.

The physical position of a pointing device has changed.

The mouse wheel has moved.

A two finger pinch gesture.

A two finger rotation gesture.

A double tap gesture.

A touch input state change.

Sent when focus has been lost for all Bevy windows.

Used to clear pressed key state.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum WindowEvent {
Show 27 variants    AppLifecycle(AppLifecycle),
    CursorEntered(CursorEntered),
    CursorLeft(CursorLeft),
    CursorMoved(CursorMoved),
    FileDragAndDrop(FileDragAndDrop),
    Ime(Ime),
    RequestRedraw(RequestRedraw),
    WindowBackendScaleFactorChanged(WindowBackendScaleFactorChanged),
    WindowCloseRequested(WindowCloseRequested),
    WindowCreated(WindowCreated),
    WindowDestroyed(WindowDestroyed),
    WindowFocused(WindowFocused),
    WindowMoved(WindowMoved),
    WindowOccluded(WindowOccluded),
    WindowResized(WindowResized),
    WindowScaleFactorChange
...
```

---

## Module gestures Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input/gestures/index.html

**Contents:**
- Module gestures Copy item path
- Structs§

Gestures functionality, from touchscreens and touchpads.

---

## Struct EnabledButtons Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.EnabledButtons.html

**Contents:**
- Struct EnabledButtons Copy item path
  - §Platform-specific
- Fields§
- Trait Implementations§
  - impl Clone for EnabledButtons
    - fn clone(&self) -> EnabledButtons
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for EnabledButtons
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for EnabledButtons

Specifies which Window control buttons should be enabled.

iOS, Android, and the Web do not have window control buttons.

On some Linux environments these values have no effect.

Enables the functionality of the minimize button.

Enables the functionality of the maximize button.

macOS note: When Window resizable member is set to false the maximize button will be disabled regardless of this value. Additionally, when resizable is set to true the window will be maximized when its bar is double-clicked regardless of whether the maximize button is enabled or not.

Enables the functionality of the close button.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct EnabledButtons {
    pub minimize: bool,
    pub maximize: bool,
    pub close: bool,
}
```

---

## Trait AnimationEvent Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/trait.AnimationEvent.html

**Contents:**
- Trait AnimationEvent Copy item path
- Dyn Compatibility§
- Implementors§

An Event that an AnimationPlayer can trigger when playing an AnimationClip. See AnimationClip::add_event.

This trait can be derived.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait AnimationEvent: Clone + for<'a> Event<Trigger<'a> = AnimationEventTrigger> { }
```

---

## Module directional_navigation Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input_focus/directional_navigation/index.html

**Contents:**
- Module directional_navigation Copy item path
- Structs§
- Enums§

A navigation framework for moving between focusable elements based on directional input.

While virtual cursors are a common way to navigate UIs with a gamepad (or arrow keys!), they are generally both slow and frustrating to use. Instead, directional inputs should provide a direct way to snap between focusable elements.

Like the rest of this crate, the InputFocus resource is manipulated to track the current focus.

Navigating between focusable entities (commonly UI nodes) is done by passing a CompassOctant into the navigate method from the DirectionalNavigation system parameter.

Under the hood, the DirectionalNavigationMap stores a directed graph of focusable entities. Each entity can have up to 8 neighbors, one for each CompassOctant, balancing flexibility and required precision. For now, this graph must be built manually, but in the future, it could be generated automatically.

---

## Struct CursorMoved Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.CursorMoved.html

**Contents:**
- Struct CursorMoved Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for CursorMoved
    - fn clone(&self) -> CursorMoved
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for CursorMoved
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for CursorMoved
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<CursorMoved, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,

An event reporting that the mouse cursor has moved inside a window.

The event is sent only if the cursor is over one of the application’s windows. It is the translated version of WindowEvent::CursorMoved from the winit crate with the addition of delta.

Not to be confused with the MouseMotion event from bevy_input.

Because the range of data is limited by the window area and it may have been transformed by the OS to implement certain effects like acceleration, you should not use it for non-cursor-like behavior such as 3D camera control. Please see MouseMotion instead.

Window that the cursor moved inside.

The cursor position in logical pixels.

The change in the position of the cursor since the last event was sent. This value is None if the cursor was outside the window area during the last frame.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CursorMoved {
    pub window: Entity,
    pub position: Vec2,
    pub delta: Option<Vec2>,
}
```

---

## Function handle_internal_asset_events Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/fn.handle_internal_asset_events.html

**Contents:**
- Function handle_internal_asset_events Copy item path

A system that manages internal AssetServer events, such as finalizing asset loads.

**Examples:**

Example 1 (unknown):
```unknown
pub fn handle_internal_asset_events(world: &mut World)
```

---

## Module keyboard Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input/keyboard/index.html

**Contents:**
- Module keyboard Copy item path
- Structs§
- Enums§
- Functions§

The keyboard input functionality.

---

## Module common_conditions Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input/common_conditions/index.html

**Contents:**
- Module common_conditions Copy item path
- Functions§

Common run conditions

---

## Module mouse Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input/mouse/index.html

**Contents:**
- Module mouse Copy item path
- Structs§
- Enums§
- Functions§

The mouse input functionality.

---

## Enum AssetEvent Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.AssetEvent.html

**Contents:**
- Enum AssetEvent Copy item path
- Variants§
  - Added
    - Fields
  - Modified
    - Fields
  - Removed
    - Fields
  - Unused
    - Fields

Messages that occur for a specific loaded Asset, such as “value changed” events and “dependency” events.

Emitted whenever an Asset is added.

Emitted whenever an Asset value is modified.

Emitted whenever an Asset is removed.

Emitted when the last Handle::Strong of an Asset is dropped.

Emitted whenever an Asset has been fully loaded (including its dependencies and all “recursive dependencies”).

Returns true if this event is AssetEvent::LoadedWithDependencies and matches the given id.

Returns true if this event is AssetEvent::Added and matches the given id.

Returns true if this event is AssetEvent::Modified and matches the given id.

Returns true if this event is AssetEvent::Removed and matches the given id.

Returns true if this event is AssetEvent::Unused and matches the given id.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum AssetEvent<A>where
    A: Asset,{
    Added {
        id: AssetId<A>,
    },
    Modified {
        id: AssetId<A>,
    },
    Removed {
        id: AssetId<A>,
    },
    Unused {
        id: AssetId<A>,
    },
    LoadedWithDependencies {
        id: AssetId<A>,
    },
}
```

Example 2 (javascript):
```javascript
111fn update_tileset_image(
112    chunk_query: Single<&TilemapChunk>,
113    mut events: MessageReader<AssetEvent<Image>>,
114    mut images: ResMut<Assets<Image>>,
115) {
116    let chunk = *chunk_query;
117    for event in events.read() {
118        if event.is_loaded_with_dependencies(chunk.tileset.id()) {
119            let image = images.get_mut(&chunk.tileset).unwrap();
120            image.reinterpret_stacked_2d_as_array(4);
121        }
122    }
123}
```

Example 3 (unknown):
```unknown
37fn check_textures(
38    mut next_state: ResMut<NextState<AppState>>,
39    rpg_sprite_folder: Res<RpgSpriteFolder>,
40    mut events: MessageReader<AssetEvent<LoadedFolder>>,
41) {
42    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
43    for event in events.read() {
44        if event.is_loaded_with_dependencies(&rpg_sprite_folder.0) {
45            next_state.set(AppState::Finished);
46        }
47    }
48}
```

---

## Type Alias AssetEvents Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/type.AssetEvents.html

**Contents:**
- Type Alias AssetEvents Copy item path
- Aliased Type§

Deprecated alias for AssetEventSystems.

**Examples:**

Example 1 (unknown):
```unknown
pub type AssetEvents = AssetEventSystems;
```

Example 2 (unknown):
```unknown
pub struct AssetEvents;
```

---

## Struct Axis Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input/struct.Axis.html

**Contents:**
- Struct Axis Copy item path
- Implementations§
  - impl<T> Axis<T>where T: Copy + Eq + Hash,
    - pub const MIN: f32 = -1f32
    - pub const MAX: f32 = 1f32
    - pub fn set( &mut self, input_device: impl Into<T>, position_data: f32, ) -> Option<f32>
    - pub fn get(&self, input_device: impl Into<T>) -> Option<f32>
    - pub fn get_unclamped(&self, input_device: impl Into<T>) -> Option<f32>
    - pub fn remove(&mut self, input_device: impl Into<T>) -> Option<f32>
    - pub fn all_axes(&self) -> impl Iterator<Item = &T>

Stores the position data of the input devices of type T.

The values are stored as f32s, using Axis::set. Use Axis::get to retrieve the value clamped between Axis::MIN and Axis::MAX inclusive, or unclamped using Axis::get_unclamped.

The smallest possible axis value.

The largest possible axis value.

Sets the position data of the input_device to position_data.

Returns the position data of the provided input_device.

This will be clamped between Axis::MIN and Axis::MAX inclusive.

Returns the unclamped position data of the provided input_device.

This value may be outside the Axis::MIN and Axis::MAX range.

Use for things like camera zoom, where you want devices like mouse wheels to be able to exceed the normal range. If being able to move faster on one input device than another would give an unfair advantage, you should likely use Axis::get instead.

Removes the position data of the input_device, returning the position data if the input device was previously set.

Returns an iterator over all axes.

Returns an iterator over all axes and their values.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Axis<T> { /* private fields */ }
```

---

## Crate input Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input/index.html

**Contents:**
- Crate input Copy item path
- §Supported input devices
- Modules§
- Structs§
- Enums§
- Type Aliases§

Input functionality for the Bevy game engine.

bevy currently supports keyboard, mouse, gamepad, and touch inputs.

---

## Struct AnimationEventTrigger Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/struct.AnimationEventTrigger.html

**Contents:**
- Struct AnimationEventTrigger Copy item path
- Fields§
- Trait Implementations§
  - impl<E> Trigger<E> for AnimationEventTriggerwhere E: AnimationEvent<Trigger<'a> = AnimationEventTrigger> + for<'a> Event,
    - unsafe fn trigger( &mut self, world: DeferredWorld<'_>, observers: &CachedObservers, trigger_context: &TriggerContext, event: &mut E, )
- Auto Trait Implementations§
  - impl Freeze for AnimationEventTrigger
  - impl RefUnwindSafe for AnimationEventTrigger
  - impl Send for AnimationEventTrigger
  - impl Sync for AnimationEventTrigger

The Trigger implementation for AnimationEvent. This passes in the AnimationPlayer context, and uses that to run any observers that target that entity.

The AnimationPlayer where this AnimationEvent occurred.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AnimationEventTrigger {
    pub animation_player: Entity,
}
```

---

## Struct InputPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input/struct.InputPlugin.html

**Contents:**
- Struct InputPlugin Copy item path
- Trait Implementations§
  - impl Default for InputPlugin
    - fn default() -> InputPlugin
  - impl Plugin for InputPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str

Adds keyboard and mouse input to an App

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct InputPlugin;
```

---
