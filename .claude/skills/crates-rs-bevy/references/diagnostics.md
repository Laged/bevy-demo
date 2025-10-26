# Crates-Rs-Bevy - Diagnostics

**Pages:** 14

---

## Function update_frame_count Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/fn.update_frame_count.html

**Contents:**
- Function update_frame_count Copy item path

A system used to increment FrameCount with wrapping addition.

See FrameCount for more details.

**Examples:**

Example 1 (unknown):
```unknown
pub fn update_frame_count(frame_count: ResMut<'_, FrameCount>)
```

---

## Struct Diagnostics Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/struct.Diagnostics.html

**Contents:**
- Struct Diagnostics Copy item path
- Implementations§
  - impl<'w, 's> Diagnostics<'w, 's>
    - pub fn add_measurement<F>(&mut self, path: &DiagnosticPath, value: F)where F: FnOnce() -> f64,
      - Examples found in repository?
- Trait Implementations§
  - impl SystemParam for Diagnostics<'_, '_>
    - type State = FetchState
    - type Item<'w, 's> = Diagnostics<'w, 's>
    - fn init_state(world: &mut World) -> <Diagnostics<'_, '_> as SystemParam>::State

Record new DiagnosticMeasurement’s.

Add a measurement to an enabled Diagnostic. The measurement is passed as a function so that it will be evaluated only if the Diagnostic is enabled. This can be useful if the value is costly to calculate.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Diagnostics<'w, 's> { /* private fields */ }
```

Example 2 (unknown):
```unknown
27fn my_system(mut diagnostics: Diagnostics) {
28    // Add a measurement of 10.0 for our diagnostic each time this system runs.
29    diagnostics.add_measurement(&SYSTEM_ITERATION_COUNT, || 10.0);
30}
```

---

## Macro debug_span Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/macro.debug_span.html

**Contents:**
- Macro debug_span Copy item path
- §Examples

Constructs a span at the debug level.

Fields and attributes are set using the same syntax as the span! macro.

See the top-level documentation for details on the syntax accepted by this macro.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! debug_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, parent: $parent:expr, $name:expr) => { ... };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (parent: $parent:expr, $name:expr) => { ... };
    (target: $target:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, $name:expr) => { ... };
    ($name:expr, $($field:tt)*) => { ... };
    ($name:expr) => { ... };
}
```

Example 2 (unknown):
```unknown
debug_span!("my_span");
// is equivalent to:
span!(Level::DEBUG, "my_span");
```

Example 3 (javascript):
```javascript
let span = debug_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
```

---

## Macro debug_once Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/macro.debug_once.html

**Contents:**
- Macro debug_once Copy item path

Call debug! once per call site.

Useful for logging within systems which are called every frame.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! debug_once {
    ($($arg:tt)+) => { ... };
}
```

---

## Struct DiagnosticPath Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/struct.DiagnosticPath.html

**Contents:**
- Struct DiagnosticPath Copy item path
- Implementations§
  - impl DiagnosticPath
    - pub const fn const_new(path: &'static str) -> DiagnosticPath
      - Examples found in repository?
    - pub fn new(path: impl Into<Cow<'static, str>>) -> DiagnosticPath
      - Examples found in repository?
    - pub fn from_components<'a>( components: impl IntoIterator<Item = &'a str>, ) -> DiagnosticPath
    - pub fn as_str(&self) -> &str
    - pub fn components(&self) -> impl Iterator<Item = &str>

Unique diagnostic path, separated by /.

Create a new DiagnosticPath. Usable in const contexts.

Note: path is not validated, so make sure it follows all the requirements.

Create a new DiagnosticPath from the specified string.

Create a new DiagnosticPath from an iterator over components.

Returns full path, joined by /

Returns an iterator over path components.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DiagnosticPath { /* private fields */ }
```

Example 2 (javascript):
```javascript
25const SYSTEM_ITERATION_COUNT: DiagnosticPath = DiagnosticPath::const_new("system_iteration_count");
```

Example 3 (javascript):
```javascript
80fn stress_test(num_entities: u32, num_components: u32, num_systems: u32) {
81    let mut rng = ChaCha8Rng::seed_from_u64(42);
82    let mut app = App::default();
83    let world = app.world_mut();
84
85    // register a bunch of components
86    let component_ids: Vec<ComponentId> = (1..=num_components)
87        .map(|i| {
88            world.register_component_with_descriptor(
89                // SAFETY:
90                // * We don't implement a drop function
91                // * u8 is Sync and Send
92                unsafe {
93                    ComponentDescriptor::new_with_layout(
...
```

---

## Module picking_debug Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/dev_tools/picking_debug/index.html

**Contents:**
- Module picking_debug Copy item path
- Structs§
- Enums§
- Functions§

Text and on-screen debugging tools

---

## Struct DiagnosticsStore Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/struct.DiagnosticsStore.html

