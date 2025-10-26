# Crates-Rs-Bevy - Ecs

**Pages:** 158

---

## Module erased_render_asset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/erased_render_asset/index.html

**Contents:**
- Module erased_render_asset Copy item path
- Structs§
- Enums§
- Traits§
- Functions§
- Type Aliases§

bevy::renderModule erased_render_asset Copy item pathSource Structs§AssetExtractionSystemsThe system set during which we extract modified assets to the render world.ErasedRenderAssetPluginThis plugin extracts the changed assets from the “app world” into the “render world” and prepares them for the GPU. They can then be accessed from the ErasedRenderAssets resource.ErasedRenderAssetsStores all GPU representations (ErasedRenderAsset) of ErasedRenderAsset::SourceAsset as long as they exist.ExtractedAssetsTemporarily stores the extracted and removed assets of the current frame.PrepareNextFrameAssetsAll assets that should be prepared next frame.Enums§PrepareAssetErrorTraits§ErasedRenderAssetDescribes how an asset gets extracted and prepared for rendering.ErasedRenderAssetDependencyFunctions§prepare_erased_assetsThis system prepares all assets of the corresponding ErasedRenderAsset::SourceAsset type which where extracted this frame for the GPU.Type Aliases§ExtractAssetsSetDeprecatedDeprecated alias for AssetExtractionSystems.

---

## Module name Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/name/index.html

**Contents:**
- Module name Copy item path
- Structs§

Provides the Name Component, used for identifying an Entity.

---

## Struct WindowPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.WindowPlugin.html

**Contents:**
- Struct WindowPlugin Copy item path
- Fields§
- Trait Implementations§
  - impl Default for WindowPlugin
    - fn default() -> WindowPlugin
  - impl Plugin for WindowPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)

A Plugin that defines an interface for windowing support in Bevy.

Settings for the primary window.

Some(custom_window) will spawn an entity with custom_window and PrimaryWindow as components. None will not spawn a primary window.

Defaults to Some(Window::default()).

Note that if there are no windows the App will exit (by default) due to exit_on_all_closed.

Settings for the cursor on the primary window.

Defaults to Some(CursorOptions::default()).

Has no effect if WindowPlugin::primary_window is None.

Whether to exit the app when there are no open windows.

If disabling this, ensure that you send the bevy_app::AppExit event when the app should exit. If this does not occur, you will create ‘headless’ processes (processes without windows), which may surprise your users. It is recommended to leave this setting to either ExitCondition::OnAllClosed or ExitCondition::OnPrimaryClosed.

ExitCondition::OnAllClosed will add exit_on_all_closed to Update. ExitCondition::OnPrimaryClosed will add exit_on_primary_closed to Update.

Whether to close windows when they are requested to be closed (i.e. when the close button is pressed).

If true, this plugin will add close_when_requested to Update. If this system (or a replacement) is not running, the close button will have no effect. This may surprise your users. It is recommended to leave this setting as true.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WindowPlugin {
    pub primary_window: Option<Window>,
    pub primary_cursor_options: Option<CursorOptions>,
    pub exit_condition: ExitCondition,
    pub close_when_requested: bool,
}
```

---

## Enum VisibilitySystems Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/visibility/enum.VisibilitySystems.html

**Contents:**
- Enum VisibilitySystems Copy item path
- Variants§
  - CalculateBounds
  - UpdateFrusta
  - VisibilityPropagate
  - CheckVisibility
  - MarkNewlyHiddenEntitiesInvisible
- Trait Implementations§
  - impl Clone for VisibilitySystems
    - fn clone(&self) -> VisibilitySystems

Label for the calculate_bounds, calculate_bounds_2d and calculate_bounds_text2d systems, calculating and inserting an Aabb to relevant entities.

Label for update_frusta in CameraProjectionPlugin.

Label for the system propagating the InheritedVisibility in a ChildOf / Children hierarchy.

Label for the check_visibility system updating ViewVisibility of each entity and the VisibleEntities of each view.\

System order ambiguities between systems in this set are ignored: the order of systems within this set is irrelevant, as check_visibility assumes that its operations are irreversible during the frame.

Label for the mark_newly_hidden_entities_invisible system, which sets ViewVisibility to ViewVisibility::HIDDEN for entities that no view has marked as visible.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum VisibilitySystems {
    CalculateBounds,
    UpdateFrusta,
    VisibilityPropagate,
    CheckVisibility,
    MarkNewlyHiddenEntitiesInvisible,
}
```

---

## Struct AssetTrackingSystems Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.AssetTrackingSystems.html

**Contents:**
- Struct AssetTrackingSystems Copy item path
- Trait Implementations§
  - impl Clone for AssetTrackingSystems
    - fn clone(&self) -> AssetTrackingSystems
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for AssetTrackingSystems
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Hash for AssetTrackingSystems
    - fn hash<__H>(&self, state: &mut __H)where __H: Hasher,
    - fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,

A system set that holds all “track asset” operations.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AssetTrackingSystems;
```

---

## Struct WindowDestroyed Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.WindowDestroyed.html

**Contents:**
- Struct WindowDestroyed Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for WindowDestroyed
    - fn clone(&self) -> WindowDestroyed
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for WindowDestroyed
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for WindowDestroyed
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<WindowDestroyed, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,

An event that is sent whenever a window is destroyed by the underlying window system.

Note that if your application only has a single window, this event may be your last chance to persist state before the application terminates.

Window that has been destroyed.

Note that this entity probably no longer exists by the time this event is received.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WindowDestroyed {
    pub window: Entity,
}
```

---

## Struct SystemInfo Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/struct.SystemInfo.html

**Contents:**
- Struct SystemInfo Copy item path
- §See also
- Fields§
- Trait Implementations§
  - impl Debug for SystemInfo
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for SystemInfo
    - fn default() -> SystemInfo
  - impl Resource for SystemInfowhere SystemInfo: Send + Sync + 'static,
- Auto Trait Implementations§

A resource that stores diagnostic information about the system. This information can be useful for debugging and profiling purposes.

SystemInformationDiagnosticsPlugin for more information.

System kernel version.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct SystemInfo {
    pub os: String,
    pub kernel: String,
    pub cpu: String,
    pub core_count: String,
    pub memory: String,
}
```

---

## Trait SystemParam Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/system/trait.SystemParam.html

**Contents:**
- Trait SystemParam Copy item path
- §Derive
  - §PhantomData
- §Example
- §Generic SystemParams
  - §Details
  - §Custom Validation Messages
  - §Builders
- §Safety
- Required Associated Types§

A parameter that can be used in a System.

This trait can be derived with the super::SystemParam macro. This macro only works if each field on the derived struct implements SystemParam. Note: There are additional requirements on the field types. See the Generic SystemParams section for details and workarounds of the probable cause if this derive causes an error to be emitted.

Derived SystemParam structs may have two lifetimes: 'w for data stored in the World, and 's for data stored in the parameter’s state.

The following list shows the most common SystemParams and which lifetime they require

PhantomData is a special type of SystemParam that does nothing. This is useful for constraining generic types or lifetimes.

When using the derive macro, you may see an error in the form of:

where [ParamType] is the type of one of your fields. To solve this error, you can wrap the field of type [ParamType] with StaticSystemParam (i.e. StaticSystemParam<[ParamType]>).

The derive macro requires that the SystemParam implementation of each field F’s Item’s is itself F (ignoring lifetimes for simplicity). This assumption is due to type inference reasons, so that the derived SystemParam can be used as an argument to a function system. If the compiler cannot validate this property for [ParamType], it will error in the form shown above.

This will most commonly occur when working with SystemParams generically, as the requirement has not been proven to the compiler.

When using the derive macro, any SystemParamValidationErrors will be propagated from the sub-parameters. If you want to override the error message, add a #[system_param(validation_message = "New message")] attribute to the parameter.

If you want to use a SystemParamBuilder with a derived SystemParam implementation, add a #[system_param(builder)] attribute to the struct. This will generate a builder struct whose name is the param struct suffixed with Builder. The builder will not be pub, so you may want to expose a method that returns an impl SystemParamBuilder<T>.

The implementor must ensure the following is true.

Used to store data which persists across invocations of a system.

The item type returned when constructing this system param. The value of this associated type should be Self, instantiated with new lifetimes.

You could think of SystemParam::Item<'w, 's> as being an operation that changes the lifetimes bound to Self.

Creates a new instance of this param’s State.

Registers any World access used 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub unsafe trait SystemParam: Sized {
    type State: Send + Sync + 'static;
    type Item<'world, 'state>: SystemParam<State = Self::State>;

    // Required methods
    fn init_state(world: &mut World) -> Self::State;
    fn init_access(
        state: &Self::State,
        system_meta: &mut SystemMeta,
        component_access_set: &mut FilteredAccessSet,
        world: &mut World,
    );
    unsafe fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        system_meta: &SystemMeta,
        world: UnsafeWorldCell<'world>,
        change_tick: Tick,
    ) -> Self::Item<'wo
...
```

Example 2 (unknown):
```unknown
Query<'w, 's, Entity>,
Query<'w, 's, &'static SomeComponent>,
Res<'w, SomeResource>,
ResMut<'w, SomeOtherResource>,
Local<'s, u8>,
Commands<'w, 's>,
MessageReader<'w, 's, SomeMessage>,
MessageWriter<'w, SomeMessage>
```

Example 3 (unknown):
```unknown
use std::marker::PhantomData;
use bevy_ecs::system::SystemParam;

#[derive(SystemParam)]
struct MyParam<'w, Marker: 'static> {
    foo: Res<'w, SomeResource>,
    marker: PhantomData<Marker>,
}

fn my_system<T: 'static>(param: MyParam<T>) {
    // Access the resource through `param.foo`
}
```

Example 4 (text):
```text
expected ... [ParamType]
found associated type `<[ParamType] as SystemParam>::Item<'_, '_>`
```

---

## Module camera Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/camera/index.html

**Contents:**
- Module camera Copy item path
- Structs§
- Enums§
- Traits§
- Functions§

bevy::renderModule camera Copy item pathSource Structs§CameraPluginCameraRenderGraphConfigures the RenderGraph name assigned to be run for a given Camera entity.ExtractedCameraMipBiasCamera component specifying a mip bias to apply when sampling from material textures.SortedCameraSortedCamerasCameras sorted by their order field. This is updated in the sort_cameras system.TemporalJitterA subpixel offset to jitter a perspective camera’s frustum by.Enums§MissingRenderTargetInfoErrorTraits§NormalizedRenderTargetExtFunctions§camera_systemSystem in charge of updating a Camera when its window or projection changes.extract_camerassort_cameras

---

## Module resource Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/resource/index.html

**Contents:**
- Module resource Copy item path
- Traits§
- Derive Macros§

Resources are unique, singleton-like data types that can be accessed from systems and stored in the World.

---

## Crate scene Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/scene/index.html

**Contents:**
- Crate scene Copy item path
- Modules§
- Structs§
- Enums§
- Functions§

Provides scene definition, instantiation and serialization/deserialization.

Scenes are collections of entities and their associated components that can be instantiated or removed from a world to allow composition. Scenes can be serialized/deserialized, for example to save part of the world state to a file.

---

## Struct MeshExtractionSystems Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshExtractionSystems.html

**Contents:**
- Struct MeshExtractionSystems Copy item path
- Trait Implementations§
  - impl Clone for MeshExtractionSystems
    - fn clone(&self) -> MeshExtractionSystems
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for MeshExtractionSystems
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Hash for MeshExtractionSystems
    - fn hash<__H>(&self, state: &mut __H)where __H: Hasher,
    - fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,

A SystemSet that encompasses both extract_meshes_for_cpu_building and extract_meshes_for_gpu_building.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshExtractionSystems;
```

---

## Enum RenderMeshInstances Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/enum.RenderMeshInstances.html

**Contents:**
- Enum RenderMeshInstances Copy item path
- Variants§
  - CpuBuilding(RenderMeshInstancesCpu)
  - GpuBuilding(RenderMeshInstancesGpu)
- Implementations§
  - impl RenderMeshInstances
    - pub fn mesh_asset_id(&self, entity: MainEntity) -> Option<AssetId<Mesh>>
    - pub fn render_mesh_queue_data( &self, entity: MainEntity, ) -> Option<RenderMeshQueueData<'_>>
      - Examples found in repository?
- Trait Implementations§

Information that the render world keeps about each entity that contains a mesh.

The set of information needed is different depending on whether CPU or GPU MeshUniform building is in use.

Information needed when using CPU mesh instance data building.

Information needed when using GPU mesh instance data building.

Returns the ID of the mesh asset attached to the given entity, if any.

Constructs RenderMeshQueueData for the given entity, if it has a mesh attached.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum RenderMeshInstances {
    CpuBuilding(RenderMeshInstancesCpu),
    GpuBuilding(RenderMeshInstancesGpu),
}
```

Example 2 (javascript):
```javascript
124fn queue_custom(
125    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
126    custom_pipeline: Res<CustomPipeline>,
127    mut pipelines: ResMut<SpecializedMeshPipelines<CustomPipeline>>,
128    pipeline_cache: Res<PipelineCache>,
129    meshes: Res<RenderAssets<RenderMesh>>,
130    render_mesh_instances: Res<RenderMeshInstances>,
131    material_meshes: Query<(Entity, &MainEntity), With<InstanceMaterialData>>,
132    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<Transparent3d>>,
133    views: Query<(&ExtractedView, &Msaa)>,
134) {
135    let draw_custom = 
...
```

Example 3 (javascript):
```javascript
496fn queue_custom_meshes(
497    custom_draw_functions: Res<DrawFunctions<Stencil3d>>,
498    mut pipelines: ResMut<SpecializedMeshPipelines<StencilPipeline>>,
499    pipeline_cache: Res<PipelineCache>,
500    custom_draw_pipeline: Res<StencilPipeline>,
501    render_meshes: Res<RenderAssets<RenderMesh>>,
502    render_mesh_instances: Res<RenderMeshInstances>,
503    mut custom_render_phases: ResMut<ViewSortedRenderPhases<Stencil3d>>,
504    mut views: Query<(&ExtractedView, &RenderVisibleEntities, &Msaa)>,
505    has_marker: Query<(), With<DrawStencil>>,
506) {
507    for (view, visible_enti
...
```

Example 4 (javascript):
```javascript
267fn queue_custom_mesh_pipeline(
268    pipeline_cache: Res<PipelineCache>,
269    custom_mesh_pipeline: Res<CustomMeshPipeline>,
270    (mut opaque_render_phases, opaque_draw_functions): (
271        ResMut<ViewBinnedRenderPhases<Opaque3d>>,
272        Res<DrawFunctions<Opaque3d>>,
273    ),
274    mut specialized_mesh_pipelines: ResMut<SpecializedMeshPipelines<CustomMeshPipeline>>,
275    views: Query<(&RenderVisibleEntities, &ExtractedView, &Msaa)>,
276    (render_meshes, render_mesh_instances): (
277        Res<RenderAssets<RenderMesh>>,
278        Res<RenderMeshInstances>,
279    ),
280 
...
```

---

## Struct SpatialAudioSink Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/struct.SpatialAudioSink.html

**Contents:**
- Struct SpatialAudioSink Copy item path
- Implementations§
  - impl SpatialAudioSink
    - pub fn new(sink: SpatialSink) -> SpatialAudioSink
  - impl SpatialAudioSink
    - pub fn set_ears_position(&self, left_position: Vec3, right_position: Vec3)
    - pub fn set_listener_position(&self, position: Transform, gap: f32)
    - pub fn set_emitter_position(&self, position: Vec3)
- Trait Implementations§
  - impl AudioSinkPlayback for SpatialAudioSink

Used to control spatial audio during playback.

Bevy inserts this component onto your entities when it begins playing an audio source that’s configured to use spatial audio.

You can use this component to modify the playback settings while the audio is playing.

If this component is removed from an entity, and a AudioSource is attached to that entity, that AudioSource will start playing. If that source is unchanged, that translates to the audio restarting.

Create a new spatial audio sink.

Set the two ears position.

Set the listener position, with an ear on each side separated by gap.

Set the emitter position.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct SpatialAudioSink { /* private fields */ }
```

---

## Struct SpatialListener Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/struct.SpatialListener.html

**Contents:**
- Struct SpatialListener Copy item path
- Fields§
- Implementations§
  - impl SpatialListener
    - pub fn new(gap: f32) -> SpatialListener
      - Examples found in repository?
- Trait Implementations§
  - impl Clone for SpatialListener
    - fn clone(&self) -> SpatialListener
    - fn clone_from(&mut self, source: &Self)

Settings for the listener for spatial audio sources.

This is accompanied by Transform and GlobalTransform. Only one entity with a SpatialListener should be present at any given time.

Left ear position relative to the GlobalTransform.

Right ear position relative to the GlobalTransform.

Creates a new SpatialListener component.

gap is the distance between the left and right “ears” of the listener. Ears are positioned on the x axis.

Required Components: Transform.

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components of the required components, recursively, in depth-first order.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct SpatialListener {
    pub left_ear_offset: Vec3,
    pub right_ear_offset: Vec3,
}
```

Example 2 (javascript):
```javascript
26fn setup(
27    mut commands: Commands,
28    mut meshes: ResMut<Assets<Mesh>>,
29    mut materials: ResMut<Assets<ColorMaterial>>,
30    asset_server: Res<AssetServer>,
31) {
32    // Space between the two ears
33    let gap = 400.0;
34
35    // sound emitter
36    commands.spawn((
37        Mesh2d(meshes.add(Circle::new(15.0))),
38        MeshMaterial2d(materials.add(Color::from(BLUE))),
39        Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)),
40        Emitter::default(),
41        AudioPlayer::new(asset_server.load("sounds/Windless Slopes.ogg")),
42        PlaybackSettings::LOOP
...
```

Example 3 (javascript):
```javascript
18fn setup(
19    mut commands: Commands,
20    asset_server: Res<AssetServer>,
21    mut meshes: ResMut<Assets<Mesh>>,
22    mut materials: ResMut<Assets<StandardMaterial>>,
23) {
24    // Space between the two ears
25    let gap = 4.0;
26
27    // sound emitter
28    commands.spawn((
29        Mesh3d(meshes.add(Sphere::new(0.2).mesh().uv(32, 18))),
30        MeshMaterial3d(materials.add(Color::from(BLUE))),
31        Transform::from_xyz(0.0, 0.0, 0.0),
32        Emitter::default(),
33        AudioPlayer::new(asset_server.load("sounds/Windless Slopes.ogg")),
34        PlaybackSettings::LOOP.w
...
```

---

## Struct Monitor Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.Monitor.html

**Contents:**
- Struct Monitor Copy item path
- §Warning
- Fields§
- Implementations§
  - impl Monitor
    - pub fn physical_size(&self) -> UVec2
- Trait Implementations§
  - impl Clone for Monitor
    - fn clone(&self) -> Monitor
    - fn clone_from(&mut self, source: &Self)

Represents an available monitor as reported by the user’s operating system, which can be used to query information about the display, such as its size, position, and video modes.

Each monitor corresponds to an entity and can be used to position a monitor using crate::window::MonitorSelection::Entity.

This component is synchronized with winit through bevy_winit, but is effectively read-only as winit does not support changing monitor properties.

The name of the monitor

The height of the monitor in physical pixels

The width of the monitor in physical pixels

The position of the monitor in physical pixels

The refresh rate of the monitor in millihertz

The scale factor of the monitor

The video modes that the monitor supports

Returns the physical size of the monitor in pixels

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Monitor {
    pub name: Option<String>,
    pub physical_height: u32,
    pub physical_width: u32,
    pub physical_position: IVec2,
    pub refresh_rate_millihertz: Option<u32>,
    pub scale_factor: f64,
    pub video_modes: Vec<VideoMode>,
}
```

---

## Module relationship Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/relationship/index.html

**Contents:**
- Module relationship Copy item path
- Structs§
- Enums§
- Traits§
- Functions§
- Type Aliases§

This module provides functionality to link entities to each other using specialized components called “relationships”. See the Relationship trait for more info.

---

## Struct AccessibilityNode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/a11y/struct.AccessibilityNode.html

**Contents:**
- Struct AccessibilityNode Copy item path
  - §Organization in the AccessKit Accessibility Tree
    - §Parent and Child
    - §On the Window
- Tuple Fields§
- Methods from Deref<Target = Node>§
    - pub fn role(&self) -> Role
    - pub fn set_role(&mut self, value: Role)
    - pub fn supports_action(&self, action: Action) -> bool
    - pub fn add_action(&mut self, action: Action)

Represents an entity to AccessKit through an accesskit::Node.

Platform-specific accessibility APIs utilize AccessKit nodes in their accessibility frameworks. So, this component acts as a translation between “Bevy entity” and “platform-agnostic accessibility element”.

AccessKit allows users to form a “tree of nodes” providing accessibility information. That tree is not Bevy’s ECS!

To explain, let’s say this component is added to an entity, E.

If E has a parent, P, and P also has this AccessibilityNode component, then E’s AccessKit node will be a child of P’s AccessKit node.

Resulting AccessKit tree:

In other words, parent-child relationships are maintained, but only if both have this component.

If E doesn’t have a parent, or if the immediate parent doesn’t have an AccessibilityNode, its AccessKit node will be an immediate child of the primary window.

Resulting AccessKit tree:

When there’s no AccessKit-compatible parent, the child lacks hierarchical information in AccessKit. As such, it is placed directly under the primary window on the AccessKit tree.

This behavior may or may not be intended, so please utilize AccessibilityNodes with care.

A representation of this component’s entity to AccessKit.

Note that, with its parent struct acting as just a newtype, users are intended to directly update this field.

Return whether the specified action is in the set supported on this node’s direct children in the filtered tree.

Add the specified action to the set supported on this node’s direct children in the filtered tree.

Remove the specified action from the set supported on this node’s direct children in the filtered tree.

Clear the set of actions supported on this node’s direct children in the filtered tree.

Exclude this node and its descendants from the tree presented to assistive technologies, and from hit testing.

If a dialog box is marked as explicitly modal.

This element allows touches to be passed through when a screen reader is in touch exploration mode, e.g. a virtual keyboard normally behaves this way.

Use for a text widget that allows focus/selection but not input.

Use for a control or group of controls that disallows input.

Indicates that this node clips its children, i.e. may have overflow: hidden or clip children by default.

Indicates whether this node causes a hard line-break (e.g. block level elements, or <br>).

Indicates whether this node causes a page break.

As with the aria-owns property in ARIA, this property should be se

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct AccessibilityNode(pub Node);
```

---

## Trait QueryFilter Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/query/trait.QueryFilter.html

**Contents:**
- Trait QueryFilter Copy item path
- §Safety
- Required Associated Constants§
    - const IS_ARCHETYPAL: bool
