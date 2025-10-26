# Crates-Rs-Bevy - Math

**Pages:** 157

---

## Struct U8Vec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.U8Vec2.html

**Contents:**
- Struct U8Vec2 Copy item path
- Fields§
- Implementations§
  - impl U8Vec2
    - pub const ZERO: U8Vec2
    - pub const ONE: U8Vec2
    - pub const MIN: U8Vec2
    - pub const MAX: U8Vec2
    - pub const X: U8Vec2
    - pub const Y: U8Vec2

A 2-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y]

Creates a vector from the first 2 values in slice.

Panics if slice is less than 2 elements long.

Writes the elements of self to the first 2 elements in slice.

Panics if slice is less than 2 elements long.

Creates a 3D vector from self and the given z value.

Creates a 2D vector from self with the given value of x.

Creates a 2D vector from self with the given value of y.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to u8::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes [self.x == rhs.x, self.y == rhs.y, ..] for all elements.

Returns a vector mask containing the result of a != comparison for each element of self and rhs.

In other words this computes [self.x != rhs.x, self.y != rhs.y, ..] for all elements.

Returns a vector mask containing the result of a >= comparison for 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct U8Vec2 {
    pub x: u8,
    pub y: u8,
}
```

---

## Struct IVec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.IVec3.html

**Contents:**
- Struct IVec3 Copy item path
- Fields§
- Implementations§
  - impl IVec3
    - pub const ZERO: IVec3
    - pub const ONE: IVec3
    - pub const NEG_ONE: IVec3
    - pub const MIN: IVec3
    - pub const MAX: IVec3
    - pub const X: IVec3

A 3-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

A unit vector pointing along the negative Z axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z]

Creates a vector from the first 3 values in slice.

Panics if slice is less than 3 elements long.

Writes the elements of self to the first 3 elements in slice.

Panics if slice is less than 3 elements long.

Creates a 4D vector from self and the given w value.

Creates a 2D vector from the x and y elements of self, discarding z.

Truncation may also be performed by using self.xy().

Creates a 3D vector from self with the given value of x.

Creates a 3D vector from self with the given value of y.

Creates a 3D vector from self with the given value of z.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Computes the cross product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to i32::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Ret

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
```

---

## Struct Vec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Vec4.html

**Contents:**
- Struct Vec4 Copy item path
- Implementations§
  - impl Vec4
    - pub const ZERO: Vec4
    - pub const ONE: Vec4
    - pub const NEG_ONE: Vec4
    - pub const MIN: Vec4
    - pub const MAX: Vec4
    - pub const NAN: Vec4
    - pub const INFINITY: Vec4

A 4-dimensional vector.

SIMD vector types are used for storage on supported platforms.

This type is 16 byte aligned.

All f32::NEG_INFINITY.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the positive W axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

A unit vector pointing along the negative Z axis.

A unit vector pointing along the negative W axis.

Vec4 uses Rust Portable SIMD

Vec4 uses scalar math

Vec4 uses WebAssembly 128-bit SIMD

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z, w]

Creates a vector from the first 4 values in slice.

Panics if slice is less than 4 elements long.

Writes the elements of self to the first 4 elements in slice.

Panics if slice is less than 4 elements long.

Creates a 3D vector from the x, y and z elements of self, discarding w.

Truncation to Vec3 may also be performed by using self.xyz().

To truncate to Vec3A use Vec3A::from_vec4().

Creates a 4D vector from self with the given value of x.

Creates a 4D vector from self with the given value of y.

Creates a 4D vector from self with the given value of z.

Creates a 4D vector from self with the given value of w.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

NaN propogation does not follow IEEE 754-2008 semantics for minNum and may differ on different SIMD architectures.

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

NaN propogation does not follow IEEE 754-2008 semantics for maxNum and may differ on different SIMD architectures.

Component-wise clamping of values, similar to f32::clamp.

