# Crates-Rs-Bevy - Text

**Pages:** 13

---

## Function start_gizmo_context Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/fn.start_gizmo_context.html

**Contents:**
- Function start_gizmo_context Copy item path

Start a new gizmo clearing context.

Internally this pushes the parent default context into a swap buffer. Gizmo contexts should be handled like a stack, so if you push a new context, you must pop the context before the parent context ends.

**Examples:**

Example 1 (unknown):
```unknown
pub fn start_gizmo_context<Config, Clear>(
    swap: ResMut<'_, GizmoStorage<Config, Swap<Clear>>>,
    default: ResMut<'_, GizmoStorage<Config, ()>>,
)where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
```

---

## Struct ClusteredDecal Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.ClusteredDecal.html

**Contents:**
- Struct ClusteredDecal Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for ClusteredDecal
    - fn clone(&self) -> ClusteredDecal
    - fn clone_from(&mut self, source: &Self)
  - impl Component for ClusteredDecalwhere ClusteredDecal: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

An object that projects a decal onto surfaces within its bounds.

Conceptually, a clustered decal is a 1×1×1 cube centered on its origin. It projects the given Self::image onto surfaces in the -Z direction (thus you may find Transform::looking_at useful).

Clustered decals are the highest-quality types of decals that Bevy supports, but they require bindless textures. This means that they presently can’t be used on WebGL 2, WebGPU, macOS, or iOS. Bevy’s clustered decals can be used with forward or deferred rendering and don’t require a prepass.

The image that the clustered decal projects.

This must be a 2D image. If it has an alpha channel, it’ll be alpha blended with the underlying surface and/or other decals. All decal images in the scene must use the same sampler.

An application-specific tag you can use for any purpose you want.

See the clustered_decals example for an example of use.

Required Components: Transform, Visibility, VisibilityClass.

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components of the required components, recursively, in depth-first order.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ClusteredDecal {
    pub image: Handle<Image>,
    pub tag: u32,
}
```

---

## Struct EnvFilter Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/tracing_subscriber/struct.EnvFilter.html

**Contents:**
- Struct EnvFilter Copy item path
- §Directives
  - §Usage Notes
  - §Example Syntax
- §Examples
- §Constructing EnvFilters
- Implementations§
  - impl EnvFilter
    - pub const DEFAULT_ENV: &'static str = "RUST_LOG"
    - pub fn builder() -> Builder

A Layer which filters spans and events based on a set of filter directives.

EnvFilter implements both the Layer and Filter traits, so it may be used for both global filtering and per-layer filtering, respectively. See the documentation on filtering with Layers for details.

The Targets type implements a similar form of filtering, but without the ability to dynamically enable events based on the current span context, and without filtering on field values. When these features are not required, Targets provides a lighter-weight alternative to EnvFilter.

A filter consists of one or more comma-separated directives which match on Spans and Events. Each directive may have a corresponding maximum verbosity level which enables (e.g., selects for) spans and events that match. Like log, tracing considers less exclusive levels (like trace or info) to be more verbose than more exclusive levels (like error or warn).

The directive syntax is similar to that of env_logger’s. At a high level, the syntax for directives consists of several parts:

Each component (target, span, field, value, and level) will be covered in turn.

When a field value directive ([{<FIELD NAME>=<FIELD_VALUE>}]=...) matches a value’s std::fmt::Debug output (i.e., the field value in the directive is not a bool, i64, u64, or f64 literal), the matched pattern may be interpreted as either a regular expression or as the precise expected output of the field’s std::fmt::Debug implementation. By default, these filters are interpreted as regular expressions, but this can be disabled using the Builder::with_regex builder method to use precise matching instead.

When field value filters are interpreted as regular expressions, the regex crate’s regular expression syntax is supported.

Note: When filters are constructed from potentially untrusted inputs, disabling regular expression matching is strongly recommended.

Parsing an EnvFilter from the default environment variable (RUST_LOG):

Parsing an EnvFilter from a user-provided environment variable:

Using EnvFilter as a per-layer filter to filter only a single Layer:

An EnvFilter is be constructed by parsing a string containing one or more directives. The EnvFilter::new constructor parses an EnvFilter from a string, ignoring any invalid directives, while EnvFilter::try_new returns an error if invalid directives are encountered. Similarly, the EnvFilter::from_env and EnvFilter::try_from_env constructors parse an EnvFilter from the value of the provided envir

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct EnvFilter { /* private fields */ }
```

