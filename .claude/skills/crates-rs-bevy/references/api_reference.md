# Crates-Rs-Bevy - Api Reference

**Pages:** 155

---

## Struct VolumetricLight Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.VolumetricLight.html

**Contents:**
- Struct VolumetricLight Copy item path
- Trait Implementations§
  - impl Clone for VolumetricLight
    - fn clone(&self) -> VolumetricLight
    - fn clone_from(&mut self, source: &Self)
  - impl Component for VolumetricLightwhere VolumetricLight: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior

Add this component to a DirectionalLight with a shadow map (shadows_enabled: true) to make volumetric fog interact with it.

This allows the light to generate light shafts/god rays.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct VolumetricLight;
```

---

## Enum TranscodeFormat Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.TranscodeFormat.html

**Contents:**
- Enum TranscodeFormat Copy item path
- Variants§
  - Etc1s
  - Uastc(DataFormat)
  - R8UnormSrgb
  - Rg8UnormSrgb
  - Rgb8
- Trait Implementations§
  - impl Clone for TranscodeFormat
    - fn clone(&self) -> TranscodeFormat

Texture data need to be transcoded from this format for use with wgpu.

Has to be transcoded from R8UnormSrgb to R8Unorm for use with wgpu.

Has to be transcoded from Rg8UnormSrgb to R8G8Unorm for use with wgpu.

Has to be transcoded from Rgb8 to Rgba8 for use with wgpu.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum TranscodeFormat {
    Etc1s,
    Uastc(DataFormat),
    R8UnormSrgb,
    Rg8UnormSrgb,
    Rgb8,
}
```

---

## Enum ImageFormatSetting Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.ImageFormatSetting.html

**Contents:**
- Enum ImageFormatSetting Copy item path
- Variants§
  - FromExtension
  - Format(ImageFormat)
  - Guess
- Trait Implementations§
  - impl Clone for ImageFormatSetting
    - fn clone(&self) -> ImageFormatSetting
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for ImageFormatSetting

How to determine an image’s format when loading.

Determine the image format from its file extension.

Declare the image format explicitly.

Guess the image format by looking for magic bytes at the beginning of its data.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ImageFormatSetting {
    FromExtension,
    Format(ImageFormat),
    Guess,
}
```

---

## Struct GpuDirectionalCascade Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.GpuDirectionalCascade.html

**Contents:**
- Struct GpuDirectionalCascade Copy item path
- Trait Implementations§
  - impl Clone for GpuDirectionalCascade
    - fn clone(&self) -> GpuDirectionalCascade
    - fn clone_from(&mut self, source: &Self)
  - impl CreateFrom for GpuDirectionalCascadewhere GpuDirectionalCascade: ShaderType<ExtraMetadata = StructMetadata<3>>, Mat4: for<'__> CreateFrom, f32: for<'__> CreateFrom + for<'__> CreateFrom,
    - fn create_from<B>(reader: &mut Reader<B>) -> GpuDirectionalCascadewhere B: BufferRef,
  - impl Debug for GpuDirectionalCascade
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for GpuDirectionalCascade

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GpuDirectionalCascade { /* private fields */ }
```

---

## Struct DynamicFunctionMut Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/reflect/func/struct.DynamicFunctionMut.html

