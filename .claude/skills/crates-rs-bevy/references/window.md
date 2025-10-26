# Crates-Rs-Bevy - Window

**Pages:** 47

---

## Struct ScreenSpaceReflectionsPipelineId Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ScreenSpaceReflectionsPipelineId.html

**Contents:**
- Struct ScreenSpaceReflectionsPipelineId Copy item path
- Tuple Fields§
- Methods from Deref<Target = CachedRenderPipelineId>§
    - pub const INVALID: CachedRenderPipelineId
    - pub fn id(&self) -> usize
- Trait Implementations§
  - impl Component for ScreenSpaceReflectionsPipelineIdwhere ScreenSpaceReflectionsPipelineId: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

Identifies which screen space reflections render pipeline a view needs.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ScreenSpaceReflectionsPipelineId(pub CachedRenderPipelineId);
```

---

## Struct VideoMode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.VideoMode.html

**Contents:**
- Struct VideoMode Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for VideoMode
    - fn clone(&self) -> VideoMode
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for VideoMode
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for VideoMode
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<VideoMode, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,

Represents a video mode that a monitor supports

The resolution of the video mode

The bit depth of the video mode

The refresh rate in millihertz

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct VideoMode {
    pub physical_size: UVec2,
    pub bit_depth: u16,
    pub refresh_rate_millihertz: u32,
}
```

---

## Function exit_on_primary_closed Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/fn.exit_on_primary_closed.html

**Contents:**
- Function exit_on_primary_closed Copy item path

Exit the application when the primary window has been closed

This system is added by the WindowPlugin

**Examples:**

Example 1 (unknown):
```unknown
pub fn exit_on_primary_closed(
    app_exit_writer: MessageWriter<'_, AppExit>,
    windows: Query<'_, '_, (), (With<Window>, With<PrimaryWindow>)>,
)
```

---

## Crate window Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/index.html

**Contents:**
- Crate window Copy item path
- Modules§
- Structs§
- Enums§
- Functions§

bevy_window provides a platform-agnostic interface for windowing in Bevy.

This crate contains types for window management and events, used by windowing implementors such as bevy_winit. The WindowPlugin sets up some global window-related parameters and is part of the DefaultPlugins.

---

## Struct WindowCreated Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.WindowCreated.html

**Contents:**
- Struct WindowCreated Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for WindowCreated
    - fn clone(&self) -> WindowCreated
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for WindowCreated
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for WindowCreated
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<WindowCreated, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,

An event that is sent whenever a new window is created.

To create a new window, spawn an entity with a crate::Window on it.

Window that has been created.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WindowCreated {
    pub window: Entity,
}
```

---

## Struct InternalWindowState Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.InternalWindowState.html

**Contents:**
- Struct InternalWindowState Copy item path
- Implementations§
  - impl InternalWindowState
    - pub fn take_maximize_request(&mut self) -> Option<bool>
    - pub fn take_minimize_request(&mut self) -> Option<bool>
    - pub fn take_move_request(&mut self) -> bool
    - pub fn take_resize_request(&mut self) -> Option<CompassOctant>
- Trait Implementations§
  - impl Clone for InternalWindowState
    - fn clone(&self) -> InternalWindowState

Stores internal Window state that isn’t directly accessible.

Consumes the current maximize request, if it exists. This should only be called by window backends.

Consumes the current minimize request, if it exists. This should only be called by window backends.

Consumes the current move request, if it exists. This should only be called by window backends.

Consumes the current resize request, if it exists. This should only be called by window backends.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct InternalWindowState { /* private fields */ }
```

---

## Struct RequestRedraw Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.RequestRedraw.html

**Contents:**
- Struct RequestRedraw Copy item path
- Trait Implementations§
  - impl Clone for RequestRedraw
    - fn clone(&self) -> RequestRedraw
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for RequestRedraw
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for RequestRedraw
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<RequestRedraw, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,
  - impl From<RequestRedraw> for WindowEvent

An event that indicates all of the application’s windows should be redrawn, even if their control flow is set to Wait and there have been no window events.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RequestRedraw;
```

---

## Struct ScreenSpaceReflectionsNode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ScreenSpaceReflectionsNode.html

**Contents:**
- Struct ScreenSpaceReflectionsNode Copy item path
- Trait Implementations§
  - impl Default for ScreenSpaceReflectionsNode
    - fn default() -> ScreenSpaceReflectionsNode
  - impl ViewNode for ScreenSpaceReflectionsNode
    - type ViewQuery = (&'static ViewTarget, &'static ViewUniformOffset, &'static ViewLightsUniformOffset, &'static ViewFogUniformOffset, &'static ViewLightProbesUniformOffset, &'static ViewScreenSpaceReflectionsUniformOffset, &'static ViewEnvironmentMapUniformOffset, &'static MeshViewBindGroup, &'static ScreenSpaceReflectionsPipelineId)
    - fn run<'w>( &self, _: &mut RenderGraphContext<'_>, render_context: &mut RenderContext<'w>, _: <<ScreenSpaceReflectionsNode as ViewNode>::ViewQuery as QueryData>::Item<'w, '_>, world: &'w World, ) -> Result<(), NodeRunError>
    - fn update(&mut self, _world: &mut World)
- Auto Trait Implementations§
  - impl Freeze for ScreenSpaceReflectionsNode

The node in the render graph that traces screen space reflections.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ScreenSpaceReflectionsNode;
```