Example 2 (text):
```text
target[span{field=value}]=level
```

Example 3 (unknown):
```unknown
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

tracing_subscriber::registry()
    .with(fmt::layer())
    .with(EnvFilter::from_default_env())
    .init();
```

Example 4 (unknown):
```unknown
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

tracing_subscriber::registry()
    .with(fmt::layer())
    .with(EnvFilter::from_env("MYAPP_LOG"))
    .init();
```

---

## Enum ImageSamplerBorderColor Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.ImageSamplerBorderColor.html

**Contents:**
- Enum ImageSamplerBorderColor Copy item path
- Variants§
  - TransparentBlack
  - OpaqueBlack
  - OpaqueWhite
  - Zero
- Trait Implementations§
  - impl Clone for ImageSamplerBorderColor
    - fn clone(&self) -> ImageSamplerBorderColor
    - fn clone_from(&mut self, source: &Self)

Color variation to use when the sampler addressing mode is ImageAddressMode::ClampToBorder.

This type mirrors SamplerBorderColor.

RGBA color [0, 0, 0, 0].

RGBA color [0, 0, 0, 1].

RGBA color [1, 1, 1, 1].

On the Metal wgpu backend, this is equivalent to Self::TransparentBlack for textures that have an alpha component, and equivalent to Self::OpaqueBlack for textures that do not have an alpha component. On other backends, this is equivalent to Self::TransparentBlack. Requires Features::ADDRESS_MODE_CLAMP_TO_ZERO. Not supported on the web.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ImageSamplerBorderColor {
    TransparentBlack,
    OpaqueBlack,
    OpaqueWhite,
    Zero,
}
```

---

## Crate ui_widgets Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ui_widgets/index.html

**Contents:**
- Crate ui_widgets Copy item path
  - §Warning: Experimental
  - §State Management
- Structs§
- Enums§
- Functions§

This crate provides a set of standard widgets for Bevy UI, such as buttons, checkboxes, and sliders. These widgets have no inherent styling, it’s the responsibility of the user to add styling appropriate for their game or application.

This crate is currently experimental and under active development. The API is likely to change substantially: be prepared to migrate your code.

We are actively seeking feedback on the design and implementation of this crate, so please file issues or create PRs if you have any comments or suggestions.

Most of the widgets use external state management: this means that the widgets do not automatically update their own internal state, but instead rely on the app to update the widget state (as well as any other related game state) in response to a change event emitted by the widget. The primary motivation for this is to avoid two-way data binding in scenarios where the user interface is showing a live view of dynamic data coming from deeper within the game engine.

---

## Struct IrradianceVolume Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.IrradianceVolume.html

**Contents:**
- Struct IrradianceVolume Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for IrradianceVolume
    - fn clone(&self) -> IrradianceVolume
    - fn clone_from(&mut self, source: &Self)
  - impl Component for IrradianceVolumewhere IrradianceVolume: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

The component that defines an irradiance volume.

See bevy_pbr::irradiance_volume for detailed information.

This component requires the LightProbe component, and is typically used with bevy_transform::components::Transform to place the volume appropriately.

The 3D texture that represents the ambient cubes, encoded in the format described in bevy_pbr::irradiance_volume.

Scale factor applied to the diffuse and specular light generated by this component.

After applying this multiplier, the resulting values should be in units of cd/m^2.

See also https://google.github.io/filament/Filament.html#lighting/imagebasedlights/iblunit.

Whether the light from this irradiance volume has an effect on meshes with lightmaps.

Set this to false if your lightmap baking tool bakes the light from this irradiance volume into the lightmaps in order to avoid counting the irradiance twice. Frequently, applications use irradiance volumes as a lower-quality alternative to lightmaps for capturing indirect illumination on dynamic objects, and such applications will want to set this value to false.

By default, this is set to true.

Required Components: LightProbe.

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components of the required components, recursively, in depth-first order.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct IrradianceVolume {
    pub voxels: Handle<Image>,
    pub intensity: f32,
    pub affects_lightmapped_meshes: bool,
}
```

---

## Module font_styles Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/feathers/font_styles/index.html

**Contents:**
- Module font_styles Copy item path
- Structs§

A framework for inheritable font styles.

---

## Function end_gizmo_context Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/fn.end_gizmo_context.html

**Contents:**
- Function end_gizmo_context Copy item path

End this gizmo clearing context.

Pop the default gizmos context out of the Swap<Clear> gizmo storage.