**Contents:**
- Struct DiagnosticsStore Copy item path
- Implementations§
  - impl DiagnosticsStore
    - pub fn add(&mut self, diagnostic: Diagnostic)
    - pub fn get(&self, path: &DiagnosticPath) -> Option<&Diagnostic>
      - Examples found in repository?
    - pub fn get_mut(&mut self, path: &DiagnosticPath) -> Option<&mut Diagnostic>
    - pub fn get_measurement( &self, path: &DiagnosticPath, ) -> Option<&DiagnosticMeasurement>
    - pub fn iter(&self) -> impl Iterator<Item = &Diagnostic>
    - pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Diagnostic>

A collection of Diagnostics.

Add a new Diagnostic.

If possible, prefer calling App::register_diagnostic.

Get the DiagnosticMeasurement with the given DiagnosticPath, if it exists.

Mutably get the DiagnosticMeasurement with the given DiagnosticPath, if it exists.

Get the latest DiagnosticMeasurement from an enabled Diagnostic.

Return an iterator over all Diagnostics.

Return an iterator over all Diagnostics, by mutable reference.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DiagnosticsStore { /* private fields */ }
```

Example 2 (javascript):
```javascript
133fn text_update_system(
134    diagnostics: Res<DiagnosticsStore>,
135    mut query: Query<&mut TextSpan, With<FpsText>>,
136) {
137    for mut span in &mut query {
138        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
139            && let Some(value) = fps.smoothed()
140        {
141            // Update the value of the second section
142            **span = format!("{value:.2}");
143        }
144    }
145}
```

Example 3 (javascript):
```javascript
99fn ui_system(mut text: Single<&mut Text>, config: Res<Config>, diag: Res<DiagnosticsStore>) {
100    let Some(fps) = diag
101        .get(&FrameTimeDiagnosticsPlugin::FPS)
102        .and_then(Diagnostic::smoothed)
103    else {
104        return;
105    };
106
107    text.0 = format!(
108        "Line count: {}\n\
109        FPS: {:.0}\n\n\
110        Controls:\n\
111        Up/Down: Raise or lower the line count.\n\
112        Spacebar: Toggle fancy mode.",
113        config.line_count, fps,
114    );
115}
```

Example 4 (javascript):
```javascript
542fn counter_system(
543    diagnostics: Res<DiagnosticsStore>,
544    counter: Res<BevyCounter>,
545    query: Single<Entity, With<StatsText>>,
546    mut writer: TextUiWriter,
547) {
548    let text = *query;
549
550    if counter.is_changed() {
551        *writer.text(text, 2) = counter.count.to_string();
552    }
553
554    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
555        if let Some(raw) = fps.value() {
556            *writer.text(text, 4) = format!("{raw:.2}");
557        }
558        if let Some(sma) = fps.average() {
559            *writer.text(text, 6
...
```

---

## Trait RegisterDiagnostic Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/trait.RegisterDiagnostic.html

**Contents:**
- Trait RegisterDiagnostic Copy item path
- Required Methods§
    - fn register_diagnostic(&mut self, diagnostic: Diagnostic) -> &mut Self
- Dyn Compatibility§
- Implementors§
  - impl RegisterDiagnostic for App
  - impl RegisterDiagnostic for SubApp

Extend App with new register_diagnostic function.

Register a new Diagnostic with an App.

Will initialize a DiagnosticsStore if it doesn’t exist.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait RegisterDiagnostic {
    // Required method
    fn register_diagnostic(&mut self, diagnostic: Diagnostic) -> &mut Self;
}
```

Example 2 (javascript):
```javascript
use bevy_app::App;
use bevy_diagnostic::{Diagnostic, DiagnosticsPlugin, DiagnosticPath, RegisterDiagnostic};

const UNIQUE_DIAG_PATH: DiagnosticPath = DiagnosticPath::const_new("foo/bar");

App::new()
    .register_diagnostic(Diagnostic::new(UNIQUE_DIAG_PATH))
    .add_plugins(DiagnosticsPlugin)
    .run();
```

---

## Struct DiagnosticMeasurement Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/struct.DiagnosticMeasurement.html

**Contents:**
- Struct DiagnosticMeasurement Copy item path
- Fields§
- Trait Implementations§
  - impl Debug for DiagnosticMeasurement
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
- Auto Trait Implementations§
  - impl Freeze for DiagnosticMeasurement
  - impl RefUnwindSafe for DiagnosticMeasurement
  - impl Send for DiagnosticMeasurement
  - impl Sync for DiagnosticMeasurement

A single measurement of a Diagnostic.

When this measurement was taken.

Value of the measurement.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DiagnosticMeasurement {
    pub time: Instant,
    pub value: f64,
}
```

---

## Module fps_overlay Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/dev_tools/fps_overlay/index.html

**Contents:**
- Module fps_overlay Copy item path
- Structs§
- Constants§

Module containing logic for FPS overlay.

---

## Struct FrameCount Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/struct.FrameCount.html

**Contents:**
- Struct FrameCount Copy item path
- §Overflows
- Tuple Fields§
- Trait Implementations§
  - impl Clone for FrameCount
    - fn clone(&self) -> FrameCount
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for FrameCount
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for FrameCount

Maintains a count of frames rendered since the start of the application.

FrameCount is incremented during Last, providing predictable behavior: it will be 0 during the first update, 1 during the next, and so forth.

FrameCount will wrap to 0 after exceeding u32::MAX. Within reasonable assumptions, one may exploit wrapping arithmetic to determine the number of frames that have elapsed between two observations – see u32::wrapping_sub().

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct FrameCount(pub u32);
```

---

## Macro debug Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/macro.debug.html

**Contents:**
- Macro debug Copy item path
- §Examples

Constructs an event at the debug level.

This functions similarly to the event! macro. See the top-level documentation for details on the syntax accepted by this macro.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! debug {
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, 
...
```

Example 2 (javascript):
```javascript
use tracing::debug;

let pos = Position { x: 3.234, y: -1.223 };

debug!(?pos.x, ?pos.y);
debug!(target: "app_events", position = ?pos, "New position");
debug!(name: "completed", position = ?pos);
```

---

## Struct Diagnostic Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/struct.Diagnostic.html

**Contents:**
- Struct Diagnostic Copy item path
- Fields§
- Implementations§
  - impl Diagnostic
    - pub fn add_measurement(&mut self, measurement: DiagnosticMeasurement)
    - pub fn new(path: DiagnosticPath) -> Diagnostic
      - Examples found in repository?
    - pub fn with_max_history_length(self, max_history_length: usize) -> Diagnostic
    - pub fn with_suffix(self, suffix: impl Into<Cow<'static, str>>) -> Diagnostic
      - Examples found in repository?

A timeline of DiagnosticMeasurements of a specific type. Diagnostic examples: frames per second, CPU usage, network latency

Suffix to use when logging measurements for this Diagnostic, for example to show units.

Disabled Diagnostics are not measured or logged.

Add a new value as a DiagnosticMeasurement.

Create a new diagnostic with the given path.

Set the maximum history length.

Add a suffix to use when logging the value, can be used to show a unit.

The smoothing factor used for the exponential smoothing used for smoothed.

If measurements come in less frequently than smoothing_factor seconds apart, no smoothing will be applied. As measurements come in more frequently, the smoothing takes a greater effect such that it takes approximately smoothing_factor seconds for 83% of an instantaneous change in measurement to e reflected in the smoothed value.

A smoothing factor of 0.0 will effectively disable smoothing.

Get the DiagnosticPath that identifies this Diagnostic.

Get the latest measurement from this diagnostic.

Get the latest value from this diagnostic.

Return the simple moving average of this diagnostic’s recent values. N.B. this a cheap operation as the sum is cached.

Return the exponential moving average of this diagnostic.

This is by default tuned to behave reasonably well for a typical measurement that changes every frame such as frametime. This can be adjusted using with_smoothing_factor.

Return the number of elements for this diagnostic.

Return the duration between the oldest and most recent values for this diagnostic.

Return the maximum number of elements for this diagnostic.

All measured values from this Diagnostic, up to the configured maximum history length.

All measurements from this Diagnostic, up to the configured maximum history length.

Clear the history of this diagnostic.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Diagnostic {
    pub suffix: Cow<'static, str>,
    pub is_enabled: bool,
    /* private fields */
}
```

Example 2 (unknown):
```unknown
10fn main() {
11    App::new()
12        .add_plugins((
13            DefaultPlugins,
14            // The "print diagnostics" plugin is optional.
15            // It just visualizes our diagnostics in the console.
16            LogDiagnosticsPlugin::default(),
17        ))
18        // Diagnostics must be initialized before measurements can be added.
19        .register_diagnostic(Diagnostic::new(SYSTEM_ITERATION_COUNT).with_suffix(" iterations"))
20        .add_systems(Update, my_system)
21        .run();
22}
```

Example 3 (unknown):
```unknown
10fn main() {
11    App::new()
12        .add_plugins((
13            DefaultPlugins,
14            // The "print diagnostics" plugin is optional.
15            // It just visualizes our diagnostics in the console.
16            LogDiagnosticsPlugin::default(),
17        ))
18        // Diagnostics must be initialized before measurements can be added.
19        .register_diagnostic(Diagnostic::new(SYSTEM_ITERATION_COUNT).with_suffix(" iterations"))
20        .add_systems(Update, my_system)
21        .run();
22}
```

Example 4 (unknown):
```unknown
25fn toggle(mut store: ResMut<DiagnosticsStore>) {
26    for diag in store.iter_mut() {
27        info!("toggling diagnostic {}", diag.path());
28        diag.is_enabled = !diag.is_enabled;
29    }
30}
```

---

## Crate diagnostic Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/index.html

**Contents:**
- Crate diagnostic Copy item path
- Structs§
- Constants§
- Traits§
- Functions§

This crate provides a straightforward solution for integrating diagnostics in the Bevy game engine. It allows users to easily add diagnostic functionality to their Bevy applications, enhancing their ability to monitor and optimize their game’s.

---