- Required Methods§
    - unsafe fn filter_fetch( state: &Self::State, fetch: &mut Self::Fetch<'_>, entity: Entity, table_row: TableRow, ) -> bool
      - §Safety
- Dyn Compatibility§
- Implementations on Foreign Types§
  - impl QueryFilter for ()

Types that filter the results of a Query.

There are many types that natively implement this trait:

Implementing the trait manually can allow for a fundamentally new type of behavior.

Query design can be easily structured by deriving QueryFilter for custom types. Despite the added complexity, this approach has several advantages over using QueryFilter tuples. The most relevant improvements are:

This trait can only be derived for structs if each field also implements QueryFilter.

The WorldQuery implementation must not take any mutable access. This is the same safety requirement as ReadOnlyQueryData.

Returns true if (and only if) this Filter relies strictly on archetypes to limit which components are accessed by the Query.

This enables optimizations for QueryIter that rely on knowing exactly how many elements are being iterated (such as Iterator::collect()).

If this is true, then QueryFilter::filter_fetch must always return true.

Returns true if the provided Entity and TableRow should be included in the query results. If false, the entity will be skipped.

Note that this is called after already restricting the matched Tables and Archetypes to the ones that are compatible with the Filter’s access.

Implementors of this method will generally either have a trivial true body (required for archetypal filters), or access the necessary data within this function to make the final decision on filter inclusion.

Must always be called after WorldQuery::set_table or WorldQuery::set_archetype. entity and table_row must be in the range of the current table and archetype.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

This trait is implemented for tuples up to 16 items long.

SAFETY: read-only access

This trait is implemented for tuples up to 16 items long.

**Examples:**

Example 1 (javascript):
```javascript
pub unsafe trait QueryFilter: WorldQuery {
    const IS_ARCHETYPAL: bool;

    // Required method
    unsafe fn filter_fetch(
        state: &Self::State,
        fetch: &mut Self::Fetch<'_>,
        entity: Entity,
        table_row: TableRow,
    ) -> bool;
}
```

Example 2 (unknown):
```unknown
#[derive(QueryFilter)]
struct MyFilter<T: Component, P: Component> {
    // Field names are not relevant, since they are never manually accessed.
    with_a: With<ComponentA>,
    or_filter: Or<(With<ComponentC>, Added<ComponentB>)>,
    generic_tuple: (With<T>, Without<P>),
}

fn my_system(query: Query<Entity, MyFilter<ComponentD, ComponentE>>) {
    // ...
}
```

---

## Struct AnimationTarget Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/struct.AnimationTarget.html

**Contents:**
- Struct AnimationTarget Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for AnimationTarget
    - fn clone(&self) -> AnimationTarget
    - fn clone_from(&mut self, source: &Self)
  - impl Component for AnimationTargetwhere AnimationTarget: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

An entity that can be animated by an AnimationPlayer.

These are frequently referred to as bones or joints, because they often refer to individually-animatable parts of an armature.

Asset loaders for armatures are responsible for adding these as necessary. Typically, they’re generated from hashed versions of the entire name path from the root of the armature to the bone. See the AnimationTargetId documentation for more details.

By convention, asset loaders add AnimationTarget components to the descendants of an AnimationPlayer, as well as to the AnimationPlayer entity itself, but Bevy doesn’t require this in any way. So, for example, it’s entirely possible for an AnimationPlayer to animate a target that it isn’t an ancestor of. If you add a new bone to or delete a bone from an armature at runtime, you may want to update the AnimationTarget component as appropriate, as Bevy won’t do this automatically.

Note that each entity can only be animated by one animation player at a time. However, you can change AnimationTarget’s player property at runtime to change which player is responsible for animating the entity.

The ID of this animation target.

Typically, this is derived from the path.

The entity containing the AnimationPlayer.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AnimationTarget {
    pub id: AnimationTargetId,
    pub player: Entity,
}
```

---

## Trait SpawnableList Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/spawn/trait.SpawnableList.html

**Contents:**
- Trait SpawnableList Copy item path
- Required Methods§
    - fn spawn(this: MovingPtr<'_, Self>, world: &mut World, entity: Entity)
    - fn size_hint(&self) -> usize
- Dyn Compatibility§
- Implementations on Foreign Types§
  - impl<R> SpawnableList<R> for ()where R: Relationship,
    - fn spawn(_this: MovingPtr<'_, ()>, _world: &mut World, _entity: Entity)where (): Sized,
    - fn size_hint(&self) -> usize
  - impl<R, P> SpawnableList<R> for (P₁, P₂, …, Pₙ)where R: Relationship, P: SpawnableList<R>,

A spawn-able list of changes to a given World and relative to a given Entity. This is generally used for spawning “related” entities, such as children.

Spawn this list of changes in a given World and relative to a given Entity. This is generally used for spawning “related” entities, such as children.

Returns a size hint, which is used to reserve space for this list in a RelationshipTarget. This should be less than or equal to the actual size of the list. When in doubt, just use 0.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

This trait is implemented for tuples up to 13 items long.

**Examples:**

Example 1 (unknown):
```unknown
pub trait SpawnableList<R>: Sized {
    // Required methods
    fn spawn(this: MovingPtr<'_, Self>, world: &mut World, entity: Entity);
    fn size_hint(&self) -> usize;
}
```

---

## Function check_dir_light_mesh_visibility Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/fn.check_dir_light_mesh_visibility.html

**Contents:**
- Function check_dir_light_mesh_visibility Copy item path

bevy::lightFunction check_dir_light_mesh_visibility Copy item pathSource pub fn check_dir_light_mesh_visibility( commands: Commands<'_, '_>, directional_lights: Query<'_, '_, (&DirectionalLight, &CascadesFrusta, &mut CascadesVisibleEntities, Option<&RenderLayers>, &ViewVisibility), Without<SpotLight>>, visible_entity_query: Query<'_, '_, (Entity, &InheritedVisibility, Option<&RenderLayers>, Option<&Aabb>, Option<&GlobalTransform>, Has<VisibilityRange>, Has<NoFrustumCulling>), (Without<NotShadowCaster>, Without<DirectionalLight>, With<Mesh3d>)>, visible_entity_ranges: Option<Res<'_, VisibleEntityRanges>>, defer_visible_entities_queue: Local<'_, Parallel<Vec<Entity>>>, view_visible_entities_queue: Local<'_, Parallel<Vec<Vec<Entity>>>>, )

**Examples:**

Example 1 (unknown):
```unknown
pub fn check_dir_light_mesh_visibility(
    commands: Commands<'_, '_>,
    directional_lights: Query<'_, '_, (&DirectionalLight, &CascadesFrusta, &mut CascadesVisibleEntities, Option<&RenderLayers>, &ViewVisibility), Without<SpotLight>>,
    visible_entity_query: Query<'_, '_, (Entity, &InheritedVisibility, Option<&RenderLayers>, Option<&Aabb>, Option<&GlobalTransform>, Has<VisibilityRange>, Has<NoFrustumCulling>), (Without<NotShadowCaster>, Without<DirectionalLight>, With<Mesh3d>)>,
    visible_entity_ranges: Option<Res<'_, VisibleEntityRanges>>,
    defer_visible_entities_queue: Local<'_, P
...
```

---

## Macro children Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/macro.children.html

**Contents:**
- Macro children Copy item path

Returns a SpawnRelatedBundle that will insert the Children component, spawn a SpawnableList of entities with given bundles that relate to the Children entity via the ChildOf component, and reserve space in the Children for each spawned entity.

Any additional arguments will be interpreted as bundles to be spawned.

Also see related for a version of this that works with any RelationshipTarget type.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! children {
    [$($child:expr),*$(,)?] => { ... };
}
```

Example 2 (javascript):
```javascript
let mut world = World::new();
world.spawn((
    Name::new("Root"),
    children![
        Name::new("Child1"),
        (
            Name::new("Child2"),
            children![Name::new("Grandchild")]
        )
    ]
));
```

---

## Struct InputDispatchPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input_focus/struct.InputDispatchPlugin.html

**Contents:**
- Struct InputDispatchPlugin Copy item path
- Trait Implementations§
  - impl Plugin for InputDispatchPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str
    - fn is_unique(&self) -> bool
- Auto Trait Implementations§

Plugin which sets up systems for dispatching bubbling keyboard and gamepad button events to the focused entity.

To add bubbling to your own input events, add the dispatch_focused_input::<MyEvent> system to your app, as described in the docs for FocusedInput.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct InputDispatchPlugin;
```

---

## Struct PerspectiveProjection Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.PerspectiveProjection.html

**Contents:**
- Struct PerspectiveProjection Copy item path
- Fields§
- Trait Implementations§
  - impl CameraProjection for PerspectiveProjection
    - fn get_clip_from_view(&self) -> Mat4
    - fn get_clip_from_view_for_sub(&self, sub_view: &SubCameraView) -> Mat4
    - fn update(&mut self, width: f32, height: f32)
    - fn far(&self) -> f32
    - fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [Vec3A; 8]
    - fn compute_frustum(&self, camera_transform: &GlobalTransform) -> Frustum

A 3D camera projection in which distant objects appear smaller than close objects.

The vertical field of view (FOV) in radians.

Defaults to a value of π/4 radians or 45 degrees.

The aspect ratio (width divided by height) of the viewing frustum.

Bevy’s camera_system automatically updates this value when the aspect ratio of the associated window changes.

Defaults to a value of 1.0.

The distance from the camera in world units of the viewing frustum’s near plane.

Objects closer to the camera than this value will not be visible.

Defaults to a value of 0.1.

The distance from the camera in world units of the viewing frustum’s far plane.

Objects farther from the camera than this value will not be visible.

Defaults to a value of 1000.0.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PerspectiveProjection {
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}
```

---

## Crate remote Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/remote/index.html

**Contents:**
- Crate remote Copy item path
  - §Request objects
  - §Response objects
  - §Error objects
  - §Built-in methods
    - §world.get_components
    - §world.query
      - §Example
    - §world.spawn_entity
    - §world.despawn_entity

An implementation of the Bevy Remote Protocol, to allow for remote control of a Bevy app.

Adding the RemotePlugin to your App will setup everything needed without starting any transports. To start accepting remote connections you will need to add a second plugin like the RemoteHttpPlugin to enable communication over HTTP. These remote clients can inspect and alter the state of the entity-component system.

The Bevy Remote Protocol is based on the JSON-RPC 2.0 protocol.

A typical client request might look like this:

The id and method fields are required. The params field may be omitted for certain methods:

id is arbitrary JSON data. The server completely ignores its contents, and the client may use it for any purpose. It will be copied via serialization and deserialization (so object property order, etc. can’t be relied upon to be identical) and sent back to the client as part of the response.

method is a string that specifies one of the possible BrpRequest variants: world.query, world.get_components, world.insert_components, etc. It’s case-sensitive.

params is parameter data specific to the request.

For more information, see the documentation for BrpRequest. BrpRequest is serialized to JSON via serde, so the serde documentation may be useful to clarify the correspondence between the Rust structure and the JSON format.

A response from the server to the client might look like this:

The id field will always be present. The result field will be present if the request was successful. Otherwise, an error field will replace it.

id is the arbitrary JSON data that was sent as part of the request. It will be identical to the id data sent during the request, modulo serialization and deserialization. If there’s an error reading the id field, it will be null.

result will be present if the request succeeded and will contain the response specific to the request.

error will be present if the request failed and will contain an error object with more information about the cause of failure.

An error object might look like this:

The code and message fields will always be present. There may also be a data field.

code is an integer representing the kind of an error that happened. Error codes documented in the error_codes module.

message is a short, one-sentence human-readable description of the error.

data is an optional field of arbitrary type containing additional information about the error.

The Bevy Remote Protocol includes a number of built-in methods for

*[Content truncated]*

**Examples:**

Example 1 (json):
```json
{
    "method": "world.get_components",
    "id": 0,
    "params": {
        "entity": 4294967298,
        "components": [
            "bevy_transform::components::transform::Transform"
        ]
    }
}
```

Example 2 (json):
```json
{
    "jsonrpc": "2.0",
    "id": 0,
    "result": {
        "bevy_transform::components::transform::Transform": {
            "rotation": { "x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0 },
            "scale": { "x": 1.0, "y": 1.0, "z": 1.0 },
            "translation": { "x": 0.0, "y": 0.5, "z": 0.0 }
        }
    }
}
```

Example 3 (json):
```json
{
    "code": -32602,
    "message": "Missing \"entity\" field"
}
```

Example 4 (json):
```json
{
    "jsonrpc": "2.0",
    "method": "bevy/query",
    "id": 0,
    "params": {
        "data": {
            "components": ["bevy_transform::components::transform::Transform"]
            "option": [],
            "has": []
       },
       "filter": {
          "with": [],
          "without": []
        },
        "strict": false
    }
}
```

---

## Struct VisibleEntities Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/visibility/struct.VisibleEntities.html

**Contents:**
- Struct VisibleEntities Copy item path
- Fields§
- Implementations§
  - impl VisibleEntities
    - pub fn get(&self, type_id: TypeId) -> &[Entity]
    - pub fn get_mut(&mut self, type_id: TypeId) -> &mut Vec<Entity>
    - pub fn iter(&self, type_id: TypeId) -> impl DoubleEndedIterator
    - pub fn len(&self, type_id: TypeId) -> usize
    - pub fn is_empty(&self, type_id: TypeId) -> bool
    - pub fn clear(&mut self, type_id: TypeId)

Collection of entities visible from the current view.

This component contains all entities which are visible from the currently rendered view. The collection is updated automatically by the VisibilitySystems::CheckVisibility system set. Renderers can use the equivalent RenderVisibleEntities to optimize rendering of a particular view, to prevent drawing items not visible from that view.

This component is intended to be attached to the same entity as the Camera and the Frustum defining the view.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct VisibleEntities {
    pub entities: HashMap<TypeId, Vec<Entity>, NoOpHash>,
}
```

---

## Enum SimulationLightSystems Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/enum.SimulationLightSystems.html

**Contents:**
- Enum SimulationLightSystems Copy item path
- Variants§
  - AddClusters
  - AssignLightsToClusters
  - UpdateDirectionalLightCascades
  - UpdateLightFrusta
  - CheckLightVisibility
- Trait Implementations§
  - impl Clone for SimulationLightSystems
    - fn clone(&self) -> SimulationLightSystems

System sets used to run light-related systems.

System order ambiguities between systems in this set are ignored: each build_directional_light_cascades system is independent of the others, and should operate on distinct sets of entities.

System order ambiguities between systems in this set are ignored: the order of systems within this set is irrelevant, as the various visibility-checking systems assumes that their operations are irreversible during the frame.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum SimulationLightSystems {
    AddClusters,
    AssignLightsToClusters,
    UpdateDirectionalLightCascades,
    UpdateLightFrusta,
    CheckLightVisibility,
}
```

---

## Enum GizmoRenderSystems Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/enum.GizmoRenderSystems.html

**Contents:**
- Enum GizmoRenderSystems Copy item path
- Variants§
  - QueueLineGizmos2d
  - QueueLineGizmos3d
- Trait Implementations§
  - impl Clone for GizmoRenderSystems
    - fn clone(&self) -> GizmoRenderSystems
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for GizmoRenderSystems
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

System set label for the systems handling the rendering of gizmos.

Adds gizmos to the Transparent2d render phase

Adds gizmos to the Transparent3d render phase

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum GizmoRenderSystems {
    QueueLineGizmos2d,
    QueueLineGizmos3d,
}
```

---

## Type Alias RenderSet Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/type.RenderSet.html

**Contents:**
- Type Alias RenderSet Copy item path
- Aliased Type§
- Variants§
  - ExtractCommands
  - PrepareAssets
  - PrepareMeshes
  - ManageViews
  - Queue
  - QueueMeshes
  - QueueSweep

Deprecated alias for RenderSystems.

This is used for applying the commands from the ExtractSchedule

Prepare assets that have been created/modified/removed this frame.

Prepares extracted meshes.

Create any additional views such as those used for shadow mapping.

Queue drawable entities as phase items in render phases ready for sorting (if necessary)

A sub-set within Queue where mesh entity queue systems are executed. Ensures prepare_assets::<RenderMesh> is completed.

A sub-set within Queue where meshes that have become invisible or changed phases are removed from the bins.

Sort the SortedRenderPhases and BinKeys here.

Prepare render resources from extracted data for the GPU based on their sorted order. Create BindGroups that depend on those data.

A sub-set within Prepare for initializing buffers, textures and uniforms for use in bind groups.

Collect phase buffers after PrepareResources has run.

Flush buffers after PrepareResources, but before PrepareBindGroups.

A sub-set within Prepare for constructing bind groups, or other data that relies on render resources prepared in PrepareResources.

Actual rendering happens here. In most cases, only the render backend should insert resources here.

Cleanup render resources here.

Final cleanup occurs: all entities will be despawned.

**Examples:**

Example 1 (unknown):
```unknown
pub type RenderSet = RenderSystems;
```

Example 2 (unknown):
```unknown
pub enum RenderSet {
Show 16 variants    ExtractCommands,
    PrepareAssets,
    PrepareMeshes,
    ManageViews,
    Queue,
    QueueMeshes,
    QueueSweep,
    PhaseSort,
    Prepare,
    PrepareResources,
    PrepareResourcesCollectPhaseBuffers,
    PrepareResourcesFlush,
    PrepareBindGroups,
    Render,
    Cleanup,
    PostCleanup,
}
```

---

## Struct LightProbe Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.LightProbe.html

**Contents:**
- Struct LightProbe Copy item path
- Implementations§
  - impl LightProbe
    - pub fn new() -> LightProbe
- Trait Implementations§
  - impl Clone for LightProbe
    - fn clone(&self) -> LightProbe
    - fn clone_from(&mut self, source: &Self)
  - impl Component for LightProbewhere LightProbe: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table

A marker component for a light probe, which is a cuboid region that provides global illumination to all fragments inside it.

Note that a light probe will have no effect unless the entity contains some kind of illumination, which can either be an EnvironmentMapLight or an IrradianceVolume.

The light probe range is conceptually a unit cube (1×1×1) centered on the origin. The Transform applied to this entity can scale, rotate, or translate that cube so that it contains all fragments that should take this light probe into account.

When multiple sources of indirect illumination can be applied to a fragment, the highest-quality one is chosen. Diffuse and specular illumination are considered separately, so, for example, Bevy may decide to sample the diffuse illumination from an irradiance volume and the specular illumination from a reflection probe. From highest priority to lowest priority, the ranking is as follows:

Note that ambient light is always added to the diffuse component and does not participate in the ranking. That is, ambient light is applied in addition to, not instead of, the light sources above.

A terminology note: Unfortunately, there is little agreement across game and graphics engines as to what to call the various techniques that Bevy groups under the term light probe. In Bevy, a light probe is the generic term that encompasses both reflection probes and irradiance volumes. In object-oriented terms, light probe is the superclass, and reflection probe and irradiance volume are subclasses. In other engines, you may see the term light probe refer to an irradiance volume with a single voxel, or perhaps some other technique, while in Bevy light probe refers not to a specific technique but rather to a class of techniques. Developers familiar with other engines should be aware of this terminology difference.

Creates a new light probe component.

Required Components: Transform, Visibility.

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components of the required components, recursively, in depth-first order.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct LightProbe;
```

---

## Crate ecs Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/index.html

**Contents:**
- Crate ecs Copy item path
- §Bevy ECS
  - §What is Bevy ECS?
  - §ECS
  - §Concepts
    - §Components
    - §Worlds
    - §Entities
    - §Systems
    - §Resources

Bevy ECS is an Entity Component System custom-built for the Bevy game engine. It aims to be simple to use, ergonomic, fast, massively parallel, opinionated, and featureful. It was created specifically for Bevy’s needs, but it can easily be used as a standalone crate in other projects.

All app logic in Bevy uses the Entity Component System paradigm, which is often shortened to ECS. ECS is a software pattern that involves breaking your program up into Entities, Components, and Systems. Entities are unique “things” that are assigned groups of Components, which are then processed using Systems.

For example, one entity might have a Position and Velocity component, whereas another entity might have a Position and UI component. You might have a movement system that runs on all entities with a Position and Velocity component.

The ECS pattern encourages clean, decoupled designs by forcing you to break up your app data and logic into its core components. It also helps make your code faster by optimizing memory access patterns and making parallelism easier.

Bevy ECS is Bevy’s implementation of the ECS pattern. Unlike other Rust ECS implementations, which often require complex lifetimes, traits, builder patterns, or macros, Bevy ECS uses normal Rust data types for all of these concepts:

Components are normal Rust structs. They are data stored in a World and specific instances of Components correlate to Entities.

Entities, Components, and Resources are stored in a World. Worlds, much like std::collections’s HashSet and Vec, expose operations to insert, read, write, and remove the data they store.

Entities are unique identifiers that correlate to zero or more Components.

Systems are normal Rust functions. Thanks to the Rust type system, Bevy ECS can use function parameter types to determine what data needs to be sent to the system. It also uses this “data access” information to determine what Systems can run in parallel with each other.

Apps often require unique resources, such as asset collections, renderers, audio servers, time, etc. Bevy ECS makes this pattern a first class citizen. Resource is a special kind of component that does not belong to any entity. Instead, it is identified uniquely by its type:

Schedules run a set of Systems according to some execution strategy. Systems can be added to any number of System Sets, which are used to control their scheduling metadata.

The built in “parallel executor” considers dependencies between systems and (by def

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
use bevy_ecs::prelude::*;

#[derive(Component)]
struct Position { x: f32, y: f32 }
```

Example 2 (javascript):
```javascript
use bevy_ecs::world::World;

let world = World::default();
```

Example 3 (javascript):
```javascript
use bevy_ecs::prelude::*;

#[derive(Component)]
struct Position { x: f32, y: f32 }
#[derive(Component)]
struct Velocity { x: f32, y: f32 }

let mut world = World::new();

let entity = world
    .spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 0.0 }))
    .id();

let entity_ref = world.entity(entity);
let position = entity_ref.get::<Position>().unwrap();
let velocity = entity_ref.get::<Velocity>().unwrap();
```

Example 4 (unknown):
```unknown
use bevy_ecs::prelude::*;

#[derive(Component)]
struct Position { x: f32, y: f32 }

fn print_position(query: Query<(Entity, &Position)>) {
    for (entity, position) in &query {
        println!("Entity {} is at position: x {}, y {}", entity, position.x, position.y);
    }
}
```

---

## Struct ViewLightProbesUniformOffset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ViewLightProbesUniformOffset.html

**Contents:**
- Struct ViewLightProbesUniformOffset Copy item path
- Methods from Deref<Target = u32>§
    - pub const MIN: u32 = 0u32
    - pub const MAX: u32 = 4_294_967_295u32
    - pub const BITS: u32 = 32u32
- Trait Implementations§
  - impl Component for ViewLightProbesUniformOffsetwhere ViewLightProbesUniformOffset: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

A component attached to each camera in the render world that stores the index of the LightProbesUniform in the LightProbesBuffer.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ViewLightProbesUniformOffset(/* private fields */);
```

---

## Trait ScheduleLabel Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/schedule/trait.ScheduleLabel.html

**Contents:**
- Trait ScheduleLabel Copy item path
- §Defining new schedule labels
- Required Methods§
    - fn dyn_clone(&self) -> Box<dyn ScheduleLabel>
- Provided Methods§
    - fn intern(&self) -> Interned<dyn ScheduleLabel>where Self: Sized,
      - Examples found in repository?
- Trait Implementations§
  - impl Hash for dyn ScheduleLabel
    - fn hash<H>(&self, state: &mut H)where H: Hasher,

A strongly-typed class of labels used to identify a Schedule.

Each schedule in a World has a unique schedule label value, and schedules can be automatically created from labels via Schedules::add_systems().

By default, you should use Bevy’s premade schedule labels which implement this trait. If you are using bevy_ecs directly or if you need to run a group of systems outside the existing schedules, you may define your own schedule labels by using #[derive(ScheduleLabel)].

Clones this ScheduleLabel.

Returns an Interned value corresponding to self.

**Examples:**

Example 1 (unknown):
```unknown
pub trait ScheduleLabel:
    Send
    + Sync
    + Debug
    + DynEq
    + DynHash {
    // Required method
    fn dyn_clone(&self) -> Box<dyn ScheduleLabel>;

    // Provided method
    fn intern(&self) -> Interned<dyn ScheduleLabel>
       where Self: Sized { ... }
}
```

Example 2 (javascript):
```javascript
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;

// Declare a new schedule label.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash, Default)]
struct Update;

let mut world = World::new();

// Add a system to the schedule with that label (creating it automatically).
fn a_system_function() {}
world.get_resource_or_init::<Schedules>().add_systems(Update, a_system_function);

// Run the schedule, and therefore run the system.
world.run_schedule(Update);
```

Example 3 (unknown):
```unknown
22    pub fn add_schedule(mut self, label: impl ScheduleLabel) -> SteppingPlugin {
23        self.schedule_labels.push(label.intern());
24        self
25    }
```

---

## Struct Lightmap Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.Lightmap.html

**Contents:**
- Struct Lightmap Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for Lightmap
    - fn clone(&self) -> Lightmap
    - fn clone_from(&mut self, source: &Self)
  - impl Component for Lightmapwhere Lightmap: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

A component that applies baked indirect diffuse global illumination from a lightmap.

When assigned to an entity that contains a Mesh3d and a MeshMaterial3d<StandardMaterial>, if the mesh has a second UV layer (ATTRIBUTE_UV_1), then the lightmap will render using those UVs.

The lightmap texture.

The rectangle within the lightmap texture that the UVs are relative to.

The top left coordinate is the min part of the rect, and the bottom right coordinate is the max part of the rect. The rect ranges from (0, 0) to (1, 1).

This field allows lightmaps for a variety of meshes to be packed into a single atlas.

Whether bicubic sampling should be used for sampling this lightmap.

Bicubic sampling is higher quality, but slower, and may lead to light leaks.

If true, the lightmap texture’s sampler must be set to bevy_image::ImageSampler::linear.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Lightmap {
    pub image: Handle<Image>,
    pub uv_rect: Rect,
    pub bicubic_sampling: bool,
}
```

---

## Module hierarchy Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/hierarchy/index.html

**Contents:**
- Module hierarchy Copy item path
- Structs§
- Functions§
- Type Aliases§

The canonical “parent-child” Relationship for entities, driven by the ChildOf Relationship and the Children RelationshipTarget.

See ChildOf for a full description of the relationship and how to use it.

---

## Struct Access Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/query/struct.Access.html

**Contents:**
- Struct Access Copy item path
- Implementations§
  - impl Access
    - pub const fn new() -> Access
    - pub fn add_component_read(&mut self, index: ComponentId)
    - pub fn add_component_write(&mut self, index: ComponentId)
    - pub fn add_resource_read(&mut self, index: ComponentId)
    - pub fn add_resource_write(&mut self, index: ComponentId)
    - pub fn remove_component_read(&mut self, index: ComponentId)
    - pub fn remove_component_write(&mut self, index: ComponentId)

Tracks read and write access to specific elements in a collection.

Used internally to ensure soundness during system initialization and execution. See the is_compatible and get_conflicts functions.

Creates an empty Access collection.

Adds access to the component given by index.

Adds exclusive access to the component given by index.

Adds access to the resource given by index.

Adds exclusive access to the resource given by index.

Removes both read and write access to the component given by index.

Because this method corresponds to the set difference operator ∖, it can create complicated logical formulas that you should verify correctness of. For example, A ∪ (B ∖ A) isn’t equivalent to (A ∪ B) ∖ A, so you can’t replace a call to remove_component_read followed by a call to extend with a call to extend followed by a call to remove_component_read.

Removes write access to the component given by index.

Because this method corresponds to the set difference operator ∖, it can create complicated logical formulas that you should verify correctness of. For example, A ∪ (B ∖ A) isn’t equivalent to (A ∪ B) ∖ A, so you can’t replace a call to remove_component_write followed by a call to extend with a call to extend followed by a call to remove_component_write.

Adds an archetypal (indirect) access to the component given by index.

This is for components whose values are not accessed (and thus will never cause conflicts), but whose presence in an archetype may affect query results.

Currently, this is only used for Has<T> and Allow<T>.

Returns true if this can access the component given by index.

Returns true if this can access any component.

Returns true if this can exclusively access the component given by index.

Returns true if this accesses any component mutably.

Returns true if this can access the resource given by index.

Returns true if this can access any resource.

Returns true if this can exclusively access the resource given by index.

Returns true if this accesses any resource mutably.

Returns true if this accesses any resources or components.

Returns true if this accesses any resources or components mutably.

Returns true if this has an archetypal (indirect) access to the component given by index.

This is a component whose value is not accessed (and thus will never cause conflicts), but whose presence in an archetype may affect query results.

Currently, this is only used for Has<T>.

Sets this as having access to all components (i.e. Entity

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Access { /* private fields */ }
```

Example 2 (javascript):
```javascript
let mut access = Access::default();

access.add_component_read(ComponentId::new(1));
access.add_component_write(ComponentId::new(2));
access.add_archetypal(ComponentId::new(3));

let result = access
    .try_iter_component_access()
    .map(Iterator::collect::<Vec<_>>);

assert_eq!(
    result,
    Ok(vec![
        ComponentAccessKind::Shared(ComponentId::new(1)),
        ComponentAccessKind::Exclusive(ComponentId::new(2)),
        ComponentAccessKind::Archetypal(ComponentId::new(3)),
    ]),
);
```

Example 3 (javascript):
```javascript
51fn main() {
52    let mut world = World::new();
53    let mut lines = std::io::stdin().lines();
54    let mut component_names = HashMap::<String, ComponentId>::new();
55    let mut component_info = HashMap::<ComponentId, ComponentInfo>::new();
56
57    println!("{PROMPT}");
58    loop {
59        print!("\n> ");
60        let _ = std::io::stdout().flush();
61        let Some(Ok(line)) = lines.next() else {
62            return;
63        };
64
65        if line.is_empty() {
66            return;
67        };
68
69        let Some((first, rest)) = line.trim().split_once(|c: char| c.is_whitesp
...
```

---

## Struct InheritWeightSystems Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.InheritWeightSystems.html

**Contents:**
- Struct InheritWeightSystems Copy item path
- Trait Implementations§
  - impl Clone for InheritWeightSystems
    - fn clone(&self) -> InheritWeightSystems
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for InheritWeightSystems
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Hash for InheritWeightSystems
    - fn hash<__H>(&self, state: &mut __H)where __H: Hasher,
    - fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,

bevy_render::mesh::inherit_weights runs in this SystemSet

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct InheritWeightSystems;
```

---

## Struct MeshMorphWeights Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/morph/struct.MeshMorphWeights.html

**Contents:**
- Struct MeshMorphWeights Copy item path
- Implementations§
  - impl MeshMorphWeights
    - pub fn new(weights: Vec<f32>) -> Result<MeshMorphWeights, MorphBuildError>
    - pub fn weights(&self) -> &[f32]
    - pub fn weights_mut(&mut self) -> &mut [f32]
    - pub fn clear_weights(&mut self)
    - pub fn extend_weights(&mut self, weights: &[f32])
- Trait Implementations§
  - impl Clone for MeshMorphWeights

Control a specific Mesh instance’s morph targets. These control the weights of specific “mesh primitives” in scene formats like GLTF. They can be set manually, but in most cases they should “automatically” synced by setting the MorphWeights component on a parent entity.

See MorphWeights for more details on Bevy’s morph target implementation.

Add this to an Entity with a Mesh3d with a MorphAttributes set to control individual weights of each morph target.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshMorphWeights { /* private fields */ }
```

---

## Trait Trigger Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/event/trait.Trigger.html

**Contents:**
- Trait Trigger Copy item path
- §Safety
- Required Methods§
    - unsafe fn trigger( &mut self, world: DeferredWorld<'_>, observers: &CachedObservers, trigger_context: &TriggerContext, event: &mut E, )
      - §Safety
- Implementors§
  - impl<'a, E> Trigger<E> for EntityComponentsTrigger<'a>where E: EntityEvent<Trigger<'a> = EntityComponentsTrigger<'a>> + Event,
  - impl<E> Trigger<E> for AnimationEventTriggerwhere E: AnimationEvent<Trigger<'a> = AnimationEventTrigger> + for<'a> Event,
  - impl<E> Trigger<E> for EntityTriggerwhere E: EntityEvent<Trigger<'a> = EntityTrigger> + for<'a> Event,
  - impl<E> Trigger<E> for GlobalTriggerwhere E: for<'a> Event<Trigger<'a> = GlobalTrigger>,

Trigger determines how an Event is triggered when World::trigger is called. This decides which Observers will run, what data gets passed to them, and the order they will be executed in.

Implementing Trigger is “advanced-level” territory, and is generally unnecessary unless you are developing highly specialized Event trigger logic.

Bevy comes with a number of built-in Trigger implementations (see their documentation for more info):

Implementing this properly is advanced soundness territory! Implementers must abide by the following:

Trigger the given event, running every Observer that matches the event, as defined by this Trigger and the state stored on self.

**Examples:**

Example 1 (unknown):
```unknown
pub unsafe trait Trigger<E>where
    E: Event,{
    // Required method
    unsafe fn trigger(
        &mut self,
        world: DeferredWorld<'_>,
        observers: &CachedObservers,
        trigger_context: &TriggerContext,
        event: &mut E,
    );
}
```

---

## Module storage Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/storage/index.html

**Contents:**
- Module storage Copy item path
- §Fetching Storages
- §Safety
- Structs§
- Traits§

Storage layouts for ECS data.

This module implements the low-level collections that store data in a World. These all offer minimal and often unsafe APIs, and have been made pub primarily for debugging and monitoring purposes.

Each of the below data stores can be fetched via Storages, which can be fetched from a World via World::storages. It exposes a top level container for each class of storage within ECS:

To avoid trivially unsound use of the APIs in this module, it is explicitly impossible to get a mutable reference to Storages from World, and none of the types publicly expose a mutable interface.

---

## Struct Extract Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/struct.Extract.html

**Contents:**
- Struct Extract Copy item path
  - §Context
  - §Examples
- Trait Implementations§
  - impl<'w, 's, P> Deref for Extract<'w, 's, P>where P: ReadOnlySystemParam,
    - type Target = <P as SystemParam>::Item<'w, 's>
    - fn deref(&self) -> &<Extract<'w, 's, P> as Deref>::Target
  - impl<'w, 's, P> DerefMut for Extract<'w, 's, P>where P: ReadOnlySystemParam,
    - fn deref_mut(&mut self) -> &mut <Extract<'w, 's, P> as Deref>::Target
  - impl<'a, 'w, 's, P> IntoIterator for &'a Extract<'w, 's, P>where P: ReadOnlySystemParam, &'a <P as SystemParam>::Item<'w, 's>: IntoIterator,

A helper for accessing MainWorld content using a system parameter.

A SystemParam adapter which applies the contained SystemParam to the World contained in MainWorld. This parameter only works for systems run during the ExtractSchedule.

This requires that the contained SystemParam does not mutate the world, as it uses a read-only reference to MainWorld internally.

ExtractSchedule is used to extract (move) data from the simulation world (MainWorld) to the render world. The render world drives rendering each frame (generally to a Window). This design is used to allow performing calculations related to rendering a prior frame at the same time as the next frame is simulated, which increases throughput (FPS).

Extract is used to get data from the main world during ExtractSchedule.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Extract<'w, 's, P>where
    P: ReadOnlySystemParam + 'static,{ /* private fields */ }
```

Example 2 (unknown):
```unknown
use bevy_ecs::prelude::*;
use bevy_render::Extract;
use bevy_render::sync_world::RenderEntity;
// Do make sure to sync the cloud entities before extracting them.
fn extract_clouds(mut commands: Commands, clouds: Extract<Query<RenderEntity, With<Cloud>>>) {
    for cloud in &clouds {
        commands.entity(cloud).insert(Cloud);
    }
}
```

---

## Type Alias RumbleSystem Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gilrs/type.RumbleSystem.html

**Contents:**
- Type Alias RumbleSystem Copy item path
- Aliased Type§

Deprecated alias for RumbleSystems.

**Examples:**

Example 1 (unknown):
```unknown
pub type RumbleSystem = RumbleSystems;
```

Example 2 (unknown):
```unknown
pub struct RumbleSystem;
```

---

## Struct HotPatchChanges Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/struct.HotPatchChanges.html

**Contents:**
- Struct HotPatchChanges Copy item path
- Trait Implementations§
  - impl Default for HotPatchChanges
    - fn default() -> HotPatchChanges
  - impl Resource for HotPatchChangeswhere HotPatchChanges: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for HotPatchChanges
  - impl RefUnwindSafe for HotPatchChanges
  - impl Send for HotPatchChanges
  - impl Sync for HotPatchChanges

Resource which “changes” when a hotpatch happens.

Exists solely for change-detection, which allows systems to know whether a hotpatch happened even if they only run irregularily and would miss the event.

Used by Executors and other places which run systems System::refresh_hotpatch only when necessary.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct HotPatchChanges;
```

---

## Module reflect Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/reflect/index.html

**Contents:**
- Module reflect Copy item path
- Structs§
- Traits§
- Functions§

Types that enable reflection support.

---

## Type Alias InputFocusSet Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input_focus/type.InputFocusSet.html

**Contents:**
- Type Alias InputFocusSet Copy item path
- Aliased Type§
- Variants§
  - Dispatch

Deprecated alias for InputFocusSystems.

System which dispatches bubbled input events to the focused entity, or to the primary window.

**Examples:**

Example 1 (unknown):
```unknown
pub type InputFocusSet = InputFocusSystems;
```

Example 2 (unknown):
```unknown
pub enum InputFocusSet {
    Dispatch,
}
```

---

## Struct AudioPlayer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/struct.AudioPlayer.html

**Contents:**
- Struct AudioPlayer Copy item path
- Tuple Fields§
- Implementations§
  - impl AudioPlayer
    - pub fn new(source: Handle<AudioSource>) -> AudioPlayer
      - Examples found in repository?
- Trait Implementations§
  - impl<Source> Clone for AudioPlayer<Source>where Source: Asset + Decodable,
    - fn clone(&self) -> AudioPlayer<Source>
    - fn clone_from(&mut self, source: &Self)

A component for playing a sound.

Insert this component onto an entity to trigger an audio source to begin playing.

If the handle refers to an unavailable asset (such as if it has not finished loading yet), the audio will not begin playing immediately. The audio will play when the asset is ready.

When Bevy begins the audio playback, an AudioSink component will be added to the entity. You can use that component to control the audio settings during playback.

Playback can be configured using the PlaybackSettings component. Note that changes to the PlaybackSettings component will not affect already-playing audio.

Creates a new AudioPlayer with the given Handle<AudioSource>.

For convenience reasons, this hard-codes the AudioSource type. If you want to initialize an AudioPlayer with a different type, just initialize it directly using normal tuple struct syntax.

Required Components: PlaybackSettings.

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components of the required components, recursively, in depth-first order.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AudioPlayer<Source = AudioSource>(pub Handle<Source>)
where
    Source: Asset + Decodable;
```

Example 2 (unknown):
```unknown
13fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
14    commands.spawn(AudioPlayer::new(
15        asset_server.load("sounds/Windless Slopes.ogg"),
16    ));
17}
```

Example 3 (unknown):
```unknown
16fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
17    commands.spawn((
18        AudioPlayer::new(asset_server.load("sounds/Windless Slopes.ogg")),
19        MyMusic,
20    ));
21
22    commands.spawn((
23        Text::new(""),
24        Node {
25            position_type: PositionType::Absolute,
26            top: px(12),
27            left: px(12),
28            ..default()
29        },
30        ProgressText,
31    ));
32
33    // example instructions
34    commands.spawn((
35        Text::new("-/=: Volume Down/Up\nSpace: Toggle Playback\nM: Toggle Mute"),
36        Nod
...
```

Example 4 (javascript):
```javascript
26fn setup(
27    mut commands: Commands,
28    mut meshes: ResMut<Assets<Mesh>>,
29    mut materials: ResMut<Assets<ColorMaterial>>,
30    asset_server: Res<AssetServer>,
31) {
32    // Space between the two ears
33    let gap = 400.0;
34
35    // sound emitter
36    commands.spawn((
37        Mesh2d(meshes.add(Circle::new(15.0))),
38        MeshMaterial2d(materials.add(Color::from(BLUE))),
39        Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)),
40        Emitter::default(),
41        AudioPlayer::new(asset_server.load("sounds/Windless Slopes.ogg")),
42        PlaybackSettings::LOOP
...
```

---

## Module gpu_component_array_buffer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/gpu_component_array_buffer/index.html

**Contents:**
- Module gpu_component_array_buffer Copy item path
- Structs§

bevy::renderModule gpu_component_array_buffer Copy item pathSource Structs§GpuComponentArrayBufferPluginThis plugin prepares the components of the corresponding type for the GPU by storing them in a GpuArrayBuffer.

---

## Struct RenderMeshInstancesCpu Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.RenderMeshInstancesCpu.html

**Contents:**
- Struct RenderMeshInstancesCpu Copy item path
- Methods from Deref<Target = HashMap<MainEntity, RenderMeshInstanceCpu, EntityHash>>§
    - pub fn hasher(&self) -> &S
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn keys(&self) -> Keys<'_, K, V> ⓘ
      - §Examples
      - Examples found in repository?
    - pub fn values(&self) -> Values<'_, K, V> ⓘ
      - §Examples

Information that the render world keeps about each entity that contains a mesh, when using CPU mesh instance data building.

Returns a reference to the map’s BuildHasher, or S parameter.

Refer to hasher for further details.

Returns the number of elements the map can hold without reallocating.

Refer to capacity for further details.

An iterator visiting all keys in arbitrary order. The iterator element type is &'a K.

Refer to keys for further details.

An iterator visiting all values in arbitrary order. The iterator element type is &'a V.

Refer to values for further details.

An iterator visiting all values mutably in arbitrary order. The iterator element type is &'a mut V.

Refer to values for further details.

An iterator visiting all key-value pairs in arbitrary order. The iterator element type is (&'a K, &'a V).

Refer to iter for further details.

An iterator visiting all key-value pairs in arbitrary order, with mutable references to the values. The iterator element type is (&'a K, &'a mut V).

Refer to iter_mut for further details.

Returns the number of elements in the map.

Refer to len for further details.

Returns true if the map contains no elements.

Refer to is_empty for further details.

Clears the map, returning all key-value pairs as an iterator. Keeps the allocated memory for reuse.

Refer to drain for further details.

Retains only the elements specified by the predicate. Keeps the allocated memory for reuse.

Refer to retain for further details.

Drains elements which are true under the given predicate, and returns an iterator over the removed items.

Refer to extract_if for further details.

Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.

Refer to clear for further details.

Reserves capacity for at least additional more elements to be inserted in the HashMap. The collection may reserve more space to avoid frequent reallocations.

Refer to reserve for further details.

Tries to reserve capacity for at least additional more elements to be inserted in the given HashMap<K,V>. The collection may reserve more space to avoid frequent reallocations.

Refer to try_reserve for further details.

Shrinks the capacity of the map as much as possible. It will drop down as much as possible while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.

Refer to shrink_to_fit for further details.

Shrinks the capacity of the map with a lower limit. It will drop down no

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderMeshInstancesCpu(/* private fields */);
```

Example 2 (javascript):
```javascript
let map = HashMap::with_capacity(5);

assert!(map.capacity() >= 5);
```

Example 3 (javascript):
```javascript
let mut map = HashMap::new();

map.insert("foo", 0);
map.insert("bar", 1);
map.insert("baz", 2);

for key in map.keys() {
    // foo, bar, baz
    // Note that the above order is not guaranteed
}
```

Example 4 (javascript):
```javascript
25fn send_scroll_events(
26    mut mouse_wheel_reader: MessageReader<MouseWheel>,
27    hover_map: Res<HoverMap>,
28    keyboard_input: Res<ButtonInput<KeyCode>>,
29    mut commands: Commands,
30) {
31    for mouse_wheel in mouse_wheel_reader.read() {
32        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);
33
34        if mouse_wheel.unit == MouseScrollUnit::Line {
35            delta *= LINE_HEIGHT;
36        }
37
38        if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
39            std::mem::swap(&mut delta.x, &mut delta.y);
40        }
41
42     
...
```

---

## Module change_detection Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/change_detection/index.html

**Contents:**
- Module change_detection Copy item path
- Structs§
- Constants§
- Traits§

Types that detect when their internal data mutate.

---

## Module error Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/error/index.html

**Contents:**
- Module error Copy item path
- Structs§
- Enums§
- Traits§
- Functions§
- Type Aliases§

Error handling for Bevy systems, commands, and observers.

When a system is added to a Schedule, and its return type is that of Result, then Bevy considers those systems to be “fallible”, and the ECS scheduler will special-case the Err variant of the returned Result.

All BevyErrors returned by a system, observer or command are handled by an “error handler”. By default, the panic error handler function is used, resulting in a panic with the error message attached.

You can change the default behavior by registering a custom error handler: Use DefaultErrorHandler to set a custom error handler function for a world, or App::set_error_handler for a whole app. In practice, this is generally feature-flagged: panicking or loudly logging errors in development, and quietly logging or ignoring them in production to avoid crashing the app.

Bevy provides a number of pre-built error-handlers for you to use:

However, you can use any custom error handler logic by providing your own function (or non-capturing closure that coerces to the function signature) as long as it matches the signature:

The ErrorContext allows you to access additional details relevant to providing context surrounding the error – such as the system’s name – in your error messages.

If you need special handling of individual fallible systems, you can use Bevy’s system piping feature to capture the Result output of the system and handle it accordingly.

When working with commands, you can handle the result of each command separately using the HandleError::handle_error_with method.

**Examples:**

Example 1 (unknown):
```unknown
fn(BevyError, ErrorContext)
```

Example 2 (javascript):
```javascript
use bevy_ecs::error::{BevyError, ErrorContext, DefaultErrorHandler};
use log::trace;

fn my_error_handler(error: BevyError, ctx: ErrorContext) {
   if ctx.name().ends_with("plz_ignore") {
      trace!("Nothing to see here, move along.");
      return;
  }
  bevy_ecs::error::error(error, ctx);
}

fn main() {
    let mut world = World::new();
    world.insert_resource(DefaultErrorHandler(my_error_handler));
    // Use your world here
}
```

---

## Struct RenderMeshInstancesGpu Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.RenderMeshInstancesGpu.html

**Contents:**
- Struct RenderMeshInstancesGpu Copy item path
- Methods from Deref<Target = HashMap<MainEntity, RenderMeshInstanceGpu, EntityHash>>§
    - pub fn hasher(&self) -> &S
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn keys(&self) -> Keys<'_, K, V> ⓘ
      - §Examples
      - Examples found in repository?
    - pub fn values(&self) -> Values<'_, K, V> ⓘ
      - §Examples

Information that the render world keeps about each entity that contains a mesh, when using GPU mesh instance data building.

Returns a reference to the map’s BuildHasher, or S parameter.

Refer to hasher for further details.

Returns the number of elements the map can hold without reallocating.

Refer to capacity for further details.

An iterator visiting all keys in arbitrary order. The iterator element type is &'a K.

Refer to keys for further details.

An iterator visiting all values in arbitrary order. The iterator element type is &'a V.

Refer to values for further details.

An iterator visiting all values mutably in arbitrary order. The iterator element type is &'a mut V.

Refer to values for further details.

An iterator visiting all key-value pairs in arbitrary order. The iterator element type is (&'a K, &'a V).

Refer to iter for further details.

An iterator visiting all key-value pairs in arbitrary order, with mutable references to the values. The iterator element type is (&'a K, &'a mut V).

Refer to iter_mut for further details.

Returns the number of elements in the map.

Refer to len for further details.

Returns true if the map contains no elements.

Refer to is_empty for further details.

Clears the map, returning all key-value pairs as an iterator. Keeps the allocated memory for reuse.

Refer to drain for further details.

Retains only the elements specified by the predicate. Keeps the allocated memory for reuse.

Refer to retain for further details.

Drains elements which are true under the given predicate, and returns an iterator over the removed items.

Refer to extract_if for further details.

Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.

Refer to clear for further details.

Reserves capacity for at least additional more elements to be inserted in the HashMap. The collection may reserve more space to avoid frequent reallocations.

Refer to reserve for further details.

Tries to reserve capacity for at least additional more elements to be inserted in the given HashMap<K,V>. The collection may reserve more space to avoid frequent reallocations.

Refer to try_reserve for further details.

Shrinks the capacity of the map as much as possible. It will drop down as much as possible while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.

Refer to shrink_to_fit for further details.

Shrinks the capacity of the map with a lower limit. It will drop down no

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderMeshInstancesGpu(/* private fields */);
```

Example 2 (javascript):
```javascript
let map = HashMap::with_capacity(5);

assert!(map.capacity() >= 5);
```

Example 3 (javascript):
```javascript
let mut map = HashMap::new();

map.insert("foo", 0);
map.insert("bar", 1);
map.insert("baz", 2);

for key in map.keys() {
    // foo, bar, baz
    // Note that the above order is not guaranteed
}
```

Example 4 (javascript):
```javascript
25fn send_scroll_events(
26    mut mouse_wheel_reader: MessageReader<MouseWheel>,
27    hover_map: Res<HoverMap>,
28    keyboard_input: Res<ButtonInput<KeyCode>>,
29    mut commands: Commands,
30) {
31    for mouse_wheel in mouse_wheel_reader.read() {
32        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);
33
34        if mouse_wheel.unit == MouseScrollUnit::Line {
35            delta *= LINE_HEIGHT;
36        }
37
38        if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
39            std::mem::swap(&mut delta.x, &mut delta.y);
40        }
41
42     
...
```

---

## Module never Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/never/index.html

**Contents:**
- Module never Copy item path
- Type Aliases§

A workaround for the ! type in stable Rust.

This approach is taken from the never_say_never crate, reimplemented here to avoid adding a new dependency.

This module exists due to a change in never type fallback inference in the Rust 2024 edition. This caused failures in bevy_ecs’s traits which are implemented for functions (like System) when working with panicking closures.

Note that using this hack is not recommended in general; by doing so you are knowingly opting out of rustc’s stability guarantees. Code that compiles due to this hack may break in future versions of Rust.

Please read issue #18778 for an explanation of why Bevy has chosen to use this workaround.

---

## Type Alias InputSystem Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input/type.InputSystem.html

**Contents:**
- Type Alias InputSystem Copy item path
- Aliased Type§

Deprecated alias for InputSystems.

**Examples:**

Example 1 (unknown):
```unknown
pub type InputSystem = InputSystems;
```

Example 2 (unknown):
```unknown
pub struct InputSystem;
```

---

## Struct DefaultGltfImageSampler Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/struct.DefaultGltfImageSampler.html

**Contents:**
- Struct DefaultGltfImageSampler Copy item path
- Implementations§
  - impl DefaultGltfImageSampler
    - pub fn new(descriptor: &ImageSamplerDescriptor) -> DefaultGltfImageSampler
    - pub fn get(&self) -> ImageSamplerDescriptor
    - pub fn get_internal(&self) -> Arc<Mutex<ImageSamplerDescriptor>>
    - pub fn set(&self, descriptor: &ImageSamplerDescriptor)
- Trait Implementations§
  - impl Resource for DefaultGltfImageSamplerwhere DefaultGltfImageSampler: Send + Sync + 'static,
- Auto Trait Implementations§

Stores default ImageSamplerDescriptor in main world.

Creates a new DefaultGltfImageSampler.

Returns the current default ImageSamplerDescriptor.

Makes a clone of internal Arc pointer.

Intended only to be used by code with no access to ECS.

Replaces default ImageSamplerDescriptor.

Doesn’t apply to samplers already built on top of it, i.e. GltfLoader’s output. Assets need to manually be reloaded.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DefaultGltfImageSampler(/* private fields */);
```

---

## Module message Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/message/index.html

**Contents:**
- Module message Copy item path
- Structs§
- Enums§
- Traits§
- Functions§
- Type Aliases§
- Derive Macros§

Message functionality.

---

## Module intern Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/intern/index.html

**Contents:**
- Module intern Copy item path
- Structs§
- Traits§

Provides types used to statically intern immutable values.

Interning is a pattern used to save memory by deduplicating identical values, speed up code by shrinking the stack size of large types, and make comparisons for any type as fast as integers.

---

## Enum CursorIcon Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.CursorIcon.html

**Contents:**
- Enum CursorIcon Copy item path
- Variants§
  - Custom(CustomCursor)
  - System(SystemCursorIcon)
- Implementations§
  - impl CursorIcon
    - pub fn as_system(&self) -> Option<&SystemCursorIcon>
- Trait Implementations§
  - impl Clone for CursorIcon
    - fn clone(&self) -> CursorIcon

Insert into a window entity to set the cursor for that window.

System provided cursor icon.

Returns the system cursor icon if this is a system cursor.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum CursorIcon {
    Custom(CustomCursor),
    System(SystemCursorIcon),
}
```

---

## Type Alias CameraUpdateSystem Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/type.CameraUpdateSystem.html

**Contents:**
- Type Alias CameraUpdateSystem Copy item path
- Aliased Type§

Deprecated alias for CameraUpdateSystems.

**Examples:**

Example 1 (unknown):
```unknown
pub type CameraUpdateSystem = CameraUpdateSystems;
```

Example 2 (unknown):
```unknown
pub struct CameraUpdateSystem;
```

---

## Function scene_spawner_system Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/scene/fn.scene_spawner_system.html

**Contents:**
- Function scene_spawner_system Copy item path

System that handles scheduled scene instance spawning and despawning through a SceneSpawner.

**Examples:**

Example 1 (unknown):
```unknown
pub fn scene_spawner_system(world: &mut World)
```

---

## Struct EntityCountDiagnosticsPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/struct.EntityCountDiagnosticsPlugin.html

**Contents:**
- Struct EntityCountDiagnosticsPlugin Copy item path
- §See also
- Fields§
- Implementations§
  - impl EntityCountDiagnosticsPlugin
    - pub fn new(max_history_length: usize) -> EntityCountDiagnosticsPlugin
  - impl EntityCountDiagnosticsPlugin
    - pub const ENTITY_COUNT: DiagnosticPath
    - pub fn diagnostic_system(diagnostics: Diagnostics<'_, '_>, entities: &Entities)
- Trait Implementations§

Adds “entity count” diagnostic to an App.

LogDiagnosticsPlugin to output diagnostics to the console.

The total number of values to keep.

Creates a new EntityCountDiagnosticsPlugin with the specified max_history_length.

Number of currently allocated entities.

Updates entity count measurement.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct EntityCountDiagnosticsPlugin {
    pub max_history_length: usize,
}
```

---

## Module visibility Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/visibility/index.html

**Contents:**
- Module visibility Copy item path
- Structs§
- Enums§
- Constants§
- Functions§
- Type Aliases§

bevy::cameraModule visibility Copy item pathSource Structs§CascadesVisibleEntitiesCubemapVisibleEntitiesInheritedVisibilityWhether or not an entity is visible in the hierarchy. This will not be accurate until VisibilityPropagate runs in the PostUpdate schedule.NoCpuCullingNoFrustumCullingUse this component to opt-out of built-in frustum culling for entities, see Frustum.PreviousVisibleEntitiesStores all entities that were visible in the previous frame.RenderLayersDefines which rendering layers an entity belongs to.ViewVisibilityAlgorithmically-computed indication of whether an entity is visible and should be extracted for rendering.VisibilityClassA bucket into which we group entities for the purposes of visibility.VisibilityPluginVisibilityRangeSpecifies the range of distances that this entity must be from the camera in order to be rendered.VisibilityRangePluginA plugin that enables VisibilityRanges, which allow entities to be hidden or shown based on distance to the camera.VisibleEntitiesCollection of entities visible from the current view.VisibleEntityRangesStores which entities are in within the VisibilityRanges of views.VisibleMeshEntitiesCollection of mesh entities visible for 3D lighting.Enums§VisibilityUser indication of whether an entity is visible. Propagates down the entity hierarchy.VisibilitySystemsConstants§DEFAULT_LAYERSFunctions§add_visibility_classA generic component add hook that automatically adds the appropriate VisibilityClass to an entity.calculate_boundsComputes and adds an Aabb component to entities with a Mesh3d component and without a NoFrustumCulling component.check_visibilitySystem updating the visibility of entities each frame.check_visibility_rangesChecks all entities against all views in order to determine which entities with VisibilityRanges are potentially visible.update_frustaUpdates Frustum.Type Aliases§LayerAn identifier for a rendering layer.

---

## Trait QueryData Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/query/trait.QueryData.html

**Contents:**
- Trait QueryData Copy item path
- §Trait derivation
  - §Macro expansion
  - §Adding mutable references
  - §Adding methods to query items
  - §Deriving traits for query items
  - §Query composition
- §Generic Queries
- §Safety
- Required Associated Constants§

Types that can be fetched from a World using a Query.

There are many types that natively implement this trait:

Implementing the trait manually can allow for a fundamentally new type of behavior.

Query design can be easily structured by deriving QueryData for custom types. Despite the added complexity, this approach has several advantages over using QueryData tuples. The most relevant improvements are:

This trait can only be derived for structs, if each field also implements QueryData.

Expanding the macro will declare one or three additional structs, depending on whether or not the struct is marked as mutable. For a struct named X, the additional structs will be:

Simply adding mutable references to a derived QueryData will result in a compilation error:

To grant mutable access to components, the struct must be marked with the #[query_data(mutable)] attribute. This will also create three more structs that will be used for accessing the query immutably (see table above).

It is possible to add methods to query items in order to write reusable logic about related components. This will often make systems more readable because low level logic is moved out from them. It is done by adding impl blocks with methods for the -Item or -ReadOnlyItem generated structs.

The QueryData derive macro does not automatically implement the traits of the struct to the query item types. Something similar can be done by using the #[query_data(derive(...))] attribute. This will apply the listed derivable traits to the query item structs.

It is possible to use any QueryData as a field of another one. This means that a QueryData can also be used as a subquery, potentially in multiple places.

When writing generic code, it is often necessary to use PhantomData to constrain type parameters. Since QueryData is implemented for all PhantomData<T> types, this pattern can be used with this macro.

True if this query is read-only and may not perform mutable access.

The read-only variant of this QueryData, which satisfies the ReadOnlyQueryData trait.

The item returned by this WorldQuery This will be the data retrieved by the query, and is visible to the end user when calling e.g. Query<Self>::get.

This function manually implements subtyping for the query items.

Fetch Self::Item for either the given entity in the current Table, or for the given entity in the current Archetype. This must always be called after WorldQuery::set_table with a table_row in the range of the current Table 

*[Content truncated]*

**Examples:**

Example 1 (javascript):
```javascript
pub unsafe trait QueryData: WorldQuery {
    type ReadOnly: ReadOnlyQueryData<State = Self::State>;
    type Item<'w, 's>;

    const IS_READ_ONLY: bool;

    // Required methods
    fn shrink<'wlong, 'wshort, 's>(
        item: Self::Item<'wlong, 's>,
    ) -> Self::Item<'wshort, 's>
       where 'wlong: 'wshort;
    unsafe fn fetch<'w, 's>(
        state: &'s Self::State,
        fetch: &mut Self::Fetch<'w>,
        entity: Entity,
        table_row: TableRow,
    ) -> Self::Item<'w, 's>;

    // Provided method
    fn provide_extra_access(
        _state: &mut Self::State,
        _access: 
...
```

Example 2 (unknown):
```unknown
use bevy_ecs::query::QueryData;

#[derive(QueryData)]
struct MyQuery {
    entity: Entity,
    // It is required that all reference lifetimes are explicitly annotated, just like in any
    // struct. Each lifetime should be 'static.
    component_a: &'static ComponentA,
    component_b: &'static ComponentB,
}

fn my_system(query: Query<MyQuery>) {
    for q in &query {
        q.component_a;
    }
}
```

Example 3 (unknown):
```unknown
#[derive(QueryData)]
struct CustomQuery {
    component_a: &'static mut ComponentA,
}
```

Example 4 (unknown):
```unknown
#[derive(QueryData)]
#[query_data(mutable)]
struct CustomQuery {
    component_a: &'static mut ComponentA,
}
```

---

## Crate ptr Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/ptr/index.html

**Contents:**
- Crate ptr Copy item path
- §Bevy Pointer
  - §How to Build a Borrow (From Scratch)
  - §Standard Pointers
  - §Available in Nightly
  - §Available in bevy_ptr
- Macros§
- Structs§
- Traits§
- Functions§

Pointers in computer programming are objects that store a memory address. They’re a fundamental building block for constructing more complex data structures.

They’re also the definitive source of memory safety bugs: you can dereference a invalid (null) pointer, access a pointer after the underlying memory has been freed, and even ignore type safety and misread or mutate the underlying memory improperly.

Rust is a programming language that heavily relies on its types to enforce correctness, and by proxy, memory safety. As a result, Rust has an entire zoo of types for working with pointers, and a graph of safe and unsafe conversions that make working with them safer.

bevy_ptr is a crate that attempts to bridge the gap between the full blown unsafety of *mut () and the safe &'a T, allowing users to choose what invariants to uphold for their pointer, with the intent to enable building progressively safer abstractions.

Correctly and safety converting a pointer into a valid borrow is at the core of all unsafe code in Rust. Looking at the documentation for [(*const T)::as_ref], a pointer must satisfy all of the following conditions:

Note these rules aren’t final and are still in flux as the Rust Project hashes out what exactly are the pointer aliasing rules, but the expectation is that the final set of constraints are going to be a superset of this list, not a subset.

This list already is non-trivial to satisfy in isolation. Thankfully, the Rust core/standard library provides a progressive list of pointer types that help build these safety guarantees…

&T, &mut T, and Box<T> are by far the most common pointer types that Rust developers will see. They’re the only ones in this list that are entirely usable without the use of unsafe.

&UnsafeCell<T> is the first step away from safety. UnsafeCell is the only way to get a mutable borrow from an immutable one in the language, so it’s the base primitive for all interior mutability in the language: Cell<T>, RefCell<T>, Mutex<T>, RwLock<T>, etc. are all built on top of UnsafeCell<T>. To safety convert &UnsafeCell<T> into a &T or &mut T, the caller must guarantee that all simultaneous access follow Rust’s aliasing rules.

NonNull<T> takes quite a step down from the aforementioned types. In addition to allowing aliasing, it’s the first pointer type on this list to drop both lifetimes and the alignment guarantees of borrows. Its only guarantees are that the pointer is not null and that it points to a valid instance of 

*[Content truncated]*

---

## Trait ReadOnlySystemParam Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/system/trait.ReadOnlySystemParam.html

**Contents:**
- Trait ReadOnlySystemParam Copy item path
- §Safety
- Dyn Compatibility§
- Implementations on Foreign Types§
  - impl ReadOnlySystemParam for ()
  - impl<P> ReadOnlySystemParam for (P₁, P₂, …, Pₙ)where P: ReadOnlySystemParam,
  - impl<T> ReadOnlySystemParam for Option<T>where T: ReadOnlySystemParam,
  - impl<T> ReadOnlySystemParam for Result<T, SystemParamValidationError>where T: ReadOnlySystemParam,
  - impl<T> ReadOnlySystemParam for PhantomData<T>where T: ?Sized,
- Implementors§

A SystemParam that only reads a given World.

This must only be implemented for SystemParam impls that exclusively read the World passed in to SystemParam::get_param

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

This trait is implemented for tuples up to 17 items long.

SAFETY: only reads world

**Examples:**

Example 1 (unknown):
```unknown
pub unsafe trait ReadOnlySystemParam: SystemParam { }
```

---

## Module batching Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/batching/index.html

**Contents:**
- Module batching Copy item path
- Structs§

Types for controlling batching behavior during parallel processing.

---

## Enum InputFocusSystems Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input_focus/enum.InputFocusSystems.html

**Contents:**
- Enum InputFocusSystems Copy item path
- Variants§
  - Dispatch
- Trait Implementations§
  - impl Clone for InputFocusSystems
    - fn clone(&self) -> InputFocusSystems
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for InputFocusSystems
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Hash for InputFocusSystems

System sets for bevy_input_focus.

These systems run in the PreUpdate schedule.

System which dispatches bubbled input events to the focused entity, or to the primary window.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum InputFocusSystems {
    Dispatch,
}
```

---

## Struct WindowTraversalItem Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input_focus/struct.WindowTraversalItem.html

**Contents:**
- Struct WindowTraversalItem Copy item path
- Auto Trait Implementations§
  - impl<'__w, '__s> Freeze for WindowTraversalItem<'__w, '__s>
  - impl<'__w, '__s> RefUnwindSafe for WindowTraversalItem<'__w, '__s>
  - impl<'__w, '__s> Send for WindowTraversalItem<'__w, '__s>
  - impl<'__w, '__s> Sync for WindowTraversalItem<'__w, '__s>
  - impl<'__w, '__s> Unpin for WindowTraversalItem<'__w, '__s>
  - impl<'__w, '__s> UnwindSafe for WindowTraversalItem<'__w, '__s>
- Blanket Implementations§
  - impl<T> Any for Twhere T: 'static + ?Sized,

Automatically generated WorldQuery item type for WindowTraversal, returned when iterating over query results.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WindowTraversalItem<'__w, '__s> { /* private fields */ }
```

---

## Struct CameraUpdateSystems Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.CameraUpdateSystems.html

**Contents:**
- Struct CameraUpdateSystems Copy item path
- Trait Implementations§
  - impl Clone for CameraUpdateSystems
    - fn clone(&self) -> CameraUpdateSystems
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for CameraUpdateSystems
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Hash for CameraUpdateSystems
    - fn hash<__H>(&self, state: &mut __H)where __H: Hasher,
    - fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,

Label for camera_system<T>, shared across all T.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CameraUpdateSystems;
```

---

## Struct PrimaryWindow Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.PrimaryWindow.html

**Contents:**
- Struct PrimaryWindow Copy item path
- Trait Implementations§
  - impl Clone for PrimaryWindow
    - fn clone(&self) -> PrimaryWindow
    - fn clone_from(&mut self, source: &Self)
  - impl Component for PrimaryWindowwhere PrimaryWindow: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior

Marker Component for the window considered the primary window.

Currently this is assumed to only exist on 1 entity at a time.

WindowPlugin will spawn a Window entity with this component if primary_window is Some.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PrimaryWindow;
```

---

## Module entity Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/entity/index.html

**Contents:**
- Module entity Copy item path
- §Usage
- Modules§
- Structs§
- Traits§
- Type Aliases§
- Derive Macros§

Entity handling types.

An entity exclusively owns zero or more component instances, all of different types, and can dynamically acquire or lose them over its lifetime.

empty entity: Entity with zero components. pending entity: Entity reserved, but not flushed yet (see Entities::flush docs for reference). reserved entity: same as pending entity. invalid entity: pending entity flushed with invalid (see Entities::flush_as_invalid docs for reference).

See Entity to learn more.

Operations involving entities and their components are performed either from a system by submitting commands, or from the outside (or from an exclusive system) by directly using World methods:

---

## Module sync_component Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/sync_component/index.html

**Contents:**
- Module sync_component Copy item path
- Structs§

bevy::renderModule sync_component Copy item pathSource Structs§SyncComponentPluginPlugin that registers a component for automatic sync to the render world. See SyncWorldPlugin for more information.

---

## Struct AudioSink Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/struct.AudioSink.html

**Contents:**
- Struct AudioSink Copy item path
- Implementations§
  - impl AudioSink
    - pub fn new(sink: Sink) -> AudioSink
- Trait Implementations§
  - impl AudioSinkPlayback for AudioSink
    - fn volume(&self) -> Volume
    - fn set_volume(&mut self, volume: Volume)
    - fn speed(&self) -> f32
    - fn set_speed(&self, speed: f32)

Used to control audio during playback.

Bevy inserts this component onto your entities when it begins playing an audio source. Use AudioPlayer to trigger that to happen.

You can use this component to modify the playback settings while the audio is playing.

If this component is removed from an entity, and an AudioSource is attached to that entity, that AudioSource will start playing. If that source is unchanged, that translates to the audio restarting.

Create a new audio sink.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AudioSink { /* private fields */ }
```

---

## Struct TextReader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/text/struct.TextReader.html

**Contents:**
- Struct TextReader Copy item path
- Implementations§
  - impl<'w, 's, R> TextReader<'w, 's, R>where R: TextRoot,
    - pub fn iter(&mut self, root_entity: Entity) -> TextSpanIter<'_, R> ⓘ
    - pub fn get( &mut self, root_entity: Entity, index: usize, ) -> Option<(Entity, usize, &str, &TextFont, Color)>
    - pub fn get_text(&mut self, root_entity: Entity, index: usize) -> Option<&str>
    - pub fn get_font( &mut self, root_entity: Entity, index: usize, ) -> Option<&TextFont>
    - pub fn get_color(&mut self, root_entity: Entity, index: usize) -> Option<Color>
    - pub fn text(&mut self, root_entity: Entity, index: usize) -> &str
    - pub fn font(&mut self, root_entity: Entity, index: usize) -> &TextFont

System parameter for reading text spans in a text block.

R is the root text component.

Returns an iterator over text spans in a text block, starting with the root entity.

Gets a text span within a text block at a specific index in the flattened span list.

Gets the text value of a text span within a text block at a specific index in the flattened span list.

Gets the TextFont of a text span within a text block at a specific index in the flattened span list.

Gets the TextColor of a text span within a text block at a specific index in the flattened span list.

Gets the text value of a text span within a text block at a specific index in the flattened span list.

Panics if there is no span at the requested index.

Gets the TextFont of a text span within a text block at a specific index in the flattened span list.

Panics if there is no span at the requested index.

Gets the TextColor of a text span within a text block at a specific index in the flattened span list.

Panics if there is no span at the requested index.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct TextReader<'w, 's, R>where
    R: TextRoot,{ /* private fields */ }
```

---

## Struct RenderMeshInstanceCpu Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.RenderMeshInstanceCpu.html

**Contents:**
- Struct RenderMeshInstanceCpu Copy item path
- Fields§
- Methods from Deref<Target = RenderMeshInstanceShared>§
    - pub fn should_batch(&self) -> bool
      - Examples found in repository?
- Trait Implementations§
  - impl Deref for RenderMeshInstanceCpu
    - type Target = RenderMeshInstanceShared
    - fn deref(&self) -> &<RenderMeshInstanceCpu as Deref>::Target
  - impl DerefMut for RenderMeshInstanceCpu

CPU data that the render world keeps for each entity, when not using GPU mesh uniform building.

Data shared between both the CPU mesh uniform building and the GPU mesh uniform building paths.

The transform of the mesh.

This will be written into the MeshUniform at the appropriate time.

Returns true if this entity is eligible to participate in automatic batching.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderMeshInstanceCpu {
    pub shared: RenderMeshInstanceShared,
    pub transforms: MeshTransforms,
}
```

Example 2 (javascript):
```javascript
384    fn get_index_and_compare_data(
385        (mesh_instances, _, _): &SystemParamItem<Self::Param>,
386        main_entity: MainEntity,
387    ) -> Option<(NonMaxU32, Option<Self::CompareData>)> {
388        // This should only be called during GPU building.
389        let RenderMeshInstances::GpuBuilding(ref mesh_instances) = **mesh_instances else {
390            error!(
391                "`get_index_and_compare_data` should never be called in CPU mesh uniform building \
392                mode"
393            );
394            return None;
395        };
396        let mesh_instance = m
...
```

Example 3 (javascript):
```javascript
267fn queue_custom_mesh_pipeline(
268    pipeline_cache: Res<PipelineCache>,
269    custom_mesh_pipeline: Res<CustomMeshPipeline>,
270    (mut opaque_render_phases, opaque_draw_functions): (
271        ResMut<ViewBinnedRenderPhases<Opaque3d>>,
272        Res<DrawFunctions<Opaque3d>>,
273    ),
274    mut specialized_mesh_pipelines: ResMut<SpecializedMeshPipelines<CustomMeshPipeline>>,
275    views: Query<(&RenderVisibleEntities, &ExtractedView, &Msaa)>,
276    (render_meshes, render_mesh_instances): (
277        Res<RenderAssets<RenderMesh>>,
278        Res<RenderMeshInstances>,
279    ),
280 
...
```

---

## Struct AutoFocus Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input_focus/struct.AutoFocus.html

**Contents:**
- Struct AutoFocus Copy item path
- Trait Implementations§
  - impl Clone for AutoFocus
    - fn clone(&self) -> AutoFocus
    - fn clone_from(&mut self, source: &Self)
  - impl Component for AutoFocuswhere AutoFocus: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn on_add() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>

Indicates that this widget should automatically receive InputFocus.

This can be useful for things like dialog boxes, the first text input in a form, or the first button in a game menu.

The focus is swapped when this component is added or an entity with this component is spawned.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AutoFocus;
```

---

## Enum AnimationEvaluationError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/enum.AnimationEvaluationError.html

**Contents:**
- Enum AnimationEvaluationError Copy item path
- Variants§
  - ComponentNotPresent(TypeId)
  - PropertyNotPresent(TypeId)
  - InconsistentEvaluatorImplementation(TypeId)
- Trait Implementations§
  - impl Clone for AnimationEvaluationError
    - fn clone(&self) -> AnimationEvaluationError
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for AnimationEvaluationError

Why Bevy failed to evaluate an animation.

The component to be animated isn’t present on the animation target.

To fix this error, make sure the entity to be animated contains all components that have animation curves.

The component to be animated was present, but the property on the component wasn’t present.

An internal error occurred in the implementation of AnimationCurveEvaluator.

You shouldn’t ordinarily see this error unless you implemented AnimationCurveEvaluator yourself. The contained TypeId is the ID of the curve evaluator.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum AnimationEvaluationError {
    ComponentNotPresent(TypeId),
    PropertyNotPresent(TypeId),
    InconsistentEvaluatorImplementation(TypeId),
}
```

---

## Module mesh Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/mesh/index.html

**Contents:**
- Module mesh Copy item path
- Modules§
- Structs§
- Enums§
- Functions§

bevy::renderModule mesh Copy item pathSource Modules§allocatorManages mesh vertex and index buffers.Structs§MeshRenderAssetPluginMakes sure that Meshes are extracted and prepared for the GPU. Does not add the Mesh as an asset. Use MeshPlugin for that.MorphPluginInherit weights from glTF mesh parent entity to direct bevy mesh child entities (ie: glTF primitive).RenderMeshThe render world representation of a Mesh.Enums§RenderMeshBufferInfoThe index/vertex buffer info of a RenderMesh.Functions§inherit_weightsBevy meshes are gltf primitives, MorphWeights on the bevy node entity should be inherited by children meshes.

---

## Module traversal Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/traversal/index.html

**Contents:**
- Module traversal Copy item path
- Traits§

A trait for components that let you traverse the ECS.

---

## Struct RenderMeshInstanceGpu Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.RenderMeshInstanceGpu.html

**Contents:**
- Struct RenderMeshInstanceGpu Copy item path
- Fields§
- Methods from Deref<Target = RenderMeshInstanceShared>§
    - pub fn should_batch(&self) -> bool
      - Examples found in repository?
- Trait Implementations§
  - impl Deref for RenderMeshInstanceGpu
    - type Target = RenderMeshInstanceShared
    - fn deref(&self) -> &<RenderMeshInstanceGpu as Deref>::Target
  - impl DerefMut for RenderMeshInstanceGpu

CPU data that the render world needs to keep for each entity that contains a mesh when using GPU mesh uniform building.

Data shared between both the CPU mesh uniform building and the GPU mesh uniform building paths.

The translation of the mesh.

This is the only part of the transform that we have to keep on CPU (for distance sorting).

The index of the MeshInputUniform in the buffer.

Returns true if this entity is eligible to participate in automatic batching.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderMeshInstanceGpu {
    pub shared: RenderMeshInstanceShared,
    pub translation: Vec3,
    pub current_uniform_index: NonMaxU32,
}
```

Example 2 (javascript):
```javascript
384    fn get_index_and_compare_data(
385        (mesh_instances, _, _): &SystemParamItem<Self::Param>,
386        main_entity: MainEntity,
387    ) -> Option<(NonMaxU32, Option<Self::CompareData>)> {
388        // This should only be called during GPU building.
389        let RenderMeshInstances::GpuBuilding(ref mesh_instances) = **mesh_instances else {
390            error!(
391                "`get_index_and_compare_data` should never be called in CPU mesh uniform building \
392                mode"
393            );
394            return None;
395        };
396        let mesh_instance = m
...
```

Example 3 (javascript):
```javascript
267fn queue_custom_mesh_pipeline(
268    pipeline_cache: Res<PipelineCache>,
269    custom_mesh_pipeline: Res<CustomMeshPipeline>,
270    (mut opaque_render_phases, opaque_draw_functions): (
271        ResMut<ViewBinnedRenderPhases<Opaque3d>>,
272        Res<DrawFunctions<Opaque3d>>,
273    ),
274    mut specialized_mesh_pipelines: ResMut<SpecializedMeshPipelines<CustomMeshPipeline>>,
275    views: Query<(&RenderVisibleEntities, &ExtractedView, &Msaa)>,
276    (render_meshes, render_mesh_instances): (
277        Res<RenderAssets<RenderMesh>>,
278        Res<RenderMeshInstances>,
279    ),
280 
...
```

---

## Enum RenderSystems Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/enum.RenderSystems.html

**Contents:**
- Enum RenderSystems Copy item path
- Variants§
  - ExtractCommands
  - PrepareAssets
  - PrepareMeshes
  - ManageViews
  - Queue
  - QueueMeshes
  - QueueSweep
  - PhaseSort

The systems sets of the default App rendering schedule.

These can be useful for ordering, but you almost never want to add your systems to these sets.

This is used for applying the commands from the ExtractSchedule

Prepare assets that have been created/modified/removed this frame.

Prepares extracted meshes.

Create any additional views such as those used for shadow mapping.

Queue drawable entities as phase items in render phases ready for sorting (if necessary)

A sub-set within Queue where mesh entity queue systems are executed. Ensures prepare_assets::<RenderMesh> is completed.

A sub-set within Queue where meshes that have become invisible or changed phases are removed from the bins.

Sort the SortedRenderPhases and BinKeys here.

Prepare render resources from extracted data for the GPU based on their sorted order. Create BindGroups that depend on those data.

A sub-set within Prepare for initializing buffers, textures and uniforms for use in bind groups.

Collect phase buffers after PrepareResources has run.

Flush buffers after PrepareResources, but before PrepareBindGroups.

A sub-set within Prepare for constructing bind groups, or other data that relies on render resources prepared in PrepareResources.

Actual rendering happens here. In most cases, only the render backend should insert resources here.

Cleanup render resources here.

Final cleanup occurs: all entities will be despawned.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum RenderSystems {
Show 16 variants    ExtractCommands,
    PrepareAssets,
    PrepareMeshes,
    ManageViews,
    Queue,
    QueueMeshes,
    QueueSweep,
    PhaseSort,
    Prepare,
    PrepareResources,
    PrepareResourcesCollectPhaseBuffers,
    PrepareResourcesFlush,
    PrepareBindGroups,
    Render,
    Cleanup,
    PostCleanup,
}
```

---

## Struct DefaultQueryFilters Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/entity_disabling/struct.DefaultQueryFilters.html

**Contents:**
- Struct DefaultQueryFilters Copy item path
- §Warning
- Implementations§
  - impl DefaultQueryFilters
    - pub fn empty() -> DefaultQueryFilters
    - pub fn register_disabling_component(&mut self, component_id: ComponentId)
      - §Warning
    - pub fn disabling_ids(&self) -> impl Iterator<Item = ComponentId>
- Trait Implementations§
  - impl Debug for DefaultQueryFilters

Default query filters work by excluding entities with certain components from most queries.

If a query does not explicitly mention a given disabling component, it will not include entities with that component. To be more precise, this checks if the query’s FilteredAccess contains the component, and if it does not, adds a Without filter for that component to the query.

Allow and Has can be used to include entities with and without the disabling component. Allow is a QueryFilter and will simply change the list of shown entities, while Has is a QueryData and will allow you to see if each entity has the disabling component or not.

This resource is initialized in the World whenever a new world is created, with the Disabled component as a disabling component.

Note that you can remove default query filters by overwriting the DefaultQueryFilters resource. This can be useful as a last resort escape hatch, but is liable to break compatibility with other libraries.

See the module docs for more info.

Default query filters are a global setting that affects all queries in the World, and incur a small performance cost for each query.

They can cause significant interoperability issues within the ecosystem, as users must be aware of each disabling component in use.

Think carefully about whether you need to use a new disabling component, and clearly communicate their presence in any libraries you publish.

Creates a new, completely empty DefaultQueryFilters.

This is provided as an escape hatch; in most cases you should initialize this using FromWorld, which is automatically called when creating a new World.

Adds this ComponentId to the set of DefaultQueryFilters, causing entities with this component to be excluded from queries.

This method is idempotent, and will not add the same component multiple times.

This method should only be called before the app starts, as it will not affect queries initialized before it is called.

As discussed in the module docs, this can have performance implications, as well as create interoperability issues, and should be used with caution.

Get an iterator over all of the components which disable entities when present.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DefaultQueryFilters { /* private fields */ }
```

---

## Function update_spot_light_frusta Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/fn.update_spot_light_frusta.html

**Contents:**
- Function update_spot_light_frusta Copy item path

bevy::lightFunction update_spot_light_frusta Copy item pathSource pub fn update_spot_light_frusta( global_lights: Res<'_, GlobalVisibleClusterableObjects>, views: Query<'_, '_, (Entity, &GlobalTransform, &SpotLight, &mut Frustum), Or<(Changed<GlobalTransform>, Changed<SpotLight>)>>, )

**Examples:**

Example 1 (unknown):
```unknown
pub fn update_spot_light_frusta(
    global_lights: Res<'_, GlobalVisibleClusterableObjects>,
    views: Query<'_, '_, (Entity, &GlobalTransform, &SpotLight, &mut Frustum), Or<(Changed<GlobalTransform>, Changed<SpotLight>)>>,
)
```

---

## Module schedule Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/schedule/index.html

**Contents:**
- Module schedule Copy item path
- Modules§
- Structs§
- Enums§
- Traits§
- Type Aliases§
- Derive Macros§

Contains APIs for ordering systems and executing them on a World

---

## Struct RenderViewLightProbes Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.RenderViewLightProbes.html

**Contents:**
- Struct RenderViewLightProbes Copy item path
- Trait Implementations§
  - impl<C> Component for RenderViewLightProbes<C>where C: LightProbeComponent, RenderViewLightProbes<C>: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior
    - fn on_add() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_insert() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_replace() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>

A component, part of the render world, that stores the mapping from asset ID or IDs to the texture index in the appropriate binding arrays.

Cubemap textures belonging to environment maps are collected into binding arrays, and the index of each texture is presented to the shader for runtime lookup. 3D textures belonging to reflection probes are likewise collected into binding arrays, and the shader accesses the 3D texture by index.

This component is attached to each view in the render world, because each view may have a different set of light probes that it considers and therefore the texture indices are per-view.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderViewLightProbes<C>where
    C: LightProbeComponent,{ /* private fields */ }
```

---

## Struct FocusedInput Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input_focus/struct.FocusedInput.html

**Contents:**
- Struct FocusedInput Copy item path
- Fields§
- Trait Implementations§
  - impl<M> Clone for FocusedInput<M>where M: Clone + Message,
    - fn clone(&self) -> FocusedInput<M>
    - fn clone_from(&mut self, source: &Self)
  - impl<M> Component for FocusedInput<M>where M: Message + Clone, FocusedInput<M>: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

A bubble-able user input event that starts at the currently focused entity.

This event is normally dispatched to the current input focus entity, if any. If no entity has input focus, then the event is dispatched to the main window.

To set up your own bubbling input event, add the dispatch_focused_input::<MyEvent> system to your app, in the InputFocusSystems::Dispatch system set during PreUpdate.

The entity that has received focused input.

The underlying input message.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct FocusedInput<M>where
    M: Message + Clone,{
    pub focused_entity: Entity,
    pub input: M,
    /* private fields */
}
```

---

## Struct GeneratedEnvironmentMapLight Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.GeneratedEnvironmentMapLight.html

**Contents:**
- Struct GeneratedEnvironmentMapLight Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for GeneratedEnvironmentMapLight
    - fn clone(&self) -> GeneratedEnvironmentMapLight
    - fn clone_from(&mut self, source: &Self)
  - impl Component for GeneratedEnvironmentMapLightwhere GeneratedEnvironmentMapLight: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

A generated environment map that is filtered at runtime.

See bevy_pbr::light_probe::generate for detailed information.

Source cubemap to be filtered on the GPU, size must be a power of two.

Scale factor applied to the diffuse and specular light generated by this component. Expressed in cd/m² (candela per square meter).

World-space rotation applied to the cubemap.

Whether this light contributes diffuse lighting to meshes that already have baked lightmaps.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GeneratedEnvironmentMapLight {
    pub environment_map: Handle<Image>,
    pub intensity: f32,
    pub rotation: Quat,
    pub affects_lightmapped_mesh_diffuse: bool,
}
```

---

## Enum AccessibilitySystems Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/a11y/enum.AccessibilitySystems.html

**Contents:**
- Enum AccessibilitySystems Copy item path
- Variants§
  - Update
- Trait Implementations§
  - impl Clone for AccessibilitySystems
    - fn clone(&self) -> AccessibilitySystems
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for AccessibilitySystems
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for AccessibilitySystems

A system set relating to accessibility.

Helps run accessibility updates all at once.

Update the accessibility tree.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum AccessibilitySystems {
    Update,
}
```

---

## Module tab_navigation Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input_focus/tab_navigation/index.html

**Contents:**
- Module tab_navigation Copy item path
- Structs§
- Enums§
- Functions§

This module provides a framework for handling linear tab-key navigation in Bevy applications.

The rules of tabbing are derived from the HTML specification, and are as follows:

Tabbable entities must be descendants of a TabGroup entity, which is a component that marks a tree of entities as containing tabbable elements. The order of tab groups is determined by the TabGroup::order field, with lower orders being tabbed first. Modal tab groups are used for ui elements that should only tab within themselves, such as modal dialog boxes.

To enable automatic tabbing, add the TabNavigationPlugin and InputDispatchPlugin to your app. This will install a keyboard event observer on the primary window which automatically handles tab navigation for you.

Alternatively, if you want to have more control over tab navigation, or are using an input-action-mapping framework, you can use the [TabNavigation] system parameter directly instead. This object can be injected into your systems, and provides a navigate method which can be used to navigate between focusable entities.

---

## Type Alias AccessibilitySystem Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/a11y/type.AccessibilitySystem.html

**Contents:**
- Type Alias AccessibilitySystem Copy item path
- Aliased Type§
- Variants§
  - Update

Deprecated alias for AccessibilitySystems.

Update the accessibility tree.

**Examples:**

Example 1 (unknown):
```unknown
pub type AccessibilitySystem = AccessibilitySystems;
```

Example 2 (unknown):
```unknown
pub enum AccessibilitySystem {
    Update,
}
```

---

## Module world Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/world/index.html

**Contents:**
- Module world Copy item path
- Modules§
- Structs§
- Enums§
- Constants§
- Traits§
- Derive Macros§

Defines the World and APIs for accessing it directly.

---

## Enum PlaybackMode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/enum.PlaybackMode.html

**Contents:**
- Enum PlaybackMode Copy item path
- Variants§
  - Once
  - Loop
  - Despawn
  - Remove
- Trait Implementations§
  - impl Clone for PlaybackMode
    - fn clone(&self) -> PlaybackMode
    - fn clone_from(&mut self, source: &Self)

The way Bevy manages the sound playback.

Play the sound once. Do nothing when it ends.

Note: It is not possible to reuse an AudioPlayer after it has finished playing and the underlying AudioSink or SpatialAudioSink has been drained.

To replay a sound, the audio components provided by AudioPlayer must be removed and added again.

Repeat the sound forever.

Despawn the entity and its children when the sound finishes playing.

Remove the audio components from the entity, when the sound finishes playing.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum PlaybackMode {
    Once,
    Loop,
    Despawn,
    Remove,
}
```

---

## Function spot_light_world_from_view Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/fn.spot_light_world_from_view.html

**Contents:**
- Function spot_light_world_from_view Copy item path

Constructs a right-handed orthonormal basis with translation, using only the forward direction and translation of a given GlobalTransform.

This is a version of orthonormalize which also includes translation.

**Examples:**

Example 1 (unknown):
```unknown
pub fn spot_light_world_from_view(transform: &GlobalTransform) -> Affine3A
```

---

## Module sync_world Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/sync_world/index.html

**Contents:**
- Module sync_world Copy item path
- Structs§
- Type Aliases§

bevy::renderModule sync_world Copy item pathSource Structs§MainEntityComponent added on the render world entities to keep track of the corresponding main world entity.RenderEntityComponent added on the main world entities that are synced to the Render World in order to keep track of the corresponding render world entity.SyncToRenderWorldMarker component that indicates that its entity needs to be synchronized to the render world.SyncWorldPluginA plugin that synchronizes entities with SyncToRenderWorld between the main world and the render world.TemporaryRenderEntityMarker component that indicates that its entity needs to be despawned at the end of the frame.Type Aliases§MainEntityHashMapA HashMap pre-configured to use EntityHash hashing with a MainEntity.MainEntityHashSetA HashSet pre-configured to use EntityHash hashing with a MainEntity..

---

## Module bundle Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/bundle/index.html

**Contents:**
- Module bundle Copy item path
- Structs§
- Enums§
- Traits§
- Derive Macros§

Types for handling Bundles.

This module contains the Bundle trait and some other helper types.

---

## Struct IsFocusedHelper Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input_focus/struct.IsFocusedHelper.html

**Contents:**
- Struct IsFocusedHelper Copy item path
- Trait Implementations§
  - impl IsFocused for IsFocusedHelper<'_, '_>
    - fn is_focused(&self, entity: Entity) -> bool
    - fn is_focus_within(&self, entity: Entity) -> bool
    - fn is_focus_visible(&self, entity: Entity) -> bool
    - fn is_focus_within_visible(&self, entity: Entity) -> bool
  - impl SystemParam for IsFocusedHelper<'_, '_>
    - type State = FetchState
    - type Item<'w, 's> = IsFocusedHelper<'w, 's>

A system param that helps get information about the current focused entity.

When working with the entire World, consider using the IsFocused instead.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct IsFocusedHelper<'w, 's> { /* private fields */ }
```

---

## Enum LightEntity Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/enum.LightEntity.html

**Contents:**
- Enum LightEntity Copy item path
- Variants§
  - Directional
    - Fields
  - Point
    - Fields
  - Spot
    - Fields
- Trait Implementations§
  - impl Component for LightEntitywhere LightEntity: Send + Sync + 'static,

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum LightEntity {
    Directional {
        light_entity: Entity,
        cascade_index: usize,
    },
    Point {
        light_entity: Entity,
        face_index: usize,
    },
    Spot {
        light_entity: Entity,
    },
}
```

---

## Struct ComputedTextBlock Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/text/struct.ComputedTextBlock.html

**Contents:**
- Struct ComputedTextBlock Copy item path
- Implementations§
  - impl ComputedTextBlock
    - pub fn entities(&self) -> &[TextEntity]
    - pub fn needs_rerender(&self) -> bool
    - pub fn buffer(&self) -> &CosmicBuffer
- Trait Implementations§
  - impl Clone for ComputedTextBlock
    - fn clone(&self) -> ComputedTextBlock
    - fn clone_from(&mut self, source: &Self)

Computed information for a text block.

Automatically updated by 2d and UI text systems.

Accesses entities in this block.

Can be used to look up TextFont components for glyphs in TextLayoutInfo using the span_index stored there.

Indicates if the text needs to be refreshed in TextLayoutInfo.

Updated automatically by detect_text_needs_rerender and cleared by TextPipeline methods.

Accesses the underlying buffer which can be used for cosmic-text APIs such as accessing layout information or calculating a cursor position.

Mutable access is not offered because changes would be overwritten during the automated layout calculation. If you want to control the buffer contents manually or use the cosmic-text editor, then you need to not use TextLayout and instead manually implement the conversion to TextLayoutInfo.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ComputedTextBlock { /* private fields */ }
```

---

## Module entity_disabling Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/entity_disabling/index.html

**Contents:**
- Module entity_disabling Copy item path
  - §Default query filters
    - §Warnings
- Structs§

Disabled entities do not show up in queries unless the query explicitly mentions them.

Entities which are disabled in this way are not removed from the World, and their relationships remain intact. In many cases, you may want to disable entire trees of entities at once, using EntityCommands::insert_recursive.

While Bevy ships with a built-in Disabled component, you can also create your own disabling components, which will operate in the same way but can have distinct semantics.

In Bevy, entity disabling is implemented through the construction of a global “default query filter” resource. Queries which do not explicitly mention the disabled component will not include entities with that component. If an entity has multiple disabling components, it will only be included in queries that mention all of them.

For example, Query<&Position> will not include entities with the Disabled component, even if they have a Position component, but Query<&Position, With<Disabled>> or Query<(&Position, Has<Disabled>)> will see them.

The Allow query filter is designed to be used with default query filters, and ensures that the query will include entities both with and without the specified disabling component.

Entities with disabling components are still present in the World and can be accessed directly, using methods on World or Commands.

As default query filters are implemented through a resource, it’s possible to temporarily ignore any default filters by using World::resource_scope.

Currently, only queries for which the cache is built after enabling a default query filter will have entities with those components filtered. As a result, they should generally only be modified before the app starts.

Because filters are applied to all queries they can have performance implication for the enire World, especially when they cause queries to mix sparse and table components. See Query performance for more info.

Custom disabling components can cause significant interoperability issues within the ecosystem, as users must be aware of each disabling component in use. Libraries should think carefully about whether they need to use a new disabling component, and clearly communicate their presence to their users to avoid the new for library compatibility flags.

**Examples:**

Example 1 (javascript):
```javascript
use bevy_ecs::prelude::*;

// Our custom disabling component!
#[derive(Component, Clone)]
struct Prefab;

#[derive(Component)]
struct A;

let mut world = World::new();
world.register_disabling_component::<Prefab>();
world.spawn((A, Prefab));
world.spawn((A,));
world.spawn((A,));

let mut normal_query = world.query::<&A>();
assert_eq!(2, normal_query.iter(&world).count());

let mut prefab_query = world.query_filtered::<&A, With<Prefab>>();
assert_eq!(1, prefab_query.iter(&world).count());

let mut maybe_prefab_query = world.query::<(&A, Has<Prefab>)>();
assert_eq!(3, maybe_prefab_query.iter(&wo
...
```

Example 2 (javascript):
```javascript
use bevy_ecs::prelude::*;
use bevy_ecs::entity_disabling::{DefaultQueryFilters, Disabled};

let mut world = World::default();

#[derive(Component)]
struct CustomDisabled;

world.register_disabling_component::<CustomDisabled>();

world.spawn(Disabled);
world.spawn(CustomDisabled);

// resource_scope removes DefaultQueryFilters temporarily before re-inserting into the world.
world.resource_scope(|world: &mut World, _: Mut<DefaultQueryFilters>| {
    // within this scope, we can query like no components are disabled.
    assert_eq!(world.query::<&Disabled>().query(&world).count(), 1);
    assert_
...
```

---

## Struct WindowClosed Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.WindowClosed.html

**Contents:**
- Struct WindowClosed Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for WindowClosed
    - fn clone(&self) -> WindowClosed
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for WindowClosed
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for WindowClosed
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<WindowClosed, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,

An event that is sent whenever a window is closed. This will be sent when the window entity loses its Window component or is despawned.

Window that has been closed.

Note that this entity probably no longer exists by the time this event is received.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WindowClosed {
    pub window: Entity,
}
```

---

## Function check_point_light_mesh_visibility Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/fn.check_point_light_mesh_visibility.html

**Contents:**
- Function check_point_light_mesh_visibility Copy item path

bevy::lightFunction check_point_light_mesh_visibility Copy item pathSource pub fn check_point_light_mesh_visibility( visible_point_lights: Query<'_, '_, &VisibleClusterableObjects>, point_lights: Query<'_, '_, (&PointLight, &GlobalTransform, &CubemapFrusta, &mut CubemapVisibleEntities, Option<&RenderLayers>)>, spot_lights: Query<'_, '_, (&SpotLight, &GlobalTransform, &Frustum, &mut VisibleMeshEntities, Option<&RenderLayers>)>, visible_entity_query: Query<'_, '_, (Entity, &InheritedVisibility, &mut ViewVisibility, Option<&RenderLayers>, Option<&Aabb>, Option<&GlobalTransform>, Has<VisibilityRange>, Has<NoFrustumCulling>), (Without<NotShadowCaster>, Without<DirectionalLight>, With<Mesh3d>)>, visible_entity_ranges: Option<Res<'_, VisibleEntityRanges>>, previous_visible_entities: ResMut<'_, PreviousVisibleEntities>, cubemap_visible_entities_queue: Local<'_, Parallel<[Vec<Entity>; 6]>>, spot_visible_entities_queue: Local<'_, Parallel<Vec<Entity>>>, checked_lights: Local<'_, EntityHashSet>, )

**Examples:**

Example 1 (unknown):
```unknown
pub fn check_point_light_mesh_visibility(
    visible_point_lights: Query<'_, '_, &VisibleClusterableObjects>,
    point_lights: Query<'_, '_, (&PointLight, &GlobalTransform, &CubemapFrusta, &mut CubemapVisibleEntities, Option<&RenderLayers>)>,
    spot_lights: Query<'_, '_, (&SpotLight, &GlobalTransform, &Frustum, &mut VisibleMeshEntities, Option<&RenderLayers>)>,
    visible_entity_query: Query<'_, '_, (Entity, &InheritedVisibility, &mut ViewVisibility, Option<&RenderLayers>, Option<&Aabb>, Option<&GlobalTransform>, Has<VisibilityRange>, Has<NoFrustumCulling>), (Without<NotShadowCaster>, Wit
...
```

---

## Trait DynamicBundle Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/bundle/trait.DynamicBundle.html

**Contents:**
- Trait DynamicBundle Copy item path
- Required Associated Types§
    - type Effect
- Required Methods§
    - unsafe fn get_components( ptr: MovingPtr<'_, Self>, func: &mut impl FnMut(StorageType, OwningPtr<'_>), )
      - §Safety
    - unsafe fn apply_effect( ptr: MovingPtr<'_, MaybeUninit<Self>>, entity: &mut EntityWorldMut<'_>, )
      - §Safety
- Dyn Compatibility§
- Implementations on Foreign Types§

The parts from Bundle that don’t require statically knowing the components of the bundle.

An operation on the entity that happens after inserting this bundle.

Moves the components out of the bundle.

Applies the after-effects of spawning this bundle.

This is applied after all residual changes to the World, including flushing the internal command queue.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

This trait is implemented for tuples up to 16 items long.

**Examples:**

Example 1 (unknown):
```unknown
pub trait DynamicBundle: Sized {
    type Effect;

    // Required methods
    unsafe fn get_components(
        ptr: MovingPtr<'_, Self>,
        func: &mut impl FnMut(StorageType, OwningPtr<'_>),
    );
    unsafe fn apply_effect(
        ptr: MovingPtr<'_, MaybeUninit<Self>>,
        entity: &mut EntityWorldMut<'_>,
    );
}
```

---

## Module render_asset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/render_asset/index.html

**Contents:**
- Module render_asset Copy item path
- Structs§
- Enums§
- Traits§
- Functions§
- Type Aliases§

bevy::renderModule render_asset Copy item pathSource Structs§AssetExtractionSystemsThe system set during which we extract modified assets to the render world.ExtractedAssetsTemporarily stores the extracted and removed assets of the current frame.PrepareNextFrameAssetsAll assets that should be prepared next frame.RenderAssetBytesPerFrameA resource that defines the amount of data allowed to be transferred from CPU to GPU each frame, preventing choppy frames at the cost of waiting longer for GPU assets to become available.RenderAssetBytesPerFrameLimiterA render-world resource that facilitates limiting the data transferred from CPU to GPU each frame, preventing choppy frames at the cost of waiting longer for GPU assets to become available.RenderAssetPluginThis plugin extracts the changed assets from the “app world” into the “render world” and prepares them for the GPU. They can then be accessed from the RenderAssets resource.RenderAssetsStores all GPU representations (RenderAsset) of RenderAsset::SourceAsset as long as they exist.Enums§PrepareAssetErrorTraits§RenderAssetDescribes how an asset gets extracted and prepared for rendering.RenderAssetDependencyFunctions§extract_render_asset_bytes_per_frameprepare_assetsThis system prepares all assets of the corresponding RenderAsset::SourceAsset type which where extracted this frame for the GPU.reset_render_asset_bytes_per_frameType Aliases§ExtractAssetsSetDeprecatedDeprecated alias for AssetExtractionSystems.

---

## Struct PlaybackSettings Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/struct.PlaybackSettings.html

**Contents:**
- Struct PlaybackSettings Copy item path
- Fields§
- Implementations§
  - impl PlaybackSettings
    - pub const ONCE: PlaybackSettings
    - pub const LOOP: PlaybackSettings
    - pub const DESPAWN: PlaybackSettings
    - pub const REMOVE: PlaybackSettings
    - pub const fn paused(self) -> PlaybackSettings
    - pub const fn muted(self) -> PlaybackSettings

Initial settings to be used when audio starts playing.

If you would like to control the audio while it is playing, query for the AudioSink or SpatialAudioSink components. Changes to this component will not be applied to already-playing audio.

The desired playback behavior.

Create the sink in paused state. Useful for “deferred playback”, if you want to prepare the entity, but hear the sound later.

Whether to create the sink in muted state or not.

This is useful for audio that should be initially muted. You can still set the initial volume and it is applied when the audio is unmuted.

Enables spatial audio for this source.

See also: SpatialListener.

Note: Bevy does not currently support HRTF or any other high-quality 3D sound rendering features. Spatial audio is implemented via simple left-right stereo panning.

Optional scale factor applied to the positions of this audio source and the listener, overriding the default value configured on AudioPlugin::default_spatial_scale.

The point in time in the audio clip where playback should start. If set to None, it will play from the beginning of the clip.

If the playback mode is set to Loop, each loop will start from this position.

How long the audio should play before stopping. If set, the clip will play for at most the specified duration. If set to None, it will play for as long as it can.

If the playback mode is set to Loop, each loop will last for this duration.

Will play the associated audio source once.

Note: It is not possible to reuse an AudioPlayer after it has finished playing and the underlying AudioSink or SpatialAudioSink has been drained.

To replay a sound, the audio components provided by AudioPlayer must be removed and added again.

Will play the associated audio source in a loop.

Will play the associated audio source once and despawn the entity afterwards.

Will play the associated audio source once and remove the audio components afterwards.

Helper to start in a paused state.

Helper to start muted.

Helper to set the volume from start of playback.

Helper to set the speed from start of playback.

Helper to enable or disable spatial audio.

Helper to use a custom spatial scale.

Helper to use a custom playback start position.

Helper to use a custom playback duration.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PlaybackSettings {
    pub mode: PlaybackMode,
    pub volume: Volume,
    pub speed: f32,
    pub paused: bool,
    pub muted: bool,
    pub spatial: bool,
    pub spatial_scale: Option<SpatialScale>,
    pub start_position: Option<Duration>,
    pub duration: Option<Duration>,
}
```

Example 2 (javascript):
```javascript
26fn setup(
27    mut commands: Commands,
28    mut meshes: ResMut<Assets<Mesh>>,
29    mut materials: ResMut<Assets<ColorMaterial>>,
30    asset_server: Res<AssetServer>,
31) {
32    // Space between the two ears
33    let gap = 400.0;
34
35    // sound emitter
36    commands.spawn((
37        Mesh2d(meshes.add(Circle::new(15.0))),
38        MeshMaterial2d(materials.add(Color::from(BLUE))),
39        Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)),
40        Emitter::default(),
41        AudioPlayer::new(asset_server.load("sounds/Windless Slopes.ogg")),
42        PlaybackSettings::LOOP
...
```

Example 3 (javascript):
```javascript
18fn setup(
19    mut commands: Commands,
20    asset_server: Res<AssetServer>,
21    mut meshes: ResMut<Assets<Mesh>>,
22    mut materials: ResMut<Assets<StandardMaterial>>,
23) {
24    // Space between the two ears
25    let gap = 4.0;
26
27    // sound emitter
28    commands.spawn((
29        Mesh3d(meshes.add(Sphere::new(0.2).mesh().uv(32, 18))),
30        MeshMaterial3d(materials.add(Color::from(BLUE))),
31        Transform::from_xyz(0.0, 0.0, 0.0),
32        Emitter::default(),
33        AudioPlayer::new(asset_server.load("sounds/Windless Slopes.ogg")),
34        PlaybackSettings::LOOP.w
...
```

---

## Enum ViewportConversionError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/enum.ViewportConversionError.html

**Contents:**
- Enum ViewportConversionError Copy item path
- Variants§
  - NoViewportSize
  - PastNearPlane
  - PastFarPlane
  - InvalidData
- Trait Implementations§
  - impl Clone for ViewportConversionError
    - fn clone(&self) -> ViewportConversionError
    - fn clone_from(&mut self, source: &Self)

Error returned when a conversion between world-space and viewport-space coordinates fails.

See world_to_viewport and viewport_to_world.

The pre-computed size of the viewport was not available.

This may be because the Camera was just created and camera_system has not been executed yet, or because the RenderTarget is misconfigured in one of the following ways:

The computed coordinate was beyond the Camera’s near plane.

Only applicable when converting from world-space to viewport-space.

The computed coordinate was beyond the Camera’s far plane.

Only applicable when converting from world-space to viewport-space.

The Normalized Device Coordinates could not be computed because the camera_transform, the world_position, or the projection matrix defined by Projection contained NAN (see world_to_ndc and ndc_to_world).

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ViewportConversionError {
    NoViewportSize,
    PastNearPlane,
    PastFarPlane,
    InvalidData,
}
```

---

## Trait IsFocused Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input_focus/trait.IsFocused.html

**Contents:**
- Trait IsFocused Copy item path
- Required Methods§
    - fn is_focused(&self, entity: Entity) -> bool
    - fn is_focus_within(&self, entity: Entity) -> bool
    - fn is_focus_visible(&self, entity: Entity) -> bool
    - fn is_focus_within_visible(&self, entity: Entity) -> bool
- Implementors§
  - impl IsFocused for World
  - impl IsFocused for IsFocusedHelper<'_, '_>

Trait which defines methods to check if an entity currently has focus.

This is implemented for World and IsFocusedHelper. DeferredWorld indirectly implements it through Deref.

For use within systems, use IsFocusedHelper.

Modify the InputFocus resource to change the focused entity.

Returns true if the given entity has input focus.

Returns true if the given entity or any of its descendants has input focus.

Note that for unusual layouts, the focus may not be within the entity’s visual bounds.

Returns true if the given entity has input focus and the focus indicator should be visible.

Returns true if the given entity, or any descendant, has input focus and the focus indicator should be visible.

**Examples:**

Example 1 (unknown):
```unknown
pub trait IsFocused {
    // Required methods
    fn is_focused(&self, entity: Entity) -> bool;
    fn is_focus_within(&self, entity: Entity) -> bool;
    fn is_focus_visible(&self, entity: Entity) -> bool;
    fn is_focus_within_visible(&self, entity: Entity) -> bool;
}
```

---

## Macro define_label Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/macro.define_label.html

**Contents:**
- Macro define_label Copy item path
- §Example

Macro to define a new label trait

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! define_label {
    (
        $(#[$label_attr:meta])*
        $label_trait_name:ident,
        $interner_name:ident
    ) => { ... };
    (
        $(#[$label_attr:meta])*
        $label_trait_name:ident,
        $interner_name:ident,
        extra_methods: { $($trait_extra_methods:tt)* },
        extra_methods_impl: { $($interned_extra_methods_impl:tt)* }
    ) => { ... };
}
```

Example 2 (unknown):
```unknown
define_label!(
    /// Documentation of label trait
    MyNewLabelTrait,
    MY_NEW_LABEL_TRAIT_INTERNER
);

define_label!(
    /// Documentation of another label trait
    MyNewExtendedLabelTrait,
    MY_NEW_EXTENDED_LABEL_TRAIT_INTERNER,
    extra_methods: {
        // Extra methods for the trait can be defined here
        fn additional_method(&self) -> i32;
    },
    extra_methods_impl: {
        // Implementation of the extra methods for Interned<dyn MyNewExtendedLabelTrait>
        fn additional_method(&self) -> i32 {
            0
        }
    }
);
```

---

## Function extract_meshes_for_cpu_building Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/fn.extract_meshes_for_cpu_building.html

**Contents:**
- Function extract_meshes_for_cpu_building Copy item path

Extracts meshes from the main world into the render world, populating the RenderMeshInstances.

This is the variant of the system that runs when we’re not using GPU MeshUniform building.

**Examples:**

Example 1 (unknown):
```unknown
pub fn extract_meshes_for_cpu_building(
    render_mesh_instances: ResMut<'_, RenderMeshInstances>,
    mesh_material_ids: Res<'_, RenderMaterialInstances>,
    render_material_bindings: Res<'_, RenderMaterialBindings>,
    render_visibility_ranges: Res<'_, RenderVisibilityRanges>,
    render_mesh_instance_queues: Local<'_, Parallel<Vec<(Entity, RenderMeshInstanceCpu)>>>,
    meshes_query: Extract<'_, '_, Query<'_, '_, (Entity, &ViewVisibility, &GlobalTransform, Option<&PreviousGlobalTransform>, &Mesh3d, Option<&MeshTag>, Has<NoFrustumCulling>, Has<NotShadowReceiver>, Has<TransmittedShadowRece
...
```

---

## Struct EntitySpecializationTicks Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.EntitySpecializationTicks.html

**Contents:**
- Struct EntitySpecializationTicks Copy item path
- Fields§
- Methods from Deref<Target = HashMap<MainEntity, Tick, EntityHash>>§
    - pub fn hasher(&self) -> &S
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn keys(&self) -> Keys<'_, K, V> ⓘ
      - §Examples
      - Examples found in repository?
    - pub fn values(&self) -> Values<'_, K, V> ⓘ

Returns a reference to the map’s BuildHasher, or S parameter.

Refer to hasher for further details.

Returns the number of elements the map can hold without reallocating.

Refer to capacity for further details.

An iterator visiting all keys in arbitrary order. The iterator element type is &'a K.

Refer to keys for further details.

An iterator visiting all values in arbitrary order. The iterator element type is &'a V.

Refer to values for further details.

An iterator visiting all values mutably in arbitrary order. The iterator element type is &'a mut V.

Refer to values for further details.

An iterator visiting all key-value pairs in arbitrary order. The iterator element type is (&'a K, &'a V).

Refer to iter for further details.

An iterator visiting all key-value pairs in arbitrary order, with mutable references to the values. The iterator element type is (&'a K, &'a mut V).

Refer to iter_mut for further details.

Returns the number of elements in the map.

Refer to len for further details.

Returns true if the map contains no elements.

Refer to is_empty for further details.

Clears the map, returning all key-value pairs as an iterator. Keeps the allocated memory for reuse.

Refer to drain for further details.

Retains only the elements specified by the predicate. Keeps the allocated memory for reuse.

Refer to retain for further details.

Drains elements which are true under the given predicate, and returns an iterator over the removed items.

Refer to extract_if for further details.

Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.

Refer to clear for further details.

Reserves capacity for at least additional more elements to be inserted in the HashMap. The collection may reserve more space to avoid frequent reallocations.

Refer to reserve for further details.

Tries to reserve capacity for at least additional more elements to be inserted in the given HashMap<K,V>. The collection may reserve more space to avoid frequent reallocations.

Refer to try_reserve for further details.

Shrinks the capacity of the map as much as possible. It will drop down as much as possible while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.

Refer to shrink_to_fit for further details.

Shrinks the capacity of the map with a lower limit. It will drop down no lower than the supplied limit while maintaining the internal rules and possibly leaving some space in accordance with the re

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct EntitySpecializationTicks {
    pub entities: HashMap<MainEntity, Tick, EntityHash>,
}
```

Example 2 (javascript):
```javascript
let map = HashMap::with_capacity(5);

assert!(map.capacity() >= 5);
```

Example 3 (javascript):
```javascript
let mut map = HashMap::new();

map.insert("foo", 0);
map.insert("bar", 1);
map.insert("baz", 2);

for key in map.keys() {
    // foo, bar, baz
    // Note that the above order is not guaranteed
}
```

Example 4 (javascript):
```javascript
25fn send_scroll_events(
26    mut mouse_wheel_reader: MessageReader<MouseWheel>,
27    hover_map: Res<HoverMap>,
28    keyboard_input: Res<ButtonInput<KeyCode>>,
29    mut commands: Commands,
30) {
31    for mouse_wheel in mouse_wheel_reader.read() {
32        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);
33
34        if mouse_wheel.unit == MouseScrollUnit::Line {
35            delta *= LINE_HEIGHT;
36        }
37
38        if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
39            std::mem::swap(&mut delta.x, &mut delta.y);
40        }
41
42     
...
```

---

## Module observer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/observer/index.html

**Contents:**
- Module observer Copy item path
- Structs§
- Type Aliases§

Observers are a push-based tool for responding to Events. The Observer component holds a System that runs whenever a matching Event is triggered.

See Event and Observer for in-depth documentation and usage examples.

---

## Module query Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/query/index.html

**Contents:**
- Module query Copy item path
- Structs§
- Enums§
- Traits§
- Type Aliases§
- Derive Macros§

Contains APIs for retrieving component data from the world.

---

## Struct Window Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.Window.html

**Contents:**
- Struct Window Copy item path
- §Example
- Fields§
    - §Platform-specific
    - §Platform-specific
    - §Platform-specific
    - §Platform-specific
    - §Platform-specific
    - §Platform-specific
    - §Platform-specific

The defining Component for window entities, storing information about how it should appear and behave.

Each window corresponds to an entity, and is uniquely identified by the value of their Entity. When the Window component is added to an entity, a new window will be opened. When it is removed or the entity is despawned, the window will close.

The primary window entity (and the corresponding window) is spawned by default by WindowPlugin and is marked with the PrimaryWindow component.

This component is synchronized with winit through bevy_winit: it will reflect the current state of the window and can be modified to change this state.

Because this component is synchronized with winit, it can be used to perform OS-integrated windowing operations. For example, here’s a simple system to change the window mode:

What presentation mode to give the window.

Which fullscreen or windowing mode should be used.

Where the window should be placed.

What resolution the window should have.

Stores the title of the window.

Stores the application ID (on Wayland), WM_CLASS (on X11) or window class name (on Windows) of the window.

For details about application ID conventions, see the Desktop Entry Spec. For details about WM_CLASS, see the X11 Manual Pages. For details about Windows’s window class names, see About Window Classes.

Notes: Changing this field during runtime will have no effect for now.

How the alpha channel of textures should be handled while compositing.

The limits of the window’s logical size (found in its resolution) when resizing.

Should the window be resizable?

Note: This does not stop the program from fullscreening/setting the size programmatically.

Specifies which window control buttons should be enabled.

iOS, Android, and the Web do not have window control buttons.

On some Linux environments these values have no effect.

Should the window have decorations enabled?

(Decorations are the minimize, maximize, and close buttons on desktop apps)

iOS, Android, and the Web do not have decorations.

Should the window be transparent?

Defines whether the background of the window should be transparent.

macOS transparent works with winit out of the box, so this issue might be related to: https://github.com/gfx-rs/wgpu/issues/687. You should also set the window composite_alpha_mode to CompositeAlphaMode::PostMultiplied.

Get/set whether the window is focused.

It cannot be set unfocused after creation.

Where should the window appear relative to other

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Window {Show 39 fields
    pub present_mode: PresentMode,
    pub mode: WindowMode,
    pub position: WindowPosition,
    pub resolution: WindowResolution,
    pub title: String,
    pub name: Option<String>,
    pub composite_alpha_mode: CompositeAlphaMode,
    pub resize_constraints: WindowResizeConstraints,
    pub resizable: bool,
    pub enabled_buttons: EnabledButtons,
    pub decorations: bool,
    pub transparent: bool,
    pub focused: bool,
    pub window_level: WindowLevel,
    pub canvas: Option<String>,
    pub fit_canvas_to_parent: bool,
    pub prevent_default_event_h
...
```

Example 2 (unknown):
```unknown
fn change_window_mode(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    // Query returns one window typically.
    for mut window in windows.iter_mut() {
        window.mode =
            WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current);
    }
}
```

Example 3 (unknown):
```unknown
21fn minimize_automatically(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
22    if frames.0 != 60 {
23        return;
24    }
25
26    window.set_minimized(true);
27}
```

Example 4 (javascript):
```javascript
122fn move_or_resize_windows(
123    mut windows: Query<&mut Window>,
124    action: Res<LeftClickAction>,
125    input: Res<ButtonInput<MouseButton>>,
126    dir: Res<ResizeDir>,
127) {
128    // Both `start_drag_move()` and `start_drag_resize()` must be called after a
129    // left mouse button press as done here.
130    //
131    // winit 0.30.5 may panic when initiated without a left mouse button press.
132    if input.just_pressed(MouseButton::Left) {
133        for mut window in windows.iter_mut() {
134            match *action {
135                LeftClickAction::Nothing => (),
136   
...
```

---

## Struct TextWriter Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/text/struct.TextWriter.html

**Contents:**
- Struct TextWriter Copy item path
- Implementations§
  - impl<'w, 's, R> TextWriter<'w, 's, R>where R: TextRoot,
    - pub fn get( &mut self, root_entity: Entity, index: usize, ) -> Option<(Entity, usize, Mut<'_, String>, Mut<'_, TextFont>, Mut<'_, TextColor>)>
    - pub fn get_text( &mut self, root_entity: Entity, index: usize, ) -> Option<Mut<'_, String>>
      - Examples found in repository?
    - pub fn get_font( &mut self, root_entity: Entity, index: usize, ) -> Option<Mut<'_, TextFont>>
    - pub fn get_color( &mut self, root_entity: Entity, index: usize, ) -> Option<Mut<'_, TextColor>>
    - pub fn text(&mut self, root_entity: Entity, index: usize) -> Mut<'_, String>
      - Examples found in repository?

System parameter for reading and writing text spans in a text block.

R is the root text component, and S is the text span component on children.

Gets a mutable reference to a text span within a text block at a specific index in the flattened span list.

Gets the text value of a text span within a text block at a specific index in the flattened span list.

Gets the TextFont of a text span within a text block at a specific index in the flattened span list.

Gets the TextColor of a text span within a text block at a specific index in the flattened span list.

Gets the text value of a text span within a text block at a specific index in the flattened span list.

Panics if there is no span at the requested index.

Gets the TextFont of a text span within a text block at a specific index in the flattened span list.

Panics if there is no span at the requested index.

Gets the TextColor of a text span within a text block at a specific index in the flattened span list.

Panics if there is no span at the requested index.

Invokes a callback on each span in a text block, starting with the root entity.

Invokes a callback on each span’s string value in a text block, starting with the root entity.

Invokes a callback on each span’s TextFont in a text block, starting with the root entity.

Invokes a callback on each span’s TextColor in a text block, starting with the root entity.

Invokes a callback on each span in a text block, starting with the root entity.

Traversal will stop when the callback returns false.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct TextWriter<'w, 's, R>where
    R: TextRoot,{ /* private fields */ }
```

Example 2 (javascript):
```javascript
371fn update_text(
372    primitive_state: Res<State<PrimitiveSelected>>,
373    header: Query<Entity, With<HeaderText>>,
374    mut writer: TextUiWriter,
375) {
376    let new_text = format!("{text}", text = primitive_state.get());
377    header.iter().for_each(|header_text| {
378        if let Some(mut text) = writer.get_text(header_text, 2) {
379            (*text).clone_from(&new_text);
380        };
381    });
382}
```

Example 3 (unknown):
```unknown
327fn update_scoreboard(
328    score: Res<Score>,
329    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
330    mut writer: TextUiWriter,
331) {
332    *writer.text(*score_root, 1) = score.to_string();
333}
```

Example 4 (unknown):
```unknown
82fn system_a(entity_a: Single<Entity, With<Text>>, mut writer: TextUiWriter) {
83    *writer.text(*entity_a, 3) = String::from("A");
84    info!("A: One shot system registered with Commands was triggered");
85}
86
87fn system_b(entity_b: Single<Entity, With<Text>>, mut writer: TextUiWriter) {
88    *writer.text(*entity_b, 3) = String::from("B");
89    info!("B: One shot system registered with World was triggered");
90}
```

---

## Enum SystemCursorIcon Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.SystemCursorIcon.html

**Contents:**
- Enum SystemCursorIcon Copy item path
- Variants§
  - Default
  - ContextMenu
  - Help
  - Pointer
  - Progress
  - Wait
  - Cell
  - Crosshair

The icon to display for a window.

Examples of all of these cursors can be found here. This enum is simply a copy of a similar enum found in winit. winit, in turn, is based upon the CSS3 UI spec.

See the window_settings example for usage.

The platform-dependent default cursor. Often rendered as arrow.

A context menu is available for the object under the cursor. Often rendered as an arrow with a small menu-like graphic next to it.

Help is available for the object under the cursor. Often rendered as a question mark or a balloon.

The cursor is a pointer that indicates a link. Often rendered as the backside of a hand with the index finger extended.

A progress indicator. The program is performing some processing, but is different from SystemCursorIcon::Wait in that the user may still interact with the program.

Indicates that the program is busy and the user should wait. Often rendered as a watch or hourglass.

Indicates that a cell or set of cells may be selected. Often rendered as a thick plus-sign with a dot in the middle.

A simple crosshair (e.g., short line segments resembling a “+” sign). Often used to indicate a two dimensional bitmap selection mode.

Indicates text that may be selected. Often rendered as an I-beam.

Indicates vertical-text that may be selected. Often rendered as a horizontal I-beam.

Indicates an alias of/shortcut to something is to be created. Often rendered as an arrow with a small curved arrow next to it.

Indicates something is to be copied. Often rendered as an arrow with a small plus sign next to it.

Indicates something is to be moved.

Indicates that the dragged item cannot be dropped at the current cursor location. Often rendered as a hand or pointer with a small circle with a line through it.

Indicates that the requested action will not be carried out. Often rendered as a circle with a line through it.

Indicates that something can be grabbed (dragged to be moved). Often rendered as the backside of an open hand.

Indicates that something is being grabbed (dragged to be moved). Often rendered as the backside of a hand with fingers closed mostly out of view.

The east border to be moved.

The north border to be moved.

The north-east corner to be moved.

The north-west corner to be moved.

The south border to be moved.

The south-east corner to be moved.

The south-west corner to be moved.

The west border to be moved.

The east and west borders to be moved.

The south and north borders to be moved.

The north-east and s

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub enum SystemCursorIcon {
Show 34 variants    Default,
    ContextMenu,
    Help,
    Pointer,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    VerticalText,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
    AllScroll,
    ZoomIn,
    ZoomOut,
}
```

---

## Struct GizmoMeshSystems Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/struct.GizmoMeshSystems.html

**Contents:**
- Struct GizmoMeshSystems Copy item path
- Trait Implementations§
  - impl Clone for GizmoMeshSystems
    - fn clone(&self) -> GizmoMeshSystems
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for GizmoMeshSystems
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Hash for GizmoMeshSystems
    - fn hash<__H>(&self, state: &mut __H)where __H: Hasher,
    - fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,

System set for updating the rendering meshes for drawing gizmos.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GizmoMeshSystems;
```

---

## Struct RelatedSpawner Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/relationship/struct.RelatedSpawner.html

**Contents:**
- Struct RelatedSpawner Copy item path
- Implementations§
  - impl<'w, R> RelatedSpawner<'w, R>where R: Relationship,
    - pub fn new(world: &'w mut World, target: Entity) -> RelatedSpawner<'w, R>
    - pub fn spawn(&mut self, bundle: impl Bundle) -> EntityWorldMut<'_>
      - Examples found in repository?
    - pub fn spawn_empty(&mut self) -> EntityWorldMut<'_>
    - pub fn target_entity(&self) -> Entity
    - pub fn world(&self) -> &World
    - pub fn world_mut(&mut self) -> &mut World

Directly spawns related “source” entities with the given Relationship, targeting a specific entity.

Creates a new instance that will spawn entities targeting the target entity.

Spawns an entity with the given bundle and an R relationship targeting the target entity this spawner was initialized with.

Spawns an entity with an R relationship targeting the target entity this spawner was initialized with.

Returns the “target entity” used when spawning entities with an R Relationship.

Returns a reference to the underlying World.

Returns a mutable reference to the underlying World.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RelatedSpawner<'w, R>where
    R: Relationship,{ /* private fields */ }
```

Example 2 (unknown):
```unknown
146fn background_and_button() -> impl Bundle {
147    (
148        Name::new("background"),
149        Node {
150            width: percent(100),
151            height: percent(100),
152            align_items: AlignItems::Center,
153            justify_content: JustifyContent::Center,
154            ..default()
155        },
156        ZIndex(-10),
157        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
158            parent
159                .spawn((
160                    Name::new("button"),
161                    Button,
162                    Node {
163            
...
```

Example 3 (javascript):
```javascript
519    fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<DisplayQuality>) {
520        fn button_node() -> Node {
521            Node {
522                width: px(200),
523                height: px(65),
524                margin: UiRect::all(px(20)),
525                justify_content: JustifyContent::Center,
526                align_items: AlignItems::Center,
527                ..default()
528            }
529        }
530        fn button_text_style() -> impl Bundle {
531            (
532                TextFont {
533                    font_size: 33.0,
534      
...
```

Example 4 (javascript):
```javascript
61fn scroll_area_demo() -> impl Bundle {
62    (
63        // Frame element which contains the scroll area and scrollbars.
64        Node {
65            display: Display::Grid,
66            width: px(200),
67            height: px(150),
68            grid_template_columns: vec![RepeatedGridTrack::flex(1, 1.), RepeatedGridTrack::auto(1)],
69            grid_template_rows: vec![RepeatedGridTrack::flex(1, 1.), RepeatedGridTrack::auto(1)],
70            row_gap: px(2),
71            column_gap: px(2),
72            ..default()
73        },
74        Children::spawn((SpawnWith(|parent: &mut Relat
...
```

---

## Struct RelatedSpawnerCommands Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/relationship/struct.RelatedSpawnerCommands.html

**Contents:**
- Struct RelatedSpawnerCommands Copy item path
- Implementations§
  - impl<'w, R> RelatedSpawnerCommands<'w, R>where R: Relationship,
    - pub fn new( commands: Commands<'w, 'w>, target: Entity, ) -> RelatedSpawnerCommands<'w, R>
    - pub fn spawn(&mut self, bundle: impl Bundle) -> EntityCommands<'_>
      - Examples found in repository?
    - pub fn spawn_empty(&mut self) -> EntityCommands<'_>
    - pub fn target_entity(&self) -> Entity
    - pub fn commands(&mut self) -> Commands<'_, '_>
    - pub fn commands_mut(&mut self) -> &mut Commands<'w, 'w>

Uses commands to spawn related “source” entities with the given Relationship, targeting a specific entity.

Creates a new instance that will spawn entities targeting the target entity.

Spawns an entity with the given bundle and an R relationship targeting the target entity this spawner was initialized with.

Spawns an entity with an R relationship targeting the target entity this spawner was initialized with.

Returns the “target entity” used when spawning entities with an R Relationship.

Returns the underlying Commands.

Returns a mutable reference to the underlying Commands.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RelatedSpawnerCommands<'w, R>where
    R: Relationship,{ /* private fields */ }
```

Example 2 (unknown):
```unknown
27fn setup(mut commands: Commands) {
28    commands
29        .spawn((Name::new("Goblin"), HitPoints(50)))
30        .observe(take_damage)
31        .with_children(|parent| {
32            parent
33                .spawn((Name::new("Helmet"), Armor(5)))
34                .observe(block_attack);
35            parent
36                .spawn((Name::new("Socks"), Armor(10)))
37                .observe(block_attack);
38            parent
39                .spawn((Name::new("Shirt"), Armor(15)))
40                .observe(block_attack);
41        });
42}
```

Example 3 (javascript):
```javascript
144fn print_logs(
145    mut log_message_reader: MessageReader<LogMessage>,
146    mut commands: Commands,
147    log_viewer_root: Single<Entity, With<LogViewerRoot>>,
148) {
149    let root_entity = *log_viewer_root;
150
151    commands.entity(root_entity).with_children(|child| {
152        for log_message in log_message_reader.read() {
153            child.spawn((
154                Text::default(),
155                children![
156                    (
157                        TextSpan::new(format!("{:5} ", log_message.level)),
158                        TextColor(level_color(&log_message
...
```

Example 4 (javascript):
```javascript
85fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut state: ResMut<State>) {
86    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");
87    state.handle = font_handle.clone();
88    commands.spawn(Camera2d);
89    commands
90        .spawn((
91            Node {
92                position_type: PositionType::Absolute,
93                bottom: Val::ZERO,
94                ..default()
95            },
96            BackgroundColor(Color::NONE),
97        ))
98        .with_children(|parent| {
99            parent.spawn((
100                Text::new("a"),
101     
...
```

---

## Struct InputSystems Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input/struct.InputSystems.html

**Contents:**
- Struct InputSystems Copy item path
- Trait Implementations§
  - impl Clone for InputSystems
    - fn clone(&self) -> InputSystems
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for InputSystems
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Hash for InputSystems
    - fn hash<__H>(&self, state: &mut __H)where __H: Hasher,
    - fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,

Label for systems that update the input data.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct InputSystems;
```

---

## Type Alias AnimationEntityMut Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/type.AnimationEntityMut.html

**Contents:**
- Type Alias AnimationEntityMut Copy item path
- Aliased Type§

A type alias for EntityMutExcept as used in animation.

**Examples:**

Example 1 (unknown):
```unknown
pub type AnimationEntityMut<'w, 's> = EntityMutExcept<'w, 's, (AnimationTarget, AnimationPlayer, AnimationGraphHandle)>;
```

Example 2 (unknown):
```unknown
pub struct AnimationEntityMut<'w, 's> { /* private fields */ }
```

---

## Enum MonitorSelection Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.MonitorSelection.html

**Contents:**
- Enum MonitorSelection Copy item path
- Variants§
  - Current
  - Primary
  - Index(usize)
  - Entity(Entity)
- Trait Implementations§
  - impl Clone for MonitorSelection
    - fn clone(&self) -> MonitorSelection
    - fn clone_from(&mut self, source: &Self)

References a screen monitor.

Used when centering a Window on a monitor.

Uses the current monitor of the window.

If WindowPosition::Centered(MonitorSelection::Current) is used when creating a window, the window doesn’t have a monitor yet, this will fall back to WindowPosition::Automatic.

Uses the primary monitor of the system.

Uses the monitor with the specified index.

Uses a given crate::monitor::Monitor entity.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum MonitorSelection {
    Current,
    Primary,
    Index(usize),
    Entity(Entity),
}
```

---

## Module archetype Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/archetype/index.html

**Contents:**
- Module archetype Copy item path
- Structs§
- Type Aliases§

Types for defining Archetypes, collections of entities that have the same set of components.

An archetype uniquely describes a group of entities that share the same components: a world only has one archetype for each unique combination of components, and all entities that have those components and only those components belong to that archetype.

Archetypes are not to be confused with Tables. Each archetype stores its table components in one table, and each archetype uniquely points to one table, but multiple archetypes may store their table components in the same table. These archetypes differ only by the SparseSet components.

Like tables, archetypes can be created but are never cleaned up. Empty archetypes are not removed, and persist until the world is dropped.

Archetypes can be fetched from Archetypes, which is accessible via World::archetypes.

---

## Trait ColorToComponents Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/trait.ColorToComponents.html

**Contents:**
- Trait ColorToComponents Copy item path
- Required Methods§
    - fn to_f32_array(self) -> [f32; 4]
    - fn to_f32_array_no_alpha(self) -> [f32; 3]
    - fn to_vec4(self) -> Vec4
    - fn to_vec3(self) -> Vec3
    - fn from_f32_array(color: [f32; 4]) -> Self
    - fn from_f32_array_no_alpha(color: [f32; 3]) -> Self
    - fn from_vec4(color: Vec4) -> Self
    - fn from_vec3(color: Vec3) -> Self

Trait with methods for converting colors to non-color types

Convert to an f32 array

Convert to an f32 array without the alpha value

Convert from an f32 array

Convert from an f32 array without the alpha value

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait ColorToComponents {
    // Required methods
    fn to_f32_array(self) -> [f32; 4];
    fn to_f32_array_no_alpha(self) -> [f32; 3];
    fn to_vec4(self) -> Vec4;
    fn to_vec3(self) -> Vec3;
    fn from_f32_array(color: [f32; 4]) -> Self;
    fn from_f32_array_no_alpha(color: [f32; 3]) -> Self;
    fn from_vec4(color: Vec4) -> Self;
    fn from_vec3(color: Vec3) -> Self;
}
```

---

## Struct HotPatched Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/struct.HotPatched.html

**Contents:**
- Struct HotPatched Copy item path
- Trait Implementations§
  - impl Default for HotPatched
    - fn default() -> HotPatched
  - impl Message for HotPatchedwhere HotPatched: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for HotPatched
  - impl RefUnwindSafe for HotPatched
  - impl Send for HotPatched
  - impl Sync for HotPatched

Event sent when a hotpatch happens.

Can be used for causing custom behavior on hot-patch.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct HotPatched;
```

---

## Struct RenderMeshInstanceShared Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.RenderMeshInstanceShared.html

**Contents:**
- Struct RenderMeshInstanceShared Copy item path
- Fields§
- Implementations§
  - impl RenderMeshInstanceShared
    - pub fn should_batch(&self) -> bool
      - Examples found in repository?
- Auto Trait Implementations§
  - impl Freeze for RenderMeshInstanceShared
  - impl RefUnwindSafe for RenderMeshInstanceShared
  - impl Send for RenderMeshInstanceShared

CPU data that the render world needs to keep about each entity that contains a mesh.

The AssetId of the mesh.

A slot for the material bind group index.

Index of the slab that the lightmap resides in, if a lightmap is present.

User supplied tag to identify this mesh instance.

Render layers that this mesh instance belongs to.

Returns true if this entity is eligible to participate in automatic batching.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderMeshInstanceShared {
    pub mesh_asset_id: AssetId<Mesh>,
    pub material_bindings_index: MaterialBindingId,
    pub flags: RenderMeshInstanceFlags,
    pub lightmap_slab_index: Option<LightmapSlabIndex>,
    pub tag: u32,
    pub render_layers: Option<RenderLayers>,
}
```

Example 2 (javascript):
```javascript
384    fn get_index_and_compare_data(
385        (mesh_instances, _, _): &SystemParamItem<Self::Param>,
386        main_entity: MainEntity,
387    ) -> Option<(NonMaxU32, Option<Self::CompareData>)> {
388        // This should only be called during GPU building.
389        let RenderMeshInstances::GpuBuilding(ref mesh_instances) = **mesh_instances else {
390            error!(
391                "`get_index_and_compare_data` should never be called in CPU mesh uniform building \
392                mode"
393            );
394            return None;
395        };
396        let mesh_instance = m
...
```

Example 3 (javascript):
```javascript
267fn queue_custom_mesh_pipeline(
268    pipeline_cache: Res<PipelineCache>,
269    custom_mesh_pipeline: Res<CustomMeshPipeline>,
270    (mut opaque_render_phases, opaque_draw_functions): (
271        ResMut<ViewBinnedRenderPhases<Opaque3d>>,
272        Res<DrawFunctions<Opaque3d>>,
273    ),
274    mut specialized_mesh_pipelines: ResMut<SpecializedMeshPipelines<CustomMeshPipeline>>,
275    views: Query<(&RenderVisibleEntities, &ExtractedView, &Msaa)>,
276    (render_meshes, render_mesh_instances): (
277        Res<RenderAssets<RenderMesh>>,
278        Res<RenderMeshInstances>,
279    ),
280 
...
```

---

## Struct Aabb Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/primitives/struct.Aabb.html

**Contents:**
- Struct Aabb Copy item path
- Fields§
- Implementations§
  - impl Aabb
    - pub fn from_min_max(minimum: Vec3, maximum: Vec3) -> Aabb
    - pub fn enclosing<T>(iter: impl IntoIterator<Item = T>) -> Option<Aabb>where T: Borrow<Vec3>,
      - §Examples
    - pub fn relative_radius(&self, p_normal: &Vec3A, world_from_local: &Mat3A) -> f32
    - pub fn min(&self) -> Vec3A
    - pub fn max(&self) -> Vec3A

An axis-aligned bounding box, defined by:

It is typically used as a component on an entity to represent the local space occupied by this entity, with faces orthogonal to its local axis.

This component is notably used during “frustum culling”, a process to determine if an entity should be rendered by a Camera if its bounding box intersects with the camera’s Frustum.

It will be added automatically by the systems in CalculateBounds to entities that:

It won’t be updated automatically if the space occupied by the entity changes, for example if the vertex positions of a Mesh3d are updated.

Returns a bounding box enclosing the specified set of points.

Returns None if the iterator is empty.

Calculate the relative radius of the AABB with respect to a plane

Check if the AABB is at the front side of the bisecting plane. Referenced from: AABB Plane intersection

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Aabb {
    pub center: Vec3A,
    pub half_extents: Vec3A,
}
```

Example 2 (javascript):
```javascript
let bb = Aabb::enclosing([Vec3::X, Vec3::Z * 2.0, Vec3::Y * -0.5]).unwrap();
assert_eq!(bb.min(), Vec3A::new(0.0, -0.5, 0.0));
assert_eq!(bb.max(), Vec3A::new(1.0, 0.0, 2.0));
```

---

## Struct RenderLightmaps Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.RenderLightmaps.html

**Contents:**
- Struct RenderLightmaps Copy item path
- Trait Implementations§
  - impl Resource for RenderLightmapswhere RenderLightmaps: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for RenderLightmaps
  - impl !RefUnwindSafe for RenderLightmaps
  - impl Send for RenderLightmaps
  - impl Sync for RenderLightmaps
  - impl Unpin for RenderLightmaps
  - impl !UnwindSafe for RenderLightmaps

Stores data for all lightmaps in the render world.

This is cleared and repopulated each frame during the extract_lightmaps system.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderLightmaps { /* private fields */ }
```

---

## Struct EntityMutExcept Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/world/struct.EntityMutExcept.html

**Contents:**
- Struct EntityMutExcept Copy item path
- Implementations§
  - impl<'w, 's, B> EntityMutExcept<'w, 's, B>where B: Bundle,
    - pub fn id(&self) -> Entity
    - pub fn reborrow(&mut self) -> EntityMutExcept<'_, 's, B>
    - pub fn as_readonly(&self) -> EntityRefExcept<'_, 's, B>
    - pub fn get<C>(&self) -> Option<&C>where C: Component,
    - pub fn get_ref<C>(&self) -> Option<Ref<'_, C>>where C: Component,
    - pub fn get_mut<C>(&mut self) -> Option<Mut<'_, C>>where C: Component<Mutability = Mutable>,
      - Examples found in repository?

Provides mutable access to all components of an entity, with the exception of an explicit set.

This is a rather niche type that should only be used if you need access to all components of an entity, while still allowing you to consult other queries that might match entities that this query also matches. If you don’t need access to all components, prefer a standard query with a Without filter.

Returns the ID of the current entity.

Returns a new instance with a shorter lifetime.

This is useful if you have &mut EntityMutExcept, but you need EntityMutExcept.

Gets read-only access to all of the entity’s components, except for the ones in CL.

Gets access to the component of type C for the current entity. Returns None if the component doesn’t have a component of that type or if the type is one of the excluded components.

Gets access to the component of type C for the current entity, including change detection information. Returns None if the component doesn’t have a component of that type or if the type is one of the excluded components.

Gets mutable access to the component of type C for the current entity. Returns None if the component doesn’t have a component of that type or if the type is one of the excluded components.

Returns the source code location from which this entity has been spawned.

Returns the Tick at which this entity has been spawned.

Returns true if the current entity has a component of type T. Otherwise, this returns false.

If you do not know the concrete type of a component, consider using Self::contains_id or Self::contains_type_id.

Returns true if the current entity has a component identified by component_id. Otherwise, this returns false.

Returns true if the current entity has a component with the type identified by type_id. Otherwise, this returns false.

Gets the component of the given ComponentId from the entity.

You should prefer to use the typed API Self::get where possible and only use this in cases where the actual component types are not known at compile time.

Unlike EntityMutExcept::get, this returns a raw pointer to the component, which is only valid while the EntityMutExcept is alive.

Gets a MutUntyped of the component of the given ComponentId from the entity.

You should prefer to use the typed API Self::get_mut where possible and only use this in cases where the actual component types are not known at compile time.

Unlike EntityMutExcept::get_mut, this returns a raw pointer to the component, which is only valid

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct EntityMutExcept<'w, 's, B>where
    B: Bundle,{ /* private fields */ }
```

Example 2 (javascript):
```javascript
173    fn get_mut<'a>(
174        &self,
175        entity: &'a mut AnimationEntityMut,
176    ) -> Result<&'a mut Self::Property, AnimationEvaluationError> {
177        let text_color = entity
178            .get_mut::<TextColor>()
179            .ok_or(AnimationEvaluationError::ComponentNotPresent(TypeId::of::<
180                TextColor,
181            >(
182            )))?
183            .into_inner();
184        match text_color.0 {
185            Color::Srgba(ref mut color) => Ok(color),
186            _ => Err(AnimationEvaluationError::PropertyNotPresent(TypeId::of::<
187            
...
```

---

## Module system Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/system/index.html

**Contents:**
- Module system Copy item path
- §System ordering
  - §Example
- §System return type
- §System parameter list
- Modules§
- Structs§
- Enums§
- Traits§
- Functions§

Tools for controlling behavior in an ECS application.

Systems define how an ECS based application behaves. Systems are added to a Schedule, which is then run. A system is usually written as a normal function, which is automatically converted into a system.

System functions can have parameters, through which one can query and mutate Bevy ECS state. Only types that implement SystemParam can be used, automatically fetching data from the World.

System functions often look like this:

By default, the execution of systems is parallel and not deterministic. Not all systems can run together: if a system mutably accesses data, no other system that reads or writes that data can be run at the same time. These systems are said to be incompatible.

The relative order in which incompatible systems are run matters. When this is not specified, a system order ambiguity exists in your schedule. You can explicitly order systems:

Systems added to a schedule through add_systems may either return empty () or a Result. Other contexts (like one shot systems) allow systems to return arbitrary values.

Following is the complete list of accepted types as system parameters:

In addition, the following parameters can be used when constructing a dynamic system with SystemParamBuilder, but will only provide an empty value when used with an ordinary system:

**Examples:**

Example 1 (unknown):
```unknown
fn update_score_system(
    mut query: Query<(&Player, &mut Score)>,
    mut round: ResMut<Round>,
) {
    for (player, mut score) in &mut query {
        if player.alive {
            score.0 += round.0;
        }
    }
    round.0 += 1;
}
```

Example 2 (unknown):
```unknown
// Configure these systems to run in order using `chain()`.
schedule.add_systems((print_first, print_last).chain());
// Prints "HelloWorld!"
schedule.run(&mut world);

// Configure this system to run in between the other two systems
// using explicit dependencies.
schedule.add_systems(print_mid.after(print_first).before(print_last));
// Prints "Hello, World!"
schedule.run(&mut world);

fn print_first() {
    print!("Hello");
}
fn print_mid() {
    print!(", ");
}
fn print_last() {
    println!("World!");
}
```

---

## Module event Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/event/index.html

**Contents:**
- Module event Copy item path
- Structs§
- Traits§
- Functions§
- Type Aliases§
- Derive Macros§

bevy::ecsModule event Copy item pathSource Expand descriptionEvent functionality. Structs§EntityComponentsTriggerAn EntityEvent Trigger that, in addition to behaving like a normal EntityTrigger, also runs observers that watch for components that match the slice of ComponentIds referenced in EntityComponentsTrigger. This includes both global observers of those components and “entity scoped” observers that watch the EntityEvent::event_target.EntityTriggerAn EntityEvent Trigger that does two things:EventKeyA unique identifier for an Event, used by observers.GlobalTriggerA Trigger that runs every “global” Observer (ex: registered via World::add_observer) that matches the given Event.PropagateEntityTriggerAn EntityEvent Trigger that behaves like EntityTrigger, but “propagates” the event using an Entity Traversal. At each step in the propagation, the EntityTrigger logic will be run, until PropagateEntityTrigger::propagate is false, or there are no entities left to traverse.Traits§EntityEventAn EntityEvent is an Event that is triggered for a specific EntityEvent::event_target entity:EventAn Event is something that “happens” at a given moment.TriggerTrigger determines how an Event is triggered when World::trigger is called. This decides which Observers will run, what data gets passed to them, and the order they will be executed in.Functions§trigger_entity_internal⚠Trigger observers watching for the given entity event. The target_entity should match the EntityEvent::event_target on event for logical correctness.Type Aliases§EventCursorDeprecatedThis is deprecated. See MessageCursorEventIteratorDeprecatedThis is deprecated. See MessageIteratorEventMutIteratorDeprecatedThis is deprecated. See MessageMutIteratorEventMutatorDeprecatedThis is deprecated. See MessageMutatorEventReaderDeprecatedThis is deprecated. See MessageReaderEventWriterDeprecatedThis is deprecated. See MessageWriterEventsDeprecatedThis is deprecated. See MessagesDerive Macros§EntityEventCheat sheet for derive syntax, see full explanation on EntityEvent trait docs.EventImplement the Event trait.

---

## Trait Relationship Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/relationship/trait.Relationship.html

**Contents:**
- Trait Relationship Copy item path
  - §Derive
- Required Associated Types§
    - type RelationshipTarget: RelationshipTarget<Relationship = Self>
- Required Methods§
    - fn get(&self) -> Entity
    - fn from(entity: Entity) -> Self
    - fn set_risky(&mut self, entity: Entity)
      - §Warning
- Provided Methods§

A Component on a “source” Entity that references another target Entity, creating a “relationship” between them. Every Relationship has a corresponding RelationshipTarget type (and vice-versa), which exists on the “target” entity of a relationship and contains the list of all “source” entities that relate to the given “target”

The Relationship component is the “source of truth” and the RelationshipTarget component reflects that source of truth. When a Relationship component is inserted on an Entity, the corresponding RelationshipTarget component is immediately inserted on the target component if it does not already exist, and the “source” entity is automatically added to the RelationshipTarget collection (this is done via “component hooks”).

A common example of a Relationship is the parent / child relationship. Bevy ECS includes a canonical form of this via the ChildOf Relationship and the Children RelationshipTarget.

Relationship and RelationshipTarget should always be derived via the Component trait to ensure the hooks are set up properly.

Relationship and RelationshipTarget can only be derived for structs with a single unnamed field, single named field or for named structs where one field is annotated with #[relationship]. If there are additional fields, they must all implement Default.

RelationshipTarget also requires that the relationship field is private to prevent direct mutation, ensuring the correctness of relationships.

When deriving RelationshipTarget you can specify the #[relationship_target(linked_spawn)] attribute to automatically despawn entities stored in an entity’s RelationshipTarget when that entity is despawned:

The Component added to the “target” entities of this Relationship, which contains the list of all “source” entities that relate to the “target”.

Gets the Entity ID of the related entity.

Creates this Relationship from the given entity.

Changes the current Entity ID of the entity containing the RelationshipTarget to another one.

This is useful for updating the relationship without overwriting other fields stored in Self.

This should generally not be called by user code, as modifying the related entity could invalidate the relationship. If this method is used, then the hooks on_replace have to run before and on_insert after it. This happens automatically when this method is called with EntityWorldMut::modify_component.

Prefer to use regular means of insertions when possible.

The on_insert component hook that maintains

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub trait Relationship: Sized + Component {
    type RelationshipTarget: RelationshipTarget<Relationship = Self>;

    // Required methods
    fn get(&self) -> Entity;
    fn from(entity: Entity) -> Self;
    fn set_risky(&mut self, entity: Entity);

    // Provided methods
    fn on_insert(world: DeferredWorld<'_>, _: HookContext) { ... }
    fn on_replace(world: DeferredWorld<'_>, _: HookContext) { ... }
}
```

Example 2 (unknown):
```unknown
#[derive(Component)]
#[relationship(relationship_target = Children)]
pub struct ChildOf {
    #[relationship]
    pub parent: Entity,
    internal: u8,
};

#[derive(Component)]
#[relationship_target(relationship = ChildOf)]
pub struct Children(Vec<Entity>);
```

Example 3 (unknown):
```unknown
#[derive(Component)]
#[relationship(relationship_target = Children)]
pub struct ChildOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = ChildOf, linked_spawn)]
pub struct Children(Vec<Entity>);
```

---

## Enum ScheduleConfigs Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/schedule/enum.ScheduleConfigs.html

**Contents:**
- Enum ScheduleConfigs Copy item path
- Variants§
  - ScheduleConfig(ScheduleConfig<T>)
  - Configs
    - Fields
- Implementations§
  - impl<T> ScheduleConfigs<T>where T: Schedulable<Metadata = GraphInfo, GroupMetadata = Chain>,
    - pub fn in_set_inner(&mut self, set: Interned<dyn SystemSet>)
    - pub fn run_if_dyn( &mut self, condition: Box<dyn ReadOnlySystem<Out = bool, In = ()>>, )
- Trait Implementations§

Single or nested configurations for Schedulables.

Configuration for a single Schedulable.

Configuration for a tuple of nested Configs instances.

Configuration for each element of the tuple.

Run conditions applied to everything in the tuple.

Metadata to be applied to all elements in the tuple.

Adds a new boxed system set to the systems.

Adds a new boxed run condition to the systems.

This is useful if you have a run condition whose concrete type is unknown. Prefer run_if for run conditions whose type is known at compile time.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ScheduleConfigs<T>where
    T: Schedulable,{
    ScheduleConfig(ScheduleConfig<T>),
    Configs {
        configs: Vec<ScheduleConfigs<T>>,
        collective_conditions: Vec<Box<dyn ReadOnlySystem<Out = bool, In = ()>>>,
        metadata: <T as Schedulable>::GroupMetadata,
    },
}
```

---

## Macro related Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/macro.related.html

**Contents:**
- Macro related Copy item path

Returns a SpawnRelatedBundle that will insert the given RelationshipTarget, spawn a SpawnableList of entities with given bundles that relate to the RelationshipTarget entity via the RelationshipTarget::Relationship component, and reserve space in the RelationshipTarget for each spawned entity.

The first argument is the RelationshipTarget type. Any additional arguments will be interpreted as bundles to be spawned.

Also see children for a Children-specific equivalent.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! related {
    ($relationship_target:ty [$($child:expr),*$(,)?]) => { ... };
}
```

Example 2 (javascript):
```javascript
let mut world = World::new();
world.spawn((
    Name::new("Root"),
    related!(Children[
        Name::new("Child1"),
        (
            Name::new("Child2"),
            related!(Children[
                Name::new("Grandchild"),
            ])
        )
    ])
));
```

---

## Struct InputFocus Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input_focus/struct.InputFocus.html

**Contents:**
- Struct InputFocus Copy item path
- §Examples
- Tuple Fields§
- Implementations§
  - impl InputFocus
    - pub const fn from_entity(entity: Entity) -> InputFocus
    - pub const fn set(&mut self, entity: Entity)
      - Examples found in repository?
    - pub const fn get(&self) -> Option<Entity>
    - pub const fn clear(&mut self)

Resource representing which entity has input focus, if any. Input events (other than pointer-like inputs) will be dispatched to the current focus entity, or to the primary window if no entity has focus.

Changing the input focus is as easy as modifying this resource.

From within a system:

With exclusive (or deferred) world access:

Create a new InputFocus resource with the given entity.

This is mostly useful for tests.

Set the entity with input focus.

Returns the entity with input focus, if any.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct InputFocus(pub Option<Entity>);
```

Example 2 (unknown):
```unknown
use bevy_ecs::prelude::*;
use bevy_input_focus::InputFocus;

fn clear_focus(mut input_focus: ResMut<InputFocus>) {
  input_focus.clear();
}
```

Example 3 (javascript):
```javascript
use bevy_ecs::prelude::*;
use bevy_input_focus::InputFocus;

fn set_focus_from_world(world: &mut World) {
    let entity = world.spawn_empty().id();

    // Fetch the resource from the world
    let mut input_focus = world.resource_mut::<InputFocus>();
    // Then mutate it!
    input_focus.set(entity);

    // Or you can just insert a fresh copy of the resource
    // which will overwrite the existing one.
    world.insert_resource(InputFocus::from_entity(entity));
}
```

Example 4 (javascript):
```javascript
20fn button_system(
21    mut input_focus: ResMut<InputFocus>,
22    mut interaction_query: Query<
23        (
24            Entity,
25            &Interaction,
26            &mut BackgroundColor,
27            &mut BorderColor,
28            &mut Button,
29            &Children,
30        ),
31        Changed<Interaction>,
32    >,
33    mut text_query: Query<&mut Text>,
34) {
35    for (entity, interaction, mut color, mut border_color, mut button, children) in
36        &mut interaction_query
37    {
38        let mut text = text_query.get_mut(children[0]).unwrap();
39
40        match *inter
...
```

---

## Struct SpawnRelatedBundle Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/spawn/struct.SpawnRelatedBundle.html

**Contents:**
- Struct SpawnRelatedBundle Copy item path
- Trait Implementations§
  - impl<R, L> Bundle for SpawnRelatedBundle<R, L>where R: Relationship, L: SpawnableList<R> + Send + Sync + 'static,
    - fn get_component_ids( components: &Components, ids: &mut impl FnMut(Option<ComponentId>), )
  - impl<R, L> DynamicBundle for SpawnRelatedBundle<R, L>where R: Relationship, L: SpawnableList<R>,
    - type Effect = SpawnRelatedBundle<R, L>
    - unsafe fn get_components( ptr: MovingPtr<'_, SpawnRelatedBundle<R, L>>, func: &mut impl FnMut(StorageType, OwningPtr<'_>), )
    - unsafe fn apply_effect( ptr: MovingPtr<'_, MaybeUninit<SpawnRelatedBundle<R, L>>>, entity: &mut EntityWorldMut<'_>, )
- Auto Trait Implementations§
  - impl<R, L> Freeze for SpawnRelatedBundle<R, L>where L: Freeze,

This is intended to be created using SpawnRelated.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct SpawnRelatedBundle<R, L>where
    R: Relationship,
    L: SpawnableList<R>,{ /* private fields */ }
```

---

## Struct Archetype Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/archetype/struct.Archetype.html

**Contents:**
- Struct Archetype Copy item path
- Implementations§
  - impl Archetype
    - pub fn id(&self) -> ArchetypeId
    - pub fn table_id(&self) -> TableId
    - pub fn entities(&self) -> &[ArchetypeEntity]
    - pub fn entities_with_location( &self, ) -> impl Iterator<Item = (Entity, EntityLocation)>
    - pub fn table_components(&self) -> impl Iterator<Item = ComponentId>
    - pub fn sparse_set_components(&self) -> impl Iterator<Item = ComponentId>
    - pub fn components(&self) -> &[ComponentId]

Metadata for a single archetype within a World.

For more information, see the module level documentation.

Fetches the ID for the archetype.

Fetches the archetype’s Table ID.

Fetches the entities contained in this archetype.

Fetches the entities contained in this archetype.

Gets an iterator of all of the components stored in Tables.

All of the IDs are unique.

Gets an iterator of all of the components stored in ComponentSparseSets.

All of the IDs are unique.

Returns a slice of all of the components in the archetype.

All of the IDs are unique.

Gets an iterator of all of the components in the archetype.

All of the IDs are unique.

Returns the total number of components in the archetype

Fetches an immutable reference to the archetype’s Edges, a cache of archetypal relationships.

Fetches the row in the Table where the components for the entity at index is stored.

An entity’s archetype row can be fetched from EntityLocation::archetype_row, which can be retrieved from Entities::get.

This function will panic if index >= self.len().

Gets the total number of entities that belong to the archetype.

Checks if the archetype has any entities.

Checks if the archetype contains a specific component. This runs in O(1) time.

Gets the type of storage where a component in the archetype can be found. Returns None if the component is not part of the archetype. This runs in O(1) time.

Returns true if any of the components in this archetype have on_add hooks

Returns true if any of the components in this archetype have on_insert hooks

Returns true if any of the components in this archetype have on_replace hooks

Returns true if any of the components in this archetype have on_remove hooks

Returns true if any of the components in this archetype have on_despawn hooks

Returns true if any of the components in this archetype have at least one Add observer

Returns true if any of the components in this archetype have at least one Insert observer

Returns true if any of the components in this archetype have at least one Replace observer

Returns true if any of the components in this archetype have at least one Remove observer

Returns true if any of the components in this archetype have at least one Despawn observer

SAFETY: Self is the same as Self::ReadOnly

SAFETY: update_component_access does nothing. This is sound because fetch does not access components.

SAFETY: access is read only

Returns the argument unchanged.

That is, this conversion is whatever the imp

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Archetype { /* private fields */ }
```

---

## Function dispatch_focused_input Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input_focus/fn.dispatch_focused_input.html

**Contents:**
- Function dispatch_focused_input Copy item path

System which dispatches bubbled input events to the focused entity, or to the primary window if no entity has focus.

If the currently focused entity no longer exists (has been despawned), this system will automatically clear the focus and dispatch events to the primary window instead.

**Examples:**

Example 1 (unknown):
```unknown
pub fn dispatch_focused_input<M>(
    input_reader: MessageReader<'_, '_, M>,
    focus: ResMut<'_, InputFocus>,
    windows: Query<'_, '_, Entity, With<PrimaryWindow>>,
    entities: &Entities,
    commands: Commands<'_, '_>,
)where
    M: Message + Clone,
```

---

## Struct WindowTraversal Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/input_focus/struct.WindowTraversal.html

**Contents:**
- Struct WindowTraversal Copy item path
- Trait Implementations§
  - impl QueryData for WindowTraversal
    - unsafe fn fetch<'__w, '__s>( _state: &'__s <WindowTraversal as WorldQuery>::State, _fetch: &mut <WindowTraversal as WorldQuery>::Fetch<'__w>, _entity: Entity, _table_row: TableRow, ) -> <WindowTraversal as QueryData>::Item<'__w, '__s>
    - const IS_READ_ONLY: bool = true
    - type ReadOnly = WindowTraversal
    - type Item<'__w, '__s> = WindowTraversalItem<'__w, '__s>
    - fn shrink<'__wlong, '__wshort, '__s>( item: <WindowTraversal as QueryData>::Item<'__wlong, '__s>, ) -> <WindowTraversal as QueryData>::Item<'__wshort, '__s>where '__wlong: '__wshort,
    - fn provide_extra_access( state: &mut <WindowTraversal as WorldQuery>::State, access: &mut Access, available_access: &Access, )
  - impl ReleaseStateQueryData for WindowTraversalwhere Option<&'static ChildOf>: for<'__a> ReleaseStateQueryData, Option<&'static Window>: for<'__a> ReleaseStateQueryData,

These are for accessing components defined on the targeted entity

SAFETY: we assert fields are readonly below

SAFETY: we call fetch for each member that implements Fetch.

SAFETY: we call set_archetype for each member that implements Fetch

SAFETY: we call set_table for each member that implements Fetch

SAFETY: we assert fields are readonly below

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WindowTraversal { /* private fields */ }
```

---

## Module morph Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/morph/index.html

**Contents:**
- Module morph Copy item path
- Structs§
- Enums§
- Constants§

bevy::meshModule morph Copy item pathSource Structs§MeshMorphWeightsControl a specific Mesh instance’s morph targets. These control the weights of specific “mesh primitives” in scene formats like GLTF. They can be set manually, but in most cases they should “automatically” synced by setting the MorphWeights component on a parent entity.MorphAttributesAttributes differences used for morph targets.MorphTargetImageAn image formatted for use with MorphWeights for rendering the morph target.MorphWeightsControls the morph targets for all child Mesh3d entities. In most cases, MorphWeights should be considered the “source of truth” when writing morph targets for meshes. However you can choose to write child MeshMorphWeights if your situation requires more granularity. Just note that if you set MorphWeights, it will overwrite child MeshMorphWeights values.Enums§MorphBuildErrorConstants§MAX_MORPH_WEIGHTSMax target count available for morph targets.

---

## Module extract_resource Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/extract_resource/index.html

**Contents:**
- Module extract_resource Copy item path
- Structs§
- Traits§
- Functions§
- Derive Macros§

bevy::renderModule extract_resource Copy item pathSource Structs§ExtractResourcePluginThis plugin extracts the resources into the “render world”.Traits§ExtractResourceDescribes how a resource gets extracted for rendering.Functions§extract_resourceThis system extracts the resource of the corresponding Resource typeDerive Macros§ExtractResource

---

## Module extract_instances Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/extract_instances/index.html

**Contents:**
- Module extract_instances Copy item path
- Structs§
- Traits§

Convenience logic for turning components from the main world into extracted instances in the render world.

This is essentially the same as the extract_component module, but higher-performance because it avoids the ECS overhead.

---

## Struct RenderMeshQueueData Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.RenderMeshQueueData.html

**Contents:**
- Struct RenderMeshQueueData Copy item path
- Fields§
- Methods from Deref<Target = &'a RenderMeshInstanceShared>§
    - pub fn should_batch(&self) -> bool
      - Examples found in repository?
- Trait Implementations§
  - impl<'a> Deref for RenderMeshQueueData<'a>
    - type Target = &'a RenderMeshInstanceShared
    - fn deref(&self) -> &<RenderMeshQueueData<'a> as Deref>::Target
- Auto Trait Implementations§

Data that crate::material::queue_material_meshes and similar systems need in order to place entities that contain meshes in the right batch.

General information about the mesh instance.

The translation of the mesh instance.

The index of the MeshInputUniform in the GPU buffer for this mesh instance.

Returns true if this entity is eligible to participate in automatic batching.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderMeshQueueData<'a> {
    pub shared: &'a RenderMeshInstanceShared,
    pub translation: Vec3,
    pub current_uniform_index: InputUniformIndex,
}
```

Example 2 (javascript):
```javascript
384    fn get_index_and_compare_data(
385        (mesh_instances, _, _): &SystemParamItem<Self::Param>,
386        main_entity: MainEntity,
387    ) -> Option<(NonMaxU32, Option<Self::CompareData>)> {
388        // This should only be called during GPU building.
389        let RenderMeshInstances::GpuBuilding(ref mesh_instances) = **mesh_instances else {
390            error!(
391                "`get_index_and_compare_data` should never be called in CPU mesh uniform building \
392                mode"
393            );
394            return None;
395        };
396        let mesh_instance = m
...
```

Example 3 (javascript):
```javascript
267fn queue_custom_mesh_pipeline(
268    pipeline_cache: Res<PipelineCache>,
269    custom_mesh_pipeline: Res<CustomMeshPipeline>,
270    (mut opaque_render_phases, opaque_draw_functions): (
271        ResMut<ViewBinnedRenderPhases<Opaque3d>>,
272        Res<DrawFunctions<Opaque3d>>,
273    ),
274    mut specialized_mesh_pipelines: ResMut<SpecializedMeshPipelines<CustomMeshPipeline>>,
275    views: Query<(&RenderVisibleEntities, &ExtractedView, &Msaa)>,
276    (render_meshes, render_mesh_instances): (
277        Res<RenderAssets<RenderMesh>>,
278        Res<RenderMeshInstances>,
279    ),
280 
...
```

---

## Module extract_component Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/extract_component/index.html

**Contents:**
- Module extract_component Copy item path
- Structs§
- Traits§
- Derive Macros§

bevy::renderModule extract_component Copy item pathSource Structs§ComponentUniformsStores all uniforms of the component type.DynamicUniformIndexStores the index of a uniform inside of ComponentUniforms.ExtractComponentPluginThis plugin extracts the components into the render world for synced entities.UniformComponentPluginThis plugin prepares the components of the corresponding type for the GPU by transforming them into uniforms.Traits§ExtractComponentDescribes how a component gets extracted for rendering.Derive Macros§ExtractComponentImplements ExtractComponent trait for a component.

---

## Struct MainWorld Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/struct.MainWorld.html

**Contents:**
- Struct MainWorld Copy item path
- Methods from Deref<Target = World>§
    - pub fn register_event_key<E>(&mut self) -> EventKeywhere E: Event,
    - pub fn event_key<E>(&self) -> Option<EventKey>where E: Event,
    - pub fn add_observer<E, B, M>( &mut self, system: impl IntoObserverSystem<E, B, M>, ) -> EntityWorldMut<'_>where E: Event, B: Bundle,
      - §Example
      - §Panics
    - pub fn trigger<'a, E>(&mut self, event: E)where E: Event, <E as Event>::Trigger<'a>: Default,
    - pub fn trigger_targets<'a>(&mut self, event: impl Event : Default>)where <impl Event as Event>::Trigger<'a>: Default,
    - pub fn trigger_with<'a, E>( &mut self, event: E, trigger: <E as Event>::Trigger<'a>, )where E: Event,

The simulation World of the application, stored as a resource.

This resource is only available during ExtractSchedule and not during command application of that schedule. See Extract for more details.

Generates the EventKey for this event type.

If this type has already been registered, this will return the existing EventKey.

This is used by various dynamically typed observer APIs, such as DeferredWorld::trigger_raw.

Fetches the EventKey for this event type, if it has already been generated.

This is used by various dynamically typed observer APIs, such as DeferredWorld::trigger_raw.

Spawns a “global” Observer which will watch for the given event. Returns its Entity as a EntityWorldMut.

system can be any system whose first parameter is On.

Calling observe on the returned EntityWorldMut will observe the observer itself, which you very likely do not want.

Panics if the given system is an exclusive system.

Triggers the given Event, which will run any Observers watching for it.

For a variant that borrows the event rather than consuming it, use World::trigger_ref instead.

A deprecated alias for trigger to ease migration.

Instead of specifying the trigger target separately, information about the target of the event is embedded in the data held by the event type itself.

Triggers the given Event using the given Trigger, which will run any Observers watching for it.

For a variant that borrows the event rather than consuming it, use World::trigger_ref instead.

Triggers the given mutable Event reference, which will run any Observers watching for it.

Compared to World::trigger, this method is most useful when it’s necessary to check or use the event after it has been modified by observers.

Triggers the given mutable Event reference using the given mutable Trigger reference, which will run any Observers watching for it.

Compared to World::trigger, this method is most useful when it’s necessary to check or use the event after it has been modified by observers.

Registers a system and returns a SystemId so it can later be called by World::run_system.

It’s possible to register multiple copies of the same system by calling this function multiple times. If that’s not what you want, consider using World::register_system_cached instead.

This is different from adding systems to a Schedule, because the SystemId that is returned can be used anywhere in the World to run the associated system. This allows for running systems in a pushed-based fashion. Using a S

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct MainWorld(/* private fields */);
```

Example 2 (unknown):
```unknown
#[derive(Component)]
struct A;

world.add_observer(|_: On<Add, A>| {
    // ...
});
world.add_observer(|_: On<Remove, A>| {
    // ...
});
```

Example 3 (javascript):
```javascript
47fn setup_with_world(world: &mut World) {
48    // We can run it once manually
49    world.run_system_once(system_b).unwrap();
50    // Or with a Callback
51    let system_id = world.register_system(system_b);
52    world.spawn((Callback(system_id), B));
53}
```

Example 4 (javascript):
```javascript
fn increment(mut counter: Local<u8>) {
   *counter += 1;
   println!("{}", *counter);
}

let mut world = World::default();
let counter_one = world.register_system(increment);
let counter_two = world.register_system(increment);
world.run_system(counter_one); // -> 1
world.run_system(counter_one); // -> 2
world.run_system(counter_two); // -> 1
```

---

## Struct MaterialExtractionSystems Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialExtractionSystems.html

**Contents:**
- Struct MaterialExtractionSystems Copy item path
- Trait Implementations§
  - impl Clone for MaterialExtractionSystems
    - fn clone(&self) -> MaterialExtractionSystems
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for MaterialExtractionSystems
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Hash for MaterialExtractionSystems
    - fn hash<__H>(&self, state: &mut __H)where __H: Hasher,
    - fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,

A SystemSet that contains all extract_mesh_materials systems.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialExtractionSystems;
```

---

## Function update_point_light_frusta Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/fn.update_point_light_frusta.html

**Contents:**
- Function update_point_light_frusta Copy item path

bevy::lightFunction update_point_light_frusta Copy item pathSource pub fn update_point_light_frusta( global_lights: Res<'_, GlobalVisibleClusterableObjects>, views: Query<'_, '_, (Entity, &GlobalTransform, &PointLight, &mut CubemapFrusta)>, changed_lights: Query<'_, '_, Entity, (With<PointLight>, Or<(Changed<GlobalTransform>, Changed<PointLight>)>)>, )

**Examples:**

Example 1 (unknown):
```unknown
pub fn update_point_light_frusta(
    global_lights: Res<'_, GlobalVisibleClusterableObjects>,
    views: Query<'_, '_, (Entity, &GlobalTransform, &PointLight, &mut CubemapFrusta)>,
    changed_lights: Query<'_, '_, Entity, (With<PointLight>, Or<(Changed<GlobalTransform>, Changed<PointLight>)>)>,
)
```

---

## Struct AssetEventSystems Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.AssetEventSystems.html

**Contents:**
- Struct AssetEventSystems Copy item path
- Trait Implementations§
  - impl Clone for AssetEventSystems
    - fn clone(&self) -> AssetEventSystems
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for AssetEventSystems
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Hash for AssetEventSystems
    - fn hash<__H>(&self, state: &mut __H)where __H: Hasher,
    - fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,

A system set where events accumulated in Assets are applied to the AssetEvent Messages resource.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AssetEventSystems;
```

---

## Struct Camera Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.Camera.html

**Contents:**
- Struct Camera Copy item path
- Fields§
- Implementations§
  - impl Camera
    - pub fn to_logical(&self, physical_size: UVec2) -> Option<Vec2>
    - pub fn physical_viewport_rect(&self) -> Option<URect>
    - pub fn logical_viewport_rect(&self) -> Option<Rect>
    - pub fn logical_viewport_size(&self) -> Option<Vec2>
    - pub fn physical_viewport_size(&self) -> Option<UVec2>
    - pub fn logical_target_size(&self) -> Option<Vec2>

The defining Component for camera entities, storing information about how and what to render through this camera.

The Camera component is added to an entity to define the properties of the viewpoint from which rendering occurs. It defines the position of the view to render, the projection method to transform the 3D objects into a 2D image, as well as the render target into which that image is produced.

Note that a Camera needs a CameraRenderGraph to render anything. This is typically provided by adding a Camera2d or Camera3d component, but custom render graphs can also be defined. Inserting a Camera with no render graph will emit an error at runtime.

If set, this camera will render to the given Viewport rectangle within the configured RenderTarget.

Cameras with a higher order are rendered later, and thus on top of lower order cameras.

If this is set to true, this camera will be rendered to its specified RenderTarget. If false, this camera will not be rendered.

Computed values for this camera, such as the projection matrix and the render target size.

The “target” that this camera will render to.

The CameraOutputMode for this camera.

If this is enabled, a previous camera exists that shares this camera’s render target, and this camera has MSAA enabled, then the previous camera’s outputs will be written to the intermediate multi-sampled render target textures for this camera. This enables cameras with MSAA enabled to “write their results on top” of previous camera results, and include them as a part of their render results. This is enabled by default to ensure cameras with MSAA enabled layer their results in the same way as cameras without MSAA enabled by default.

The clear color operation to perform on the render target.

If set, this camera will be a sub camera of a large view, defined by a SubCameraView.

Converts a physical size in this Camera to a logical size.

The rendered physical bounds URect of the camera. If the viewport field is set to Some, this will be the rect of that custom viewport. Otherwise it will default to the full physical rect of the current RenderTarget.

The rendered logical bounds Rect of the camera. If the viewport field is set to Some, this will be the rect of that custom viewport. Otherwise it will default to the full logical rect of the current RenderTarget.

The logical size of this camera’s viewport. If the viewport field is set to Some, this will be the size of that custom viewport. Otherwise it will default to the f

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Camera {
    pub viewport: Option<Viewport>,
    pub order: isize,
    pub is_active: bool,
    pub computed: ComputedCameraValues,
    pub target: RenderTarget,
    pub output_mode: CameraOutputMode,
    pub msaa_writeback: bool,
    pub clear_color: ClearColorConfig,
    pub sub_camera_view: Option<SubCameraView>,
}
```

Example 2 (javascript):
```javascript
22fn draw_cursor(
23    camera_query: Single<(&Camera, &GlobalTransform)>,
24    window: Single<&Window>,
25    mut gizmos: Gizmos,
26) {
27    let (camera, camera_transform) = *camera_query;
28
29    if let Some(cursor_position) = window.cursor_position()
30        // Calculate a world position based on the cursor's position.
31        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
32        // To test Camera::world_to_viewport, convert result back to viewport space and then back to world space.
33        && let Ok(viewport_check) = camera.world_to_viewp
...
```

Example 3 (javascript):
```javascript
247fn example_control_system(
248    mut materials: ResMut<Assets<StandardMaterial>>,
249    controllable: Query<(&MeshMaterial3d<StandardMaterial>, &ExampleControls)>,
250    camera: Single<
251        (
252            Entity,
253            &mut Camera,
254            &mut Transform,
255            &GlobalTransform,
256            Has<Hdr>,
257        ),
258        With<Camera3d>,
259    >,
260    mut labels: Query<(&mut Node, &ExampleLabel)>,
261    mut display: Single<&mut Text, With<ExampleDisplay>>,
262    labeled: Query<&GlobalTransform>,
263    mut state: Local<ExampleState>,
264    ti
...
```

Example 4 (javascript):
```javascript
fn system(camera_query: Single<(&Camera, &GlobalTransform)>, window: Single<&Window>) {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        // Calculate a ray pointing from the camera into the world based on the cursor's position.
        && let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position)
    {
        println!("{ray:?}");
    }
}

// Run the system after transform propagation so the camera's global transform is up-to-date.
app.add_systems(PostUpdate, system.after(TransformSystems::Propagate));
```

---

## Struct LightViewEntities Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.LightViewEntities.html

**Contents:**
- Struct LightViewEntities Copy item path
- Methods from Deref<Target = EntityHashMap<Vec<Entity>>>§
    - pub fn keys(&self) -> Keys<'_, V> ⓘ
- Methods from Deref<Target = HashMap<Entity, V, EntityHash>>§
    - pub fn hasher(&self) -> &S
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn keys(&self) -> Keys<'_, K, V> ⓘ
      - §Examples
      - Examples found in repository?

Component automatically attached to a light entity to track light-view entities for each view.

An iterator visiting all keys in arbitrary order. The iterator element type is &'a Entity.

Equivalent to HashMap::keys.

Returns a reference to the map’s BuildHasher, or S parameter.

Refer to hasher for further details.

Returns the number of elements the map can hold without reallocating.

Refer to capacity for further details.

An iterator visiting all keys in arbitrary order. The iterator element type is &'a K.

Refer to keys for further details.

An iterator visiting all values in arbitrary order. The iterator element type is &'a V.

Refer to values for further details.

An iterator visiting all values mutably in arbitrary order. The iterator element type is &'a mut V.

Refer to values for further details.

An iterator visiting all key-value pairs in arbitrary order. The iterator element type is (&'a K, &'a V).

Refer to iter for further details.

An iterator visiting all key-value pairs in arbitrary order, with mutable references to the values. The iterator element type is (&'a K, &'a mut V).

Refer to iter_mut for further details.

Returns the number of elements in the map.

Refer to len for further details.

Returns true if the map contains no elements.

Refer to is_empty for further details.

Clears the map, returning all key-value pairs as an iterator. Keeps the allocated memory for reuse.

Refer to drain for further details.

Retains only the elements specified by the predicate. Keeps the allocated memory for reuse.

Refer to retain for further details.

Drains elements which are true under the given predicate, and returns an iterator over the removed items.

Refer to extract_if for further details.

Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.

Refer to clear for further details.

Reserves capacity for at least additional more elements to be inserted in the HashMap. The collection may reserve more space to avoid frequent reallocations.

Refer to reserve for further details.

Tries to reserve capacity for at least additional more elements to be inserted in the given HashMap<K,V>. The collection may reserve more space to avoid frequent reallocations.

Refer to try_reserve for further details.

Shrinks the capacity of the map as much as possible. It will drop down as much as possible while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.

Refer to shrink_to_fit f

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct LightViewEntities(/* private fields */);
```

Example 2 (javascript):
```javascript
let map = HashMap::with_capacity(5);

assert!(map.capacity() >= 5);
```

Example 3 (javascript):
```javascript
let mut map = HashMap::new();

map.insert("foo", 0);
map.insert("bar", 1);
map.insert("baz", 2);

for key in map.keys() {
    // foo, bar, baz
    // Note that the above order is not guaranteed
}
```

Example 4 (javascript):
```javascript
25fn send_scroll_events(
26    mut mouse_wheel_reader: MessageReader<MouseWheel>,
27    hover_map: Res<HoverMap>,
28    keyboard_input: Res<ButtonInput<KeyCode>>,
29    mut commands: Commands,
30) {
31    for mouse_wheel in mouse_wheel_reader.read() {
32        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);
33
34        if mouse_wheel.unit == MouseScrollUnit::Line {
35            delta *= LINE_HEIGHT;
36        }
37
38        if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
39            std::mem::swap(&mut delta.x, &mut delta.y);
40        }
41
42     
...
```

---

## Struct RumbleSystems Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gilrs/struct.RumbleSystems.html

**Contents:**
- Struct RumbleSystems Copy item path
- Trait Implementations§
  - impl Clone for RumbleSystems
    - fn clone(&self) -> RumbleSystems
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for RumbleSystems
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Hash for RumbleSystems
    - fn hash<__H>(&self, state: &mut __H)where __H: Hasher,
    - fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,

Updates the running gamepad rumble effects.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RumbleSystems;
```

---

## Module component Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/component/index.html

**Contents:**
- Module component Copy item path
- Structs§
- Enums§
- Traits§
- Functions§
- Type Aliases§
- Derive Macros§

Types for declaring and storing Components.

---

## Module lifecycle Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/lifecycle/index.html

**Contents:**
- Module lifecycle Copy item path
- §Types of lifecycle events
  - §Lifecycle events and component types
- Structs§
- Constants§
- Type Aliases§

This module contains various tools to allow you to react to component insertion or removal, as well as entity spawning and despawning.

There are four main ways to react to these lifecycle events:

There are five types of lifecycle events, split into two categories. First, we have lifecycle events that are triggered when a component is added to an entity:

When both events occur, Add hooks are evaluated before Insert.

Next, we have lifecycle events that are triggered when a component is removed from an entity:

Replace hooks are evaluated before Remove, then finally Despawn hooks are evaluated.

Add and Remove are counterparts: they are only triggered when a component is added or removed from an entity in such a way as to cause a change in the component’s presence on that entity. Similarly, Insert and Replace are counterparts: they are triggered when a component is added or replaced on an entity, regardless of whether this results in a change in the component’s presence on that entity.

To reliably synchronize data structures using with component lifecycle events, you can combine Insert and Replace to fully capture any changes to the data. This is particularly useful in combination with immutable components, to avoid any lifecycle-bypassing mutations.

Despite the absence of generics, each lifecycle event is associated with a specific component. When defining a component hook for a Component type, that component is used. When observers watch lifecycle events, the B: Bundle generic is used.

Each of these lifecycle events also corresponds to a fixed ComponentId, which are assigned during World initialization. For example, Add corresponds to ADD. This is used to skip TypeId lookups in hot paths.

---

## Trait WorldQuery Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/query/trait.WorldQuery.html

**Contents:**
- Trait WorldQuery Copy item path
- §Safety
- Required Associated Constants§
    - const IS_DENSE: bool
- Required Associated Types§
    - type Fetch<'w>: Clone
    - type State: Send + Sync
- Required Methods§
    - fn shrink_fetch<'wlong, 'wshort>( fetch: Self::Fetch<'wlong>, ) -> Self::Fetch<'wshort>where 'wlong: 'wshort,
    - unsafe fn init_fetch<'w, 's>( world: UnsafeWorldCell<'w>, state: &'s Self::State, last_run: Tick, this_run: Tick, ) -> Self::Fetch<'w>

Types that can be used as parameters in a Query. Types that implement this should also implement either QueryData or QueryFilter

Implementor must ensure that update_component_access, QueryData::provide_extra_access, matches_component_set, QueryData::fetch, QueryFilter::filter_fetch and init_fetch obey the following:

When implementing update_component_access, note that add_read and add_write both also add a With filter, whereas extend_access does not change the filters.

Returns true if (and only if) every table of every archetype matched by this fetch contains all of the matched components.

This is used to select a more efficient “table iterator” for “dense” queries. If this returns true, WorldQuery::set_table must be used before QueryData::fetch can be called for iterators. If this returns false, WorldQuery::set_archetype must be used before QueryData::fetch can be called for iterators.

Per archetype/table state retrieved by this WorldQuery to compute Self::Item for each entity.

State used to construct a Self::Fetch. This will be cached inside QueryState, so it is best to move as much data / computation here as possible to reduce the cost of constructing Self::Fetch.

This function manually implements subtyping for the query fetches.

Creates a new instance of Self::Fetch, by combining data from the World with the cached Self::State. Readonly accesses resources registered in WorldQuery::update_component_access.

Adjusts internal state to account for the next Archetype. This will always be called on archetypes that match this WorldQuery.

Adjusts internal state to account for the next Table. This will always be called on tables that match this WorldQuery.

Adds any component accesses used by this WorldQuery to access.

Used to check which queries are disjoint and can run in parallel

Creates and initializes a State for this WorldQuery type.

Attempts to initialize a State for this WorldQuery type using read-only access to Components.

Returns true if this query matches a set of components. Otherwise, returns false.

Used to check which Archetypes can be skipped by the query (if none of the Components match). This is how archetypal query filters like With work.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

SAFETY: fetch accesses are the conjunction of the subqueries’ accesses This is sound because update_component_access adds accesses according to the impl

*[Content truncated]*

**Examples:**

Example 1 (javascript):
```javascript
pub unsafe trait WorldQuery {
    type Fetch<'w>: Clone;
    type State: Send + Sync;

    const IS_DENSE: bool;

    // Required methods
    fn shrink_fetch<'wlong, 'wshort>(
        fetch: Self::Fetch<'wlong>,
    ) -> Self::Fetch<'wshort>
       where 'wlong: 'wshort;
    unsafe fn init_fetch<'w, 's>(
        world: UnsafeWorldCell<'w>,
        state: &'s Self::State,
        last_run: Tick,
        this_run: Tick,
    ) -> Self::Fetch<'w>;
    unsafe fn set_archetype<'w, 's>(
        fetch: &mut Self::Fetch<'w>,
        state: &'s Self::State,
        archetype: &'w Archetype,
        tabl
...
```

---

## Function extract_meshes_for_gpu_building Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/fn.extract_meshes_for_gpu_building.html

**Contents:**
- Function extract_meshes_for_gpu_building Copy item path

Extracts meshes from the main world into the render world and queues MeshInputUniforms to be uploaded to the GPU.

This is optimized to only look at entities that have changed since the last frame.

This is the variant of the system that runs when we’re using GPU MeshUniform building.

**Examples:**

Example 1 (unknown):
```unknown
pub fn extract_meshes_for_gpu_building(
    render_mesh_instances: ResMut<'_, RenderMeshInstances>,
    render_visibility_ranges: Res<'_, RenderVisibilityRanges>,
    render_mesh_instance_queues: ResMut<'_, RenderMeshInstanceGpuQueues>,
    changed_meshes_query: Extract<'_, '_, Query<'_, '_, (Entity, &'static ViewVisibility, &'static GlobalTransform, Option<&'static PreviousGlobalTransform>, Option<&'static Lightmap>, Option<&'static Aabb>, &'static Mesh3d, Option<&'static MeshTag>, Has<NoFrustumCulling>, Has<NotShadowReceiver>, Has<TransmittedShadowReceiver>, Has<NotShadowCaster>, Has<NoAutom
...
```

---

## Module label Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/label/index.html

**Contents:**
- Module label Copy item path
- Structs§
- Traits§

Traits used by label implementations

---

## Struct RenderMaterialInstances Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.RenderMaterialInstances.html

**Contents:**
- Struct RenderMaterialInstances Copy item path
- Fields§
- Trait Implementations§
  - impl Default for RenderMaterialInstances
    - fn default() -> RenderMaterialInstances
  - impl Resource for RenderMaterialInstanceswhere RenderMaterialInstances: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for RenderMaterialInstances
  - impl RefUnwindSafe for RenderMaterialInstances
  - impl Send for RenderMaterialInstances

Stores all extracted instances of all Materials in the render world.

Maps from each entity in the main world to the RenderMaterialInstance associated with it.

A monotonically-increasing counter, which we use to sweep RenderMaterialInstances::instances when the entities and/or required components are removed.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderMaterialInstances {
    pub instances: HashMap<MainEntity, RenderMaterialInstance, EntityHash>,
    pub current_change_tick: Tick,
}
```

---

## Module spawn Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ecs/spawn/index.html

**Contents:**
- Module spawn Copy item path
- Structs§
- Traits§

Entity spawning abstractions, largely focused on spawning related hierarchies of entities. See related and SpawnRelated for the best entry points into these APIs and examples of how to use them.

---

## Struct SystemInformationDiagnosticsPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/struct.SystemInformationDiagnosticsPlugin.html

**Contents:**
- Struct SystemInformationDiagnosticsPlugin Copy item path
- §See also
- Implementations§
  - impl SystemInformationDiagnosticsPlugin
    - pub const SYSTEM_CPU_USAGE: DiagnosticPath
    - pub const SYSTEM_MEM_USAGE: DiagnosticPath
    - pub const PROCESS_CPU_USAGE: DiagnosticPath
    - pub const PROCESS_MEM_USAGE: DiagnosticPath
- Trait Implementations§
  - impl Default for SystemInformationDiagnosticsPlugin

Adds a System Information Diagnostic, specifically cpu_usage (in %) and mem_usage (in %)

Note that gathering system information is a time intensive task and therefore can’t be done on every frame. Any system diagnostics gathered by this plugin may not be current when you access them.

NOT supported when using the bevy/dynamic feature even when using previously mentioned targets.

LogDiagnosticsPlugin to output diagnostics to the console.

Total system cpu usage in %

Total system memory usage in %

Process cpu usage in %

Process memory usage in %

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct SystemInformationDiagnosticsPlugin;
```

---

## Type Alias GizmoRenderSystem Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/type.GizmoRenderSystem.html

**Contents:**
- Type Alias GizmoRenderSystem Copy item path
- Aliased Type§
- Variants§
  - QueueLineGizmos2d
  - QueueLineGizmos3d

Deprecated alias for GizmoRenderSystems.

Adds gizmos to the Transparent2d render phase

Adds gizmos to the Transparent3d render phase

**Examples:**

Example 1 (unknown):
```unknown
pub type GizmoRenderSystem = GizmoRenderSystems;
```

Example 2 (unknown):
```unknown
pub enum GizmoRenderSystem {
    QueueLineGizmos2d,
    QueueLineGizmos3d,
}
```

---