Each element in min must be less-or-equal to the corresponding elemen

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Vec4(/* private fields */);
```

Example 2 (javascript):
```javascript
20fn setup(
21    mut commands: Commands,
22    mut ui_materials: ResMut<Assets<CustomUiMaterial>>,
23    asset_server: Res<AssetServer>,
24) {
25    // Camera so we can see UI
26    commands.spawn(Camera2d);
27
28    commands
29        .spawn(Node {
30            width: percent(100),
31            height: percent(100),
32            align_items: AlignItems::Center,
33            justify_content: JustifyContent::Center,
34            ..default()
35        })
36        .with_children(|parent| {
37            let banner_scale_factor = 0.5;
38            parent.spawn((
39                Node {
40
...
```

---

## Function dvec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.dvec4.html

**Contents:**
- Function dvec4 Copy item path

Creates a 4-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn dvec4(x: f64, y: f64, z: f64, w: f64) -> DVec4
```

---

## Struct WithDerivative Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.WithDerivative.html

**Contents:**
- Struct WithDerivative Copy item path
- Fields§
- Trait Implementations§
  - impl<T> Clone for WithDerivative<T>where T: Clone + HasTangent, <T as HasTangent>::Tangent: Clone,
    - fn clone(&self) -> WithDerivative<T>
    - fn clone_from(&mut self, source: &Self)
  - impl<T, C> Curve<WithDerivative<T>> for SampleDerivativeWrapper<C>where T: HasTangent, C: SampleDerivative<T>,
    - fn domain(&self) -> Interval
    - fn sample_unchecked(&self, t: f32) -> WithDerivative<T>
    - fn sample(&self, t: f32) -> Option<WithDerivative<T>>

A value with its derivative.

The underlying value.

The derivative at value.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WithDerivative<T>where
    T: HasTangent,{
    pub value: T,
    pub derivative: <T as HasTangent>::Tangent,
}
```

---

## Function usizevec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.usizevec2.html

**Contents:**
- Function usizevec2 Copy item path

Creates a 2-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn usizevec2(x: usize, y: usize) -> USizeVec2
```

---

## Function u8vec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.u8vec3.html

**Contents:**
- Function u8vec3 Copy item path

Creates a 3-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn u8vec3(x: u8, y: u8, z: u8) -> U8Vec3
```

---

## Struct DVec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.DVec2.html

**Contents:**
- Struct DVec2 Copy item path
- Fields§
- Implementations§
  - impl DVec2
    - pub const ZERO: DVec2
    - pub const ONE: DVec2
    - pub const NEG_ONE: DVec2
    - pub const MIN: DVec2
    - pub const MAX: DVec2
    - pub const NAN: DVec2

A 2-dimensional vector.

All f64::NEG_INFINITY.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

DVec2 uses Rust Portable SIMD

DVec2 uses scalar math

DVec2 uses Intel SSE2

DVec2 uses WebAssembly 128-bit SIMD

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y]

Creates a vector from the first 2 values in slice.

Panics if slice is less than 2 elements long.

Writes the elements of self to the first 2 elements in slice.

Panics if slice is less than 2 elements long.

Creates a 3D vector from self and the given z value.

Creates a 2D vector from self with the given value of x.

Creates a 2D vector from self with the given value of y.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

NaN propogation does not follow IEEE 754-2008 semantics for minNum and may differ on different SIMD architectures.

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

NaN propogation does not follow IEEE 754-2008 semantics for maxNum and may differ on different SIMD architectures.

Component-wise clamping of values, similar to f64::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

NaN propogation does not follow IEEE 754-2008 semantics and ma

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct DVec2 {
    pub x: f64,
    pub y: f64,
}
```

Example 2 (unknown):
```unknown
438fn fibonacci_spiral_on_sphere(golden_ratio: f64, i: usize, n: usize) -> DVec2 {
439    DVec2::new(
440        PI * 2. * (i as f64 / golden_ratio),
441        f64::acos(1.0 - 2.0 * (i as f64 + EPSILON) / (n as f64 - 1.0 + 2.0 * EPSILON)),
442    )
443}
```

Example 3 (unknown):
```unknown
120fn fibonacci_spiral_on_sphere(golden_ratio: f64, i: usize, n: usize) -> DVec2 {
121    DVec2::new(
122        PI * 2. * (i as f64 / golden_ratio),
123        ops::acos((1.0 - 2.0 * (i as f64 + EPSILON) / (n as f64 - 1.0 + 2.0 * EPSILON)) as f32)
124            as f64,
125    )
126}
```

---

## Function bvec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.bvec3.html

**Contents:**
- Function bvec3 Copy item path

Creates a 3-dimensional bool vector mask.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn bvec3(x: bool, y: bool, z: bool) -> BVec3
```

---

## Struct Isometry3d Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Isometry3d.html

**Contents:**
- Struct Isometry3d Copy item path
- §Example
- Fields§
- Implementations§
  - impl Isometry3d
    - pub const IDENTITY: Isometry3d
    - pub fn new(translation: impl Into<Vec3A>, rotation: Quat) -> Isometry3d
      - Examples found in repository?
    - pub const fn from_rotation(rotation: Quat) -> Isometry3d
    - pub fn from_translation(translation: impl Into<Vec3A>) -> Isometry3d

An isometry in three dimensions, representing a rotation followed by a translation. This can often be useful for expressing relative positions and transformations from one position to another.

In particular, this type represents a distance-preserving transformation known as a rigid motion or a direct motion, and belongs to the special Euclidean group SE(3). This includes translation and rotation, but excludes reflection.

For the two-dimensional version, see Isometry2d.

Isometries can be created from a given translation and rotation:

Or from separate parts:

The isometries can be used to transform points:

Isometries can also be composed together:

One common operation is to compute an isometry representing the relative positions of two objects for things like intersection tests. This can be done with an inverse transformation:

The rotational part of a three-dimensional isometry.

The translational part of a three-dimensional isometry.

The identity isometry which represents the rigid motion of not doing anything.

Create a three-dimensional isometry from a rotation and a translation.

Create a three-dimensional isometry from a rotation.

Create a three-dimensional isometry from a translation.

Create a three-dimensional isometry from a translation with the given x, y, and z components.

The inverse isometry that undoes this one.

Compute iso1.inverse() * iso2 in a more efficient way for one-shot cases.

If the same isometry is used multiple times, it is more efficient to instead compute the inverse once and use that for each transformation.

Transform a point by rotating and translating it using this isometry.

Transform a point by rotating and translating it using the inverse of this isometry.

This is more efficient than iso.inverse().transform_point(point) for one-shot cases. If the same isometry is used multiple times, it is more efficient to instead compute the inverse once and use that for each transformation.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Isometry3d {
    pub rotation: Quat,
    pub translation: Vec3A,
}
```

Example 2 (javascript):
```javascript
let iso = Isometry3d::new(Vec3::new(2.0, 1.0, 3.0), Quat::from_rotation_z(FRAC_PI_2));
```

Example 3 (javascript):
```javascript
let iso1 = Isometry3d::from_translation(Vec3::new(2.0, 1.0, 3.0));
let iso2 = Isometry3d::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
```

Example 4 (javascript):
```javascript
let iso = Isometry3d::new(Vec3::new(2.0, 1.0, 3.0), Quat::from_rotation_z(FRAC_PI_2));
let point = Vec3::new(4.0, 4.0, 4.0);

// These are equivalent
let result = iso.transform_point(point);
let result = iso * point;

assert_relative_eq!(result, Vec3::new(-2.0, 5.0, 7.0));
```

---

## Module u16 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/u16/index.html

**Contents:**
- Module u16 Copy item path
- Structs§
- Functions§

bevy::mathModule u16 Copy item pathSource Expand descriptionu16 vector types. Structs§U16Vec2A 2-dimensional vector.U16Vec3A 3-dimensional vector.U16Vec4A 4-dimensional vector.Functions§u16vec2Creates a 2-dimensional vector.u16vec3Creates a 3-dimensional vector.u16vec4Creates a 4-dimensional vector.

---

## Struct UVec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.UVec4.html

**Contents:**
- Struct UVec4 Copy item path
- Fields§
- Implementations§
  - impl UVec4
    - pub const ZERO: UVec4
    - pub const ONE: UVec4
    - pub const MIN: UVec4
    - pub const MAX: UVec4
    - pub const X: UVec4
    - pub const Y: UVec4

A 4-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the positive W axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z, w]

Creates a vector from the first 4 values in slice.

Panics if slice is less than 4 elements long.

Writes the elements of self to the first 4 elements in slice.

Panics if slice is less than 4 elements long.

Creates a 3D vector from the x, y and z elements of self, discarding w.

Truncation to UVec3 may also be performed by using self.xyz().

Creates a 4D vector from self with the given value of x.

Creates a 4D vector from self with the given value of y.

Creates a 4D vector from self with the given value of z.

Creates a 4D vector from self with the given value of w.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to u32::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes [s

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct UVec4 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}
```

---

## Struct Mat4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Mat4.html

**Contents:**
- Struct Mat4 Copy item path
- Fields§
- Implementations§
  - impl Mat4
    - pub const ZERO: Mat4
    - pub const IDENTITY: Mat4
    - pub const NAN: Mat4
    - pub const fn from_cols( x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4, ) -> Mat4
    - pub const fn from_cols_array(m: &[f32; 16]) -> Mat4
    - pub const fn to_cols_array(&self) -> [f32; 16]

A 4x4 column major matrix.

This 4x4 matrix type features convenience methods for creating and using affine transforms and perspective projections. If you are primarily dealing with 3D affine transformations considering using Affine3A which is faster than a 4x4 matrix for some affine operations.

Affine transformations including 3D translation, rotation and scale can be created using methods such as Self::from_translation(), Self::from_quat(), Self::from_scale() and Self::from_scale_rotation_translation().

Orthographic projections can be created using the methods Self::orthographic_lh() for left-handed coordinate systems and Self::orthographic_rh() for right-handed systems. The resulting matrix is also an affine transformation.

The Self::transform_point3() and Self::transform_vector3() convenience methods are provided for performing affine transformations on 3D vectors and points. These multiply 3D inputs as 4D vectors with an implicit w value of 1 for points and 0 for vectors respectively. These methods assume that Self contains a valid affine transform.

Perspective projections can be created using methods such as Self::perspective_lh(), Self::perspective_infinite_lh() and Self::perspective_infinite_reverse_lh() for left-handed co-ordinate systems and Self::perspective_rh(), Self::perspective_infinite_rh() and Self::perspective_infinite_reverse_rh() for right-handed co-ordinate systems.

The resulting perspective project can be use to transform 3D vectors as points with perspective correction using the Self::project_point3() convenience method.

A 4x4 matrix with all elements set to 0.0.

A 4x4 identity matrix, where all diagonal elements are 1, and all off-diagonal elements are 0.

Creates a 4x4 matrix from four column vectors.

Creates a 4x4 matrix from a [f32; 16] array stored in column major order. If your data is stored in row major you will need to transpose the returned matrix.

Creates a [f32; 16] array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates a 4x4 matrix from a [[f32; 4]; 4] 4D array stored in column major order. If your data is in row major order you will need to transpose the returned matrix.

Creates a [[f32; 4]; 4] 4D array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates a 4x4 matrix with its diagonal set to diagonal and all other entries set to 0.

Creates an affine transformation matrix from the given 3D

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct Mat4 {
    pub x_axis: Vec4,
    pub y_axis: Vec4,
    pub z_axis: Vec4,
    pub w_axis: Vec4,
}
```

Example 2 (unknown):
```unknown
59static VOXEL_FROM_WORLD: Mat4 = Mat4::from_cols_array_2d(&[
60    [-42.317566, 0.0, 0.0, 0.0],
61    [0.0, 0.0, 44.601563, 0.0],
62    [0.0, 16.73776, 0.0, 0.0],
63    [0.0, 6.544792, 0.0, 1.0],
64]);
```

Example 3 (javascript):
```javascript
92fn setup_meshes(
93    mut commands: Commands,
94    mut mesh_assets: ResMut<Assets<Mesh>>,
95    mut material_assets: ResMut<Assets<StandardMaterial>>,
96    mut inverse_bindposes_assets: ResMut<Assets<SkinnedMeshInverseBindposes>>,
97) {
98    // Create a mesh with two rectangles.
99    let unskinned_mesh = Mesh::new(
100        PrimitiveTopology::TriangleList,
101        RenderAssetUsages::default(),
102    )
103    .with_inserted_attribute(
104        Mesh::ATTRIBUTE_POSITION,
105        vec![
106            [-0.3, -0.3, 0.0],
107            [0.3, -0.3, 0.0],
108            [-0.3, 0.3, 0
...
```

Example 4 (javascript):
```javascript
37fn setup(
38    mut commands: Commands,
39    asset_server: Res<AssetServer>,
40    mut meshes: ResMut<Assets<Mesh>>,
41    mut materials: ResMut<Assets<StandardMaterial>>,
42    mut skinned_mesh_inverse_bindposes_assets: ResMut<Assets<SkinnedMeshInverseBindposes>>,
43) {
44    // Create a camera
45    commands.spawn((
46        Camera3d::default(),
47        Transform::from_xyz(2.5, 2.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
48    ));
49
50    // Create inverse bindpose matrices for a skeleton consists of 2 joints
51    let inverse_bindposes = skinned_mesh_inverse_bindposes_assets.add(vec![
...
```

---

## Function uvec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.uvec2.html

**Contents:**
- Function uvec2 Copy item path

Creates a 2-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn uvec2(x: u32, y: u32) -> UVec2
```

---

## Module sampling Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/sampling/index.html

**Contents:**
- Module sampling Copy item path
- Modules§
- Structs§
- Traits§

This module contains tools related to random sampling.

To use this, the “rand” feature must be enabled.

---

## Struct U64Vec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.U64Vec4.html

**Contents:**
- Struct U64Vec4 Copy item path
- Fields§
- Implementations§
  - impl U64Vec4
    - pub const ZERO: U64Vec4
    - pub const ONE: U64Vec4
    - pub const MIN: U64Vec4
    - pub const MAX: U64Vec4
    - pub const X: U64Vec4
    - pub const Y: U64Vec4

A 4-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the positive W axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z, w]

Creates a vector from the first 4 values in slice.

Panics if slice is less than 4 elements long.

Writes the elements of self to the first 4 elements in slice.

Panics if slice is less than 4 elements long.

Creates a 3D vector from the x, y and z elements of self, discarding w.

Truncation to U64Vec3 may also be performed by using self.xyz().

Creates a 4D vector from self with the given value of x.

Creates a 4D vector from self with the given value of y.

Creates a 4D vector from self with the given value of z.

Creates a 4D vector from self with the given value of w.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to u64::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct U64Vec4 {
    pub x: u64,
    pub y: u64,
    pub z: u64,
    pub w: u64,
}
```

---

## Function mat3a Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.mat3a.html

**Contents:**
- Function mat3a Copy item path

Creates a 3x3 matrix from three column vectors.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn mat3a(x_axis: Vec3A, y_axis: Vec3A, z_axis: Vec3A) -> Mat3A
```

---

## Struct USizeVec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.USizeVec4.html

**Contents:**
- Struct USizeVec4 Copy item path
- Fields§
- Implementations§
  - impl USizeVec4
    - pub const ZERO: USizeVec4
    - pub const ONE: USizeVec4
    - pub const MIN: USizeVec4
    - pub const MAX: USizeVec4
    - pub const X: USizeVec4
    - pub const Y: USizeVec4

A 4-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the positive W axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z, w]

Creates a vector from the first 4 values in slice.

Panics if slice is less than 4 elements long.

Writes the elements of self to the first 4 elements in slice.

Panics if slice is less than 4 elements long.

Creates a 3D vector from the x, y and z elements of self, discarding w.

Truncation to USizeVec3 may also be performed by using self.xyz().

Creates a 4D vector from self with the given value of x.

Creates a 4D vector from self with the given value of y.

Creates a 4D vector from self with the given value of z.

Creates a 4D vector from self with the given value of w.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to usize::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this compu

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct USizeVec4 {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub w: usize,
}
```

---

## Module curve Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/curve/index.html

**Contents:**
- Module curve Copy item path
  - §Overview
  - §Analogy with Iterator
  - §Defining curves
  - §Transforming curves
  - §Combining curves
  - §Resampling and rasterization
  - §Ownership and borrowing
- Structs§
- Enums§

The Curve trait, providing a domain-agnostic description of curves.

At a high level, Curve is a trait that abstracts away the implementation details of curves, which comprise any kind of data parametrized by a single continuous variable. For example, that variable could represent time, in which case a curve would represent a value that changes over time, as in animation; on the other hand, it could represent something like displacement or distance, as in graphs, gradients, and curves in space.

The trait itself has two fundamental components: a curve must have a domain, which is a nonempty range of f32 values, and it must be able to be sampled on every one of those values, producing output of some fixed type.

A primary goal of the trait is to allow interfaces to simply accept impl Curve<T> as input rather than requiring for input curves to be defined in data in any particular way. This is supported by a number of interface methods which allow changing parametrizations, mapping output, and rasterization.

The Curve API behaves, in many ways, like a continuous counterpart to Iterator. The analogy looks something like this with some of the common methods:

Of course, there are very important differences, as well. For instance, the continuous nature of curves means that many iterator methods make little sense in the context of curves, or at least require numerical techniques. For example, the analogue of sum would be an integral, approximated by something like Riemann summation.

Furthermore, the two also differ greatly in their orientation to borrowing and mutation: iterators are mutated by being iterated, and by contrast, all curve methods are immutable. More information on the implications of this can be found below.

Curves may be defined in a number of ways. The following are common:

Among these, the first is the most versatile1: the domain and the sampling output are just specified directly in the construction. For this reason, function curves are a reliable go-to for simple one-off constructions and procedural uses, where flexibility is desirable. For example:

Sample-interpolated curves commonly arises in both rasterization and in animation, and this library has support for producing them in both fashions. See below for more information about rasterization. Here is what an explicit sample-interpolated curve might look like:

For more information on spline curves and easing curves, see their respective modules.

And, of course, you are also free to d

*[Content truncated]*

**Examples:**

Example 1 (javascript):
```javascript
// A sinusoid:
let sine_curve = FunctionCurve::new(Interval::EVERYWHERE, f32::sin);

// A sawtooth wave:
let sawtooth_curve = FunctionCurve::new(Interval::EVERYWHERE, |t| t % 1.0);

// A helix:
let helix_curve = FunctionCurve::new(Interval::EVERYWHERE, |theta| vec3(theta.sin(), theta, theta.cos()));
```

Example 2 (javascript):
```javascript
// A list of angles that we want to traverse:
let angles = [
    0.0,
    -FRAC_PI_2,
    0.0,
    FRAC_PI_2,
    0.0
];

// Make each angle into a rotation by that angle:
let rotations = angles.map(|angle| Rot2::radians(angle));

// Interpolate these rotations with a `Rot2`-valued curve:
let rotation_curve = SampleAutoCurve::new(interval(0.0, 4.0).unwrap(), rotations).unwrap();
```

Example 3 (unknown):
```unknown
struct ExponentialCurve {
    exponent: f32,
}

impl Curve<f32> for ExponentialCurve {
    fn domain(&self) -> Interval {
        Interval::EVERYWHERE
    }

    fn sample_unchecked(&self, t: f32) -> f32 {
        f32::exp(self.exponent * t)
    }

    // All other trait methods can be inferred from these.
}
```

Example 4 (javascript):
```javascript
// Our original curve, which may look something like this:
let ellipse_curve = FunctionCurve::new(
    interval(0.0, TAU).unwrap(),
    |t| vec2(t.cos(), t.sin() * 2.0)
);

// Use `map` to situate this in 3D as a Curve<Vec3>; in this case, it will be in the xy-plane:
let ellipse_motion_curve = ellipse_curve.map(|pos| pos.extend(0.0));
```

---

## Trait ScalarField Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/trait.ScalarField.html

**Contents:**
- Trait ScalarField Copy item path
- Required Associated Constants§
    - const ZERO: Self
    - const ONE: Self
- Provided Methods§
    - fn recip(self) -> Self
- Dyn Compatibility§
- Implementations on Foreign Types§
  - impl ScalarField for f32
    - const ZERO: f32 = 0f32

A type that supports the operations of a scalar field. An implementation should support:

Within the limitations of floating point arithmetic, all the following are required to hold:

The additive identity.

The multiplicative identity.

The multiplicative inverse of this element. This is equivalent to 1.0 / self.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (javascript):
```javascript
pub trait ScalarField:
    Mul<Output = Self>
    + Div<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
    + Default
    + Debug
    + Clone
    + Copy {
    const ZERO: Self;
    const ONE: Self;

    // Provided method
    fn recip(self) -> Self { ... }
}
```

---

## Function i16vec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.i16vec3.html

**Contents:**
- Function i16vec3 Copy item path

Creates a 3-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn i16vec3(x: i16, y: i16, z: i16) -> I16Vec3
```

---

## Struct Vec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Vec3.html

**Contents:**
- Struct Vec3 Copy item path
- Fields§
- Implementations§
  - impl Vec3
    - pub const ZERO: Vec3
    - pub const ONE: Vec3
    - pub const NEG_ONE: Vec3
    - pub const MIN: Vec3
    - pub const MAX: Vec3
    - pub const NAN: Vec3

A 3-dimensional vector.

All f32::NEG_INFINITY.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

A unit vector pointing along the negative Z axis.

Vec3 uses Rust Portable SIMD

Vec3 uses scalar math

Vec3 uses WebAssembly 128-bit SIMD

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z]

Creates a vector from the first 3 values in slice.

Panics if slice is less than 3 elements long.

Writes the elements of self to the first 3 elements in slice.

Panics if slice is less than 3 elements long.

Creates a 4D vector from self and the given w value.

Creates a 2D vector from the x and y elements of self, discarding z.

Truncation may also be performed by using self.xy().

Creates a 3D vector from self with the given value of x.

Creates a 3D vector from self with the given value of y.

Creates a 3D vector from self with the given value of z.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Computes the cross product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

NaN propogation does not follow IEEE 754-2008 semantics for minNum and may differ on different SIMD architectures.

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

NaN propogation does not follow IEEE 754-2008 semantics for maxNum and may differ on different SIMD architectures.

Component-wise clamping of values, similar to f32::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
```

Example 2 (javascript):
```javascript
47const DISTANCE_BETWEEN_SHAPES: Vec3 = Vec3::new(2.0, 0.0, 0.0);
48
49/// Maximum amount of points allowed to be present.
50/// Should be set such that it does not cause large amounts of lag when reached.
51const MAX_POINTS: usize = 3000; // TODO: Test wasm and add a wasm-specific-bound
52
53/// How many points should be spawned each frame
54const POINTS_PER_FRAME: usize = 3;
55
56/// Color used for the inside points
57const INSIDE_POINT_COLOR: LinearRgba = LinearRgba::rgb(0.855, 1.1, 0.01);
58/// Color used for the points on the boundary
59const BOUNDARY_POINT_COLOR: LinearRgba = LinearRgba:
...
```

Example 3 (javascript):
```javascript
21const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
22const BALL_DIAMETER: f32 = 30.;
23const BALL_SPEED: f32 = 400.0;
24const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
25
26const WALL_THICKNESS: f32 = 10.0;
27// x coordinates
28const LEFT_WALL: f32 = -450.;
29const RIGHT_WALL: f32 = 450.;
30// y coordinates
31const BOTTOM_WALL: f32 = -300.;
32const TOP_WALL: f32 = 300.;
33
34const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
35// These values are exact
36const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
37const GAP_BETWEEN_BRICKS: f32 = 5.0;
38// These values are lower b
...
```

Example 4 (javascript):
```javascript
153const CUBOID: Cuboid = Cuboid {
154    half_size: Vec3::new(BIG_3D, SMALL_3D, BIG_3D),
155};
156
157const CIRCLE: Circle = Circle { radius: BIG_2D };
158const SPHERE: Sphere = Sphere { radius: BIG_3D };
159
160const ELLIPSE: Ellipse = Ellipse {
161    half_size: Vec2::new(BIG_2D, SMALL_2D),
162};
163
164const TRIANGLE_2D: Triangle2d = Triangle2d {
165    vertices: [
166        Vec2::new(BIG_2D, 0.0),
167        Vec2::new(0.0, BIG_2D),
168        Vec2::new(-BIG_2D, 0.0),
169    ],
170};
171
172const TRIANGLE_3D: Triangle3d = Triangle3d {
173    vertices: [
174        Vec3::new(BIG_3D, 0.0, 0
...
```

---

## Struct Mat3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Mat3.html

**Contents:**
- Struct Mat3 Copy item path
- Fields§
- Implementations§
  - impl Mat3
    - pub const ZERO: Mat3
    - pub const IDENTITY: Mat3
    - pub const NAN: Mat3
    - pub const fn from_cols(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Mat3
      - Examples found in repository?
    - pub const fn from_cols_array(m: &[f32; 9]) -> Mat3

A 3x3 column major matrix.

This 3x3 matrix type features convenience methods for creating and using linear and affine transformations. If you are primarily dealing with 2D affine transformations the Affine2 type is much faster and more space efficient than using a 3x3 matrix.

Linear transformations including 3D rotation and scale can be created using methods such as Self::from_diagonal(), Self::from_quat(), Self::from_axis_angle(), Self::from_rotation_x(), Self::from_rotation_y(), or Self::from_rotation_z().

The resulting matrices can be use to transform 3D vectors using regular vector multiplication.

Affine transformations including 2D translation, rotation and scale can be created using methods such as Self::from_translation(), Self::from_angle(), Self::from_scale() and Self::from_scale_angle_translation().

The Self::transform_point2() and Self::transform_vector2() convenience methods are provided for performing affine transforms on 2D vectors and points. These multiply 2D inputs as 3D vectors with an implicit z value of 1 for points and 0 for vectors respectively. These methods assume that Self contains a valid affine transform.

A 3x3 matrix with all elements set to 0.0.

A 3x3 identity matrix, where all diagonal elements are 1, and all off-diagonal elements are 0.

Creates a 3x3 matrix from three column vectors.

Creates a 3x3 matrix from a [f32; 9] array stored in column major order. If your data is stored in row major you will need to transpose the returned matrix.

Creates a [f32; 9] array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates a 3x3 matrix from a [[f32; 3]; 3] 3D array stored in column major order. If your data is in row major order you will need to transpose the returned matrix.

Creates a [[f32; 3]; 3] 3D array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates a 3x3 matrix with its diagonal set to diagonal and all other entries set to 0.

Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column.

Creates a 3x3 matrix from the minor of the given 4x4 matrix, discarding the ith column and jth row.

Panics if i or j is greater than 3.

Creates a 3D rotation matrix from the given quaternion.

Will panic if rotation is not normalized when glam_assert is enabled.

Creates a 3D rotation matrix from a normalized rotation axis and angle (in radians).

Will panic if axis is not normalized when glam_ass

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct Mat3 {
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3,
}
```

Example 2 (javascript):
```javascript
136fn track_targets(
137    // `Single` ensures the system runs ONLY when exactly one matching entity exists.
138    mut player: Single<(&mut Transform, &Player)>,
139    // `Option<Single>` never prevents the system from running, but will be `None` if there is not exactly one matching entity.
140    enemy: Option<Single<&Transform, (With<Enemy>, Without<Player>)>>,
141    time: Res<Time>,
142) {
143    let (player_transform, player) = &mut *player;
144    if let Some(enemy_transform) = enemy {
145        // Enemy found, rotate and move towards it.
146        let delta = enemy_transform.transl
...
```

Example 3 (javascript):
```javascript
215fn move_camera(
216    keyboard_input: Res<ButtonInput<KeyCode>>,
217    mut mouse_wheel_reader: MessageReader<MouseWheel>,
218    mut cameras: Query<&mut Transform, With<Camera3d>>,
219) {
220    let (mut zoom_delta, mut theta_delta) = (0.0, 0.0);
221
222    // Process zoom in and out via the keyboard.
223    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
224        zoom_delta -= CAMERA_KEYBOARD_ZOOM_SPEED;
225    } else if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
226        zoom_delta += CAMERA_KEYBOARD_Z
...
```

Example 4 (javascript):
```javascript
591fn rotate_primitive_2d_meshes(
592    mut primitives_2d: Query<
593        (&mut Transform, &ViewVisibility),
594        (With<PrimitiveData>, With<MeshDim2>),
595    >,
596    time: Res<Time>,
597) {
598    let rotation_2d = Quat::from_mat3(&Mat3::from_angle(time.elapsed_secs()));
599    primitives_2d
600        .iter_mut()
601        .filter(|(_, vis)| vis.get())
602        .for_each(|(mut transform, _)| {
603            transform.rotation = rotation_2d;
604        });
605}
```

---

## Module cubic_splines Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/cubic_splines/index.html

**Contents:**
- Module cubic_splines Copy item path
- Structs§
- Enums§
- Traits§

Provides types for building cubic splines for rendering curves and use with animation easing.

---

## Function mat2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.mat2.html

**Contents:**
- Function mat2 Copy item path

Creates a 2x2 matrix from two column vectors.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn mat2(x_axis: Vec2, y_axis: Vec2) -> Mat2
```

---

## Function usizevec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.usizevec3.html

**Contents:**
- Function usizevec3 Copy item path

Creates a 3-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn usizevec3(x: usize, y: usize, z: usize) -> USizeVec3
```

---

## Trait Vec4Swizzles Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/trait.Vec4Swizzles.html

**Contents:**
- Trait Vec4Swizzles Copy item path
- Required Associated Types§
    - type Vec2
    - type Vec3
- Required Methods§
    - fn xx(self) -> Self::Vec2
    - fn xy(self) -> Self::Vec2
    - fn with_xy(self, rhs: Self::Vec2) -> Self
    - fn xz(self) -> Self::Vec2
    - fn with_xz(self, rhs: Self::Vec2) -> Self

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait Vec4Swizzles:
    Sized
    + Copy
    + Clone {
    type Vec2;
    type Vec3;

Show 372 methods    // Required methods
    fn xx(self) -> Self::Vec2;
    fn xy(self) -> Self::Vec2;
    fn with_xy(self, rhs: Self::Vec2) -> Self;
    fn xz(self) -> Self::Vec2;
    fn with_xz(self, rhs: Self::Vec2) -> Self;
    fn xw(self) -> Self::Vec2;
    fn with_xw(self, rhs: Self::Vec2) -> Self;
    fn yx(self) -> Self::Vec2;
    fn with_yx(self, rhs: Self::Vec2) -> Self;
    fn yy(self) -> Self::Vec2;
    fn yz(self) -> Self::Vec2;
    fn with_yz(self, rhs: Self::Vec2) -> Self;
    fn yw(self) ->
...
```

---

## Function u64vec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.u64vec3.html

**Contents:**
- Function u64vec3 Copy item path

Creates a 3-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn u64vec3(x: u64, y: u64, z: u64) -> U64Vec3
```

---

## Struct Dir3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Dir3.html

**Contents:**
- Struct Dir3 Copy item path
- Implementations§
  - impl Dir3
    - pub const X: Dir3
    - pub const Y: Dir3
    - pub const Z: Dir3
    - pub const NEG_X: Dir3
    - pub const NEG_Y: Dir3
    - pub const NEG_Z: Dir3
    - pub const AXES: [Dir3; 3]

A normalized vector pointing in a direction in 3D space

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

A unit vector pointing along the negative Z axis.

The directional axes.

Create a direction from a finite, nonzero Vec3, normalizing it.

Returns Err(InvalidDirectionError) if the length of the given vector is zero (or very close to zero), infinite, or NaN.

Create a Dir3 from a Vec3 that is already normalized.

value must be normalized, i.e its length must be 1.0.

Create a direction from a finite, nonzero Vec3, normalizing it and also returning its original length.

Returns Err(InvalidDirectionError) if the length of the given vector is zero (or very close to zero), infinite, or NaN.

Create a direction from its x, y, and z components.

Returns Err(InvalidDirectionError) if the length of the vector formed by the components is zero (or very close to zero), infinite, or NaN.

Create a direction from its x, y, and z components, assuming the resulting vector is normalized.

The vector produced from x, y, and z must be normalized, i.e its length must be 1.0.

Returns the inner Vec3

Performs a spherical linear interpolation between self and rhs based on the value s.

This corresponds to interpolating between the two directions at a constant angular velocity.

When s == 0.0, the result will be equal to self. When s == 1.0, the result will be equal to rhs.

Returns self after an approximate normalization, assuming the value is already nearly normalized. Useful for preventing numerical error accumulation.

The following seemingly benign code would start accumulating errors over time, leading to dir eventually not being normalized anymore.

Instead, do the following.

Converts self to [x, y, z]

Moves towards rhs based on the value d.

When d is 0.0, the result will be equal to self. When d is equal to self.distance(rhs), the result will be equal to rhs. Will not go past rhs.

Returns some vector that is orthogonal to the given one.

The input vector must be finite and non-zero.

The output vector is not necessarily unit length. For that use Self::any_orthonormal_vector() instead.

Returns any unit vector that is orthogonal to the given one.

The input vector must be unit length.

Will panic if self is not normalized when glam_assert is enabled.

Given a u

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Dir3(/* private fields */);
```

Example 2 (javascript):
```javascript
23fn bouncing_raycast(
24    mut ray_cast: MeshRayCast,
25    mut gizmos: Gizmos,
26    time: Res<Time>,
27    // The ray map stores rays cast by the cursor
28    ray_map: Res<RayMap>,
29) {
30    // Cast an automatically moving ray and bounce it off of surfaces
31    let t = ops::cos((time.elapsed_secs() - 4.0).max(0.0) * LASER_SPEED) * PI;
32    let ray_pos = Vec3::new(ops::sin(t), ops::cos(3.0 * t) * 0.5, ops::cos(t)) * 0.5;
33    let ray_dir = Dir3::new(-ray_pos).unwrap();
34    let ray = Ray3d::new(ray_pos, ray_dir);
35    gizmos.sphere(ray_pos, 0.1, Color::WHITE);
36    bounce_ray(ray, &
...
```

Example 3 (javascript):
```javascript
92fn move_target(
93    mut target: Single<&mut Transform, With<TargetSphere>>,
94    target_speed: Res<TargetSphereSpeed>,
95    mut target_pos: ResMut<TargetPosition>,
96    time: Res<Time>,
97    mut rng: ResMut<RandomSource>,
98) {
99    match Dir3::new(target_pos.0 - target.translation) {
100        // The target and the present position of the target sphere are far enough to have a well-
101        // defined direction between them, so let's move closer:
102        Ok(dir) => {
103            let delta_time = time.delta_secs();
104            let abs_delta = (target_pos.0 - target.transl
...
```

Example 4 (javascript):
```javascript
13fn draw_cursor(
14    camera_query: Single<(&Camera, &GlobalTransform)>,
15    ground: Single<&GlobalTransform, With<Ground>>,
16    window: Single<&Window>,
17    mut gizmos: Gizmos,
18) {
19    let (camera, camera_transform) = *camera_query;
20
21    if let Some(cursor_position) = window.cursor_position()
22        // Calculate a ray pointing from the camera into the world based on the cursor's position.
23        && let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position)
24        // Calculate if and at what distance the ray is hitting the ground plane.
25        && let 
...
```

---

## Enum CompassQuadrant Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/enum.CompassQuadrant.html

**Contents:**
- Enum CompassQuadrant Copy item path
- Variants§
  - North
  - East
  - South
  - West
- Implementations§
  - impl CompassQuadrant
    - pub const fn from_index(index: usize) -> Option<CompassQuadrant>
    - pub const fn to_index(self) -> usize

A compass enum with 4 directions.

Corresponds to Dir2::Y and Dir2::NORTH

Corresponds to Dir2::X and Dir2::EAST

Corresponds to Dir2::NEG_X and Dir2::SOUTH

Corresponds to Dir2::NEG_Y and Dir2::WEST

Converts a standard index to a CompassQuadrant.

Starts at 0 for CompassQuadrant::North and increments clockwise.

Converts a CompassQuadrant to a standard index.

Starts at 0 for CompassQuadrant::North and increments clockwise.

Returns the opposite CompassQuadrant, located 180 degrees from self.

This can also be accessed via the - operator, using the Neg trait.

Converts a Dir2 to a CompassQuadrant in a lossy manner. Converting back to a Dir2 is not guaranteed to yield the same value.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum CompassQuadrant {
    North,
    East,
    South,
    West,
}
```

Example 2 (text):
```text
N (North)
         ▲
         │
         │
W (West) ┼─────► E (East)
         │
         │
         ▼
         S (South)
```

---

## Module u32 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/u32/index.html

**Contents:**
- Module u32 Copy item path
- Structs§
- Functions§

bevy::mathModule u32 Copy item pathSource Expand descriptionu32 vector types. Structs§UVec2A 2-dimensional vector.UVec3A 3-dimensional vector.UVec4A 4-dimensional vector.Functions§uvec2Creates a 2-dimensional vector.uvec3Creates a 3-dimensional vector.uvec4Creates a 4-dimensional vector.

---

## Trait StableInterpolate Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/trait.StableInterpolate.html

**Contents:**
- Trait StableInterpolate Copy item path
- Required Methods§
    - fn interpolate_stable(&self, other: &Self, t: f32) -> Self
- Provided Methods§
    - fn interpolate_stable_assign(&mut self, other: &Self, t: f32)
    - fn smooth_nudge(&mut self, target: &Self, decay_rate: f32, delta: f32)
      - §Example
      - Examples found in repository?
- Dyn Compatibility§
- Implementations on Foreign Types§

A type with a natural interpolation that provides strong subdivision guarantees.

Although the only required method is interpolate_stable, many things are expected of it:

The notion of interpolation should follow naturally from the semantics of the type, so that inferring the interpolation mode from the type alone is sensible.

The interpolation recovers something equivalent to the starting value at t = 0.0 and likewise with the ending value at t = 1.0. They do not have to be data-identical, but they should be semantically identical. For example, Quat::slerp doesn’t always yield its second rotation input exactly at t = 1.0, but it always returns an equivalent rotation.

Importantly, the interpolation must be subdivision-stable: for any interpolation curve between two (unnamed) values and any parameter-value pairs (t0, p) and (t1, q), the interpolation curve between p and q must be the linear reparameterization of the original interpolation curve restricted to the interval [t0, t1].

The last of these conditions is very strong and indicates something like constant speed. It is called “subdivision stability” because it guarantees that breaking up the interpolation into segments and joining them back together has no effect.

Here is a diagram depicting it:

Note that some common forms of interpolation do not satisfy this criterion. For example, Quat::lerp and Rot2::nlerp are not subdivision-stable.

Furthermore, this is not to be used as a general trait for abstract interpolation. Consumers rely on the strong guarantees in order for behavior based on this trait to be well-behaved.

Interpolate between this value and the other given value using the parameter t. At t = 0.0, a value equivalent to self is recovered, while t = 1.0 recovers a value equivalent to other, with intermediate values interpolating between the two. See the trait-level documentation for details.

A version of interpolate_stable that assigns the result to self for convenience.

Smoothly nudge this value towards the target at a given decay rate. The decay_rate parameter controls how fast the distance between self and target decays relative to the units of delta; the intended usage is for decay_rate to generally remain fixed, while delta is something like delta_time from an updating system. This produces a smooth following of the target that is independent of framerate.

More specifically, when this is called repeatedly, the result is that the distance between self and a fixed target attenuat

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub trait StableInterpolate: Clone {
    // Required method
    fn interpolate_stable(&self, other: &Self, t: f32) -> Self;

    // Provided methods
    fn interpolate_stable_assign(&mut self, other: &Self, t: f32) { ... }
    fn smooth_nudge(&mut self, target: &Self, decay_rate: f32, delta: f32) { ... }
}
```

Example 2 (text):
```text
top curve = u.interpolate_stable(v, t)

             t0 => p   t1 => q    
  |-------------|---------|-------------|
0 => u         /           \          1 => v
             /               \
           /                   \
         /        linear         \
       /     reparameterization    \
     /   t = t0 * (1 - s) + t1 * s   \
   /                                   \
  |-------------------------------------|
0 => p                                1 => q

bottom curve = p.interpolate_stable(q, s)
```

Example 3 (javascript):
```javascript
let mut object_position: Vec3 = Vec3::ZERO;
let target_position: Vec3 = Vec3::new(2.0, 3.0, 5.0);
// Decay rate of ln(10) => after 1 second, remaining distance is 1/10th
let decay_rate = f32::ln(10.0);
// Calling this repeatedly will move `object_position` towards `target_position`:
object_position.smooth_nudge(&target_position, decay_rate, delta_time);
```

Example 4 (javascript):
```javascript
140fn rotate_ship(ship: Single<(&mut Ship, &mut Transform)>, time: Res<Time>) {
141    let (mut ship, mut ship_transform) = ship.into_inner();
142
143    if !ship.in_motion {
144        return;
145    }
146
147    let target_rotation = ship.target_transform.rotation;
148
149    ship_transform
150        .rotation
151        .smooth_nudge(&target_rotation, 3.0, time.delta_secs());
152
153    if ship_transform.rotation.angle_between(target_rotation) <= f32::EPSILON {
154        ship.in_motion = false;
155    }
156}
```

---

## Function u64vec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.u64vec2.html

**Contents:**
- Function u64vec2 Copy item path

Creates a 2-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn u64vec2(x: u64, y: u64) -> U64Vec2
```

---

## Struct Dir3A Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Dir3A.html

**Contents:**
- Struct Dir3A Copy item path
- Implementations§
  - impl Dir3A
    - pub const X: Dir3A
    - pub const Y: Dir3A
    - pub const Z: Dir3A
    - pub const NEG_X: Dir3A
    - pub const NEG_Y: Dir3A
    - pub const NEG_Z: Dir3A
    - pub const AXES: [Dir3A; 3]

A normalized SIMD vector pointing in a direction in 3D space.

This type stores a 16 byte aligned Vec3A. This may or may not be faster than Dir3: make sure to benchmark!

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

A unit vector pointing along the negative Z axis.

The directional axes.

Create a direction from a finite, nonzero Vec3A, normalizing it.

Returns Err(InvalidDirectionError) if the length of the given vector is zero (or very close to zero), infinite, or NaN.

Create a Dir3A from a Vec3A that is already normalized.

value must be normalized, i.e its length must be 1.0.

Create a direction from a finite, nonzero Vec3A, normalizing it and also returning its original length.

Returns Err(InvalidDirectionError) if the length of the given vector is zero (or very close to zero), infinite, or NaN.

Create a direction from its x, y, and z components.

Returns Err(InvalidDirectionError) if the length of the vector formed by the components is zero (or very close to zero), infinite, or NaN.

Create a direction from its x, y, and z components, assuming the resulting vector is normalized.

The vector produced from x, y, and z must be normalized, i.e its length must be 1.0.

Returns the inner Vec3A

Performs a spherical linear interpolation between self and rhs based on the value s.

This corresponds to interpolating between the two directions at a constant angular velocity.

When s == 0.0, the result will be equal to self. When s == 1.0, the result will be equal to rhs.

Returns self after an approximate normalization, assuming the value is already nearly normalized. Useful for preventing numerical error accumulation.

See Dir3::fast_renormalize for an example of when such error accumulation might occur.

Converts self to [x, y, z]

Moves towards rhs based on the value d.

When d is 0.0, the result will be equal to self. When d is equal to self.distance(rhs), the result will be equal to rhs. Will not go past rhs.

Returns some vector that is orthogonal to the given one.

The input vector must be finite and non-zero.

The output vector is not necessarily unit length. For that use Self::any_orthonormal_vector() instead.

Returns any unit vector that is orthogonal to the given one.

The input vector must be unit length.

Will panic if self is not normali

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Dir3A(/* private fields */);
```

Example 2 (javascript):
```javascript
let dir1 = Dir3A::X;
let dir2 = Dir3A::Y;

let result1 = dir1.slerp(dir2, 1.0 / 3.0);
#[cfg(feature = "approx")]
assert_relative_eq!(
    result1,
    Dir3A::from_xyz(0.75_f32.sqrt(), 0.5, 0.0).unwrap(),
    epsilon = 0.000001
);

let result2 = dir1.slerp(dir2, 0.5);
#[cfg(feature = "approx")]
assert_relative_eq!(result2, Dir3A::from_xyz(0.5_f32.sqrt(), 0.5_f32.sqrt(), 0.0).unwrap());
```

---

## Struct Rect Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Rect.html

**Contents:**
- Struct Rect Copy item path
- Fields§
- Implementations§
  - impl Rect
    - pub const EMPTY: Rect
    - pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Rect
      - §Examples
    - pub fn from_corners(p0: Vec2, p1: Vec2) -> Rect
      - §Examples
    - pub fn from_center_size(origin: Vec2, size: Vec2) -> Rect

A rectangle defined by two opposite corners.

The rectangle is axis aligned, and defined by its minimum and maximum coordinates, stored in Rect::min and Rect::max, respectively. The minimum/maximum invariant must be upheld by the user when directly assigning the fields, otherwise some methods produce invalid results. It is generally recommended to use one of the constructor methods instead, which will ensure this invariant is met, unless you already have the minimum and maximum corners.

The minimum corner point of the rect.

The maximum corner point of the rect.

An empty Rect, represented by maximum and minimum corner points at Vec2::NEG_INFINITY and Vec2::INFINITY, respectively. This is so the Rect has a infinitely negative size. This is useful, because when taking a union B of a non-empty Rect A and this empty Rect, B will simply equal A.

Create a new rectangle from two corner points.

The two points do not need to be the minimum and/or maximum corners. They only need to be two opposite corners.

Create a new rectangle from two corner points.

The two points do not need to be the minimum and/or maximum corners. They only need to be two opposite corners.

Create a new rectangle from its center and size.

This method panics if any of the components of the size is negative.

Create a new rectangle from its center and half-size.

This method panics if any of the components of the half-size is negative.

Check if the rectangle is empty.

Rectangle width (max.x - min.x).

Rectangle height (max.y - min.y).

The center point of the rectangle.

Check if a point lies within this rectangle, inclusive of its edges.

Build a new rectangle formed of the union of this rectangle and another rectangle.

The union is the smallest rectangle enclosing both rectangles.

Build a new rectangle formed of the union of this rectangle and a point.

The union is the smallest rectangle enclosing both the rectangle and the point. If the point is already inside the rectangle, this method returns a copy of the rectangle.

Build a new rectangle formed of the intersection of this rectangle and another rectangle.

The intersection is the largest rectangle enclosed in both rectangles. If the intersection is empty, this method returns an empty rectangle (Rect::is_empty() returns true), but the actual values of Rect::min and Rect::max are implementation-dependent.

Create a new rectangle by expanding it evenly on all sides.

A positive expansion value produces a larger rectangle, while a 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}
```

Example 2 (javascript):
```javascript
let r = Rect::new(0., 4., 10., 6.); // w=10 h=2
let r = Rect::new(2., 3., 5., -1.); // w=3 h=4
```

Example 3 (javascript):
```javascript
// Unit rect from [0,0] to [1,1]
let r = Rect::from_corners(Vec2::ZERO, Vec2::ONE); // w=1 h=1
// Same; the points do not need to be ordered
let r = Rect::from_corners(Vec2::ONE, Vec2::ZERO); // w=1 h=1
```

Example 4 (javascript):
```javascript
let r = Rect::from_center_size(Vec2::ZERO, Vec2::ONE); // w=1 h=1
assert!(r.min.abs_diff_eq(Vec2::splat(-0.5), 1e-5));
assert!(r.max.abs_diff_eq(Vec2::splat(0.5), 1e-5));
```

---

## Function vec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.vec3.html

**Contents:**
- Function vec3 Copy item path
      - Examples found in repository?

Creates a 3-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3
```

Example 2 (javascript):
```javascript
15const CAMERA_FOCAL_POINT: Vec3 = vec3(0.0, 0.3, 0.0);
```

Example 3 (javascript):
```javascript
14const CAMERA_INITIAL_POSITION: Vec3 = vec3(-0.4, 0.0, 0.0);
15
16/// The current settings of the app, as chosen by the user.
17#[derive(Resource)]
18struct AppStatus {
19    /// Which type of light is in the scene.
20    light_mode: LightMode,
21    /// Whether anisotropy is enabled.
22    anisotropy_enabled: bool,
23    /// Which mesh is visible
24    visible_scene: Scene,
25}
26
27/// Which type of light we're using: a directional light, a point light, or an
28/// environment map.
29#[derive(Clone, Copy, PartialEq, Default)]
30enum LightMode {
31    /// A rotating directional light.
32    
...
```

Example 4 (javascript):
```javascript
115const INITIAL_SPHERE_POSITION: Vec3 = vec3(0.0, 0.5233223, 0.0);
116
117fn main() {
118    App::new()
119        .add_plugins(DefaultPlugins.set(WindowPlugin {
120            primary_window: Some(Window {
121                title: "Bevy Mixed Lighting Example".into(),
122                ..default()
123            }),
124            ..default()
125        }))
126        .add_plugins(MeshPickingPlugin)
127        .insert_resource(AmbientLight {
128            color: ClearColor::default().0,
129            brightness: 10000.0,
130            affects_lightmapped_meshes: true,
131        })
132 
...
```

---

## Function dvec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.dvec2.html

**Contents:**
- Function dvec2 Copy item path

Creates a 2-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn dvec2(x: f64, y: f64) -> DVec2
```

---

## Struct Quat Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Quat.html

**Contents:**
- Struct Quat Copy item path
- Implementations§
  - impl Quat
    - pub const IDENTITY: Quat
    - pub const NAN: Quat
    - pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Quat
      - §Preconditions
      - Examples found in repository?
    - pub const fn from_array(a: [f32; 4]) -> Quat
      - §Preconditions

A quaternion representing an orientation.

This quaternion is intended to be of unit length but may denormalize due to floating point “error creep” which can occur when successive quaternion operations are applied.

SIMD vector types are used for storage on supported platforms.

This type is 16 byte aligned.

The identity quaternion. Corresponds to no rotation.

Creates a new rotation quaternion.

This should generally not be called manually unless you know what you are doing. Use one of the other constructors instead such as identity or from_axis_angle.

from_xyzw is mostly used by unit tests and serde deserialization.

This function does not check if the input is normalized, it is up to the user to provide normalized input or to normalized the resulting quaternion.

Creates a rotation quaternion from an array.

This function does not check if the input is normalized, it is up to the user to provide normalized input or to normalized the resulting quaternion.

Creates a new rotation quaternion from a 4D vector.

This function does not check if the input is normalized, it is up to the user to provide normalized input or to normalized the resulting quaternion.

Creates a rotation quaternion from a slice.

This function does not check if the input is normalized, it is up to the user to provide normalized input or to normalized the resulting quaternion.

Panics if slice length is less than 4.

Writes the quaternion to an unaligned slice.

Panics if slice length is less than 4.

Create a quaternion for a normalized rotation axis and angle (in radians).

The axis must be a unit vector.

Will panic if axis is not normalized when glam_assert is enabled.

Create a quaternion that rotates v.length() radians around v.normalize().

from_scaled_axis(Vec3::ZERO) results in the identity quaternion.

Creates a quaternion from the angle (in radians) around the x axis.

Creates a quaternion from the angle (in radians) around the y axis.

Creates a quaternion from the angle (in radians) around the z axis.

Creates a quaternion from the given Euler rotation sequence and the angles (in radians).

Creates a quaternion from a 3x3 rotation matrix.

Note if the input matrix contain scales, shears, or other non-rotation transformations then the resulting quaternion will be ill-defined.

Will panic if any input matrix column is not normalized when glam_assert is enabled.

Creates a quaternion from a 3x3 SIMD aligned rotation matrix.

Note if the input matrix contain scales, shears, 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Quat(/* private fields */);
```

Example 2 (javascript):
```javascript
50const TRANSFORM_3D: Transform = Transform {
51    translation: Vec3::ZERO,
52    // The camera is pointing at the 3D shape
53    rotation: Quat::from_xyzw(-0.14521316, -0.0, -0.0, 0.98940045),
54    scale: Vec3::ONE,
55};
```

Example 3 (javascript):
```javascript
164const CAMERA_POSITIONS: &[Transform] = &[
165    Transform {
166        translation: Vec3::new(1.5, 1.5, 1.5),
167        rotation: Quat::from_xyzw(-0.279, 0.364, 0.115, 0.880),
168        scale: Vec3::ONE,
169    },
170    Transform {
171        translation: Vec3::new(2.4, 0.0, 0.2),
172        rotation: Quat::from_xyzw(0.094, 0.676, 0.116, 0.721),
173        scale: Vec3::ONE,
174    },
175    Transform {
176        translation: Vec3::new(2.4, 2.6, -4.3),
177        rotation: Quat::from_xyzw(0.170, 0.908, 0.308, 0.225),
178        scale: Vec3::ONE,
179    },
180    Transform {
181        t
...
```

Example 4 (javascript):
```javascript
58fn setup(
59    mut commands: Commands,
60    asset_server: Res<AssetServer>,
61    args: Res<Args>,
62    #[cfg(all(feature = "dlss", not(feature = "force_disable_dlss")))] dlss_rr_supported: Option<
63        Res<DlssRayReconstructionSupported>,
64    >,
65) {
66    commands
67        .spawn((
68            SceneRoot(
69                asset_server.load(
70                    GltfAssetLabel::Scene(0)
71                        .from_asset("https://github.com/bevyengine/bevy_asset_files/raw/2a5950295a8b6d9d051d59c0df69e87abcda58c3/pica_pica/mini_diorama_01.glb")
72                ),
73      
...
```

---

## Module f64 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/f64/index.html

**Contents:**
- Module f64 Copy item path
- Structs§
- Functions§

f64 vector, quaternion and matrix types.

---

## Enum CompassOctant Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/enum.CompassOctant.html

**Contents:**
- Enum CompassOctant Copy item path
- Variants§
  - North
  - NorthEast
  - East
  - SouthEast
  - South
  - SouthWest
  - West
  - NorthWest

A compass enum with 8 directions.

Corresponds to Dir2::Y and Dir2::NORTH

Corresponds to Dir2::NORTH_EAST

Corresponds to Dir2::X and Dir2::EAST

Corresponds to Dir2::SOUTH_EAST

Corresponds to Dir2::NEG_X and Dir2::SOUTH

Corresponds to Dir2::SOUTH_WEST

Corresponds to Dir2::NEG_Y and Dir2::WEST

Corresponds to Dir2::NORTH_WEST

Converts a standard index to a CompassOctant.

Starts at 0 for CompassOctant::North and increments clockwise.

Converts a CompassOctant to a standard index.

Starts at 0 for CompassOctant::North and increments clockwise.

Returns the opposite CompassOctant, located 180 degrees from self.

This can also be accessed via the - operator, using the Neg trait.

Converts a Dir2 to a CompassOctant in a lossy manner. Converting back to a Dir2 is not guaranteed to yield the same value.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum CompassOctant {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
```

Example 2 (text):
```text
N (North)
         ▲
    NW   │   NE
       ╲ │ ╱
W (West) ┼─────► E (East)
       ╱ │ ╲
    SW   │   SE
         ▼
         S (South)
```

---

## Trait Vec2Swizzles Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/trait.Vec2Swizzles.html

**Contents:**
- Trait Vec2Swizzles Copy item path
- Required Associated Types§
    - type Vec3
    - type Vec4
- Required Methods§
    - fn xx(self) -> Self
    - fn yx(self) -> Self
    - fn yy(self) -> Self
    - fn xxx(self) -> Self::Vec3
    - fn xxy(self) -> Self::Vec3

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait Vec2Swizzles:
    Sized
    + Copy
    + Clone {
    type Vec3;
    type Vec4;

Show 28 methods    // Required methods
    fn xx(self) -> Self;
    fn yx(self) -> Self;
    fn yy(self) -> Self;
    fn xxx(self) -> Self::Vec3;
    fn xxy(self) -> Self::Vec3;
    fn xyx(self) -> Self::Vec3;
    fn xyy(self) -> Self::Vec3;
    fn yxx(self) -> Self::Vec3;
    fn yxy(self) -> Self::Vec3;
    fn yyx(self) -> Self::Vec3;
    fn yyy(self) -> Self::Vec3;
    fn xxxx(self) -> Self::Vec4;
    fn xxxy(self) -> Self::Vec4;
    fn xxyx(self) -> Self::Vec4;
    fn xxyy(self) -> Self::Vec4;
    fn x
...
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

---

## Struct USizeVec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.USizeVec2.html

**Contents:**
- Struct USizeVec2 Copy item path
- Fields§
- Implementations§
  - impl USizeVec2
    - pub const ZERO: USizeVec2
    - pub const ONE: USizeVec2
    - pub const MIN: USizeVec2
    - pub const MAX: USizeVec2
    - pub const X: USizeVec2
    - pub const Y: USizeVec2

A 2-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y]

Creates a vector from the first 2 values in slice.

Panics if slice is less than 2 elements long.

Writes the elements of self to the first 2 elements in slice.

Panics if slice is less than 2 elements long.

Creates a 3D vector from self and the given z value.

Creates a 2D vector from self with the given value of x.

Creates a 2D vector from self with the given value of y.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to usize::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes [self.x == rhs.x, self.y == rhs.y, ..] for all elements.

Returns a vector mask containing the result of a != comparison for each element of self and rhs.

In other words this computes [self.x != rhs.x, self.y != rhs.y, ..] for all elements.

Returns a vector mask containing the result of a >= comparison f

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct USizeVec2 {
    pub x: usize,
    pub y: usize,
}
```

---

## Module i64 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/i64/index.html

**Contents:**
- Module i64 Copy item path
- Structs§
- Functions§

bevy::mathModule i64 Copy item pathSource Expand descriptioni64 vector types. Structs§I64Vec2A 2-dimensional vector.I64Vec3A 3-dimensional vector.I64Vec4A 4-dimensional vector.Functions§i64vec2Creates a 2-dimensional vector.i64vec3Creates a 3-dimensional vector.i64vec4Creates a 4-dimensional vector.

---

## Module usize Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/usize/index.html

**Contents:**
- Module usize Copy item path
- Structs§
- Functions§

bevy::mathModule usize Copy item pathSource Expand descriptionusize vector types. Structs§USizeVec2A 2-dimensional vector.USizeVec3A 3-dimensional vector.USizeVec4A 4-dimensional vector.Functions§usizevec2Creates a 2-dimensional vector.usizevec3Creates a 3-dimensional vector.usizevec4Creates a 4-dimensional vector.

---

## Function dmat3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.dmat3.html

**Contents:**
- Function dmat3 Copy item path

Creates a 3x3 matrix from three column vectors.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn dmat3(x_axis: DVec3, y_axis: DVec3, z_axis: DVec3) -> DMat3
```

---

## Struct DAffine2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.DAffine2.html

**Contents:**
- Struct DAffine2 Copy item path
- Fields§
- Implementations§
  - impl DAffine2
    - pub const ZERO: DAffine2
    - pub const IDENTITY: DAffine2
    - pub const NAN: DAffine2
    - pub const fn from_cols(x_axis: DVec2, y_axis: DVec2, z_axis: DVec2) -> DAffine2
    - pub fn from_cols_array(m: &[f64; 6]) -> DAffine2
    - pub fn to_cols_array(&self) -> [f64; 6]

A 2D affine transform, which can represent translation, rotation, scaling and shear.

The degenerate zero transform.

This transforms any finite vector and point to zero. The zero transform is non-invertible.

The identity transform.

Multiplying a vector with this returns the same vector.

Creates an affine transform from three column vectors.

Creates an affine transform from a [f64; 6] array stored in column major order.

Creates a [f64; 6] array storing data in column major order.

Creates an affine transform from a [[f64; 2]; 3] 2D array stored in column major order. If your data is in row major order you will need to transpose the returned matrix.

Creates a [[f64; 2]; 3] 2D array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates an affine transform from the first 6 values in slice.

Panics if slice is less than 6 elements long.

Writes the columns of self to the first 6 elements in slice.

Panics if slice is less than 6 elements long.

Creates an affine transform that changes scale. Note that if any scale is zero the transform will be non-invertible.

Creates an affine transform from the given rotation angle.

Creates an affine transformation from the given 2D translation.

Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)

Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation) and a translation vector.

Equivalent to DAffine2::from_translation(translation) * DAffine2::from_mat2(mat2)

Creates an affine transform from the given 2D scale, rotation angle (in radians) and translation.

Equivalent to DAffine2::from_translation(translation) * DAffine2::from_angle(angle) * DAffine2::from_scale(scale)

Creates an affine transform from the given 2D rotation angle (in radians) and translation.

Equivalent to DAffine2::from_translation(translation) * DAffine2::from_angle(angle)

The given DMat3 must be an affine transform,

Extracts scale, angle and translation from self.

The transform is expected to be non-degenerate and without shearing, or the output will be invalid.

Will panic if the determinant self.matrix2 is zero or if the resulting scale vector contains any zero elements when glam_assert is enabled.

Transforms the given 2D point, applying shear, scale, rotation and translation.

Transforms the given 2D vector, applying shear, scale and rotation (but NOT translation).

To also apply translation, use Self::transform_point2() 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct DAffine2 {
    pub matrix2: DMat2,
    pub translation: DVec2,
}
```

---

## Function i8vec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.i8vec4.html

**Contents:**
- Function i8vec4 Copy item path

Creates a 4-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn i8vec4(x: i8, y: i8, z: i8, w: i8) -> I8Vec4
```

---

## Function u16vec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.u16vec3.html

**Contents:**
- Function u16vec3 Copy item path

Creates a 3-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn u16vec3(x: u16, y: u16, z: u16) -> U16Vec3
```

---

## Trait FromRng Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/trait.FromRng.html

**Contents:**
- Trait FromRng Copy item path
- Provided Methods§
    - fn from_rng<R>(rng: &mut R) -> Selfwhere R: Rng + ?Sized,
- Dyn Compatibility§
- Implementors§
  - impl FromRng for Dir2
  - impl FromRng for Dir3
  - impl FromRng for Dir3A
  - impl FromRng for Quat
  - impl FromRng for Rot2

Ergonomics trait for a type with a StandardUniform distribution, allowing values to be generated uniformly from an Rng by a method in its own namespace.

Construct a value of this type uniformly at random using rng as the source of randomness.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait FromRng: Sizedwhere
    StandardUniform: Distribution<Self>,{
    // Provided method
    fn from_rng<R>(rng: &mut R) -> Self
       where R: Rng + ?Sized { ... }
}
```

Example 2 (javascript):
```javascript
let mut rng = StdRng::seed_from_u64(451);
let random_dir = Dir3::from_rng(&mut rng);
```

---

## Function mat3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.mat3.html

**Contents:**
- Function mat3 Copy item path

Creates a 3x3 matrix from three column vectors.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn mat3(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Mat3
```

---

## Trait ToExtents Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/trait.ToExtents.html

**Contents:**
- Trait ToExtents Copy item path
- Required Methods§
    - fn to_extents(self) -> Extent3d
- Implementors§
  - impl ToExtents for UVec2
  - impl ToExtents for UVec3

bevy::imageTrait ToExtents Copy item pathSource pub trait ToExtents { // Required method fn to_extents(self) -> Extent3d; }Required Methods§Sourcefn to_extents(self) -> Extent3dImplementors§Source§impl ToExtents for UVec2Source§impl ToExtents for UVec3

**Examples:**

Example 1 (unknown):
```unknown
pub trait ToExtents {
    // Required method
    fn to_extents(self) -> Extent3d;
}
```

---

## Struct Ray2d Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Ray2d.html

**Contents:**
- Struct Ray2d Copy item path
- Fields§
- Implementations§
  - impl Ray2d
    - pub const fn new(origin: Vec2, direction: Dir2) -> Ray2d
    - pub fn get_point(&self, distance: f32) -> Vec2
    - pub fn intersect_plane(&self, plane_origin: Vec2, plane: Plane2d) -> Option<f32>
- Trait Implementations§
  - impl Clone for Ray2d
    - fn clone(&self) -> Ray2d

An infinite half-line starting at origin and going in direction in 2D space.

The origin of the ray.

The direction of the ray.

Create a new Ray2d from a given origin and direction

Get a point at a given distance along the ray

Get the distance to a plane if the ray intersects it

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Ray2d {
    pub origin: Vec2,
    pub direction: Dir2,
}
```

---

## Struct FloatOrd Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.FloatOrd.html

**Contents:**
- Struct FloatOrd Copy item path
- Tuple Fields§
- Trait Implementations§
  - impl Clone for FloatOrd
    - fn clone(&self) -> FloatOrd
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for FloatOrd
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl FromArg for FloatOrd
    - type This<'from_arg> = FloatOrd

A wrapper for floats that implements Ord, Eq, and Hash traits.

This is a work around for the fact that the IEEE 754-2008 standard, implemented by Rust’s f32 type, doesn’t define an ordering for NaN, and NaN is not considered equal to any other NaN.

Wrapping a float with FloatOrd breaks conformance with the standard by sorting NaN as less than all other numbers and equal to any other NaN.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct FloatOrd(pub f32);
```

---

## Trait ShapeSample Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/trait.ShapeSample.html

**Contents:**
- Trait ShapeSample Copy item path
- Required Associated Types§
    - type Output
- Required Methods§
    - fn sample_interior<R>(&self, rng: &mut R) -> Self::Outputwhere R: Rng + ?Sized,
      - §Example
    - fn sample_boundary<R>(&self, rng: &mut R) -> Self::Outputwhere R: Rng + ?Sized,
      - §Example
- Provided Methods§
    - fn interior_dist(self) -> impl Distribution<Self::Output>where Self: Sized,

Exposes methods to uniformly sample a variety of primitive shapes.

The type of vector returned by the sample methods, Vec2 for 2D shapes and Vec3 for 3D shapes.

Uniformly sample a point from inside the area/volume of this shape, centered on 0.

Shapes like Cylinder, Capsule2d and Capsule3d are oriented along the y-axis.

Uniformly sample a point from the surface of this shape, centered on 0.

Shapes like Cylinder, Capsule2d and Capsule3d are oriented along the y-axis.

Extract a Distribution whose samples are points of this shape’s interior, taken uniformly.

Extract a Distribution whose samples are points of this shape’s boundary, taken uniformly.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait ShapeSample {
    type Output;

    // Required methods
    fn sample_interior<R>(&self, rng: &mut R) -> Self::Output
       where R: Rng + ?Sized;
    fn sample_boundary<R>(&self, rng: &mut R) -> Self::Output
       where R: Rng + ?Sized;

    // Provided methods
    fn interior_dist(self) -> impl Distribution<Self::Output>
       where Self: Sized { ... }
    fn boundary_dist(self) -> impl Distribution<Self::Output>
       where Self: Sized { ... }
}
```

Example 2 (javascript):
```javascript
let square = Rectangle::new(2.0, 2.0);

// Returns a Vec2 with both x and y between -1 and 1.
println!("{}", square.sample_interior(&mut rand::rng()));
```

Example 3 (javascript):
```javascript
let square = Rectangle::new(2.0, 2.0);

// Returns a Vec2 where one of the coordinates is at ±1,
//  and the other is somewhere between -1 and 1.
println!("{}", square.sample_boundary(&mut rand::rng()));
```

Example 4 (javascript):
```javascript
let square = Rectangle::new(2.0, 2.0);
let rng = rand::rng();

// Iterate over points randomly drawn from `square`'s interior:
for random_val in square.interior_dist().sample_iter(rng).take(5) {
    println!("{}", random_val);
}
```

---

## Function bvec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.bvec4.html

**Contents:**
- Function bvec4 Copy item path

Creates a 4-dimensional bool vector mask.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn bvec4(x: bool, y: bool, z: bool, w: bool) -> BVec4
```

---

## Struct U8Vec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.U8Vec4.html

**Contents:**
- Struct U8Vec4 Copy item path
- Fields§
- Implementations§
  - impl U8Vec4
    - pub const ZERO: U8Vec4
    - pub const ONE: U8Vec4
    - pub const MIN: U8Vec4
    - pub const MAX: U8Vec4
    - pub const X: U8Vec4
    - pub const Y: U8Vec4

A 4-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the positive W axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z, w]

Creates a vector from the first 4 values in slice.

Panics if slice is less than 4 elements long.

Writes the elements of self to the first 4 elements in slice.

Panics if slice is less than 4 elements long.

Creates a 3D vector from the x, y and z elements of self, discarding w.

Truncation to U8Vec3 may also be performed by using self.xyz().

Creates a 4D vector from self with the given value of x.

Creates a 4D vector from self with the given value of y.

Creates a 4D vector from self with the given value of z.

Creates a 4D vector from self with the given value of w.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to u8::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes [s

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct U8Vec4 {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub w: u8,
}
```

---

## Struct I64Vec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.I64Vec4.html

**Contents:**
- Struct I64Vec4 Copy item path
- Fields§
- Implementations§
  - impl I64Vec4
    - pub const ZERO: I64Vec4
    - pub const ONE: I64Vec4
    - pub const NEG_ONE: I64Vec4
    - pub const MIN: I64Vec4
    - pub const MAX: I64Vec4
    - pub const X: I64Vec4

A 4-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the positive W axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

A unit vector pointing along the negative Z axis.

A unit vector pointing along the negative W axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z, w]

Creates a vector from the first 4 values in slice.

Panics if slice is less than 4 elements long.

Writes the elements of self to the first 4 elements in slice.

Panics if slice is less than 4 elements long.

Creates a 3D vector from the x, y and z elements of self, discarding w.

Truncation to I64Vec3 may also be performed by using self.xyz().

Creates a 4D vector from self with the given value of x.

Creates a 4D vector from self with the given value of y.

Creates a 4D vector from self with the given value of z.

Creates a 4D vector from self with the given value of w.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to i64::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct I64Vec4 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub w: i64,
}
```

---

## Function usizevec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.usizevec4.html

**Contents:**
- Function usizevec4 Copy item path

Creates a 4-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn usizevec4(x: usize, y: usize, z: usize, w: usize) -> USizeVec4
```

---

## Function bvec4a Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.bvec4a.html

**Contents:**
- Function bvec4a Copy item path

Creates a 4-dimensional bool vector mask.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn bvec4a(x: bool, y: bool, z: bool, w: bool) -> BVec4A
```

---

## Struct I8Vec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.I8Vec2.html

**Contents:**
- Struct I8Vec2 Copy item path
- Fields§
- Implementations§
  - impl I8Vec2
    - pub const ZERO: I8Vec2
    - pub const ONE: I8Vec2
    - pub const NEG_ONE: I8Vec2
    - pub const MIN: I8Vec2
    - pub const MAX: I8Vec2
    - pub const X: I8Vec2

A 2-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y]

Creates a vector from the first 2 values in slice.

Panics if slice is less than 2 elements long.

Writes the elements of self to the first 2 elements in slice.

Panics if slice is less than 2 elements long.

Creates a 3D vector from self and the given z value.

Creates a 2D vector from self with the given value of x.

Creates a 2D vector from self with the given value of y.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to i8::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes [self.x == rhs.x, self.y == rhs.y, ..] for all elements.

Returns a vector mask containing the result of a != comparison for each element of self and rhs.

In other words this computes [self.x != rhs.x, self.

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct I8Vec2 {
    pub x: i8,
    pub y: i8,
}
```

---

## Struct AspectRatio Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.AspectRatio.html

**Contents:**
- Struct AspectRatio Copy item path
- Implementations§
  - impl AspectRatio
    - pub const SIXTEEN_NINE: AspectRatio
    - pub const FOUR_THREE: AspectRatio
    - pub const ULTRAWIDE: AspectRatio
    - pub const fn try_new( width: f32, height: f32, ) -> Result<AspectRatio, AspectRatioError>
      - §Errors
    - pub const fn try_from_pixels( x: u32, y: u32, ) -> Result<AspectRatio, AspectRatioError>
    - pub const fn ratio(&self) -> f32

An AspectRatio is the ratio of width to height.

Standard 16:9 aspect ratio

Standard 4:3 aspect ratio

Standard 21:9 ultrawide aspect ratio

Attempts to create a new AspectRatio from a given width and height.

Returns an Err with AspectRatioError if:

Attempts to create a new AspectRatio from a given amount of x pixels and y pixels.

Returns the aspect ratio as a f32 value.

Returns the inverse of this aspect ratio (height/width).

Returns true if the aspect ratio represents a landscape orientation.

Returns true if the aspect ratio represents a portrait orientation.

Returns true if the aspect ratio is exactly square.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AspectRatio(/* private fields */);
```

---

## Module i32 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/i32/index.html

**Contents:**
- Module i32 Copy item path
- Structs§
- Functions§

bevy::mathModule i32 Copy item pathSource Expand descriptioni32 vector types. Structs§IVec2A 2-dimensional vector.IVec3A 3-dimensional vector.IVec4A 4-dimensional vector.Functions§ivec2Creates a 2-dimensional vector.ivec3Creates a 3-dimensional vector.ivec4Creates a 4-dimensional vector.

---

## Struct DMat3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.DMat3.html

**Contents:**
- Struct DMat3 Copy item path
- Fields§
- Implementations§
  - impl DMat3
    - pub const ZERO: DMat3
    - pub const IDENTITY: DMat3
    - pub const NAN: DMat3
    - pub const fn from_cols(x_axis: DVec3, y_axis: DVec3, z_axis: DVec3) -> DMat3
    - pub const fn from_cols_array(m: &[f64; 9]) -> DMat3
    - pub const fn to_cols_array(&self) -> [f64; 9]

A 3x3 column major matrix.

This 3x3 matrix type features convenience methods for creating and using linear and affine transformations. If you are primarily dealing with 2D affine transformations the DAffine2 type is much faster and more space efficient than using a 3x3 matrix.

Linear transformations including 3D rotation and scale can be created using methods such as Self::from_diagonal(), Self::from_quat(), Self::from_axis_angle(), Self::from_rotation_x(), Self::from_rotation_y(), or Self::from_rotation_z().

The resulting matrices can be use to transform 3D vectors using regular vector multiplication.

Affine transformations including 2D translation, rotation and scale can be created using methods such as Self::from_translation(), Self::from_angle(), Self::from_scale() and Self::from_scale_angle_translation().

The Self::transform_point2() and Self::transform_vector2() convenience methods are provided for performing affine transforms on 2D vectors and points. These multiply 2D inputs as 3D vectors with an implicit z value of 1 for points and 0 for vectors respectively. These methods assume that Self contains a valid affine transform.

A 3x3 matrix with all elements set to 0.0.

A 3x3 identity matrix, where all diagonal elements are 1, and all off-diagonal elements are 0.

Creates a 3x3 matrix from three column vectors.

Creates a 3x3 matrix from a [f64; 9] array stored in column major order. If your data is stored in row major you will need to transpose the returned matrix.

Creates a [f64; 9] array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates a 3x3 matrix from a [[f64; 3]; 3] 3D array stored in column major order. If your data is in row major order you will need to transpose the returned matrix.

Creates a [[f64; 3]; 3] 3D array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates a 3x3 matrix with its diagonal set to diagonal and all other entries set to 0.

Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column.

Creates a 3x3 matrix from the minor of the given 4x4 matrix, discarding the ith column and jth row.

Panics if i or j is greater than 3.

Creates a 3D rotation matrix from the given quaternion.

Will panic if rotation is not normalized when glam_assert is enabled.

Creates a 3D rotation matrix from a normalized rotation axis and angle (in radians).

Will panic if axis is not normalized when glam_as

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct DMat3 {
    pub x_axis: DVec3,
    pub y_axis: DVec3,
    pub z_axis: DVec3,
}
```

---

## Function vec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.vec2.html

**Contents:**
- Function vec2 Copy item path
      - Examples found in repository?

Creates a 2-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn vec2(x: f32, y: f32) -> Vec2
```

Example 2 (javascript):
```javascript
86static LIGHTMAPS: [(&str, Rect); 5] = [
87    (
88        "Plane",
89        uv_rect_opengl(Vec2::splat(0.026), Vec2::splat(0.710)),
90    ),
91    (
92        "SheenChair_fabric",
93        uv_rect_opengl(vec2(0.7864, 0.02377), vec2(0.1910, 0.1912)),
94    ),
95    (
96        "SheenChair_label",
97        uv_rect_opengl(vec2(0.275, -0.016), vec2(0.858, 0.486)),
98    ),
99    (
100        "SheenChair_metal",
101        uv_rect_opengl(vec2(0.998, 0.506), vec2(-0.029, -0.067)),
102    ),
103    (
104        "SheenChair_wood",
105        uv_rect_opengl(vec2(0.787, 0.257), vec2(0.179, 0.177)),
...
```

Example 3 (javascript):
```javascript
35fn setup(mut commands: Commands) {
36    // Initialize the modes with their defaults:
37    let spline_mode = SplineMode::default();
38    commands.insert_resource(spline_mode);
39    let cycling_mode = CyclingMode::default();
40    commands.insert_resource(cycling_mode);
41
42    // Starting data for [`ControlPoints`]:
43    let default_points = vec![
44        vec2(-500., -200.),
45        vec2(-250., 250.),
46        vec2(250., 250.),
47        vec2(500., -200.),
48    ];
49
50    let default_tangents = vec![
51        vec2(0., 200.),
52        vec2(200., 0.),
53        vec2(0., -200.),
5
...
```

Example 4 (javascript):
```javascript
32fn setup(
33    mut commands: Commands,
34    mut meshes: ResMut<Assets<Mesh>>,
35    mut materials: ResMut<Assets<StandardMaterial>>,
36    mut compensation_curves: ResMut<Assets<AutoExposureCompensationCurve>>,
37    asset_server: Res<AssetServer>,
38) {
39    let metering_mask = asset_server.load("textures/basic_metering_mask.png");
40
41    commands.spawn((
42        Camera3d::default(),
43        Transform::from_xyz(1.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
44        AutoExposure {
45            metering_mask: metering_mask.clone(),
46            ..default()
47        },
48       
...
```

---

## Function i64vec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.i64vec4.html

**Contents:**
- Function i64vec4 Copy item path

Creates a 4-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn i64vec4(x: i64, y: i64, z: i64, w: i64) -> I64Vec4
```

---

## Function i64vec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.i64vec3.html

**Contents:**
- Function i64vec3 Copy item path

Creates a 3-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn i64vec3(x: i64, y: i64, z: i64) -> I64Vec3
```

---

## Function quat Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.quat.html

**Contents:**
- Function quat Copy item path

Creates a quaternion from x, y, z and w values.

This should generally not be called manually unless you know what you are doing. Use one of the other constructors instead such as identity or from_axis_angle.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn quat(x: f32, y: f32, z: f32, w: f32) -> Quat
```

---

## Function vec3a Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.vec3a.html

**Contents:**
- Function vec3a Copy item path

Creates a 3-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn vec3a(x: f32, y: f32, z: f32) -> Vec3A
```

---

## Struct I8Vec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.I8Vec4.html

**Contents:**
- Struct I8Vec4 Copy item path
- Fields§
- Implementations§
  - impl I8Vec4
    - pub const ZERO: I8Vec4
    - pub const ONE: I8Vec4
    - pub const NEG_ONE: I8Vec4
    - pub const MIN: I8Vec4
    - pub const MAX: I8Vec4
    - pub const X: I8Vec4

A 4-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the positive W axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

A unit vector pointing along the negative Z axis.

A unit vector pointing along the negative W axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z, w]

Creates a vector from the first 4 values in slice.

Panics if slice is less than 4 elements long.

Writes the elements of self to the first 4 elements in slice.

Panics if slice is less than 4 elements long.

Creates a 3D vector from the x, y and z elements of self, discarding w.

Truncation to I8Vec3 may also be performed by using self.xyz().

Creates a 4D vector from self with the given value of x.

Creates a 4D vector from self with the given value of y.

Creates a 4D vector from self with the given value of z.

Creates a 4D vector from self with the given value of w.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to i8::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of al

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct I8Vec4 {
    pub x: i8,
    pub y: i8,
    pub z: i8,
    pub w: i8,
}
```

---

## Struct Affine2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Affine2.html

**Contents:**
- Struct Affine2 Copy item path
- Fields§
- Implementations§
  - impl Affine2
    - pub const ZERO: Affine2
    - pub const IDENTITY: Affine2
    - pub const NAN: Affine2
    - pub const fn from_cols(x_axis: Vec2, y_axis: Vec2, z_axis: Vec2) -> Affine2
    - pub fn from_cols_array(m: &[f32; 6]) -> Affine2
    - pub fn to_cols_array(&self) -> [f32; 6]

A 2D affine transform, which can represent translation, rotation, scaling and shear.

The degenerate zero transform.

This transforms any finite vector and point to zero. The zero transform is non-invertible.

The identity transform.

Multiplying a vector with this returns the same vector.

Creates an affine transform from three column vectors.

Creates an affine transform from a [f32; 6] array stored in column major order.

Creates a [f32; 6] array storing data in column major order.

Creates an affine transform from a [[f32; 2]; 3] 2D array stored in column major order. If your data is in row major order you will need to transpose the returned matrix.

Creates a [[f32; 2]; 3] 2D array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates an affine transform from the first 6 values in slice.

Panics if slice is less than 6 elements long.

Writes the columns of self to the first 6 elements in slice.

Panics if slice is less than 6 elements long.

Creates an affine transform that changes scale. Note that if any scale is zero the transform will be non-invertible.

Creates an affine transform from the given rotation angle.

Creates an affine transformation from the given 2D translation.

Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)

Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation) and a translation vector.

Equivalent to Affine2::from_translation(translation) * Affine2::from_mat2(mat2)

Creates an affine transform from the given 2D scale, rotation angle (in radians) and translation.

Equivalent to Affine2::from_translation(translation) * Affine2::from_angle(angle) * Affine2::from_scale(scale)

Creates an affine transform from the given 2D rotation angle (in radians) and translation.

Equivalent to Affine2::from_translation(translation) * Affine2::from_angle(angle)

The given Mat3 must be an affine transform,

The given Mat3A must be an affine transform,

Extracts scale, angle and translation from self.

The transform is expected to be non-degenerate and without shearing, or the output will be invalid.

Will panic if the determinant self.matrix2 is zero or if the resulting scale vector contains any zero elements when glam_assert is enabled.

Transforms the given 2D point, applying shear, scale, rotation and translation.

Transforms the given 2D vector, applying shear, scale and rotation (but NOT translation).

To also apply tran

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct Affine2 {
    pub matrix2: Mat2,
    pub translation: Vec2,
}
```

Example 2 (javascript):
```javascript
17fn setup(
18    mut commands: Commands,
19    asset_server: Res<AssetServer>,
20    mut meshes: ResMut<Assets<Mesh>>,
21    mut materials: ResMut<Assets<StandardMaterial>>,
22) {
23    let image_with_default_sampler =
24        asset_server.load("textures/fantasy_ui_borders/panel-border-010.png");
25
26    // central cube with not repeated texture
27    commands.spawn((
28        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
29        MeshMaterial3d(materials.add(StandardMaterial {
30            base_color_texture: Some(image_with_default_sampler.clone()),
31            ..default()
32     
...
```

Example 3 (javascript):
```javascript
25fn setup(
26    mut commands: Commands,
27    asset_server: Res<AssetServer>,
28    mut meshes: ResMut<Assets<Mesh>>,
29    mut materials: ResMut<Assets<ColorMaterial>>,
30) {
31    // #11111: We use a duplicated image so that it can be load with and without
32    // settings
33    let image_with_default_sampler =
34        asset_server.load("textures/fantasy_ui_borders/panel-border-010.png");
35    let image_with_repeated_sampler = asset_server.load_with_settings(
36        "textures/fantasy_ui_borders/panel-border-010-repeated.png",
37        |s: &mut _| {
38            *s = ImageLoaderSet
...
```

---

## Function ivec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.ivec4.html

**Contents:**
- Function ivec4 Copy item path

Creates a 4-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn ivec4(x: i32, y: i32, z: i32, w: i32) -> IVec4
```

---

## Struct Mat2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Mat2.html

**Contents:**
- Struct Mat2 Copy item path
- Implementations§
  - impl Mat2
    - pub const ZERO: Mat2
    - pub const IDENTITY: Mat2
    - pub const NAN: Mat2
    - pub const fn from_cols(x_axis: Vec2, y_axis: Vec2) -> Mat2
    - pub const fn from_cols_array(m: &[f32; 4]) -> Mat2
    - pub const fn to_cols_array(&self) -> [f32; 4]
    - pub const fn from_cols_array_2d(m: &[[f32; 2]; 2]) -> Mat2

A 2x2 column major matrix.

SIMD vector types are used for storage on supported platforms.

This type is 16 byte aligned.

A 2x2 matrix with all elements set to 0.0.

A 2x2 identity matrix, where all diagonal elements are 1, and all off-diagonal elements are 0.

Creates a 2x2 matrix from two column vectors.

Creates a 2x2 matrix from a [f32; 4] array stored in column major order. If your data is stored in row major you will need to transpose the returned matrix.

Creates a [f32; 4] array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates a 2x2 matrix from a [[f32; 2]; 2] 2D array stored in column major order. If your data is in row major order you will need to transpose the returned matrix.

Creates a [[f32; 2]; 2] 2D array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates a 2x2 matrix with its diagonal set to diagonal and all other entries set to 0.

Creates a 2x2 matrix containing the combining non-uniform scale and rotation of angle (in radians).

Creates a 2x2 matrix containing a rotation of angle (in radians).

Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.

Creates a 2x2 matrix from the minor of the given 3x3 matrix, discarding the ith column and jth row.

Panics if i or j is greater than 2.

Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.

Creates a 2x2 matrix from the minor of the given 3x3 matrix, discarding the ith column and jth row.

Panics if i or j is greater than 2.

Creates a 2x2 matrix from the first 4 values in slice.

Panics if slice is less than 4 elements long.

Writes the columns of self to the first 4 elements in slice.

Panics if slice is less than 4 elements long.

Returns the matrix column for the given index.

Panics if index is greater than 1.

Returns a mutable reference to the matrix column for the given index.

Panics if index is greater than 1.

Returns the matrix row for the given index.

Panics if index is greater than 1.

Returns true if, and only if, all elements are finite. If any element is either NaN, positive or negative infinity, this will return false.

Returns true if any elements are NaN.

Returns the transpose of self.

Returns the determinant of self.

Returns the inverse of self.

If the matrix is not invertible the returned matrix will be invalid.

Will panic if the determinant of self is zero when glam_assert is enabled.

Transforms 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Mat2(/* private fields */);
```

---

## Function bvec3a Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.bvec3a.html

**Contents:**
- Function bvec3a Copy item path

Creates a 3-dimensional bool vector mask.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn bvec3a(x: bool, y: bool, z: bool) -> BVec3A
```

---

## Struct I16Vec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.I16Vec2.html

**Contents:**
- Struct I16Vec2 Copy item path
- Fields§
- Implementations§
  - impl I16Vec2
    - pub const ZERO: I16Vec2
    - pub const ONE: I16Vec2
    - pub const NEG_ONE: I16Vec2
    - pub const MIN: I16Vec2
    - pub const MAX: I16Vec2
    - pub const X: I16Vec2

A 2-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y]

Creates a vector from the first 2 values in slice.

Panics if slice is less than 2 elements long.

Writes the elements of self to the first 2 elements in slice.

Panics if slice is less than 2 elements long.

Creates a 3D vector from self and the given z value.

Creates a 2D vector from self with the given value of x.

Creates a 2D vector from self with the given value of y.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to i16::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes [self.x == rhs.x, self.y == rhs.y, ..] for all elements.

Returns a vector mask containing the result of a != comparison for each element of self and rhs.

In other words this computes [self.x != rhs.x, self

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct I16Vec2 {
    pub x: i16,
    pub y: i16,
}
```

---

## Trait HasTangent Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/trait.HasTangent.html

**Contents:**
- Trait HasTangent Copy item path
- Required Associated Types§
    - type Tangent: VectorSpace
- Implementations on Foreign Types§
  - impl<F, U, V, M, N> HasTangent for (M, N)where F: ScalarField, U: VectorSpace<Scalar = F>, V: VectorSpace<Scalar = F>, M: HasTangent<Tangent = U>, N: HasTangent<Tangent = V>,
    - type Tangent = Sum<<M as HasTangent>::Tangent, <N as HasTangent>::Tangent>
- Implementors§
  - impl<V> HasTangent for Vwhere V: VectorSpace,
    - type Tangent = V

A type that has tangents.

**Examples:**

Example 1 (unknown):
```unknown
pub trait HasTangent {
    type Tangent: VectorSpace;
}
```

---

## Function u8vec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.u8vec4.html

**Contents:**
- Function u8vec4 Copy item path

Creates a 4-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn u8vec4(x: u8, y: u8, z: u8, w: u8) -> U8Vec4
```

---

## Struct I8Vec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.I8Vec3.html

**Contents:**
- Struct I8Vec3 Copy item path
- Fields§
- Implementations§
  - impl I8Vec3
    - pub const ZERO: I8Vec3
    - pub const ONE: I8Vec3
    - pub const NEG_ONE: I8Vec3
    - pub const MIN: I8Vec3
    - pub const MAX: I8Vec3
    - pub const X: I8Vec3

A 3-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

A unit vector pointing along the negative Z axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z]

Creates a vector from the first 3 values in slice.

Panics if slice is less than 3 elements long.

Writes the elements of self to the first 3 elements in slice.

Panics if slice is less than 3 elements long.

Creates a 4D vector from self and the given w value.

Creates a 2D vector from the x and y elements of self, discarding z.

Truncation may also be performed by using self.xy().

Creates a 3D vector from self with the given value of x.

Creates a 3D vector from self with the given value of y.

Creates a 3D vector from self with the given value of z.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Computes the cross product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to i8::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Retu

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct I8Vec3 {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}
```

---

## Enum EulerRot Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/enum.EulerRot.html

**Contents:**
- Enum EulerRot Copy item path
- Variants§
  - ZYX
  - ZXY
  - YXZ
  - YZX
  - XYZ
  - XZY
  - ZYZ
  - ZXZ

Euler rotation sequences.

The three elemental rotations may be extrinsic (rotations about the axes xyz of the original coordinate system, which is assumed to remain motionless), or intrinsic (rotations about the axes of the rotating coordinate system XYZ, solidary with the moving body, which changes its orientation after each elemental rotation).

Intrinsic three-axis rotation ZYX

Intrinsic three-axis rotation ZXY

Intrinsic three-axis rotation YXZ

Intrinsic three-axis rotation YZX

Intrinsic three-axis rotation XYZ

Intrinsic three-axis rotation XZY

Intrinsic two-axis rotation ZYZ

Intrinsic two-axis rotation ZXZ

Intrinsic two-axis rotation YXY

Intrinsic two-axis rotation YZY

Intrinsic two-axis rotation XYX

Intrinsic two-axis rotation XZX

Extrinsic three-axis rotation ZYX

Extrinsic three-axis rotation ZXY

Extrinsic three-axis rotation YXZ

Extrinsic three-axis rotation YZX

Extrinsic three-axis rotation XYZ

Extrinsic three-axis rotation XZY

Extrinsic two-axis rotation ZYZ

Extrinsic two-axis rotation ZXZ

Extrinsic two-axis rotation YXY

Extrinsic two-axis rotation YZY

Extrinsic two-axis rotation XYX

Extrinsic two-axis rotation XZX

Default YXZ as yaw (y-axis), pitch (x-axis), roll (z-axis).

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum EulerRot {
Show 24 variants    ZYX,
    ZXY,
    YXZ,
    YZX,
    XYZ,
    XZY,
    ZYZ,
    ZXZ,
    YXY,
    YZY,
    XYX,
    XZX,
    ZYXEx,
    ZXYEx,
    YXZEx,
    YZXEx,
    XYZEx,
    XZYEx,
    ZYZEx,
    ZXZEx,
    YXYEx,
    YZYEx,
    XYXEx,
    XZXEx,
}
```

Example 2 (javascript):
```javascript
let m_intrinsic = Mat3::from_rotation_x(i) * Mat3::from_rotation_y(j) * Mat3::from_rotation_z(k);
let n_intrinsic = Mat3::from_euler(EulerRot::XYZ, i, j, k);
assert!(m_intrinsic.abs_diff_eq(n_intrinsic, 2e-6));

let m_extrinsic = Mat3::from_rotation_z(k) * Mat3::from_rotation_y(j) * Mat3::from_rotation_x(i);
let n_extrinsic = Mat3::from_euler(EulerRot::XYZEx, i, j, k);
assert!(m_extrinsic.abs_diff_eq(n_extrinsic, 2e-6));
```

---

## Module bool Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/bool/index.html

**Contents:**
- Module bool Copy item path
- Structs§
- Functions§

bool vector mask types.

---

## Module primitives Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/primitives/index.html

**Contents:**
- Module primitives Copy item path
- Structs§
- Enums§
- Traits§

This module defines primitive shapes. The origin is (0, 0) for 2D primitives and (0, 0, 0) for 3D primitives, unless stated otherwise.

---

## Struct Vec3A Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Vec3A.html

**Contents:**
- Struct Vec3A Copy item path
- Implementations§
  - impl Vec3A
    - pub const ZERO: Vec3A
    - pub const ONE: Vec3A
    - pub const NEG_ONE: Vec3A
    - pub const MIN: Vec3A
    - pub const MAX: Vec3A
    - pub const NAN: Vec3A
    - pub const INFINITY: Vec3A

A 3-dimensional vector.

SIMD vector types are used for storage on supported platforms for better performance than the Vec3 type.

It is possible to convert between Vec3 and Vec3A types using From or Into trait implementations.

This type is 16 byte aligned.

All f32::NEG_INFINITY.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

A unit vector pointing along the negative Z axis.

Vec3A uses Rust Portable SIMD

Vec3A uses scalar math

Vec3A uses Intel SSE2

Vec3A uses WebAssembly 128-bit SIMD

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z]

Creates a vector from the first 3 values in slice.

Panics if slice is less than 3 elements long.

Writes the elements of self to the first 3 elements in slice.

Panics if slice is less than 3 elements long.

Creates a Vec3A from the x, y and z elements of self discarding w.

On architectures where SIMD is supported such as SSE2 on x86_64 this conversion is a noop.

Creates a 4D vector from self and the given w value.

Creates a 2D vector from the x and y elements of self, discarding z.

Truncation may also be performed by using self.xy().

Creates a 3D vector from self with the given value of x.

Creates a 3D vector from self with the given value of y.

Creates a 3D vector from self with the given value of z.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Computes the cross product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

NaN propogation does not follow IEEE 754-2008 semantics for minNum and may differ on different SIMD architectures.

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

NaN propogation does not follow IEEE 754

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Vec3A(/* private fields */);
```

Example 2 (unknown):
```unknown
184fn setup(mut commands: Commands) {
185    // Spawn a single entity that has custom rendering. It'll be extracted into
186    // the render world via [`ExtractComponent`].
187    commands.spawn((
188        Visibility::default(),
189        Transform::default(),
190        // This `Aabb` is necessary for the visibility checks to work.
191        Aabb {
192            center: Vec3A::ZERO,
193            half_extents: Vec3A::splat(0.5),
194        },
195        CustomRenderedEntity,
196    ));
197
198    // Spawn the camera.
199    commands.spawn((
200        Camera3d::default(),
201        Tr
...
```

Example 3 (javascript):
```javascript
99fn draw_axes(mut gizmos: Gizmos, query: Query<(&Transform, &Aabb), With<ShowAxes>>) {
100    for (&transform, &aabb) in &query {
101        let length = aabb.half_extents.length();
102        gizmos.axes(transform, length);
103    }
104}
```

---

## Function dvec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.dvec3.html

**Contents:**
- Function dvec3 Copy item path

Creates a 3-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn dvec3(x: f64, y: f64, z: f64) -> DVec3
```

---

## Function orthonormalize Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/fn.orthonormalize.html

**Contents:**
- Function orthonormalize Copy item path

Constructs a right-handed orthonormal basis from a given unit Z vector.

This method of constructing a basis from a Vec3 is used by bevy_math::Vec3::any_orthonormal_pair

**Examples:**

Example 1 (unknown):
```unknown
pub fn orthonormalize(z_basis: Dir3) -> Mat3
```

---

## Struct U16Vec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.U16Vec2.html

**Contents:**
- Struct U16Vec2 Copy item path
- Fields§
- Implementations§
  - impl U16Vec2
    - pub const ZERO: U16Vec2
    - pub const ONE: U16Vec2
    - pub const MIN: U16Vec2
    - pub const MAX: U16Vec2
    - pub const X: U16Vec2
    - pub const Y: U16Vec2

A 2-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y]

Creates a vector from the first 2 values in slice.

Panics if slice is less than 2 elements long.

Writes the elements of self to the first 2 elements in slice.

Panics if slice is less than 2 elements long.

Creates a 3D vector from self and the given z value.

Creates a 2D vector from self with the given value of x.

Creates a 2D vector from self with the given value of y.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to u16::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes [self.x == rhs.x, self.y == rhs.y, ..] for all elements.

Returns a vector mask containing the result of a != comparison for each element of self and rhs.

In other words this computes [self.x != rhs.x, self.y != rhs.y, ..] for all elements.

Returns a vector mask containing the result of a >= comparison for

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct U16Vec2 {
    pub x: u16,
    pub y: u16,
}
```

---

## Struct DQuat Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.DQuat.html

**Contents:**
- Struct DQuat Copy item path
- Fields§
- Implementations§
  - impl DQuat
    - pub const IDENTITY: DQuat
    - pub const NAN: DQuat
    - pub const fn from_xyzw(x: f64, y: f64, z: f64, w: f64) -> DQuat
      - §Preconditions
    - pub const fn from_array(a: [f64; 4]) -> DQuat
      - §Preconditions

A quaternion representing an orientation.

This quaternion is intended to be of unit length but may denormalize due to floating point “error creep” which can occur when successive quaternion operations are applied.

The identity quaternion. Corresponds to no rotation.

Creates a new rotation quaternion.

This should generally not be called manually unless you know what you are doing. Use one of the other constructors instead such as identity or from_axis_angle.

from_xyzw is mostly used by unit tests and serde deserialization.

This function does not check if the input is normalized, it is up to the user to provide normalized input or to normalized the resulting quaternion.

Creates a rotation quaternion from an array.

This function does not check if the input is normalized, it is up to the user to provide normalized input or to normalized the resulting quaternion.

Creates a new rotation quaternion from a 4D vector.

This function does not check if the input is normalized, it is up to the user to provide normalized input or to normalized the resulting quaternion.

Creates a rotation quaternion from a slice.

This function does not check if the input is normalized, it is up to the user to provide normalized input or to normalized the resulting quaternion.

Panics if slice length is less than 4.

Writes the quaternion to an unaligned slice.

Panics if slice length is less than 4.

Create a quaternion for a normalized rotation axis and angle (in radians).

The axis must be a unit vector.

Will panic if axis is not normalized when glam_assert is enabled.

Create a quaternion that rotates v.length() radians around v.normalize().

from_scaled_axis(Vec3::ZERO) results in the identity quaternion.

Creates a quaternion from the angle (in radians) around the x axis.

Creates a quaternion from the angle (in radians) around the y axis.

Creates a quaternion from the angle (in radians) around the z axis.

Creates a quaternion from the given Euler rotation sequence and the angles (in radians).

Creates a quaternion from a 3x3 rotation matrix.

Note if the input matrix contain scales, shears, or other non-rotation transformations then the resulting quaternion will be ill-defined.

Will panic if any input matrix column is not normalized when glam_assert is enabled.

Creates a quaternion from the upper 3x3 rotation matrix inside a homogeneous 4x4 matrix.

Note if the upper 3x3 matrix contain scales, shears, or other non-rotation transformations then the resulting quatern

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct DQuat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
```

---

## Function u16vec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.u16vec4.html

**Contents:**
- Function u16vec4 Copy item path

Creates a 4-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn u16vec4(x: u16, y: u16, z: u16, w: u16) -> U16Vec4
```

---

## Module common_traits Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/common_traits/index.html

**Contents:**
- Module common_traits Copy item path
- Structs§
- Traits§

This module contains abstract mathematical traits shared by types used in bevy_math.

---

## Struct Sum Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Sum.html

**Contents:**
- Struct Sum Copy item path
- Tuple Fields§
- Trait Implementations§
  - impl<V, W> Add for Sum<V, W>where V: VectorSpace, W: VectorSpace,
    - type Output = Sum<V, W>
    - fn add(self, other: Sum<V, W>) -> <Sum<V, W> as Add>::Output
  - impl<V, W> Clone for Sum<V, W>where V: Clone, W: Clone,
    - fn clone(&self) -> Sum<V, W>
    - fn clone_from(&mut self, source: &Self)
  - impl<V, W> Debug for Sum<V, W>where V: Debug, W: Debug,

A type consisting of formal sums of elements from V and W. That is, each value Sum(v, w) is thought of as v + w, with no available simplification. In particular, if V and W are vector spaces, then Sum<V, W> is a vector space whose dimension is the sum of those of V and W, and the field accessors .0 and .1 are vector space projections.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Sum<V, W>(pub V, pub W);
```

---

## Module u8 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/u8/index.html

**Contents:**
- Module u8 Copy item path
- Structs§
- Functions§

bevy::mathModule u8 Copy item pathSource Expand descriptionu8 vector types. Structs§U8Vec2A 2-dimensional vector.U8Vec3A 3-dimensional vector.U8Vec4A 4-dimensional vector.Functions§u8vec2Creates a 2-dimensional vector.u8vec3Creates a 3-dimensional vector.u8vec4Creates a 4-dimensional vector.

---

## Function mat4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.mat4.html

**Contents:**
- Function mat4 Copy item path

Creates a 4x4 matrix from four column vectors.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn mat4(
    x_axis: Vec4,
    y_axis: Vec4,
    z_axis: Vec4,
    w_axis: Vec4,
) -> Mat4
```

---

## Struct U64Vec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.U64Vec3.html

**Contents:**
- Struct U64Vec3 Copy item path
- Fields§
- Implementations§
  - impl U64Vec3
    - pub const ZERO: U64Vec3
    - pub const ONE: U64Vec3
    - pub const MIN: U64Vec3
    - pub const MAX: U64Vec3
    - pub const X: U64Vec3
    - pub const Y: U64Vec3

A 3-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z]

Creates a vector from the first 3 values in slice.

Panics if slice is less than 3 elements long.

Writes the elements of self to the first 3 elements in slice.

Panics if slice is less than 3 elements long.

Creates a 4D vector from self and the given w value.

Creates a 2D vector from the x and y elements of self, discarding z.

Truncation may also be performed by using self.xy().

Creates a 3D vector from self with the given value of x.

Creates a 3D vector from self with the given value of y.

Creates a 3D vector from self with the given value of z.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Computes the cross product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to u64::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes [self.x == rhs.x, self.y == 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct U64Vec3 {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}
```

---

## Struct BVec4A Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.BVec4A.html

**Contents:**
- Struct BVec4A Copy item path
- Implementations§
  - impl BVec4A
    - pub const FALSE: BVec4A
    - pub const TRUE: BVec4A
    - pub const fn new(x: bool, y: bool, z: bool, w: bool) -> BVec4A
    - pub const fn splat(v: bool) -> BVec4A
    - pub const fn from_array(a: [bool; 4]) -> BVec4A
    - pub fn bitmask(self) -> u32
    - pub fn any(self) -> bool

A 4-dimensional SIMD vector mask.

This type is 16 byte aligned.

Creates a new vector mask.

Creates a vector mask with all elements set to v.

Creates a new vector mask from a bool array.

Returns a bitmask with the lowest 4 bits set from the elements of self.

A true element results in a 1 bit and a false element in a 0 bit. Element x goes into the first lowest bit, element y into the second, etc.

Returns true if any of the elements are true, false otherwise.

Returns true if all the elements are true, false otherwise.

Tests the value at index.

Panics if index is greater than 3.

Sets the element at index.

Panics if index is greater than 3.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct BVec4A(/* private fields */);
```

---

## Module swizzles Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/swizzles/index.html

**Contents:**
- Module swizzles Copy item path
- Traits§

Traits adding swizzle methods to all vector types.

---

## Struct BVec3A Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.BVec3A.html

**Contents:**
- Struct BVec3A Copy item path
- Implementations§
  - impl BVec3A
    - pub const FALSE: BVec3A
    - pub const TRUE: BVec3A
    - pub const fn new(x: bool, y: bool, z: bool) -> BVec3A
    - pub const fn splat(v: bool) -> BVec3A
    - pub const fn from_array(a: [bool; 3]) -> BVec3A
    - pub fn bitmask(self) -> u32
    - pub fn any(self) -> bool

A 3-dimensional SIMD vector mask.

This type is 16 byte aligned.

Creates a new vector mask.

Creates a vector mask with all elements set to v.

Creates a new vector mask from a bool array.

Returns a bitmask with the lowest 3 bits set from the elements of self.

A true element results in a 1 bit and a false element in a 0 bit. Element x goes into the first lowest bit, element y into the second, etc.

Returns true if any of the elements are true, false otherwise.

Returns true if all the elements are true, false otherwise.

Tests the value at index.

Panics if index is greater than 2.

Sets the element at index.

Panics if index is greater than 2.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct BVec3A(/* private fields */);
```

---

## Struct U16Vec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.U16Vec3.html

**Contents:**
- Struct U16Vec3 Copy item path
- Fields§
- Implementations§
  - impl U16Vec3
    - pub const ZERO: U16Vec3
    - pub const ONE: U16Vec3
    - pub const MIN: U16Vec3
    - pub const MAX: U16Vec3
    - pub const X: U16Vec3
    - pub const Y: U16Vec3

A 3-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z]

Creates a vector from the first 3 values in slice.

Panics if slice is less than 3 elements long.

Writes the elements of self to the first 3 elements in slice.

Panics if slice is less than 3 elements long.

Creates a 4D vector from self and the given w value.

Creates a 2D vector from the x and y elements of self, discarding z.

Truncation may also be performed by using self.xy().

Creates a 3D vector from self with the given value of x.

Creates a 3D vector from self with the given value of y.

Creates a 3D vector from self with the given value of z.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Computes the cross product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to u16::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes [self.x == rhs.x, self.y == 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct U16Vec3 {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}
```

---

## Struct Mat3A Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Mat3A.html

**Contents:**
- Struct Mat3A Copy item path
- Fields§
- Implementations§
  - impl Mat3A
    - pub const ZERO: Mat3A
    - pub const IDENTITY: Mat3A
    - pub const NAN: Mat3A
    - pub const fn from_cols(x_axis: Vec3A, y_axis: Vec3A, z_axis: Vec3A) -> Mat3A
    - pub const fn from_cols_array(m: &[f32; 9]) -> Mat3A
    - pub const fn to_cols_array(&self) -> [f32; 9]

A 3x3 column major matrix.

This 3x3 matrix type features convenience methods for creating and using linear and affine transformations. If you are primarily dealing with 2D affine transformations the Affine2 type is much faster and more space efficient than using a 3x3 matrix.

Linear transformations including 3D rotation and scale can be created using methods such as Self::from_diagonal(), Self::from_quat(), Self::from_axis_angle(), Self::from_rotation_x(), Self::from_rotation_y(), or Self::from_rotation_z().

The resulting matrices can be use to transform 3D vectors using regular vector multiplication.

Affine transformations including 2D translation, rotation and scale can be created using methods such as Self::from_translation(), Self::from_angle(), Self::from_scale() and Self::from_scale_angle_translation().

The Self::transform_point2() and Self::transform_vector2() convenience methods are provided for performing affine transforms on 2D vectors and points. These multiply 2D inputs as 3D vectors with an implicit z value of 1 for points and 0 for vectors respectively. These methods assume that Self contains a valid affine transform.

A 3x3 matrix with all elements set to 0.0.

A 3x3 identity matrix, where all diagonal elements are 1, and all off-diagonal elements are 0.

Creates a 3x3 matrix from three column vectors.

Creates a 3x3 matrix from a [f32; 9] array stored in column major order. If your data is stored in row major you will need to transpose the returned matrix.

Creates a [f32; 9] array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates a 3x3 matrix from a [[f32; 3]; 3] 3D array stored in column major order. If your data is in row major order you will need to transpose the returned matrix.

Creates a [[f32; 3]; 3] 3D array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates a 3x3 matrix with its diagonal set to diagonal and all other entries set to 0.

Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column.

Creates a 3x3 matrix from the minor of the given 4x4 matrix, discarding the ith column and jth row.

Panics if i or j is greater than 3.

Creates a 3D rotation matrix from the given quaternion.

Will panic if rotation is not normalized when glam_assert is enabled.

Creates a 3D rotation matrix from a normalized rotation axis and angle (in radians).

Will panic if axis is not normalized when glam_ass

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct Mat3A {
    pub x_axis: Vec3A,
    pub y_axis: Vec3A,
    pub z_axis: Vec3A,
}
```

---

## Struct WithTwoDerivatives Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.WithTwoDerivatives.html

**Contents:**
- Struct WithTwoDerivatives Copy item path
- Fields§
- Trait Implementations§
  - impl<T> Clone for WithTwoDerivatives<T>where T: Clone + HasTangent, <T as HasTangent>::Tangent: Clone,
    - fn clone(&self) -> WithTwoDerivatives<T>
    - fn clone_from(&mut self, source: &Self)
  - impl<T, C> Curve<WithTwoDerivatives<T>> for SampleTwoDerivativesWrapper<C>where T: HasTangent, C: SampleTwoDerivatives<T>,
    - fn domain(&self) -> Interval
    - fn sample_unchecked(&self, t: f32) -> WithTwoDerivatives<T>
    - fn sample(&self, t: f32) -> Option<WithTwoDerivatives<T>>

A value together with its first and second derivatives.

The underlying value.

The derivative at value.

The second derivative at value.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct WithTwoDerivatives<T>where
    T: HasTangent,{
    pub value: T,
    pub derivative: <T as HasTangent>::Tangent,
    pub second_derivative: <<T as HasTangent>::Tangent as HasTangent>::Tangent,
}
```

---

## Function i16vec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.i16vec4.html

**Contents:**
- Function i16vec4 Copy item path

Creates a 4-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn i16vec4(x: i16, y: i16, z: i16, w: i16) -> I16Vec4
```

---

## Enum InvalidDirectionError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/enum.InvalidDirectionError.html

**Contents:**
- Enum InvalidDirectionError Copy item path
- Variants§
  - Zero
  - Infinite
  - NaN
- Implementations§
  - impl InvalidDirectionError
    - pub fn from_length(length: f32) -> InvalidDirectionError
- Trait Implementations§
  - impl Debug for InvalidDirectionError

An error indicating that a direction is invalid.

The length of the direction vector is zero or very close to zero.

The length of the direction vector is std::f32::INFINITY.

The length of the direction vector is NaN.

Creates an InvalidDirectionError from the length of an invalid direction vector.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum InvalidDirectionError {
    Zero,
    Infinite,
    NaN,
}
```

---

## Struct USizeVec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.USizeVec3.html

**Contents:**
- Struct USizeVec3 Copy item path
- Fields§
- Implementations§
  - impl USizeVec3
    - pub const ZERO: USizeVec3
    - pub const ONE: USizeVec3
    - pub const MIN: USizeVec3
    - pub const MAX: USizeVec3
    - pub const X: USizeVec3
    - pub const Y: USizeVec3

A 3-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z]

Creates a vector from the first 3 values in slice.

Panics if slice is less than 3 elements long.

Writes the elements of self to the first 3 elements in slice.

Panics if slice is less than 3 elements long.

Creates a 4D vector from self and the given w value.

Creates a 2D vector from the x and y elements of self, discarding z.

Truncation may also be performed by using self.xy().

Creates a 3D vector from self with the given value of x.

Creates a 3D vector from self with the given value of y.

Creates a 3D vector from self with the given value of z.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Computes the cross product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to usize::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes [self.x == rhs.x, self.y =

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct USizeVec3 {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}
```

---

## Trait Curve Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/trait.Curve.html

**Contents:**
- Trait Curve Copy item path
- Required Methods§
    - fn domain(&self) -> Interval
    - fn sample_unchecked(&self, t: f32) -> T
- Provided Methods§
    - fn sample(&self, t: f32) -> Option<T>
      - Examples found in repository?
    - fn sample_clamped(&self, t: f32) -> T
- Implementors§
  - impl Curve<f32> for EaseFunction

A trait for a type that can represent values of type T parametrized over a fixed interval.

Typical examples of this are actual geometric curves where T: VectorSpace, but other kinds of output data can be represented as well. See the module-level documentation for details.

The interval over which this curve is parametrized.

This is the range of values of t where we can sample the curve and receive valid output.

Sample a point on this curve at the parameter value t, extracting the associated value. This is the unchecked version of sampling, which should only be used if the sample time t is already known to lie within the curve’s domain.

Values sampled from outside of a curve’s domain are generally considered invalid; data which is nonsensical or otherwise useless may be returned in such a circumstance, and extrapolation beyond a curve’s domain should not be relied upon.

Sample a point on this curve at the parameter value t, returning None if the point is outside of the curve’s domain.

Sample a point on this curve at the parameter value t, clamping t to lie inside the domain of the curve.

**Examples:**

Example 1 (unknown):
```unknown
pub trait Curve<T> {
    // Required methods
    fn domain(&self) -> Interval;
    fn sample_unchecked(&self, t: f32) -> T;

    // Provided methods
    fn sample(&self, t: f32) -> Option<T> { ... }
    fn sample_clamped(&self, t: f32) -> T { ... }
}
```

Example 2 (javascript):
```javascript
124fn display_curves(
125    mut gizmos: Gizmos,
126    ease_functions: Query<(&EaseFunctionPlot, &Transform, &Children)>,
127    mut transforms: Query<&mut Transform, Without<EaseFunctionPlot>>,
128    mut ui_text: Single<&mut Text>,
129    time: Res<Time>,
130) {
131    let samples = 100;
132    let duration = 2.5;
133    let time_margin = 0.5;
134
135    let now = ((time.elapsed_secs() % (duration + time_margin * 2.0) - time_margin) / duration)
136        .clamp(0.0, 1.0);
137
138    ui_text.0 = format!("Progress: {now:.2}");
139
140    for (EaseFunctionPlot(function, color), transform, chi
...
```

---

## Trait NormedVectorSpace Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/trait.NormedVectorSpace.html

**Contents:**
- Trait NormedVectorSpace Copy item path
- Required Methods§
    - fn norm(self) -> Self::Scalar
- Provided Methods§
    - fn norm_squared(self) -> Self::Scalar
    - fn distance(self, rhs: Self) -> Self::Scalar
    - fn distance_squared(self, rhs: Self) -> Self::Scalar
- Dyn Compatibility§
- Implementations on Foreign Types§
  - impl NormedVectorSpace for f32

A type that supports the operations of a normed vector space; i.e. a norm operation in addition to those of VectorSpace. Specifically, the implementor must guarantee that the following relationships hold, within the limitations of floating point arithmetic:

Note that, because implementing types use floating point arithmetic, they are not required to actually implement PartialEq or Eq.

The size of this element. The return value should always be nonnegative.

The squared norm of this element. Computing this is often faster than computing NormedVectorSpace::norm.

The distance between this element and another, as determined by the norm.

The squared distance between this element and another, as determined by the norm. Note that this is often faster to compute in practice than NormedVectorSpace::distance.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait NormedVectorSpace: VectorSpace {
    // Required method
    fn norm(self) -> Self::Scalar;

    // Provided methods
    fn norm_squared(self) -> Self::Scalar { ... }
    fn distance(self, rhs: Self) -> Self::Scalar { ... }
    fn distance_squared(self, rhs: Self) -> Self::Scalar { ... }
}
```

---

## Struct IRect Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.IRect.html

**Contents:**
- Struct IRect Copy item path
- Fields§
- Implementations§
  - impl IRect
    - pub const EMPTY: IRect
    - pub fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> IRect
      - §Examples
    - pub fn from_corners(p0: IVec2, p1: IVec2) -> IRect
      - §Examples
    - pub fn from_center_size(origin: IVec2, size: IVec2) -> IRect

A rectangle defined by two opposite corners.

The rectangle is axis aligned, and defined by its minimum and maximum coordinates, stored in IRect::min and IRect::max, respectively. The minimum/maximum invariant must be upheld by the user when directly assigning the fields, otherwise some methods produce invalid results. It is generally recommended to use one of the constructor methods instead, which will ensure this invariant is met, unless you already have the minimum and maximum corners.

The minimum corner point of the rect.

The maximum corner point of the rect.

An empty IRect, represented by maximum and minimum corner points with max == IVec2::MIN and min == IVec2::MAX, so the rect has an extremely large negative size. This is useful, because when taking a union B of a non-empty IRect A and this empty IRect, B will simply equal A.

Create a new rectangle from two corner points.

The two points do not need to be the minimum and/or maximum corners. They only need to be two opposite corners.

Create a new rectangle from two corner points.

The two points do not need to be the minimum and/or maximum corners. They only need to be two opposite corners.

Create a new rectangle from its center and size.

If the size contains odd numbers they will be rounded down to the nearest whole number.

This method panics if any of the components of the size is negative.

Create a new rectangle from its center and half-size.

This method panics if any of the components of the half-size is negative.

Check if the rectangle is empty.

Rectangle width (max.x - min.x).

Rectangle height (max.y - min.y).

If the full size contains odd numbers they will be rounded down to the nearest whole number when calculating the half size.

The center point of the rectangle.

If the (min + max) contains odd numbers they will be rounded down to the nearest whole number when calculating the center.

Check if a point lies within this rectangle, inclusive of its edges.

Build a new rectangle formed of the union of this rectangle and another rectangle.

The union is the smallest rectangle enclosing both rectangles.

Build a new rectangle formed of the union of this rectangle and a point.

The union is the smallest rectangle enclosing both the rectangle and the point. If the point is already inside the rectangle, this method returns a copy of the rectangle.

Build a new rectangle formed of the intersection of this rectangle and another rectangle.

The intersection is the largest rectangle enclo

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct IRect {
    pub min: IVec2,
    pub max: IVec2,
}
```

Example 2 (javascript):
```javascript
let r = IRect::new(0, 4, 10, 6); // w=10 h=2
let r = IRect::new(2, 3, 5, -1); // w=3 h=4
```

Example 3 (javascript):
```javascript
// Unit rect from [0,0] to [1,1]
let r = IRect::from_corners(IVec2::ZERO, IVec2::ONE); // w=1 h=1
// Same; the points do not need to be ordered
let r = IRect::from_corners(IVec2::ONE, IVec2::ZERO); // w=1 h=1
```

Example 4 (javascript):
```javascript
let r = IRect::from_center_size(IVec2::ZERO, IVec2::new(3, 2)); // w=2 h=2
assert_eq!(r.min, IVec2::splat(-1));
assert_eq!(r.max, IVec2::splat(1));
```

---

## Struct U16Vec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.U16Vec4.html

**Contents:**
- Struct U16Vec4 Copy item path
- Fields§
- Implementations§
  - impl U16Vec4
    - pub const ZERO: U16Vec4
    - pub const ONE: U16Vec4
    - pub const MIN: U16Vec4
    - pub const MAX: U16Vec4
    - pub const X: U16Vec4
    - pub const Y: U16Vec4

A 4-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the positive W axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z, w]

Creates a vector from the first 4 values in slice.

Panics if slice is less than 4 elements long.

Writes the elements of self to the first 4 elements in slice.

Panics if slice is less than 4 elements long.

Creates a 3D vector from the x, y and z elements of self, discarding w.

Truncation to U16Vec3 may also be performed by using self.xyz().

Creates a 4D vector from self with the given value of x.

Creates a 4D vector from self with the given value of y.

Creates a 4D vector from self with the given value of z.

Creates a 4D vector from self with the given value of w.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to u16::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct U16Vec4 {
    pub x: u16,
    pub y: u16,
    pub z: u16,
    pub w: u16,
}
```

---

## Struct Vec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Vec2.html

**Contents:**
- Struct Vec2 Copy item path
- Fields§
- Implementations§
  - impl Vec2
    - pub const ZERO: Vec2
    - pub const ONE: Vec2
    - pub const NEG_ONE: Vec2
    - pub const MIN: Vec2
    - pub const MAX: Vec2
    - pub const NAN: Vec2

A 2-dimensional vector.

All f32::NEG_INFINITY.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

Vec2 uses Rust Portable SIMD

Vec2 uses scalar math

Vec2 uses WebAssembly 128-bit SIMD

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y]

Creates a vector from the first 2 values in slice.

Panics if slice is less than 2 elements long.

Writes the elements of self to the first 2 elements in slice.

Panics if slice is less than 2 elements long.

Creates a 3D vector from self and the given z value.

Creates a 2D vector from self with the given value of x.

Creates a 2D vector from self with the given value of y.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

NaN propogation does not follow IEEE 754-2008 semantics for minNum and may differ on different SIMD architectures.

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

NaN propogation does not follow IEEE 754-2008 semantics for maxNum and may differ on different SIMD architectures.

Component-wise clamping of values, similar to f32::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
```

Example 2 (javascript):
```javascript
5const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);
```

Example 3 (javascript):
```javascript
18const EXTENT: Vec2 = Vec2::new(1172.0, 520.0);
19const PLOT_SIZE: Vec2 = Vec2::splat(80.0);
20
21fn setup(mut commands: Commands) {
22    commands.spawn(Camera2d);
23
24    let text_font = TextFont {
25        font_size: 10.0,
26        ..default()
27    };
28
29    let chunks = [
30        // "In" row
31        EaseFunction::SineIn,
32        EaseFunction::QuadraticIn,
33        EaseFunction::CubicIn,
34        EaseFunction::QuarticIn,
35        EaseFunction::QuinticIn,
36        EaseFunction::SmoothStepIn,
37        EaseFunction::SmootherStepIn,
38        EaseFunction::CircularIn,
39      
...
```

Example 4 (javascript):
```javascript
14const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
15const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
16const PADDLE_SPEED: f32 = 500.0;
17// How close can the paddle get to the wall
18const PADDLE_PADDING: f32 = 10.0;
19
20// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
21const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
22const BALL_DIAMETER: f32 = 30.;
23const BALL_SPEED: f32 = 400.0;
24const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
25
26const WALL_THICKNESS: f32 = 10.0;
27// x coordinates
28const LEFT_WALL: f32 = -4
...
```

---

## Function dmat2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.dmat2.html

**Contents:**
- Function dmat2 Copy item path

Creates a 2x2 matrix from two column vectors.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn dmat2(x_axis: DVec2, y_axis: DVec2) -> DMat2
```

---

## Struct U64Vec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.U64Vec2.html

**Contents:**
- Struct U64Vec2 Copy item path
- Fields§
- Implementations§
  - impl U64Vec2
    - pub const ZERO: U64Vec2
    - pub const ONE: U64Vec2
    - pub const MIN: U64Vec2
    - pub const MAX: U64Vec2
    - pub const X: U64Vec2
    - pub const Y: U64Vec2

A 2-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y]

Creates a vector from the first 2 values in slice.

Panics if slice is less than 2 elements long.

Writes the elements of self to the first 2 elements in slice.

Panics if slice is less than 2 elements long.

Creates a 3D vector from self and the given z value.

Creates a 2D vector from self with the given value of x.

Creates a 2D vector from self with the given value of y.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to u64::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes [self.x == rhs.x, self.y == rhs.y, ..] for all elements.

Returns a vector mask containing the result of a != comparison for each element of self and rhs.

In other words this computes [self.x != rhs.x, self.y != rhs.y, ..] for all elements.

Returns a vector mask containing the result of a >= comparison for

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct U64Vec2 {
    pub x: u64,
    pub y: u64,
}
```

---

## Struct IVec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.IVec2.html

**Contents:**
- Struct IVec2 Copy item path
- Fields§
- Implementations§
  - impl IVec2
    - pub const ZERO: IVec2
    - pub const ONE: IVec2
    - pub const NEG_ONE: IVec2
    - pub const MIN: IVec2
    - pub const MAX: IVec2
    - pub const X: IVec2

A 2-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y]

Creates a vector from the first 2 values in slice.

Panics if slice is less than 2 elements long.

Writes the elements of self to the first 2 elements in slice.

Panics if slice is less than 2 elements long.

Creates a 3D vector from self and the given z value.

Creates a 2D vector from self with the given value of x.

Creates a 2D vector from self with the given value of y.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to i32::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes [self.x == rhs.x, self.y == rhs.y, ..] for all elements.

Returns a vector mask containing the result of a != comparison for each element of self and rhs.

In other words this computes [self.x != rhs.x, self

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}
```

---

## Struct DVec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.DVec4.html

**Contents:**
- Struct DVec4 Copy item path
- Fields§
- Implementations§
  - impl DVec4
    - pub const ZERO: DVec4
    - pub const ONE: DVec4
    - pub const NEG_ONE: DVec4
    - pub const MIN: DVec4
    - pub const MAX: DVec4
    - pub const NAN: DVec4

A 4-dimensional vector.

All f64::NEG_INFINITY.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the positive W axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

A unit vector pointing along the negative Z axis.

A unit vector pointing along the negative W axis.

DVec4 uses Rust Portable SIMD

DVec4 uses scalar math

DVec4 uses Intel SSE2

DVec4 uses WebAssembly 128-bit SIMD

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z, w]

Creates a vector from the first 4 values in slice.

Panics if slice is less than 4 elements long.

Writes the elements of self to the first 4 elements in slice.

Panics if slice is less than 4 elements long.

Creates a 3D vector from the x, y and z elements of self, discarding w.

Truncation to DVec3 may also be performed by using self.xyz().

Creates a 4D vector from self with the given value of x.

Creates a 4D vector from self with the given value of y.

Creates a 4D vector from self with the given value of z.

Creates a 4D vector from self with the given value of w.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

NaN propogation does not follow IEEE 754-2008 semantics for minNum and may differ on different SIMD architectures.

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

NaN propogation does not follow IEEE 754-2008 semantics for maxNum and may differ on different SIMD architectures.

Component-wise clamping of values, similar to f64::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct DVec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
```

---

## Struct I64Vec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.I64Vec2.html

**Contents:**
- Struct I64Vec2 Copy item path
- Fields§
- Implementations§
  - impl I64Vec2
    - pub const ZERO: I64Vec2
    - pub const ONE: I64Vec2
    - pub const NEG_ONE: I64Vec2
    - pub const MIN: I64Vec2
    - pub const MAX: I64Vec2
    - pub const X: I64Vec2

A 2-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y]

Creates a vector from the first 2 values in slice.

Panics if slice is less than 2 elements long.

Writes the elements of self to the first 2 elements in slice.

Panics if slice is less than 2 elements long.

Creates a 3D vector from self and the given z value.

Creates a 2D vector from self with the given value of x.

Creates a 2D vector from self with the given value of y.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to i64::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes [self.x == rhs.x, self.y == rhs.y, ..] for all elements.

Returns a vector mask containing the result of a != comparison for each element of self and rhs.

In other words this computes [self.x != rhs.x, self

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct I64Vec2 {
    pub x: i64,
    pub y: i64,
}
```

---

## Function dmat4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.dmat4.html

**Contents:**
- Function dmat4 Copy item path

Creates a 4x4 matrix from four column vectors.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn dmat4(
    x_axis: DVec4,
    y_axis: DVec4,
    z_axis: DVec4,
    w_axis: DVec4,
) -> DMat4
```

---

## Struct I16Vec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.I16Vec3.html

**Contents:**
- Struct I16Vec3 Copy item path
- Fields§
- Implementations§
  - impl I16Vec3
    - pub const ZERO: I16Vec3
    - pub const ONE: I16Vec3
    - pub const NEG_ONE: I16Vec3
    - pub const MIN: I16Vec3
    - pub const MAX: I16Vec3
    - pub const X: I16Vec3

A 3-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

A unit vector pointing along the negative Z axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z]

Creates a vector from the first 3 values in slice.

Panics if slice is less than 3 elements long.

Writes the elements of self to the first 3 elements in slice.

Panics if slice is less than 3 elements long.

Creates a 4D vector from self and the given w value.

Creates a 2D vector from the x and y elements of self, discarding z.

Truncation may also be performed by using self.xy().

Creates a 3D vector from self with the given value of x.

Creates a 3D vector from self with the given value of y.

Creates a 3D vector from self with the given value of z.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Computes the cross product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to i16::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Ret

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct I16Vec3 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}
```

---

## Struct BVec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.BVec2.html

**Contents:**
- Struct BVec2 Copy item path
- Fields§
- Implementations§
  - impl BVec2
    - pub const FALSE: BVec2
    - pub const TRUE: BVec2
    - pub const fn new(x: bool, y: bool) -> BVec2
    - pub const fn splat(v: bool) -> BVec2
    - pub const fn from_array(a: [bool; 2]) -> BVec2
    - pub fn bitmask(self) -> u32

A 2-dimensional bool vector mask.

Creates a new vector mask.

Creates a vector mask with all elements set to v.

Creates a new vector mask from a bool array.

Returns a bitmask with the lowest 2 bits set from the elements of self.

A true element results in a 1 bit and a false element in a 0 bit. Element x goes into the first lowest bit, element y into the second, etc.

Returns true if any of the elements are true, false otherwise.

Returns true if all the elements are true, false otherwise.

Tests the value at index.

Panics if index is greater than 1.

Sets the element at index.

Panics if index is greater than 1.

Deserialize expects a sequence of 2 values.

Serialize as a sequence of 2 values.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C, align(1))]pub struct BVec2 {
    pub x: bool,
    pub y: bool,
}
```

---

## Struct Affine3A Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Affine3A.html

**Contents:**
- Struct Affine3A Copy item path
- Fields§
- Implementations§
  - impl Affine3A
    - pub const ZERO: Affine3A
    - pub const IDENTITY: Affine3A
    - pub const NAN: Affine3A
    - pub const fn from_cols( x_axis: Vec3A, y_axis: Vec3A, z_axis: Vec3A, w_axis: Vec3A, ) -> Affine3A
    - pub fn from_cols_array(m: &[f32; 12]) -> Affine3A
    - pub fn to_cols_array(&self) -> [f32; 12]

A 3D affine transform, which can represent translation, rotation, scaling and shear.

This type is 16 byte aligned.

The degenerate zero transform.

This transforms any finite vector and point to zero. The zero transform is non-invertible.

The identity transform.

Multiplying a vector with this returns the same vector.

Creates an affine transform from three column vectors.

Creates an affine transform from a [f32; 12] array stored in column major order.

Creates a [f32; 12] array storing data in column major order.

Creates an affine transform from a [[f32; 3]; 4] 3D array stored in column major order. If your data is in row major order you will need to transpose the returned matrix.

Creates a [[f32; 3]; 4] 3D array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates an affine transform from the first 12 values in slice.

Panics if slice is less than 12 elements long.

Writes the columns of self to the first 12 elements in slice.

Panics if slice is less than 12 elements long.

Creates an affine transform that changes scale. Note that if any scale is zero the transform will be non-invertible.

Creates an affine transform from the given rotation quaternion.

Creates an affine transform containing a 3D rotation around a normalized rotation axis of angle (in radians).

Creates an affine transform containing a 3D rotation around the x axis of angle (in radians).

Creates an affine transform containing a 3D rotation around the y axis of angle (in radians).

Creates an affine transform containing a 3D rotation around the z axis of angle (in radians).

Creates an affine transformation from the given 3D translation.

Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation)

Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation) and a translation vector.

Equivalent to Affine3A::from_translation(translation) * Affine3A::from_mat3(mat3)

Creates an affine transform from the given 3D scale, rotation and translation.

Equivalent to Affine3A::from_translation(translation) * Affine3A::from_quat(rotation) * Affine3A::from_scale(scale)

Creates an affine transform from the given 3D rotation and translation.

Equivalent to Affine3A::from_translation(translation) * Affine3A::from_quat(rotation)

The given Mat4 must be an affine transform, i.e. contain no perspective transform.

Extracts scale, rotation and translation from self.

The transform is expected

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct Affine3A {
    pub matrix3: Mat3A,
    pub translation: Vec3A,
}
```

---

## Function u16vec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.u16vec2.html

**Contents:**
- Function u16vec2 Copy item path

Creates a 2-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn u16vec2(x: u16, y: u16) -> U16Vec2
```

---

## Function dquat Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.dquat.html

**Contents:**
- Function dquat Copy item path

Creates a quaternion from x, y, z and w values.

This should generally not be called manually unless you know what you are doing. Use one of the other constructors instead such as identity or from_axis_angle.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn dquat(x: f64, y: f64, z: f64, w: f64) -> DQuat
```

---

## Function bvec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.bvec2.html

**Contents:**
- Function bvec2 Copy item path

Creates a 2-dimensional bool vector mask.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn bvec2(x: bool, y: bool) -> BVec2
```

---

## Trait FloatExt Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/trait.FloatExt.html

**Contents:**
- Trait FloatExt Copy item path
- Required Methods§
    - fn lerp(self, rhs: Self, s: Self) -> Self
    - fn inverse_lerp(a: Self, b: Self, v: Self) -> Self
    - fn remap( self, in_start: Self, in_end: Self, out_start: Self, out_end: Self, ) -> Self
    - fn fract_gl(self) -> Self
- Dyn Compatibility§
- Implementations on Foreign Types§
  - impl FloatExt for f32
    - fn lerp(self, rhs: f32, t: f32) -> f32

A trait for extending f32 and f64 with extra methods.

Performs a linear interpolation between self and rhs based on the value s.

When s is 0, the result will be self. When s is 1, the result will be rhs. When s is outside of the range [0, 1], the result is linearly extrapolated.

Returns v normalized to the range [a, b].

When v is equal to a the result will be 0. When v is equal to b will be 1.

When v is outside of the range [a, b], the result is linearly extrapolated.

a and b must not be equal, otherwise the result will be either infinite or NAN.

Remap self from the input range to the output range.

When self is equal to in_start this returns out_start. When self is equal to in_end this returns out_end.

When self is outside of the range [in_start, in_end], the result is linearly extrapolated.

in_start and in_end must not be equal, otherwise the result will be either infinite or NAN.

Returns the fractional part of the input as self - self.floor().

Note that this differs from the Rust implementation of fract which returns self - self.trunc().

Note that this is fast but not precise for large numbers.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait FloatExt {
    // Required methods
    fn lerp(self, rhs: Self, s: Self) -> Self;
    fn inverse_lerp(a: Self, b: Self, v: Self) -> Self;
    fn remap(
        self,
        in_start: Self,
        in_end: Self,
        out_start: Self,
        out_end: Self,
    ) -> Self;
    fn fract_gl(self) -> Self;
}
```

---

## Function ivec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.ivec2.html

**Contents:**
- Function ivec2 Copy item path

Creates a 2-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn ivec2(x: i32, y: i32) -> IVec2
```

---

## Function i64vec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.i64vec2.html

**Contents:**
- Function i64vec2 Copy item path

Creates a 2-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn i64vec2(x: i64, y: i64) -> I64Vec2
```

---

## Struct URect Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.URect.html

**Contents:**
- Struct URect Copy item path
- Fields§
- Implementations§
  - impl URect
    - pub const EMPTY: URect
    - pub fn new(x0: u32, y0: u32, x1: u32, y1: u32) -> URect
      - §Examples
    - pub fn from_corners(p0: UVec2, p1: UVec2) -> URect
      - §Examples
    - pub fn from_center_size(origin: UVec2, size: UVec2) -> URect

A rectangle defined by two opposite corners.

The rectangle is axis aligned, and defined by its minimum and maximum coordinates, stored in URect::min and URect::max, respectively. The minimum/maximum invariant must be upheld by the user when directly assigning the fields, otherwise some methods produce invalid results. It is generally recommended to use one of the constructor methods instead, which will ensure this invariant is met, unless you already have the minimum and maximum corners.

The minimum corner point of the rect.

The maximum corner point of the rect.

An empty URect, represented by maximum and minimum corner points with max == UVec2::MIN and min == UVec2::MAX, so the rect has an extremely large negative size. This is useful, because when taking a union B of a non-empty URect A and this empty URect, B will simply equal A.

Create a new rectangle from two corner points.

The two points do not need to be the minimum and/or maximum corners. They only need to be two opposite corners.

Create a new rectangle from two corner points.

The two points do not need to be the minimum and/or maximum corners. They only need to be two opposite corners.

Create a new rectangle from its center and size.

If the size contains odd numbers they will be rounded down to the nearest whole number.

This method panics if any of the components of the size is negative or if origin - (size / 2) results in any negatives.

Create a new rectangle from its center and half-size.

This method panics if any of the components of the half-size is negative or if origin - half_size results in any negatives.

Check if the rectangle is empty.

Rectangle width (max.x - min.x).

Rectangle height (max.y - min.y).

If the full size contains odd numbers they will be rounded down to the nearest whole number when calculating the half size.

The center point of the rectangle.

If the (min + max) contains odd numbers they will be rounded down to the nearest whole number when calculating the center.

Check if a point lies within this rectangle, inclusive of its edges.

Build a new rectangle formed of the union of this rectangle and another rectangle.

The union is the smallest rectangle enclosing both rectangles.

Build a new rectangle formed of the union of this rectangle and a point.

The union is the smallest rectangle enclosing both the rectangle and the point. If the point is already inside the rectangle, this method returns a copy of the rectangle.

Build a new rectangle formed of the i

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct URect {
    pub min: UVec2,
    pub max: UVec2,
}
```

Example 2 (javascript):
```javascript
let r = URect::new(0, 4, 10, 6); // w=10 h=2
let r = URect::new(2, 4, 5, 0); // w=3 h=4
```

Example 3 (javascript):
```javascript
// Unit rect from [0,0] to [1,1]
let r = URect::from_corners(UVec2::ZERO, UVec2::ONE); // w=1 h=1
// Same; the points do not need to be ordered
let r = URect::from_corners(UVec2::ONE, UVec2::ZERO); // w=1 h=1
```

Example 4 (javascript):
```javascript
let r = URect::from_center_size(UVec2::ONE, UVec2::splat(2)); // w=2 h=2
assert_eq!(r.min, UVec2::splat(0));
assert_eq!(r.max, UVec2::splat(2));
```

---

## Struct DAffine3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.DAffine3.html

**Contents:**
- Struct DAffine3 Copy item path
- Fields§
- Implementations§
  - impl DAffine3
    - pub const ZERO: DAffine3
    - pub const IDENTITY: DAffine3
    - pub const NAN: DAffine3
    - pub const fn from_cols( x_axis: DVec3, y_axis: DVec3, z_axis: DVec3, w_axis: DVec3, ) -> DAffine3
    - pub fn from_cols_array(m: &[f64; 12]) -> DAffine3
    - pub fn to_cols_array(&self) -> [f64; 12]

A 3D affine transform, which can represent translation, rotation, scaling and shear.

The degenerate zero transform.

This transforms any finite vector and point to zero. The zero transform is non-invertible.

The identity transform.

Multiplying a vector with this returns the same vector.

Creates an affine transform from three column vectors.

Creates an affine transform from a [f64; 12] array stored in column major order.

Creates a [f64; 12] array storing data in column major order.

Creates an affine transform from a [[f64; 3]; 4] 3D array stored in column major order. If your data is in row major order you will need to transpose the returned matrix.

Creates a [[f64; 3]; 4] 3D array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates an affine transform from the first 12 values in slice.

Panics if slice is less than 12 elements long.

Writes the columns of self to the first 12 elements in slice.

Panics if slice is less than 12 elements long.

Creates an affine transform that changes scale. Note that if any scale is zero the transform will be non-invertible.

Creates an affine transform from the given rotation quaternion.

Creates an affine transform containing a 3D rotation around a normalized rotation axis of angle (in radians).

Creates an affine transform containing a 3D rotation around the x axis of angle (in radians).

Creates an affine transform containing a 3D rotation around the y axis of angle (in radians).

Creates an affine transform containing a 3D rotation around the z axis of angle (in radians).

Creates an affine transformation from the given 3D translation.

Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation)

Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation) and a translation vector.

Equivalent to DAffine3::from_translation(translation) * DAffine3::from_mat3(mat3)

Creates an affine transform from the given 3D scale, rotation and translation.

Equivalent to DAffine3::from_translation(translation) * DAffine3::from_quat(rotation) * DAffine3::from_scale(scale)

Creates an affine transform from the given 3D rotation and translation.

Equivalent to DAffine3::from_translation(translation) * DAffine3::from_quat(rotation)

The given DMat4 must be an affine transform, i.e. contain no perspective transform.

Extracts scale, rotation and translation from self.

The transform is expected to be non-degenerate and with

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct DAffine3 {
    pub matrix3: DMat3,
    pub translation: DVec3,
}
```

---

## Struct BVec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.BVec4.html

**Contents:**
- Struct BVec4 Copy item path
- Fields§
- Implementations§
  - impl BVec4
    - pub const FALSE: BVec4
    - pub const TRUE: BVec4
    - pub const fn new(x: bool, y: bool, z: bool, w: bool) -> BVec4
    - pub const fn splat(v: bool) -> BVec4
    - pub const fn from_array(a: [bool; 4]) -> BVec4
    - pub fn bitmask(self) -> u32

A 4-dimensional bool vector mask.

Creates a new vector mask.

Creates a vector mask with all elements set to v.

Creates a new vector mask from a bool array.

Returns a bitmask with the lowest 4 bits set from the elements of self.

A true element results in a 1 bit and a false element in a 0 bit. Element x goes into the first lowest bit, element y into the second, etc.

Returns true if any of the elements are true, false otherwise.

Returns true if all the elements are true, false otherwise.

Tests the value at index.

Panics if index is greater than 3.

Sets the element at index.

Panics if index is greater than 3.

Deserialize expects a sequence of 4 values.

Serialize as a sequence of 4 values.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C, align(1))]pub struct BVec4 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub w: bool,
}
```

---

## Struct BVec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.BVec3.html

**Contents:**
- Struct BVec3 Copy item path
- Fields§
- Implementations§
  - impl BVec3
    - pub const FALSE: BVec3
    - pub const TRUE: BVec3
    - pub const fn new(x: bool, y: bool, z: bool) -> BVec3
    - pub const fn splat(v: bool) -> BVec3
    - pub const fn from_array(a: [bool; 3]) -> BVec3
    - pub fn bitmask(self) -> u32

A 3-dimensional bool vector mask.

Creates a new vector mask.

Creates a vector mask with all elements set to v.

Creates a new vector mask from a bool array.

Returns a bitmask with the lowest 3 bits set from the elements of self.

A true element results in a 1 bit and a false element in a 0 bit. Element x goes into the first lowest bit, element y into the second, etc.

Returns true if any of the elements are true, false otherwise.

Returns true if all the elements are true, false otherwise.

Tests the value at index.

Panics if index is greater than 2.

Sets the element at index.

Panics if index is greater than 2.

Deserialize expects a sequence of 3 values.

Serialize as a sequence of 3 values.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C, align(1))]pub struct BVec3 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}
```

---

## Struct Rot2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Rot2.html

**Contents:**
- Struct Rot2 Copy item path
- §Example
- Fields§
- Implementations§
  - impl Rot2
    - pub const IDENTITY: Rot2
    - pub const PI: Rot2
    - pub const FRAC_PI_2: Rot2
    - pub const FRAC_PI_3: Rot2
    - pub const FRAC_PI_4: Rot2

The cosine of the rotation angle.

This is the real part of the unit complex number representing the rotation.

The sine of the rotation angle.

This is the imaginary part of the unit complex number representing the rotation.

No rotation. Also equals a full turn that returns back to its original position.

A rotation of π radians. Corresponds to a half-turn.

A counterclockwise rotation of π/2 radians. Corresponds to a counterclockwise quarter-turn.

A counterclockwise rotation of π/3 radians. Corresponds to a counterclockwise turn by 60°.

A counterclockwise rotation of π/4 radians. Corresponds to a counterclockwise turn by 45°.

A counterclockwise rotation of π/6 radians. Corresponds to a counterclockwise turn by 30°.

A counterclockwise rotation of π/8 radians. Corresponds to a counterclockwise turn by 22.5°.

Creates a Rot2 from a counterclockwise angle in radians. A negative argument corresponds to a clockwise rotation.

Angles larger than or equal to 2π (in either direction) loop around to smaller rotations, since a full rotation returns an object to its starting orientation.

Creates a Rot2 from a counterclockwise angle in degrees. A negative argument corresponds to a clockwise rotation.

Angles larger than or equal to 360° (in either direction) loop around to smaller rotations, since a full rotation returns an object to its starting orientation.

Creates a Rot2 from a counterclockwise fraction of a full turn of 360 degrees. A negative argument corresponds to a clockwise rotation.

Angles larger than or equal to 1 turn (in either direction) loop around to smaller rotations, since a full rotation returns an object to its starting orientation.

Creates a Rot2 from the sine and cosine of an angle.

The rotation is only valid if sin * sin + cos * cos == 1.0.

Panics if sin * sin + cos * cos != 1.0 when the glam_assert feature is enabled.

Returns a corresponding rotation angle in radians in the (-pi, pi] range.

Returns a corresponding rotation angle in degrees in the (-180, 180] range.

Returns a corresponding rotation angle as a fraction of a full 360 degree turn in the (-0.5, 0.5] range.

Returns the sine and cosine of the rotation angle.

Computes the length or norm of the complex number used to represent the rotation.

The length is typically expected to be 1.0. Unexpectedly denormalized rotations can be a result of incorrect construction or floating point error caused by successive operations.

Computes the squared length or norm of the complex n

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Rot2 {
    pub cos: f32,
    pub sin: f32,
}
```

Example 2 (javascript):
```javascript
use std::f32::consts::PI;

// Create rotations from counterclockwise angles in radians or degrees
let rotation1 = Rot2::radians(PI / 2.0);
let rotation2 = Rot2::degrees(45.0);

// Get the angle back as radians or degrees
assert_eq!(rotation1.as_degrees(), 90.0);
assert_eq!(rotation2.as_radians(), PI / 4.0);

// "Add" rotations together using `*`
#[cfg(feature = "approx")]
assert_relative_eq!(rotation1 * rotation2, Rot2::degrees(135.0));

// Rotate vectors
#[cfg(feature = "approx")]
assert_relative_eq!(rotation1 * Vec2::X, Vec2::Y);
```

Example 3 (unknown):
```unknown
#[cfg(feature = "approx")]
assert_relative_eq!(Rot2::IDENTITY, Rot2::degrees(360.0), epsilon = 2e-7);
```

Example 4 (javascript):
```javascript
let rot1 = Rot2::radians(3.0 * FRAC_PI_2);
let rot2 = Rot2::radians(-FRAC_PI_2);
#[cfg(feature = "approx")]
assert_relative_eq!(rot1, rot2);

let rot3 = Rot2::radians(PI);
#[cfg(feature = "approx")]
assert_relative_eq!(rot1 * rot1, rot3);

// A rotation by 3π and 1π are the same
#[cfg(feature = "approx")]
assert_relative_eq!(Rot2::radians(3.0 * PI), Rot2::radians(PI));
```

---

## Struct DVec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.DVec3.html

**Contents:**
- Struct DVec3 Copy item path
- Fields§
- Implementations§
  - impl DVec3
    - pub const ZERO: DVec3
    - pub const ONE: DVec3
    - pub const NEG_ONE: DVec3
    - pub const MIN: DVec3
    - pub const MAX: DVec3
    - pub const NAN: DVec3

A 3-dimensional vector.

All f64::NEG_INFINITY.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

A unit vector pointing along the negative Z axis.

DVec3 uses Rust Portable SIMD

DVec3 uses scalar math

DVec3 uses Intel SSE2

DVec3 uses WebAssembly 128-bit SIMD

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z]

Creates a vector from the first 3 values in slice.

Panics if slice is less than 3 elements long.

Writes the elements of self to the first 3 elements in slice.

Panics if slice is less than 3 elements long.

Creates a 4D vector from self and the given w value.

Creates a 2D vector from the x and y elements of self, discarding z.

Truncation may also be performed by using self.xy().

Creates a 3D vector from self with the given value of x.

Creates a 3D vector from self with the given value of y.

Creates a 3D vector from self with the given value of z.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Computes the cross product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

NaN propogation does not follow IEEE 754-2008 semantics for minNum and may differ on different SIMD architectures.

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

NaN propogation does not follow IEEE 754-2008 semantics for maxNum and may differ on different SIMD architectures.

Component-wise clamping of values, similar to f64::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

Will panic if min is greater than max when glam_assert is enabled.

Returns

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct DVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```

Example 2 (javascript):
```javascript
445fn spherical_polar_to_cartesian(p: DVec2) -> DVec3 {
446    let (sin_theta, cos_theta) = p.x.sin_cos();
447    let (sin_phi, cos_phi) = p.y.sin_cos();
448    DVec3::new(cos_theta * sin_phi, sin_theta * sin_phi, cos_phi)
449}
```

Example 3 (javascript):
```javascript
128fn spherical_polar_to_cartesian(p: DVec2) -> DVec3 {
129    let (sin_theta, cos_theta) = p.x.sin_cos();
130    let (sin_phi, cos_phi) = p.y.sin_cos();
131    DVec3::new(cos_theta * sin_phi, sin_theta * sin_phi, cos_phi)
132}
```

Example 4 (javascript):
```javascript
41fn setup(
42    mut commands: Commands,
43    mut meshes: ResMut<Assets<Mesh>>,
44    mut materials: ResMut<Assets<StandardMaterial>>,
45) {
46    warn!(include_str!("warning_string.txt"));
47
48    const LIGHT_RADIUS: f32 = 0.3;
49    const LIGHT_INTENSITY: f32 = 1000.0;
50    const RADIUS: f32 = 50.0;
51    const N_LIGHTS: usize = 100_000;
52
53    commands.spawn((
54        Mesh3d(meshes.add(Sphere::new(RADIUS).mesh().ico(9).unwrap())),
55        MeshMaterial3d(materials.add(Color::WHITE)),
56        Transform::from_scale(Vec3::NEG_ONE),
57    ));
58
59    let mesh = meshes.add(Cuboid::de
...
```

---

## Trait FloatPow Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/trait.FloatPow.html

**Contents:**
- Trait FloatPow Copy item path
- Required Methods§
    - fn squared(self) -> Self
    - fn cubed(self) -> Self
- Dyn Compatibility§
- Implementations on Foreign Types§
  - impl FloatPow for f32
    - fn squared(self) -> f32
    - fn cubed(self) -> f32
- Implementors§

This extension trait covers shortfall in determinacy from the lack of a libm counterpart to f32::powi. Use this for the common small exponents.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait FloatPow {
    // Required methods
    fn squared(self) -> Self;
    fn cubed(self) -> Self;
}
```

---

## Trait Vec3Swizzles Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/trait.Vec3Swizzles.html

**Contents:**
- Trait Vec3Swizzles Copy item path
- Required Associated Types§
    - type Vec2
    - type Vec4
- Required Methods§
    - fn xx(self) -> Self::Vec2
    - fn xy(self) -> Self::Vec2
    - fn with_xy(self, rhs: Self::Vec2) -> Self
    - fn xz(self) -> Self::Vec2
    - fn with_xz(self, rhs: Self::Vec2) -> Self

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait Vec3Swizzles:
    Sized
    + Copy
    + Clone {
    type Vec2;
    type Vec4;

Show 123 methods    // Required methods
    fn xx(self) -> Self::Vec2;
    fn xy(self) -> Self::Vec2;
    fn with_xy(self, rhs: Self::Vec2) -> Self;
    fn xz(self) -> Self::Vec2;
    fn with_xz(self, rhs: Self::Vec2) -> Self;
    fn yx(self) -> Self::Vec2;
    fn with_yx(self, rhs: Self::Vec2) -> Self;
    fn yy(self) -> Self::Vec2;
    fn yz(self) -> Self::Vec2;
    fn with_yz(self, rhs: Self::Vec2) -> Self;
    fn zx(self) -> Self::Vec2;
    fn with_zx(self, rhs: Self::Vec2) -> Self;
    fn zy(self) ->
...
```

---

## Module u64 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/u64/index.html

**Contents:**
- Module u64 Copy item path
- Structs§
- Functions§

bevy::mathModule u64 Copy item pathSource Expand descriptionu64 vector types. Structs§U64Vec2A 2-dimensional vector.U64Vec3A 3-dimensional vector.U64Vec4A 4-dimensional vector.Functions§u64vec2Creates a 2-dimensional vector.u64vec3Creates a 3-dimensional vector.u64vec4Creates a 4-dimensional vector.

---

## Module i16 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/i16/index.html

**Contents:**
- Module i16 Copy item path
- Structs§
- Functions§

bevy::mathModule i16 Copy item pathSource Expand descriptioni16 vector types. Structs§I16Vec2A 2-dimensional vector.I16Vec3A 3-dimensional vector.I16Vec4A 4-dimensional vector.Functions§i16vec2Creates a 2-dimensional vector.i16vec3Creates a 3-dimensional vector.i16vec4Creates a 4-dimensional vector.

---

## Module bounding Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/bounding/index.html

**Contents:**
- Module bounding Copy item path
- Structs§
- Traits§

This module contains traits and implements for working with bounding shapes

There are four traits used:

---

## Struct Isometry2d Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Isometry2d.html

**Contents:**
- Struct Isometry2d Copy item path
- §Example
- Fields§
- Implementations§
  - impl Isometry2d
    - pub const IDENTITY: Isometry2d
    - pub const fn new(translation: Vec2, rotation: Rot2) -> Isometry2d
      - Examples found in repository?
    - pub const fn from_rotation(rotation: Rot2) -> Isometry2d
    - pub const fn from_translation(translation: Vec2) -> Isometry2d

An isometry in two dimensions, representing a rotation followed by a translation. This can often be useful for expressing relative positions and transformations from one position to another.

In particular, this type represents a distance-preserving transformation known as a rigid motion or a direct motion, and belongs to the special Euclidean group SE(2). This includes translation and rotation, but excludes reflection.

For the three-dimensional version, see Isometry3d.

Isometries can be created from a given translation and rotation:

Or from separate parts:

The isometries can be used to transform points:

Isometries can also be composed together:

One common operation is to compute an isometry representing the relative positions of two objects for things like intersection tests. This can be done with an inverse transformation:

The rotational part of a two-dimensional isometry.

The translational part of a two-dimensional isometry.

The identity isometry which represents the rigid motion of not doing anything.

Create a two-dimensional isometry from a rotation and a translation.

Create a two-dimensional isometry from a rotation.

Create a two-dimensional isometry from a translation.

Create a two-dimensional isometry from a translation with the given x and y components.

The inverse isometry that undoes this one.

Compute iso1.inverse() * iso2 in a more efficient way for one-shot cases.

If the same isometry is used multiple times, it is more efficient to instead compute the inverse once and use that for each transformation.

Transform a point by rotating and translating it using this isometry.

Transform a point by rotating and translating it using the inverse of this isometry.

This is more efficient than iso.inverse().transform_point(point) for one-shot cases. If the same isometry is used multiple times, it is more efficient to instead compute the inverse once and use that for each transformation.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Isometry2d {
    pub rotation: Rot2,
    pub translation: Vec2,
}
```

Example 2 (javascript):
```javascript
let iso = Isometry2d::new(Vec2::new(2.0, 1.0), Rot2::degrees(90.0));
```

Example 3 (javascript):
```javascript
let iso1 = Isometry2d::from_translation(Vec2::new(2.0, 1.0));
let iso2 = Isometry2d::from_rotation(Rot2::degrees(90.0));
```

Example 4 (javascript):
```javascript
let iso = Isometry2d::new(Vec2::new(2.0, 1.0), Rot2::degrees(90.0));
let point = Vec2::new(4.0, 4.0);

// These are equivalent
let result = iso.transform_point(point);
let result = iso * point;

assert_eq!(result, Vec2::new(-2.0, 5.0));
```

---

## Function u64vec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.u64vec4.html

**Contents:**
- Function u64vec4 Copy item path

Creates a 4-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn u64vec4(x: u64, y: u64, z: u64, w: u64) -> U64Vec4
```

---

## Struct Dir4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Dir4.html

**Contents:**
- Struct Dir4 Copy item path
- Implementations§
  - impl Dir4
    - pub const X: Dir4
    - pub const Y: Dir4
    - pub const Z: Dir4
    - pub const W: Dir4
    - pub const NEG_X: Dir4
    - pub const NEG_Y: Dir4
    - pub const NEG_Z: Dir4

A normalized vector pointing in a direction in 4D space

A unit vector pointing along the positive X axis

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the positive W axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

A unit vector pointing along the negative Z axis.

A unit vector pointing along the negative W axis.

The directional axes.

Create a direction from a finite, nonzero Vec4, normalizing it.

Returns Err(InvalidDirectionError) if the length of the given vector is zero (or very close to zero), infinite, or NaN.

Create a Dir4 from a Vec4 that is already normalized.

value must be normalized, i.e its length must be 1.0.

Create a direction from a finite, nonzero Vec4, normalizing it and also returning its original length.

Returns Err(InvalidDirectionError) if the length of the given vector is zero (or very close to zero), infinite, or NaN.

Create a direction from its x, y, z, and w components.

Returns Err(InvalidDirectionError) if the length of the vector formed by the components is zero (or very close to zero), infinite, or NaN.

Create a direction from its x, y, z, and w components, assuming the resulting vector is normalized.

The vector produced from x, y, z, and w must be normalized, i.e its length must be 1.0.

Returns the inner Vec4

Returns self after an approximate normalization, assuming the value is already nearly normalized. Useful for preventing numerical error accumulation.

Converts self to [x, y, z, w]

Moves towards rhs based on the value d.

When d is 0.0, the result will be equal to self. When d is equal to self.distance(rhs), the result will be equal to rhs. Will not go past rhs.

Casts all elements of self to f64.

Casts all elements of self to i8.

Casts all elements of self to u8.

Casts all elements of self to i16.

Casts all elements of self to u16.

Casts all elements of self to i32.

Casts all elements of self to u32.

Casts all elements of self to i64.

Casts all elements of self to u64.

Casts all elements of self to usize.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Dir4(/* private fields */);
```

---

## Struct IVec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.IVec4.html

**Contents:**
- Struct IVec4 Copy item path
- Fields§
- Implementations§
  - impl IVec4
    - pub const ZERO: IVec4
    - pub const ONE: IVec4
    - pub const NEG_ONE: IVec4
    - pub const MIN: IVec4
    - pub const MAX: IVec4
    - pub const X: IVec4

A 4-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the positive W axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

A unit vector pointing along the negative Z axis.

A unit vector pointing along the negative W axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z, w]

Creates a vector from the first 4 values in slice.

Panics if slice is less than 4 elements long.

Writes the elements of self to the first 4 elements in slice.

Panics if slice is less than 4 elements long.

Creates a 3D vector from the x, y and z elements of self, discarding w.

Truncation to IVec3 may also be performed by using self.xyz().

Creates a 4D vector from self with the given value of x.

Creates a 4D vector from self with the given value of y.

Creates a 4D vector from self with the given value of z.

Creates a 4D vector from self with the given value of w.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to i32::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of al

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct IVec4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}
```

---

## Crate math Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/index.html

**Contents:**
- Crate math Copy item path
- Modules§
- Structs§
- Enums§
- Traits§
- Functions§

Provides math types and functionality for the Bevy game engine.

The commonly used types are vectors like Vec2 and Vec3, matrices like Mat2, Mat3 and Mat4 and orientation representations like Quat.

---

## Trait VectorSpace Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/trait.VectorSpace.html

**Contents:**
- Trait VectorSpace Copy item path
- Required Associated Constants§
    - const ZERO: Self
- Required Associated Types§
    - type Scalar: ScalarField
- Provided Methods§
    - fn lerp(self, rhs: Self, t: Self::Scalar) -> Self
- Dyn Compatibility§
- Implementors§
  - impl VectorSpace for Laba

A type that supports the mathematical operations of a real vector space, irrespective of dimension. In particular, this means that the implementing type supports:

Within the limitations of floating point arithmetic, all the following are required to hold:

Note that, because implementing types use floating point arithmetic, they are not required to actually implement PartialEq or Eq.

The zero vector, which is the identity of addition for the vector space type.

The scalar type of this vector space.

Perform vector space linear interpolation between this element and another, based on the parameter t. When t is 0, self is recovered. When t is 1, rhs is recovered.

Note that the value of t is not clamped by this function, so extrapolating outside of the interval [0,1] is allowed.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (javascript):
```javascript
pub trait VectorSpace:
    Mul<Self::Scalar, Output = Self>
    + Div<Self::Scalar, Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
    + Default
    + Debug
    + Clone
    + Copy {
    type Scalar: ScalarField;

    const ZERO: Self;

    // Provided method
    fn lerp(self, rhs: Self, t: Self::Scalar) -> Self { ... }
}
```

---

## Struct Dir2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Dir2.html

**Contents:**
- Struct Dir2 Copy item path
- Implementations§
  - impl Dir2
    - pub const X: Dir2
    - pub const Y: Dir2
    - pub const NEG_X: Dir2
    - pub const NEG_Y: Dir2
    - pub const AXES: [Dir2; 2]
    - pub const NORTH: Dir2
    - pub const SOUTH: Dir2

A normalized vector pointing in a direction in 2D space

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

The directional axes.

The “north” direction, equivalent to Dir2::Y.

The “south” direction, equivalent to Dir2::NEG_Y.

The “east” direction, equivalent to Dir2::X.

The “west” direction, equivalent to Dir2::NEG_X.

The “north-east” direction, between Dir2::NORTH and Dir2::EAST.

The “north-west” direction, between Dir2::NORTH and Dir2::WEST.

The “south-east” direction, between Dir2::SOUTH and Dir2::EAST.

The “south-west” direction, between Dir2::SOUTH and Dir2::WEST.

Create a direction from a finite, nonzero Vec2, normalizing it.

Returns Err(InvalidDirectionError) if the length of the given vector is zero (or very close to zero), infinite, or NaN.

Create a Dir2 from a Vec2 that is already normalized.

value must be normalized, i.e its length must be 1.0.

Create a direction from a finite, nonzero Vec2, normalizing it and also returning its original length.

Returns Err(InvalidDirectionError) if the length of the given vector is zero (or very close to zero), infinite, or NaN.

Create a direction from its x and y components.

Returns Err(InvalidDirectionError) if the length of the vector formed by the components is zero (or very close to zero), infinite, or NaN.

Create a direction from its x and y components, assuming the resulting vector is normalized.

The vector produced from x and y must be normalized, i.e its length must be 1.0.

Returns the inner Vec2

Performs a spherical linear interpolation between self and rhs based on the value s.

This corresponds to interpolating between the two directions at a constant angular velocity.

When s == 0.0, the result will be equal to self. When s == 1.0, the result will be equal to rhs.

Get the rotation that rotates this direction to other.

Get the rotation that rotates other to this direction.

Get the rotation that rotates the X-axis to this direction.

Get the rotation that rotates this direction to the X-axis.

Get the rotation that rotates the Y-axis to this direction.

Get the rotation that rotates this direction to the Y-axis.

Returns self after an approximate normalization, assuming the value is already nearly normalized. Useful for preventing numerical error accumulation. See Dir3::fast_renormalize for an example of when such error accumul

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Dir2(/* private fields */);
```

Example 2 (javascript):
```javascript
285fn get_and_draw_ray(gizmos: &mut Gizmos, time: &Time) -> RayCast2d {
286    let ray = Vec2::new(ops::cos(time.elapsed_secs()), ops::sin(time.elapsed_secs()));
287    let dist = 150. + ops::sin(0.5 * time.elapsed_secs()).abs() * 500.;
288
289    let aabb_ray = Ray2d {
290        origin: ray * 250.,
291        direction: Dir2::new_unchecked(-ray),
292    };
293    let ray_cast = RayCast2d::from_ray(aabb_ray, dist - 20.);
294
295    draw_ray(gizmos, &ray_cast);
296    ray_cast
297}
```

Example 3 (unknown):
```unknown
202fn setup(mut commands: Commands) {
203    commands.spawn(Camera2d);
204
205    commands.spawn((
206        Transform::from_xyz(-OFFSET_X, OFFSET_Y, 0.),
207        Shape::Circle(Circle::new(45.)),
208        DesiredVolume::Aabb,
209        Intersects::default(),
210    ));
211
212    commands.spawn((
213        Transform::from_xyz(0., OFFSET_Y, 0.),
214        Shape::Rectangle(Rectangle::new(80., 80.)),
215        Spin,
216        DesiredVolume::Circle,
217        Intersects::default(),
218    ));
219
220    commands.spawn((
221        Transform::from_xyz(OFFSET_X, OFFSET_Y, 0.),
222       
...
```

Example 4 (javascript):
```javascript
let dir1 = Dir2::X;
let dir2 = Dir2::Y;

let result1 = dir1.slerp(dir2, 1.0 / 3.0);
#[cfg(feature = "approx")]
assert_relative_eq!(result1, Dir2::from_xy(0.75_f32.sqrt(), 0.5).unwrap());

let result2 = dir1.slerp(dir2, 0.5);
#[cfg(feature = "approx")]
assert_relative_eq!(result2, Dir2::from_xy(0.5_f32.sqrt(), 0.5_f32.sqrt()).unwrap());
```

---

## Struct U8Vec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.U8Vec3.html

**Contents:**
- Struct U8Vec3 Copy item path
- Fields§
- Implementations§
  - impl U8Vec3
    - pub const ZERO: U8Vec3
    - pub const ONE: U8Vec3
    - pub const MIN: U8Vec3
    - pub const MAX: U8Vec3
    - pub const X: U8Vec3
    - pub const Y: U8Vec3

A 3-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z]

Creates a vector from the first 3 values in slice.

Panics if slice is less than 3 elements long.

Writes the elements of self to the first 3 elements in slice.

Panics if slice is less than 3 elements long.

Creates a 4D vector from self and the given w value.

Creates a 2D vector from the x and y elements of self, discarding z.

Truncation may also be performed by using self.xy().

Creates a 3D vector from self with the given value of x.

Creates a 3D vector from self with the given value of y.

Creates a 3D vector from self with the given value of z.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Computes the cross product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to u8::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes [self.x == rhs.x, self.y == r

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct U8Vec3 {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}
```

---

## Function i8vec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.i8vec3.html

**Contents:**
- Function i8vec3 Copy item path

Creates a 3-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn i8vec3(x: i8, y: i8, z: i8) -> I8Vec3
```

---

## Struct UVec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.UVec2.html

**Contents:**
- Struct UVec2 Copy item path
- Fields§
- Implementations§
  - impl UVec2
    - pub const ZERO: UVec2
    - pub const ONE: UVec2
    - pub const MIN: UVec2
    - pub const MAX: UVec2
    - pub const X: UVec2
    - pub const Y: UVec2

A 2-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y]

Creates a vector from the first 2 values in slice.

Panics if slice is less than 2 elements long.

Writes the elements of self to the first 2 elements in slice.

Panics if slice is less than 2 elements long.

Creates a 3D vector from self and the given z value.

Creates a 2D vector from self with the given value of x.

Creates a 2D vector from self with the given value of y.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to u32::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes [self.x == rhs.x, self.y == rhs.y, ..] for all elements.

Returns a vector mask containing the result of a != comparison for each element of self and rhs.

In other words this computes [self.x != rhs.x, self.y != rhs.y, ..] for all elements.

Returns a vector mask containing the result of a >= comparison for

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}
```

Example 2 (javascript):
```javascript
29const SIZE: UVec2 = UVec2::new(1280 / DISPLAY_FACTOR, 720 / DISPLAY_FACTOR);
```

Example 3 (javascript):
```javascript
195    const SECTIONS: [Option<URect>; 5] = [
196        Some(URect {
197            min: UVec2::ZERO,
198            max: UVec2::splat(RECT_SIZE),
199        }),
200        Some(URect {
201            min: UVec2::new(RECT_SIZE, 0),
202            max: UVec2::new(2 * RECT_SIZE, RECT_SIZE),
203        }),
204        Some(URect {
205            min: UVec2::new(0, RECT_SIZE),
206            max: UVec2::new(RECT_SIZE, 2 * RECT_SIZE),
207        }),
208        Some(URect {
209            min: UVec2::new(RECT_SIZE, RECT_SIZE),
210            max: UVec2::splat(2 * RECT_SIZE),
211        }),
212      
...
```

Example 4 (javascript):
```javascript
65fn spawn_fake_player(
66    mut commands: Commands,
67    mut meshes: ResMut<Assets<Mesh>>,
68    mut materials: ResMut<Assets<ColorMaterial>>,
69    chunk: Single<&TilemapChunk>,
70) {
71    let mut transform = chunk.calculate_tile_transform(UVec2::new(0, 0));
72    transform.translation.z = 1.;
73
74    commands.spawn((
75        Mesh2d(meshes.add(Rectangle::new(8., 8.))),
76        MeshMaterial2d(materials.add(Color::from(RED_400))),
77        transform,
78        MovePlayer,
79    ));
80
81    let mut transform = chunk.calculate_tile_transform(UVec2::new(5, 6));
82    transform.translati
...
```

---

## Function uvec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.uvec4.html

**Contents:**
- Function uvec4 Copy item path

Creates a 4-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn uvec4(x: u32, y: u32, z: u32, w: u32) -> UVec4
```

---

## Function u8vec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.u8vec2.html

**Contents:**
- Function u8vec2 Copy item path

Creates a 2-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn u8vec2(x: u8, y: u8) -> U8Vec2
```

---

## Struct I16Vec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.I16Vec4.html

**Contents:**
- Struct I16Vec4 Copy item path
- Fields§
- Implementations§
  - impl I16Vec4
    - pub const ZERO: I16Vec4
    - pub const ONE: I16Vec4
    - pub const NEG_ONE: I16Vec4
    - pub const MIN: I16Vec4
    - pub const MAX: I16Vec4
    - pub const X: I16Vec4

A 4-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the positive W axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

A unit vector pointing along the negative Z axis.

A unit vector pointing along the negative W axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z, w]

Creates a vector from the first 4 values in slice.

Panics if slice is less than 4 elements long.

Writes the elements of self to the first 4 elements in slice.

Panics if slice is less than 4 elements long.

Creates a 3D vector from the x, y and z elements of self, discarding w.

Truncation to I16Vec3 may also be performed by using self.xyz().

Creates a 4D vector from self with the given value of x.

Creates a 4D vector from self with the given value of y.

Creates a 4D vector from self with the given value of z.

Creates a 4D vector from self with the given value of w.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to i16::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct I16Vec4 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub w: i16,
}
```

---

## Struct UVec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.UVec3.html

**Contents:**
- Struct UVec3 Copy item path
- Fields§
- Implementations§
  - impl UVec3
    - pub const ZERO: UVec3
    - pub const ONE: UVec3
    - pub const MIN: UVec3
    - pub const MAX: UVec3
    - pub const X: UVec3
    - pub const Y: UVec3

A 3-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z]

Creates a vector from the first 3 values in slice.

Panics if slice is less than 3 elements long.

Writes the elements of self to the first 3 elements in slice.

Panics if slice is less than 3 elements long.

Creates a 4D vector from self and the given w value.

Creates a 2D vector from the x and y elements of self, discarding z.

Truncation may also be performed by using self.xy().

Creates a 3D vector from self with the given value of x.

Creates a 3D vector from self with the given value of y.

Creates a 3D vector from self with the given value of z.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Computes the cross product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to u32::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Returns a vector mask containing the result of a == comparison for each element of self and rhs.

In other words, this computes [self.x == rhs.x, self.y == 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct UVec3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
```

Example 2 (javascript):
```javascript
320    pub fn draw_gizmos(mut gizmos: Gizmos) {
321        gizmos.cuboid(
322            Transform::from_translation(Vec3::X * -1.75).with_scale(Vec3::splat(1.25)),
323            RED,
324        );
325        gizmos
326            .sphere(Isometry3d::from_translation(Vec3::X * -3.5), 0.75, GREEN)
327            .resolution(30_000 / 3);
328
329        // 3d grids with all variations of outer edges on or off
330        for i in 0..8 {
331            let x = 1.5 * (i % 4) as f32;
332            let y = 1.0 * (0.5 - (i / 4) as f32);
333            let mut grid = gizmos.grid_3d(
334               
...
```

Example 3 (javascript):
```javascript
38fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
39    commands.spawn(Camera2d);
40
41    // Create an image that we are going to draw into
42    let mut image = Image::new_fill(
43        // 2D image of size 256x256
44        Extent3d {
45            width: IMAGE_WIDTH,
46            height: IMAGE_HEIGHT,
47            depth_or_array_layers: 1,
48        },
49        TextureDimension::D2,
50        // Initialize it with a beige color
51        &(css::BEIGE.to_u8_array()),
52        // Use the same encoding as the color we set
53        TextureFormat::Rgba8UnormSrgb,
54 
...
```

Example 4 (javascript):
```javascript
525fn create_cubes(
526    image_assets: Res<Assets<Image>>,
527    mut commands: Commands,
528    irradiance_volumes: Query<(&IrradianceVolume, &GlobalTransform)>,
529    voxel_cube_parents: Query<Entity, With<VoxelCubeParent>>,
530    voxel_cubes: Query<Entity, With<VoxelCube>>,
531    example_assets: Res<ExampleAssets>,
532    mut voxel_visualization_material_assets: ResMut<Assets<VoxelVisualizationMaterial>>,
533) {
534    // If voxel cubes have already been spawned, don't do anything.
535    if !voxel_cubes.is_empty() {
536        return;
537    }
538
539    let Some(voxel_cube_parent) = 
...
```

---

## Module i8 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/i8/index.html

**Contents:**
- Module i8 Copy item path
- Structs§
- Functions§

bevy::mathModule i8 Copy item pathSource Expand descriptioni8 vector types. Structs§I8Vec2A 2-dimensional vector.I8Vec3A 3-dimensional vector.I8Vec4A 4-dimensional vector.Functions§i8vec2Creates a 2-dimensional vector.i8vec3Creates a 3-dimensional vector.i8vec4Creates a 4-dimensional vector.

---

## Function vec4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.vec4.html

**Contents:**
- Function vec4 Copy item path
      - Examples found in repository?

Creates a 4-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4
```

Example 2 (javascript):
```javascript
52fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
53    // Build a custom triangle mesh with colors
54    // We define a custom mesh because the examples only uses a limited
55    // set of vertex attributes for simplicity
56    let mesh = Mesh::new(
57        PrimitiveTopology::TriangleList,
58        RenderAssetUsages::default(),
59    )
60    .with_inserted_indices(Indices::U32(vec![0, 1, 2]))
61    .with_inserted_attribute(
62        Mesh::ATTRIBUTE_POSITION,
63        vec![
64            vec3(-0.5, -0.5, 0.0),
65            vec3(0.5, -0.5, 0.0),
66            vec3(0.0
...
```

Example 3 (unknown):
```unknown
180fn spawn_water(
181    commands: &mut Commands,
182    asset_server: &AssetServer,
183    meshes: &mut Assets<Mesh>,
184    water_materials: &mut Assets<ExtendedMaterial<StandardMaterial, Water>>,
185) {
186    commands.spawn((
187        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1.0)))),
188        MeshMaterial3d(water_materials.add(ExtendedMaterial {
189            base: StandardMaterial {
190                base_color: BLACK.into(),
191                perceptual_roughness: 0.0,
192                ..default()
193            },
194            extension: Water {
195               
...
```

---

## Module f32 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/f32/index.html

**Contents:**
- Module f32 Copy item path
- Structs§
- Functions§

f32 vector, quaternion and matrix types.

---

## Function uvec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.uvec3.html

**Contents:**
- Function uvec3 Copy item path
      - Examples found in repository?

Creates a 3-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn uvec3(x: u32, y: u32, z: u32) -> UVec3
```

Example 2 (javascript):
```javascript
525fn create_cubes(
526    image_assets: Res<Assets<Image>>,
527    mut commands: Commands,
528    irradiance_volumes: Query<(&IrradianceVolume, &GlobalTransform)>,
529    voxel_cube_parents: Query<Entity, With<VoxelCubeParent>>,
530    voxel_cubes: Query<Entity, With<VoxelCube>>,
531    example_assets: Res<ExampleAssets>,
532    mut voxel_visualization_material_assets: ResMut<Assets<VoxelVisualizationMaterial>>,
533) {
534    // If voxel cubes have already been spawned, don't do anything.
535    if !voxel_cubes.is_empty() {
536        return;
537    }
538
539    let Some(voxel_cube_parent) = 
...
```

---

## Struct DMat2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.DMat2.html

**Contents:**
- Struct DMat2 Copy item path
- Fields§
- Implementations§
  - impl DMat2
    - pub const ZERO: DMat2
    - pub const IDENTITY: DMat2
    - pub const NAN: DMat2
    - pub const fn from_cols(x_axis: DVec2, y_axis: DVec2) -> DMat2
    - pub const fn from_cols_array(m: &[f64; 4]) -> DMat2
    - pub const fn to_cols_array(&self) -> [f64; 4]

A 2x2 column major matrix.

A 2x2 matrix with all elements set to 0.0.

A 2x2 identity matrix, where all diagonal elements are 1, and all off-diagonal elements are 0.

Creates a 2x2 matrix from two column vectors.

Creates a 2x2 matrix from a [f64; 4] array stored in column major order. If your data is stored in row major you will need to transpose the returned matrix.

Creates a [f64; 4] array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates a 2x2 matrix from a [[f64; 2]; 2] 2D array stored in column major order. If your data is in row major order you will need to transpose the returned matrix.

Creates a [[f64; 2]; 2] 2D array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates a 2x2 matrix with its diagonal set to diagonal and all other entries set to 0.

Creates a 2x2 matrix containing the combining non-uniform scale and rotation of angle (in radians).

Creates a 2x2 matrix containing a rotation of angle (in radians).

Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.

Creates a 2x2 matrix from the minor of the given 3x3 matrix, discarding the ith column and jth row.

Panics if i or j is greater than 2.

Creates a 2x2 matrix from the first 4 values in slice.

Panics if slice is less than 4 elements long.

Writes the columns of self to the first 4 elements in slice.

Panics if slice is less than 4 elements long.

Returns the matrix column for the given index.

Panics if index is greater than 1.

Returns a mutable reference to the matrix column for the given index.

Panics if index is greater than 1.

Returns the matrix row for the given index.

Panics if index is greater than 1.

Returns true if, and only if, all elements are finite. If any element is either NaN, positive or negative infinity, this will return false.

Returns true if any elements are NaN.

Returns the transpose of self.

Returns the determinant of self.

Returns the inverse of self.

If the matrix is not invertible the returned matrix will be invalid.

Will panic if the determinant of self is zero when glam_assert is enabled.

Transforms a 2D vector.

Multiplies two 2x2 matrices.

Adds two 2x2 matrices.

Subtracts two 2x2 matrices.

Multiplies a 2x2 matrix by a scalar.

Divides a 2x2 matrix by a scalar.

Returns true if the absolute difference of all elements between self and rhs is less than or equal to max_abs_diff.

This can be used to co

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct DMat2 {
    pub x_axis: DVec2,
    pub y_axis: DVec2,
}
```

---

## Function i16vec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.i16vec2.html

**Contents:**
- Function i16vec2 Copy item path

Creates a 2-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn i16vec2(x: i16, y: i16) -> I16Vec2
```

---

## Function i8vec2 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.i8vec2.html

**Contents:**
- Function i8vec2 Copy item path

Creates a 2-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn i8vec2(x: i8, y: i8) -> I8Vec2
```

---

## Struct Affine3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Affine3.html

**Contents:**
- Struct Affine3 Copy item path
- Fields§
- Implementations§
  - impl Affine3
    - pub fn to_transpose(&self) -> [Vec4; 3]
      - Examples found in repository?
    - pub fn inverse_transpose_3x3(&self) -> ([Vec4; 2], f32)
      - Examples found in repository?
- Trait Implementations§
  - impl From<&Affine3> for Affine3A

Reduced-size version of glam::Affine3A for use when storage has significant performance impact. Convert to glam::Affine3A to do non-trivial calculations.

Scaling, rotation, shears, and other non-translation affine transforms

Calculates the transpose of the affine 4x3 matrix to a 3x4 and formats it for packing into GPU buffers

Calculates the inverse transpose of the 3x3 matrix and formats it for packing into GPU buffers

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Affine3 {
    pub matrix3: Mat3,
    pub translation: Vec3,
}
```

Example 2 (javascript):
```javascript
342    fn get_batch_data(
343        (mesh_instances, _render_assets, mesh_allocator): &SystemParamItem<Self::Param>,
344        (_entity, main_entity): (Entity, MainEntity),
345    ) -> Option<(Self::BufferData, Option<Self::CompareData>)> {
346        let RenderMeshInstances::CpuBuilding(ref mesh_instances) = **mesh_instances else {
347            error!(
348                "`get_batch_data` should never be called in GPU mesh uniform \
349                building mode"
350            );
351            return None;
352        };
353        let mesh_instance = mesh_instances.get(&main_entity)?
...
```

Example 3 (javascript):
```javascript
342    fn get_batch_data(
343        (mesh_instances, _render_assets, mesh_allocator): &SystemParamItem<Self::Param>,
344        (_entity, main_entity): (Entity, MainEntity),
345    ) -> Option<(Self::BufferData, Option<Self::CompareData>)> {
346        let RenderMeshInstances::CpuBuilding(ref mesh_instances) = **mesh_instances else {
347            error!(
348                "`get_batch_data` should never be called in GPU mesh uniform \
349                building mode"
350            );
351            return None;
352        };
353        let mesh_instance = mesh_instances.get(&main_entity)?
...
```

---

## Struct DMat4 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.DMat4.html

**Contents:**
- Struct DMat4 Copy item path
- Fields§
- Implementations§
  - impl DMat4
    - pub const ZERO: DMat4
    - pub const IDENTITY: DMat4
    - pub const NAN: DMat4
    - pub const fn from_cols( x_axis: DVec4, y_axis: DVec4, z_axis: DVec4, w_axis: DVec4, ) -> DMat4
    - pub const fn from_cols_array(m: &[f64; 16]) -> DMat4
    - pub const fn to_cols_array(&self) -> [f64; 16]

A 4x4 column major matrix.

This 4x4 matrix type features convenience methods for creating and using affine transforms and perspective projections. If you are primarily dealing with 3D affine transformations considering using DAffine3 which is faster than a 4x4 matrix for some affine operations.

Affine transformations including 3D translation, rotation and scale can be created using methods such as Self::from_translation(), Self::from_quat(), Self::from_scale() and Self::from_scale_rotation_translation().

Orthographic projections can be created using the methods Self::orthographic_lh() for left-handed coordinate systems and Self::orthographic_rh() for right-handed systems. The resulting matrix is also an affine transformation.

The Self::transform_point3() and Self::transform_vector3() convenience methods are provided for performing affine transformations on 3D vectors and points. These multiply 3D inputs as 4D vectors with an implicit w value of 1 for points and 0 for vectors respectively. These methods assume that Self contains a valid affine transform.

Perspective projections can be created using methods such as Self::perspective_lh(), Self::perspective_infinite_lh() and Self::perspective_infinite_reverse_lh() for left-handed co-ordinate systems and Self::perspective_rh(), Self::perspective_infinite_rh() and Self::perspective_infinite_reverse_rh() for right-handed co-ordinate systems.

The resulting perspective project can be use to transform 3D vectors as points with perspective correction using the Self::project_point3() convenience method.

A 4x4 matrix with all elements set to 0.0.

A 4x4 identity matrix, where all diagonal elements are 1, and all off-diagonal elements are 0.

Creates a 4x4 matrix from four column vectors.

Creates a 4x4 matrix from a [f64; 16] array stored in column major order. If your data is stored in row major you will need to transpose the returned matrix.

Creates a [f64; 16] array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates a 4x4 matrix from a [[f64; 4]; 4] 4D array stored in column major order. If your data is in row major order you will need to transpose the returned matrix.

Creates a [[f64; 4]; 4] 4D array storing data in column major order. If you require data in row major order transpose the matrix first.

Creates a 4x4 matrix with its diagonal set to diagonal and all other entries set to 0.

Creates an affine transformation matrix from the given 3D

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct DMat4 {
    pub x_axis: DVec4,
    pub y_axis: DVec4,
    pub z_axis: DVec4,
    pub w_axis: DVec4,
}
```

---

## Function ivec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/fn.ivec3.html

**Contents:**
- Function ivec3 Copy item path

Creates a 3-dimensional vector.

**Examples:**

Example 1 (javascript):
```javascript
pub const fn ivec3(x: i32, y: i32, z: i32) -> IVec3
```

---

## Struct I64Vec3 Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.I64Vec3.html

**Contents:**
- Struct I64Vec3 Copy item path
- Fields§
- Implementations§
  - impl I64Vec3
    - pub const ZERO: I64Vec3
    - pub const ONE: I64Vec3
    - pub const NEG_ONE: I64Vec3
    - pub const MIN: I64Vec3
    - pub const MAX: I64Vec3
    - pub const X: I64Vec3

A 3-dimensional vector.

A unit vector pointing along the positive X axis.

A unit vector pointing along the positive Y axis.

A unit vector pointing along the positive Z axis.

A unit vector pointing along the negative X axis.

A unit vector pointing along the negative Y axis.

A unit vector pointing along the negative Z axis.

Creates a new vector.

Creates a vector with all elements set to v.

Returns a vector containing each element of self modified by a mapping function f.

Creates a vector from the elements in if_true and if_false, selecting which to use for each element of self.

A true element in the mask uses the corresponding element from if_true, and false uses the element from if_false.

Creates a new vector from an array.

Converts self to [x, y, z]

Creates a vector from the first 3 values in slice.

Panics if slice is less than 3 elements long.

Writes the elements of self to the first 3 elements in slice.

Panics if slice is less than 3 elements long.

Creates a 4D vector from self and the given w value.

Creates a 2D vector from the x and y elements of self, discarding z.

Truncation may also be performed by using self.xy().

Creates a 3D vector from self with the given value of x.

Creates a 3D vector from self with the given value of y.

Creates a 3D vector from self with the given value of z.

Computes the dot product of self and rhs.

Returns a vector where every component is the dot product of self and rhs.

Computes the cross product of self and rhs.

Returns a vector containing the minimum values for each element of self and rhs.

In other words this computes [min(x, rhs.x), min(self.y, rhs.y), ..].

Returns a vector containing the maximum values for each element of self and rhs.

In other words this computes [max(self.x, rhs.x), max(self.y, rhs.y), ..].

Component-wise clamping of values, similar to i64::clamp.

Each element in min must be less-or-equal to the corresponding element in max.

Will panic if min is greater than max when glam_assert is enabled.

Returns the horizontal minimum of self.

In other words this computes min(x, y, ..).

Returns the horizontal maximum of self.

In other words this computes max(x, y, ..).

Returns the index of the first minimum element of self.

Returns the index of the first maximum element of self.

Returns the sum of all elements of self.

In other words, this computes self.x + self.y + ...

Returns the product of all elements of self.

In other words, this computes self.x * self.y * ...

Ret

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct I64Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
```

---

## Struct Ray3d Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/math/struct.Ray3d.html

**Contents:**
- Struct Ray3d Copy item path
- Fields§
- Implementations§
  - impl Ray3d
    - pub const fn new(origin: Vec3, direction: Dir3) -> Ray3d
      - Examples found in repository?
    - pub fn get_point(&self, distance: f32) -> Vec3
      - Examples found in repository?
    - pub fn intersect_plane( &self, plane_origin: Vec3, plane: InfinitePlane3d, ) -> Option<f32>
      - Examples found in repository?

An infinite half-line starting at origin and going in direction in 3D space.

The origin of the ray.

The direction of the ray.

Create a new Ray3d from a given origin and direction

Get a point at a given distance along the ray

Get the distance to a plane if the ray intersects it

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Ray3d {
    pub origin: Vec3,
    pub direction: Dir3,
}
```

Example 2 (javascript):
```javascript
23fn bouncing_raycast(
24    mut ray_cast: MeshRayCast,
25    mut gizmos: Gizmos,
26    time: Res<Time>,
27    // The ray map stores rays cast by the cursor
28    ray_map: Res<RayMap>,
29) {
30    // Cast an automatically moving ray and bounce it off of surfaces
31    let t = ops::cos((time.elapsed_secs() - 4.0).max(0.0) * LASER_SPEED) * PI;
32    let ray_pos = Vec3::new(ops::sin(t), ops::cos(3.0 * t) * 0.5, ops::cos(t)) * 0.5;
33    let ray_dir = Dir3::new(-ray_pos).unwrap();
34    let ray = Ray3d::new(ray_pos, ray_dir);
35    gizmos.sphere(ray_pos, 0.1, Color::WHITE);
36    bounce_ray(ray, &
...
```

Example 3 (javascript):
```javascript
13fn draw_cursor(
14    camera_query: Single<(&Camera, &GlobalTransform)>,
15    ground: Single<&GlobalTransform, With<Ground>>,
16    window: Single<&Window>,
17    mut gizmos: Gizmos,
18) {
19    let (camera, camera_transform) = *camera_query;
20
21    if let Some(cursor_position) = window.cursor_position()
22        // Calculate a ray pointing from the camera into the world based on the cursor's position.
23        && let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position)
24        // Calculate if and at what distance the ray is hitting the ground plane.
25        && let 
...
```

Example 4 (javascript):
```javascript
13fn draw_cursor(
14    camera_query: Single<(&Camera, &GlobalTransform)>,
15    ground: Single<&GlobalTransform, With<Ground>>,
16    window: Single<&Window>,
17    mut gizmos: Gizmos,
18) {
19    let (camera, camera_transform) = *camera_query;
20
21    if let Some(cursor_position) = window.cursor_position()
22        // Calculate a ray pointing from the camera into the world based on the cursor's position.
23        && let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position)
24        // Calculate if and at what distance the ray is hitting the ground plane.
25        && let 
...
```

---