**Contents:**
- Struct DynamicFunctionMut Copy item path
- §Example
- Implementations§
  - impl<'env> DynamicFunctionMut<'env>
    - pub fn new<F>( func: F, info: impl TryInto, ) -> DynamicFunctionMut<'env>where F: for<'a> FnMut(ArgList<'a>) -> Result<Return<'a>, FunctionError> + 'env, <impl TryInto<FunctionInfo> as TryInto<FunctionInfo>>::Error: Debug,
      - §Panics
    - pub fn with_name( self, name: impl Into<Cow<'static, str>>, ) -> DynamicFunctionMut<'env>
    - pub fn with_overload<'a, F, Marker>(self, function: F) -> DynamicFunctionMut<'a>where 'env: 'a, F: IntoFunctionMut<'a, Marker>,
      - §Panics
      - §Example

A dynamic representation of a function.

This type can be used to represent any callable that satisfies FnMut (or the reflection-based equivalent, ReflectFnMut). That is, any function or closure.

For functions that do not need to capture their environment mutably, it’s recommended to use DynamicFunction instead.

This type can be seen as a superset of DynamicFunction.

See the module-level documentation for more information.

You will generally not need to construct this manually. Instead, many functions and closures can be automatically converted using the IntoFunctionMut trait.

Most of the time, a DynamicFunctionMut can be created using the IntoFunctionMut trait:

Create a new DynamicFunctionMut.

The given function can be used to call out to any other callable, including functions, closures, or methods.

It’s important that the function signature matches the provided FunctionInfo. as this will be used to validate arguments when calling the function. This is also required in order for function overloading to work correctly.

This function may panic for any of the following reasons:

Set the name of the function.

For DynamicFunctionMuts created using IntoFunctionMut, the default name will always be the full path to the function as returned by core::any::type_name, unless the function is a closure, anonymous function, or function pointer, in which case the name will be None.

Add an overload to this function.

Overloads allow a single DynamicFunctionMut to represent multiple functions of different signatures.

This can be used to handle multiple monomorphizations of a generic function or to allow functions with a variable number of arguments.

Any functions with the same argument signature will be overwritten by the one from the new function, F. For example, if the existing function had the signature (i32, i32) -> i32, and the new function, F, also had the signature (i32, i32) -> i32, the one from F would replace the one from the existing function.

Overloaded functions retain the name of the original function.

Note that it may be impossible to overload closures that mutably borrow from their environment due to Rust’s borrowing rules. However, it’s still possible to overload functions that do not capture their environment mutably, or those that maintain mutually exclusive mutable references to their environment.

Panics if the function, F, contains a signature already found in this function.

For a non-panicking version, see try_with_overload.

Attempt

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct DynamicFunctionMut<'env> { /* private fields */ }
```

Example 2 (javascript):
```javascript
let mut list: Vec<i32> = vec![1, 2, 3];

// `replace` is a closure that captures a mutable reference to `list`
let mut replace = |index: usize, value: i32| -> i32 {
  let old_value = list[index];
  list[index] = value;
  old_value
};

// Since this closure mutably borrows data, we can't convert it into a regular `DynamicFunction`,
// as doing so would result in a compile-time error:
// let mut func: DynamicFunction = replace.into_function();

// Instead, we convert it into a `DynamicFunctionMut` using `IntoFunctionMut::into_function_mut`:
let mut func: DynamicFunctionMut = replace.into_functio
...
```

Example 3 (javascript):
```javascript
let mut total_i32 = 0;
let mut add_i32 = |a: i32| total_i32 += a;

let mut total_f32 = 0.0;
let mut add_f32 = |a: f32| total_f32 += a;

// Currently, the only generic type `func` supports is `i32`.
let mut func = add_i32.into_function_mut();

// However, we can add an overload to handle `f32` as well:
func = func.with_overload(add_f32);

// Test `i32`:
let args = bevy_reflect::func::ArgList::new().with_owned(123_i32);
func.call(args).unwrap();

// Test `f32`:
let args = bevy_reflect::func::ArgList::new().with_owned(1.23_f32);
func.call(args).unwrap();

drop(func);
assert_eq!(total_i32, 123);
a
...
```

Example 4 (javascript):
```javascript
let mut total = 0;
let add = |a: i32, b: i32| -> i32 {
  total = a + b;
  total
};

let mut func = add.into_function_mut().with_name("add");
let args = ArgList::new().with_owned(25_i32).with_owned(75_i32);
let result = func.call(args).unwrap().unwrap_owned();
assert_eq!(result.try_take::<i32>().unwrap(), 100);
```

---

## Macro error Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/macro.error.html

**Contents:**
- Macro error Copy item path
- §Examples

Constructs an event at the error level.

This functions similarly to the event! macro. See the top-level documentation for details on the syntax accepted by this macro.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! error {
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
use tracing::error;

let (err_info, port) = ("No connection", 22);

error!(port, error = %err_info);
error!(target: "app_events", "App Error: {}", err_info);
error!({ info = err_info }, "error on port: {}", port);
error!(name: "invalid_input", "Invalid input: {}", err_info);
```

---

## Struct SkinUniforms Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.SkinUniforms.html

**Contents:**
- Struct SkinUniforms Copy item path
- Fields§
- Implementations§
  - impl SkinUniforms
    - pub fn skin_index(&self, skin: MainEntity) -> Option<u32>
    - pub fn skin_byte_offset(&self, skin: MainEntity) -> Option<SkinByteOffset>
    - pub fn all_skins(&self) -> impl Iterator<Item = &MainEntity>
- Trait Implementations§
  - impl FromWorld for SkinUniforms
    - fn from_world(world: &mut World) -> SkinUniforms

The GPU buffers containing joint matrices for all skinned meshes.

This is double-buffered: we store the joint matrices of each mesh for the previous frame in addition to those of each mesh for the current frame. This is for motion vector calculation. Every frame, we swap buffers and overwrite the joint matrix buffer from two frames ago with the data for the current frame.

Notes on implementation: see comment on top of the extract_skins system.

The CPU-side buffer that stores the joint matrices for skinned meshes in the current frame.

The GPU-side buffer that stores the joint matrices for skinned meshes in the current frame.

The GPU-side buffer that stores the joint matrices for skinned meshes in the previous frame.

Returns the current offset in joints of the skin in the buffer.

Returns the current offset in bytes of the skin in the buffer.

Returns an iterator over all skins in the scene.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct SkinUniforms {
    pub current_staging_buffer: Vec<Mat4>,
    pub current_buffer: Buffer,
    pub prev_buffer: Buffer,
    /* private fields */
}
```

---

## Struct MorphUniforms Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MorphUniforms.html

**Contents:**
- Struct MorphUniforms Copy item path
- Fields§
- Trait Implementations§
  - impl Default for MorphUniforms
    - fn default() -> MorphUniforms
  - impl Resource for MorphUniformswhere MorphUniforms: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for MorphUniforms
  - impl !RefUnwindSafe for MorphUniforms
  - impl Send for MorphUniforms

The GPU buffers containing morph weights for all meshes with morph targets.

This is double-buffered: we store the weights of the previous frame in addition to those of the current frame. This is for motion vector calculation. Every frame, we swap buffers and reuse the morph target weight buffer from two frames ago for the current frame.

The morph weights for the current frame.

The morph weights for the previous frame.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MorphUniforms {
    pub current_buffer: RawBufferVec<f32>,
    pub prev_buffer: RawBufferVec<f32>,
}
```

---

## Struct SkipGpuPreprocess Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.SkipGpuPreprocess.html

**Contents:**
- Struct SkipGpuPreprocess Copy item path
- Trait Implementations§
  - impl Component for SkipGpuPreprocesswhere SkipGpuPreprocess: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior
    - fn on_add() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_insert() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_replace() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>

Stops the GpuPreprocessNode attempting to generate the buffer for this view useful to avoid duplicating effort if the bind group is shared between views

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct SkipGpuPreprocess;
```

---

## Struct ViewKeyPrepassCache Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ViewKeyPrepassCache.html

**Contents:**
- Struct ViewKeyPrepassCache Copy item path
- Methods from Deref<Target = HashMap<RetainedViewEntity, MeshPipelineKey>>§
    - pub fn hasher(&self) -> &S
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn keys(&self) -> Keys<'_, K, V> ⓘ
      - §Examples
      - Examples found in repository?
    - pub fn values(&self) -> Values<'_, K, V> ⓘ
      - §Examples

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
pub struct ViewKeyPrepassCache(/* private fields */);
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

## Struct ColorCurve Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/struct.ColorCurve.html

**Contents:**
- Struct ColorCurve Copy item path
- Implementations§
  - impl<T> ColorCurve<T>where T: Mix + Clone,
    - pub fn new( colors: impl IntoIterator<Item = T>, ) -> Result<ColorCurve<T>, EvenCoreError>
      - §Example
- Trait Implementations§
  - impl<T> Clone for ColorCurve<T>where T: Clone,
    - fn clone(&self) -> ColorCurve<T>
    - fn clone_from(&mut self, source: &Self)
  - impl<T> Curve<T> for ColorCurve<T>where T: Mix + Clone,

A curve whose samples are defined by a collection of colors.

Create a new ColorCurve from a collection of mixable types. The domain of this curve will always be [0.0, len - 1] where len is the amount of mixable objects in the collection.

This fails if there’s not at least two mixable things in the collection.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ColorCurve<T> { /* private fields */ }
```

Example 2 (javascript):
```javascript
let broken = ColorCurve::new([RED]);
assert!(broken.is_err());
let gradient = ColorCurve::new([RED, GREEN, BLUE]);
assert!(gradient.is_ok());
assert_eq!(gradient.unwrap().domain(), Interval::new(0.0, 2.0).unwrap());
```

---

## Enum Color Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/enum.Color.html

**Contents:**
- Enum Color Copy item path
- §Conversion
- §Operations
- Variants§
  - Srgba(Srgba)
  - LinearRgba(LinearRgba)
  - Hsla(Hsla)
  - Hsva(Hsva)
  - Hwba(Hwba)
  - Laba(Laba)

An enumerated type that can represent any of the color types in this crate.

This is useful when you need to store a color in a data structure that can’t be generic over the color type.

Conversion between the various color spaces is achieved using Rust’s native From trait. Because certain color spaces are defined by their transformation to and from another space, these From implementations reflect that set of definitions.

For example, the sRGB space is defined by its relationship with Linear RGB, and HWB by its with sRGB. As such, it is the responsibility of sRGB to provide From implementations for Linear RGB, and HWB for sRGB. To then provide conversion between Linear RGB and HWB directly, HWB is responsible for implementing these conversions, delegating to sRGB as an intermediatory. This ensures that all conversions take the shortest path between any two spaces, and limit the proliferation of domain specific knowledge for each color space to their respective definitions.

Color supports all the standard color operations, such as mixing, luminance and hue adjustment, and diffing. These operations delegate to the concrete color space contained by Color, but will convert to Oklch for operations which aren’t supported in the current space. After performing the operation, if a conversion was required, the result will be converted back into the original color space.

Oklch has been chosen as the intermediary space in cases where conversion is required due to its perceptual uniformity and broad support for Bevy’s color operations. To avoid the cost of repeated conversion, and ensure consistent results where that is desired, first convert this Color into your desired color space.

A color in the sRGB color space with alpha.

A color in the linear sRGB color space with alpha.

A color in the HSL color space with alpha.

A color in the HSV color space with alpha.

A color in the HWB color space with alpha.

A color in the LAB color space with alpha.

A color in the LCH color space with alpha.

A color in the Oklab color space with alpha.

A color in the Oklch color space with alpha.

A color in the XYZ color space with alpha.

A fully white Color::LinearRgba color with an alpha of 1.0.

A fully black Color::LinearRgba color with an alpha of 1.0.

A fully transparent Color::LinearRgba color with 0 red, green and blue.

Return the color as a linear RGBA color.

Return the color as an SRGBA color.

Creates a new Color object storing a Srgba color.

Creates a new Co

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub enum Color {
    Srgba(Srgba),
    LinearRgba(LinearRgba),
    Hsla(Hsla),
    Hsva(Hsva),
    Hwba(Hwba),
    Laba(Laba),
    Lcha(Lcha),
    Oklaba(Oklaba),
    Oklcha(Oklcha),
    Xyza(Xyza),
}
```

Example 2 (javascript):
```javascript
let color = Srgba::rgb(0.5, 0.5, 0.5);

// Using From explicitly
let linear_color = LinearRgba::from(color);

// Using Into
let linear_color: LinearRgba = color.into();
```

Example 3 (javascript):
```javascript
let red_hsv = Color::hsv(0., 1., 1.);
let red_srgb = Color::srgb(1., 0., 0.);

// HSV has a definition of hue, so it will be returned.
red_hsv.hue();

// SRGB doesn't have a native definition for hue.
// Converts to Oklch and returns that result.
red_srgb.hue();
```

Example 4 (javascript):
```javascript
90fn animate(
91    mut materials: ResMut<Assets<CustomUiMaterial>>,
92    q: Query<&MaterialNode<CustomUiMaterial>>,
93    time: Res<Time>,
94) {
95    let duration = 2.0;
96    for handle in &q {
97        if let Some(material) = materials.get_mut(handle) {
98            // rainbow color effect
99            let new_color = Color::hsl((time.elapsed_secs() * 60.0) % 360.0, 1., 0.5);
100            let border_color = Color::hsl((time.elapsed_secs() * 60.0) % 360.0, 0.75, 0.75);
101            material.color = new_color.to_linear().to_vec4();
102            material.slider.x =
103              
...
```

---

## Trait Luminance Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/trait.Luminance.html

**Contents:**
- Trait Luminance Copy item path
- Required Methods§
    - fn luminance(&self) -> f32
    - fn with_luminance(&self, value: f32) -> Self
    - fn darker(&self, amount: f32) -> Self
    - fn lighter(&self, amount: f32) -> Self
- Dyn Compatibility§
- Implementors§
  - impl Luminance for Color
  - impl Luminance for Hsla

Methods for changing the luminance of a color. Note that these methods are not guaranteed to produce consistent results across color spaces, but will be within a given space.

Return the luminance of this color (0.0 - 1.0).

Return a new version of this color with the given luminance. The resulting color will be clamped to the valid range for the color space; for some color spaces, clamping may cause the hue or chroma to change.

Return a darker version of this color. The amount should be between 0.0 and 1.0. The amount represents an absolute decrease in luminance, and is distributive: color.darker(a).darker(b) == color.darker(a + b). Colors are clamped to black if the amount would cause them to go below black.

For a relative decrease in luminance, you can simply mix() with black.

Return a lighter version of this color. The amount should be between 0.0 and 1.0. The amount represents an absolute increase in luminance, and is distributive: color.lighter(a).lighter(b) == color.lighter(a + b). Colors are clamped to white if the amount would cause them to go above white.

For a relative increase in luminance, you can simply mix() with white.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait Luminance: Sized {
    // Required methods
    fn luminance(&self) -> f32;
    fn with_luminance(&self, value: f32) -> Self;
    fn darker(&self, amount: f32) -> Self;
    fn lighter(&self, amount: f32) -> Self;
}
```

---

## Struct TypeRegistry Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/reflect/struct.TypeRegistry.html

**Contents:**
- Struct TypeRegistry Copy item path
- Implementations§
  - impl TypeRegistry
    - pub fn empty() -> TypeRegistry
    - pub fn new() -> TypeRegistry
    - pub fn register_derived_types(&mut self)
      - §Example
    - pub fn register<T>(&mut self)where T: GetTypeRegistration,
      - §Example
      - Examples found in repository?

A registry of reflected types.

This struct is used as the central store for type information. Registering a type will generate a new TypeRegistration entry in this store using a type’s GetTypeRegistration implementation (which is automatically implemented when using #[derive(Reflect)]).

See the crate-level documentation for more information.

Create a type registry with no registered types.

Create a type registry with default registrations for primitive types.

Register all non-generic types annotated with #[derive(Reflect)].

Calling this method is equivalent to calling register on all types without generic parameters that derived Reflect trait.

This method is supported on Linux, macOS, Windows, iOS, Android, and Web via the inventory crate. It does nothing on platforms not supported by either of those crates.

Attempts to register the type T if it has not yet been registered already.

This will also recursively register any type dependencies as specified by GetTypeRegistration::register_type_dependencies. When deriving Reflect, this will generally be all the fields of the struct or enum variant. As with any type registration, these type dependencies will not be registered more than once.

If the registration for type T already exists, it will not be registered again and neither will its type dependencies. To register the type, overwriting any existing registration, use register instead.

Additionally, this will add any reflect type data as specified in the Reflect derive.

Attempts to register the referenced type T if it has not yet been registered.

See register for more details.

Attempts to register the type described by registration.

If the registration for the type already exists, it will not be registered again.

To forcibly register the type, overwriting any existing registration, use the overwrite_registration method instead.

This method will not register type dependencies. Use register to register a type with its dependencies.

Returns true if the registration was added and false if it already exists.

Registers the type described by registration.

If the registration for the type already exists, it will be overwritten.

To avoid overwriting existing registrations, it’s recommended to use the register or add_registration methods instead.

This method will not register type dependencies. Use register to register a type with its dependencies.

Registers the type data D for type T.

Most of the time TypeRegistry::register can be used instead 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct TypeRegistry { /* private fields */ }
```

Example 2 (javascript):
```javascript
#[derive(Reflect, Default)]
#[reflect(Default)]
struct Foo {
  name: Option<String>,
  value: i32
}

let mut type_registry = TypeRegistry::empty();
type_registry.register_derived_types();

// The main type
assert!(type_registry.contains(TypeId::of::<Foo>()));

// Its type dependencies
assert!(type_registry.contains(TypeId::of::<Option<String>>()));
assert!(type_registry.contains(TypeId::of::<i32>()));

// Its type data
assert!(type_registry.get_type_data::<ReflectDefault>(TypeId::of::<Foo>()).is_some());
```

Example 3 (javascript):
```javascript
#[derive(Reflect, Default)]
#[reflect(Default)]
struct Foo {
  name: Option<String>,
  value: i32
}

let mut type_registry = TypeRegistry::default();

type_registry.register::<Foo>();

// The main type
assert!(type_registry.contains(TypeId::of::<Foo>()));

// Its type dependencies
assert!(type_registry.contains(TypeId::of::<Option<String>>()));
assert!(type_registry.contains(TypeId::of::<i32>()));

// Its type data
assert!(type_registry.get_type_data::<ReflectDefault>(TypeId::of::<Foo>()).is_some());
```

Example 4 (javascript):
```javascript
12fn main() {
13    #[derive(Reflect, Default, PartialEq, Debug)]
14    #[reflect(Identifiable, Default)]
15    struct Player {
16        id: u32,
17    }
18
19    #[reflect_trait]
20    trait Identifiable {
21        fn id(&self) -> u32;
22    }
23
24    impl Identifiable for Player {
25        fn id(&self) -> u32 {
26            self.id
27        }
28    }
29
30    // Normally, when instantiating a type, you get back exactly that type.
31    // This is because the type is known at compile time.
32    // We call this the "concrete" or "canonical" type.
33    let player: Player = Player { id: 
...
```

---

## Struct LightProbesBuffer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.LightProbesBuffer.html

**Contents:**
- Struct LightProbesBuffer Copy item path
- Methods from Deref<Target = DynamicUniformBuffer<LightProbesUniform>>§
    - pub fn buffer(&self) -> Option<&Buffer>
    - pub fn binding(&self) -> Option<BindingResource<'_>>
      - Examples found in repository?
    - pub fn is_empty(&self) -> bool
    - pub fn push(&mut self, value: &T) -> u32
    - pub fn set_label(&mut self, label: Option<&str>)
    - pub fn get_label(&self) -> Option<&str>
    - pub fn add_usages(&mut self, usage: BufferUsages)

A GPU buffer that stores information about all light probes.

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
pub struct LightProbesBuffer(/* private fields */);
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

## Enum FogFalloff Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/enum.FogFalloff.html

**Contents:**
- Enum FogFalloff Copy item path
  - §Convenience Methods
- Variants§
  - Linear
      - §Formula
    - Fields
  - Exponential
      - §Tips
      - §Formula
    - Fields

Allows switching between different fog falloff modes, and configuring their parameters.

When using non-linear fog modes it can be hard to determine the right parameter values for a given scene.

For easier artistic control, instead of creating the enum variants directly, you can use the visibility-based convenience methods:

For FogFalloff::Exponential:

For FogFalloff::ExponentialSquared:

For FogFalloff::Atmospheric:

A linear fog falloff that grows in intensity between start and end distances.

This falloff mode is simpler to control than other modes, however it can produce results that look “artificial”, depending on the scene.

The fog intensity for a given point in the scene is determined by the following formula:

Distance from the camera where fog is completely transparent, in world units.

Distance from the camera where fog is completely opaque, in world units.

An exponential fog falloff with a given density.

Initially gains intensity quickly with distance, then more slowly. Typically produces more natural results than FogFalloff::Linear, but is a bit harder to control.

To move the fog “further away”, use lower density values. To move it “closer” use higher density values.

The fog intensity for a given point in the scene is determined by the following formula:

Multiplier applied to the world distance (within the exponential fog falloff calculation).

A squared exponential fog falloff with a given density.

Similar to FogFalloff::Exponential, but grows more slowly in intensity for closer distances before “catching up”.

To move the fog “further away”, use lower density values. To move it “closer” use higher density values.

The fog intensity for a given point in the scene is determined by the following formula:

Multiplier applied to the world distance (within the exponential squared fog falloff calculation).

A more general form of the FogFalloff::Exponential mode. The falloff formula is separated into two terms, extinction and inscattering, for a somewhat simplified atmospheric scattering model. Additionally, individual color channels can have their own density values, resulting in a total of six different configuration parameters.

Unlike other modes, atmospheric falloff doesn’t use a simple intensity-based blend of fog color with object color. Instead, it calculates per-channel extinction and inscattering factors, which are then used to calculate the final color.

For a density value of D, the following two falloff modes will produce iden

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub enum FogFalloff {
    Linear {
        start: f32,
        end: f32,
    },
    Exponential {
        density: f32,
    },
    ExponentialSquared {
        density: f32,
    },
    Atmospheric {
        extinction: Vec3,
        inscattering: Vec3,
    },
}
```

Example 2 (text):
```text
let fog_intensity = 1.0 - ((end - distance) / (end - start)).clamp(0.0, 1.0);
```

Example 3 (text):
```text
let fog_intensity = 1.0 - 1.0 / (distance * density).exp();
```

Example 4 (text):
```text
let fog_intensity = 1.0 - 1.0 / (distance * density).squared().exp();
```

---

## Trait Gray Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/trait.Gray.html

**Contents:**
- Trait Gray Copy item path
- Required Associated Constants§
    - const BLACK: Self
    - const WHITE: Self
- Provided Methods§
    - fn gray(lightness: f32) -> Self
      - Examples found in repository?
- Dyn Compatibility§
- Implementors§
  - impl Gray for Hsla

Trait for returning a grayscale color of a provided lightness.

Returns a grey color with the provided lightness from (0.0 - 1.0). 0 is black, 1 is white.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (javascript):
```javascript
pub trait Gray: Sized + Mix {
    const BLACK: Self;
    const WHITE: Self;

    // Provided method
    fn gray(lightness: f32) -> Self { ... }
}
```

Example 2 (javascript):
```javascript
40fn draw_example_collection(
41    mut gizmos: Gizmos,
42    mut my_gizmos: Gizmos<MyRoundGizmos>,
43    time: Res<Time>,
44) {
45    let sin_t_scaled = ops::sin(time.elapsed_secs()) * 50.;
46    gizmos.line_2d(Vec2::Y * -sin_t_scaled, Vec2::splat(-80.), RED);
47    gizmos.ray_2d(Vec2::Y * sin_t_scaled, Vec2::splat(80.), LIME);
48
49    gizmos
50        .grid_2d(
51            Isometry2d::IDENTITY,
52            UVec2::new(16, 9),
53            Vec2::new(80., 80.),
54            // Dark gray
55            LinearRgba::gray(0.05),
56        )
57        .outer_edges();
58
59    // Triangle
60   
...
```

Example 3 (javascript):
```javascript
98fn draw_example_collection(
99    mut gizmos: Gizmos,
100    mut my_gizmos: Gizmos<MyRoundGizmos>,
101    time: Res<Time>,
102) {
103    gizmos.grid(
104        Quat::from_rotation_x(PI / 2.),
105        UVec2::splat(20),
106        Vec2::new(2., 2.),
107        // Light gray
108        LinearRgba::gray(0.65),
109    );
110    gizmos.grid(
111        Isometry3d::new(Vec3::splat(10.0), Quat::from_rotation_x(PI / 3. * 2.)),
112        UVec2::splat(20),
113        Vec2::new(2., 2.),
114        PURPLE,
115    );
116    gizmos.sphere(Vec3::splat(10.0), 1.0, PURPLE);
117
118    gizmos
119        .
...
```

---

## Trait AsyncRead Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/tasks/futures_lite/trait.AsyncRead.html

**Contents:**
- Trait AsyncRead Copy item path
- Required Methods§
    - fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8], ) -> Poll<Result<usize, Error>>
      - §Implementation
- Provided Methods§
    - fn poll_read_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &mut [IoSliceMut<'_>], ) -> Poll<Result<usize, Error>>
      - §Implementation
- Implementations on Foreign Types§
  - impl AsyncRead for &[u8]
    - fn poll_read( self: Pin<&mut &[u8]>, _: &mut Context<'_>, buf: &mut [u8], ) -> Poll<Result<usize, Error>>

Read bytes asynchronously.

This trait is analogous to the std::io::Read trait, but integrates with the asynchronous task system. In particular, the poll_read method, unlike Read::read, will automatically queue the current task for wakeup and return if data is not yet available, rather than blocking the calling thread.

Attempt to read from the AsyncRead into buf.

On success, returns Poll::Ready(Ok(num_bytes_read)).

If no data is available for reading, the method returns Poll::Pending and arranges for the current task (via cx.waker().wake_by_ref()) to receive a notification when the object becomes readable or is closed.

This function may not return errors of kind WouldBlock or Interrupted. Implementations must convert WouldBlock into Poll::Pending and either internally retry or convert Interrupted into another error kind.

Attempt to read from the AsyncRead into bufs using vectored IO operations.

This method is similar to poll_read, but allows data to be read into multiple buffers using a single operation.

On success, returns Poll::Ready(Ok(num_bytes_read)).

If no data is available for reading, the method returns Poll::Pending and arranges for the current task (via cx.waker().wake_by_ref()) to receive a notification when the object becomes readable or is closed. By default, this method delegates to using poll_read on the first nonempty buffer in bufs, or an empty one if none exists. Objects which support vectored IO should override this method.

This function may not return errors of kind WouldBlock or Interrupted. Implementations must convert WouldBlock into Poll::Pending and either internally retry or convert Interrupted into another error kind.

**Examples:**

Example 1 (unknown):
```unknown
pub trait AsyncRead {
    // Required method
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>>;

    // Provided method
    fn poll_read_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [IoSliceMut<'_>],
    ) -> Poll<Result<usize, Error>> { ... }
}
```

---

## Trait Saturation Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/trait.Saturation.html

**Contents:**
- Trait Saturation Copy item path
- Required Methods§
    - fn with_saturation(&self, saturation: f32) -> Self
    - fn saturation(&self) -> f32
    - fn set_saturation(&mut self, saturation: f32)
- Dyn Compatibility§
- Implementors§
  - impl Saturation for Color
  - impl Saturation for Hsla
  - impl Saturation for Hsva

Trait for manipulating the saturation of a color.

When working with color spaces that do not have native saturation components the operations are performed in Hsla.

Return a new version of this color with the saturation channel set to the given value.

Return the saturation of this color [0.0, 1.0].

Sets the saturation of this color.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait Saturation: Sized {
    // Required methods
    fn with_saturation(&self, saturation: f32) -> Self;
    fn saturation(&self) -> f32;
    fn set_saturation(&mut self, saturation: f32);
}
```

---

## Trait Volume Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/trait.Volume.html

**Contents:**
- Trait Volume Copy item path
- Required Methods§
    - fn volume(&self) -> usize
- Implementors§
  - impl Volume for Extent3d

Used to calculate the volume of an item.

**Examples:**

Example 1 (unknown):
```unknown
pub trait Volume {
    // Required method
    fn volume(&self) -> usize;
}
```

---

## Struct ViewFogUniformOffset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ViewFogUniformOffset.html

**Contents:**
- Struct ViewFogUniformOffset Copy item path
- Fields§
- Trait Implementations§
  - impl Component for ViewFogUniformOffsetwhere ViewFogUniformOffset: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior
    - fn on_add() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_insert() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>

Inserted on each Entity with an ExtractedView to keep track of its offset in the gpu_fogs DynamicUniformBuffer within FogMeta

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ViewFogUniformOffset {
    pub offset: u32,
}
```

---

## Struct Lcha Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/struct.Lcha.html

**Contents:**
- Struct Lcha Copy item path
- §Conversion
- Fields§
- Implementations§
  - impl Lcha
    - pub const fn new(lightness: f32, chroma: f32, hue: f32, alpha: f32) -> Lcha
      - §Arguments
    - pub const fn lch(lightness: f32, chroma: f32, hue: f32) -> Lcha
      - §Arguments
    - pub const fn with_chroma(self, chroma: f32) -> Lcha

Color in LCH color space, with alpha

Conversion between the various color spaces is achieved using Rust’s native From trait. Because certain color spaces are defined by their transformation to and from another space, these From implementations reflect that set of definitions.

For example, the sRGB space is defined by its relationship with Linear RGB, and HWB by its with sRGB. As such, it is the responsibility of sRGB to provide From implementations for Linear RGB, and HWB for sRGB. To then provide conversion between Linear RGB and HWB directly, HWB is responsible for implementing these conversions, delegating to sRGB as an intermediatory. This ensures that all conversions take the shortest path between any two spaces, and limit the proliferation of domain specific knowledge for each color space to their respective definitions.

The lightness channel. [0.0, 1.5]

The chroma channel. [0.0, 1.5]

The hue channel. [0.0, 360.0]

The alpha channel. [0.0, 1.0]

Construct a new Lcha color from components.

Construct a new Lcha color from (h, s, l) components, with the default alpha (1.0).

Return a copy of this color with the chroma channel set to the given value.

Return a copy of this color with the lightness channel set to the given value.

Generate a deterministic but quasi-randomly distributed color from a provided index.

This can be helpful for generating debug colors.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Lcha {
    pub lightness: f32,
    pub chroma: f32,
    pub hue: f32,
    pub alpha: f32,
}
```

Example 2 (javascript):
```javascript
let color = Srgba::rgb(0.5, 0.5, 0.5);

// Using From explicitly
let linear_color = LinearRgba::from(color);

// Using Into
let linear_color: LinearRgba = color.into();
```

Example 3 (javascript):
```javascript
// Unique color for an entity
// let entity_index = entity.index();
let color = Lcha::sequential_dispersed(entity_index);

// Palette with 5 distinct hues
let palette = (0..5).map(Lcha::sequential_dispersed).collect::<Vec<_>>();
```

---

## Struct EntitiesNeedingSpecialization Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.EntitiesNeedingSpecialization.html

**Contents:**
- Struct EntitiesNeedingSpecialization Copy item path
- Fields§
- Methods from Deref<Target = Vec<Entity>>§
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn reserve(&mut self, additional: usize)
      - §Panics
      - §Examples
    - pub fn reserve_exact(&mut self, additional: usize)
      - §Panics

Returns the total number of elements the vector can hold without reallocating.

A vector with zero-sized elements will always have a capacity of usize::MAX:

Reserves capacity for at least additional more elements to be inserted in the given Vec<T>. The collection may reserve more space to speculatively avoid frequent reallocations. After calling reserve, capacity will be greater than or equal to self.len() + additional. Does nothing if capacity is already sufficient.

Panics if the new capacity exceeds isize::MAX bytes.

Reserves the minimum capacity for at least additional more elements to be inserted in the given Vec<T>. Unlike reserve, this will not deliberately over-allocate to speculatively avoid frequent allocations. After calling reserve_exact, capacity will be greater than or equal to self.len() + additional. Does nothing if the capacity is already sufficient.

Note that the allocator may give the collection more space than it requests. Therefore, capacity can not be relied upon to be precisely minimal. Prefer reserve if future insertions are expected.

Panics if the new capacity exceeds isize::MAX bytes.

Tries to reserve capacity for at least additional more elements to be inserted in the given Vec<T>. The collection may reserve more space to speculatively avoid frequent reallocations. After calling try_reserve, capacity will be greater than or equal to self.len() + additional if it returns Ok(()). Does nothing if capacity is already sufficient. This method preserves the contents even if an error occurs.

If the capacity overflows, or the allocator reports a failure, then an error is returned.

Tries to reserve the minimum capacity for at least additional elements to be inserted in the given Vec<T>. Unlike try_reserve, this will not deliberately over-allocate to speculatively avoid frequent allocations. After calling try_reserve_exact, capacity will be greater than or equal to self.len() + additional if it returns Ok(()). Does nothing if the capacity is already sufficient.

Note that the allocator may give the collection more space than it requests. Therefore, capacity can not be relied upon to be precisely minimal. Prefer try_reserve if future insertions are expected.

If the capacity overflows, or the allocator reports a failure, then an error is returned.

Shrinks the capacity of the vector as much as possible.

The behavior of this method depends on the allocator, which may either shrink the vector in-place or reallocate. The resulting vecto

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct EntitiesNeedingSpecialization<M> {
    pub entities: Vec<Entity>,
    /* private fields */
}
```

Example 2 (javascript):
```javascript
let mut vec: Vec<i32> = Vec::with_capacity(10);
vec.push(42);
assert!(vec.capacity() >= 10);
```

Example 3 (javascript):
```javascript
#[derive(Clone)]
struct ZeroSized;

fn main() {
    assert_eq!(std::mem::size_of::<ZeroSized>(), 0);
    let v = vec![ZeroSized; 0];
    assert_eq!(v.capacity(), usize::MAX);
}
```

Example 4 (javascript):
```javascript
let mut vec = vec![1];
vec.reserve(10);
assert!(vec.capacity() >= 11);
```

---

## Struct DynamicFunction Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/reflect/func/struct.DynamicFunction.html

**Contents:**
- Struct DynamicFunction Copy item path
- §Example
- Implementations§
  - impl<'env> DynamicFunction<'env>
    - pub fn new<F>( func: F, info: impl TryInto, ) -> DynamicFunction<'env>where F: for<'a> Fn(ArgList<'a>) -> Result<Return<'a>, FunctionError> + Send + Sync + 'env, <impl TryInto<FunctionInfo> as TryInto<FunctionInfo>>::Error: Debug,
      - §Panics
      - Examples found in repository?
    - pub fn with_name( self, name: impl Into<Cow<'static, str>>, ) -> DynamicFunction<'env>
    - pub fn with_overload<'a, F, Marker>(self, function: F) -> DynamicFunction<'a>where 'env: 'a, F: IntoFunction<'a, Marker>,
      - §Panics

A dynamic representation of a function.

This type can be used to represent any callable that satisfies Fn (or the reflection-based equivalent, ReflectFn). That is, any function or closure that does not mutably borrow data from its environment.

For functions that do need to capture their environment mutably (i.e. mutable closures), see DynamicFunctionMut.

See the module-level documentation for more information.

You will generally not need to construct this manually. Instead, many functions and closures can be automatically converted using the IntoFunction trait.

Most of the time, a DynamicFunction can be created using the IntoFunction trait:

Create a new DynamicFunction.

The given function can be used to call out to any other callable, including functions, closures, or methods.

It’s important that the function signature matches the provided FunctionInfo. as this will be used to validate arguments when calling the function. This is also required in order for function overloading to work correctly.

This function may panic for any of the following reasons:

Set the name of the function.

For DynamicFunctions created using IntoFunction, the default name will always be the full path to the function as returned by core::any::type_name, unless the function is a closure, anonymous function, or function pointer, in which case the name will be None.

Add an overload to this function.

Overloads allow a single DynamicFunction to represent multiple functions of different signatures.

This can be used to handle multiple monomorphizations of a generic function or to allow functions with a variable number of arguments.

Any functions with the same argument signature will be overwritten by the one from the new function, F. For example, if the existing function had the signature (i32, i32) -> i32, and the new function, F, also had the signature (i32, i32) -> i32, the one from F would replace the one from the existing function.

Overloaded functions retain the name of the original function.

Panics if the function, F, contains a signature already found in this function.

For a non-panicking version, see try_with_overload.

Attempt to add an overload to this function.

If the function, F, contains a signature already found in this function, an error will be returned along with the original function.

For a panicking version, see with_overload.

Call the function with the given arguments.

This method will return an error if the number of arguments provided does not m

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct DynamicFunction<'env> { /* private fields */ }
```

Example 2 (javascript):
```javascript
fn add(a: i32, b: i32) -> i32 {
  a + b
}

// Convert the function into a dynamic function using `IntoFunction::into_function`:
let mut func: DynamicFunction = add.into_function();

// Dynamically call it:
let args = ArgList::default().with_owned(25_i32).with_owned(75_i32);
let value = func.call(args).unwrap().unwrap_owned();

// Check the result:
assert_eq!(value.try_downcast_ref::<i32>(), Some(&100));
```

Example 3 (javascript):
```javascript
19fn main() {
20    // There are times when it may be helpful to store a function away for later.
21    // In Rust, we can do this by storing either a function pointer or a function trait object.
22    // For example, say we wanted to store the following function:
23    fn add(left: i32, right: i32) -> i32 {
24        left + right
25    }
26
27    // We could store it as either of the following:
28    let fn_pointer: fn(i32, i32) -> i32 = add;
29    let fn_trait_object: Box<dyn Fn(i32, i32) -> i32> = Box::new(add);
30
31    // And we can call them like so:
32    let result = fn_pointer(2, 2);

...
```

Example 4 (javascript):
```javascript
fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// Currently, the only generic type `func` supports is `i32`:
let mut func = add::<i32>.into_function();

// However, we can add an overload to handle `f32` as well:
func = func.with_overload(add::<f32>);

// Test `i32`:
let args = ArgList::default().with_owned(25_i32).with_owned(75_i32);
let result = func.call(args).unwrap().unwrap_owned();
assert_eq!(result.try_take::<i32>().unwrap(), 100);

// Test `f32`:
let args = ArgList::default().with_owned(25.0_f32).with_owned(75.0_f32);
let result = func.call(args).unwrap().unwrap_owned();
as
...
```

---

## Struct LightmapSlotIndex Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.LightmapSlotIndex.html

**Contents:**
- Struct LightmapSlotIndex Copy item path
- Methods from Deref<Target = NonMaxU16>§
    - pub const ZERO: NonMaxU16
    - pub const ONE: NonMaxU16
    - pub const MAX: NonMaxU16
    - pub fn get(&self) -> u16
- Trait Implementations§
  - impl Clone for LightmapSlotIndex
    - fn clone(&self) -> LightmapSlotIndex
    - fn clone_from(&mut self, source: &Self)

The index of the slot (element within the binding array) in the slab in which a lightmap is located.

Returns the value as a primitive type.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct LightmapSlotIndex(/* private fields */);
```

---

## Function get_transcoded_formats Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/fn.get_transcoded_formats.html

**Contents:**
- Function get_transcoded_formats Copy item path

bevy::imageFunction get_transcoded_formats Copy item pathSource pub fn get_transcoded_formats( supported_compressed_formats: CompressedImageFormats, data_format: DataFormat, is_srgb: bool, ) -> (TranscoderBlockFormat, TextureFormat)

**Examples:**

Example 1 (unknown):
```unknown
pub fn get_transcoded_formats(
    supported_compressed_formats: CompressedImageFormats,
    data_format: DataFormat,
    is_srgb: bool,
) -> (TranscoderBlockFormat, TextureFormat)
```

---

## Struct ShadowsDrawFunction Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ShadowsDrawFunction.html

**Contents:**
- Struct ShadowsDrawFunction Copy item path
- Trait Implementations§
  - impl Clone for ShadowsDrawFunction
    - fn clone(&self) -> ShadowsDrawFunction
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for ShadowsDrawFunction
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for ShadowsDrawFunction
    - fn default() -> ShadowsDrawFunction
  - impl DrawFunctionLabel for ShadowsDrawFunctionwhere ShadowsDrawFunction: 'static + Send + Sync + Clone + Eq + Debug + Hash,

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ShadowsDrawFunction;
```

---

## Struct LinearRgba Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/struct.LinearRgba.html

**Contents:**
- Struct LinearRgba Copy item path
- §Conversion
- Fields§
- Implementations§
  - impl LinearRgba
    - pub const BLACK: LinearRgba
    - pub const WHITE: LinearRgba
    - pub const NONE: LinearRgba
    - pub const RED: LinearRgba
    - pub const GREEN: LinearRgba

Linear RGB color with alpha.

Conversion between the various color spaces is achieved using Rust’s native From trait. Because certain color spaces are defined by their transformation to and from another space, these From implementations reflect that set of definitions.

For example, the sRGB space is defined by its relationship with Linear RGB, and HWB by its with sRGB. As such, it is the responsibility of sRGB to provide From implementations for Linear RGB, and HWB for sRGB. To then provide conversion between Linear RGB and HWB directly, HWB is responsible for implementing these conversions, delegating to sRGB as an intermediatory. This ensures that all conversions take the shortest path between any two spaces, and limit the proliferation of domain specific knowledge for each color space to their respective definitions.

The red channel. [0.0, 1.0]

The green channel. [0.0, 1.0]

The blue channel. [0.0, 1.0]

The alpha channel. [0.0, 1.0]

A fully black color with full alpha.

A fully white color with full alpha.

A fully transparent color.

A fully red color with full alpha.

A fully green color with full alpha.

A fully blue color with full alpha.

This type can be used to represent an invalid color value; in some rendering applications the color will be ignored, enabling performant hacks like hiding lines by setting their color to INVALID.

Construct a new LinearRgba color from components.

Construct a new LinearRgba color from (r, g, b) components, with the default alpha (1.0).

Return a copy of this color with the red channel set to the given value.

Return a copy of this color with the green channel set to the given value.

Return a copy of this color with the blue channel set to the given value.

Converts this color to a u32.

Maps the RGBA channels in RGBA order to a little-endian byte array (GPUs are little-endian). A will be the most significant byte and R the least significant.

Construct a new LinearRgba color with the default values (white with full alpha).

Luminance calculated using the CIE XYZ formula.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct LinearRgba {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}
```

Example 2 (javascript):
```javascript
let color = Srgba::rgb(0.5, 0.5, 0.5);

// Using From explicitly
let linear_color = LinearRgba::from(color);

// Using Into
let linear_color: LinearRgba = color.into();
```

Example 3 (javascript):
```javascript
38fn setup(
39    mut commands: Commands,
40    mut meshes: ResMut<Assets<Mesh>>,
41    mut materials: ResMut<Assets<StandardMaterial>>,
42) {
43    // ground plane
44    commands.spawn((
45        Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0))),
46        MeshMaterial3d(materials.add(Color::WHITE)),
47        Movable,
48    ));
49
50    // cubes
51
52    // We're seeding the PRNG here to make this example deterministic for testing purposes.
53    // This isn't strictly required in practical use unless you need your app to be deterministic.
54    let mut rng = ChaCha8Rng::seed
...
```

Example 4 (javascript):
```javascript
42fn setup(
43    parameters: Res<Parameters>,
44    mut commands: Commands,
45    mut meshes: ResMut<Assets<Mesh>>,
46    mut materials: ResMut<Assets<StandardMaterial>>,
47    asset_server: Res<AssetServer>,
48) {
49    // ground plane
50    commands.spawn((
51        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
52        MeshMaterial3d(materials.add(StandardMaterial {
53            base_color: Color::WHITE,
54            perceptual_roughness: 1.0,
55            ..default()
56        })),
57    ));
58
59    // left wall
60    let mut transform = Transform::from_xyz(2.5, 2.
...
```

---

## Trait Hue Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/trait.Hue.html

**Contents:**
- Trait Hue Copy item path
- Required Methods§
    - fn with_hue(&self, hue: f32) -> Self
    - fn hue(&self) -> f32
    - fn set_hue(&mut self, hue: f32)
- Provided Methods§
    - fn rotate_hue(&self, degrees: f32) -> Self
      - Examples found in repository?
- Dyn Compatibility§
- Implementors§

Trait for manipulating the hue of a color.

Return a new version of this color with the hue channel set to the given value.

Return the hue of this color [0.0, 360.0].

Sets the hue of this color.

Return a new version of this color with the hue channel rotated by the given degrees.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait Hue: Sized {
    // Required methods
    fn with_hue(&self, hue: f32) -> Self;
    fn hue(&self) -> f32;
    fn set_hue(&mut self, hue: f32);

    // Provided method
    fn rotate_hue(&self, degrees: f32) -> Self { ... }
}
```

Example 2 (javascript):
```javascript
13fn setup(
14    mut commands: Commands,
15    asset_server: Res<AssetServer>,
16    mut meshes: ResMut<Assets<Mesh>>,
17    mut materials: ResMut<Assets<StandardMaterial>>,
18) {
19    commands.spawn((
20        Camera3d::default(),
21        Transform::from_xyz(3.0, 1.0, 3.0).looking_at(Vec3::new(0.0, -0.5, 0.0), Vec3::Y),
22        EnvironmentMapLight {
23            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
24            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
25            intensity: 2_000.0,
26          
...
```

Example 3 (javascript):
```javascript
16fn find_top_material_and_mesh(
17    mut materials: ResMut<Assets<StandardMaterial>>,
18    mut meshes: ResMut<Assets<Mesh>>,
19    time: Res<Time>,
20    mat_query: Query<(
21        &MeshMaterial3d<StandardMaterial>,
22        &Mesh3d,
23        &GltfMaterialName,
24    )>,
25) {
26    for (mat_handle, mesh_handle, name) in mat_query.iter() {
27        // locate a material by material name
28        if name.0 == "Top" {
29            if let Some(material) = materials.get_mut(mat_handle) {
30                if let Color::Hsla(ref mut hsla) = material.base_color {
31                    *hsla
...
```

---

## Struct PrepassEnabled Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PrepassEnabled.html

**Contents:**
- Struct PrepassEnabled Copy item path
- Trait Implementations§
  - impl<M> Debug for PrepassEnabled<M>where M: Debug + Material,
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<M> Default for PrepassEnabled<M>where M: Material,
    - fn default() -> PrepassEnabled<M>
  - impl<M> Resource for PrepassEnabled<M>where M: Material, PrepassEnabled<M>: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl<M> Freeze for PrepassEnabled<M>
  - impl<M> RefUnwindSafe for PrepassEnabled<M>where M: RefUnwindSafe,

Marker resource for whether prepass is enabled globally for this material type

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PrepassEnabled<M>(/* private fields */)
where
    M: Material;
```

---

## Type Alias BoxedLayer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/type.BoxedLayer.html

**Contents:**
- Type Alias BoxedLayer Copy item path
- Aliased Type§

A boxed Layer that can be used with LogPlugin::custom_layer.

**Examples:**

Example 1 (unknown):
```unknown
pub type BoxedLayer = Box<dyn Layer<Registry> + Sync + Send>;
```

Example 2 (unknown):
```unknown
pub struct BoxedLayer(/* private fields */);
```

---

## Function update_directional_light_frusta Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/fn.update_directional_light_frusta.html

**Contents:**
- Function update_directional_light_frusta Copy item path

bevy::lightFunction update_directional_light_frusta Copy item pathSource pub fn update_directional_light_frusta( views: Query<'_, '_, (&Cascades, &DirectionalLight, &ViewVisibility, &mut CascadesFrusta), (Without<Camera>,)>, )

**Examples:**

Example 1 (unknown):
```unknown
pub fn update_directional_light_frusta(
    views: Query<'_, '_, (&Cascades, &DirectionalLight, &ViewVisibility, &mut CascadesFrusta), (Without<Camera>,)>,
)
```

---

## Struct SetPrepassViewBindGroup Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.SetPrepassViewBindGroup.html

**Contents:**
- Struct SetPrepassViewBindGroup Copy item path
- Trait Implementations§
  - impl<P, const I: usize> RenderCommand<P> for SetPrepassViewBindGroup<I>where P: PhaseItem,
    - type Param = Res<'static, PrepassViewBindGroup>
    - type ViewQuery = (&'static ViewUniformOffset, Has<MotionVectorPrepass>, Option<&'static PreviousViewUniformOffset>)
    - type ItemQuery = ()
    - fn render<'w>( _item: &P, _: (&ViewUniformOffset, bool, Option<&PreviousViewUniformOffset>), _entity: Option<()>, prepass_view_bind_group: <<SetPrepassViewBindGroup<I> as RenderCommand<P>>::Param as SystemParam>::Item<'w, '_>, pass: &mut TrackedRenderPass<'w>, ) -> RenderCommandResult
- Auto Trait Implementations§
  - impl<const I: usize> Freeze for SetPrepassViewBindGroup<I>
  - impl<const I: usize> RefUnwindSafe for SetPrepassViewBindGroup<I>

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (javascript):
```javascript
pub struct SetPrepassViewBindGroup<const I: usize>;
```

---

## Enum ShadowFilteringMethod Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/enum.ShadowFilteringMethod.html

**Contents:**
- Enum ShadowFilteringMethod Copy item path
- Variants§
  - Hardware2x2
  - Gaussian
  - Temporal
- Trait Implementations§
  - impl Clone for ShadowFilteringMethod
    - fn clone(&self) -> ShadowFilteringMethod
    - fn clone_from(&mut self, source: &Self)
  - impl Component for ShadowFilteringMethodwhere ShadowFilteringMethod: Send + Sync + 'static,

Add this component to a Camera3d to control how to anti-alias shadow edges.

The different modes use different approaches to Percentage Closer Filtering.

Fast but poor quality.

Approximates a fixed Gaussian blur, good when TAA isn’t in use.

Good quality, good performance.

For directional and spot lights, this uses a method by Ignacio Castaño for The Witness using 9 samples and smart filtering to achieve the same as a regular 5x5 filter kernel.

A randomized filter that varies over time, good when TAA is in use.

Good quality when used with TemporalAntiAliasing and good performance.

For directional and spot lights, this uses a method by Jorge Jimenez for Call of Duty: Advanced Warfare using 8 samples in spiral pattern, randomly-rotated by interleaved gradient noise with spatial variation.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ShadowFilteringMethod {
    Hardware2x2,
    Gaussian,
    Temporal,
}
```

---

## Struct ShadowSamplers Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ShadowSamplers.html

**Contents:**
- Struct ShadowSamplers Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for ShadowSamplers
    - fn clone(&self) -> ShadowSamplers
    - fn clone_from(&mut self, source: &Self)
  - impl Resource for ShadowSamplerswhere ShadowSamplers: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for ShadowSamplers
  - impl !RefUnwindSafe for ShadowSamplers

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ShadowSamplers {
    pub point_light_comparison_sampler: Sampler,
    pub point_light_linear_sampler: Sampler,
    pub directional_light_comparison_sampler: Sampler,
    pub directional_light_linear_sampler: Sampler,
}
```

---

## Struct ManageAccessibilityUpdates Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/a11y/struct.ManageAccessibilityUpdates.html

**Contents:**
- Struct ManageAccessibilityUpdates Copy item path
- Implementations§
  - impl ManageAccessibilityUpdates
    - pub fn get(&self) -> bool
    - pub fn set(&mut self, value: bool)
- Trait Implementations§
  - impl Clone for ManageAccessibilityUpdates
    - fn clone(&self) -> ManageAccessibilityUpdates
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for ManageAccessibilityUpdates

Determines whether Bevy’s ECS updates the accessibility tree.

This Resource tells Bevy internals whether it should be handling AccessKit updates (true), or if something else is doing that (false).

It defaults to true. So, by default, Bevy is configured to maintain the AccessKit tree.

Set to false in cases where an external GUI library is sending accessibility updates instead. When this option is set inconsistently with that requirement, the external library and ECS will generate conflicting updates.

Returns true if Bevy’s ECS should update the accessibility tree.

Sets whether Bevy’s ECS should update the accessibility tree.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ManageAccessibilityUpdates(/* private fields */);
```

---

## Struct ExtractedDirectionalLight Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ExtractedDirectionalLight.html

**Contents:**
- Struct ExtractedDirectionalLight Copy item path
- Fields§
- Trait Implementations§
  - impl Component for ExtractedDirectionalLightwhere ExtractedDirectionalLight: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior
    - fn on_add() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_insert() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>

whether this directional light contributes diffuse light to lightmapped meshes

True if this light is using two-phase occlusion culling.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ExtractedDirectionalLight {Show 16 fields
    pub color: LinearRgba,
    pub illuminance: f32,
    pub transform: GlobalTransform,
    pub shadows_enabled: bool,
    pub volumetric: bool,
    pub affects_lightmapped_mesh_diffuse: bool,
    pub shadow_depth_bias: f32,
    pub shadow_normal_bias: f32,
    pub cascade_shadow_config: CascadeShadowConfig,
    pub cascades: EntityHashMap<Vec<Cascade>>,
    pub frusta: EntityHashMap<Vec<Frustum>>,
    pub render_layers: RenderLayers,
    pub soft_shadow_size: Option<f32>,
    pub occlusion_culling: bool,
    pub sun_disk_angular_size: f32,
...
```

---

## Struct Hsla Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/struct.Hsla.html

**Contents:**
- Struct Hsla Copy item path
- §Conversion
- Fields§
- Implementations§
  - impl Hsla
    - pub const fn new(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Hsla
      - §Arguments
      - Examples found in repository?
    - pub const fn hsl(hue: f32, saturation: f32, lightness: f32) -> Hsla
      - §Arguments

Color in Hue-Saturation-Lightness (HSL) color space with alpha. Further information on this color model can be found on Wikipedia.

Conversion between the various color spaces is achieved using Rust’s native From trait. Because certain color spaces are defined by their transformation to and from another space, these From implementations reflect that set of definitions.

For example, the sRGB space is defined by its relationship with Linear RGB, and HWB by its with sRGB. As such, it is the responsibility of sRGB to provide From implementations for Linear RGB, and HWB for sRGB. To then provide conversion between Linear RGB and HWB directly, HWB is responsible for implementing these conversions, delegating to sRGB as an intermediatory. This ensures that all conversions take the shortest path between any two spaces, and limit the proliferation of domain specific knowledge for each color space to their respective definitions.

The hue channel. [0.0, 360.0]

The saturation channel. [0.0, 1.0]

The lightness channel. [0.0, 1.0]

The alpha channel. [0.0, 1.0]

Construct a new Hsla color from components.

Construct a new Hsla color from (h, s, l) components, with the default alpha (1.0).

Return a copy of this color with the saturation channel set to the given value.

Return a copy of this color with the lightness channel set to the given value.

Generate a deterministic but quasi-randomly distributed color from a provided index.

This can be helpful for generating debug colors.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Hsla {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
    pub alpha: f32,
}
```

Example 2 (javascript):
```javascript
let color = Srgba::rgb(0.5, 0.5, 0.5);

// Using From explicitly
let linear_color = LinearRgba::from(color);

// Using Into
let linear_color: LinearRgba = color.into();
```

Example 3 (javascript):
```javascript
75const DESELECTED: Hsla = Hsla::new(0.0, 0.3, 0.2, 0.92);
```

Example 4 (javascript):
```javascript
74const SELECTED: Hsla = Hsla::hsl(0.0, 0.9, 0.7);
```

---

## Constant CLUSTERED_FORWARD_STORAGE_BUFFER_COUNT Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/constant.CLUSTERED_FORWARD_STORAGE_BUFFER_COUNT.html

**Contents:**
- Constant CLUSTERED_FORWARD_STORAGE_BUFFER_COUNT Copy item path

bevy::pbrConstant CLUSTERED_FORWARD_STORAGE_BUFFER_COUNT Copy item pathSource pub const CLUSTERED_FORWARD_STORAGE_BUFFER_COUNT: u32 = 3; // 3u32

**Examples:**

Example 1 (javascript):
```javascript
pub const CLUSTERED_FORWARD_STORAGE_BUFFER_COUNT: u32 = 3; // 3u32
```

---

## Struct GizmoBuffer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/gizmos/struct.GizmoBuffer.html

**Contents:**
- Struct GizmoBuffer Copy item path
- Implementations§
  - impl<Config, Clear> GizmoBuffer<Config, Clear>where Config: GizmoConfigGroup, Clear: 'static + Send + Sync,
    - pub fn arc_2d( &mut self, isometry: impl Into<Isometry2d>, arc_angle: f32, radius: f32, color: impl Into<Color>, ) -> Arc2dBuilder<'_, Config, Clear>
      - §Arguments
      - §Example
      - Examples found in repository?
  - impl<Config, Clear> GizmoBuffer<Config, Clear>where Config: GizmoConfigGroup, Clear: 'static + Send + Sync,
    - pub fn arc_3d( &mut self, angle: f32, radius: f32, isometry: impl Into<Isometry3d>, color: impl Into<Color>, ) -> Arc3dBuilder<'_, Config, Clear>
      - §Arguments

Buffer for gizmo vertex data.

Draw an arc, which is a part of the circumference of a circle, in 2D.

This should be called for each frame the arc needs to be rendered.

Draw an arc, which is a part of the circumference of a circle, in 3D. For default values this is drawing a standard arc. A standard arc is defined as

This should be called for each frame the arc needs to be rendered.

The resolution of the arc (i.e. the level of detail) can be adjusted with the .resolution(...) method.

Draws the shortest arc between two points (from and to) relative to a specified center point.

The resolution of the arc (i.e. the level of detail) can be adjusted with the .resolution(...) method.

Draws the longest arc between two points (from and to) relative to a specified center point.

The resolution of the arc (i.e. the level of detail) can be adjusted with the .resolution(...) method.

Draws the shortest arc between two points (from and to) relative to a specified center point.

The resolution of the arc (i.e. the level of detail) can be adjusted with the .resolution(...) method.

Draws the longest arc between two points (from and to) relative to a specified center point.

The resolution of the arc (i.e. the level of detail) can be adjusted with the .resolution(...) method.

Draw an arrow in 3D, from start to end. Has four tips for convenient viewing from any direction.

This should be called for each frame the arrow needs to be rendered.

Draw an arrow in 2D (on the xy plane), from start to end.

This should be called for each frame the arrow needs to be rendered.

Draw a set of axes local to the given transform (transform), with length scaled by a factor of base_length.

This should be called for each frame the axes need to be rendered.

Draw a set of axes local to the given transform (transform), with length scaled by a factor of base_length.

This should be called for each frame the axes need to be rendered.

Draw an ellipse in 3D with the given isometry applied.

If isometry == Isometry3d::IDENTITY then

This should be called for each frame the ellipse needs to be rendered.

Draw an ellipse in 2D with the given isometry applied.

If isometry == Isometry2d::IDENTITY then

This should be called for each frame the ellipse needs to be rendered.

Draw a circle in 3D with the given isometry applied.

If isometry == Isometry3d::IDENTITY then

Draw a circle in 2D with the given isometry applied.

If isometry == Isometry2d::IDENTITY then

This should be called for each

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct GizmoBuffer<Config, Clear>where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,{ /* private fields */ }
```

Example 2 (unknown):
```unknown
fn system(mut gizmos: Gizmos) {
    gizmos.arc_2d(Isometry2d::IDENTITY, FRAC_PI_4, 1., GREEN);

    // Arcs have 32 line-segments by default.
    // You may want to increase this for larger arcs.
    gizmos
        .arc_2d(Isometry2d::IDENTITY, FRAC_PI_4, 5., RED)
        .resolution(64);
}
```

Example 3 (javascript):
```javascript
40fn draw_example_collection(
41    mut gizmos: Gizmos,
42    mut my_gizmos: Gizmos<MyRoundGizmos>,
43    time: Res<Time>,
44) {
45    let sin_t_scaled = ops::sin(time.elapsed_secs()) * 50.;
46    gizmos.line_2d(Vec2::Y * -sin_t_scaled, Vec2::splat(-80.), RED);
47    gizmos.ray_2d(Vec2::Y * sin_t_scaled, Vec2::splat(80.), LIME);
48
49    gizmos
50        .grid_2d(
51            Isometry2d::IDENTITY,
52            UVec2::new(16, 9),
53            Vec2::new(80., 80.),
54            // Dark gray
55            LinearRgba::gray(0.05),
56        )
57        .outer_edges();
58
59    // Triangle
60   
...
```

Example 4 (javascript):
```javascript
fn system(mut gizmos: Gizmos) {
    // rotation rotates normal to point in the direction of `Vec3::NEG_ONE`
    let rotation = Quat::from_rotation_arc(Vec3::Y, Vec3::NEG_ONE.normalize());

    gizmos
       .arc_3d(
         270.0_f32.to_radians(),
         0.25,
         Isometry3d::new(Vec3::ONE, rotation),
         ORANGE
         )
         .resolution(100);
}
```

---

## Constant LIGHTMAPS_PER_SLAB Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/constant.LIGHTMAPS_PER_SLAB.html

**Contents:**
- Constant LIGHTMAPS_PER_SLAB Copy item path

The number of lightmaps that we store in a single slab, if bindless textures are in use.

If bindless textures aren’t in use, then only a single lightmap can be bound at a time.

**Examples:**

Example 1 (javascript):
```javascript
pub const LIGHTMAPS_PER_SLAB: usize = 4; // 4usize
```

---

## Trait Alpha Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/trait.Alpha.html

**Contents:**
- Trait Alpha Copy item path
- Required Methods§
    - fn with_alpha(&self, alpha: f32) -> Self
    - fn alpha(&self) -> f32
    - fn set_alpha(&mut self, alpha: f32)
- Provided Methods§
    - fn is_fully_transparent(&self) -> bool
    - fn is_fully_opaque(&self) -> bool
- Dyn Compatibility§
- Implementations on Foreign Types§

Methods for manipulating alpha values.

Return a new version of this color with the given alpha value.

Return the alpha component of this color.

Sets the alpha component of this color.

Is the alpha component of this color less than or equal to 0.0?

Is the alpha component of this color greater than or equal to 1.0?

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait Alpha: Sized {
    // Required methods
    fn with_alpha(&self, alpha: f32) -> Self;
    fn alpha(&self) -> f32;
    fn set_alpha(&mut self, alpha: f32);

    // Provided methods
    fn is_fully_transparent(&self) -> bool { ... }
    fn is_fully_opaque(&self) -> bool { ... }
}
```

---

## Struct GpuLights Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.GpuLights.html

**Contents:**
- Struct GpuLights Copy item path
- Trait Implementations§
  - impl Clone for GpuLights
    - fn clone(&self) -> GpuLights
    - fn clone_from(&mut self, source: &Self)
  - impl CreateFrom for GpuLightswhere GpuLights: ShaderType<ExtraMetadata = StructMetadata<7>>, [GpuDirectionalLight; 10]: for<'__> CreateFrom, Vec4: for<'__> CreateFrom + for<'__> CreateFrom, UVec4: for<'__> CreateFrom, u32: for<'__> CreateFrom + for<'__> CreateFrom, i32: for<'__> CreateFrom,
    - fn create_from<B>(reader: &mut Reader<B>) -> GpuLightswhere B: BufferRef,
  - impl Debug for GpuLights
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl ReadFrom for GpuLightswhere GpuLights: ShaderType<ExtraMetadata = StructMetadata<7>>, [GpuDirectionalLight; 10]: for<'__> ReadFrom, Vec4: for<'__> ReadFrom + for<'__> ReadFrom, UVec4: for<'__> ReadFrom, u32: for<'__> ReadFrom + for<'__> ReadFrom, i32: for<'__> ReadFrom,

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GpuLights { /* private fields */ }
```

---

## Struct ViewClusterBindings Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ViewClusterBindings.html

**Contents:**
- Struct ViewClusterBindings Copy item path
- Implementations§
  - impl ViewClusterBindings
    - pub const MAX_OFFSETS: usize = 4_096usize
    - pub const MAX_INDICES: usize = 16_384usize
    - pub fn new(buffer_binding_type: BufferBindingType) -> ViewClusterBindings
    - pub fn clear(&mut self)
    - pub fn n_indices(&self) -> usize
    - pub fn push_index(&mut self, index: usize)
    - pub fn write_buffers( &mut self, render_device: &RenderDevice, render_queue: &RenderQueue, )

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ViewClusterBindings { /* private fields */ }
```

---

## Struct IoTaskPool Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/tasks/struct.IoTaskPool.html

**Contents:**
- Struct IoTaskPool Copy item path
- Implementations§
  - impl IoTaskPool
    - pub fn get_or_init(f: impl FnOnce() -> TaskPool) -> &'static IoTaskPool
    - pub fn try_get() -> Option<&'static IoTaskPool>
    - pub fn get() -> &'static IoTaskPool
      - §Panics
      - Examples found in repository?
- Methods from Deref<Target = TaskPool>§
    - pub fn thread_num(&self) -> usize

A newtype for a task pool for IO-intensive work (i.e. tasks that spend very little time in a “woken” state)

See TaskPool documentation for details on Bevy tasks.

Gets the global IoTaskPool instance, or initializes it with f.

Attempts to get the global IoTaskPool instance, or returns None if it is not initialized.

Gets the global IoTaskPool instance.

Panics if the global instance has not been initialized yet.

Return the number of threads owned by the task pool

Allows spawning non-'static futures on the thread pool. The function takes a callback, passing a scope object into it. The scope object provided to the callback can be used to spawn tasks. This function will await the completion of all tasks before returning.

This is similar to thread::scope and rayon::scope.

The Scope object takes two lifetimes: 'scope and 'env.

The 'scope lifetime represents the lifetime of the scope. That is the time during which the provided closure and tasks that are spawned into the scope are run.

The 'env lifetime represents the lifetime of whatever is borrowed by the scope. Thus this lifetime must outlive 'scope.

This allows passing an external executor to spawn tasks on. When you pass an external executor Scope::spawn_on_scope spawns is then run on the thread that ThreadExecutor is being ticked on. If None is passed the scope will use a ThreadExecutor that is ticked on the current thread.

When tick_task_pool_executor is set to true, the multithreaded task stealing executor is ticked on the scope thread. Disabling this can be useful when finishing the scope is latency sensitive. Pulling tasks from global executor can run tasks unrelated to the scope and delay when the scope returns.

See Self::scope for more details in general about how scopes work.

Spawns a static future onto the thread pool. The returned Task is a future that can be polled for the result. It can also be canceled and “detached”, allowing the task to continue running even if dropped. In any case, the pool will execute the task even without polling by the end-user.

If the provided future is non-Send, TaskPool::spawn_local should be used instead.

Spawns a static future on the thread-local async executor for the current thread. The task will run entirely on the thread the task was spawned on.

The returned Task is a future that can be polled for the result. It can also be canceled and “detached”, allowing the task to continue running even if dropped. In any case, the pool will execute the task eve

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct IoTaskPool(/* private fields */);
```

Example 2 (javascript):
```javascript
151fn setup_assets_programmatically(
152    commands: &mut Commands,
153    asset_server: &mut AssetServer,
154    animation_graphs: &mut Assets<AnimationGraph>,
155    _save: bool,
156) {
157    // Create the nodes.
158    let mut animation_graph = AnimationGraph::new();
159    let blend_node = animation_graph.add_blend(0.5, animation_graph.root);
160    animation_graph.add_clip(
161        asset_server.load(GltfAssetLabel::Animation(0).from_asset("models/animated/Fox.glb")),
162        1.0,
163        animation_graph.root,
164    );
165    animation_graph.add_clip(
166        asset_server.lo
...
```

Example 3 (javascript):
```javascript
157fn save_scene_system(world: &mut World) {
158    // Scenes can be created from any ECS World.
159    // You can either create a new one for the scene or use the current World.
160    // For demonstration purposes, we'll create a new one.
161    let mut scene_world = World::new();
162
163    // The `TypeRegistry` resource contains information about all registered types (including components).
164    // This is used to construct scenes, so we'll want to ensure that our previous type registrations
165    // exist in this new scene world as well.
166    // To do this, we can simply clone the `A
...
```

Example 4 (javascript):
```javascript
use bevy_tasks::TaskPool;

let pool = TaskPool::new();
let mut x = 0;
let results = pool.scope(|s| {
    s.spawn(async {
        // you can borrow the spawner inside a task and spawn tasks from within the task
        s.spawn(async {
            // borrow x and mutate it.
            x = 2;
            // return a value from the task
            1
        });
        // return some other value from the first task
        0
    });
});

// The ordering of results is non-deterministic if you spawn from within tasks as above.
// If you're doing this, you'll have to write your code to not depend o
...
```

---

## Struct ViewLightEntities Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ViewLightEntities.html

**Contents:**
- Struct ViewLightEntities Copy item path
- Fields§
- Trait Implementations§
  - impl Component for ViewLightEntitieswhere ViewLightEntities: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior
    - fn on_add() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_insert() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>

A component that holds the shadow cascade views for all shadow cascades associated with a camera.

Note: Despite the name, this component actually holds the shadow cascade views, not the lights themselves.

The shadow cascade views for all shadow cascades associated with a camera.

Note: Despite the name, this component actually holds the shadow cascade views, not the lights themselves.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ViewLightEntities {
    pub lights: Vec<Entity>,
}
```

---

## Struct GpuClusterableObjectsUniform Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.GpuClusterableObjectsUniform.html

**Contents:**
- Struct GpuClusterableObjectsUniform Copy item path
- Trait Implementations§
  - impl CreateFrom for GpuClusterableObjectsUniformwhere GpuClusterableObjectsUniform: ShaderType<ExtraMetadata = StructMetadata<1>>, Box<[GpuClusterableObject; 204]>: for<'__> CreateFrom,
    - fn create_from<B>(reader: &mut Reader<B>) -> GpuClusterableObjectsUniformwhere B: BufferRef,
  - impl Default for GpuClusterableObjectsUniform
    - fn default() -> GpuClusterableObjectsUniform
  - impl ReadFrom for GpuClusterableObjectsUniformwhere GpuClusterableObjectsUniform: ShaderType<ExtraMetadata = StructMetadata<1>>, Box<[GpuClusterableObject; 204]>: for<'__> ReadFrom,
    - fn read_from<B>(&mut self, reader: &mut Reader<B>)where B: BufferRef,
  - impl ShaderSize for GpuClusterableObjectsUniformwhere Box<[GpuClusterableObject; 204]>: ShaderSize,
    - const SHADER_SIZE: NonZero<u64> = _

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GpuClusterableObjectsUniform { /* private fields */ }
```

---

## Struct Hsva Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/struct.Hsva.html

**Contents:**
- Struct Hsva Copy item path
- §Conversion
- Fields§
- Implementations§
  - impl Hsva
    - pub const fn new(hue: f32, saturation: f32, value: f32, alpha: f32) -> Hsva
      - §Arguments
    - pub const fn hsv(hue: f32, saturation: f32, value: f32) -> Hsva
      - §Arguments
    - pub const fn with_saturation(self, saturation: f32) -> Hsva

Color in Hue-Saturation-Value (HSV) color space with alpha. Further information on this color model can be found on Wikipedia.

Conversion between the various color spaces is achieved using Rust’s native From trait. Because certain color spaces are defined by their transformation to and from another space, these From implementations reflect that set of definitions.

For example, the sRGB space is defined by its relationship with Linear RGB, and HWB by its with sRGB. As such, it is the responsibility of sRGB to provide From implementations for Linear RGB, and HWB for sRGB. To then provide conversion between Linear RGB and HWB directly, HWB is responsible for implementing these conversions, delegating to sRGB as an intermediatory. This ensures that all conversions take the shortest path between any two spaces, and limit the proliferation of domain specific knowledge for each color space to their respective definitions.

The hue channel. [0.0, 360.0]

The saturation channel. [0.0, 1.0]

The value channel. [0.0, 1.0]

The alpha channel. [0.0, 1.0]

Construct a new Hsva color from components.

Construct a new Hsva color from (h, s, v) components, with the default alpha (1.0).

Return a copy of this color with the saturation channel set to the given value.

Return a copy of this color with the value channel set to the given value.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Hsva {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
    pub alpha: f32,
}
```

Example 2 (javascript):
```javascript
let color = Srgba::rgb(0.5, 0.5, 0.5);

// Using From explicitly
let linear_color = LinearRgba::from(color);

// Using Into
let linear_color: LinearRgba = color.into();
```

---

## Trait AsyncWrite Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/tasks/futures_lite/trait.AsyncWrite.html

**Contents:**
- Trait AsyncWrite Copy item path
- Required Methods§
    - fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize, Error>>
      - §Implementation
    - fn poll_flush( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<(), Error>>
      - §Implementation
    - fn poll_close( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<(), Error>>
      - §Implementation
- Provided Methods§
    - fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize, Error>>

Write bytes asynchronously.

This trait is analogous to the std::io::Write trait, but integrates with the asynchronous task system. In particular, the poll_write method, unlike Write::write, will automatically queue the current task for wakeup and return if the writer cannot take more data, rather than blocking the calling thread.

Attempt to write bytes from buf into the object.

On success, returns Poll::Ready(Ok(num_bytes_written)).

If the object is not ready for writing, the method returns Poll::Pending and arranges for the current task (via cx.waker().wake_by_ref()) to receive a notification when the object becomes writable or is closed.

This function may not return errors of kind WouldBlock or Interrupted. Implementations must convert WouldBlock into Poll::Pending and either internally retry or convert Interrupted into another error kind.

poll_write must try to make progress by flushing the underlying object if that is the only way the underlying object can become writable again.

Attempt to flush the object, ensuring that any buffered data reach their destination.

On success, returns Poll::Ready(Ok(())).

If flushing cannot immediately complete, this method returns Poll::Pending and arranges for the current task (via cx.waker().wake_by_ref()) to receive a notification when the object can make progress towards flushing.

This function may not return errors of kind WouldBlock or Interrupted. Implementations must convert WouldBlock into Poll::Pending and either internally retry or convert Interrupted into another error kind.

It only makes sense to do anything here if you actually buffer data.

Attempt to close the object.

On success, returns Poll::Ready(Ok(())).

If closing cannot immediately complete, this function returns Poll::Pending and arranges for the current task (via cx.waker().wake_by_ref()) to receive a notification when the object can make progress towards closing.

This function may not return errors of kind WouldBlock or Interrupted. Implementations must convert WouldBlock into Poll::Pending and either internally retry or convert Interrupted into another error kind.

Attempt to write bytes from bufs into the object using vectored IO operations.

This method is similar to poll_write, but allows data from multiple buffers to be written using a single operation.

On success, returns Poll::Ready(Ok(num_bytes_written)).

If the object is not ready for writing, the method returns Poll::Pending and arranges for the current task (via cx.wak

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub trait AsyncWrite {
    // Required methods
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>>;
    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Error>>;
    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Error>>;

    // Provided method
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<Result<usize, Error>> { ... }
}
```

---

## Struct PrepassDrawFunction Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PrepassDrawFunction.html

**Contents:**
- Struct PrepassDrawFunction Copy item path
- Trait Implementations§
  - impl Clone for PrepassDrawFunction
    - fn clone(&self) -> PrepassDrawFunction
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for PrepassDrawFunction
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for PrepassDrawFunction
    - fn default() -> PrepassDrawFunction
  - impl DrawFunctionLabel for PrepassDrawFunctionwhere PrepassDrawFunction: 'static + Send + Sync + Clone + Eq + Debug + Hash,

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PrepassDrawFunction;
```

---

## Function collect_requested_gizmos Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/fn.collect_requested_gizmos.html

**Contents:**
- Function collect_requested_gizmos Copy item path

Collect the requested gizmos into a specific clear context.

**Examples:**

Example 1 (unknown):
```unknown
pub fn collect_requested_gizmos<Config, Clear>(
    update: ResMut<'_, GizmoStorage<Config, ()>>,
    context: ResMut<'_, GizmoStorage<Config, Clear>>,
)where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
```

---

## Enum ImageFilterMode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.ImageFilterMode.html

**Contents:**
- Enum ImageFilterMode Copy item path
- Variants§
  - Nearest
  - Linear
- Trait Implementations§
  - impl Clone for ImageFilterMode
    - fn clone(&self) -> ImageFilterMode
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for ImageFilterMode
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

Texel mixing mode when sampling between texels.

This type mirrors FilterMode.

Nearest neighbor sampling.

This creates a pixelated effect when used as a mag filter.

Linear Interpolation.

This makes textures smooth but blurry when used as a mag filter.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ImageFilterMode {
    Nearest,
    Linear,
}
```

---

## Struct Shadow Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.Shadow.html

**Contents:**
- Struct Shadow Copy item path
- Fields§
- Trait Implementations§
  - impl BinnedPhaseItem for Shadow
    - type BatchSetKey = ShadowBatchSetKey
    - type BinKey = ShadowBinKey
    - fn new( batch_set_key: <Shadow as BinnedPhaseItem>::BatchSetKey, bin_key: <Shadow as BinnedPhaseItem>::BinKey, representative_entity: (Entity, MainEntity), batch_range: Range<u32>, extra_index: PhaseItemExtraIndex, ) -> Shadow
  - impl CachedRenderPipelinePhaseItem for Shadow
    - fn cached_pipeline(&self) -> CachedRenderPipelineId
  - impl PhaseItem for Shadow

Determines which objects can be placed into a batch set.

Objects in a single batch set can potentially be multi-drawn together, if it’s enabled and the current platform supports it.

Information that separates items into bins.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Shadow {
    pub batch_set_key: ShadowBatchSetKey,
    pub bin_key: ShadowBinKey,
    pub representative_entity: (Entity, MainEntity),
    pub batch_range: Range<u32>,
    pub extra_index: PhaseItemExtraIndex,
}
```

---

## Struct LightSpecializationTicks Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.LightSpecializationTicks.html

**Contents:**
- Struct LightSpecializationTicks Copy item path
- Methods from Deref<Target = HashMap<RetainedViewEntity, Tick>>§
    - pub fn hasher(&self) -> &S
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn keys(&self) -> Keys<'_, K, V> ⓘ
      - §Examples
      - Examples found in repository?
    - pub fn values(&self) -> Values<'_, K, V> ⓘ
      - §Examples

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
pub struct LightSpecializationTicks(/* private fields */);
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

## Struct Level Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/struct.Level.html

**Contents:**
- Struct Level Copy item path
- §Comparing Levels
- §Filtering
  - §Examples
- Implementations§
  - impl Level
    - pub const ERROR: Level
    - pub const WARN: Level
    - pub const INFO: Level
    - pub const DEBUG: Level

Describes the level of verbosity of a span or event.

Level implements the PartialOrd and Ord traits, allowing two Levels to be compared to determine which is considered more or less verbose. Levels which are more verbose are considered “greater than” levels which are less verbose, with Level::ERROR considered the lowest, and Level::TRACE considered the highest.

Levels are typically used to implement filtering that determines which spans and events are enabled. Depending on the use case, more or less verbose diagnostics may be desired. For example, when running in development, DEBUG-level traces may be enabled by default. When running in production, only INFO-level and lower traces might be enabled. Libraries may include very verbose diagnostics at the DEBUG and/or TRACE levels. Applications using those libraries typically chose to ignore those traces. However, when debugging an issue involving said libraries, it may be useful to temporarily enable the more verbose traces.

The LevelFilter type is provided to enable filtering traces by verbosity. Levels can be compared against LevelFilters, and LevelFilter has a variant for each Level, which compares analogously to that level. In addition, LevelFilter adds a LevelFilter::OFF variant, which is considered “less verbose” than every other Level. This is intended to allow filters to completely disable tracing in a particular context.

Below is a simple example of how a Subscriber could implement filtering through a LevelFilter. When a span or event is recorded, the Subscriber::enabled method compares the span or event’s Level against the configured LevelFilter. The optional Subscriber::max_level_hint method can also be implemented to allow spans and events above a maximum verbosity level to be skipped more efficiently, often improving performance in short-lived programs.

It is worth noting that the tracing-subscriber crate provides additional APIs for performing more sophisticated filtering, such as enabling different levels based on which module or crate a span or event is recorded in.

Designates very serious errors.

Designates hazardous situations.

Designates useful information.

Designates lower priority information.

Designates very low priority, often extremely verbose, information.

Returns the string representation of the Level.

This returns the same string as the fmt::Display implementation.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Level(/* private fields */);
```

Example 2 (unknown):
```unknown
use tracing_core::Level;

assert!(Level::TRACE > Level::DEBUG);
assert!(Level::ERROR < Level::WARN);
assert!(Level::INFO <= Level::DEBUG);
assert_eq!(Level::TRACE, Level::TRACE);
```

Example 3 (unknown):
```unknown
use tracing_core::{Level, LevelFilter};

assert!(LevelFilter::OFF < Level::TRACE);
assert!(LevelFilter::TRACE > Level::DEBUG);
assert!(LevelFilter::ERROR < Level::WARN);
assert!(LevelFilter::INFO <= Level::DEBUG);
assert!(LevelFilter::INFO >= Level::INFO);
```

Example 4 (unknown):
```unknown
use tracing_core::{span, Event, Level, LevelFilter, Subscriber, Metadata};

#[derive(Debug)]
pub struct MySubscriber {
    /// The most verbose level that this subscriber will enable.
    max_level: LevelFilter,

    // ...
}

impl MySubscriber {
    /// Returns a new `MySubscriber` which will record spans and events up to
    /// `max_level`.
    pub fn with_max_level(max_level: LevelFilter) -> Self {
        Self {
            max_level,
            // ...
        }
    }
}
impl Subscriber for MySubscriber {
    fn enabled(&self, meta: &Metadata<'_>) -> bool {
        // A span or event is e
...
```

---

## Struct FallbackBindlessResources Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.FallbackBindlessResources.html

**Contents:**
- Struct FallbackBindlessResources Copy item path
- Trait Implementations§
  - impl Resource for FallbackBindlessResourceswhere FallbackBindlessResources: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for FallbackBindlessResources
  - impl !RefUnwindSafe for FallbackBindlessResources
  - impl Send for FallbackBindlessResources
  - impl Sync for FallbackBindlessResources
  - impl Unpin for FallbackBindlessResources
  - impl !UnwindSafe for FallbackBindlessResources

Dummy instances of various resources that we fill unused slots in binding arrays with.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct FallbackBindlessResources { /* private fields */ }
```

---

## Struct EnvironmentMapLight Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.EnvironmentMapLight.html

**Contents:**
- Struct EnvironmentMapLight Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for EnvironmentMapLight
    - fn clone(&self) -> EnvironmentMapLight
    - fn clone_from(&mut self, source: &Self)
  - impl Component for EnvironmentMapLightwhere EnvironmentMapLight: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

A pair of cubemap textures that represent the surroundings of a specific area in space.

See bevy_pbr::environment_map for detailed information.

The blurry image that represents diffuse radiance surrounding a region.

The typically-sharper, mipmapped image that represents specular radiance surrounding a region.

Scale factor applied to the diffuse and specular light generated by this component.

After applying this multiplier, the resulting values should be in units of cd/m^2.

See also https://google.github.io/filament/Filament.html#lighting/imagebasedlights/iblunit.

World space rotation applied to the environment light cubemaps. This is useful for users who require a different axis, such as the Z-axis, to serve as the vertical axis.

Whether the light from this environment map contributes diffuse lighting to meshes with lightmaps.

Set this to false if your lightmap baking tool bakes the diffuse light from this environment light into the lightmaps in order to avoid counting the radiance from this environment map twice.

By default, this is set to true.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct EnvironmentMapLight {
    pub diffuse_map: Handle<Image>,
    pub specular_map: Handle<Image>,
    pub intensity: f32,
    pub rotation: Quat,
    pub affects_lightmapped_mesh_diffuse: bool,
}
```

---

## Struct ViewEnvironmentMapUniformOffset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ViewEnvironmentMapUniformOffset.html

**Contents:**
- Struct ViewEnvironmentMapUniformOffset Copy item path
- Methods from Deref<Target = u32>§
    - pub const MIN: u32 = 0u32
    - pub const MAX: u32 = 4_294_967_295u32
    - pub const BITS: u32 = 32u32
- Trait Implementations§
  - impl Component for ViewEnvironmentMapUniformOffsetwhere ViewEnvironmentMapUniformOffset: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

A component that stores the offset within the EnvironmentMapUniformBuffer for each view.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ViewEnvironmentMapUniformOffset(/* private fields */);
```

---

## Struct PreprocessPhasePipelines Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PreprocessPhasePipelines.html

**Contents:**
- Struct PreprocessPhasePipelines Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for PreprocessPhasePipelines
    - fn clone(&self) -> PreprocessPhasePipelines
    - fn clone_from(&mut self, source: &Self)
- Auto Trait Implementations§
  - impl Freeze for PreprocessPhasePipelines
  - impl !RefUnwindSafe for PreprocessPhasePipelines
  - impl Send for PreprocessPhasePipelines

Compute shader pipelines for a specific phase: early, late, or main.

The distinction between these phases is relevant for occlusion culling.

The pipeline that resets the indirect draw counts used in multi_draw_indirect_count to 0 in preparation for a new pass.

The pipeline used for indexed indirect parameter building.

This pipeline converts indirect parameter metadata into indexed indirect parameters.

The pipeline used for non-indexed indirect parameter building.

This pipeline converts indirect parameter metadata into non-indexed indirect parameters.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PreprocessPhasePipelines {
    pub reset_indirect_batch_sets: ResetIndirectBatchSetsPipeline,
    pub gpu_occlusion_culling_build_indexed_indirect_params: BuildIndirectParametersPipeline,
    pub gpu_occlusion_culling_build_non_indexed_indirect_params: BuildIndirectParametersPipeline,
}
```

---

## Macro warn Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/macro.warn.html

**Contents:**
- Macro warn Copy item path
- §Examples

Constructs an event at the warn level.

This functions similarly to the event! macro. See the top-level documentation for details on the syntax accepted by this macro.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! warn {
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, {
...
```

Example 2 (javascript):
```javascript
use tracing::warn;

let warn_description = "Invalid Input";
let input = &[0x27, 0x45];

warn!(?input, warning = warn_description);
warn!(
    target: "input_events",
    warning = warn_description,
    "Received warning for input: {:?}", input,
);
warn!(name: "invalid", ?input);
```

---

## Struct Hwba Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/struct.Hwba.html

**Contents:**
- Struct Hwba Copy item path
- §Conversion
- Fields§
- Implementations§
  - impl Hwba
    - pub const fn new(hue: f32, whiteness: f32, blackness: f32, alpha: f32) -> Hwba
      - §Arguments
    - pub const fn hwb(hue: f32, whiteness: f32, blackness: f32) -> Hwba
      - §Arguments
    - pub const fn with_whiteness(self, whiteness: f32) -> Hwba

Color in Hue-Whiteness-Blackness (HWB) color space with alpha. Further information on this color model can be found on Wikipedia.

Conversion between the various color spaces is achieved using Rust’s native From trait. Because certain color spaces are defined by their transformation to and from another space, these From implementations reflect that set of definitions.

For example, the sRGB space is defined by its relationship with Linear RGB, and HWB by its with sRGB. As such, it is the responsibility of sRGB to provide From implementations for Linear RGB, and HWB for sRGB. To then provide conversion between Linear RGB and HWB directly, HWB is responsible for implementing these conversions, delegating to sRGB as an intermediatory. This ensures that all conversions take the shortest path between any two spaces, and limit the proliferation of domain specific knowledge for each color space to their respective definitions.

The hue channel. [0.0, 360.0]

The whiteness channel. [0.0, 1.0]

The blackness channel. [0.0, 1.0]

The alpha channel. [0.0, 1.0]

Construct a new Hwba color from components.

Construct a new Hwba color from (h, s, l) components, with the default alpha (1.0).

Return a copy of this color with the whiteness channel set to the given value.

Return a copy of this color with the blackness channel set to the given value.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Hwba {
    pub hue: f32,
    pub whiteness: f32,
    pub blackness: f32,
    pub alpha: f32,
}
```

Example 2 (javascript):
```javascript
let color = Srgba::rgb(0.5, 0.5, 0.5);

// Using From explicitly
let linear_color = LinearRgba::from(color);

// Using Into
let linear_color: LinearRgba = color.into();
```

---

## Struct Xyza Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/struct.Xyza.html

**Contents:**
- Struct Xyza Copy item path
- §Conversion
- Fields§
- Implementations§
  - impl Xyza
    - pub const D65_WHITE: Xyza
    - pub const fn new(x: f32, y: f32, z: f32, alpha: f32) -> Xyza
      - §Arguments
    - pub const fn xyz(x: f32, y: f32, z: f32) -> Xyza
      - §Arguments

CIE 1931 color space, also known as XYZ, with an alpha channel.

Conversion between the various color spaces is achieved using Rust’s native From trait. Because certain color spaces are defined by their transformation to and from another space, these From implementations reflect that set of definitions.

For example, the sRGB space is defined by its relationship with Linear RGB, and HWB by its with sRGB. As such, it is the responsibility of sRGB to provide From implementations for Linear RGB, and HWB for sRGB. To then provide conversion between Linear RGB and HWB directly, HWB is responsible for implementing these conversions, delegating to sRGB as an intermediatory. This ensures that all conversions take the shortest path between any two spaces, and limit the proliferation of domain specific knowledge for each color space to their respective definitions.

The x-axis. [0.0, 1.0]

The y-axis, intended to represent luminance. [0.0, 1.0]

The z-axis. [0.0, 1.0]

The alpha channel. [0.0, 1.0]

Construct a new Xyza color from components.

Construct a new Xyza color from (x, y, z) components, with the default alpha (1.0).

Return a copy of this color with the ‘x’ channel set to the given value.

Return a copy of this color with the ‘y’ channel set to the given value.

Return a copy of this color with the ‘z’ channel set to the given value.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Xyza {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub alpha: f32,
}
```

Example 2 (javascript):
```javascript
let color = Srgba::rgb(0.5, 0.5, 0.5);

// Using From explicitly
let linear_color = LinearRgba::from(color);

// Using Into
let linear_color: LinearRgba = color.into();
```

---

## Constant MAX_CASCADES_PER_LIGHT Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/constant.MAX_CASCADES_PER_LIGHT.html

**Contents:**
- Constant MAX_CASCADES_PER_LIGHT Copy item path

bevy::pbrConstant MAX_CASCADES_PER_LIGHT Copy item pathSource pub const MAX_CASCADES_PER_LIGHT: usize = 4; // 4usize

**Examples:**

Example 1 (javascript):
```javascript
pub const MAX_CASCADES_PER_LIGHT: usize = 4; // 4usize
```

---

## Struct DirectionalLightShadowMap Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.DirectionalLightShadowMap.html

**Contents:**
- Struct DirectionalLightShadowMap Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for DirectionalLightShadowMap
    - fn clone(&self) -> DirectionalLightShadowMap
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for DirectionalLightShadowMap
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for DirectionalLightShadowMap
    - fn default() -> DirectionalLightShadowMap

Controls the resolution of DirectionalLight and SpotLight shadow maps.

Must be a power of two to avoid unstable cascade positioning.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DirectionalLightShadowMap {
    pub size: usize,
}
```

Example 2 (unknown):
```unknown
App::new()
    .insert_resource(DirectionalLightShadowMap { size: 4096 });
```

---

## Struct PrepassViewBindGroup Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PrepassViewBindGroup.html

**Contents:**
- Struct PrepassViewBindGroup Copy item path
- Fields§
- Trait Implementations§
  - impl Resource for PrepassViewBindGroupwhere PrepassViewBindGroup: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for PrepassViewBindGroup
  - impl !RefUnwindSafe for PrepassViewBindGroup
  - impl Send for PrepassViewBindGroup
  - impl Sync for PrepassViewBindGroup
  - impl Unpin for PrepassViewBindGroup

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PrepassViewBindGroup {
    pub motion_vectors: Option<BindGroup>,
    pub no_motion_vectors: Option<BindGroup>,
    pub empty_bind_group: BindGroup,
}
```

---

## Struct Laba Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/struct.Laba.html

**Contents:**
- Struct Laba Copy item path
- §Conversion
- Fields§
- Implementations§
  - impl Laba
    - pub const CIE_EPSILON: f32 = 0.00885645207f32
    - pub const CIE_KAPPA: f32 = 903.296325f32
    - pub const fn new(lightness: f32, a: f32, b: f32, alpha: f32) -> Laba
      - §Arguments
    - pub const fn lab(lightness: f32, a: f32, b: f32) -> Laba

Color in LAB color space, with alpha

Conversion between the various color spaces is achieved using Rust’s native From trait. Because certain color spaces are defined by their transformation to and from another space, these From implementations reflect that set of definitions.

For example, the sRGB space is defined by its relationship with Linear RGB, and HWB by its with sRGB. As such, it is the responsibility of sRGB to provide From implementations for Linear RGB, and HWB for sRGB. To then provide conversion between Linear RGB and HWB directly, HWB is responsible for implementing these conversions, delegating to sRGB as an intermediatory. This ensures that all conversions take the shortest path between any two spaces, and limit the proliferation of domain specific knowledge for each color space to their respective definitions.

The lightness channel. [0.0, 1.5]

The a axis. [-1.5, 1.5]

The b axis. [-1.5, 1.5]

The alpha channel. [0.0, 1.0]

See Continuity (16) (17)

See Continuity (16) (17)

Construct a new Laba color from components.

Construct a new Laba color from (l, a, b) components, with the default alpha (1.0).

Return a copy of this color with the lightness channel set to the given value.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Laba {
    pub lightness: f32,
    pub a: f32,
    pub b: f32,
    pub alpha: f32,
}
```

Example 2 (javascript):
```javascript
let color = Srgba::rgb(0.5, 0.5, 0.5);

// Using From explicitly
let linear_color = LinearRgba::from(color);

// Using Into
let linear_color: LinearRgba = color.into();
```

---

## Function spot_light_clip_from_view Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/fn.spot_light_clip_from_view.html

**Contents:**
- Function spot_light_clip_from_view Copy item path

bevy::lightFunction spot_light_clip_from_view Copy item pathSource pub fn spot_light_clip_from_view(angle: f32, near_z: f32) -> Mat4

**Examples:**

Example 1 (unknown):
```unknown
pub fn spot_light_clip_from_view(angle: f32, near_z: f32) -> Mat4
```

---

## Constant MAX_DIRECTIONAL_LIGHTS Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/constant.MAX_DIRECTIONAL_LIGHTS.html

**Contents:**
- Constant MAX_DIRECTIONAL_LIGHTS Copy item path

bevy::pbrConstant MAX_DIRECTIONAL_LIGHTS Copy item pathSource pub const MAX_DIRECTIONAL_LIGHTS: usize = 10; // 10usize

**Examples:**

Example 1 (javascript):
```javascript
pub const MAX_DIRECTIONAL_LIGHTS: usize = 10; // 10usize
```

---

## Trait ColorToPacked Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/trait.ColorToPacked.html

**Contents:**
- Trait ColorToPacked Copy item path
- Required Methods§
    - fn to_u8_array(self) -> [u8; 4]
    - fn to_u8_array_no_alpha(self) -> [u8; 3]
    - fn from_u8_array(color: [u8; 4]) -> Self
    - fn from_u8_array_no_alpha(color: [u8; 3]) -> Self
- Dyn Compatibility§
- Implementors§
  - impl ColorToPacked for LinearRgba
  - impl ColorToPacked for Srgba

Trait with methods for converting colors to packed non-color types

Convert to [u8; 4] where that makes sense (Srgba is most relevant)

Convert to [u8; 3] where that makes sense (Srgba is most relevant)

Convert from [u8; 4] where that makes sense (Srgba is most relevant)

Convert to [u8; 3] where that makes sense (Srgba is most relevant)

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait ColorToPacked {
    // Required methods
    fn to_u8_array(self) -> [u8; 4];
    fn to_u8_array_no_alpha(self) -> [u8; 3];
    fn from_u8_array(color: [u8; 4]) -> Self;
    fn from_u8_array_no_alpha(color: [u8; 3]) -> Self;
}
```

---

## Struct PointLightShadowMap Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.PointLightShadowMap.html

**Contents:**
- Struct PointLightShadowMap Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for PointLightShadowMap
    - fn clone(&self) -> PointLightShadowMap
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for PointLightShadowMap
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for PointLightShadowMap
    - fn default() -> PointLightShadowMap

Controls the resolution of PointLight shadow maps.

The width and height of each of the 6 faces of the cubemap.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PointLightShadowMap {
    pub size: usize,
}
```

Example 2 (unknown):
```unknown
App::new()
    .insert_resource(PointLightShadowMap { size: 2048 });
```

---

## Struct SpotLight Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.SpotLight.html

**Contents:**
- Struct SpotLight Copy item path
- Fields§
- Implementations§
  - impl SpotLight
    - pub const DEFAULT_SHADOW_DEPTH_BIAS: f32 = 0.0199999996f32
    - pub const DEFAULT_SHADOW_NORMAL_BIAS: f32 = 1.79999995f32
    - pub const DEFAULT_SHADOW_MAP_NEAR_Z: f32 = 0.100000001f32
- Trait Implementations§
  - impl Clone for SpotLight
    - fn clone(&self) -> SpotLight

A light that emits light in a given direction from a central point.

Behaves like a point light in a perfectly absorbent housing that shines light only in a given direction. The direction is taken from the transform, and can be specified with Transform::looking_at.

To control the resolution of the shadow maps, use the DirectionalLightShadowMap resource.

The color of the light.

By default, this is white.

Luminous power in lumens, representing the amount of light emitted by this source in all directions.

Range in meters that this light illuminates.

Note that this value affects resolution of the shadow maps; generally, the higher you set it, the lower-resolution your shadow maps will be. Consequently, you should set this value to be only the size that you need.

Simulates a light source coming from a spherical volume with the given radius.

This affects the size of specular highlights created by this light, as well as the soft shadow penumbra size. Because of this, large values may not produce the intended result – for example, light radius does not affect shadow softness or diffuse lighting.

Whether this light casts shadows.

Note that shadows are rather expensive and become more so with every light that casts them. In general, it’s best to aggressively limit the number of lights with shadows enabled to one or two at most.

Whether soft shadows are enabled.

Soft shadows, also known as percentage-closer soft shadows or PCSS, cause shadows to become blurrier (i.e. their penumbra increases in radius) as they extend away from objects. The blurriness of the shadow depends on the SpotLight::radius of the light; larger lights result in larger penumbras and therefore blurrier shadows.

Currently, soft shadows are rather noisy if not using the temporal mode. If you enable soft shadows, consider choosing ShadowFilteringMethod::Temporal and enabling temporal antialiasing (TAA) to smooth the noise out over time.

Note that soft shadows are significantly more expensive to render than hard shadows.

Whether this spot light contributes diffuse lighting to meshes with lightmaps.

Set this to false if your lightmap baking tool bakes the direct diffuse light from this directional light into the lightmaps in order to avoid counting the radiance from this light twice. Note that the specular portion of the light is always considered, because Bevy currently has no means to bake specular light.

By default, this is set to true.

A value that adjusts the tradeoff between se

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct SpotLight {
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
    pub radius: f32,
    pub shadows_enabled: bool,
    pub soft_shadows_enabled: bool,
    pub affects_lightmapped_mesh_diffuse: bool,
    pub shadow_depth_bias: f32,
    pub shadow_normal_bias: f32,
    pub shadow_map_near_z: f32,
    pub outer_angle: f32,
    pub inner_angle: f32,
}
```

---

## Function ktx2_buffer_to_image Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/fn.ktx2_buffer_to_image.html

**Contents:**
- Function ktx2_buffer_to_image Copy item path

bevy::imageFunction ktx2_buffer_to_image Copy item pathSource pub fn ktx2_buffer_to_image( buffer: &[u8], supported_compressed_formats: CompressedImageFormats, is_srgb: bool, ) -> Result<Image, TextureError>

**Examples:**

Example 1 (unknown):
```unknown
pub fn ktx2_buffer_to_image(
    buffer: &[u8],
    supported_compressed_formats: CompressedImageFormats,
    is_srgb: bool,
) -> Result<Image, TextureError>
```

---

## Struct GlobalClusterableObjectMeta Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.GlobalClusterableObjectMeta.html

**Contents:**
- Struct GlobalClusterableObjectMeta Copy item path
- Fields§
- Implementations§
  - impl GlobalClusterableObjectMeta
    - pub fn new( buffer_binding_type: BufferBindingType, ) -> GlobalClusterableObjectMeta
- Trait Implementations§
  - impl Resource for GlobalClusterableObjectMetawhere GlobalClusterableObjectMeta: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for GlobalClusterableObjectMeta
  - impl !RefUnwindSafe for GlobalClusterableObjectMeta

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GlobalClusterableObjectMeta {
    pub gpu_clusterable_objects: GpuClusterableObjects,
    pub entity_to_index: EntityHashMap<usize>,
}
```

---

## Type Alias BoxedFmtLayer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/type.BoxedFmtLayer.html

**Contents:**
- Type Alias BoxedFmtLayer Copy item path
- Aliased Type§

A boxed Layer that can be used with LogPlugin::fmt_layer.

**Examples:**

Example 1 (unknown):
```unknown
pub type BoxedFmtLayer = Box<dyn Layer<Layered<ErrorLayer<Layered<EnvFilter, Layered<Option<Box<dyn Layer<Registry> + Sync + Send>>, Registry>>>, Layered<EnvFilter, Layered<Option<Box<dyn Layer<Registry> + Sync + Send>>, Registry>>>> + Sync + Send>;
```

Example 2 (unknown):
```unknown
pub struct BoxedFmtLayer(/* private fields */);
```

---

## Struct ExtractedClusterConfig Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ExtractedClusterConfig.html

**Contents:**
- Struct ExtractedClusterConfig Copy item path
- Trait Implementations§
  - impl Component for ExtractedClusterConfigwhere ExtractedClusterConfig: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior
    - fn on_add() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_insert() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_replace() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ExtractedClusterConfig { /* private fields */ }
```

---

## Struct GpuDirectionalLight Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.GpuDirectionalLight.html

**Contents:**
- Struct GpuDirectionalLight Copy item path
- Trait Implementations§
  - impl Clone for GpuDirectionalLight
    - fn clone(&self) -> GpuDirectionalLight
    - fn clone_from(&mut self, source: &Self)
  - impl CreateFrom for GpuDirectionalLightwhere GpuDirectionalLight: ShaderType<ExtraMetadata = StructMetadata<13>>, [GpuDirectionalCascade; 4]: for<'__> CreateFrom, Vec4: for<'__> CreateFrom, Vec3: for<'__> CreateFrom, u32: for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom, f32: for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom,
    - fn create_from<B>(reader: &mut Reader<B>) -> GpuDirectionalLightwhere B: BufferRef,
  - impl Debug for GpuDirectionalLight
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for GpuDirectionalLight

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GpuDirectionalLight { /* private fields */ }
```

---

## Enum HexColorError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/enum.HexColorError.html

**Contents:**
- Enum HexColorError Copy item path
- Variants§
  - Parse(ParseIntError)
  - Length
  - Char(char)
- Trait Implementations§
  - impl Debug for HexColorError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for HexColorError
    - fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>

Error returned if a hex string could not be parsed as a color.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum HexColorError {
    Parse(ParseIntError),
    Length,
    Char(char),
}
```

---

## Function dds_buffer_to_image Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/fn.dds_buffer_to_image.html

**Contents:**
- Function dds_buffer_to_image Copy item path

bevy::imageFunction dds_buffer_to_image Copy item pathSource pub fn dds_buffer_to_image( buffer: &[u8], supported_compressed_formats: CompressedImageFormats, is_srgb: bool, ) -> Result<Image, TextureError>

**Examples:**

Example 1 (unknown):
```unknown
pub fn dds_buffer_to_image(
    buffer: &[u8],
    supported_compressed_formats: CompressedImageFormats,
    is_srgb: bool,
) -> Result<Image, TextureError>
```

---

## Struct ShadowsEnabled Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ShadowsEnabled.html

**Contents:**
- Struct ShadowsEnabled Copy item path
- Trait Implementations§
  - impl<M> Debug for ShadowsEnabled<M>where M: Debug + Material,
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<M> Default for ShadowsEnabled<M>where M: Material,
    - fn default() -> ShadowsEnabled<M>
  - impl<M> Resource for ShadowsEnabled<M>where M: Material, ShadowsEnabled<M>: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl<M> Freeze for ShadowsEnabled<M>
  - impl<M> RefUnwindSafe for ShadowsEnabled<M>where M: RefUnwindSafe,

Marker resource for whether shadows are enabled for this material type

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ShadowsEnabled<M>(/* private fields */)
where
    M: Material;
```

---

## Struct FogMeta Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.FogMeta.html

**Contents:**
- Struct FogMeta Copy item path
- Fields§
- Trait Implementations§
  - impl Default for FogMeta
    - fn default() -> FogMeta
  - impl Resource for FogMetawhere FogMeta: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for FogMeta
  - impl !RefUnwindSafe for FogMeta
  - impl Send for FogMeta

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct FogMeta {
    pub gpu_fogs: DynamicUniformBuffer<GpuFog>,
}
```

---

## Struct PointLight Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.PointLight.html

**Contents:**
- Struct PointLight Copy item path
  - §Shadows
- Fields§
- Implementations§
  - impl PointLight
    - pub const DEFAULT_SHADOW_DEPTH_BIAS: f32 = 0.0799999982f32
    - pub const DEFAULT_SHADOW_NORMAL_BIAS: f32 = 0.600000024f32
    - pub const DEFAULT_SHADOW_MAP_NEAR_Z: f32 = 0.100000001f32
- Trait Implementations§
  - impl Clone for PointLight

A light that emits light in all directions from a central point.

Real-world values for intensity (luminous power in lumens) based on the electrical power consumption of the type of real-world light are:

To enable shadows, set the shadows_enabled property to true.

To control the resolution of the shadow maps, use the PointLightShadowMap resource.

The color of this light source.

Luminous power in lumens, representing the amount of light emitted by this source in all directions.

Cut-off for the light’s area-of-effect. Fragments outside this range will not be affected by this light at all, so it’s important to tune this together with intensity to prevent hard lighting cut-offs.

Simulates a light source coming from a spherical volume with the given radius.

This affects the size of specular highlights created by this light, as well as the soft shadow penumbra size. Because of this, large values may not produce the intended result – for example, light radius does not affect shadow softness or diffuse lighting.

Whether this light casts shadows.

Whether soft shadows are enabled.

Soft shadows, also known as percentage-closer soft shadows or PCSS, cause shadows to become blurrier (i.e. their penumbra increases in radius) as they extend away from objects. The blurriness of the shadow depends on the PointLight::radius of the light; larger lights result in larger penumbras and therefore blurrier shadows.

Currently, soft shadows are rather noisy if not using the temporal mode. If you enable soft shadows, consider choosing ShadowFilteringMethod::Temporal and enabling temporal antialiasing (TAA) to smooth the noise out over time.

Note that soft shadows are significantly more expensive to render than hard shadows.

Whether this point light contributes diffuse lighting to meshes with lightmaps.

Set this to false if your lightmap baking tool bakes the direct diffuse light from this point light into the lightmaps in order to avoid counting the radiance from this light twice. Note that the specular portion of the light is always considered, because Bevy currently has no means to bake specular light.

By default, this is set to true.

A bias used when sampling shadow maps to avoid “shadow-acne”, or false shadow occlusions that happen as a result of shadow-map fragments not mapping 1:1 to screen-space fragments. Too high of a depth bias can lead to shadows detaching from their casters, or “peter-panning”. This bias can be tuned together with shadow_normal_bias to co

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct PointLight {
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
    pub radius: f32,
    pub shadows_enabled: bool,
    pub soft_shadows_enabled: bool,
    pub affects_lightmapped_mesh_diffuse: bool,
    pub shadow_depth_bias: f32,
    pub shadow_normal_bias: f32,
    pub shadow_map_near_z: f32,
}
```

---

## Enum DataFormat Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.DataFormat.html

**Contents:**
- Enum DataFormat Copy item path
- Variants§
  - Rgb
  - Rgba
  - Rrr
  - Rrrg
  - Rg
- Trait Implementations§
  - impl Clone for DataFormat
    - fn clone(&self) -> DataFormat

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum DataFormat {
    Rgb,
    Rgba,
    Rrr,
    Rrrg,
    Rg,
}
```

---

## Struct GpuAtmosphereSettings Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.GpuAtmosphereSettings.html

**Contents:**
- Struct GpuAtmosphereSettings Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for GpuAtmosphereSettings
    - fn clone(&self) -> GpuAtmosphereSettings
    - fn clone_from(&mut self, source: &Self)
  - impl Component for GpuAtmosphereSettingswhere GpuAtmosphereSettings: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GpuAtmosphereSettings {Show 13 fields
    pub transmittance_lut_size: UVec2,
    pub multiscattering_lut_size: UVec2,
    pub sky_view_lut_size: UVec2,
    pub aerial_view_lut_size: UVec3,
    pub transmittance_lut_samples: u32,
    pub multiscattering_lut_dirs: u32,
    pub multiscattering_lut_samples: u32,
    pub sky_view_lut_samples: u32,
    pub aerial_view_lut_samples: u32,
    pub aerial_view_lut_max_distance: f32,
    pub scene_units_to_m: f32,
    pub sky_max_samples: u32,
    pub rendering_method: u32,
}
```

---

## Struct ExtractedPointLight Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ExtractedPointLight.html

**Contents:**
- Struct ExtractedPointLight Copy item path
- Fields§
- Trait Implementations§
  - impl Component for ExtractedPointLightwhere ExtractedPointLight: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior
    - fn on_add() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_insert() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>

luminous intensity in lumens per steradian

whether this point light contributes diffuse light to lightmapped meshes

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ExtractedPointLight {Show 13 fields
    pub color: LinearRgba,
    pub intensity: f32,
    pub range: f32,
    pub radius: f32,
    pub transform: GlobalTransform,
    pub shadows_enabled: bool,
    pub shadow_depth_bias: f32,
    pub shadow_normal_bias: f32,
    pub shadow_map_near_z: f32,
    pub spot_light_angles: Option<(f32, f32)>,
    pub volumetric: bool,
    pub soft_shadows_enabled: bool,
    pub affects_lightmapped_mesh_diffuse: bool,
}
```

---

## Module constants Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/feathers/constants/index.html

**Contents:**
- Module constants Copy item path
- Modules§

Various non-themable constants for the Feathers look and feel.

---

## Struct FogVolume Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.FogVolume.html

**Contents:**
- Struct FogVolume Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for FogVolume
    - fn clone(&self) -> FogVolume
    - fn clone_from(&mut self, source: &Self)
  - impl Component for FogVolumewhere FogVolume: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

The color of the fog.

Note that the fog must be lit by a VolumetricLight or ambient light in order for this color to appear.

The density of fog, which measures how dark the fog is.

The default value is 0.1.

Optional 3D voxel density texture for the fog.

Configurable offset of the density texture in UVW coordinates.

This can be used to scroll a repeating density texture in a direction over time to create effects like fog moving in the wind. Make sure to configure the texture to use ImageAddressMode::Repeat if this is your intention.

Has no effect when no density texture is present.

The default value is (0, 0, 0).

The absorption coefficient, which measures what fraction of light is absorbed by the fog at each step.

Increasing this value makes the fog darker.

The default value is 0.3.

The scattering coefficient, which measures the fraction of light that’s scattered toward, and away from, the viewer.

The default value is 0.3.

Measures the fraction of light that’s scattered toward the camera, as opposed to away from the camera.

Increasing this value makes light shafts become more prominent when the camera is facing toward their source and less prominent when the camera is facing away. Essentially, a high value here means the light shafts will fade into view as the camera focuses on them and fade away when the camera is pointing away.

The default value is 0.8.

Applies a nonphysical color to the light.

This can be useful for artistic purposes but is nonphysical.

The default value is white.

Scales the light by a fixed fraction.

This can be useful for artistic purposes but is nonphysical.

The default value is 1.0, which results in no adjustment.

Required Components: Transform, Visibility.

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components of the required components, recursively, in depth-first order.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct FogVolume {
    pub fog_color: Color,
    pub density_factor: f32,
    pub density_texture: Option<Handle<Image>>,
    pub density_texture_offset: Vec3,
    pub absorption: f32,
    pub scattering: f32,
    pub scattering_asymmetry: f32,
    pub light_tint: Color,
    pub light_intensity: f32,
}
```

---

## Struct Oklaba Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/struct.Oklaba.html

**Contents:**
- Struct Oklaba Copy item path
- §Conversion
- Fields§
- Implementations§
  - impl Oklaba
    - pub const fn new(lightness: f32, a: f32, b: f32, alpha: f32) -> Oklaba
      - §Arguments
    - pub const fn lab(lightness: f32, a: f32, b: f32) -> Oklaba
      - §Arguments
    - pub const fn with_lightness(self, lightness: f32) -> Oklaba

Color in Oklab color space, with alpha

Conversion between the various color spaces is achieved using Rust’s native From trait. Because certain color spaces are defined by their transformation to and from another space, these From implementations reflect that set of definitions.

For example, the sRGB space is defined by its relationship with Linear RGB, and HWB by its with sRGB. As such, it is the responsibility of sRGB to provide From implementations for Linear RGB, and HWB for sRGB. To then provide conversion between Linear RGB and HWB directly, HWB is responsible for implementing these conversions, delegating to sRGB as an intermediatory. This ensures that all conversions take the shortest path between any two spaces, and limit the proliferation of domain specific knowledge for each color space to their respective definitions.

The ‘lightness’ channel. [0.0, 1.0]

The ‘a’ channel. [-1.0, 1.0]

The ‘b’ channel. [-1.0, 1.0]

The alpha channel. [0.0, 1.0]

Construct a new Oklaba color from components.

Construct a new Oklaba color from (l, a, b) components, with the default alpha (1.0).

Return a copy of this color with the ‘lightness’ channel set to the given value.

Return a copy of this color with the ‘a’ channel set to the given value.

Return a copy of this color with the ‘b’ channel set to the given value.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Oklaba {
    pub lightness: f32,
    pub a: f32,
    pub b: f32,
    pub alpha: f32,
}
```

Example 2 (javascript):
```javascript
let color = Srgba::rgb(0.5, 0.5, 0.5);

// Using From explicitly
let linear_color = LinearRgba::from(color);

// Using Into
let linear_color: LinearRgba = color.into();
```

---

## Macro info Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/macro.info.html

**Contents:**
- Macro info Copy item path
- §Examples

Constructs an event at the info level.

This functions similarly to the event! macro. See the top-level documentation for details on the syntax accepted by this macro.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! info {
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, {
...
```

Example 2 (javascript):
```javascript
use tracing::info;
use tracing::field;

let addr = Ipv4Addr::new(127, 0, 0, 1);
let conn = Connection { port: 40, speed: 3.20 };

info!(conn.port, "connected to {:?}", addr);
info!(
    target: "connection_events",
    ip = ?addr,
    conn.port,
    ?conn.speed,
);
info!(name: "completed", "completed connection to {:?}", addr);
```

---

## Struct LightKeyCache Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.LightKeyCache.html

**Contents:**
- Struct LightKeyCache Copy item path
- Methods from Deref<Target = HashMap<RetainedViewEntity, MeshPipelineKey>>§
    - pub fn hasher(&self) -> &S
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn keys(&self) -> Keys<'_, K, V> ⓘ
      - §Examples
      - Examples found in repository?
    - pub fn values(&self) -> Values<'_, K, V> ⓘ
      - §Examples

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
pub struct LightKeyCache(/* private fields */);
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

## Enum IntoDynamicImageError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.IntoDynamicImageError.html

**Contents:**
- Enum IntoDynamicImageError Copy item path
- Variants (Non-exhaustive)§
  - UnsupportedFormat(TextureFormat)
  - UnknownConversionError(TextureFormat)
  - UninitializedImage
- Trait Implementations§
  - impl Debug for IntoDynamicImageError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for IntoDynamicImageError
    - fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>

Errors that occur while converting an Image into a DynamicImage

Conversion into dynamic image not supported for source format.

Encountered an unknown error during conversion.

Tried to convert an image that has no texture data

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[non_exhaustive]pub enum IntoDynamicImageError {
    UnsupportedFormat(TextureFormat),
    UnknownConversionError(TextureFormat),
    UninitializedImage,
}
```

---

## Crate tracing_subscriber Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/tracing_subscriber/index.html

**Contents:**
- Crate tracing_subscriber Copy item path
  - §Layers and Filters
  - §Included Subscribers
  - §Feature Flags
    - §Optional Dependencies
    - §no_std Support
    - §Unstable Features
      - §Enabling Unstable Features
  - §Supported Rust Versions
- Modules§

Utilities for implementing and composing tracing subscribers.

tracing is a framework for instrumenting Rust programs to collect scoped, structured, and async-aware diagnostics. The Subscriber trait represents the functionality necessary to collect this trace data. This crate contains tools for composing subscribers out of smaller units of behaviour, and batteries-included implementations of common subscriber functionality.

tracing-subscriber is intended for use by both Subscriber authors and application authors using tracing to instrument their applications.

Compiler support: requires rustc 1.65+

The most important component of the tracing-subscriber API is the Layer trait, which provides a composable abstraction for building Subscribers. Like the Subscriber trait, a Layer defines a particular behavior for collecting trace data. Unlike Subscribers, which implement a complete strategy for how trace data is collected, Layers provide modular implementations of specific behaviors. Therefore, they can be composed together to form a Subscriber which is capable of recording traces in a variety of ways. See the layer module’s documentation for details on using Layers.

In addition, the Filter trait defines an interface for filtering what spans and events are recorded by a particular layer. This allows different Layers to handle separate subsets of the trace data emitted by a program. See the documentation on per-layer filtering for more information on using Filters.

The following Subscribers are provided for application authors:

In embedded systems and other bare-metal applications, tracing can be used without requiring the Rust standard library, although some features are disabled. Although most of the APIs provided by tracing-subscriber, such as fmt and EnvFilter, require the standard library, some functionality, such as the Layer trait, can still be used in no_std environments.

The dependency on the standard library is controlled by two crate feature flags, “std”, which enables the dependency on libstd, and “alloc”, which enables the dependency on liballoc (and is enabled by the “std” feature). These features are enabled by default, but no_std users can disable them using:

Additional APIs are available when liballoc is available. To enable liballoc but not std, use:

These feature flags enable unstable features. The public API may break in 0.1.x releases. To enable these features, the --cfg tracing_unstable must be passed to rustc when compiling.

The f

*[Content truncated]*

**Examples:**

Example 1 (toml):
```toml
# Cargo.toml
tracing-subscriber = { version = "0.3", default-features = false }
```

Example 2 (toml):
```toml
# Cargo.toml
tracing-subscriber = { version = "0.3", default-features = false, features = ["alloc"] }
```

Example 3 (shell):
```shell
RUSTFLAGS="--cfg tracing_unstable" cargo build
```

Example 4 (toml):
```toml
[build]
rustflags = ["--cfg", "tracing_unstable"]
```

---

## Struct HashSet Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/platform/collections/struct.HashSet.html

**Contents:**
- Struct HashSet Copy item path
- Implementations§
  - impl<T> HashSet<T>
    - pub const fn new() -> HashSet<T>
      - §Examples
    - pub fn with_capacity(capacity: usize) -> HashSet<T>
      - §Examples
  - impl<T, S> HashSet<T, S>
    - pub fn capacity(&self) -> usize
      - §Examples

New-type for HashSet with FixedHasher as the default hashing provider. Can be trivially converted to and from a hashbrown HashSet using From.

A new-type is used instead of a type alias due to critical methods like new being incompatible with Bevy’s choice of default hasher.

Unlike hashbrown::HashSet, HashSet defaults to FixedHasher instead of RandomState. This provides determinism by default with an acceptable compromise to denial of service resistance in the context of a game engine.

Creates an empty HashSet.

Refer to new for further details.

Creates an empty HashSet with the specified capacity.

Refer to with_capacity for further details.

Returns the number of elements the set can hold without reallocating.

Refer to capacity for further details.

An iterator visiting all elements in arbitrary order. The iterator element type is &'a T.

Refer to iter for further details.

Returns the number of elements in the set.

Refer to len for further details.

Returns true if the set contains no elements.

Refer to is_empty for further details.

Clears the set, returning all elements in an iterator.

Refer to drain for further details.

Retains only the elements specified by the predicate.

Refer to retain for further details.

Drains elements which are true under the given predicate, and returns an iterator over the removed items.

Refer to extract_if for further details.

Clears the set, removing all values.

Refer to clear for further details.

Creates a new empty hash set which will use the given hasher to hash keys.

Refer to with_hasher for further details.

Creates an empty HashSet with the specified capacity, using hasher to hash the keys.

Refer to with_capacity_and_hasher for further details.

Returns a reference to the set’s BuildHasher.

Refer to hasher for further details.

Takes the inner HashSet out of this wrapper.

Reserves capacity for at least additional more elements to be inserted in the HashSet. The collection may reserve more space to avoid frequent reallocations.

Refer to reserve for further details.

Tries to reserve capacity for at least additional more elements to be inserted in the given HashSet<K,V>. The collection may reserve more space to avoid frequent reallocations.

Refer to try_reserve for further details.

Shrinks the capacity of the set as much as possible. It will drop down as much as possible while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.

Refer to shrink_to_fi

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct HashSet<T, S = FixedHasher>(/* private fields */);
```

Example 2 (javascript):
```javascript
// Creates a HashSet with zero capacity.
let map = HashSet::new();
```

Example 3 (javascript):
```javascript
// Creates a HashSet with capacity for at least 5 entries.
let map = HashSet::with_capacity(5);
```

Example 4 (javascript):
```javascript
let map = HashSet::with_capacity(5);

assert!(map.capacity() >= 5);
```

---

## Struct NotShadowReceiver Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.NotShadowReceiver.html

**Contents:**
- Struct NotShadowReceiver Copy item path
- Trait Implementations§
  - impl Component for NotShadowReceiverwhere NotShadowReceiver: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior
    - fn on_add() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_insert() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_replace() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>

Add this component to make a Mesh3d not receive shadows.

Note: If you’re using diffuse transmission, setting NotShadowReceiver will cause both “regular” shadows as well as diffusely transmitted shadows to be disabled, even when TransmittedShadowReceiver is being used.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct NotShadowReceiver;
```

---

## Struct GpuClusterableObjectsStorage Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.GpuClusterableObjectsStorage.html

**Contents:**
- Struct GpuClusterableObjectsStorage Copy item path
- Trait Implementations§
  - impl CalculateSizeFor for GpuClusterableObjectsStoragewhere GpuClusterableObjectsStorage: ShaderType<ExtraMetadata = StructMetadata<1>>, Vec<GpuClusterableObject>: CalculateSizeFor,
    - fn calculate_size_for(nr_of_el: u64) -> NonZero<u64>
  - impl CreateFrom for GpuClusterableObjectsStoragewhere GpuClusterableObjectsStorage: ShaderType<ExtraMetadata = StructMetadata<1>>, Vec<GpuClusterableObject>: for<'__> CreateFrom,
    - fn create_from<B>(reader: &mut Reader<B>) -> GpuClusterableObjectsStoragewhere B: BufferRef,
  - impl Default for GpuClusterableObjectsStorage
    - fn default() -> GpuClusterableObjectsStorage
  - impl ReadFrom for GpuClusterableObjectsStoragewhere GpuClusterableObjectsStorage: ShaderType<ExtraMetadata = StructMetadata<1>>, Vec<GpuClusterableObject>: for<'__> ReadFrom,
    - fn read_from<B>(&mut self, reader: &mut Reader<B>)where B: BufferRef,

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GpuClusterableObjectsStorage { /* private fields */ }
```

---

## Struct NotShadowCaster Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.NotShadowCaster.html

**Contents:**
- Struct NotShadowCaster Copy item path
- Trait Implementations§
  - impl Clone for NotShadowCaster
    - fn clone(&self) -> NotShadowCaster
    - fn clone_from(&mut self, source: &Self)
  - impl Component for NotShadowCasterwhere NotShadowCaster: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior

Add this component to make a Mesh3d not cast shadows.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct NotShadowCaster;
```

---

## Enum ImageType Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.ImageType.html

**Contents:**
- Enum ImageType Copy item path
- Variants§
  - MimeType(&'a str)
  - Extension(&'a str)
  - Format(ImageFormat)
- Implementations§
  - impl<'a> ImageType<'a>
    - pub fn to_image_format(&self) -> Result<ImageFormat, TextureError>
- Trait Implementations§
  - impl<'a> Debug for ImageType<'a>

The type of a raw image buffer.

The mime type of an image, for example "image/png".

The extension of an image file, for example "png".

The direct format of the image

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ImageType<'a> {
    MimeType(&'a str),
    Extension(&'a str),
    Format(ImageFormat),
}
```

---

## Struct Srgba Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/struct.Srgba.html

**Contents:**
- Struct Srgba Copy item path
- §Conversion
- Fields§
- Implementations§
  - impl Srgba
    - pub const BLACK: Srgba
    - pub const NONE: Srgba
    - pub const WHITE: Srgba
    - pub const RED: Srgba
    - pub const GREEN: Srgba

Non-linear standard RGB with alpha.

Conversion between the various color spaces is achieved using Rust’s native From trait. Because certain color spaces are defined by their transformation to and from another space, these From implementations reflect that set of definitions.

For example, the sRGB space is defined by its relationship with Linear RGB, and HWB by its with sRGB. As such, it is the responsibility of sRGB to provide From implementations for Linear RGB, and HWB for sRGB. To then provide conversion between Linear RGB and HWB directly, HWB is responsible for implementing these conversions, delegating to sRGB as an intermediatory. This ensures that all conversions take the shortest path between any two spaces, and limit the proliferation of domain specific knowledge for each color space to their respective definitions.

The red channel. [0.0, 1.0]

The green channel. [0.0, 1.0]

The blue channel. [0.0, 1.0]

The alpha channel. [0.0, 1.0]

A fully red color with full alpha.

A fully green color with full alpha.

A fully blue color with full alpha.

Construct a new Srgba color from components.

Construct a new Srgba color from (r, g, b) components, with the default alpha (1.0).

Return a copy of this color with the red channel set to the given value.

Return a copy of this color with the green channel set to the given value.

Return a copy of this color with the blue channel set to the given value.

New Srgba from a CSS-style hexadecimal string.

Convert this color to CSS-style hexadecimal notation.

New Srgba from sRGB colorspace.

See also Srgba::new, Srgba::rgba_u8, Srgba::hex.

New Srgba from sRGB colorspace.

See also Srgba::new, Srgba::rgb_u8, Srgba::hex.

Converts a non-linear sRGB value to a linear one via gamma correction.

Converts a linear sRGB value to a non-linear one via gamma correction.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Srgba {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}
```

Example 2 (javascript):
```javascript
let color = Srgba::rgb(0.5, 0.5, 0.5);

// Using From explicitly
let linear_color = LinearRgba::from(color);

// Using Into
let linear_color: LinearRgba = color.into();
```

Example 3 (javascript):
```javascript
201    pub const GRAY1: Srgba = Srgba::new(0.224, 0.224, 0.243, 1.0);
202    pub const GRAY2: Srgba = Srgba::new(0.486, 0.486, 0.529, 1.0);
203    pub const GRAY3: Srgba = Srgba::new(1.0, 1.0, 1.0, 1.0);
```

Example 4 (javascript):
```javascript
let color = Srgba::hex("FF00FF").unwrap(); // fuchsia
let color = Srgba::hex("FF00FF7F").unwrap(); // partially transparent fuchsia

// A standard hex color notation is also available
assert_eq!(Srgba::hex("#FFFFFF").unwrap(), Srgba::new(1.0, 1.0, 1.0, 1.0));
```

---

## Enum ImageCompareFunction Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.ImageCompareFunction.html

**Contents:**
- Enum ImageCompareFunction Copy item path
- Variants§
  - Never
  - Less
  - Equal
  - LessEqual
  - Greater
  - NotEqual
  - GreaterEqual
  - Always

Comparison function used for depth and stencil operations.

This type mirrors CompareFunction.

Function never passes

Function passes if new value less than existing value

Function passes if new value is equal to existing value. When using this compare function, make sure to mark your Vertex Shader’s @builtin(position) output as @invariant to prevent artifacting.

Function passes if new value is less than or equal to existing value

Function passes if new value is greater than existing value

Function passes if new value is not equal to existing value. When using this compare function, make sure to mark your Vertex Shader’s @builtin(position) output as @invariant to prevent artifacting.

Function passes if new value is greater than or equal to existing value

Function always passes

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ImageCompareFunction {
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}
```

---

## Struct LightProbesUniform Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.LightProbesUniform.html

**Contents:**
- Struct LightProbesUniform Copy item path
- Trait Implementations§
  - impl CreateFrom for LightProbesUniformwhere LightProbesUniform: ShaderType<ExtraMetadata = StructMetadata<8>>, [RenderLightProbe; 8]: for<'__> CreateFrom + for<'__> CreateFrom, i32: for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom, u32: for<'__> CreateFrom + for<'__> CreateFrom, f32: for<'__> CreateFrom,
    - fn create_from<B>(reader: &mut Reader<B>) -> LightProbesUniformwhere B: BufferRef,
  - impl Default for LightProbesUniform
    - fn default() -> LightProbesUniform
  - impl ReadFrom for LightProbesUniformwhere LightProbesUniform: ShaderType<ExtraMetadata = StructMetadata<8>>, [RenderLightProbe; 8]: for<'__> ReadFrom + for<'__> ReadFrom, i32: for<'__> ReadFrom + for<'__> ReadFrom + for<'__> ReadFrom, u32: for<'__> ReadFrom + for<'__> ReadFrom, f32: for<'__> ReadFrom,
    - fn read_from<B>(&mut self, reader: &mut Reader<B>)where B: BufferRef,
  - impl ShaderSize for LightProbesUniformwhere [RenderLightProbe; 8]: ShaderSize, i32: ShaderSize, u32: ShaderSize, f32: ShaderSize,
    - const SHADER_SIZE: NonZero<u64> = _

A per-view shader uniform that specifies all the light probes that the view takes into account.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct LightProbesUniform { /* private fields */ }
```

---

## Struct Oklcha Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/struct.Oklcha.html

**Contents:**
- Struct Oklcha Copy item path
- §Conversion
- Fields§
- Implementations§
  - impl Oklcha
    - pub const fn new(lightness: f32, chroma: f32, hue: f32, alpha: f32) -> Oklcha
      - §Arguments
    - pub const fn lch(lightness: f32, chroma: f32, hue: f32) -> Oklcha
      - §Arguments
    - pub const fn with_lightness(self, lightness: f32) -> Oklcha

Color in Oklch color space, with alpha

Conversion between the various color spaces is achieved using Rust’s native From trait. Because certain color spaces are defined by their transformation to and from another space, these From implementations reflect that set of definitions.

For example, the sRGB space is defined by its relationship with Linear RGB, and HWB by its with sRGB. As such, it is the responsibility of sRGB to provide From implementations for Linear RGB, and HWB for sRGB. To then provide conversion between Linear RGB and HWB directly, HWB is responsible for implementing these conversions, delegating to sRGB as an intermediatory. This ensures that all conversions take the shortest path between any two spaces, and limit the proliferation of domain specific knowledge for each color space to their respective definitions.

The ‘lightness’ channel. [0.0, 1.0]

The ‘chroma’ channel. [0.0, 1.0]

The ‘hue’ channel. [0.0, 360.0]

The alpha channel. [0.0, 1.0]

Construct a new Oklcha color from components.

Construct a new Oklcha color from (l, c, h) components, with the default alpha (1.0).

Return a copy of this color with the ‘lightness’ channel set to the given value.

Return a copy of this color with the ‘chroma’ channel set to the given value.

Generate a deterministic but quasi-randomly distributed color from a provided index.

This can be helpful for generating debug colors.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Oklcha {
    pub lightness: f32,
    pub chroma: f32,
    pub hue: f32,
    pub alpha: f32,
}
```

Example 2 (javascript):
```javascript
let color = Srgba::rgb(0.5, 0.5, 0.5);

// Using From explicitly
let linear_color = LinearRgba::from(color);

// Using Into
let linear_color: LinearRgba = color.into();
```

Example 3 (javascript):
```javascript
// Unique color for an entity
// let entity_index = entity.index();
let color = Oklcha::sequential_dispersed(entity_index);

// Palette with 5 distinct hues
let palette = (0..5).map(Oklcha::sequential_dispersed).collect::<Vec<_>>();
```

---

## Macro warn_span Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/macro.warn_span.html

**Contents:**
- Macro warn_span Copy item path
- §Examples

Constructs a span at the warn level.

Fields and attributes are set using the same syntax as the span! macro.

See the top-level documentation for details on the syntax accepted by this macro.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! warn_span {
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
warn_span!("my_span");
// is equivalent to:
span!(Level::WARN, "my_span");
```

Example 3 (javascript):
```javascript
use tracing::warn_span;
let span = warn_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
```

---

## Macro info_span Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/macro.info_span.html

**Contents:**
- Macro info_span Copy item path
- §Examples

Constructs a span at the info level.

Fields and attributes are set using the same syntax as the span! macro.

See the top-level documentation for details on the syntax accepted by this macro.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! info_span {
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
info_span!("my_span");
// is equivalent to:
span!(Level::INFO, "my_span");
```

Example 3 (javascript):
```javascript
let span = info_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
```

---

## Struct CompressedImageFormatSupport Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.CompressedImageFormatSupport.html

**Contents:**
- Struct CompressedImageFormatSupport Copy item path
- Tuple Fields§
- Trait Implementations§
  - impl Resource for CompressedImageFormatSupportwhere CompressedImageFormatSupport: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for CompressedImageFormatSupport
  - impl RefUnwindSafe for CompressedImageFormatSupport
  - impl Send for CompressedImageFormatSupport
  - impl Sync for CompressedImageFormatSupport
  - impl Unpin for CompressedImageFormatSupport

For defining which compressed image formats are supported. This will be initialized from available device features in finish() of the bevy RenderPlugin, but is left for the user to specify if not using the RenderPlugin, or the WGPU backend.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CompressedImageFormatSupport(pub CompressedImageFormats);
```

---

## Enum ImageFormat Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.ImageFormat.html

**Contents:**
- Enum ImageFormat Copy item path
- Variants§
  - Basis
  - Bmp
  - Dds
  - Farbfeld
  - Gif
  - OpenExr
  - Hdr
  - Ico

Gets the file extensions for a given format.

Gets the MIME types for a given format.

If a format doesn’t have any dedicated MIME types, this list will be empty.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ImageFormat {
Show 16 variants    Basis,
    Bmp,
    Dds,
    Farbfeld,
    Gif,
    OpenExr,
    Hdr,
    Ico,
    Jpeg,
    Ktx2,
    Png,
    Pnm,
    Qoi,
    Tga,
    Tiff,
    WebP,
}
```

---

## Module cascade Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/cascade/index.html

**Contents:**
- Module cascade Copy item path
- Structs§
- Functions§

bevy::lightModule cascade Copy item pathSource Structs§CascadeCascadeShadowConfigControls how cascaded shadow mapping works. Prefer using CascadeShadowConfigBuilder to construct an instance.CascadeShadowConfigBuilderBuilder for CascadeShadowConfig.CascadesFunctions§build_directional_light_cascadesclear_directional_light_cascades

---

## Struct GpuFog Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.GpuFog.html

**Contents:**
- Struct GpuFog Copy item path
- Trait Implementations§
  - impl Clone for GpuFog
    - fn clone(&self) -> GpuFog
    - fn clone_from(&mut self, source: &Self)
  - impl CreateFrom for GpuFogwhere GpuFog: ShaderType<ExtraMetadata = StructMetadata<6>>, Vec4: for<'__> CreateFrom + for<'__> CreateFrom, Vec3: for<'__> CreateFrom + for<'__> CreateFrom, f32: for<'__> CreateFrom, u32: for<'__> CreateFrom,
    - fn create_from<B>(reader: &mut Reader<B>) -> GpuFogwhere B: BufferRef,
  - impl Debug for GpuFog
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for GpuFog

The GPU-side representation of the fog configuration that’s sent as a uniform to the shader

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GpuFog { /* private fields */ }
```

---

## Struct AtmosphereSettings Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.AtmosphereSettings.html

**Contents:**
- Struct AtmosphereSettings Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for AtmosphereSettings
    - fn clone(&self) -> AtmosphereSettings
    - fn clone_from(&mut self, source: &Self)
  - impl Component for AtmosphereSettingswhere AtmosphereSettings: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

This component controls the resolution of the atmosphere LUTs, and how many samples are used when computing them.

The transmittance LUT stores the transmittance from a point in the atmosphere to the outer edge of the atmosphere in any direction, parametrized by the point’s radius and the cosine of the zenith angle of the ray.

The multiscattering LUT stores the factor representing luminance scattered towards the camera with scattering order >2, parametrized by the point’s radius and the cosine of the zenith angle of the sun.

The sky-view lut is essentially the actual skybox, storing the light scattered towards the camera in every direction with a cubemap.

The aerial-view lut is a 3d LUT fit to the view frustum, which stores the luminance scattered towards the camera at each point (RGB channels), alongside the average transmittance to that point (A channel).

The size of the transmittance LUT

The size of the multiscattering LUT

The size of the sky-view LUT.

The size of the aerial-view LUT.

The number of points to sample along each ray when computing the transmittance LUT

The number of rays to sample when computing each pixel of the multiscattering LUT

The number of points to sample when integrating along each multiscattering ray

The number of points to sample along each ray when computing the sky-view LUT.

The number of points to sample for each slice along the z-axis of the aerial-view LUT.

The maximum distance from the camera to evaluate the aerial view LUT. The slices along the z-axis of the texture will be distributed linearly from the camera to this value.

A conversion factor between scene units and meters, used to ensure correctness at different length scales.

The number of points to sample for each fragment when the using ray marching to render the sky

The rendering method to use for the atmosphere.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AtmosphereSettings {Show 13 fields
    pub transmittance_lut_size: UVec2,
    pub multiscattering_lut_size: UVec2,
    pub sky_view_lut_size: UVec2,
    pub aerial_view_lut_size: UVec3,
    pub transmittance_lut_samples: u32,
    pub multiscattering_lut_dirs: u32,
    pub multiscattering_lut_samples: u32,
    pub sky_view_lut_samples: u32,
    pub aerial_view_lut_samples: u32,
    pub aerial_view_lut_max_distance: f32,
    pub scene_units_to_m: f32,
    pub sky_max_samples: u32,
    pub rendering_method: AtmosphereMode,
}
```

---

## Struct ViewSpecializationTicks Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ViewSpecializationTicks.html

**Contents:**
- Struct ViewSpecializationTicks Copy item path
- Methods from Deref<Target = HashMap<RetainedViewEntity, Tick>>§
    - pub fn hasher(&self) -> &S
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn keys(&self) -> Keys<'_, K, V> ⓘ
      - §Examples
      - Examples found in repository?
    - pub fn values(&self) -> Values<'_, K, V> ⓘ
      - §Examples

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
pub struct ViewSpecializationTicks(/* private fields */);
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

## Trait Layer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/tracing_subscriber/trait.Layer.html

**Contents:**
- Trait Layer Copy item path
- Provided Methods§
    - fn on_register_dispatch(&self, subscriber: &Dispatch)
        - §Avoiding Memory Leaks
    - fn on_layer(&mut self, subscriber: &mut S)
    - fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest
    - fn enabled(&self, metadata: &Metadata<'_>, ctx: Context<'_, S>) -> bool
    - fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>)
    - fn on_record(&self, _span: &Id, _values: &Record<'_>, _ctx: Context<'_, S>)
    - fn on_follows_from(&self, _span: &Id, _follows: &Id, _ctx: Context<'_, S>)

A composable handler for tracing events.

A Layer implements a behavior for recording or collecting traces that can be composed together with other Layers to build a Subscriber. See the module-level documentation for details.

Performs late initialization when installing this layer as a Subscriber.

Layers should not store the Dispatch pointing to the Subscriber that they are a part of. Because the Dispatch owns the Subscriber, storing the Dispatch within the Subscriber will create a reference count cycle, preventing the Dispatch from ever being dropped.

Instead, when it is necessary to store a cyclical reference to the Dispatch within a Layer, use Dispatch::downgrade to convert a Dispatch into a WeakDispatch. This type is analogous to std::sync::Weak, and does not create a reference count cycle. A WeakDispatch can be stored within a subscriber without causing a memory leak, and can be upgraded into a Dispatch temporarily when the Dispatch must be accessed by the subscriber.

Performs late initialization when attaching a Layer to a Subscriber.

This is a callback that is called when the Layer is added to a Subscriber (e.g. in Layer::with_subscriber and SubscriberExt::with). Since this can only occur before the Subscriber has been set as the default, both the Layer and Subscriber are passed to this method mutably. This gives the Layer the opportunity to set any of its own fields with values received by method calls on the Subscriber.

For example, Filtered layers implement on_layer to call the Subscriber’s register_filter method, and store the returned FilterId as a field.

Note In most cases, Layer implementations will not need to implement this method. However, in cases where a type implementing Layer wraps one or more other types that implement Layer, like the Layered and Filtered types in this crate, that type MUST ensure that the inner Layers’ on_layer methods are called. Otherwise, functionality that relies on on_layer, such as per-layer filtering, may not work correctly.

Registers a new callsite with this layer, returning whether or not the layer is interested in being notified about the callsite, similarly to Subscriber::register_callsite.

By default, this returns Interest::always() if self.enabled returns true, or Interest::never() if it returns false.

See the trait-level documentation for more information on filtering with Layers.

Layers may also implement this method to perform any behaviour that should be run once per callsite. If the layer

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub trait Layer<S>: 'staticwhere
    S: Subscriber,{
Show 17 methods    // Provided methods
    fn on_register_dispatch(&self, subscriber: &Dispatch) { ... }
    fn on_layer(&mut self, subscriber: &mut S) { ... }
    fn register_callsite(
        &self,
        metadata: &'static Metadata<'static>,
    ) -> Interest { ... }
    fn enabled(&self, metadata: &Metadata<'_>, ctx: Context<'_, S>) -> bool { ... }
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) { ... }
    fn on_record(&self, _span: &Id, _values: &Record<'_>, _ctx: Context<'_, S>) { ... }
    fn on_foll
...
```

Example 2 (unknown):
```unknown
Layer::enabled
```

Example 3 (unknown):
```unknown
Layer::register_callsite
```

Example 4 (javascript):
```javascript
pub struct FooLayer {
    // ...
}

pub struct BarLayer {
    // ...
}

pub struct MySubscriber {
    // ...
}

impl<S: Subscriber> Layer<S> for FooLayer {
    // ...
}

impl<S: Subscriber> Layer<S> for BarLayer {
    // ...
}

let subscriber = FooLayer::new()
    .and_then(BarLayer::new())
    .with_subscriber(MySubscriber::new());
```

---

## Struct MorphIndices Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MorphIndices.html

**Contents:**
- Struct MorphIndices Copy item path
- Fields§
- Trait Implementations§
  - impl Default for MorphIndices
    - fn default() -> MorphIndices
  - impl Resource for MorphIndiceswhere MorphIndices: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for MorphIndices
  - impl RefUnwindSafe for MorphIndices
  - impl Send for MorphIndices

Maps each mesh affected by morph targets to the applicable offset within the MorphUniforms buffer.

We store both the current frame’s mapping and the previous frame’s mapping for the purposes of motion vector calculation.

Maps each entity with a morphed mesh to the appropriate offset within MorphUniforms::current_buffer.

Maps each entity with a morphed mesh to the appropriate offset within MorphUniforms::prev_buffer.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MorphIndices {
    pub current: HashMap<MainEntity, MorphIndex, EntityHash>,
    pub prev: HashMap<MainEntity, MorphIndex, EntityHash>,
}
```

---

## Struct LightMeta Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.LightMeta.html

**Contents:**
- Struct LightMeta Copy item path
- Fields§
- Trait Implementations§
  - impl Default for LightMeta
    - fn default() -> LightMeta
  - impl Resource for LightMetawhere LightMeta: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for LightMeta
  - impl !RefUnwindSafe for LightMeta
  - impl Send for LightMeta

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct LightMeta {
    pub view_gpu_lights: DynamicUniformBuffer<GpuLights>,
}
```

---

## Struct ShadowBinKey Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ShadowBinKey.html

**Contents:**
- Struct ShadowBinKey Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for ShadowBinKey
    - fn clone(&self) -> ShadowBinKey
    - fn clone_from(&mut self, source: &Self)
  - impl Hash for ShadowBinKey
    - fn hash<__H>(&self, state: &mut __H)where __H: Hasher,
    - fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,
  - impl Ord for ShadowBinKey

Data used to bin each object in the shadow map phase.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ShadowBinKey {
    pub asset_id: UntypedAssetId,
}
```

---

## Struct ViewPrepassSpecializationTicks Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ViewPrepassSpecializationTicks.html

**Contents:**
- Struct ViewPrepassSpecializationTicks Copy item path
- Methods from Deref<Target = HashMap<RetainedViewEntity, Tick>>§
    - pub fn hasher(&self) -> &S
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn keys(&self) -> Keys<'_, K, V> ⓘ
      - §Examples
      - Examples found in repository?
    - pub fn values(&self) -> Values<'_, K, V> ⓘ
      - §Examples

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
pub struct ViewPrepassSpecializationTicks(/* private fields */);
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

## Struct FunctionRegistry Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/reflect/func/struct.FunctionRegistry.html

**Contents:**
- Struct FunctionRegistry Copy item path
- Implementations§
  - impl FunctionRegistry
    - pub fn register<F, Marker>( &mut self, function: F, ) -> Result<&mut FunctionRegistry, FunctionRegistrationError>where F: IntoFunction<'static, Marker> + 'static,
      - §Examples
    - pub fn register_with_name<F, Marker>( &mut self, name: impl Into<Cow<'static, str>>, function: F, ) -> Result<&mut FunctionRegistry, FunctionRegistrationError>where F: IntoFunction<'static, Marker> + 'static,
      - §Examples
    - pub fn overwrite_registration<F, Marker>( &mut self, function: F, ) -> Result<Option<DynamicFunction<'static>>, FunctionRegistrationError>where F: IntoFunction<'static, Marker> + 'static,
    - pub fn overwrite_registration_with_name<F, Marker>( &mut self, name: impl Into<Cow<'static, str>>, function: F, ) -> Option<DynamicFunction<'static>>where F: IntoFunction<'static, Marker> + 'static,
    - pub fn call<'a>( &self, name: &str, args: ArgList<'a>, ) -> Option<Result<Return<'a>, FunctionError>>

A registry of reflected functions.

This is the function-equivalent to the TypeRegistry.

All functions must be 'static as they are stored as DynamicFunction<'static>.

Attempts to register the given function.

This function accepts both functions that satisfy IntoFunction and direct DynamicFunction instances. The given function will internally be stored as a DynamicFunction<'static> and mapped according to its name.

Because the function must have a name, anonymous functions (e.g. |a: i32, b: i32| { a + b }) and closures must instead be registered using register_with_name or manually converted to a DynamicFunction and named using DynamicFunction::with_name. Failure to do so will result in an error being returned.

If a registered function with the same name already exists, it will not be registered again and an error will be returned. To register the function anyway, overwriting any existing registration, use overwrite_registration instead.

Functions cannot be registered more than once.

Anonymous functions and closures should be registered using register_with_name or given a name using DynamicFunction::with_name.

Attempts to register the given function with the given name.

This function accepts both functions that satisfy IntoFunction and direct DynamicFunction instances. The given function will internally be stored as a DynamicFunction<'static> with its name set to the given name.

For named functions (e.g. fn add(a: i32, b: i32) -> i32 { a + b }) where a custom name is not needed, it’s recommended to use register instead as the generated name is guaranteed to be unique.

If a registered function with the same name already exists, it will not be registered again and an error will be returned. To register the function anyway, overwriting any existing registration, use overwrite_registration_with_name instead.

To avoid conflicts, it’s recommended to use a unique name for the function. This can be achieved by “namespacing” the function with a unique identifier, such as the name of your crate.

For example, to register a function, add, from a crate, my_crate, you could use the name, "my_crate::add".

Another approach could be to use the type name of the function, however, it should be noted that anonymous functions and closures are not guaranteed to have unique type names.

This method is a convenience around calling IntoFunction::into_function and DynamicFunction::with_name on the function and inserting it into the registry using the register method.



*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct FunctionRegistry { /* private fields */ }
```

Example 2 (javascript):
```javascript
fn add(a: i32, b: i32) -> i32 {
    a + b
}

let mut registry = FunctionRegistry::default();
registry.register(add)?;
```

Example 3 (javascript):
```javascript
fn add(a: i32, b: i32) -> i32 {
    a + b
}

let mut registry = FunctionRegistry::default();
registry.register(add).unwrap();

let result = registry.register(add);
assert!(matches!(result, Err(FunctionRegistrationError::DuplicateName(_))));

// Note that this simply relies on the name of the function to determine uniqueness.
// You can rename the function to register a separate instance of it.
let result = registry.register(add.into_function().with_name("add2"));
assert!(result.is_ok());
```

Example 4 (javascript):
```javascript
let anonymous = || -> i32 { 123 };

let mut registry = FunctionRegistry::default();

let result = registry.register(|a: i32, b: i32| a + b);
assert!(matches!(result, Err(FunctionRegistrationError::MissingName)));

let result = registry.register_with_name("my_crate::add", |a: i32, b: i32| a + b);
assert!(result.is_ok());

let result = registry.register((|a: i32, b: i32| a * b).into_function().with_name("my_crate::mul"));
assert!(result.is_ok());
```

---

## Struct SetPrepassViewEmptyBindGroup Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.SetPrepassViewEmptyBindGroup.html

**Contents:**
- Struct SetPrepassViewEmptyBindGroup Copy item path
- Trait Implementations§
  - impl<P, const I: usize> RenderCommand<P> for SetPrepassViewEmptyBindGroup<I>where P: PhaseItem,
    - type Param = Res<'static, PrepassViewBindGroup>
    - type ViewQuery = ()
    - type ItemQuery = ()
    - fn render<'w>( _item: &P, _view: (), _entity: Option<()>, prepass_view_bind_group: <<SetPrepassViewEmptyBindGroup<I> as RenderCommand<P>>::Param as SystemParam>::Item<'w, '_>, pass: &mut TrackedRenderPass<'w>, ) -> RenderCommandResult
- Auto Trait Implementations§
  - impl<const I: usize> Freeze for SetPrepassViewEmptyBindGroup<I>
  - impl<const I: usize> RefUnwindSafe for SetPrepassViewEmptyBindGroup<I>

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (javascript):
```javascript
pub struct SetPrepassViewEmptyBindGroup<const I: usize>;
```

---

## Struct VolumetricFog Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.VolumetricFog.html

**Contents:**
- Struct VolumetricFog Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for VolumetricFog
    - fn clone(&self) -> VolumetricFog
    - fn clone_from(&mut self, source: &Self)
  - impl Component for VolumetricFogwhere VolumetricFog: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

When placed on a bevy_camera::Camera3d, enables volumetric fog and volumetric lighting, also known as light shafts or god rays.

Requires using WebGPU on Wasm builds.

Color of the ambient light.

This is separate from Bevy’s AmbientLight because an EnvironmentMapLight is still considered an ambient light for the purposes of volumetric fog. If you’re using a EnvironmentMapLight, for best results, this should be a good approximation of the average color of the environment map.

The brightness of the ambient light.

If there’s no EnvironmentMapLight, set this to 0.

The maximum distance to offset the ray origin randomly by, in meters.

This is intended for use with temporal antialiasing. It helps fog look less blocky by varying the start position of the ray, using interleaved gradient noise.

The number of raymarching steps to perform.

Higher values produce higher-quality results with less banding, but reduce performance.

The default value is 64.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct VolumetricFog {
    pub ambient_color: Color,
    pub ambient_intensity: f32,
    pub jitter: f32,
    pub step_count: u32,
}
```

---

## Struct CompressedImageSaver Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.CompressedImageSaver.html

**Contents:**
- Struct CompressedImageSaver Copy item path
- Trait Implementations§
  - impl AssetSaver for CompressedImageSaver
    - type Asset = Image
    - type Settings = ()
    - type OutputLoader = ImageLoader
    - type Error = CompressedImageSaverError
    - async fn save( &self, writer: &mut (dyn AsyncWrite + Unpin + Sync + Send + 'static), image: SavedAsset<'_, <CompressedImageSaver as AssetSaver>::Asset>, _settings: &<CompressedImageSaver as AssetSaver>::Settings, ) -> Result<ImageLoaderSettings, <CompressedImageSaver as AssetSaver>::Error>
- Auto Trait Implementations§
  - impl Freeze for CompressedImageSaver

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CompressedImageSaver;
```

---

## Constant MAX_JOINTS Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/constant.MAX_JOINTS.html

**Contents:**
- Constant MAX_JOINTS Copy item path

Maximum number of joints supported for skinned meshes.

It is used to allocate buffers. The correctness of the value depends on the GPU/platform. The current value is chosen because it is guaranteed to work everywhere. To allow for bigger values, a check must be made for the limits of the GPU at runtime, which would mean not using consts anymore.

**Examples:**

Example 1 (javascript):
```javascript
pub const MAX_JOINTS: usize = 256; // 256usize
```

---

## Macro trace_span Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/macro.trace_span.html

**Contents:**
- Macro trace_span Copy item path
- §Examples

Constructs a span at the trace level.

Fields and attributes are set using the same syntax as the span! macro.

See the top-level documentation for details on the syntax accepted by this macro.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! trace_span {
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
trace_span!("my_span");
// is equivalent to:
span!(Level::TRACE, "my_span");
```

Example 3 (javascript):
```javascript
let span = trace_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
```

---

## Macro error_span Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/macro.error_span.html

**Contents:**
- Macro error_span Copy item path
- §Examples

Constructs a span at the error level.

Fields and attributes are set using the same syntax as the span! macro.

See the top-level documentation for details on the syntax accepted by this macro.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! error_span {
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
error_span!("my_span");
// is equivalent to:
span!(Level::ERROR, "my_span");
```

Example 3 (javascript):
```javascript
let span = error_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
```

---

## Macro trace Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/macro.trace.html

**Contents:**
- Macro trace Copy item path
- §Examples

Constructs an event at the trace level.

This functions similarly to the event! macro. See the top-level documentation for details on the syntax accepted by this macro.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! trace {
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
use tracing::trace;
let pos = Position { x: 3.234, y: -1.223 };
let origin_dist = pos.dist(Position::ORIGIN);

trace!(position = ?pos, ?origin_dist);
trace!(
    target: "app_events",
    position = ?pos,
    "x is {} and y is {}",
    if pos.x >= 0.0 { "positive" } else { "negative" },
    if pos.y >= 0.0 { "positive" } else { "negative" }
);
trace!(name: "completed", position = ?pos);
```

---

## Struct ViewKeyCache Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ViewKeyCache.html

**Contents:**
- Struct ViewKeyCache Copy item path
- Methods from Deref<Target = HashMap<RetainedViewEntity, MeshPipelineKey>>§
    - pub fn hasher(&self) -> &S
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn keys(&self) -> Keys<'_, K, V> ⓘ
      - §Examples
      - Examples found in repository?
    - pub fn values(&self) -> Values<'_, K, V> ⓘ
      - §Examples

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
pub struct ViewKeyCache(/* private fields */);
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

## Struct ShadowView Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ShadowView.html

**Contents:**
- Struct ShadowView Copy item path
- Fields§
- Trait Implementations§
  - impl Component for ShadowViewwhere ShadowView: Send + Sync + 'static,
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
pub struct ShadowView {
    pub depth_attachment: DepthAttachment,
    pub pass_name: String,
}
```

---

## Struct AtmosphereEnvironmentMapLight Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.AtmosphereEnvironmentMapLight.html

**Contents:**
- Struct AtmosphereEnvironmentMapLight Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for AtmosphereEnvironmentMapLight
    - fn clone(&self) -> AtmosphereEnvironmentMapLight
    - fn clone_from(&mut self, source: &Self)
  - impl Component for AtmosphereEnvironmentMapLightwhere AtmosphereEnvironmentMapLight: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

Lets the atmosphere contribute environment lighting (reflections and ambient diffuse) to your scene.

Attach this to a Camera3d to light the entire view, or to a LightProbe to light only a specific region. Behind the scenes, this generates an environment map from the atmosphere for image-based lighting and inserts a corresponding GeneratedEnvironmentMapLight.

For HDRI-based lighting, use a preauthored EnvironmentMapLight or filter one at runtime with GeneratedEnvironmentMapLight.

Controls how bright the atmosphere’s environment lighting is. Increase this value to brighten reflections and ambient diffuse lighting.

The default is 1.0 so that the generated environment lighting matches the light intensity of the atmosphere in the scene.

Whether the diffuse contribution should affect meshes that already have lightmaps.

Cubemap resolution in pixels (must be a power-of-two).

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AtmosphereEnvironmentMapLight {
    pub intensity: f32,
    pub affects_lightmapped_mesh_diffuse: bool,
    pub size: UVec2,
}
```

---

## Struct ViewLightsUniformOffset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ViewLightsUniformOffset.html

**Contents:**
- Struct ViewLightsUniformOffset Copy item path
- Fields§
- Trait Implementations§
  - impl Component for ViewLightsUniformOffsetwhere ViewLightsUniformOffset: Send + Sync + 'static,
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
pub struct ViewLightsUniformOffset {
    pub offset: u32,
}
```

---

## Struct AsyncComputeTaskPool Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/tasks/struct.AsyncComputeTaskPool.html

**Contents:**
- Struct AsyncComputeTaskPool Copy item path
- Implementations§
  - impl AsyncComputeTaskPool
    - pub fn get_or_init( f: impl FnOnce() -> TaskPool, ) -> &'static AsyncComputeTaskPool
    - pub fn try_get() -> Option<&'static AsyncComputeTaskPool>
    - pub fn get() -> &'static AsyncComputeTaskPool
      - §Panics
      - Examples found in repository?
- Methods from Deref<Target = TaskPool>§
    - pub fn thread_num(&self) -> usize

A newtype for a task pool for CPU-intensive work that may span across multiple frames

See TaskPool documentation for details on Bevy tasks. Use ComputeTaskPool if the work must be complete before advancing to the next frame.

Gets the global AsyncComputeTaskPool instance, or initializes it with f.

Attempts to get the global AsyncComputeTaskPool instance, or returns None if it is not initialized.

Gets the global AsyncComputeTaskPool instance.

Panics if the global instance has not been initialized yet.

Return the number of threads owned by the task pool

Allows spawning non-'static futures on the thread pool. The function takes a callback, passing a scope object into it. The scope object provided to the callback can be used to spawn tasks. This function will await the completion of all tasks before returning.

This is similar to thread::scope and rayon::scope.

The Scope object takes two lifetimes: 'scope and 'env.

The 'scope lifetime represents the lifetime of the scope. That is the time during which the provided closure and tasks that are spawned into the scope are run.

The 'env lifetime represents the lifetime of whatever is borrowed by the scope. Thus this lifetime must outlive 'scope.

This allows passing an external executor to spawn tasks on. When you pass an external executor Scope::spawn_on_scope spawns is then run on the thread that ThreadExecutor is being ticked on. If None is passed the scope will use a ThreadExecutor that is ticked on the current thread.

When tick_task_pool_executor is set to true, the multithreaded task stealing executor is ticked on the scope thread. Disabling this can be useful when finishing the scope is latency sensitive. Pulling tasks from global executor can run tasks unrelated to the scope and delay when the scope returns.

See Self::scope for more details in general about how scopes work.

Spawns a static future onto the thread pool. The returned Task is a future that can be polled for the result. It can also be canceled and “detached”, allowing the task to continue running even if dropped. In any case, the pool will execute the task even without polling by the end-user.

If the provided future is non-Send, TaskPool::spawn_local should be used instead.

Spawns a static future on the thread-local async executor for the current thread. The task will run entirely on the thread the task was spawned on.

The returned Task is a future that can be polled for the result. It can also be canceled and “detached”, allowing 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct AsyncComputeTaskPool(/* private fields */);
```

Example 2 (javascript):
```javascript
144fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
145    let (barrier, guard) = AssetBarrier::new();
146    commands.insert_resource(OneHundredThings(std::array::from_fn(|i| match i % 5 {
147        0 => asset_server.load_acquire("models/GolfBall/GolfBall.glb", guard.clone()),
148        1 => asset_server.load_acquire("models/AlienCake/alien.glb", guard.clone()),
149        2 => asset_server.load_acquire("models/AlienCake/cakeBirthday.glb", guard.clone()),
150        3 => asset_server.load_acquire("models/FlightHelmet/FlightHelmet.gltf", guard.clone()),
151        4 
...
```

Example 3 (javascript):
```javascript
53fn spawn_tasks(mut commands: Commands) {
54    let thread_pool = AsyncComputeTaskPool::get();
55    for x in 0..NUM_CUBES {
56        for y in 0..NUM_CUBES {
57            for z in 0..NUM_CUBES {
58                // Spawn new task on the AsyncComputeTaskPool; the task will be
59                // executed in the background, and the Task future returned by
60                // spawn() can be used to poll for the result
61                let entity = commands.spawn_empty().id();
62                let task = thread_pool.spawn(async move {
63                    let duration = Duration::from_sec
...
```

Example 4 (javascript):
```javascript
use bevy_tasks::TaskPool;

let pool = TaskPool::new();
let mut x = 0;
let results = pool.scope(|s| {
    s.spawn(async {
        // you can borrow the spawner inside a task and spawn tasks from within the task
        s.spawn(async {
            // borrow x and mutate it.
            x = 2;
            // return a value from the task
            1
        });
        // return some other value from the first task
        0
    });
});

// The ordering of results is non-deterministic if you spawn from within tasks as above.
// If you're doing this, you'll have to write your code to not depend o
...
```

---

## Struct ComputeTaskPool Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/tasks/struct.ComputeTaskPool.html

**Contents:**
- Struct ComputeTaskPool Copy item path
- Implementations§
  - impl ComputeTaskPool
    - pub fn get_or_init(f: impl FnOnce() -> TaskPool) -> &'static ComputeTaskPool
    - pub fn try_get() -> Option<&'static ComputeTaskPool>
    - pub fn get() -> &'static ComputeTaskPool
      - §Panics
- Methods from Deref<Target = TaskPool>§
    - pub fn thread_num(&self) -> usize
    - pub fn scope<'env, F, T>(&self, f: F) -> Vec<T>where F: for<'scope> FnOnce(&'scope Scope<'scope, 'env, T>), T: Send + 'static,

A newtype for a task pool for CPU-intensive work that must be completed to deliver the next frame

See TaskPool documentation for details on Bevy tasks. AsyncComputeTaskPool should be preferred if the work does not have to be completed before the next frame.

Gets the global ComputeTaskPool instance, or initializes it with f.

Attempts to get the global ComputeTaskPool instance, or returns None if it is not initialized.

Gets the global ComputeTaskPool instance.

Panics if the global instance has not been initialized yet.

Return the number of threads owned by the task pool

Allows spawning non-'static futures on the thread pool. The function takes a callback, passing a scope object into it. The scope object provided to the callback can be used to spawn tasks. This function will await the completion of all tasks before returning.

This is similar to thread::scope and rayon::scope.

The Scope object takes two lifetimes: 'scope and 'env.

The 'scope lifetime represents the lifetime of the scope. That is the time during which the provided closure and tasks that are spawned into the scope are run.

The 'env lifetime represents the lifetime of whatever is borrowed by the scope. Thus this lifetime must outlive 'scope.

This allows passing an external executor to spawn tasks on. When you pass an external executor Scope::spawn_on_scope spawns is then run on the thread that ThreadExecutor is being ticked on. If None is passed the scope will use a ThreadExecutor that is ticked on the current thread.

When tick_task_pool_executor is set to true, the multithreaded task stealing executor is ticked on the scope thread. Disabling this can be useful when finishing the scope is latency sensitive. Pulling tasks from global executor can run tasks unrelated to the scope and delay when the scope returns.

See Self::scope for more details in general about how scopes work.

Spawns a static future onto the thread pool. The returned Task is a future that can be polled for the result. It can also be canceled and “detached”, allowing the task to continue running even if dropped. In any case, the pool will execute the task even without polling by the end-user.

If the provided future is non-Send, TaskPool::spawn_local should be used instead.

Spawns a static future on the thread-local async executor for the current thread. The task will run entirely on the thread the task was spawned on.

The returned Task is a future that can be polled for the result. It can also be canceled and “de

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct ComputeTaskPool(/* private fields */);
```

Example 2 (javascript):
```javascript
use bevy_tasks::TaskPool;

let pool = TaskPool::new();
let mut x = 0;
let results = pool.scope(|s| {
    s.spawn(async {
        // you can borrow the spawner inside a task and spawn tasks from within the task
        s.spawn(async {
            // borrow x and mutate it.
            x = 2;
            // return a value from the task
            1
        });
        // return some other value from the first task
        0
    });
});

// The ordering of results is non-deterministic if you spawn from within tasks as above.
// If you're doing this, you'll have to write your code to not depend o
...
```

Example 3 (javascript):
```javascript
use bevy_tasks::TaskPool;
fn scope_escapes_closure() {
    let pool = TaskPool::new();
    let foo = Box::new(42);
    pool.scope(|scope| {
        std::thread::spawn(move || {
            // UB. This could spawn on the scope after `.scope` returns and the internal Scope is dropped.
            scope.spawn(async move {
                assert_eq!(*foo, 42);
            });
        });
    });
}
```

Example 4 (javascript):
```javascript
use bevy_tasks::TaskPool;
fn cannot_borrow_from_closure() {
    let pool = TaskPool::new();
    pool.scope(|scope| {
        let x = 1;
        let y = &x;
        scope.spawn(async move {
            assert_eq!(*y, 1);
        });
    });
}
```

---

## Struct Atmosphere Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.Atmosphere.html

**Contents:**
- Struct Atmosphere Copy item path
- Fields§
- Implementations§
  - impl Atmosphere
    - pub const EARTH: Atmosphere
    - pub fn with_density_multiplier(self, mult: f32) -> Atmosphere
- Trait Implementations§
  - impl Clone for Atmosphere
    - fn clone(&self) -> Atmosphere
    - fn clone_from(&mut self, source: &Self)

This component describes the atmosphere of a planet, and when added to a camera will enable atmospheric scattering for that camera. This is only compatible with HDR cameras.

Most atmospheric particles scatter and absorb light in two main ways:

Rayleigh scattering occurs among very small particles, like individual gas molecules. It’s wavelength dependent, and causes colors to separate out as light travels through the atmosphere. These particles don’t absorb light.

Mie scattering occurs among slightly larger particles, like dust and sea spray. These particles do absorb light, but Mie scattering and absorption is wavelength independent.

Ozone acts differently from the other two, and is special-cased because it’s very important to the look of Earth’s atmosphere. It’s wavelength dependent, but only absorbs light. Also, while the density of particles participating in Rayleigh and Mie scattering falls off roughly exponentially from the planet’s surface, ozone only exists in a band centered at a fairly high altitude.

Radius at which we consider the atmosphere to ‘end’ for our calculations (from center of planet)

An approximation of the average albedo (or color, roughly) of the planet’s surface. This is used when calculating multiscattering.

The rate of falloff of rayleigh particulate with respect to altitude: optical density = exp(-rayleigh_density_exp_scale * altitude in meters).

THIS VALUE MUST BE POSITIVE

The scattering optical density of rayleigh particulate, or how much light it scatters per meter

The rate of falloff of mie particulate with respect to altitude: optical density = exp(-mie_density_exp_scale * altitude in meters)

THIS VALUE MUST BE POSITIVE

The scattering optical density of mie particulate, or how much light it scatters per meter.

The absorbing optical density of mie particulate, or how much light it absorbs per meter.

The “asymmetry” of mie scattering, or how much light tends to scatter forwards, rather than backwards or to the side.

domain: (-1, 1) units: N/A

The altitude at which the ozone layer is centered.

The width of the ozone layer

The optical density of ozone, or how much of each wavelength of light it absorbs per meter.

Required Components: AtmosphereSettings, Hdr.

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components of the required components, recursively, in depth-first order.

Returns the argument unchanged.

Creates Self using default().


*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Atmosphere {
    pub bottom_radius: f32,
    pub top_radius: f32,
    pub ground_albedo: Vec3,
    pub rayleigh_density_exp_scale: f32,
    pub rayleigh_scattering: Vec3,
    pub mie_density_exp_scale: f32,
    pub mie_scattering: f32,
    pub mie_absorption: f32,
    pub mie_asymmetry: f32,
    pub ozone_layer_altitude: f32,
    pub ozone_layer_width: f32,
    pub ozone_absorption: Vec3,
}
```

---

## Struct Error Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/tasks/futures_lite/io/struct.Error.html

**Contents:**
- Struct Error Copy item path
- Implementations§
  - impl Error
  - impl Error
    - pub fn new<E>(kind: ErrorKind, error: E) -> Errorwhere E: Into<Box<dyn Error + Sync + Send>>,
      - §Examples
    - pub fn other<E>(error: E) -> Errorwhere E: Into<Box<dyn Error + Sync + Send>>,
      - §Examples
    - pub fn last_os_error() -> Error
      - §Examples

The error type for I/O operations of the Read, Write, Seek, and associated traits.

Errors mostly originate from the underlying OS, but custom instances of Error can be created with crafted error messages and a particular value of ErrorKind.

Common errors constants for use in std

Creates a new I/O error from a known kind of error as well as an arbitrary error payload.

This function is used to generically create I/O errors which do not originate from the OS itself. The error argument is an arbitrary payload which will be contained in this Error.

Note that this function allocates memory on the heap. If no extra payload is required, use the From conversion from ErrorKind.

Creates a new I/O error from an arbitrary error payload.

This function is used to generically create I/O errors which do not originate from the OS itself. It is a shortcut for Error::new with ErrorKind::Other.

Returns an error representing the last OS error which occurred.

This function reads the value of errno for the target platform (e.g. GetLastError on Windows) and will return a corresponding instance of Error for the error code.

This should be called immediately after a call to a platform function, otherwise the state of the error value is indeterminate. In particular, other standard library functions may call platform functions that may (or may not) reset the error value even if they succeed.

Creates a new instance of an Error from a particular OS error code.

Returns the OS error that this error represents (if any).

If this Error was constructed via last_os_error or from_raw_os_error, then this function will return Some, otherwise it will return None.

Returns a reference to the inner error wrapped by this error (if any).

If this Error was constructed via new then this function will return Some, otherwise it will return None.

Returns a mutable reference to the inner error wrapped by this error (if any).

If this Error was constructed via new then this function will return Some, otherwise it will return None.

Consumes the Error, returning its inner error (if any).

If this Error was constructed via new or other, then this function will return Some, otherwise it will return None.

Attempts to downcast the custom boxed error to E.

If this Error contains a custom boxed error, then it would attempt downcasting on the boxed error, otherwise it will return Err.

If the custom boxed error has the same type as E, it will return Ok, otherwise it will also return Err.

This method

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Error { /* private fields */ }
```

Example 2 (javascript):
```javascript
use std::io::{Error, ErrorKind};

// errors can be created from strings
let custom_error = Error::new(ErrorKind::Other, "oh no!");

// errors can also be created from other errors
let custom_error2 = Error::new(ErrorKind::Interrupted, custom_error);

// creating an error without payload (and without memory allocation)
let eof_error = Error::from(ErrorKind::UnexpectedEof);
```

Example 3 (javascript):
```javascript
use std::io::Error;

// errors can be created from strings
let custom_error = Error::other("oh no!");

// errors can also be created from other errors
let custom_error2 = Error::other(custom_error);
```

Example 4 (javascript):
```javascript
use std::io::Error;

let os_error = Error::last_os_error();
println!("last OS error: {os_error:?}");
```

---

## Enum CompressedImageSaverError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.CompressedImageSaverError.html

**Contents:**
- Enum CompressedImageSaverError Copy item path
- Variants (Non-exhaustive)§
  - Io(Error)
  - UninitializedImage
- Trait Implementations§
  - impl Debug for CompressedImageSaverError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for CompressedImageSaverError
    - fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>
  - impl Error for CompressedImageSaverError

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[non_exhaustive]pub enum CompressedImageSaverError {
    Io(Error),
    UninitializedImage,
}
```

---

## Trait Mix Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/trait.Mix.html

**Contents:**
- Trait Mix Copy item path
- Required Methods§
    - fn mix(&self, other: &Self, factor: f32) -> Self
- Provided Methods§
    - fn mix_assign(&mut self, other: Self, factor: f32)
- Dyn Compatibility§
- Implementors§
  - impl Mix for Color
  - impl Mix for Hsla
  - impl Mix for Hsva

Linear interpolation of two colors within a given color space.

Linearly interpolate between this and another color, by factor. Factor should be between 0.0 and 1.0.

Linearly interpolate between this and another color, by factor, storing the result in this color. Factor should be between 0.0 and 1.0.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait Mix: Sized {
    // Required method
    fn mix(&self, other: &Self, factor: f32) -> Self;

    // Provided method
    fn mix_assign(&mut self, other: Self, factor: f32) { ... }
}
```

---

## Struct PassHash Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/platform/hash/struct.PassHash.html

**Contents:**
- Struct PassHash Copy item path
- Trait Implementations§
  - impl BuildHasher for PassHash
    - type Hasher = PassHasher
    - fn build_hasher(&self) -> <PassHash as BuildHasher>::Hasher
    - fn hash_one<T>(&self, x: T) -> u64where T: Hash, Self: Sized, Self::Hasher: Hasher,
  - impl Clone for PassHash
    - fn clone(&self) -> PassHash
    - fn clone_from(&mut self, source: &Self)
  - impl Default for PassHash

A BuildHasher that results in a PassHasher.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PassHash;
```

---

## Trait ColorRange Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/trait.ColorRange.html

**Contents:**
- Trait ColorRange Copy item path
- Required Methods§
    - fn at(&self, factor: f32) -> T
- Implementations on Foreign Types§
  - impl<T> ColorRange<T> for Range<T>where T: Mix,
    - fn at(&self, factor: f32) -> T
- Implementors§

Represents a range of colors that can be linearly interpolated, defined by a start and end point which must be in the same color space. It works for any color type that implements Mix.

This is useful for defining gradients or animated color transitions.

Get the color value at the given interpolation factor, which should be between 0.0 (start) and 1.0 (end).

**Examples:**

Example 1 (unknown):
```unknown
pub trait ColorRange<T>where
    T: Mix,{
    // Required method
    fn at(&self, factor: f32) -> T;
}
```

---

## Struct PrepassPipelineSpecializer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PrepassPipelineSpecializer.html

**Contents:**
- Struct PrepassPipelineSpecializer Copy item path
- Trait Implementations§
  - impl SpecializedMeshPipeline for PrepassPipelineSpecializer
    - type Key = ErasedMaterialPipelineKey
    - fn specialize( &self, key: <PrepassPipelineSpecializer as SpecializedMeshPipeline>::Key, layout: &MeshVertexBufferLayoutRef, ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError>
- Auto Trait Implementations§
  - impl Freeze for PrepassPipelineSpecializer
  - impl !RefUnwindSafe for PrepassPipelineSpecializer
  - impl Send for PrepassPipelineSpecializer
  - impl Sync for PrepassPipelineSpecializer

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PrepassPipelineSpecializer { /* private fields */ }
```

---

## Trait ConditionalSendFuture Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/tasks/trait.ConditionalSendFuture.html

**Contents:**
- Trait ConditionalSendFuture Copy item path
- Implementors§
  - impl<T> ConditionalSendFuture for Twhere T: Future + ConditionalSend,

Use ConditionalSendFuture for a future with an optional Send trait bound, as on certain platforms (eg. Wasm), futures aren’t Send.

**Examples:**

Example 1 (unknown):
```unknown
pub trait ConditionalSendFuture: Future + ConditionalSend { }
```

---

## Struct CascadeShadowConfig Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.CascadeShadowConfig.html

**Contents:**
- Struct CascadeShadowConfig Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for CascadeShadowConfig
    - fn clone(&self) -> CascadeShadowConfig
    - fn clone_from(&mut self, source: &Self)
  - impl Component for CascadeShadowConfigwhere CascadeShadowConfig: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

Controls how cascaded shadow mapping works. Prefer using CascadeShadowConfigBuilder to construct an instance.

The (positive) distance to the far boundary of each cascade.

The proportion of overlap each cascade has with the previous cascade.

The (positive) distance to the near boundary of the first cascade.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CascadeShadowConfig {
    pub bounds: Vec<f32>,
    pub overlap_proportion: f32,
    pub minimum_distance: f32,
}
```

Example 2 (javascript):
```javascript
let config: CascadeShadowConfig = CascadeShadowConfigBuilder {
  maximum_distance: 100.0,
  ..default()
}.into();
```

---

## Struct EnvironmentMapUniformBuffer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.EnvironmentMapUniformBuffer.html

**Contents:**
- Struct EnvironmentMapUniformBuffer Copy item path
- Tuple Fields§
- Methods from Deref<Target = DynamicUniformBuffer<EnvironmentMapUniform>>§
    - pub fn buffer(&self) -> Option<&Buffer>
    - pub fn binding(&self) -> Option<BindingResource<'_>>
      - Examples found in repository?
    - pub fn is_empty(&self) -> bool
    - pub fn push(&mut self, value: &T) -> u32
    - pub fn set_label(&mut self, label: Option<&str>)
    - pub fn get_label(&self) -> Option<&str>

A GPU buffer that stores the environment map settings for each view.

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
pub struct EnvironmentMapUniformBuffer(pub DynamicUniformBuffer<EnvironmentMapUniform>);
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

## Struct CompressedImageFormats Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.CompressedImageFormats.html

**Contents:**
- Struct CompressedImageFormats Copy item path
- Implementations§
  - impl CompressedImageFormats
    - pub const NONE: CompressedImageFormats
    - pub const ASTC_LDR: CompressedImageFormats
    - pub const BC: CompressedImageFormats
    - pub const ETC2: CompressedImageFormats
  - impl CompressedImageFormats
    - pub const fn empty() -> CompressedImageFormats
    - pub const fn all() -> CompressedImageFormats

Get a flags value with all bits unset.

Get a flags value with all known bits set.

Get the underlying bits value.

The returned value is exactly the bits set in this flags value.

Convert from a bits value.

This method will return None if any unknown bits are set.

Convert from a bits value, unsetting any unknown bits.

Convert from a bits value exactly.

Get a flags value with the bits of a flag with the given name set.

This method will return None if name is empty or doesn’t correspond to any named flag.

Whether all bits in this flags value are unset.

Whether all known bits in this flags value are set.

Whether any set bits in a source flags value are also set in a target flags value.

Whether all set bits in a source flags value are also set in a target flags value.

The bitwise or (|) of the bits in two flags values.

The intersection of a source flags value with the complement of a target flags value (&!).

This method is not equivalent to self & !other when other has unknown bits set. remove won’t truncate other, but the ! operator will.

The bitwise exclusive-or (^) of the bits in two flags values.

Call insert when value is true or remove when value is false.

The bitwise and (&) of the bits in two flags values.

The bitwise or (|) of the bits in two flags values.

The intersection of a source flags value with the complement of a target flags value (&!).

This method is not equivalent to self & !other when other has unknown bits set. difference won’t truncate other, but the ! operator will.

The bitwise exclusive-or (^) of the bits in two flags values.

The bitwise negation (!) of the bits in a flags value, truncating the result.

Yield a set of contained flags values.

Each yielded flags value will correspond to a defined named flag. Any unknown bits will be yielded together as a final flags value.

Yield a set of contained named flags values.

This method is like iter, except only yields bits in contained named flags. Any unknown bits, or bits not corresponding to a contained flag will not be yielded.

The bitwise and (&) of the bits in two flags values.

The bitwise and (&) of the bits in two flags values.

The bitwise or (|) of the bits in two flags values.

The bitwise or (|) of the bits in two flags values.

The bitwise exclusive-or (^) of the bits in two flags values.

The bitwise exclusive-or (^) of the bits in two flags values.

The bitwise or (|) of the bits in each flags value.

The bitwise or (|) of the bits in each flags value.

T

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct CompressedImageFormats(/* private fields */);
```

Example 2 (javascript):
```javascript
106fn cycle_cubemap_asset(
107    time: Res<Time>,
108    mut next_swap: Local<f32>,
109    mut cubemap: ResMut<Cubemap>,
110    asset_server: Res<AssetServer>,
111    render_device: Res<RenderDevice>,
112) {
113    let now = time.elapsed_secs();
114    if *next_swap == 0.0 {
115        *next_swap = now + CUBEMAP_SWAP_DELAY;
116        return;
117    } else if now < *next_swap {
118        return;
119    }
120    *next_swap += CUBEMAP_SWAP_DELAY;
121
122    let supported_compressed_formats =
123        CompressedImageFormats::from_features(render_device.features());
124
125    let mut new_inde
...
```

Example 3 (javascript):
```javascript
106fn cycle_cubemap_asset(
107    time: Res<Time>,
108    mut next_swap: Local<f32>,
109    mut cubemap: ResMut<Cubemap>,
110    asset_server: Res<AssetServer>,
111    render_device: Res<RenderDevice>,
112) {
113    let now = time.elapsed_secs();
114    if *next_swap == 0.0 {
115        *next_swap = now + CUBEMAP_SWAP_DELAY;
116        return;
117    } else if now < *next_swap {
118        return;
119    }
120    *next_swap += CUBEMAP_SWAP_DELAY;
121
122    let supported_compressed_formats =
123        CompressedImageFormats::from_features(render_device.features());
124
125    let mut new_inde
...
```

---

## Struct ViewShadowBindings Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ViewShadowBindings.html

**Contents:**
- Struct ViewShadowBindings Copy item path
- Fields§
- Trait Implementations§
  - impl Component for ViewShadowBindingswhere ViewShadowBindings: Send + Sync + 'static,
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
pub struct ViewShadowBindings {
    pub point_light_depth_texture: Texture,
    pub point_light_depth_texture_view: TextureView,
    pub directional_light_depth_texture: Texture,
    pub directional_light_depth_texture_view: TextureView,
}
```

---

## Struct Hashed Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/platform/hash/struct.Hashed.html

**Contents:**
- Struct Hashed Copy item path
- Implementations§
  - impl<V, H> Hashed<V, H>where V: Hash, H: BuildHasher + Default,
    - pub fn new(value: V) -> Hashed<V, H>
    - pub fn hash(&self) -> u64
- Trait Implementations§
  - impl<V, H> Clone for Hashed<V, H>where V: Clone,
    - fn clone(&self) -> Hashed<V, H>
    - fn clone_from(&mut self, source: &Self)
  - impl<V, H> Debug for Hashed<V, H>where V: Debug,

A pre-hashed value of a specific type. Pre-hashing enables memoization of hashes that are expensive to compute.

It also enables faster PartialEq comparisons by short circuiting on hash equality. See PassHash and PassHasher for a “pass through” BuildHasher and Hasher implementation designed to work with Hashed See PreHashMap for a hashmap pre-configured to use Hashed keys.

Pre-hashes the given value using the BuildHasher configured in the Hashed type.

The pre-computed hash.

A fast impl of PartialEq that first checks that other’s pre-computed hash matches this value’s pre-computed hash.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Hashed<V, S = FixedHasher> { /* private fields */ }
```

---

## Enum ImageSampler Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.ImageSampler.html

**Contents:**
- Enum ImageSampler Copy item path
- Variants§
  - Default
  - Descriptor(ImageSamplerDescriptor)
- Implementations§
  - impl ImageSampler
    - pub fn linear() -> ImageSampler
      - Examples found in repository?
    - pub fn nearest() -> ImageSampler
      - Examples found in repository?

Used in Image, this determines what image sampler to use when rendering. The default setting, ImageSampler::Default, will read the sampler from the ImagePlugin at setup. Setting this to ImageSampler::Descriptor will override the global default descriptor for this Image.

Default image sampler, derived from the ImagePlugin setup.

Custom sampler for this image which will override global default.

Returns an image sampler with ImageFilterMode::Linear min and mag filters

Returns an image sampler with ImageFilterMode::Nearest min and mag filters

Initialize the descriptor if it is not already initialized.

Descriptor is typically initialized by Bevy when the image is loaded, so this is convenient shortcut for updating the descriptor.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ImageSampler {
    Default,
    Descriptor(ImageSamplerDescriptor),
}
```

Example 2 (javascript):
```javascript
50fn setup(
51    mut commands: Commands,
52    rpg_sprite_handles: Res<RpgSpriteFolder>,
53    asset_server: Res<AssetServer>,
54    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
55    loaded_folders: Res<Assets<LoadedFolder>>,
56    mut textures: ResMut<Assets<Image>>,
57) {
58    let loaded_folder = loaded_folders.get(&rpg_sprite_handles.0).unwrap();
59
60    // Create texture atlases with different padding and sampling
61
62    let (texture_atlas_linear, linear_sources, linear_texture) = create_texture_atlas(
63        loaded_folder,
64        None,
65        Some(ImageSampler::
...
```

Example 3 (javascript):
```javascript
17fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
18    let image = asset_server.load_with_settings(
19        "textures/fantasy_ui_borders/numbered_slices.png",
20        |settings: &mut ImageLoaderSettings| {
21            // Need to use nearest filtering to avoid bleeding between the slices with tiling
22            settings.sampler = ImageSampler::nearest();
23        },
24    );
25
26    let slicer = TextureSlicer {
27        // `numbered_slices.png` is 48 pixels square. `BorderRect::square(16.)` insets the slicing line from each edge by 16 pixels, resulting in nine sli
...
```

Example 4 (unknown):
```unknown
21fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
22    // Without any .meta file specifying settings, the default sampler [ImagePlugin::default()] is used for loading images.
23    // If you are using a very small image and rendering it larger like seen here, the default linear filtering will result in a blurry image.
24    // Useful note: The default sampler specified by the ImagePlugin is *not* the same as the default implementation of sampler. This is why
25    // everything uses linear by default but if you look at the default of sampler, it uses nearest.
26    commands
...
```

---

## Struct Cascades Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.Cascades.html

**Contents:**
- Struct Cascades Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for Cascades
    - fn clone(&self) -> Cascades
    - fn clone_from(&mut self, source: &Self)
  - impl Component for Cascadeswhere Cascades: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

Map from a view to the configuration of each of its Cascades.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Cascades {
    pub cascades: EntityHashMap<Vec<Cascade>>,
}
```

---

## Module func Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/reflect/func/index.html

**Contents:**
- Module func Copy item path
- §Example
- §Types of Functions
- §Valid Signatures
- §Generic Functions
- §Overloading Functions
- §Function Registration
- Modules§
- Structs§
- Enums§

Reflection-based dynamic functions.

This module provides a way to pass around and call functions dynamically using the DynamicFunction and DynamicFunctionMut types.

Many simple functions and closures can be automatically converted to these types using the IntoFunction and IntoFunctionMut traits, respectively.

Once this dynamic representation is created, it can be called with a set of arguments provided via an ArgList.

This returns a FunctionResult containing the Return value, which can be used to extract a PartialReflect trait object.

For simplicity, this module uses the umbrella term “function” to refer to any Rust callable: code that can be invoked with a set of arguments to perform some action.

In Rust, there are two main categories of callables: functions and closures.

A “function” is a callable that does not capture its environment. These are typically defined with the fn keyword, which are referred to as named functions. But there are also anonymous functions, which are unnamed and defined with anonymous function syntax.

Closures, on the other hand, are special functions that do capture their environment. These are always defined with anonymous function syntax.

Many of the traits in this module have default blanket implementations over a specific set of function signatures.

These signatures are:

Where ... represents 0 to 15 arguments (inclusive) of the form T, &T, or &mut T. The lifetime of any reference to the return type R, must be tied to a “receiver” argument (i.e. the first argument in the signature, normally self).

Each trait will also have its own requirements for what traits are required for both arguments and return types, but a good rule-of-thumb is that all types should derive Reflect.

The reason for such a small subset of valid signatures is due to limitations in Rust— namely the lack of variadic generics and certain coherence issues.

For other functions that don’t conform to one of the above signatures, DynamicFunction and DynamicFunctionMut can instead be created manually.

In Rust, generic functions are monomorphized by the compiler, which means that a separate copy of the function is generated for each concrete set of type parameters.

When converting a generic function to a DynamicFunction or DynamicFunctionMut, the function must be manually monomorphized with concrete types. In other words, you cannot write add<T>.into_function(). Instead, you will need to write add::<i32>.into_function().

This means that reflected fu

*[Content truncated]*

**Examples:**

Example 1 (javascript):
```javascript
fn add(a: i32, b: i32) -> i32 {
  a + b
}

let mut func: DynamicFunction = add.into_function();
let args: ArgList = ArgList::default()
  // Pushing a known type with owned ownership
  .with_owned(25_i32)
  // Pushing a reflected type with owned ownership
  .with_boxed(Box::new(75_i32) as Box<dyn PartialReflect>);
let result: FunctionResult = func.call(args);
let value: Return = result.unwrap();
assert_eq!(value.unwrap_owned().try_downcast_ref::<i32>(), Some(&100));
```

Example 2 (javascript):
```javascript
// This is a named function:
fn add(a: i32, b: i32) -> i32 {
  a + b
}

// This is an anonymous function:
let add = |a: i32, b: i32| a + b;
```

Example 3 (javascript):
```javascript
// A closure that captures an immutable reference to a variable
let c = 123;
let add = |a: i32, b: i32| a + b + c;

// A closure that captures a mutable reference to a variable
let mut total = 0;
let add = |a: i32, b: i32| total += a + b;

// A closure that takes ownership of its captured variables by moving them
let c = 123;
let add = move |a: i32, b: i32| a + b + c;
```

Example 4 (javascript):
```javascript
fn add(a: i32, b: i32) -> i32 {
    a + b
}

let mut registry = FunctionRegistry::default();

// You can register functions and methods by their `core::any::type_name`:
registry.register(add).unwrap();

// Or you can register them by a custom name:
registry.register_with_name("mul", |a: i32, b: i32| a * b).unwrap();

// You can then retrieve and call these functions by name:
let reflect_add = registry.get(core::any::type_name_of_val(&add)).unwrap();
let value = reflect_add.call(ArgList::default().with_owned(10_i32).with_owned(5_i32)).unwrap();
assert_eq!(value.unwrap_owned().try_downcast_ref::
...
```

---

## Struct DirectionalLight Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.DirectionalLight.html

**Contents:**
- Struct DirectionalLight Copy item path
  - §Shadows
- Fields§
- Implementations§
  - impl DirectionalLight
    - pub const DEFAULT_SHADOW_DEPTH_BIAS: f32 = 0.0199999996f32
    - pub const DEFAULT_SHADOW_NORMAL_BIAS: f32 = 1.79999995f32
- Trait Implementations§
  - impl Clone for DirectionalLight
    - fn clone(&self) -> DirectionalLight

Directional lights don’t exist in reality but they are a good approximation for light sources VERY far away, like the sun or the moon.

The light shines along the forward direction of the entity’s transform. With a default transform this would be along the negative-Z axis.

Valid values for illuminance are:

To enable shadows, set the shadows_enabled property to true.

Shadows are produced via cascaded shadow maps.

To modify the cascade setup, such as the number of cascades or the maximum shadow distance, change the CascadeShadowConfig component of the entity with the DirectionalLight.

To control the resolution of the shadow maps, use the DirectionalLightShadowMap resource.

The color of the light.

By default, this is white.

Illuminance in lux (lumens per square meter), representing the amount of light projected onto surfaces by this light source. Lux is used here instead of lumens because a directional light illuminates all surfaces more-or-less the same way (depending on the angle of incidence). Lumens can only be specified for light sources which emit light from a specific area.

Whether this light casts shadows.

Note that shadows are rather expensive and become more so with every light that casts them. In general, it’s best to aggressively limit the number of lights with shadows enabled to one or two at most.

Whether soft shadows are enabled, and if so, the size of the light.

Soft shadows, also known as percentage-closer soft shadows or PCSS, cause shadows to become blurrier (i.e. their penumbra increases in radius) as they extend away from objects. The blurriness of the shadow depends on the size of the light; larger lights result in larger penumbras and therefore blurrier shadows.

Currently, soft shadows are rather noisy if not using the temporal mode. If you enable soft shadows, consider choosing ShadowFilteringMethod::Temporal and enabling temporal antialiasing (TAA) to smooth the noise out over time.

Note that soft shadows are significantly more expensive to render than hard shadows.

Whether this directional light contributes diffuse lighting to meshes with lightmaps.

Set this to false if your lightmap baking tool bakes the direct diffuse light from this directional light into the lightmaps in order to avoid counting the radiance from this light twice. Note that the specular portion of the light is always considered, because Bevy currently has no means to bake specular light.

By default, this is set to true.

A value that adjusts the t

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct DirectionalLight {
    pub color: Color,
    pub illuminance: f32,
    pub shadows_enabled: bool,
    pub soft_shadow_size: Option<f32>,
    pub affects_lightmapped_mesh_diffuse: bool,
    pub shadow_depth_bias: f32,
    pub shadow_normal_bias: f32,
}
```

---

## Struct Bluenoise Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.Bluenoise.html

**Contents:**
- Struct Bluenoise Copy item path
- Fields§
- Trait Implementations§
  - impl Resource for Bluenoisewhere Bluenoise: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for Bluenoise
  - impl !RefUnwindSafe for Bluenoise
  - impl Send for Bluenoise
  - impl Sync for Bluenoise
  - impl Unpin for Bluenoise

A resource that stores the spatio-temporal blue noise texture.

Texture handle for spatio-temporal blue noise

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Bluenoise {
    pub texture: Handle<Image>,
}
```

---

## Type Alias WithLight Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/type.WithLight.html

**Contents:**
- Type Alias WithLight Copy item path
- Aliased Type§

A convenient alias for Or<(With<PointLight>, With<SpotLight>, With<DirectionalLight>)>, for use with bevy_camera::visibility::VisibleEntities.

**Examples:**

Example 1 (unknown):
```unknown
pub type WithLight = Or<(With<PointLight>, With<SpotLight>, With<DirectionalLight>)>;
```

Example 2 (unknown):
```unknown
pub struct WithLight(/* private fields */);
```

---

## Struct LightmapSlab Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.LightmapSlab.html

**Contents:**
- Struct LightmapSlab Copy item path
- Auto Trait Implementations§
  - impl Freeze for LightmapSlab
  - impl !RefUnwindSafe for LightmapSlab
  - impl Send for LightmapSlab
  - impl Sync for LightmapSlab
  - impl Unpin for LightmapSlab
  - impl !UnwindSafe for LightmapSlab
- Blanket Implementations§
  - impl<T> Any for Twhere T: 'static + ?Sized,

A binding array that contains lightmaps.

This will have a single binding if bindless lightmaps aren’t in use.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct LightmapSlab { /* private fields */ }
```

---

## Struct EnvironmentMapUniform Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.EnvironmentMapUniform.html

**Contents:**
- Struct EnvironmentMapUniform Copy item path
- Trait Implementations§
  - impl Clone for EnvironmentMapUniform
    - fn clone(&self) -> EnvironmentMapUniform
    - fn clone_from(&mut self, source: &Self)
  - impl Component for EnvironmentMapUniformwhere EnvironmentMapUniform: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior

The uniform struct extracted from EnvironmentMapLight. Will be available for use in the Environment Map shader.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct EnvironmentMapUniform { /* private fields */ }
```

---

## Struct GpuClusterableObject Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.GpuClusterableObject.html

**Contents:**
- Struct GpuClusterableObject Copy item path
- Trait Implementations§
  - impl Clone for GpuClusterableObject
    - fn clone(&self) -> GpuClusterableObject
    - fn clone_from(&mut self, source: &Self)
  - impl CreateFrom for GpuClusterableObjectwhere GpuClusterableObject: ShaderType<ExtraMetadata = StructMetadata<11>>, Vec4: for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom, u32: for<'__> CreateFrom + for<'__> CreateFrom, f32: for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom,
    - fn create_from<B>(reader: &mut Reader<B>) -> GpuClusterableObjectwhere B: BufferRef,
  - impl Debug for GpuClusterableObject
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for GpuClusterableObject

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GpuClusterableObject { /* private fields */ }
```

---

## Enum GpuClusterableObjects Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/enum.GpuClusterableObjects.html

**Contents:**
- Enum GpuClusterableObjects Copy item path
- Variants§
  - Uniform(UniformBuffer<GpuClusterableObjectsUniform>)
  - Storage(StorageBuffer<GpuClusterableObjectsStorage>)
- Implementations§
  - impl GpuClusterableObjects
    - pub fn binding(&self) -> Option<BindingResource<'_>>
    - pub fn min_size(buffer_binding_type: BufferBindingType) -> NonZero<u64>
- Auto Trait Implementations§
  - impl Freeze for GpuClusterableObjects

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum GpuClusterableObjects {
    Uniform(UniformBuffer<GpuClusterableObjectsUniform>),
    Storage(StorageBuffer<GpuClusterableObjectsStorage>),
}
```

---

## Struct MorphIndex Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MorphIndex.html

**Contents:**
- Struct MorphIndex Copy item path
- Fields§
- Trait Implementations§
  - impl Component for MorphIndexwhere MorphIndex: Send + Sync + 'static,
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
pub struct MorphIndex {
    pub index: u32,
}
```

---

## Struct ExtractedClusterableObjects Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ExtractedClusterableObjects.html

**Contents:**
- Struct ExtractedClusterableObjects Copy item path
- Trait Implementations§
  - impl Component for ExtractedClusterableObjectswhere ExtractedClusterableObjects: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior
    - fn on_add() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_insert() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_replace() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ExtractedClusterableObjects { /* private fields */ }
```

---

## Struct LightmapSlabIndex Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.LightmapSlabIndex.html

**Contents:**
- Struct LightmapSlabIndex Copy item path
- Methods from Deref<Target = NonMaxU32>§
    - pub const ZERO: NonMaxU32
    - pub const ONE: NonMaxU32
    - pub const MAX: NonMaxU32
    - pub fn get(&self) -> u32
- Trait Implementations§
  - impl Clone for LightmapSlabIndex
    - fn clone(&self) -> LightmapSlabIndex
    - fn clone_from(&mut self, source: &Self)

The index of the slab (binding array) in which a lightmap is located.

Returns the value as a primitive type.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct LightmapSlabIndex(/* private fields */);
```

---

## Struct DeferredDrawFunction Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.DeferredDrawFunction.html

**Contents:**
- Struct DeferredDrawFunction Copy item path
- Trait Implementations§
  - impl Clone for DeferredDrawFunction
    - fn clone(&self) -> DeferredDrawFunction
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for DeferredDrawFunction
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for DeferredDrawFunction
    - fn default() -> DeferredDrawFunction
  - impl DrawFunctionLabel for DeferredDrawFunctionwhere DeferredDrawFunction: 'static + Send + Sync + Clone + Eq + Debug + Hash,

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DeferredDrawFunction;
```

---

## Struct ActionRequest Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/a11y/struct.ActionRequest.html

**Contents:**
- Struct ActionRequest Copy item path
- Tuple Fields§
- Trait Implementations§
  - impl Deref for ActionRequest
    - type Target = ActionRequest
    - fn deref(&self) -> &<ActionRequest as Deref>::Target
  - impl DerefMut for ActionRequest
    - fn deref_mut(&mut self) -> &mut <ActionRequest as Deref>::Target
  - impl<'de> Deserialize<'de> for ActionRequest
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<ActionRequest, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,

Wrapper struct for accesskit::ActionRequest.

This newtype is required to use ActionRequest as a Bevy Event.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ActionRequest(pub ActionRequest);
```

---
