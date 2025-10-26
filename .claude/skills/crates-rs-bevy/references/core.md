# Crates-Rs-Bevy - Core

**Pages:** 43

---

## Enum RecursiveDependencyLoadState Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.RecursiveDependencyLoadState.html

**Contents:**
- Enum RecursiveDependencyLoadState Copy item path
- Variants§
  - NotLoaded
  - Loading
  - Loaded
  - Failed(Arc<AssetLoadError>)
- Implementations§
  - impl RecursiveDependencyLoadState
    - pub fn is_loading(&self) -> bool
    - pub fn is_loaded(&self) -> bool

The recursive load state of an asset’s dependencies.

The asset has not started loading yet

Dependencies in this asset’s dependency tree are still loading

Dependencies in this asset’s dependency tree have all loaded

One or more dependencies have failed to load in this asset’s dependency tree. The underlying AssetLoadError is referenced by Arc clones in all related LoadStates and DependencyLoadStates in the asset’s dependency tree.

Returns true if this instance is RecursiveDependencyLoadState::Loading

Returns true if this instance is RecursiveDependencyLoadState::Loaded

Returns true if this instance is RecursiveDependencyLoadState::Failed

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum RecursiveDependencyLoadState {
    NotLoaded,
    Loading,
    Loaded,
    Failed(Arc<AssetLoadError>),
}
```

Example 2 (unknown):
```unknown
196fn update_loading_data(
197    mut loading_data: ResMut<LoadingData>,
198    mut loading_state: ResMut<LoadingState>,
199    asset_server: Res<AssetServer>,
200    pipelines_ready: Res<PipelinesReady>,
201) {
202    if !loading_data.loading_assets.is_empty() || !pipelines_ready.0 {
203        // If we are still loading assets / pipelines are not fully compiled,
204        // we reset the confirmation frame count.
205        loading_data.confirmation_frames_count = 0;
206
207        loading_data.loading_assets.retain(|asset| {
208            asset_server
209                .get_recursive_dep
...
```

---

## Struct FrameTimeDiagnosticsPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/struct.FrameTimeDiagnosticsPlugin.html

**Contents:**
- Struct FrameTimeDiagnosticsPlugin Copy item path
- §See also
- Fields§
- Implementations§
  - impl FrameTimeDiagnosticsPlugin
    - pub fn new(max_history_length: usize) -> FrameTimeDiagnosticsPlugin
  - impl FrameTimeDiagnosticsPlugin
    - pub const FPS: DiagnosticPath
    - pub const FRAME_COUNT: DiagnosticPath
    - pub const FRAME_TIME: DiagnosticPath

Adds “frame time” diagnostic to an App, specifically “frame time”, “fps” and “frame count”

LogDiagnosticsPlugin to output diagnostics to the console.

The total number of values to keep for averaging.

The smoothing factor for the exponential moving average. Usually 2.0 / (history_length + 1.0).

Creates a new FrameTimeDiagnosticsPlugin with the specified max_history_length and a reasonable smoothing_factor.

Total frames since application start.

Updates frame count, frame time and fps measurements.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct FrameTimeDiagnosticsPlugin {
    pub max_history_length: usize,
    pub smoothing_factor: f64,
}
```

---

## Struct DevToolsPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/dev_tools/struct.DevToolsPlugin.html

**Contents:**
- Struct DevToolsPlugin Copy item path
- Trait Implementations§
  - impl Default for DevToolsPlugin
    - fn default() -> DevToolsPlugin
  - impl Plugin for DevToolsPlugin
    - fn build(&self, _app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str

Enables developer tools in an App. This plugin is added automatically with bevy_dev_tools feature.

Warning: It is not recommended to enable this in final shipped games or applications. Dev tools provide a high level of access to the internals of your application, and may interfere with ordinary use and gameplay.

To enable developer tools, you can either:

cargo run --features bevy/bevy_dev_tools

features = ["bevy_dev_tools"]

Note: The third method is not recommended, as it requires you to remove the feature before creating a build for release to the public.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DevToolsPlugin;
```

Example 2 (toml):
```toml
[feature]
dev_mode = ["bevy/bevy_dev_tools", "other_dev_tools"]
```

---

## Trait FreelyMutableState Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/state/state/trait.FreelyMutableState.html

**Contents:**
- Trait FreelyMutableState Copy item path
- Provided Methods§
    - fn register_state(schedule: &mut Schedule)
- Dyn Compatibility§
- Implementors§

This trait allows a state to be mutated directly using the NextState<S> resource.

While ordinary states are freely mutable (and implement this trait as part of their derive macro), computed states are not: instead, they can only change when the states that drive them do.

This function registers all the necessary systems to apply state changes and run transition schedules

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait FreelyMutableState: States {
    // Provided method
    fn register_state(schedule: &mut Schedule) { ... }
}
```

---

## Struct ClusteredDecalPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ClusteredDecalPlugin.html

**Contents:**
- Struct ClusteredDecalPlugin Copy item path
- Trait Implementations§
  - impl Plugin for ClusteredDecalPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str
    - fn is_unique(&self) -> bool
- Auto Trait Implementations§

A plugin that adds support for clustered decals.

In environments where bindless textures aren’t available, clustered decals can still be added to a scene, but they won’t project any decals.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ClusteredDecalPlugin;
```

---

## Struct DiagnosticsPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/struct.DiagnosticsPlugin.html

**Contents:**
- Struct DiagnosticsPlugin Copy item path
- Trait Implementations§
  - impl Default for DiagnosticsPlugin
    - fn default() -> DiagnosticsPlugin
  - impl Plugin for DiagnosticsPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str

Adds core diagnostics resources to an App.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DiagnosticsPlugin;
```

---

## Struct LogDiagnosticsPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/struct.LogDiagnosticsPlugin.html

**Contents:**
- Struct LogDiagnosticsPlugin Copy item path
- Fields§
- Implementations§
  - impl LogDiagnosticsPlugin
    - pub fn filtered(filter: HashSet<DiagnosticPath>) -> LogDiagnosticsPlugin
      - Examples found in repository?
- Trait Implementations§
  - impl Default for LogDiagnosticsPlugin
    - fn default() -> LogDiagnosticsPlugin
  - impl Plugin for LogDiagnosticsPlugin

An App Plugin that logs diagnostics to the console.

Diagnostics are collected by plugins such as FrameTimeDiagnosticsPlugin or can be provided by the user.

When no diagnostics are provided, this plugin does nothing.

If true then the Debug representation of each Diagnostic is logged. If false then a (smoothed) current value and historical average are logged.

Time to wait between logging diagnostics and logging them again.

If Some then only these diagnostics are logged.

Filter logging to only the paths in filter.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct LogDiagnosticsPlugin {
    pub debug: bool,
    pub wait_duration: Duration,
    pub filter: Option<HashSet<DiagnosticPath>>,
}
```

Example 2 (javascript):
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

## Module states Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/dev_tools/states/index.html

**Contents:**
- Module states Copy item path
- Functions§

Tools for debugging states.

---

## Struct DefaultPlugins Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/struct.DefaultPlugins.html

**Contents:**
- Struct DefaultPlugins Copy item path
- Trait Implementations§
  - impl PluginGroup for DefaultPlugins
    - fn build(self) -> PluginGroupBuilder
    - fn name() -> String
    - fn set<T>(self, plugin: T) -> PluginGroupBuilderwhere T: Plugin,
- Auto Trait Implementations§
  - impl Freeze for DefaultPlugins
  - impl RefUnwindSafe for DefaultPlugins
  - impl Send for DefaultPlugins

This plugin group will add all the default plugins for a Bevy application:

DefaultPlugins obeys Cargo feature flags. Users may exert control over this plugin group by disabling default-features in their Cargo.toml and enabling only those features that they wish to use.

DefaultPlugins contains all the plugins typically required to build a Bevy application which includes a window and presentation components. For the absolute minimum number of plugins needed to run a Bevy application, see MinimalPlugins.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DefaultPlugins;
```

---

## Module fxaa Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/anti_alias/fxaa/index.html

**Contents:**
- Module fxaa Copy item path
- Structs§
- Enums§
- Functions§

bevy::anti_aliasModule fxaa Copy item pathSource Structs§CameraFxaaPipelineFxaaA component for enabling Fast Approximate Anti-Aliasing (FXAA) for a bevy_camera::Camera.FxaaNodeFxaaPipelineFxaaPipelineKeyFxaaPluginAdds support for Fast Approximate Anti-Aliasing (FXAA)Enums§SensitivityFunctions§init_fxaa_pipelineprepare_fxaa_pipelines

---

## Struct LogDiagnosticsState Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/struct.LogDiagnosticsState.html

**Contents:**
- Struct LogDiagnosticsState Copy item path
- Implementations§
  - impl LogDiagnosticsState
    - pub fn set_timer_duration(&mut self, duration: Duration)
    - pub fn add_filter(&mut self, diagnostic_path: DiagnosticPath) -> bool
    - pub fn extend_filter(&mut self, iter: impl IntoIterator<Item = DiagnosticPath>)
      - Examples found in repository?
    - pub fn remove_filter(&mut self, diagnostic_path: &DiagnosticPath) -> bool
      - Examples found in repository?
    - pub fn clear_filter(&mut self)

State used by the LogDiagnosticsPlugin

Sets a new duration for the log timer

Add a filter to the log state, returning true if the DiagnosticPath was not present

Extends the filter of the log state with multiple DiagnosticPaths

Removes a filter from the log state, returning true if it was present

Clears the filters of the log state

Enables filtering with empty filters

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct LogDiagnosticsState { /* private fields */ }
```

Example 2 (unknown):
```unknown
164fn enable_filters(
165    log_state: &mut LogDiagnosticsState,
166    diagnostics: impl IntoIterator<Item = DiagnosticPath>,
167) {
168    log_state.extend_filter(diagnostics);
169}
```

Example 3 (unknown):
```unknown
171fn disable_filters(
172    log_state: &mut LogDiagnosticsState,
173    diagnostics: impl IntoIterator<Item = DiagnosticPath>,
174) {
175    for diagnostic in diagnostics {
176        log_state.remove_filter(&diagnostic);
177    }
178}
```

Example 4 (javascript):
```javascript
103fn filters_inputs(
104    keys: Res<ButtonInput<KeyCode>>,
105    mut status: ResMut<LogDiagnosticsStatus>,
106    mut filters: ResMut<LogDiagnosticsFilters>,
107    mut log_state: ResMut<LogDiagnosticsState>,
108) {
109    if keys.just_pressed(KeyCode::KeyQ) {
110        *status = match *status {
111            LogDiagnosticsStatus::Enabled => {
112                log_state.disable_filtering();
113                LogDiagnosticsStatus::Disabled
114            }
115            LogDiagnosticsStatus::Disabled => {
116                log_state.enable_filtering();
117                if filters.f
...
```

---

## Enum LoadState Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.LoadState.html

**Contents:**
- Enum LoadState Copy item path
- Variants§
  - NotLoaded
  - Loading
  - Loaded
  - Failed(Arc<AssetLoadError>)
- Implementations§
  - impl LoadState
    - pub fn is_loading(&self) -> bool
    - pub fn is_loaded(&self) -> bool

The load state of an asset.

The asset has not started loading yet

The asset is in the process of loading.

The asset has been loaded and has been added to the World

The asset failed to load. The underlying AssetLoadError is referenced by Arc clones in all related DependencyLoadStates and RecursiveDependencyLoadStates in the asset’s dependency tree.

Returns true if this instance is LoadState::Loading

Returns true if this instance is LoadState::Loaded

Returns true if this instance is LoadState::Failed

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum LoadState {
    NotLoaded,
    Loading,
    Loaded,
    Failed(Arc<AssetLoadError>),
}
```

Example 2 (javascript):
```javascript
127fn environment_map_load_finish(
128    mut commands: Commands,
129    asset_server: Res<AssetServer>,
130    environment_map: Single<&EnvironmentMapLight>,
131    label_entity: Option<Single<Entity, With<EnvironmentMapLabel>>>,
132) {
133    if asset_server
134        .load_state(&environment_map.diffuse_map)
135        .is_loaded()
136        && asset_server
137            .load_state(&environment_map.specular_map)
138            .is_loaded()
139    {
140        // Do not attempt to remove `label_entity` if it has already been removed.
141        if let Some(label_entity) = label_entity {

...
```

Example 3 (javascript):
```javascript
148fn asset_loaded(
149    asset_server: Res<AssetServer>,
150    mut images: ResMut<Assets<Image>>,
151    mut cubemap: ResMut<Cubemap>,
152    mut skyboxes: Query<&mut Skybox>,
153) {
154    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle).is_loaded() {
155        info!("Swapping to {}...", CUBEMAPS[cubemap.index].0);
156        let image = images.get_mut(&cubemap.image_handle).unwrap();
157        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
158        // so they appear as one texture. The following code reconfigures the t
...
```

Example 4 (javascript):
```javascript
48fn create_array_texture(
49    mut commands: Commands,
50    asset_server: Res<AssetServer>,
51    mut loading_texture: ResMut<LoadingTexture>,
52    mut images: ResMut<Assets<Image>>,
53    mut meshes: ResMut<Assets<Mesh>>,
54    mut materials: ResMut<Assets<ArrayTextureMaterial>>,
55) {
56    if loading_texture.is_loaded
57        || !asset_server
58            .load_state(loading_texture.handle.id())
59            .is_loaded()
60    {
61        return;
62    }
63    loading_texture.is_loaded = true;
64    let image = images.get_mut(&loading_texture.handle).unwrap();
65
66    // Create a n
...
```

---

## Struct FeathersPlugins Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/feathers/struct.FeathersPlugins.html

**Contents:**
- Struct FeathersPlugins Copy item path
- Trait Implementations§
  - impl PluginGroup for FeathersPlugins
    - fn build(self) -> PluginGroupBuilder
    - fn name() -> String
    - fn set<T>(self, plugin: T) -> PluginGroupBuilderwhere T: Plugin,
- Auto Trait Implementations§
  - impl Freeze for FeathersPlugins
  - impl RefUnwindSafe for FeathersPlugins
  - impl Send for FeathersPlugins

A plugin group that adds all dependencies for Feathers

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct FeathersPlugins;
```

---

## Crate state Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/state/index.html

**Contents:**
- Crate state Copy item path
- Modules§

In Bevy, states are app-wide interdependent, finite state machines that are generally used to model the large scale structure of your program: whether a game is paused, if the player is in combat, if assets are loaded and so on.

This module provides 3 distinct types of state, all of which implement the States trait:

Most of the utilities around state involve running systems during transitions between states, or determining whether to run certain systems, though they can be used more directly as well. This makes it easier to transition between menus, add loading screens, pause games, and more.

Specifically, Bevy provides the following utilities:

Bevy also provides (“state-scoped entities”)crate::state_scoped functionality for managing the lifetime of entities in the context of game states. This, especially in combination with system scheduling, enables a flexible and expressive way to manage spawning and despawning entities.

---

## Crate a11y Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/a11y/index.html

**Contents:**
- Crate a11y Copy item path
  - §Some notes on utility
  - §This crate no longer re-exports AccessKit
- Structs§
- Enums§
- Type Aliases§

Reusable accessibility primitives

This crate provides accessibility integration for the engine. It exposes the AccessibilityPlugin. This plugin integrates AccessKit, a Rust crate providing OS-agnostic accessibility primitives, with Bevy’s ECS.

While this crate defines useful types for accessibility, it does not actually power accessibility features in Bevy.

Instead, it helps other interfaces coordinate their approach to accessibility. Binary authors should add the AccessibilityPlugin, while library maintainers may use the AccessibilityRequested and ManageAccessibilityUpdates resources.

The AccessibilityNode component is useful in both cases. It helps describe an entity in terms of its accessibility factors through an AccessKit “node”.

Typical UI concepts, like buttons, checkboxes, and textboxes, are easily described by this component, though, technically, it can represent any kind of Bevy Entity.

As of Bevy version 0.15, the accesskit crate is no longer re-exported from this crate.1 If you need to use AccessKit yourself, you’ll have to add it as a separate dependency in your project’s Cargo.toml.

Make sure to use the same version of the accesskit crate as Bevy. Otherwise, you may experience errors similar to: “Perhaps two different versions of crate accesskit are being used?”

Some users were confused about AccessKit’s Node type, sometimes thinking it was Bevy UI’s primary way to define nodes!

For this reason, its re-export was removed by default. Users who need its types can instead manually depend on the accesskit crate. ↩

---

## Module deferred Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/core_pipeline/deferred/index.html

**Contents:**
- Module deferred Copy item path
- Modules§
- Structs§
- Constants§

bevy::core_pipelineModule deferred Copy item pathSource Modules§copy_lighting_idnodeStructs§AlphaMask3dDeferredAlpha mask phase of the 3D Deferred pass.Opaque3dDeferredOpaque phase of the 3D Deferred pass.Constants§DEFERRED_LIGHTING_PASS_ID_DEPTH_FORMATDEFERRED_LIGHTING_PASS_ID_FORMATDEFERRED_PREPASS_FORMAT

---

## Struct LightProbePlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.LightProbePlugin.html

**Contents:**
- Struct LightProbePlugin Copy item path
- Trait Implementations§
  - impl Plugin for LightProbePlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str
    - fn is_unique(&self) -> bool
- Auto Trait Implementations§

Adds support for light probes: cuboid bounding regions that apply global illumination to objects within them.

This also adds support for view environment maps: diffuse and specular cubemaps applied to all objects that a view renders.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct LightProbePlugin;
```

---

## Struct AccessibilityRequested Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/a11y/struct.AccessibilityRequested.html

**Contents:**
- Struct AccessibilityRequested Copy item path
- Implementations§
  - impl AccessibilityRequested
    - pub fn get(&self) -> bool
    - pub fn set(&self, value: bool)
- Trait Implementations§
  - impl Clone for AccessibilityRequested
    - fn clone(&self) -> AccessibilityRequested
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for AccessibilityRequested

Tracks whether an assistive technology has requested accessibility information.

This type is a Resource initialized by the AccessibilityPlugin. It may be useful if a third-party plugin needs to conditionally integrate with AccessKit.

In other words, this resource represents whether accessibility providers are “turned on” or “turned off” across an entire Bevy App.

By default, it is set to false, indicating that nothing has requested accessibility information yet.

Checks if any assistive technology has requested accessibility information.

If so, this method returns true, indicating that accessibility tree updates should be sent.

Sets the app’s preference for sending accessibility updates.

If the value argument is true, this method requests that the app, including both Bevy and third-party interfaces, provides updates to accessibility information.

Setting with false requests that the entire app stops providing these updates.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AccessibilityRequested(/* private fields */);
```

---

## Struct LightPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.LightPlugin.html

**Contents:**
- Struct LightPlugin Copy item path
- Trait Implementations§
  - impl Default for LightPlugin
    - fn default() -> LightPlugin
  - impl Plugin for LightPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct LightPlugin;
```

---

## Struct AntiAliasPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/anti_alias/struct.AntiAliasPlugin.html

**Contents:**
- Struct AntiAliasPlugin Copy item path
- Trait Implementations§
  - impl Default for AntiAliasPlugin
    - fn default() -> AntiAliasPlugin
  - impl Plugin for AntiAliasPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str

Adds fxaa, smaa, taa, contrast aware sharpening, and optional dlss support.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AntiAliasPlugin;
```

---

## Module blit Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/core_pipeline/blit/index.html

**Contents:**
- Module blit Copy item path
- Structs§
- Functions§

bevy::core_pipelineModule blit Copy item pathSource Structs§BlitPipelineBlitPipelineKeyBlitPluginAdds support for specialized “blit pipelines”, which can be used to write one texture to another.Functions§init_blit_pipeline

---

## Struct FeathersPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/feathers/struct.FeathersPlugin.html

**Contents:**
- Struct FeathersPlugin Copy item path
- Trait Implementations§
  - impl Plugin for FeathersPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str
    - fn is_unique(&self) -> bool
- Auto Trait Implementations§

Plugin which installs observers and systems for feathers themes, cursors, and all controls.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct FeathersPlugin;
```

---

## Struct AnimationPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/struct.AnimationPlugin.html

**Contents:**
- Struct AnimationPlugin Copy item path
- Trait Implementations§
  - impl Default for AnimationPlugin
    - fn default() -> AnimationPlugin
  - impl Plugin for AnimationPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str

Adds animation support to an app

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AnimationPlugin;
```

---

## Struct AnimationEvaluationState Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/struct.AnimationEvaluationState.html

**Contents:**
- Struct AnimationEvaluationState Copy item path
- Trait Implementations§
  - impl Default for AnimationEvaluationState
    - fn default() -> AnimationEvaluationState
- Auto Trait Implementations§
  - impl Freeze for AnimationEvaluationState
  - impl !RefUnwindSafe for AnimationEvaluationState
  - impl Send for AnimationEvaluationState
  - impl Sync for AnimationEvaluationState
  - impl Unpin for AnimationEvaluationState

Temporary data that the animate_targets system maintains.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AnimationEvaluationState { /* private fields */ }
```

---

## Struct ImagePlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.ImagePlugin.html

**Contents:**
- Struct ImagePlugin Copy item path
- Fields§
- Implementations§
  - impl ImagePlugin
    - pub fn default_linear() -> ImagePlugin
    - pub fn default_nearest() -> ImagePlugin
      - Examples found in repository?
- Trait Implementations§
  - impl Default for ImagePlugin
    - fn default() -> ImagePlugin

Adds the Image as an asset and makes sure that they are extracted and prepared for the GPU.

The default image sampler to use when ImageSampler is set to Default.

Creates image settings with linear sampling by default.

Creates image settings with nearest sampling by default.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ImagePlugin {
    pub default_sampler: ImageSamplerDescriptor,
}
```

Example 2 (unknown):
```unknown
6fn main() {
7    App::new()
8        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
9        .add_systems(Startup, setup)
10        .add_systems(Update, animate_sprite)
11        .run();
12}
```

Example 3 (unknown):
```unknown
7fn main() {
8    App::new()
9        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
10        .add_systems(Startup, (setup, setup_atlas))
11        .add_systems(Update, (move_sprite, animate_sprite))
12        .run();
13}
```

Example 4 (unknown):
```unknown
27fn main() {
28    App::new()
29        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
30        .add_systems(Startup, (setup_camera, setup_sprite, setup_mesh))
31        .add_systems(Update, (rotate, fit_canvas))
32        .run();
33}
```

---

## Module oit Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/core_pipeline/oit/index.html

**Contents:**
- Module oit Copy item path
- Modules§
- Structs§
- Functions§

Order Independent Transparency (OIT) for 3d rendering. See OrderIndependentTransparencyPlugin for more details.

---

## Struct GilrsPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gilrs/struct.GilrsPlugin.html

**Contents:**
- Struct GilrsPlugin Copy item path
- Trait Implementations§
  - impl Default for GilrsPlugin
    - fn default() -> GilrsPlugin
  - impl Plugin for GilrsPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str

Plugin that provides gamepad handling to an App.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GilrsPlugin;
```

---

## Enum DependencyLoadState Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.DependencyLoadState.html

**Contents:**
- Enum DependencyLoadState Copy item path
- Variants§
  - NotLoaded
  - Loading
  - Loaded
  - Failed(Arc<AssetLoadError>)
- Implementations§
  - impl DependencyLoadState
    - pub fn is_loading(&self) -> bool
    - pub fn is_loaded(&self) -> bool

The load state of an asset’s dependencies.

The asset has not started loading yet

Dependencies are still loading

Dependencies have all loaded

One or more dependencies have failed to load. The underlying AssetLoadError is referenced by Arc clones in all related LoadState and RecursiveDependencyLoadStates in the asset’s dependency tree.

Returns true if this instance is DependencyLoadState::Loading

Returns true if this instance is DependencyLoadState::Loaded

Returns true if this instance is DependencyLoadState::Failed

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum DependencyLoadState {
    NotLoaded,
    Loading,
    Loaded,
    Failed(Arc<AssetLoadError>),
}
```

---

## Struct AudioPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/struct.AudioPlugin.html

**Contents:**
- Struct AudioPlugin Copy item path
- Fields§
- Trait Implementations§
  - impl Default for AudioPlugin
    - fn default() -> AudioPlugin
  - impl Plugin for AudioPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)

Adds support for audio playback to a Bevy Application

Insert an AudioPlayer onto your entities to play audio.

The global volume for all audio entities.

The scale factor applied to the positions of audio sources and listeners for spatial audio.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AudioPlugin {
    pub global_volume: GlobalVolume,
    pub default_spatial_scale: SpatialScale,
}
```

---

## Struct FrameCountPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/struct.FrameCountPlugin.html

**Contents:**
- Struct FrameCountPlugin Copy item path
- Trait Implementations§
  - impl Default for FrameCountPlugin
    - fn default() -> FrameCountPlugin
  - impl Plugin for FrameCountPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str

Adds frame counting functionality to Apps.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct FrameCountPlugin;
```

---

## Crate winit Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/winit/index.html

**Contents:**
- Crate winit Copy item path
- Modules§
- Structs§
- Enums§
- Constants§
- Functions§
- Type Aliases§

bevy_winit provides utilities to handle window creation and the eventloop through winit

Most commonly, the WinitPlugin is used as part of DefaultPlugins. The app’s runner is set by WinitPlugin and handles the winit EventLoop. See winit_runner for details.

---

## Struct MinimalPlugins Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/struct.MinimalPlugins.html

**Contents:**
- Struct MinimalPlugins Copy item path
- §Example:
- Trait Implementations§
  - impl PluginGroup for MinimalPlugins
    - fn build(self) -> PluginGroupBuilder
    - fn name() -> String
    - fn set<T>(self, plugin: T) -> PluginGroupBuilderwhere T: Plugin,
- Auto Trait Implementations§
  - impl Freeze for MinimalPlugins
  - impl RefUnwindSafe for MinimalPlugins

This plugin group will add the minimal plugins for a Bevy application:

This plugin group represents the absolute minimum, bare-bones, bevy application. Use this if you want to have absolute control over the plugins used.

It includes a schedule runner (ScheduleRunnerPlugin) to provide functionality that would otherwise be driven by a windowed application’s event loop or message loop.

By default, this loop will run as fast as possible, which can result in high CPU usage. You can add a delay using run_loop, or remove the loop using run_once.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MinimalPlugins;
```

Example 2 (unknown):
```unknown
App::new().add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
    // Run 60 times per second.
    Duration::from_secs_f64(1.0 / 60.0),
))).run();
```

---

## Struct PrepassPipelinePlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PrepassPipelinePlugin.html

**Contents:**
- Struct PrepassPipelinePlugin Copy item path
- Trait Implementations§
  - impl Plugin for PrepassPipelinePlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str
    - fn is_unique(&self) -> bool
- Auto Trait Implementations§

Sets up everything required to use the prepass pipeline.

This does not add the actual prepasses, see PrepassPlugin for that.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PrepassPipelinePlugin;
```

---

## Struct FogPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.FogPlugin.html

**Contents:**
- Struct FogPlugin Copy item path
- Trait Implementations§
  - impl Plugin for FogPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str
    - fn is_unique(&self) -> bool
- Auto Trait Implementations§

A plugin that consolidates fog extraction, preparation and related resources/assets

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct FogPlugin;
```

---

## Trait AddAudioSource Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/trait.AddAudioSource.html

**Contents:**
- Trait AddAudioSource Copy item path
- Required Methods§
    - fn add_audio_source<T>(&mut self) -> &mut Selfwhere T: Decodable + Asset, f32: FromSample<<T as Decodable>::DecoderItem>,
- Dyn Compatibility§
- Implementors§
  - impl AddAudioSource for App

A trait that allows adding a custom audio source to the object. This is implemented for App to allow registering custom Decodable types.

Registers an audio source. The type must implement Decodable, so that it can be converted to a rodio::Source type, and Asset, so that it can be registered as an asset. To use this method on App, the audio and asset plugins must be added first.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait AddAudioSource {
    // Required method
    fn add_audio_source<T>(&mut self) -> &mut Self
       where T: Decodable + Asset,
             f32: FromSample<<T as Decodable>::DecoderItem>;
}
```

---

## Struct CorePipelinePlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/core_pipeline/struct.CorePipelinePlugin.html

**Contents:**
- Struct CorePipelinePlugin Copy item path
- Trait Implementations§
  - impl Default for CorePipelinePlugin
    - fn default() -> CorePipelinePlugin
  - impl Plugin for CorePipelinePlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CorePipelinePlugin;
```

---

## Struct VolumetricFogPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.VolumetricFogPlugin.html

**Contents:**
- Struct VolumetricFogPlugin Copy item path
- Trait Implementations§
  - impl Plugin for VolumetricFogPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str
    - fn is_unique(&self) -> bool
- Auto Trait Implementations§

A plugin that implements volumetric fog.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct VolumetricFogPlugin;
```

---

## Module contrast_adaptive_sharpening Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/anti_alias/contrast_adaptive_sharpening/index.html

**Contents:**
- Module contrast_adaptive_sharpening Copy item path
- Structs§
- Functions§

bevy::anti_aliasModule contrast_adaptive_sharpening Copy item pathSource Structs§CasNodeCasPipelineCasPipelineKeyCasPluginAdds Support for Contrast Adaptive Sharpening (CAS).ContrastAdaptiveSharpeningApplies a contrast adaptive sharpening (CAS) filter to the camera.DenoiseCasViewCasPipelineFunctions§init_cas_pipeline

---

## Struct AssetPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.AssetPlugin.html

**Contents:**
- Struct AssetPlugin Copy item path
- Fields§
- Trait Implementations§
  - impl Default for AssetPlugin
    - fn default() -> AssetPlugin
  - impl Plugin for AssetPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)

Provides “asset” loading and processing functionality. An Asset is a “runtime value” that is loaded from an AssetSource, which can be something like a filesystem, a network, etc.

Supports flexible “modes”, such as AssetMode::Processed and AssetMode::Unprocessed that enable using the asset workflow that best suits your project.

The default file path to use (relative to the project root) for unprocessed assets.

The default file path to use (relative to the project root) for processed assets.

If set, will override the default “watch for changes” setting. By default “watch for changes” will be false unless the watch cargo feature is set. watch can be enabled manually, or it will be automatically enabled if a specific watcher like file_watcher is enabled.

Most use cases should leave this set to None and enable a specific watcher feature such as file_watcher to enable watching for dev-scenarios.

The AssetMode to use for this server.

How/If asset meta files should be checked.

How to handle load requests of files that are outside the approved directories.

Approved folders are AssetPlugin::file_path and the folder of each AssetSource. Subfolders within these folders are also valid.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AssetPlugin {
    pub file_path: String,
    pub processed_file_path: String,
    pub watch_for_changes_override: Option<bool>,
    pub mode: AssetMode,
    pub meta_check: AssetMetaCheck,
    pub unapproved_path_mode: UnapprovedPathMode,
}
```

---

## Module upscaling Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/core_pipeline/upscaling/index.html

**Contents:**
- Module upscaling Copy item path
- Structs§

bevy::core_pipelineModule upscaling Copy item pathSource Structs§UpscalingNodeUpscalingPluginViewUpscalingPipeline

---

## Module experimental Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/core_pipeline/experimental/index.html

**Contents:**
- Module experimental Copy item path
- Modules§

Experimental rendering features.

Experimental features are features with known problems, missing features, compatibility issues, low performance, and/or future breaking changes, but are included nonetheless for testing purposes.

---

## Crate log Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/index.html

**Contents:**
- Crate log Copy item path
- Modules§
- Macros§
- Structs§
- Constants§
- Type Aliases§

This crate provides logging functions and configuration for Bevy apps, and automatically configures platform specific log handlers (i.e. Wasm or Android).

The macros provided for logging are reexported from tracing, and behave identically to it.

By default, the LogPlugin from this crate is included in Bevy’s DefaultPlugins and the logging macros can be used out of the box, if used.

For more fine-tuned control over logging behavior, set up the LogPlugin or DefaultPlugins during app initialization.

---

## Struct AccessibilityPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/a11y/struct.AccessibilityPlugin.html

**Contents:**
- Struct AccessibilityPlugin Copy item path
  - §Behavior
- Trait Implementations§
  - impl Default for AccessibilityPlugin
    - fn default() -> AccessibilityPlugin
  - impl Plugin for AccessibilityPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)

Plugin managing integration with accessibility APIs.

Note that it doesn’t handle GUI aspects of this integration, instead providing helpful resources for other interfaces to utilize.

This plugin’s main role is to initialize the AccessibilityRequested and ManageAccessibilityUpdates resources to their default values, meaning:

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AccessibilityPlugin;
```

---