---

## Enum WindowRef Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.WindowRef.html

**Contents:**
- Enum WindowRef Copy item path
- Variants§
  - Primary
  - Entity(Entity)
- Implementations§
  - impl WindowRef
    - pub fn normalize( &self, primary_window: Option<Entity>, ) -> Option<NormalizedWindowRef>
- Trait Implementations§
  - impl Clone for WindowRef
    - fn clone(&self) -> WindowRef

Reference to a Window, whether it be a direct link to a specific entity or a more vague defaulting choice.

This will be linked to the primary window that is created by default in the WindowPlugin.

A more direct link to a window entity.

Use this if you want to reference a secondary/tertiary/… window.

To create a new window you can spawn an entity with a Window, then you can use that entity here for usage in cameras.

Normalize the window reference so that it can be compared to other window references.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub enum WindowRef {
    Primary,
    Entity(Entity),
}
```

---

## Struct PrimaryMonitor Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.PrimaryMonitor.html

**Contents:**
- Struct PrimaryMonitor Copy item path
- Trait Implementations§
  - impl Clone for PrimaryMonitor
    - fn clone(&self) -> PrimaryMonitor
    - fn clone_from(&mut self, source: &Self)
  - impl Component for PrimaryMonitorwhere PrimaryMonitor: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior

A marker component for the primary monitor

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PrimaryMonitor;
```

---

## Struct CursorEntered Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.CursorEntered.html

**Contents:**
- Struct CursorEntered Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for CursorEntered
    - fn clone(&self) -> CursorEntered
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for CursorEntered
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for CursorEntered
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<CursorEntered, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,

An event that is sent whenever the user’s cursor enters a window.

Window that the cursor entered.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CursorEntered {
    pub window: Entity,
}
```

---

## Struct WindowFocused Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.WindowFocused.html

**Contents:**
- Struct WindowFocused Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for WindowFocused
    - fn clone(&self) -> WindowFocused
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for WindowFocused
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for WindowFocused
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<WindowFocused, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,

An event that indicates a window has received or lost focus.

Window that changed focus.

Whether it was focused (true) or lost focused (false).

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WindowFocused {
    pub window: Entity,
    pub focused: bool,
}
```

---

## Struct ClosingWindow Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.ClosingWindow.html

**Contents:**
- Struct ClosingWindow Copy item path
- Trait Implementations§
  - impl Component for ClosingWindowwhere ClosingWindow: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior
    - fn on_add() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_insert() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_replace() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>

Marker component for a Window that has been requested to close and is in the process of closing (on the next frame).

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ClosingWindow;
```

---

## Struct ScreenSpaceReflectionsUniform Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ScreenSpaceReflectionsUniform.html

**Contents:**
- Struct ScreenSpaceReflectionsUniform Copy item path
- Trait Implementations§
  - impl Clone for ScreenSpaceReflectionsUniform
    - fn clone(&self) -> ScreenSpaceReflectionsUniform
    - fn clone_from(&mut self, source: &Self)
  - impl Component for ScreenSpaceReflectionsUniformwhere ScreenSpaceReflectionsUniform: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior

A version of ScreenSpaceReflections for upload to the GPU.

For more information on these fields, see the corresponding documentation in ScreenSpaceReflections.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ScreenSpaceReflectionsUniform { /* private fields */ }
```

---

## Struct ScreenSpaceAmbientOcclusionPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ScreenSpaceAmbientOcclusionPlugin.html