This must be called before GizmoMeshSystems in the Last schedule.

**Examples:**

Example 1 (unknown):
```unknown
pub fn end_gizmo_context<Config, Clear>(
    swap: ResMut<'_, GizmoStorage<Config, Swap<Clear>>>,
    default: ResMut<'_, GizmoStorage<Config, ()>>,
)where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
```

---

## Enum ImageAddressMode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.ImageAddressMode.html

**Contents:**
- Enum ImageAddressMode Copy item path
- Variants§
  - ClampToEdge
  - Repeat
  - MirrorRepeat
  - ClampToBorder
- Trait Implementations§
  - impl Clone for ImageAddressMode
    - fn clone(&self) -> ImageAddressMode
    - fn clone_from(&mut self, source: &Self)

How edges should be handled in texture addressing.

See ImageSamplerDescriptor for information how to configure this.

This type mirrors AddressMode.

Clamp the value to the edge of the texture.

-0.25 -> 0.0 1.25 -> 1.0

Repeat the texture in a tiling fashion.

-0.25 -> 0.75 1.25 -> 0.25

Repeat the texture, mirroring it every repeat.

-0.25 -> 0.25 1.25 -> 0.75

Clamp the value to the border of the texture Requires the wgpu feature Features::ADDRESS_MODE_CLAMP_TO_BORDER.

-0.25 -> border 1.25 -> border

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ImageAddressMode {
    ClampToEdge,
    Repeat,
    MirrorRepeat,
    ClampToBorder,
}
```

---

## Crate ui Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ui/index.html

**Contents:**
- Crate ui Copy item path
- §Basic usage
- Modules§
- Structs§
- Enums§
- Traits§
- Functions§
- Type Aliases§

This crate contains Bevy’s UI system, which can be used to create UI for both 2D and 3D games

Spawn UI elements with widget::Button, ImageNode, Text and Node This UI is laid out with the Flexbox and CSS Grid layout models (see https://cssreference.io/flexbox/)

---

## Struct BuildIndirectParametersBindGroups Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.BuildIndirectParametersBindGroups.html

**Contents:**
- Struct BuildIndirectParametersBindGroups Copy item path
- Tuple Fields§
- Implementations§
  - impl BuildIndirectParametersBindGroups
    - pub fn new() -> BuildIndirectParametersBindGroups
- Methods from Deref<Target = HashMap<TypeId, PhaseBuildIndirectParametersBindGroups, NoOpHash>>§
    - pub fn hasher(&self) -> &S
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn keys(&self) -> Keys<'_, K, V> ⓘ

The bind groups for the compute shaders that reset indirect draw counts and build indirect parameters.

There’s one set of bind group for each phase. Phases are keyed off their core::any::TypeId.

Creates a new, empty BuildIndirectParametersBindGroups table.

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

Shrinks the capacity of the map as much as possible. It will drop down as much as possible while maintaining the internal rules and possibly leaving some space in accordance with th

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct BuildIndirectParametersBindGroups(pub HashMap<TypeId, PhaseBuildIndirectParametersBindGroups, NoOpHash>);
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

## Function clear_gizmo_context Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/fn.clear_gizmo_context.html

**Contents:**
- Function clear_gizmo_context Copy item path

Clear out the contextual gizmos.

**Examples:**

Example 1 (unknown):
```unknown
pub fn clear_gizmo_context<Config, Clear>(
    context: ResMut<'_, GizmoStorage<Config, Clear>>,
)where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
```

---

## Struct PhaseBuildIndirectParametersBindGroups Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PhaseBuildIndirectParametersBindGroups.html

**Contents:**
- Struct PhaseBuildIndirectParametersBindGroups Copy item path
- Auto Trait Implementations§
  - impl Freeze for PhaseBuildIndirectParametersBindGroups
  - impl !RefUnwindSafe for PhaseBuildIndirectParametersBindGroups
  - impl Send for PhaseBuildIndirectParametersBindGroups
  - impl Sync for PhaseBuildIndirectParametersBindGroups
  - impl Unpin for PhaseBuildIndirectParametersBindGroups
  - impl !UnwindSafe for PhaseBuildIndirectParametersBindGroups
- Blanket Implementations§
  - impl<T> Any for Twhere T: 'static + ?Sized,

The per-phase set of bind groups for the compute shaders that reset indirect draw counts and build indirect parameters.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PhaseBuildIndirectParametersBindGroups { /* private fields */ }
```

---