**Contents:**
- Struct ScreenSpaceAmbientOcclusionPlugin Copy item path
- Trait Implementations§
  - impl Plugin for ScreenSpaceAmbientOcclusionPlugin
    - fn build(&self, app: &mut App)
    - fn finish(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str
    - fn is_unique(&self) -> bool
- Auto Trait Implementations§

Plugin for screen space ambient occlusion.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ScreenSpaceAmbientOcclusionPlugin;
```

---

## Struct ViewScreenSpaceReflectionsUniformOffset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ViewScreenSpaceReflectionsUniformOffset.html

**Contents:**
- Struct ViewScreenSpaceReflectionsUniformOffset Copy item path
- Methods from Deref<Target = u32>§
    - pub const MIN: u32 = 0u32
    - pub const MAX: u32 = 4_294_967_295u32
    - pub const BITS: u32 = 32u32
- Trait Implementations§
  - impl Component for ViewScreenSpaceReflectionsUniformOffsetwhere ViewScreenSpaceReflectionsUniformOffset: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

A component that stores the offset within the ScreenSpaceReflectionsBuffer for each view.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ViewScreenSpaceReflectionsUniformOffset(/* private fields */);
```

---

## Enum WindowTheme Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.WindowTheme.html

**Contents:**
- Enum WindowTheme Copy item path
- Variants§
  - Light
  - Dark
- Trait Implementations§
  - impl Clone for WindowTheme
    - fn clone(&self) -> WindowTheme
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for WindowTheme
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

The Window theme variant to use.

Use the light variant.

Use the dark variant.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum WindowTheme {
    Light,
    Dark,
}
```

---

## Enum VideoModeSelection Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.VideoModeSelection.html

**Contents:**
- Enum VideoModeSelection Copy item path
- Variants§
  - Current
  - Specific(VideoMode)
- Trait Implementations§
  - impl Clone for VideoModeSelection
    - fn clone(&self) -> VideoModeSelection
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for VideoModeSelection
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

References an exclusive fullscreen video mode.

Used when setting WindowMode::Fullscreen on a window.

Uses the video mode that the monitor is already in.

Uses a given crate::monitor::VideoMode. A list of video modes supported by the monitor is supplied by crate::monitor::Monitor::video_modes.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum VideoModeSelection {
    Current,
    Specific(VideoMode),
}
```

---

## Struct ScreenSpaceReflectionsPipelineKey Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ScreenSpaceReflectionsPipelineKey.html

**Contents:**
- Struct ScreenSpaceReflectionsPipelineKey Copy item path
- Trait Implementations§
  - impl Clone for ScreenSpaceReflectionsPipelineKey
    - fn clone(&self) -> ScreenSpaceReflectionsPipelineKey
    - fn clone_from(&mut self, source: &Self)
  - impl Hash for ScreenSpaceReflectionsPipelineKey
    - fn hash<__H>(&self, state: &mut __H)where __H: Hasher,
    - fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,
  - impl PartialEq for ScreenSpaceReflectionsPipelineKey
    - fn eq(&self, other: &ScreenSpaceReflectionsPipelineKey) -> bool

Identifies a specific configuration of the SSR pipeline shader.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ScreenSpaceReflectionsPipelineKey { /* private fields */ }
```

---

## Struct WindowResolution Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.WindowResolution.html

**Contents:**
- Struct WindowResolution Copy item path
  - §Physical, logical and requested sizes
  - §Scale factor
  - §Requested and obtained sizes
- Implementations§
  - impl WindowResolution
    - pub fn new(physical_width: u32, physical_height: u32) -> WindowResolution
      - Examples found in repository?
    - pub fn with_scale_factor_override( self, scale_factor_override: f32, ) -> WindowResolution
      - Examples found in repository?

Controls the size of a Window

There are three sizes associated with a window:

The reason logical size and physical size are separated and can be different is to account for the cases where:

The factor between physical size and logical size can be retrieved with WindowResolution::scale_factor.

For the first two cases, a scale factor is set automatically by the operating system through the window backend. You can get it with WindowResolution::base_scale_factor.

For the third case, you can override this automatic scale factor with WindowResolution::set_scale_factor_override.

The logical size should be equal to the requested size after creating/resizing, when possible. The reason the requested size and logical size might be different is because the corresponding physical size might exceed limits (either the size limits of the monitor, or limits defined in WindowResizeConstraints).

Note: The requested size is not kept in memory, for example requesting a size too big for the screen, making the logical size different from the requested size, and then setting a scale factor that makes the previous requested size within the limits of the screen will not get back that previous requested size.

Creates a new WindowResolution.

Builder method for adding a scale factor override to the resolution.

The window’s client area width in logical pixels.

The window’s client area height in logical pixels.

The window’s client size in logical pixels

The window’s client area width in physical pixels.

The window’s client area height in physical pixels.

The window’s client size in physical pixels

The ratio of physical pixels to logical pixels.

physical_pixels = logical_pixels * scale_factor

The window scale factor as reported by the window backend.

This value is unaffected by WindowResolution::scale_factor_override.

The scale factor set with WindowResolution::set_scale_factor_override.

This value may be different from the scale factor reported by the window backend.

Set the window’s logical resolution.

Set the window’s physical resolution.

This will ignore the scale factor setting, so most of the time you should prefer to use WindowResolution::set.

Set the window’s scale factor, this may get overridden by the backend.

Set the window’s scale factor, this will be used over what the backend decides.

This can change the logical and physical sizes if the resulting physical size is not within the limits.

Returns the argument unchanged.

Creates Self using default(

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct WindowResolution { /* private fields */ }
```

Example 2 (unknown):
```unknown
9fn main() {
10    App::new()
11        .add_plugins(DefaultPlugins.set(WindowPlugin {
12            primary_window: Some(Window {
13                resolution: WindowResolution::new(500, 300).with_scale_factor_override(1.0),
14                ..default()
15            }),
16            ..default()
17        }))
18        .add_systems(Startup, setup)
19        .add_systems(
20            Update,
21            (display_override, toggle_override, change_scale_factor),
22        )
23        .run();
24}
```

Example 3 (unknown):
```unknown
13fn main() {
14    App::new()
15        .add_plugins(DefaultPlugins.set(WindowPlugin {
16            primary_window: Some(Window {
17                present_mode: PresentMode::AutoNoVsync,
18                resolution: WindowResolution::new(1920, 1080).with_scale_factor_override(1.0),
19                ..default()
20            }),
21            ..default()
22        }))
23        .insert_resource(WinitSettings::continuous())
24        .add_systems(Startup, setup)
25        .add_systems(Update, rotate_cameras)
26        .run();
27}
```

Example 4 (unknown):
```unknown
14fn main() {
15    App::new()
16        .add_plugins((
17            DefaultPlugins.set(WindowPlugin {
18                primary_window: Some(Window {
19                    present_mode: PresentMode::AutoNoVsync,
20                    resolution: WindowResolution::new(1920, 1080).with_scale_factor_override(1.0),
21                    ..default()
22                }),
23                ..default()
24            }),
25            FrameTimeDiagnosticsPlugin::default(),
26            LogDiagnosticsPlugin::default(),
27        ))
28        .insert_resource(WinitSettings::continuous())
29        .a
...
```

---

## Struct WindowResizeConstraints Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.WindowResizeConstraints.html

**Contents:**
- Struct WindowResizeConstraints Copy item path
- Fields§
- Implementations§
  - impl WindowResizeConstraints
    - pub fn check_constraints(&self) -> WindowResizeConstraints
- Trait Implementations§
  - impl Clone for WindowResizeConstraints
    - fn clone(&self) -> WindowResizeConstraints
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for WindowResizeConstraints

The size limits on a Window.

These values are measured in logical pixels (see WindowResolution), so the user’s scale factor does affect the size limits on the window.

Please note that if the window is resizable, then when the window is maximized it may have a size outside of these limits. The functionality required to disable maximizing is not yet exposed by winit.

The minimum width the window can have.

The minimum height the window can have.

The maximum width the window can have.

The maximum height the window can have.

Checks if the constraints are valid.

Will output warnings if it isn’t.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WindowResizeConstraints {
    pub min_width: f32,
    pub min_height: f32,
    pub max_width: f32,
    pub max_height: f32,
}
```

---

## Function exit_on_all_closed Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/fn.exit_on_all_closed.html

**Contents:**
- Function exit_on_all_closed Copy item path

Exit the application when there are no open windows.

This system is added by the WindowPlugin in the default configuration. To disable this behavior, set close_when_requested (on the WindowPlugin) to false. Ensure that you read the caveats documented on that field if doing so.

**Examples:**

Example 1 (unknown):
```unknown
pub fn exit_on_all_closed(
    app_exit_writer: MessageWriter<'_, AppExit>,
    windows: Query<'_, '_, &Window>,
)
```

---

## Enum FileDragAndDrop Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.FileDragAndDrop.html

**Contents:**
- Enum FileDragAndDrop Copy item path
- Variants§
  - DroppedFile
    - Fields
  - HoveredFile
    - Fields
  - HoveredFileCanceled
    - Fields
- Trait Implementations§
  - impl Clone for FileDragAndDrop

Events related to files being dragged and dropped on a window.

File is being dropped into a window.

Window the file was dropped into.

Path to the file that was dropped in.

File is currently being hovered over a window.

Window a file is possibly going to be dropped into.

Path to the file that might be dropped in.

File hovering was canceled.

Window that had a canceled file drop.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum FileDragAndDrop {
    DroppedFile {
        window: Entity,
        path_buf: PathBuf,
    },
    HoveredFile {
        window: Entity,
        path_buf: PathBuf,
    },
    HoveredFileCanceled {
        window: Entity,
    },
}
```

---

## Struct WindowClosing Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.WindowClosing.html

**Contents:**
- Struct WindowClosing Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for WindowClosing
    - fn clone(&self) -> WindowClosing
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for WindowClosing
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for WindowClosing
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<WindowClosing, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,

An event that is sent whenever a window is closing. This will be sent when after a WindowCloseRequested event is received and the window is in the process of closing.

Window that has been requested to close and is the process of closing.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WindowClosing {
    pub window: Entity,
}
```

---

## Enum WindowLevel Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.WindowLevel.html

**Contents:**
- Enum WindowLevel Copy item path
  - §Platform-specific
- Variants§
  - AlwaysOnBottom
  - Normal
  - AlwaysOnTop
- Trait Implementations§
  - impl Clone for WindowLevel
    - fn clone(&self) -> WindowLevel
    - fn clone_from(&mut self, source: &Self)

Specifies where a Window should appear relative to other overlapping windows (on top or under) .

Levels are groups of windows with respect to their z-position.

The relative ordering between windows in different window levels is fixed. The z-order of windows within the same window level may change dynamically on user interaction.

The window will always be below WindowLevel::Normal and WindowLevel::AlwaysOnTop windows.

This is useful for a widget-based app.

The window will always be on top of WindowLevel::Normal and WindowLevel::AlwaysOnBottom windows.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum WindowLevel {
    AlwaysOnBottom,
    Normal,
    AlwaysOnTop,
}
```

---

## Enum ExitCondition Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.ExitCondition.html

**Contents:**
- Enum ExitCondition Copy item path
- Variants§
  - OnPrimaryClosed
  - OnAllClosed
  - DontExit
- Trait Implementations§
  - impl Clone for ExitCondition
    - fn clone(&self) -> ExitCondition
    - fn clone_from(&mut self, source: &Self)
- Auto Trait Implementations§

Defines the specific conditions the application should exit on

Close application when the primary window is closed

The plugin will add exit_on_primary_closed to PostUpdate.

Close application when all windows are closed

The plugin will add exit_on_all_closed to PostUpdate.

Keep application running headless even after closing all windows

If selecting this, ensure that you send the bevy_app::AppExit event when the app should exit. If this does not occur, you will create ‘headless’ processes (processes without windows), which may surprise your users.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ExitCondition {
    OnPrimaryClosed,
    OnAllClosed,
    DontExit,
}
```

---

## Struct WindowOccluded Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.WindowOccluded.html

**Contents:**
- Struct WindowOccluded Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for WindowOccluded
    - fn clone(&self) -> WindowOccluded
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for WindowOccluded
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for WindowOccluded
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<WindowOccluded, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,

The window has been occluded (completely hidden from view).

This is different to window visibility as it depends on whether the window is closed, minimized, set invisible, or fully occluded by another window.

It is the translated version of WindowEvent::Occluded from the winit crate.

Window that changed occluded state.

Whether it was occluded (true) or not occluded (false).

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WindowOccluded {
    pub window: Entity,
    pub occluded: bool,
}
```

---

## Enum CompositeAlphaMode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.CompositeAlphaMode.html

**Contents:**
- Enum CompositeAlphaMode Copy item path
- Variants§
  - Auto = 0
  - Opaque = 1
  - PreMultiplied = 2
  - PostMultiplied = 3
  - Inherit = 4
- Trait Implementations§
  - impl Clone for CompositeAlphaMode
    - fn clone(&self) -> CompositeAlphaMode

Specifies how the alpha channel of the textures should be handled during compositing, for a Window.

Chooses either Opaque or Inherit automatically, depending on the alpha_mode that the current surface can support.

The alpha channel, if it exists, of the textures is ignored in the compositing process. Instead, the textures is treated as if it has a constant alpha of 1.0.

The alpha channel, if it exists, of the textures is respected in the compositing process. The non-alpha channels of the textures are expected to already be multiplied by the alpha channel by the application.

The alpha channel, if it exists, of the textures is respected in the compositing process. The non-alpha channels of the textures are not expected to already be multiplied by the alpha channel by the application; instead, the compositor will multiply the non-alpha channels of the texture by the alpha channel during compositing.

The alpha channel, if it exists, of the textures is unknown for processing during compositing. Instead, the application is responsible for setting the composite alpha blending mode using native WSI command. If not set, then a platform-specific default will be used.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub enum CompositeAlphaMode {
    Auto = 0,
    Opaque = 1,
    PreMultiplied = 2,
    PostMultiplied = 3,
    Inherit = 4,
}
```

---

## Struct WindowThemeChanged Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.WindowThemeChanged.html

**Contents:**
- Struct WindowThemeChanged Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for WindowThemeChanged
    - fn clone(&self) -> WindowThemeChanged
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for WindowThemeChanged
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for WindowThemeChanged
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<WindowThemeChanged, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,

An event sent when the system theme changes for a window.

This event is only sent when the window is relying on the system theme to control its appearance. i.e. It is only sent when Window::window_theme is None and the system theme changes.

Window for which the system theme has changed.

The new system theme.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WindowThemeChanged {
    pub window: Entity,
    pub theme: WindowTheme,
}
```

---

## Function close_when_requested Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/fn.close_when_requested.html

**Contents:**
- Function close_when_requested Copy item path

Close windows in response to WindowCloseRequested (e.g. when the close button is pressed).

This system is added by the WindowPlugin in the default configuration. To disable this behavior, set close_when_requested (on the WindowPlugin) to false. Ensure that you read the caveats documented on that field if doing so.

**Examples:**

Example 1 (unknown):
```unknown
pub fn close_when_requested(
    commands: Commands<'_, '_>,
    closed: MessageReader<'_, '_, WindowCloseRequested>,
    closing: Query<'_, '_, Entity, With<ClosingWindow>>,
)
```

---

## Enum CustomCursor Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.CustomCursor.html

**Contents:**
- Enum CustomCursor Copy item path
- Variants§
  - Image(CustomCursorImage)
  - Url(CustomCursorUrl)
- Trait Implementations§
  - impl Clone for CustomCursor
    - fn clone(&self) -> CustomCursor
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for CustomCursor
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

Custom cursor image data.

Use an image as the cursor.

Use a URL to an image as the cursor. Note that this currently only works on the web.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum CustomCursor {
    Image(CustomCursorImage),
    Url(CustomCursorUrl),
}
```

---

## Struct WindowScaleFactorChanged Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.WindowScaleFactorChanged.html

**Contents:**
- Struct WindowScaleFactorChanged Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for WindowScaleFactorChanged
    - fn clone(&self) -> WindowScaleFactorChanged
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for WindowScaleFactorChanged
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for WindowScaleFactorChanged
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<WindowScaleFactorChanged, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,

An event that indicates a window’s scale factor has changed.

Window that had its scale factor changed.

The new scale factor.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WindowScaleFactorChanged {
    pub window: Entity,
    pub scale_factor: f64,
}
```

---

## Struct CursorLeft Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.CursorLeft.html

**Contents:**
- Struct CursorLeft Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for CursorLeft
    - fn clone(&self) -> CursorLeft
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for CursorLeft
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for CursorLeft
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<CursorLeft, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,

An event that is sent whenever the user’s cursor leaves a window.

Window that the cursor left.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CursorLeft {
    pub window: Entity,
}
```

---

## Struct ScreenSpaceAmbientOcclusion Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ScreenSpaceAmbientOcclusion.html

**Contents:**
- Struct ScreenSpaceAmbientOcclusion Copy item path
- §Usage Notes
- Fields§
- Trait Implementations§
  - impl Clone for ScreenSpaceAmbientOcclusion
    - fn clone(&self) -> ScreenSpaceAmbientOcclusion
    - fn clone_from(&mut self, source: &Self)
  - impl Component for ScreenSpaceAmbientOcclusionwhere ScreenSpaceAmbientOcclusion: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable

Component to apply screen space ambient occlusion to a 3d camera.

Screen space ambient occlusion (SSAO) approximates small-scale, local occlusion of indirect diffuse light between objects, based on what’s visible on-screen. SSAO does not apply to direct lighting, such as point or directional lights.

This darkens creases, e.g. on staircases, and gives nice contact shadows where objects meet, giving entities a more “grounded” feel.

Requires that you add ScreenSpaceAmbientOcclusionPlugin to your app.

It strongly recommended that you use SSAO in conjunction with TAA (TemporalAntiAliasing). Doing so greatly reduces SSAO noise.

SSAO is not supported on WebGL2, and is not currently supported on WebGPU.

Quality of the SSAO effect.

A constant estimated thickness of objects.

This value is used to decide how far behind an object a ray of light needs to be in order to pass behind it. Any ray closer than that will be occluded.

Required Components: DepthPrepass, NormalPrepass.

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components of the required components, recursively, in depth-first order.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ScreenSpaceAmbientOcclusion {
    pub quality_level: ScreenSpaceAmbientOcclusionQualityLevel,
    pub constant_object_thickness: f32,
}
```

---

## Struct WindowMoved Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.WindowMoved.html

**Contents:**
- Struct WindowMoved Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for WindowMoved
    - fn clone(&self) -> WindowMoved
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for WindowMoved
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for WindowMoved
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<WindowMoved, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,

An event that is sent when a window is repositioned in physical pixels.

Where the window moved to in physical pixels.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WindowMoved {
    pub window: Entity,
    pub position: IVec2,
}
```

---

## Enum WindowMode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.WindowMode.html

**Contents:**
- Enum WindowMode Copy item path
- Variants§
  - Windowed
  - BorderlessFullscreen(MonitorSelection)
  - Fullscreen(MonitorSelection, VideoModeSelection)
- Trait Implementations§
  - impl Clone for WindowMode
    - fn clone(&self) -> WindowMode
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for WindowMode

Defines the way a Window is displayed.

The window should take a portion of the screen, using the window resolution size.

The window should appear fullscreen by being borderless and using the full size of the screen on the given MonitorSelection.

When setting this, the window’s physical size will be modified to match the size of the current monitor resolution, and the logical size will follow based on the scale factor, see WindowResolution.

Note: As this mode respects the scale factor provided by the operating system, the window’s logical size may be different from its physical size. If you want to avoid that behavior, you can use the WindowResolution::set_scale_factor_override function or the WindowResolution::with_scale_factor_override builder method to set the scale factor to 1.0.

The window should be in “true”/“legacy”/“exclusive” Fullscreen mode on the given MonitorSelection.

The resolution, refresh rate, and bit depth are selected based on the given VideoModeSelection.

Note: As this mode respects the scale factor provided by the operating system, the window’s logical size may be different from its physical size. If you want to avoid that behavior, you can use the WindowResolution::set_scale_factor_override function or the WindowResolution::with_scale_factor_override builder method to set the scale factor to 1.0.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum WindowMode {
    Windowed,
    BorderlessFullscreen(MonitorSelection),
    Fullscreen(MonitorSelection, VideoModeSelection),
}
```

---

## Enum ScreenEdge Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.ScreenEdge.html

**Contents:**
- Enum ScreenEdge Copy item path
- §Platform-specific
- Variants§
  - None
  - Top
  - Left
  - Bottom
  - Right
  - All
- Trait Implementations§

The edges of a screen. Corresponds to winit::platform::ios::ScreenEdge.

The top edge of the screen.

The left edge of the screen.

The bottom edge of the screen.

The right edge of the screen.

All edges of the screen.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ScreenEdge {
    None,
    Top,
    Left,
    Bottom,
    Right,
    All,
}
```

---

## Struct WindowResized Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.WindowResized.html

**Contents:**
- Struct WindowResized Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for WindowResized
    - fn clone(&self) -> WindowResized
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for WindowResized
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for WindowResized
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<WindowResized, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,

A window event that is sent whenever a window’s logical size has changed.

Window that has changed.

The new logical width of the window.

The new logical height of the window.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WindowResized {
    pub window: Entity,
    pub width: f32,
    pub height: f32,
}
```

---

## Enum WindowPosition Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.WindowPosition.html

**Contents:**
- Enum WindowPosition Copy item path
- Variants§
  - Automatic
  - Centered(MonitorSelection)
  - At(IVec2)
- Implementations§
  - impl WindowPosition
    - pub fn new(position: IVec2) -> WindowPosition
    - pub fn set(&mut self, position: IVec2)
    - pub fn center(&mut self, monitor: MonitorSelection)

Defines where a Window should be placed on the screen.

Position will be set by the window manager. Bevy will delegate this decision to the window manager and no guarantees can be made about where the window will be placed.

Used at creation but will be changed to At.

Window will be centered on the selected monitor.

Note that this does not account for window decorations.

Used at creation or for update but will be changed to At

The window’s top-left corner should be placed at the specified position (in physical pixels).

(0,0) represents top-left corner of screen space.

Creates a new WindowPosition at a position.

Set the position to a specific point.

Set the window to a specific monitor.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum WindowPosition {
    Automatic,
    Centered(MonitorSelection),
    At(IVec2),
}
```

---

## Struct NormalizedWindowRef Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.NormalizedWindowRef.html

**Contents:**
- Struct NormalizedWindowRef Copy item path
- Trait Implementations§
  - impl Clone for NormalizedWindowRef
    - fn clone(&self) -> NormalizedWindowRef
    - fn clone_from(&mut self, source: &Self)
  - impl ContainsEntity for NormalizedWindowRef
    - fn entity(&self) -> Entity
  - impl Debug for NormalizedWindowRef
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for NormalizedWindowRef

A flattened representation of a window reference for equality/hashing purposes.

For most purposes you probably want to use the unnormalized version WindowRef.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct NormalizedWindowRef(/* private fields */);
```

---

## Enum ScreenSpaceAmbientOcclusionQualityLevel Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/enum.ScreenSpaceAmbientOcclusionQualityLevel.html

**Contents:**
- Enum ScreenSpaceAmbientOcclusionQualityLevel Copy item path
- Variants§
  - Low
  - Medium
  - High
  - Ultra
  - Custom
    - Fields
- Trait Implementations§
  - impl Clone for ScreenSpaceAmbientOcclusionQualityLevel

Higher slice count means less noise, but worse performance.

Samples per slice side is also tweakable, but recommended to be left at 2 or 3.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ScreenSpaceAmbientOcclusionQualityLevel {
    Low,
    Medium,
    High,
    Ultra,
    Custom {
        slice_count: u32,
        samples_per_slice_side: u32,
    },
}
```

---

## Enum CursorGrabMode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.CursorGrabMode.html

**Contents:**
- Enum CursorGrabMode Copy item path
  - §Platform-specific
- Variants§
  - None
  - Confined
  - Locked
- Trait Implementations§
  - impl Clone for CursorGrabMode
    - fn clone(&self) -> CursorGrabMode
    - fn clone_from(&mut self, source: &Self)

Defines if and how the cursor is grabbed by a Window.

Since macOS and X11 don’t have full CursorGrabMode support, we first try to set the grab mode that was asked for. If it doesn’t work then use the alternate grab mode.

The cursor can freely leave the window.

The cursor is confined to the window area.

The cursor is locked inside the window area to a certain position.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum CursorGrabMode {
    None,
    Confined,
    Locked,
}
```

---

## Enum PresentMode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/enum.PresentMode.html

**Contents:**
- Enum PresentMode Copy item path
- Variants§
  - AutoVsync = 0
  - AutoNoVsync = 1
  - Fifo = 2
  - FifoRelaxed = 3
  - Immediate = 4
  - Mailbox = 5
- Trait Implementations§
  - impl Clone for PresentMode

Presentation mode for a Window.

The presentation mode specifies when a frame is presented to the window. The Fifo option corresponds to a traditional VSync, where the framerate is capped by the display refresh rate. Both Immediate and Mailbox are low-latency and are not capped by the refresh rate, but may not be available on all platforms. Tearing may be observed with Immediate mode, but will not be observed with Mailbox or Fifo.

AutoVsync or AutoNoVsync will gracefully fallback to Fifo when unavailable.

Immediate or Mailbox will panic if not supported by the platform.

Chooses FifoRelaxed -> Fifo based on availability.

Because of the fallback behavior, it is supported everywhere.

Chooses Immediate -> Mailbox -> Fifo (on web) based on availability.

Because of the fallback behavior, it is supported everywhere.

Presentation frames are kept in a First-In-First-Out queue approximately 3 frames long. Every vertical blanking period, the presentation engine will pop a frame off the queue to display. If there is no frame to display, it will present the same frame again until the next vblank.

When a present command is executed on the gpu, the presented image is added on the queue.

No tearing will be observed.

Calls to get_current_texture will block until there is a spot in the queue.

Supported on all platforms.

If you don’t know what mode to choose, choose this mode. This is traditionally called “Vsync On”.

Presentation frames are kept in a First-In-First-Out queue approximately 3 frames long. Every vertical blanking period, the presentation engine will pop a frame off the queue to display. If there is no frame to display, it will present the same frame until there is a frame in the queue. The moment there is a frame in the queue, it will immediately pop the frame off the queue.

When a present command is executed on the gpu, the presented image is added on the queue.

Tearing will be observed if frames last more than one vblank as the front buffer.

Calls to get_current_texture will block until there is a spot in the queue.

Supported on AMD on Vulkan.

This is traditionally called “Adaptive Vsync”

Presentation frames are not queued at all. The moment a present command is executed on the GPU, the presented image is swapped onto the front buffer immediately.

Tearing can be observed.

Supported on most platforms except older DX12 and Wayland.

This is traditionally called “Vsync Off”.

Presentation frames are kept in a single-frame queue. Every vertic

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub enum PresentMode {
    AutoVsync = 0,
    AutoNoVsync = 1,
    Fifo = 2,
    FifoRelaxed = 3,
    Immediate = 4,
    Mailbox = 5,
}
```

---

## Struct ScreenSpaceAmbientOcclusionResources Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ScreenSpaceAmbientOcclusionResources.html

**Contents:**
- Struct ScreenSpaceAmbientOcclusionResources Copy item path
- Fields§
- Trait Implementations§
  - impl Component for ScreenSpaceAmbientOcclusionResourceswhere ScreenSpaceAmbientOcclusionResources: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior
    - fn on_add() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_insert() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ScreenSpaceAmbientOcclusionResources {
    pub screen_space_ambient_occlusion_texture: CachedTexture,
    /* private fields */
}
```

---

## Struct WindowBackendScaleFactorChanged Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.WindowBackendScaleFactorChanged.html

**Contents:**
- Struct WindowBackendScaleFactorChanged Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for WindowBackendScaleFactorChanged
    - fn clone(&self) -> WindowBackendScaleFactorChanged
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for WindowBackendScaleFactorChanged
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for WindowBackendScaleFactorChanged
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<WindowBackendScaleFactorChanged, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,

An event that indicates a window’s OS-reported scale factor has changed.

Window that had its scale factor changed by the backend.

The new scale factor.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WindowBackendScaleFactorChanged {
    pub window: Entity,
    pub scale_factor: f64,
}
```

---

## Struct CustomCursorUrl Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.CustomCursorUrl.html

**Contents:**
- Struct CustomCursorUrl Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for CustomCursorUrl
    - fn clone(&self) -> CustomCursorUrl
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for CustomCursorUrl
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for CustomCursorUrl
    - fn default() -> CustomCursorUrl

A custom cursor created from a URL. Note that this currently only works on the web.

Web URL to an image to use as the cursor. PNGs are preferred. Cursor creation can fail if the image is invalid or not reachable.

X and Y coordinates of the hotspot in pixels. The hotspot must be within the image bounds.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CustomCursorUrl {
    pub url: String,
    pub hotspot: (u16, u16),
}
```

---

## Struct ScreenSpaceReflectionsBuffer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ScreenSpaceReflectionsBuffer.html

**Contents:**
- Struct ScreenSpaceReflectionsBuffer Copy item path
- Tuple Fields§
- Methods from Deref<Target = DynamicUniformBuffer<ScreenSpaceReflectionsUniform>>§
    - pub fn buffer(&self) -> Option<&Buffer>
    - pub fn binding(&self) -> Option<BindingResource<'_>>
      - Examples found in repository?
    - pub fn is_empty(&self) -> bool
    - pub fn push(&mut self, value: &T) -> u32
    - pub fn set_label(&mut self, label: Option<&str>)
    - pub fn get_label(&self) -> Option<&str>

A GPU buffer that stores the screen space reflection settings for each view.

Push data into the DynamicUniformBuffer’s internal vector (residing on system RAM).

Add more BufferUsages to the buffer.

This method only allows addition of flags to the default usage flags.

The default values for buffer usage are BufferUsages::COPY_DST and BufferUsages::UNIFORM.

Creates a writer that can be used to directly write elements into the target buffer.

This method uses less memory and performs fewer memory copies using over push and write_buffer.

max_count must be greater than or equal to the number of elements that are to be written to the buffer, or the writer will panic while writing. Dropping the writer will schedule the buffer write into the provided RenderQueue.

If there is no GPU-side buffer allocated to hold the data currently stored, or if a GPU-side buffer previously allocated does not have enough capacity to hold max_count elements, a new GPU-side buffer is created.

Returns None if there is no allocated GPU-side buffer, and max_count is 0.

Queues writing of data from system RAM to VRAM using the RenderDevice and the provided RenderQueue.

If there is no GPU-side buffer allocated to hold the data currently stored, or if a GPU-side buffer previously allocated does not have enough capacity, a new GPU-side buffer is created.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ScreenSpaceReflectionsBuffer(pub DynamicUniformBuffer<ScreenSpaceReflectionsUniform>);
```

Example 2 (javascript):
```javascript
135    fn run(
136        &self,
137        _graph: &mut RenderGraphContext,
138        render_context: &mut RenderContext,
139        (view_target, _post_process_settings, settings_index): QueryItem<Self::ViewQuery>,
140        world: &World,
141    ) -> Result<(), NodeRunError> {
142        // Get the pipeline resource that contains the global data we need
143        // to create the render pipeline
144        let post_process_pipeline = world.resource::<PostProcessPipeline>();
145
146        // The pipeline cache is a cache of all previously created pipelines.
147        // It is required t
...
```

---
