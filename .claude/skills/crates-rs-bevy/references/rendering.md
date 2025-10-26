# Crates-Rs-Bevy - Rendering

**Pages:** 302

---

## Struct RenderMaterialBindings Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.RenderMaterialBindings.html

**Contents:**
- Struct RenderMaterialBindings Copy item path
- Methods from Deref<Target = HashMap<UntypedAssetId, MaterialBindingId>>§
    - pub fn hasher(&self) -> &S
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn keys(&self) -> Keys<'_, K, V> ⓘ
      - §Examples
      - Examples found in repository?
    - pub fn values(&self) -> Values<'_, K, V> ⓘ
      - §Examples

A resource that maps each untyped material ID to its binding.

This duplicates information in RenderAssets<M>, but it doesn’t have the M type parameter, so it can be used in untyped contexts like crate::render::mesh::collect_meshes_for_gpu_building.

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

Shrinks the capacity of the map as much as possible. It will drop down as much as possible while maintaining the internal rules and possibly leaving some space in accordance with the resize 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderMaterialBindings(/* private fields */);
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

## Struct MeshVertexBufferLayoutRef Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.MeshVertexBufferLayoutRef.html

**Contents:**
- Struct MeshVertexBufferLayoutRef Copy item path
- Tuple Fields§
- Trait Implementations§
  - impl Clone for MeshVertexBufferLayoutRef
    - fn clone(&self) -> MeshVertexBufferLayoutRef
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for MeshVertexBufferLayoutRef
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Hash for MeshVertexBufferLayoutRef
    - fn hash<H>(&self, state: &mut H)where H: Hasher,

Describes the layout of the mesh vertices in GPU memory.

At most one copy of a mesh vertex buffer layout ever exists in GPU memory at once. Therefore, comparing these for equality requires only a single pointer comparison, and this type’s PartialEq and Hash implementations take advantage of this. To that end, this type doesn’t implement bevy_derive::Deref or bevy_derive::DerefMut in order to reduce the possibility of accidental deep comparisons, which would be needlessly expensive.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshVertexBufferLayoutRef(pub Arc<MeshVertexBufferLayout>);
```

---

## Struct MaterialExtensionPipeline Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialExtensionPipeline.html

**Contents:**
- Struct MaterialExtensionPipeline Copy item path
- Fields§
- Auto Trait Implementations§
  - impl Freeze for MaterialExtensionPipeline
  - impl !RefUnwindSafe for MaterialExtensionPipeline
  - impl Send for MaterialExtensionPipeline
  - impl Sync for MaterialExtensionPipeline
  - impl Unpin for MaterialExtensionPipeline
  - impl !UnwindSafe for MaterialExtensionPipeline
- Blanket Implementations§

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialExtensionPipeline {
    pub mesh_pipeline: MeshPipeline,
}
```

---

## Module graph Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/graph/index.html

**Contents:**
- Module graph Copy item path
- Structs§

bevy::renderModule graph Copy item pathSource Structs§CameraDriverLabel

---

## Struct Gltf Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/struct.Gltf.html

**Contents:**
- Struct Gltf Copy item path
- Fields§
- Trait Implementations§
  - impl Debug for Gltf
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl TypePath for Gltf
    - fn type_path() -> &'static str
    - fn short_type_path() -> &'static str
    - fn type_ident() -> Option<&'static str>
    - fn crate_name() -> Option<&'static str>

Representation of a loaded glTF file.

All scenes loaded from the glTF file.

Named scenes loaded from the glTF file.

All meshes loaded from the glTF file.

Named meshes loaded from the glTF file.

All materials loaded from the glTF file.

Named materials loaded from the glTF file.

All nodes loaded from the glTF file.

Named nodes loaded from the glTF file.

All skins loaded from the glTF file.

Named skins loaded from the glTF file.

Default scene to be displayed.

All animations loaded from the glTF file.

Named animations loaded from the glTF file.

The gltf root of the gltf asset, see https://docs.rs/gltf/latest/gltf/struct.Gltf.html. Only has a value when GltfLoaderSettings::include_source is true.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Gltf {Show 14 fields
    pub scenes: Vec<Handle<Scene>>,
    pub named_scenes: HashMap<Box<str>, Handle<Scene>>,
    pub meshes: Vec<Handle<GltfMesh>>,
    pub named_meshes: HashMap<Box<str>, Handle<GltfMesh>>,
    pub materials: Vec<Handle<StandardMaterial>>,
    pub named_materials: HashMap<Box<str>, Handle<StandardMaterial>>,
    pub nodes: Vec<Handle<GltfNode>>,
    pub named_nodes: HashMap<Box<str>, Handle<GltfNode>>,
    pub skins: Vec<Handle<GltfSkin>>,
    pub named_skins: HashMap<Box<str>, Handle<GltfSkin>>,
    pub default_scene: Option<Handle<Scene>>,
    pub animations: 
...
```

---

## Struct SpecializedMaterialPipelineCache Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.SpecializedMaterialPipelineCache.html

**Contents:**
- Struct SpecializedMaterialPipelineCache Copy item path
- Methods from Deref<Target = HashMap<RetainedViewEntity, SpecializedMaterialViewPipelineCache>>§
    - pub fn hasher(&self) -> &S
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn keys(&self) -> Keys<'_, K, V> ⓘ
      - §Examples
      - Examples found in repository?
    - pub fn values(&self) -> Values<'_, K, V> ⓘ
      - §Examples

Stores the SpecializedMaterialViewPipelineCache for each view.

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

Shrinks the capacity of the map with a lower limit. It will drop down no lower than the supplied limit while maintaining the internal

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct SpecializedMaterialPipelineCache { /* private fields */ }
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

## Struct MeshletMesh Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/experimental/meshlet/struct.MeshletMesh.html

**Contents:**
- Struct MeshletMesh Copy item path
- Implementations§
  - impl MeshletMesh
    - pub fn from_mesh( mesh: &Mesh, vertex_position_quantization_factor: u8, ) -> Result<MeshletMesh, MeshToMeshletMeshConversionError>
      - §Requirements
      - §Vertex precision
- Trait Implementations§
  - impl Clone for MeshletMesh
    - fn clone(&self) -> MeshletMesh
    - fn clone_from(&mut self, source: &Self)

A mesh that has been pre-processed into multiple small clusters of triangles called meshlets.

A bevy_mesh::Mesh can be converted to a MeshletMesh using MeshletMesh::from_mesh when the meshlet_processor cargo feature is enabled. The conversion step is very slow, and is meant to be ran once ahead of time, and not during runtime. This type of mesh is not suitable for dynamically generated geometry.

There are restrictions on the crate::Material functionality that can be used with this type of mesh.

See also super::MeshletMesh3d and super::MeshletPlugin.

Process a Mesh to generate a MeshletMesh.

This process is very slow, and should be done ahead of time, and not at runtime.

This function requires the meshlet_processor cargo feature.

vertex_position_quantization_factor is the amount of precision to use when quantizing vertex positions.

Vertices are snapped to the nearest (1/2^x)th of a centimeter, where x = vertex_position_quantization_factor. E.g. if x = 4, then vertices are snapped to the nearest 1/2^4 = 1/16th of a centimeter.

Use MESHLET_DEFAULT_VERTEX_POSITION_QUANTIZATION_FACTOR as a default, adjusting lower to save memory and disk space, and higher to prevent artifacts if needed.

To ensure that two different meshes do not have cracks between them when placed directly next to each other:

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshletMesh { /* private fields */ }
```

---

## Function get_mali_driver_version Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/fn.get_mali_driver_version.html

**Contents:**
- Function get_mali_driver_version Copy item path

Get the Mali driver version if the adapter is a Mali GPU.

**Examples:**

Example 1 (unknown):
```unknown
pub fn get_mali_driver_version(adapter_info: &RenderAdapterInfo) -> Option<u32>
```

---

## Struct ExtendedMaterial Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ExtendedMaterial.html

**Contents:**
- Struct ExtendedMaterial Copy item path
- Fields§
- Trait Implementations§
  - impl<B, E> AsBindGroup for ExtendedMaterial<B, E>where B: Material, E: MaterialExtension,
    - type Data = MaterialExtensionBindGroupData<<B as AsBindGroup>::Data, <E as AsBindGroup>::Data>
    - type Param = (<B as AsBindGroup>::Param, <E as AsBindGroup>::Param)
    - fn bindless_slot_count() -> Option<BindlessSlabResourceLimit>
    - fn bind_group_data(&self) -> <ExtendedMaterial<B, E> as AsBindGroup>::Data
    - fn unprepared_bind_group( &self, layout: &BindGroupLayout, render_device: &RenderDevice, _: &mut <<ExtendedMaterial<B, E> as AsBindGroup>::Param as SystemParam>::Item<'_, '_>, force_non_bindless: bool, ) -> Result<UnpreparedBindGroup, AsBindGroupError>
    - fn bind_group_layout_entries( render_device: &RenderDevice, force_non_bindless: bool, ) -> Vec<BindGroupLayoutEntry>where ExtendedMaterial<B, E>: Sized,

A material that extends a base Material with additional shaders and data.

The data from both materials will be combined and made available to the shader so that shader functions built for the base material (and referencing the base material bindings) will work as expected, and custom alterations based on custom data can also be used.

If the extension E returns a non-default result from vertex_shader() it will be used in place of the base material’s vertex shader.

If the extension E returns a non-default result from fragment_shader() it will be used in place of the base fragment shader.

When used with StandardMaterial as the base, all the standard material fields are present, so the pbr_fragment shader functions can be called from the extension shader (see the extended_material example).

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ExtendedMaterial<B, E>where
    B: Material,
    E: MaterialExtension,{
    pub base: B,
    pub extension: E,
}
```

---

## Struct MeshVertexBufferLayout Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.MeshVertexBufferLayout.html

**Contents:**
- Struct MeshVertexBufferLayout Copy item path
- Implementations§
  - impl MeshVertexBufferLayout
    - pub fn new( attribute_ids: Vec<MeshVertexAttributeId>, layout: VertexBufferLayout, ) -> MeshVertexBufferLayout
    - pub fn contains(&self, attribute_id: impl Into<MeshVertexAttributeId>) -> bool
      - Examples found in repository?
    - pub fn attribute_ids(&self) -> &[MeshVertexAttributeId]
    - pub fn layout(&self) -> &VertexBufferLayout
    - pub fn get_layout( &self, attribute_descriptors: &[VertexAttributeDescriptor], ) -> Result<VertexBufferLayout, MissingVertexAttributeError>
      - Examples found in repository?

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshVertexBufferLayout { /* private fields */ }
```

Example 2 (javascript):
```javascript
180    fn specialize(
181        &self,
182        key: Self::Key,
183        layout: &MeshVertexBufferLayoutRef,
184    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
185        // We will only use the position of the mesh in our shader so we only need to specify that
186        let mut vertex_attributes = Vec::new();
187        if layout.0.contains(Mesh::ATTRIBUTE_POSITION) {
188            // Make sure this matches the shader location
189            vertex_attributes.push(Mesh::ATTRIBUTE_POSITION.at_shader_location(0));
190        }
191        // This will automatical
...
```

Example 3 (javascript):
```javascript
186    fn specialize(
187        &self,
188        mesh_key: Self::Key,
189        layout: &MeshVertexBufferLayoutRef,
190    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
191        // Define the vertex attributes based on a standard bevy [`Mesh`]
192        let mut vertex_attributes = Vec::new();
193        if layout.0.contains(Mesh::ATTRIBUTE_POSITION) {
194            // Make sure this matches the shader location
195            vertex_attributes.push(Mesh::ATTRIBUTE_POSITION.at_shader_location(0));
196        }
197        if layout.0.contains(Mesh::ATTRIBUTE_COLOR) 
...
```

Example 4 (javascript):
```javascript
74    fn specialize(
75        _pipeline: &MaterialPipeline,
76        descriptor: &mut RenderPipelineDescriptor,
77        layout: &MeshVertexBufferLayoutRef,
78        _key: MaterialPipelineKey<Self>,
79    ) -> Result<(), SpecializedMeshPipelineError> {
80        let vertex_layout = layout.0.get_layout(&[
81            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
82            ATTRIBUTE_BLEND_COLOR.at_shader_location(1),
83        ])?;
84        descriptor.vertex.buffers = vec![vertex_layout];
85        Ok(())
86    }
```

---

## Function get_adreno_model Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/fn.get_adreno_model.html

**Contents:**
- Function get_adreno_model Copy item path

If the RenderAdapterInfo is a Qualcomm Adreno, returns its model number.

This lets us work around hardware bugs.

**Examples:**

Example 1 (unknown):
```unknown
pub fn get_adreno_model(adapter_info: &RenderAdapterInfo) -> Option<u32>
```

---

## Enum MaterialBindGroupAllocator Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/enum.MaterialBindGroupAllocator.html

**Contents:**
- Enum MaterialBindGroupAllocator Copy item path
- Variants§
  - Bindless(Box<MaterialBindGroupBindlessAllocator>)
  - NonBindless(Box<MaterialBindGroupNonBindlessAllocator>)
- Implementations§
  - impl MaterialBindGroupAllocator
    - pub fn new( render_device: &RenderDevice, label: Option<&'static str>, bindless_descriptor: Option<BindlessDescriptor>, bind_group_layout: BindGroupLayout, slab_capacity: Option<BindlessSlabResourceLimit>, ) -> MaterialBindGroupAllocator
      - Examples found in repository?
    - pub fn get(&self, group: MaterialBindGroupIndex) -> Option<MaterialSlab<'_>>
    - pub fn allocate_unprepared( &mut self, unprepared_bind_group: UnpreparedBindGroup, bind_group_layout: &BindGroupLayout, ) -> MaterialBindingId

A resource that places materials into bind groups and tracks their resources.

Internally, Bevy has separate allocators for bindless and non-bindless materials. This resource provides a common interface to the specific allocator in use.

The allocator used when the material is bindless.

The allocator used when the material is non-bindless.

Creates a new MaterialBindGroupAllocator managing the data for a single material.

Returns the slab with the given index, if one exists.

Allocates an UnpreparedBindGroup and returns the resulting binding ID.

This method should generally be preferred over Self::allocate_prepared, because this method supports both bindless and non-bindless bind groups. Only use Self::allocate_prepared if you need to prepare the bind group yourself.

Places a pre-prepared bind group into a slab.

For bindless materials, the allocator internally manages the bind groups, so calling this method will panic if this is a bindless allocator. Only non-bindless allocators support this method.

It’s generally preferred to use Self::allocate_unprepared, because that method supports both bindless and non-bindless allocators. Only use this method if you need to prepare the bind group yourself.

Deallocates the material with the given binding ID.

Any resources that are no longer referenced are removed from the slab.

Recreates any bind groups corresponding to slabs that have been modified since last calling MaterialBindGroupAllocator::prepare_bind_groups.

Uploads the contents of all buffers that this MaterialBindGroupAllocator manages to the GPU.

Non-bindless allocators don’t currently manage any buffers, so this method only has an effect for bindless allocators.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum MaterialBindGroupAllocator {
    Bindless(Box<MaterialBindGroupBindlessAllocator>),
    NonBindless(Box<MaterialBindGroupNonBindlessAllocator>),
}
```

Example 2 (javascript):
```javascript
75fn init_image_material_resources(
76    mut commands: Commands,
77    render_device: Res<RenderDevice>,
78    mut bind_group_allocators: ResMut<MaterialBindGroupAllocators>,
79) {
80    let bind_group_layout = render_device.create_bind_group_layout(
81        "image_material_layout",
82        &BindGroupLayoutEntries::sequential(
83            ShaderStages::FRAGMENT,
84            (
85                texture_2d(TextureSampleType::Float { filterable: false }),
86                sampler(SamplerBindingType::NonFiltering),
87            ),
88        ),
89    );
90    let sampler = render_device.
...
```

Example 3 (javascript):
```javascript
135    fn prepare_asset(
136        source_asset: Self::SourceAsset,
137        asset_id: AssetId<Self::SourceAsset>,
138        (
139            opaque_draw_functions,
140            material_layout,
141            asset_server,
142            bind_group_allocators,
143            render_material_bindings,
144            gpu_images,
145            image_material_sampler,
146        ): &mut SystemParamItem<Self::Param>,
147    ) -> std::result::Result<Self::ErasedAsset, PrepareAssetError<Self::SourceAsset>> {
148        let material_layout = material_layout.0.clone();
149        let draw_funct
...
```

Example 4 (javascript):
```javascript
135    fn prepare_asset(
136        source_asset: Self::SourceAsset,
137        asset_id: AssetId<Self::SourceAsset>,
138        (
139            opaque_draw_functions,
140            material_layout,
141            asset_server,
142            bind_group_allocators,
143            render_material_bindings,
144            gpu_images,
145            image_material_sampler,
146        ): &mut SystemParamItem<Self::Param>,
147    ) -> std::result::Result<Self::ErasedAsset, PrepareAssetError<Self::SourceAsset>> {
148        let material_layout = material_layout.0.clone();
149        let draw_funct
...
```

---

## Struct ImageLoaderSettings Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.ImageLoaderSettings.html

**Contents:**
- Struct ImageLoaderSettings Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for ImageLoaderSettings
    - fn clone(&self) -> ImageLoaderSettings
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for ImageLoaderSettings
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for ImageLoaderSettings
    - fn default() -> ImageLoaderSettings

Settings for loading an Image using an ImageLoader.

How to determine the image’s container format.

Forcibly use a specific wgpu_types::TextureFormat. Useful to control how data is handled when used in a shader. Ex: data that would be R16Uint that needs to be sampled as a float using R16Snorm.

Specifies whether image data is linear or in sRGB space when this is not determined by the image format.

ImageSampler to use when rendering - this does not affect the loading of the image data.

Where the asset will be used - see the docs on RenderAssetUsages for details.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ImageLoaderSettings {
    pub format: ImageFormatSetting,
    pub texture_format: Option<TextureFormat>,
    pub is_srgb: bool,
    pub sampler: ImageSampler,
    pub asset_usage: RenderAssetUsages,
}
```

---

## Struct SetMaterialBindGroup Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.SetMaterialBindGroup.html

**Contents:**
- Struct SetMaterialBindGroup Copy item path
- Trait Implementations§
  - impl<P, const I: usize> RenderCommand<P> for SetMaterialBindGroup<I>where P: PhaseItem,
    - type Param = (Res<'static, ErasedRenderAssets<PreparedMaterial>>, Res<'static, RenderMaterialInstances>, Res<'static, MaterialBindGroupAllocators>)
    - type ViewQuery = ()
    - type ItemQuery = ()
    - fn render<'w>( item: &P, _view: (), _item_query: Option<()>, _: <<SetMaterialBindGroup<I> as RenderCommand<P>>::Param as SystemParam>::Item<'w, '_>, pass: &mut TrackedRenderPass<'w>, ) -> RenderCommandResult
- Auto Trait Implementations§
  - impl<const I: usize> Freeze for SetMaterialBindGroup<I>
  - impl<const I: usize> RefUnwindSafe for SetMaterialBindGroup<I>

Sets the bind group for a given Material at the configured I index.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (javascript):
```javascript
pub struct SetMaterialBindGroup<const I: usize>;
```

---

## Struct SetMeshViewEmptyBindGroup Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.SetMeshViewEmptyBindGroup.html

**Contents:**
- Struct SetMeshViewEmptyBindGroup Copy item path
- Trait Implementations§
  - impl<P, const I: usize> RenderCommand<P> for SetMeshViewEmptyBindGroup<I>where P: PhaseItem,
    - type Param = ()
    - type ViewQuery = (&'static MeshViewBindGroup,)
    - type ItemQuery = ()
    - fn render<'w>( _item: &P, _: <<<SetMeshViewEmptyBindGroup<I> as RenderCommand<P>>::ViewQuery as QueryData>::ReadOnly as QueryData>::Item<'w, '_>, _entity: Option<()>, _: <<SetMeshViewEmptyBindGroup<I> as RenderCommand<P>>::Param as SystemParam>::Item<'w, '_>, pass: &mut TrackedRenderPass<'w>, ) -> RenderCommandResult
- Auto Trait Implementations§
  - impl<const I: usize> Freeze for SetMeshViewEmptyBindGroup<I>
  - impl<const I: usize> RefUnwindSafe for SetMeshViewEmptyBindGroup<I>

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (javascript):
```javascript
pub struct SetMeshViewEmptyBindGroup<const I: usize>;
```

---

## Struct MeshVertexAttributeId Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.MeshVertexAttributeId.html

**Contents:**
- Struct MeshVertexAttributeId Copy item path
- Trait Implementations§
  - impl Clone for MeshVertexAttributeId
    - fn clone(&self) -> MeshVertexAttributeId
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for MeshVertexAttributeId
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl<'de> Deserialize<'de> for MeshVertexAttributeId
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<MeshVertexAttributeId, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,
  - impl From<MeshVertexAttribute> for MeshVertexAttributeId

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshVertexAttributeId(/* private fields */);
```

---

## Struct RenderTargetInfo Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.RenderTargetInfo.html

**Contents:**
- Struct RenderTargetInfo Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for RenderTargetInfo
    - fn clone(&self) -> RenderTargetInfo
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for RenderTargetInfo
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for RenderTargetInfo
    - fn default() -> RenderTargetInfo

Information about the current RenderTarget.

The physical size of this render target (in physical pixels, ignoring scale factor).

The scale factor of this render target.

When rendering to a window, typically it is a value greater or equal than 1.0, representing the ratio between the size of the window in physical pixels and the logical size of the window.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderTargetInfo {
    pub physical_size: UVec2,
    pub scale_factor: f32,
}
```

---

## Module core_3d Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/core_pipeline/core_3d/index.html

**Contents:**
- Module core_3d Copy item path
- Modules§
- Structs§
- Constants§
- Functions§

bevy::core_pipelineModule core_3d Copy item pathSource Modules§graphStructs§AlphaMask3dCore3dPluginMainOpaquePass3dNodeA bevy_render::render_graph::Node that runs the Opaque3d and AlphaMask3d ViewBinnedRenderPhasess.MainTransparentPass3dNodeA bevy_render::render_graph::Node that runs the Transparent3d ViewSortedRenderPhases.Opaque3dOpaque 3D BinnedPhaseItems.Opaque3dBatchSetKeyInformation that must be identical in order to place opaque meshes in the same batch set.Opaque3dBinKeyData that must be identical in order to batch phase items together.Transmissive3dTransparent3dViewTransmissionTextureConstants§CORE_3D_DEPTH_FORMATDEPTH_TEXTURE_SAMPLING_SUPPORTEDTrue if multisampled depth textures are supported on this platform.Functions§check_msaaextract_camera_prepass_phaseextract_core_3d_camera_phasesprepare_core_3d_depth_texturesprepare_core_3d_transmission_texturesprepare_prepass_textures

---

## Module deferred Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/deferred/index.html

**Contents:**
- Module deferred Copy item path
- Structs§
- Constants§
- Functions§

bevy::pbrModule deferred Copy item pathSource Structs§DeferredLightingLayoutDeferredLightingPipelineDeferredOpaquePass3dPbrLightingNodeDeferredPbrLightingPluginPbrDeferredLightingDepthIdComponent with a depth_id for specifying which corresponding materials should be rendered by this specific PBR deferred lighting pass.SkipDeferredLightingComponent to skip running the deferred lighting pass in DeferredOpaquePass3dPbrLightingNode for a specific view.Constants§DEFAULT_PBR_DEFERRED_LIGHTING_PASS_IDFunctions§init_deferred_lighting_layoutinsert_deferred_lighting_pass_id_componentprepare_deferred_lighting_pipelines

---

## Struct TransmittedShadowReceiver Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.TransmittedShadowReceiver.html

**Contents:**
- Struct TransmittedShadowReceiver Copy item path
- Trait Implementations§
  - impl Component for TransmittedShadowReceiverwhere TransmittedShadowReceiver: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior
    - fn on_add() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_insert() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_replace() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>

Add this component to make a Mesh3d using a PBR material with StandardMaterial::diffuse_transmission > 0.0 receive shadows on its diffuse transmission lobe. (i.e. its “backside”)

Not enabled by default, as it requires carefully setting up StandardMaterial::thickness (and potentially even baking a thickness texture!) to match the geometry of the mesh, in order to avoid self-shadow artifacts.

Note: Using NotShadowReceiver overrides this component.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct TransmittedShadowReceiver;
```

---

## Struct TextureAtlasSources Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.TextureAtlasSources.html

**Contents:**
- Struct TextureAtlasSources Copy item path
- Fields§
- Implementations§
  - impl TextureAtlasSources
    - pub fn texture_index(&self, texture: impl Into<AssetId<Image>>) -> Option<usize>
    - pub fn handle( &self, layout: Handle<TextureAtlasLayout>, texture: impl Into<AssetId<Image>>, ) -> Option<TextureAtlas>
      - Examples found in repository?
    - pub fn texture_rect( &self, layout: &TextureAtlasLayout, texture: impl Into<AssetId<Image>>, ) -> Option<URect>
    - pub fn uv_rect( &self, layout: &TextureAtlasLayout, texture: impl Into<AssetId<Image>>, ) -> Option<Rect>
- Trait Implementations§

Stores a mapping from sub texture handles to the related area index.

Generated by TextureAtlasBuilder.

Maps from a specific image handle to the index in textures where they can be found.

Retrieves the texture section index of the given texture handle.

Creates a TextureAtlas handle for the given texture handle.

Retrieves the texture section rectangle of the given texture handle in pixels.

Retrieves the texture section rectangle of the given texture handle in UV coordinates. These are within the range [0..1], as a fraction of the entire texture atlas’ size.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct TextureAtlasSources {
    pub texture_ids: HashMap<AssetId<Image>, usize>,
}
```

Example 2 (unknown):
```unknown
251fn create_sprite_from_atlas(
252    commands: &mut Commands,
253    translation: (f32, f32, f32),
254    atlas_texture: Handle<Image>,
255    atlas_sources: TextureAtlasSources,
256    atlas_handle: Handle<TextureAtlasLayout>,
257    vendor_handle: &Handle<Image>,
258) {
259    commands.spawn((
260        Transform {
261            translation: Vec3::new(translation.0, translation.1, translation.2),
262            scale: Vec3::splat(3.0),
263            ..default()
264        },
265        Sprite::from_atlas_image(
266            atlas_texture,
267            atlas_sources.handle(atlas_hand
...
```

---

## Enum CircularMeshUvMode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/enum.CircularMeshUvMode.html

**Contents:**
- Enum CircularMeshUvMode Copy item path
- Variants (Non-exhaustive)§
  - Mask
    - Fields
- Trait Implementations§
  - impl Clone for CircularMeshUvMode
    - fn clone(&self) -> CircularMeshUvMode
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for CircularMeshUvMode
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

Specifies how to generate UV-mappings for the CircularSector and CircularSegment shapes.

Currently the only variant is Mask, which is good for showing a portion of a texture that includes the entire circle, particularly the same texture will be displayed with different fractions of a complete circle.

It’s expected that more will be added in the future, such as a variant that causes the texture to be scaled to fit the bounding box of the shape, which would be good for packed textures only including the portion of the circle that is needed to display.

Treats the shape as a mask over a circle of equal size and radius, with the center of the circle at the center of the texture.

Angle by which to rotate the shape when generating the UV map.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[non_exhaustive]pub enum CircularMeshUvMode {
    Mask {
        angle: f32,
    },
}
```

---

## Struct SpecializedPrepassMaterialViewPipelineCache Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.SpecializedPrepassMaterialViewPipelineCache.html

**Contents:**
- Struct SpecializedPrepassMaterialViewPipelineCache Copy item path
- Methods from Deref<Target = HashMap<MainEntity, (Tick, CachedRenderPipelineId), EntityHash>>§
    - pub fn hasher(&self) -> &S
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn keys(&self) -> Keys<'_, K, V> ⓘ
      - §Examples
      - Examples found in repository?
    - pub fn values(&self) -> Values<'_, K, V> ⓘ
      - §Examples

Stores the cached render pipeline ID for each entity in a single view, as well as the last time it was changed.

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

Shrinks the capacity of the map with a lower limit. It will drop down no lower than 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct SpecializedPrepassMaterialViewPipelineCache { /* private fields */ }
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

## Type Alias UpdateGizmoMeshes Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/type.UpdateGizmoMeshes.html

**Contents:**
- Type Alias UpdateGizmoMeshes Copy item path
- Aliased Type§

Deprecated alias for GizmoMeshSystems.

**Examples:**

Example 1 (unknown):
```unknown
pub type UpdateGizmoMeshes = GizmoMeshSystems;
```

Example 2 (unknown):
```unknown
pub struct UpdateGizmoMeshes;
```

---

## Struct ErasedMaterialKey Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ErasedMaterialKey.html

**Contents:**
- Struct ErasedMaterialKey Copy item path
- Implementations§
  - impl ErasedMaterialKey
    - pub fn new<T>(material_key: T) -> ErasedMaterialKeywhere T: Clone + Hash + PartialEq + Send + Sync + 'static,
    - pub fn to_key<T>(&self) -> Twhere T: Clone + 'static,
- Trait Implementations§
  - impl Clone for ErasedMaterialKey
    - fn clone(&self) -> ErasedMaterialKey
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for ErasedMaterialKey

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ErasedMaterialKey { /* private fields */ }
```

---

## Struct MeshPhaseBindGroups Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshPhaseBindGroups.html

**Contents:**
- Struct MeshPhaseBindGroups Copy item path
- Implementations§
  - impl MeshPhaseBindGroups
    - pub fn reset(&mut self)
    - pub fn get( &self, asset_id: AssetId<Mesh>, lightmap: Option<LightmapSlabIndex>, is_skinned: bool, morph: bool, motion_vectors: bool, ) -> Option<&BindGroup>
- Trait Implementations§
  - impl Default for MeshPhaseBindGroups
    - fn default() -> MeshPhaseBindGroups
- Auto Trait Implementations§
  - impl Freeze for MeshPhaseBindGroups

The bind groups for meshes currently loaded.

If GPU mesh preprocessing isn’t in use, these are global to the scene. If GPU mesh preprocessing is in use, these are specific to a single phase.

Get the BindGroup for RenderMesh with given handle_id and lightmap key lightmap.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshPhaseBindGroups { /* private fields */ }
```

---

## Crate camera Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/index.html

**Contents:**
- Crate camera Copy item path
- Modules§
- Structs§
- Enums§
- Traits§
- Type Aliases§

bevyCrate camera Copy item pathSource Modules§preludeThe camera prelude.primitivesvisibilityStructs§CameraThe defining Component for camera entities, storing information about how and what to render through this camera.Camera2dA 2D camera component. Enables the 2D render graph for a Camera.Camera3dA 3D camera component. Enables the main 3D render graph for a Camera.Camera3dDepthTextureUsageCameraMainTextureUsagesThis component lets you control the TextureUsages field of the main texture generated for the cameraCameraPluginCameraProjectionPluginAdds Camera driver systems for a given projection type.CameraUpdateSystemsLabel for camera_system<T>, shared across all T.ClearColorA Resource that stores the default color that cameras use to clear the screen between frames.ComputedCameraValuesHolds internally computed Camera values.CustomProjectionHolds a dynamic CameraProjection trait object. Use Projection::custom() to construct a custom projection.ExposureHow much energy a Camera3d absorbs from incoming light.ImageRenderTargetA render target that renders to an Image.MainPassResolutionOverrideOverride the resolution a 3d camera’s main pass is rendered at.ManualTextureViewHandleA unique id that corresponds to a specific ManualTextureView in the ManualTextureViews collection.OrthographicProjectionProject a 3D space onto a 2D surface using parallel lines, i.e., unlike PerspectiveProjection, the size of objects remains the same regardless of their distance to the camera.PerspectiveProjectionA 3D camera projection in which distant objects appear smaller than close objects.PhysicalCameraParametersParameters based on physical camera characteristics for calculating EV100 values for use with Exposure. This is also used for depth of field.RenderTargetInfoInformation about the current RenderTarget.SubCameraViewSettings to define a camera sub view.ViewportRender viewport configuration for the Camera component.Enums§Camera3dDepthLoadOpThe depth clear operation to perform for the main 3d pass.CameraOutputModeControl how this Camera outputs once rendering is completed.ClearColorConfigFor a camera, specifies the color used to clear the viewport before rendering or when writing to the final render target texture.NormalizedRenderTargetNormalized version of the render target.ProjectionComponent that defines how to compute a Camera’s projection matrix.RenderTargetThe “target” that a Camera will render to. For example, this could be a Window swapchain or an Image.ScalingModeScaling m

*[Content truncated]*

---

## Module skinning Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/skinning/index.html

**Contents:**
- Module skinning Copy item path
- Structs§

bevy::meshModule skinning Copy item pathSource Structs§SkinnedMeshSkinnedMeshInverseBindposes

---

## Enum MeshWindingInvertError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/enum.MeshWindingInvertError.html

**Contents:**
- Enum MeshWindingInvertError Copy item path
- Variants§
  - WrongTopology
  - AbruptIndicesEnd
- Trait Implementations§
  - impl Debug for MeshWindingInvertError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for MeshWindingInvertError
    - fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>
  - impl Error for MeshWindingInvertError

An error that occurred while trying to invert the winding of a Mesh.

This error occurs when you try to invert the winding for a mesh with PrimitiveTopology::PointList.

This error occurs when you try to invert the winding for a mesh with

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum MeshWindingInvertError {
    WrongTopology,
    AbruptIndicesEnd,
}
```

---

## Enum TextureError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.TextureError.html

**Contents:**
- Enum TextureError Copy item path
- Variants§
  - InvalidImageMimeType(String)
  - InvalidImageExtension(String)
  - ImageError(ImageError)
  - UnsupportedTextureFormat(String)
  - SuperCompressionNotSupported(String)
  - SuperDecompressionError(String)
  - InvalidData(String)
  - TranscodeError(String)

An error that occurs when loading a texture.

Image MIME type is invalid.

Image extension is invalid.

Failed to load an image.

Texture format isn’t supported.

Supercompression isn’t supported.

Failed to decompress an image.

Format requires transcoding.

Only cubemaps with six faces are supported.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum TextureError {
    InvalidImageMimeType(String),
    InvalidImageExtension(String),
    ImageError(ImageError),
    UnsupportedTextureFormat(String),
    SuperCompressionNotSupported(String),
    SuperDecompressionError(String),
    InvalidData(String),
    TranscodeError(String),
    FormatRequiresTranscodingError(TranscodeFormat),
    IncompleteCubemap,
}
```

---

## Enum ConeAnchor Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/enum.ConeAnchor.html

**Contents:**
- Enum ConeAnchor Copy item path
- Variants§
  - MidPoint
  - Tip
  - Base
- Trait Implementations§
  - impl Clone for ConeAnchor
    - fn clone(&self) -> ConeAnchor
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for ConeAnchor

Anchoring options for ConeMeshBuilder

Midpoint between the tip of the cone and the center of its base.

The Tip of the triangle

The center of the base circle

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ConeAnchor {
    MidPoint,
    Tip,
    Base,
}
```

---

## Struct SpecializedShadowMaterialPipelineCache Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.SpecializedShadowMaterialPipelineCache.html

**Contents:**
- Struct SpecializedShadowMaterialPipelineCache Copy item path
- Methods from Deref<Target = HashMap<RetainedViewEntity, SpecializedShadowMaterialViewPipelineCache>>§
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
pub struct SpecializedShadowMaterialPipelineCache { /* private fields */ }
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

## Struct GpuMeshPreprocessPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.GpuMeshPreprocessPlugin.html

**Contents:**
- Struct GpuMeshPreprocessPlugin Copy item path
- Fields§
- Trait Implementations§
  - impl Plugin for GpuMeshPreprocessPlugin
    - fn build(&self, app: &mut App)
    - fn finish(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str
    - fn is_unique(&self) -> bool

A plugin that builds mesh uniforms on GPU.

This will only be added if the platform supports compute shaders (e.g. not on WebGL 2).

Whether we’re building MeshUniforms on GPU.

This requires compute shader support and so will be forcibly disabled if the platform doesn’t support those.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GpuMeshPreprocessPlugin {
    pub use_gpu_instance_buffer_builder: bool,
}
```

---

## Struct BindGroupLayout Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/render_resource/struct.BindGroupLayout.html

**Contents:**
- Struct BindGroupLayout Copy item path
- Implementations§
  - impl BindGroupLayout
    - pub fn id(&self) -> BindGroupLayoutId
    - pub fn value(&self) -> &BindGroupLayout
- Trait Implementations§
  - impl Clone for BindGroupLayout
    - fn clone(&self) -> BindGroupLayout
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for BindGroupLayout

Bind group layouts define the interface of resources (e.g. buffers, textures, samplers) for a shader. The actual resource binding is done via a BindGroup.

This is a lightweight thread-safe wrapper around wgpu’s own BindGroupLayout, which can be cloned as needed to workaround lifetime management issues. It may be converted from and dereferences to wgpu’s BindGroupLayout.

Can be created via RenderDevice::create_bind_group_layout.

Returns the BindGroupLayoutId representing the unique ID of the bind group layout.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct BindGroupLayout { /* private fields */ }
```

---

## Struct MainPassResolutionOverride Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.MainPassResolutionOverride.html

**Contents:**
- Struct MainPassResolutionOverride Copy item path
  - §Usage
- Tuple Fields§
- Methods from Deref<Target = UVec2>§
    - pub const ZERO: UVec2
    - pub const ONE: UVec2
    - pub const MIN: UVec2
    - pub const MAX: UVec2
    - pub const X: UVec2
    - pub const Y: UVec2

Override the resolution a 3d camera’s main pass is rendered at.

Does not affect post processing.

Converts self to [x, y]

Casts all elements of self to f32.

Casts all elements of self to f64.

Casts all elements of self to i8.

Casts all elements of self to u8.

Casts all elements of self to i16.

Casts all elements of self to u16.

Casts all elements of self to i32.

Casts all elements of self to i64.

Casts all elements of self to u64.

Casts all elements of self to usize.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MainPassResolutionOverride(pub UVec2);
```

Example 2 (javascript):
```javascript
54fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
55    let mut image = Image::new_target_texture(SIZE.x, SIZE.y, TextureFormat::Rgba32Float);
56    image.asset_usage = RenderAssetUsages::RENDER_WORLD;
57    image.texture_descriptor.usage =
58        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
59    let image0 = images.add(image.clone());
60    let image1 = images.add(image);
61
62    commands.spawn((
63        Sprite {
64            image: image0.clone(),
65            custom_size: Some(SIZE.as_vec2()),
66            ..defau
...
```

Example 3 (javascript):
```javascript
121fn setup(
122    mut commands: Commands,
123    mut meshes: ResMut<Assets<Mesh>>,
124    mut materials: ResMut<Assets<ColorMaterial>>,
125    window: Single<&Window>,
126) {
127    let window_size = window.resolution.physical_size().as_vec2();
128
129    // Initialize centered, non-window-filling viewport
130    commands.spawn((
131        Camera2d,
132        Camera {
133            viewport: Some(Viewport {
134                physical_position: (window_size * 0.125).as_uvec2(),
135                physical_size: (window_size * 0.75).as_uvec2(),
136                ..default()
137           
...
```

Example 4 (javascript):
```javascript
174fn drive_diegetic_pointer(
175    mut cursor_last: Local<Vec2>,
176    mut raycast: MeshRayCast,
177    rays: Res<RayMap>,
178    cubes: Query<&Mesh3d, With<Cube>>,
179    ui_camera: Query<&Camera, With<Camera2d>>,
180    primary_window: Query<Entity, With<PrimaryWindow>>,
181    windows: Query<(Entity, &Window)>,
182    images: Res<Assets<Image>>,
183    manual_texture_views: Res<ManualTextureViews>,
184    mut window_events: MessageReader<WindowEvent>,
185    mut pointer_inputs: MessageWriter<PointerInput>,
186) -> Result {
187    // Get the size of the texture, so we can convert from dim
...
```

---

## Struct MaterialExtensionKey Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialExtensionKey.html

**Contents:**
- Struct MaterialExtensionKey Copy item path
- Fields§
- Auto Trait Implementations§
  - impl<E> Freeze for MaterialExtensionKey<E>where <E as AsBindGroup>::Data: Freeze,
  - impl<E> RefUnwindSafe for MaterialExtensionKey<E>where <E as AsBindGroup>::Data: RefUnwindSafe,
  - impl<E> Send for MaterialExtensionKey<E>
  - impl<E> Sync for MaterialExtensionKey<E>
  - impl<E> Unpin for MaterialExtensionKey<E>where <E as AsBindGroup>::Data: Unpin,
  - impl<E> UnwindSafe for MaterialExtensionKey<E>where <E as AsBindGroup>::Data: UnwindSafe,
- Blanket Implementations§

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialExtensionKey<E>where
    E: MaterialExtension,{
    pub mesh_key: MeshPipelineKey,
    pub bind_group_data: <E as AsBindGroup>::Data,
}
```

---

## Struct Mesh Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.Mesh.html

**Contents:**
- Struct Mesh Copy item path
  - §Manual creation
  - §Other examples
  - §Common points of confusion
  - §Use with StandardMaterial
  - §Remote Inspection
- Fields§
- Implementations§
  - impl Mesh
    - pub const ATTRIBUTE_POSITION: MeshVertexAttribute

A 3D object made out of vertices representing triangles, lines, or points, with “attribute” values for each vertex.

Meshes can be automatically generated by a bevy AssetLoader (generally by loading a Gltf file), or by converting a primitive using into. It is also possible to create one manually. They can be edited after creation.

Meshes can be rendered with a Mesh2d and MeshMaterial2d or Mesh3d and MeshMaterial3d for 2D and 3D respectively.

A Mesh in Bevy is equivalent to a “primitive” in the glTF format, for a glTF Mesh representation, see GltfMesh.

The following function will construct a flat mesh, to be rendered with a StandardMaterial or ColorMaterial:

You can see how it looks like here, used in a Mesh3d with a square bevy logo texture, with added axis, points, lines and text for clarity.

For further visualization, explanation, and examples, see the built-in Bevy examples, and the implementation of the built-in shapes. In particular, generate_custom_mesh teaches you to access and modify the attributes of a Mesh after creating it.

To render correctly with StandardMaterial, a mesh needs to have properly defined:

To transmit a Mesh between two running Bevy apps, e.g. through BRP, use SerializedMesh. This type is only meant for short-term transmission between same versions and should not be stored anywhere.

Whether or not to build a BLAS for use with bevy_solari raytracing.

Note that this is not whether the mesh is compatible with bevy_solari raytracing. This field just controls whether or not a BLAS gets built for this mesh, assuming that the mesh is compatible.

The use case for this field is using lower-resolution proxy meshes for raytracing (to save on BLAS memory usage), while using higher-resolution meshes for raster. You can set this field to true for the lower-resolution proxy mesh, and to false for the high-resolution raster mesh.

Alternatively, you can use the same mesh for both raster and raytracing, with this field set to true.

Does nothing if not used with bevy_solari, or if the mesh is not compatible with bevy_solari (see bevy_solari’s docs).

Where the vertex is located in space. Use in conjunction with Mesh::insert_attribute or Mesh::with_inserted_attribute.

The format of this attribute is VertexFormat::Float32x3.

The direction the vertex normal is facing in. Use in conjunction with Mesh::insert_attribute or Mesh::with_inserted_attribute.

The format of this attribute is VertexFormat::Float32x3.

Texture coordinates for the ve

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Mesh {
    pub asset_usage: RenderAssetUsages,
    pub enable_raytracing: bool,
    /* private fields */
}
```

Example 2 (unknown):
```unknown
fn create_simple_parallelogram() -> Mesh {
    // Create a new mesh using a triangle list topology, where each set of 3 vertices composes a triangle.
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        // Add 4 vertices, each with its own position attribute (coordinate in
        // 3D space), for each of the corners of the parallelogram.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![[0.0, 0.0, 0.0], [1.0, 2.0, 0.0], [2.0, 2.0, 0.0], [1.0, 0.0, 0.0]]
        )
        // Assign a UV coordinate to each vertex.
        .
...
```

Example 3 (javascript):
```javascript
96    fn from(line: LineList) -> Self {
97        let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();
98
99        Mesh::new(
100            // This tells wgpu that the positions are list of lines
101            // where every pair is a start and end point
102            PrimitiveTopology::LineList,
103            RenderAssetUsages::RENDER_WORLD,
104        )
105        // Add the vertices positions as an attribute
106        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
107    }
108}
109
110/// A list of points that will have a line drawn between 
...
```

Example 4 (javascript):
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

---

## Struct MaterialsPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialsPlugin.html

**Contents:**
- Struct MaterialsPlugin Copy item path
- Fields§
- Trait Implementations§
  - impl Default for MaterialsPlugin
    - fn default() -> MaterialsPlugin
  - impl Plugin for MaterialsPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)

Debugging flags that can optionally be set when constructing the renderer.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialsPlugin {
    pub debug_flags: RenderDebugFlags,
}
```

---

## Struct RenderVisibleMeshEntities Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.RenderVisibleMeshEntities.html

**Contents:**
- Struct RenderVisibleMeshEntities Copy item path
- Fields§
- Methods from Deref<Target = Vec<(Entity, MainEntity)>>§
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
pub struct RenderVisibleMeshEntities {
    pub entities: Vec<(Entity, MainEntity)>,
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

## Struct MaterialSlab Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialSlab.html

**Contents:**
- Struct MaterialSlab Copy item path
- Implementations§
  - impl<'a> MaterialSlab<'a>
    - pub fn bind_group(&self) -> Option<&'a BindGroup>
- Auto Trait Implementations§
  - impl<'a> Freeze for MaterialSlab<'a>
  - impl<'a> !RefUnwindSafe for MaterialSlab<'a>
  - impl<'a> Send for MaterialSlab<'a>
  - impl<'a> Sync for MaterialSlab<'a>
  - impl<'a> Unpin for MaterialSlab<'a>

The public interface to a slab, which represents a single bind group.

Returns the BindGroup corresponding to this slab, if it’s been prepared.

You can prepare bind groups by calling MaterialBindGroupAllocator::prepare_bind_groups. If the bind group isn’t ready, this method returns None.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialSlab<'a>(/* private fields */);
```

---

## Struct SphereMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.SphereMeshBuilder.html

**Contents:**
- Struct SphereMeshBuilder Copy item path
- Fields§
- Implementations§
  - impl SphereMeshBuilder
    - pub const fn new(radius: f32, kind: SphereKind) -> SphereMeshBuilder
      - Examples found in repository?
    - pub const fn kind(self, kind: SphereKind) -> SphereMeshBuilder
      - Examples found in repository?
    - pub fn ico(&self, subdivisions: u32) -> Result<Mesh, IcosphereError>
      - Examples found in repository?

A builder used for creating a Mesh with an Sphere shape.

The type of sphere mesh that will be built.

Creates a new SphereMeshBuilder from a radius and SphereKind.

Sets the SphereKind that will be used for building the mesh.

Creates an icosphere mesh with the given number of subdivisions.

The number of faces quadruples with each subdivision. If there are 80 or more subdivisions, the vertex count will be too large, and an IcosphereError is returned.

A good default is 5 subdivisions.

Creates a UV sphere Mesh with the given number of longitudinal sectors and latitudinal stacks, aka horizontal and vertical resolution.

A good default is 32 sectors and 18 stacks.

Builds a Mesh according to the configuration in self.

Panics if the sphere is a SphereKind::Ico with a subdivision count that is greater than or equal to 80 because there will be too many vertices.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct SphereMeshBuilder {
    pub sphere: Sphere,
    pub kind: SphereKind,
}
```

Example 2 (unknown):
```unknown
109fn setup(
110    mut commands: Commands,
111    asset_server: Res<AssetServer>,
112    mut meshes: ResMut<Assets<Mesh>>,
113    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, ExampleBindlessExtension>>>,
114) {
115    // Create a gray sphere, modulated with a red-tinted Bevy logo.
116    commands.spawn((
117        Mesh3d(meshes.add(SphereMeshBuilder::new(
118            1.0,
119            SphereKind::Uv {
120                sectors: 20,
121                stacks: 20,
122            },
123        ))),
124        MeshMaterial3d(materials.add(ExtendedMaterial {
125          
...
```

Example 3 (javascript):
```javascript
51fn setup(
52    mut commands: Commands,
53    mut meshes: ResMut<Assets<Mesh>>,
54    mut materials: ResMut<Assets<StandardMaterial>>,
55) {
56    // Use seeded rng and store it in a resource; this makes the random output reproducible.
57    let seeded_rng = ChaCha8Rng::seed_from_u64(19878367467712);
58    commands.insert_resource(RandomSource(seeded_rng));
59
60    // Make a plane for establishing space.
61    commands.spawn((
62        Mesh3d(meshes.add(Plane3d::default().mesh().size(12.0, 12.0))),
63        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
64        Transform::fr
...
```

Example 4 (javascript):
```javascript
130fn spawn_sphere(
131    commands: &mut Commands,
132    meshes: &mut Assets<Mesh>,
133    materials: &mut Assets<StandardMaterial>,
134    app_status: &AppStatus,
135) {
136    // Create a sphere mesh.
137    let sphere_mesh = meshes.add(Sphere::new(1.0).mesh().ico(7).unwrap());
138
139    // Create a sphere.
140    commands.spawn((
141        Mesh3d(sphere_mesh.clone()),
142        MeshMaterial3d(materials.add(StandardMaterial {
143            base_color: Srgba::hex("#ffffff").unwrap().into(),
144            metallic: 1.0,
145            perceptual_roughness: app_status.sphere_roughness,
1
...
```

---

## Enum MeshMergeError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/enum.MeshMergeError.html

**Contents:**
- Enum MeshMergeError Copy item path
- Variants§
  - IncompatibleVertexAttributes
    - Fields
  - IncompatiblePrimitiveTopology
    - Fields
- Trait Implementations§
  - impl Clone for MeshMergeError
    - fn clone(&self) -> MeshMergeError
    - fn clone_from(&mut self, source: &Self)

Error that can occur when calling Mesh::merge.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum MeshMergeError {
    IncompatibleVertexAttributes {
        self_attribute: MeshVertexAttribute,
        other_attribute: Option<MeshVertexAttribute>,
    },
    IncompatiblePrimitiveTopology {
        self_primitive_topology: PrimitiveTopology,
        other_primitive_topology: PrimitiveTopology,
    },
}
```

---

## Trait Extrudable Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/trait.Extrudable.html

**Contents:**
- Trait Extrudable Copy item path
  - §Warning
- Required Methods§
    - fn perimeter(&self) -> Vec<PerimeterSegment>
- Implementors§
  - impl Extrudable for AnnulusMeshBuilder
  - impl Extrudable for Capsule2dMeshBuilder
  - impl Extrudable for CircleMeshBuilder
  - impl Extrudable for CircularSectorMeshBuilder
  - impl Extrudable for CircularSegmentMeshBuilder

A trait required for implementing Meshable for Extrusion<T>.

By implementing this trait you guarantee that the primitive_topology of the mesh returned by this builder is PrimitiveTopology::TriangleList and that your mesh has a Mesh::ATTRIBUTE_POSITION attribute.

A list of the indices each representing a part of the perimeter of the mesh.

**Examples:**

Example 1 (unknown):
```unknown
pub trait Extrudable: MeshBuilder {
    // Required method
    fn perimeter(&self) -> Vec<PerimeterSegment>;
}
```

---

## Struct Camera2d Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.Camera2d.html

**Contents:**
- Struct Camera2d Copy item path
- Trait Implementations§
  - impl Clone for Camera2d
    - fn clone(&self) -> Camera2d
    - fn clone_from(&mut self, source: &Self)
  - impl Component for Camera2dwhere Camera2d: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior

A 2D camera component. Enables the 2D render graph for a Camera.

Required Components: Camera, Projection, Frustum.

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components of the required components, recursively, in depth-first order.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Camera2d;
```

---

## Function triangle_area_normal Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/fn.triangle_area_normal.html

**Contents:**
- Function triangle_area_normal Copy item path

Compute a vector whose direction is the normal of the triangle formed by points a, b, c, and whose magnitude is double the area of the triangle. This is useful for computing smooth normals where the contributing normals are proportionate to the areas of the triangles as discussed here.

Question: Why double the area? Because the area of a triangle A is determined by this equation:

A = |(b - a) x (c - a)| / 2

By computing 2 A we avoid a division operation, and when calculating the the sum of these vectors which are then normalized, a constant multiple has no effect.

**Examples:**

Example 1 (unknown):
```unknown
pub fn triangle_area_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3]
```

---

## Module alpha Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/alpha/index.html

**Contents:**
- Module alpha Copy item path
- Enums§

bevy::renderModule alpha Copy item pathSource Enums§AlphaModeSets how a material’s base color alpha channel is used for transparency.

---

## Enum GenerateTangentsError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/enum.GenerateTangentsError.html

**Contents:**
- Enum GenerateTangentsError Copy item path
- Variants§
  - UnsupportedTopology(PrimitiveTopology)
  - MissingIndices
  - MissingVertexAttribute(&'static str)
  - InvalidVertexAttributeFormat(&'static str, VertexFormat)
  - MikktspaceError(GenerateTangentSpaceError)
- Trait Implementations§
  - impl Debug for GenerateTangentsError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

Failed to generate tangents for the mesh.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum GenerateTangentsError {
    UnsupportedTopology(PrimitiveTopology),
    MissingIndices,
    MissingVertexAttribute(&'static str),
    InvalidVertexAttributeFormat(&'static str, VertexFormat),
    MikktspaceError(GenerateTangentSpaceError),
}
```

---

## Enum Projection Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/enum.Projection.html

**Contents:**
- Enum Projection Copy item path
  - §What’s a projection?
- Variants§
  - Perspective(PerspectiveProjection)
  - Orthographic(OrthographicProjection)
  - Custom(CustomProjection)
- Implementations§
  - impl Projection
    - pub fn custom<P>(projection: P) -> Projectionwhere P: CameraProjection + Debug + Send + Sync + Clone + 'static,
      - Examples found in repository?

Component that defines how to compute a Camera’s projection matrix.

Common projections, like perspective and orthographic, are provided out of the box to handle the majority of use cases. Custom projections can be added using the CameraProjection trait and the Projection::custom constructor.

A camera projection essentially describes how 3d points from the point of view of a camera are projected onto a 2d screen. This is where properties like a camera’s field of view are defined. More specifically, a projection is a 4x4 matrix that transforms points from view space (the point of view of the camera) into clip space. Clip space is almost, but not quite, equivalent to the rectangle that is rendered to your screen, with a depth axis. Any points that land outside the bounds of this cuboid are “clipped” and not rendered.

You can also think of the projection as the thing that describes the shape of a camera’s frustum: the volume in 3d space that is visible to a camera.

Construct a new custom camera projection from a type that implements CameraProjection.

Check if the projection is perspective. For CustomProjection, this checks if the projection matrix’s w-axis’s w is 0.0.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum Projection {
    Perspective(PerspectiveProjection),
    Orthographic(OrthographicProjection),
    Custom(CustomProjection),
}
```

Example 2 (unknown):
```unknown
50fn setup(
51    mut commands: Commands,
52    mut meshes: ResMut<Assets<Mesh>>,
53    mut materials: ResMut<Assets<StandardMaterial>>,
54) {
55    commands.spawn((
56        Camera3d::default(),
57        // Use our custom projection:
58        Projection::custom(ObliquePerspectiveProjection {
59            horizontal_obliqueness: 0.2,
60            vertical_obliqueness: 0.6,
61            perspective: PerspectiveProjection::default(),
62        }),
63        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
64    ));
65
66    // Scene setup
67    commands.spawn((
68      
...
```

---

## Struct MaterialBindGroupBindlessAllocator Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialBindGroupBindlessAllocator.html

**Contents:**
- Struct MaterialBindGroupBindlessAllocator Copy item path
- Auto Trait Implementations§
  - impl Freeze for MaterialBindGroupBindlessAllocator
  - impl !RefUnwindSafe for MaterialBindGroupBindlessAllocator
  - impl Send for MaterialBindGroupBindlessAllocator
  - impl Sync for MaterialBindGroupBindlessAllocator
  - impl Unpin for MaterialBindGroupBindlessAllocator
  - impl !UnwindSafe for MaterialBindGroupBindlessAllocator
- Blanket Implementations§
  - impl<T> Any for Twhere T: 'static + ?Sized,

The allocator that places bindless materials into bind groups and tracks their resources.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialBindGroupBindlessAllocator { /* private fields */ }
```

---

## Struct MeshesToReextractNextFrame Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshesToReextractNextFrame.html

**Contents:**
- Struct MeshesToReextractNextFrame Copy item path
- Methods from Deref<Target = HashSet<MainEntity, EntityHash>>§
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn iter(&self) -> Iter<'_, T> ⓘ
      - §Examples
      - Examples found in repository?
    - pub fn len(&self) -> usize
      - §Examples
    - pub fn is_empty(&self) -> bool

Holds a list of meshes that couldn’t be extracted this frame because their materials weren’t prepared yet.

On subsequent frames, we try to reextract those meshes.

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

Returns a reference to the set’s BuildHasher.

Refer to hasher for further details.

Reserves capacity for at least additional more elements to be inserted in the HashSet. The collection may reserve more space to avoid frequent reallocations.

Refer to reserve for further details.

Tries to reserve capacity for at least additional more elements to be inserted in the given HashSet<K,V>. The collection may reserve more space to avoid frequent reallocations.

Refer to try_reserve for further details.

Shrinks the capacity of the set as much as possible. It will drop down as much as possible while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.

Refer to shrink_to_fit for further details.

Shrinks the capacity of the set with a lower limit. It will drop down no lower than the supplied limit while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.

Refer to shrink_to for further details.

Visits the values representing the difference, i.e., the values that are in self but not in other.

Refer to difference for further details.

Visits the values representing the symmetric difference, i.e., the values that are in self or in other but not in both.

Refer to symmetric_difference for further details.

Visits the values representing the intersection, i.e., the values that are both in self and other.

Refer to intersection for further details.

Visits the values representing the union, i.e., all the values

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshesToReextractNextFrame(/* private fields */);
```

Example 2 (javascript):
```javascript
let map = HashSet::with_capacity(5);

assert!(map.capacity() >= 5);
```

Example 3 (javascript):
```javascript
let mut map = HashSet::new();

map.insert("foo");
map.insert("bar");
map.insert("baz");

for value in map.iter() {
    // "foo", "bar", "baz"
    // Note that the above order is not guaranteed
}
```

Example 4 (javascript):
```javascript
198    fn get_nearby(&self, pos: Vec2) -> Vec<Entity> {
199        let tile = (
200            (pos.x / CELL_SIZE).floor() as i32,
201            (pos.y / CELL_SIZE).floor() as i32,
202        );
203        let mut nearby = Vec::new();
204        for x in -1..2 {
205            for y in -1..2 {
206                if let Some(mines) = self.map.get(&(tile.0 + x, tile.1 + y)) {
207                    nearby.extend(mines.iter());
208                }
209            }
210        }
211        nearby
212    }
```

---

## Struct Capsule3dMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.Capsule3dMeshBuilder.html

**Contents:**
- Struct Capsule3dMeshBuilder Copy item path
- Fields§
- Implementations§
  - impl Capsule3dMeshBuilder
    - pub fn new( radius: f32, height: f32, longitudes: u32, latitudes: u32, ) -> Capsule3dMeshBuilder
    - pub const fn rings(self, rings: u32) -> Capsule3dMeshBuilder
    - pub const fn longitudes(self, longitudes: u32) -> Capsule3dMeshBuilder
    - pub const fn latitudes(self, latitudes: u32) -> Capsule3dMeshBuilder
    - pub const fn uv_profile( self, uv_profile: CapsuleUvProfile, ) -> Capsule3dMeshBuilder
- Trait Implementations§

A builder used for creating a Mesh with a Capsule3d shape.

The number of horizontal lines subdividing the cylindrical part of the capsule. The default is 0.

The number of vertical lines subdividing the hemispheres of the capsule. The default is 32.

The number of horizontal lines subdividing the hemispheres of the capsule. The default is 16.

The manner in which UV coordinates are distributed vertically. The default is CapsuleUvProfile::Aspect.

Creates a new Capsule3dMeshBuilder from a given radius, height, longitudes, and latitudes.

Note that height is the distance between the centers of the hemispheres. radius will be added to both ends to get the real height of the mesh.

Sets the number of horizontal lines subdividing the cylindrical part of the capsule.

Sets the number of vertical lines subdividing the hemispheres of the capsule.

Sets the number of horizontal lines subdividing the hemispheres of the capsule.

Sets the manner in which UV coordinates are distributed vertically.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Capsule3dMeshBuilder {
    pub capsule: Capsule3d,
    pub rings: u32,
    pub longitudes: u32,
    pub latitudes: u32,
    pub uv_profile: CapsuleUvProfile,
}
```

---

## Struct OrderIndependentTransparencyPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/core_pipeline/oit/struct.OrderIndependentTransparencyPlugin.html

**Contents:**
- Struct OrderIndependentTransparencyPlugin Copy item path
- §Implementation details
- Trait Implementations§
  - impl Plugin for OrderIndependentTransparencyPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str
    - fn is_unique(&self) -> bool

A plugin that adds support for Order Independent Transparency (OIT). This can correctly render some scenes that would otherwise have artifacts due to alpha blending, but uses more memory.

To enable OIT for a camera you need to add the OrderIndependentTransparencySettings component to it.

If you want to use OIT for your custom material you need to call oit_draw(position, color) in your fragment shader. You also need to make sure that your fragment shader doesn’t output any colors.

This implementation uses 2 passes.

The first pass writes the depth and color of all the fragments to a big buffer. The buffer contains N layers for each pixel, where N can be set with OrderIndependentTransparencySettings::layer_count. This pass is essentially a forward pass.

The second pass is a single fullscreen triangle pass that sorts all the fragments then blends them together and outputs the result to the screen.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct OrderIndependentTransparencyPlugin;
```

---

## Module prepass Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/core_pipeline/prepass/index.html

**Contents:**
- Module prepass Copy item path
- Modules§
- Structs§
- Constants§
- Functions§

Run a prepass before the main pass to generate depth, normals, and/or motion vectors textures, sometimes called a thin g-buffer. These textures are useful for various screen-space effects and reducing overdraw in the main pass.

The prepass only runs for opaque meshes or meshes with an alpha mask. Transparent meshes are ignored.

To enable the prepass, you need to add a prepass component to a bevy_camera::Camera3d.

DepthPrepass NormalPrepass MotionVectorPrepass

The textures are automatically added to the default mesh view bindings. You can also get the raw textures by querying the ViewPrepassTextures component on any camera with a prepass component.

The depth prepass will always run and generate the depth buffer as a side effect, but it won’t copy it to a separate texture unless the DepthPrepass is activated. This means that if any prepass component is present it will always create a depth buffer that will be used by the main pass.

When using the default mesh view bindings you should be able to use prepass_depth(), prepass_normal(), and prepass_motion_vector() to load the related textures. These functions are defined in bevy_pbr::prepass_utils. See the shader_prepass example that shows how to use them.

The prepass runs for each Material. You can control if the prepass should run per-material by setting the prepass_enabled flag on the MaterialPlugin.

Currently only works for 3D.

---

## Struct RenderMaterialInstance Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.RenderMaterialInstance.html

**Contents:**
- Struct RenderMaterialInstance Copy item path
- Fields§
- Auto Trait Implementations§
  - impl Freeze for RenderMaterialInstance
  - impl RefUnwindSafe for RenderMaterialInstance
  - impl Send for RenderMaterialInstance
  - impl Sync for RenderMaterialInstance
  - impl Unpin for RenderMaterialInstance
  - impl UnwindSafe for RenderMaterialInstance
- Blanket Implementations§

The material associated with a single mesh instance in the main world.

Note that this uses an UntypedAssetId and isn’t generic over the material type, for simplicity.

The RenderMaterialInstances::current_change_tick at which this material instance was last modified.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderMaterialInstance {
    pub asset_id: UntypedAssetId,
    pub last_change_tick: Tick,
}
```

---

## Enum ScreenSpaceTransmissionQuality Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/enum.ScreenSpaceTransmissionQuality.html

**Contents:**
- Enum ScreenSpaceTransmissionQuality Copy item path
- Variants§
  - Low
  - Medium
  - High
  - Ultra
- Trait Implementations§
  - impl Clone for ScreenSpaceTransmissionQuality
    - fn clone(&self) -> ScreenSpaceTransmissionQuality
    - fn clone_from(&mut self, source: &Self)

The quality of the screen space transmission blur effect, applied to whatever’s “behind” transmissive objects when their roughness is greater than 0.0.

Higher qualities are more GPU-intensive.

Note: You can get better-looking results at any quality level by enabling TAA. See: TemporalAntiAliasPlugin

Best performance at the cost of quality. Suitable for lower end GPUs. (e.g. Mobile)

A balanced option between quality and performance.

Better quality. Suitable for high end GPUs. (e.g. Desktop)

The highest quality, suitable for non-realtime rendering. (e.g. Pre-rendered cinematics and photo mode)

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ScreenSpaceTransmissionQuality {
    Low,
    Medium,
    High,
    Ultra,
}
```

---

## Struct Triangle3dMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.Triangle3dMeshBuilder.html

**Contents:**
- Struct Triangle3dMeshBuilder Copy item path
- Trait Implementations§
  - impl Clone for Triangle3dMeshBuilder
    - fn clone(&self) -> Triangle3dMeshBuilder
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for Triangle3dMeshBuilder
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for Triangle3dMeshBuilder
    - fn default() -> Triangle3dMeshBuilder
  - impl FromArg for Triangle3dMeshBuilder

A builder used for creating a Mesh with a Triangle3d shape.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Triangle3dMeshBuilder { /* private fields */ }
```

---

## Struct GltfMesh Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/struct.GltfMesh.html

**Contents:**
- Struct GltfMesh Copy item path
- Fields§
- Implementations§
  - impl GltfMesh
    - pub fn new( mesh: &Mesh<'_>, primitives: Vec<GltfPrimitive>, extras: Option<GltfExtras>, ) -> GltfMesh
    - pub fn asset_label(&self) -> GltfAssetLabel
- Trait Implementations§
  - impl Clone for GltfMesh
    - fn clone(&self) -> GltfMesh
    - fn clone_from(&mut self, source: &Self)

A glTF mesh, which may consist of multiple GltfPrimitives and an optional GltfExtras.

See the relevant glTF specification section.

Index of the mesh inside the scene

Computed name for a mesh - either a user defined mesh name from gLTF or a generated name from index

Primitives of the glTF mesh.

Create a mesh extracting name and index from glTF def

Subasset label for this mesh within the gLTF parent asset.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GltfMesh {
    pub index: usize,
    pub name: String,
    pub primitives: Vec<GltfPrimitive>,
    pub extras: Option<GltfExtras>,
}
```

---

## Function ktx2_format_to_texture_format Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/fn.ktx2_format_to_texture_format.html

**Contents:**
- Function ktx2_format_to_texture_format Copy item path

bevy::imageFunction ktx2_format_to_texture_format Copy item pathSource pub fn ktx2_format_to_texture_format( ktx2_format: Format, is_srgb: bool, ) -> Result<TextureFormat, TextureError>

**Examples:**

Example 1 (unknown):
```unknown
pub fn ktx2_format_to_texture_format(
    ktx2_format: Format,
    is_srgb: bool,
) -> Result<TextureFormat, TextureError>
```

---

## Enum GltfError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/enum.GltfError.html

**Contents:**
- Enum GltfError Copy item path
- Variants§
  - UnsupportedPrimitive
    - Fields
  - Gltf(Error)
  - MissingBlob
  - Base64Decode(DecodeError)
  - BufferFormatUnsupported
  - InvalidImageMimeType(String)
  - ImageError(TextureError)

An error that occurs when loading a glTF file.

Unsupported primitive mode.

Binary blob is missing.

Decoding the base64 mesh data failed.

Unsupported buffer format.

Invalid image mime type.

Error when loading a texture. Might be due to a disabled image file format feature.

Failed to read bytes from an asset path.

Failed to load asset from an asset path.

Missing sampler for an animation.

Failed to generate tangents.

Failed to generate morph targets.

Circular children in Nodes

Failed to load a file.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum GltfError {
Show 14 variants    UnsupportedPrimitive {
        mode: Mode,
    },
    Gltf(Error),
    MissingBlob,
    Base64Decode(DecodeError),
    BufferFormatUnsupported,
    InvalidImageMimeType(String),
    ImageError(TextureError),
    ReadAssetBytesError(ReadAssetBytesError),
    AssetLoadError(AssetLoadError),
    MissingAnimationSampler(usize),
    GenerateTangentsError(GenerateTangentsError),
    MorphTarget(MorphBuildError),
    CircularChildren(String),
    Io(Error),
}
```

---

## Struct PrepassVertexShader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PrepassVertexShader.html

**Contents:**
- Struct PrepassVertexShader Copy item path
- Trait Implementations§
  - impl Clone for PrepassVertexShader
    - fn clone(&self) -> PrepassVertexShader
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for PrepassVertexShader
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for PrepassVertexShader
    - fn default() -> PrepassVertexShader
  - impl Hash for PrepassVertexShader

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PrepassVertexShader;
```

---

## Struct ExrTextureLoaderSettings Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.ExrTextureLoaderSettings.html

**Contents:**
- Struct ExrTextureLoaderSettings Copy item path
- Fields§
- Trait Implementations§
  - impl Debug for ExrTextureLoaderSettings
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for ExrTextureLoaderSettings
    - fn default() -> ExrTextureLoaderSettings
  - impl<'de> Deserialize<'de> for ExrTextureLoaderSettings
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<ExrTextureLoaderSettings, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,
  - impl Serialize for ExrTextureLoaderSettings

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ExrTextureLoaderSettings {
    pub asset_usage: RenderAssetUsages,
}
```

---

## Enum Indices Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/enum.Indices.html

**Contents:**
- Enum Indices Copy item path
- Variants§
  - U16(Vec<u16>)
  - U32(Vec<u32>)
- Implementations§
  - impl Indices
    - pub fn iter(&self) -> impl Iterator<Item = usize>
    - pub fn len(&self) -> usize
    - pub fn is_empty(&self) -> bool
    - pub fn push(&mut self, index: u32)

An array of indices into the VertexAttributeValues for a mesh.

It describes the order in which the vertex attributes should be joined into faces.

Returns an iterator over the indices.

Returns the number of indices.

Returns true if there are no indices.

Add an index. If the index is greater than u16::MAX, the storage will be converted to u32.

Extend the indices with indices from an iterator. Semantically equivalent to calling push for each element in the iterator, but more efficient.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum Indices {
    U16(Vec<u16>),
    U32(Vec<u32>),
}
```

---

## Struct Exposure Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.Exposure.html

**Contents:**
- Struct Exposure Copy item path
- Fields§
- Implementations§
  - impl Exposure
    - pub const SUNLIGHT: Exposure
    - pub const OVERCAST: Exposure
    - pub const INDOOR: Exposure
    - pub const BLENDER: Exposure
    - pub const EV100_SUNLIGHT: f32 = 15f32
    - pub const EV100_OVERCAST: f32 = 12f32

How much energy a Camera3d absorbs from incoming light.

https://en.wikipedia.org/wiki/Exposure_(photography)

https://en.wikipedia.org/wiki/Exposure_value#Tabulated_exposure_values

This value was calibrated to match Blender’s implicit/default exposure as closely as possible. It also happens to be a reasonable default.

See https://github.com/bevyengine/bevy/issues/11577 for details.

This value was calibrated to match Blender’s implicit/default exposure as closely as possible. It also happens to be a reasonable default.

See https://github.com/bevyengine/bevy/issues/11577 for details.

Converts EV100 values to exposure values. https://google.github.io/filament/Filament.md.html#imagingpipeline/physicallybasedcamera/exposure

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Exposure {
    pub ev100: f32,
}
```

Example 2 (javascript):
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

## Struct PreprocessBindGroups Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PreprocessBindGroups.html

**Contents:**
- Struct PreprocessBindGroups Copy item path
- Tuple Fields§
- Methods from Deref<Target = HashMap<TypeId, PhasePreprocessBindGroups, NoOpHash>>§
    - pub fn hasher(&self) -> &S
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn keys(&self) -> Keys<'_, K, V> ⓘ
      - §Examples
      - Examples found in repository?
    - pub fn values(&self) -> Values<'_, K, V> ⓘ

The compute shader bind group for the mesh preprocessing pass for each render phase.

This goes on the view. It maps the core::any::TypeId of a render phase (e.g. bevy_core_pipeline::core_3d::Opaque3d) to the PhasePreprocessBindGroups for that phase.

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

Shrinks the capacity of the map as much as possible. It will drop down as much as possible while maintaining the internal rules and possibly leaving some space in accordance with the resize

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct PreprocessBindGroups(pub HashMap<TypeId, PhasePreprocessBindGroups, NoOpHash>);
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

## Module renderer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/renderer/index.html

**Contents:**
- Module renderer Copy item path
- Modules§
- Structs§
- Enums§
- Functions§

bevy::renderModule renderer Copy item pathSource Modules§raw_vulkan_initStructs§RenderAdapterThe handle to the physical device being used for rendering. See Adapter for more info.RenderAdapterInfoThe AdapterInfo of the adapter in use by the renderer.RenderContextThe context with all information required to interact with the GPU.RenderDeviceThis GPU device is responsible for the creation of most rendering and compute resources.RenderInstanceThe GPU instance is used to initialize the RenderQueue and RenderDevice, as well as to create WindowSurfaces.RenderQueueThis queue is used to enqueue tasks for the GPU to execute asynchronously.WgpuWrapperA wrapper to safely make wgpu types Send / Sync on web with atomics enabled.Enums§RenderGraphRunnerErrorFunctions§initialize_rendererInitializes the renderer by retrieving and preparing the GPU instance, device and queue for the specified backend.render_systemUpdates the RenderGraph with all of its nodes and then runs it to render the entire frame.

---

## Struct Skybox Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/core_pipeline/struct.Skybox.html

**Contents:**
- Struct Skybox Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for Skybox
    - fn clone(&self) -> Skybox
    - fn clone_from(&mut self, source: &Self)
  - impl Component for Skyboxwhere Skybox: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

Adds a skybox to a 3D camera, based on a cubemap texture.

Note that this component does not (currently) affect the scene’s lighting. To do so, use EnvironmentMapLight alongside this component.

See also https://en.wikipedia.org/wiki/Skybox_(video_games).

Scale factor applied to the skybox image. After applying this multiplier to the image samples, the resulting values should be in units of cd/m^2.

View space rotation applied to the skybox cubemap. This is useful for users who require a different axis, such as the Z-axis, to serve as the vertical axis.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Skybox {
    pub image: Handle<Image>,
    pub brightness: f32,
    pub rotation: Quat,
}
```

---

## Struct PreprocessPipelineKey Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PreprocessPipelineKey.html

**Contents:**
- Struct PreprocessPipelineKey Copy item path
- Implementations§
  - impl PreprocessPipelineKey
    - pub const FRUSTUM_CULLING: PreprocessPipelineKey
    - pub const OCCLUSION_CULLING: PreprocessPipelineKey
    - pub const EARLY_PHASE: PreprocessPipelineKey
  - impl PreprocessPipelineKey
    - pub const fn empty() -> PreprocessPipelineKey
    - pub const fn all() -> PreprocessPipelineKey
    - pub const fn bits(&self) -> u8

Specifies variants of the mesh preprocessing shader.

Whether GPU frustum culling is in use.

This #define’s FRUSTUM_CULLING in the shader.

Whether GPU two-phase occlusion culling is in use.

This #define’s OCCLUSION_CULLING in the shader.

Whether this is the early phase of GPU two-phase occlusion culling.

This #define’s EARLY_PHASE in the shader.

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

The bitwise and (&) of the bits in two 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct PreprocessPipelineKey(/* private fields */);
```

---

## Struct GizmoPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/struct.GizmoPlugin.html

**Contents:**
- Struct GizmoPlugin Copy item path
- Trait Implementations§
  - impl Default for GizmoPlugin
    - fn default() -> GizmoPlugin
  - impl Plugin for GizmoPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str

A Plugin that provides an immediate mode drawing api for visual debugging.

Requires to be loaded after PbrPlugin or SpriteRenderPlugin.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GizmoPlugin;
```

---

## Module view Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/view/index.html

**Contents:**
- Module view Copy item path
- Modules§
- Structs§
- Enums§
- Constants§
- Functions§

bevy::renderModule view Copy item pathSource Modules§visibilitywindowStructs§ColorGradingConfigures filmic color grading parameters to adjust the image appearance.ColorGradingGlobalFilmic color grading values applied to the image as a whole (as opposed to individual sections, like shadows and highlights).ColorGradingSectionA section of color grading values that can be selectively applied to shadows, midtones, and highlights.ColorGradingUniformThe ColorGrading structure, packed into the most efficient form for the GPU.ExtractedViewDescribes a camera in the render world.ExtractedWindowExtractedWindowsHdrIf this component is added to a camera, the camera will use an intermediate “high dynamic range” render texture. This allows rendering with a wider range of lighting values. However, this does not affect whether the camera will render with hdr display output (which bevy does not support currently) and only affects the intermediate render texture.NoIndirectDrawingAdd this component to a camera to disable indirect mode.PostProcessWriteRenderVisibilityRangePluginA plugin that enables RenderVisibilityRangess, which allow entities to be hidden or shown based on distance to the camera.RenderVisibilityRangesStores information related to VisibilityRanges in the render world.RenderVisibleEntitiesCollection of entities visible from the current view.RetainedViewEntityAn identifier for a view that is stable across frames.ViewDepthTextureViewPluginViewTargetViewTargetAttachmentsContains OutputColorAttachment used for each target present on any view in the current frame, after being prepared by prepare_view_attachments. Users that want to override the default output color attachment for a specific target can do so by adding a OutputColorAttachment to this resource before prepare_view_targets is called.ViewUniformViewUniformOffsetViewUniformsWindowRenderPluginWindowSurfacesEnums§MsaaComponent for configuring the number of samples for Multi-Sample Anti-Aliasing for a Camera.Constants§VISIBILITY_RANGES_STORAGE_BUFFER_COUNTWe need at least 4 storage buffer bindings available to enable the visibility range buffer.Functions§clear_view_attachmentsClears the view target OutputColorAttachments.create_surfacesCreates window surfaces.extract_visibility_rangesExtracts all VisibilityRange components from the main world to the render world and inserts them into RenderVisibilityRanges.need_surface_configurationprepare_view_attachmentsPrepares the view target OutputColorAttachment for eac

*[Content truncated]*

---

## Struct MeshPipelineKey Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshPipelineKey.html

**Contents:**
- Struct MeshPipelineKey Copy item path
- Implementations§
  - impl MeshPipelineKey
    - pub const NONE: MeshPipelineKey
    - pub const MORPH_TARGETS: MeshPipelineKey
    - pub const HDR: MeshPipelineKey
    - pub const TONEMAP_IN_SHADER: MeshPipelineKey
    - pub const DEBAND_DITHER: MeshPipelineKey
    - pub const DEPTH_PREPASS: MeshPipelineKey
    - pub const NORMAL_PREPASS: MeshPipelineKey

MSAA uses the highest 3 bits for the MSAA log2(sample count) to support up to 128x MSAA.

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

The bitwise or (|

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshPipelineKey(/* private fields */);
```

Example 2 (javascript):
```javascript
186    fn specialize(
187        &self,
188        mesh_key: Self::Key,
189        layout: &MeshVertexBufferLayoutRef,
190    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
191        // Define the vertex attributes based on a standard bevy [`Mesh`]
192        let mut vertex_attributes = Vec::new();
193        if layout.0.contains(Mesh::ATTRIBUTE_POSITION) {
194            // Make sure this matches the shader location
195            vertex_attributes.push(Mesh::ATTRIBUTE_POSITION.at_shader_location(0));
196        }
197        if layout.0.contains(Mesh::ATTRIBUTE_COLOR) 
...
```

Example 3 (javascript):
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

Example 4 (javascript):
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

---

## Enum ExrTextureLoaderError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.ExrTextureLoaderError.html

**Contents:**
- Enum ExrTextureLoaderError Copy item path
- Variants (Non-exhaustive)§
  - Io(Error)
  - ImageError(ImageError)
  - TextureAccess(TextureAccessError)
- Trait Implementations§
  - impl Debug for ExrTextureLoaderError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for ExrTextureLoaderError
    - fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>

Possible errors that can be produced by ExrTextureLoader

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[non_exhaustive]pub enum ExrTextureLoaderError {
    Io(Error),
    ImageError(ImageError),
    TextureAccess(TextureAccessError),
}
```

---

## Struct MeshFlags Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshFlags.html

**Contents:**
- Struct MeshFlags Copy item path
- Implementations§
  - impl MeshFlags
    - pub const LOD_INDEX_MASK: MeshFlags
    - pub const NO_FRUSTUM_CULLING: MeshFlags
    - pub const SHADOW_RECEIVER: MeshFlags
    - pub const TRANSMITTED_SHADOW_RECEIVER: MeshFlags
    - pub const SIGN_DETERMINANT_MODEL_3X3: MeshFlags
    - pub const NONE: MeshFlags
    - pub const UNINITIALIZED: MeshFlags

Various flags and tightly-packed values on a mesh.

Flags grow from the top bit down; other values grow from the bottom bit up.

Bitmask for the 16-bit index into the LOD array.

This will be u16::MAX if this mesh has no LOD.

Disables frustum culling for this mesh.

This corresponds to the [bevy_render::view::visibility::NoFrustumCulling] component.

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

The first bit of the LOD index.

The bitwise and (&) of the bits in two flags values.

The bi

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshFlags(/* private fields */);
```

---

## Struct RenderCubemapVisibleEntities Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.RenderCubemapVisibleEntities.html

**Contents:**
- Struct RenderCubemapVisibleEntities Copy item path
- Implementations§
  - impl RenderCubemapVisibleEntities
    - pub fn get(&self, i: usize) -> &RenderVisibleMeshEntities
    - pub fn get_mut(&mut self, i: usize) -> &mut RenderVisibleMeshEntities
    - pub fn iter(&self) -> impl DoubleEndedIterator
    - pub fn iter_mut(&mut self) -> impl DoubleEndedIterator
- Trait Implementations§
  - impl Clone for RenderCubemapVisibleEntities
    - fn clone(&self) -> RenderCubemapVisibleEntities

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderCubemapVisibleEntities { /* private fields */ }
```

---

## Struct MeshCullingDataBuffer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshCullingDataBuffer.html

**Contents:**
- Struct MeshCullingDataBuffer Copy item path
- Methods from Deref<Target = RawBufferVec<MeshCullingData>>§
    - pub fn buffer(&self) -> Option<&Buffer>
      - Examples found in repository?
    - pub fn binding(&self) -> Option<BindingResource<'_>>
    - pub fn capacity(&self) -> usize
    - pub fn len(&self) -> usize
    - pub fn is_empty(&self) -> bool
    - pub fn push(&mut self, value: T) -> usize
      - Examples found in repository?

A GPU buffer that holds the information needed to cull meshes on GPU.

At the moment, this simply holds each mesh’s AABB.

To avoid wasting CPU time in the CPU culling case, this buffer will be empty if GPU culling isn’t in use.

Returns a handle to the buffer, if the data has been uploaded.

Returns the binding for the buffer if the data has been uploaded.

Returns the amount of space that the GPU will use before reallocating.

Returns the number of items that have been pushed to this buffer.

Returns true if the buffer is empty.

Adds a new value and returns its index.

Returns the value at the given index.

Sets the value at the given index.

The index must be less than RawBufferVec::len.

Preallocates space for count elements in the internal CPU-side buffer.

Unlike RawBufferVec::reserve, this doesn’t have any effect on the GPU buffer.

Changes the debugging label of the buffer.

The next time the buffer is updated (via reserve), Bevy will inform the driver of the new label.

Creates a Buffer on the RenderDevice with size at least size_of::<T>() * capacity, unless a such a buffer already exists.

If a Buffer exists, but is too small, references to it will be discarded, and a new Buffer will be created. Any previously created Buffers that are no longer referenced will be deleted by the RenderDevice once it is done using them (typically 1-2 frames).

In addition to any BufferUsages provided when the RawBufferVec was created, the buffer on the RenderDevice is marked as BufferUsages::COPY_DST.

Queues writing of data from system RAM to VRAM using the RenderDevice and the provided RenderQueue.

Before queuing the write, a reserve operation is executed.

Queues writing of data from system RAM to VRAM using the RenderDevice and the provided RenderQueue.

If the buffer is not initialized on the GPU or the range is bigger than the capacity it will return an error. You’ll need to either reserve a new buffer which will lose data on the GPU or create a new buffer and copy the old data to it.

This will only write the data contained in the given range. It is useful if you only want to update a part of the buffer.

Reduces the length of the buffer.

Removes all elements from the buffer.

Removes and returns the last element in the buffer.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshCullingDataBuffer(/* private fields */);
```

Example 2 (javascript):
```javascript
69    fn render<'w>(
70        _: &P,
71        _: ROQueryItem<'w, '_, Self::ViewQuery>,
72        _: Option<ROQueryItem<'w, '_, Self::ItemQuery>>,
73        custom_phase_item_buffers: SystemParamItem<'w, '_, Self::Param>,
74        pass: &mut TrackedRenderPass<'w>,
75    ) -> RenderCommandResult {
76        // Borrow check workaround.
77        let custom_phase_item_buffers = custom_phase_item_buffers.into_inner();
78
79        // Tell the GPU where the vertices are.
80        pass.set_vertex_buffer(
81            0,
82            custom_phase_item_buffers
83                .vertices
84      
...
```

Example 3 (javascript):
```javascript
365    fn from_world(world: &mut World) -> Self {
366        let render_device = world.resource::<RenderDevice>();
367        let render_queue = world.resource::<RenderQueue>();
368
369        // Create the vertex and index buffers.
370        let mut vbo = RawBufferVec::new(BufferUsages::VERTEX);
371        let mut ibo = RawBufferVec::new(BufferUsages::INDEX);
372
373        for vertex in &VERTICES {
374            vbo.push(*vertex);
375        }
376        for index in 0..3 {
377            ibo.push(index);
378        }
379
380        // These two lines are required in order to trigger the u
...
```

Example 4 (javascript):
```javascript
365    fn from_world(world: &mut World) -> Self {
366        let render_device = world.resource::<RenderDevice>();
367        let render_queue = world.resource::<RenderQueue>();
368
369        // Create the vertex and index buffers.
370        let mut vbo = RawBufferVec::new(BufferUsages::VERTEX);
371        let mut ibo = RawBufferVec::new(BufferUsages::INDEX);
372
373        for vertex in &VERTICES {
374            vbo.push(*vertex);
375        }
376        for index in 0..3 {
377            ibo.push(index);
378        }
379
380        // These two lines are required in order to trigger the u
...
```

---

## Module pipelined_rendering Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/pipelined_rendering/index.html

**Contents:**
- Module pipelined_rendering Copy item path
- Structs§

bevy::renderModule pipelined_rendering Copy item pathSource Structs§PipelinedRenderingPluginThe PipelinedRenderingPlugin can be added to your application to enable pipelined rendering.RenderAppChannelsChannels used by the main app to send and receive the render app.RenderExtractAppA Label for the sub app that runs the parts of pipelined rendering that need to run on the main thread.

---

## Crate sprite Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/sprite/index.html

**Contents:**
- Crate sprite Copy item path
- Modules§
- Structs§
- Enums§
- Functions§
- Type Aliases§

Provides 2D sprite functionality.

---

## Module decal Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/decal/index.html

**Contents:**
- Module decal Copy item path
- Modules§
- Structs§
- Type Aliases§

Decals are a material that render on top of the surface that they’re placed above. They can be used to render signs, paint, snow, impact craters, and other effects on top of surfaces.

---

## Enum MeshTrianglesError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/enum.MeshTrianglesError.html

**Contents:**
- Enum MeshTrianglesError Copy item path
- Variants§
  - WrongTopology
  - MissingPositions
  - PositionsFormat
  - MissingIndices
  - BadIndices
- Trait Implementations§
  - impl Debug for MeshTrianglesError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

An error that occurred while trying to extract a collection of triangles from a Mesh.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum MeshTrianglesError {
    WrongTopology,
    MissingPositions,
    PositionsFormat,
    MissingIndices,
    BadIndices,
}
```

---

## Struct ConvexPolygonMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.ConvexPolygonMeshBuilder.html

**Contents:**
- Struct ConvexPolygonMeshBuilder Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for ConvexPolygonMeshBuilder
    - fn clone(&self) -> ConvexPolygonMeshBuilder
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for ConvexPolygonMeshBuilder
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Extrudable for ConvexPolygonMeshBuilder
    - fn perimeter(&self) -> Vec<PerimeterSegment>

A builder used for creating a Mesh with a ConvexPolygon shape.

You must verify that the vertices are not concave when constructing this type. You can guarantee this by creating a ConvexPolygon first, then calling ConvexPolygon::mesh().

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ConvexPolygonMeshBuilder {
    pub vertices: Vec<Vec2>,
}
```

---

## Crate mesh Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/index.html

**Contents:**
- Crate mesh Copy item path
- Modules§
- Structs§
- Enums§
- Constants§
- Traits§
- Functions§

bevyCrate mesh Copy item pathSource Modules§morphpreludeThe mesh prelude.primitivesMesh generation for primitive shapes.skinningStructs§AnnulusMeshBuilderA builder for creating a Mesh with an Annulus shape.BaseMeshPipelineKeyOur base mesh pipeline key bits start from the highest bit and go downward. The PBR mesh pipeline key bits start from the lowest bit and go upward. This allows the PBR bits in the downstream crate bevy_pbr to coexist in the same field without any shifts.Capsule2dMeshBuilderA builder used for creating a Mesh with a Capsule2d shape.Capsule3dMeshBuilderA builder used for creating a Mesh with a Capsule3d shape.CircleMeshBuilderA builder used for creating a Mesh with a Circle shape.CircularSectorMeshBuilderA builder used for creating a Mesh with a CircularSector shape.CircularSegmentMeshBuilderA builder used for creating a Mesh with a CircularSegment shape.ConeMeshBuilderA builder used for creating a Mesh with a Cone shape.ConicalFrustumMeshBuilderA builder used for creating a Mesh with a ConicalFrustum shape.ConvexPolygonMeshBuilderA builder used for creating a Mesh with a ConvexPolygon shape.CuboidMeshBuilderA builder used for creating a Mesh with a Cuboid shape.CylinderMeshBuilderA builder used for creating a Mesh with a Cylinder shape.EllipseMeshBuilderA builder used for creating a Mesh with an Ellipse shape.ExtrusionBuilderA builder used for creating a Mesh with an Extrusion shape.InheritWeightSystemsbevy_render::mesh::inherit_weights runs in this SystemSetMeshA 3D object made out of vertices representing triangles, lines, or points, with “attribute” values for each vertex.Mesh2dA component for 2D meshes. Requires a MeshMaterial2d to be rendered, commonly using a ColorMaterial.Mesh3dA component for 3D meshes. Requires a MeshMaterial3d to be rendered, commonly using a StandardMaterial.MeshDeserializerUse to specify extra options when deserializing a SerializedMesh into a Mesh.MeshPluginAdds Mesh as an asset.MeshTagA component that stores an arbitrary index used to identify the mesh instance when rendering.MeshVertexAttributeMeshVertexAttributeIdMeshVertexBufferLayoutMeshVertexBufferLayoutRefDescribes the layout of the mesh vertices in GPU memory.MeshVertexBufferLayoutsStores the single copy of each mesh vertex buffer layout.MissingVertexAttributeErrorPlaneMeshBuilderA builder used for creating a Mesh with a Plane3d shape.Polyline2dMeshBuilderA builder used for creating a Mesh with a Polyline2d shape.RectangleMeshBuilderA builder used fo

*[Content truncated]*

---

## Enum NormalizedRenderTarget Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/enum.NormalizedRenderTarget.html

**Contents:**
- Enum NormalizedRenderTarget Copy item path
- Variants§
  - Window(NormalizedWindowRef)
  - Image(ImageRenderTarget)
  - TextureView(ManualTextureViewHandle)
  - None
    - Fields
- Trait Implementations§
  - impl Clone for NormalizedRenderTarget
    - fn clone(&self) -> NormalizedRenderTarget

Normalized version of the render target.

Once we have this we shouldn’t need to resolve it down anymore.

Window to which the camera’s view is rendered.

Image to which the camera’s view is rendered.

Texture View to which the camera’s view is rendered. Useful when the texture view needs to be created outside of Bevy, for example OpenXR.

The camera won’t render to any color target.

This is useful when you want a camera that only renders prepasses, for example a depth prepass. See the render_depth_to_texture example.

The physical width of the viewport.

The physical height of the viewport.

Retrieves the TextureFormat of this render target, if it exists.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum NormalizedRenderTarget {
    Window(NormalizedWindowRef),
    Image(ImageRenderTarget),
    TextureView(ManualTextureViewHandle),
    None {
        width: u32,
        height: u32,
    },
}
```

---

## Struct CustomCursorImage Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/window/struct.CustomCursorImage.html

**Contents:**
- Struct CustomCursorImage Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for CustomCursorImage
    - fn clone(&self) -> CustomCursorImage
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for CustomCursorImage
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for CustomCursorImage
    - fn default() -> CustomCursorImage

A custom cursor created from an image.

Handle to the image to use as the cursor. The image must be in 8 bit int or 32 bit float rgba. PNG images work well for this.

An optional texture atlas used to render the image.

Whether the image should be flipped along its x-axis.

If true, the cursor’s hotspot automatically flips along with the image.

Whether the image should be flipped along its y-axis.

If true, the cursor’s hotspot automatically flips along with the image.

An optional rectangle representing the region of the image to render, instead of rendering the full image. This is an easy one-off alternative to using a TextureAtlas.

When used with a TextureAtlas, the rect is offset by the atlas’s minimal (top-left) corner position.

X and Y coordinates of the hotspot in pixels. The hotspot must be within the image bounds.

If you are flipping the image using flip_x or flip_y, you don’t need to adjust this field to account for the flip because it is adjusted automatically.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CustomCursorImage {
    pub handle: Handle<Image>,
    pub texture_atlas: Option<TextureAtlas>,
    pub flip_x: bool,
    pub flip_y: bool,
    pub rect: Option<URect>,
    pub hotspot: (u16, u16),
}
```

---

## Enum UvChannel Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/enum.UvChannel.html

**Contents:**
- Enum UvChannel Copy item path
- Variants§
  - Uv0
  - Uv1
- Trait Implementations§
  - impl Clone for UvChannel
    - fn clone(&self) -> UvChannel
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for UvChannel
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

An enum to define which UV attribute to use for a texture.

It is used for every texture in the StandardMaterial. It only supports two UV attributes, bevy_mesh::Mesh::ATTRIBUTE_UV_0 and bevy_mesh::Mesh::ATTRIBUTE_UV_1. The default is UvChannel::Uv0.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum UvChannel {
    Uv0,
    Uv1,
}
```

---

## Constant VERTEX_ATTRIBUTE_BUFFER_ID Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/constant.VERTEX_ATTRIBUTE_BUFFER_ID.html

**Contents:**
- Constant VERTEX_ATTRIBUTE_BUFFER_ID Copy item path

bevy::meshConstant VERTEX_ATTRIBUTE_BUFFER_ID Copy item pathSource pub const VERTEX_ATTRIBUTE_BUFFER_ID: u64 = 10; // 10u64

**Examples:**

Example 1 (javascript):
```javascript
pub const VERTEX_ATTRIBUTE_BUFFER_ID: u64 = 10; // 10u64
```

---

## Struct MaterialExtensionBindGroupData Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialExtensionBindGroupData.html

**Contents:**
- Struct MaterialExtensionBindGroupData Copy item path
- Fields§
- Trait Implementations§
  - impl<B, E> Clone for MaterialExtensionBindGroupData<B, E>where B: Clone + Copy, E: Clone + Copy,
    - fn clone(&self) -> MaterialExtensionBindGroupData<B, E>
    - fn clone_from(&mut self, source: &Self)
  - impl<B, E> Hash for MaterialExtensionBindGroupData<B, E>where B: Hash + Copy, E: Hash + Copy,
    - fn hash<__H>(&self, state: &mut __H)where __H: Hasher,
    - fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,
  - impl<B, E> PartialEq for MaterialExtensionBindGroupData<B, E>where B: PartialEq + Copy, E: PartialEq + Copy,

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C, packed(1))]pub struct MaterialExtensionBindGroupData<B, E> {
    pub base: B,
    pub extension: E,
}
```

---

## Enum RenderMeshInstanceGpuQueue Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/enum.RenderMeshInstanceGpuQueue.html

**Contents:**
- Enum RenderMeshInstanceGpuQueue Copy item path
- Variants§
  - None
  - CpuCulling
    - Fields
  - GpuCulling
    - Fields
- Trait Implementations§
  - impl Default for RenderMeshInstanceGpuQueue
    - fn default() -> RenderMeshInstanceGpuQueue

The per-thread queues used during extract_meshes_for_gpu_building.

There are two varieties of these: one for when culling happens on CPU and one for when culling happens on GPU. Having the two varieties avoids wasting space if GPU culling is disabled.

This becomes RenderMeshInstanceGpuQueue::CpuCulling or RenderMeshInstanceGpuQueue::GpuCulling once extraction starts.

The version of RenderMeshInstanceGpuQueue that omits the MeshCullingData, so that we don’t waste space when GPU culling is disabled.

Stores GPU data for each entity that became visible or changed in such a way that necessitates updating the MeshInputUniform (e.g. changed transform).

Stores the IDs of entities that became invisible this frame.

The version of RenderMeshInstanceGpuQueue that contains the MeshCullingData, used when any view has GPU culling enabled.

Stores GPU data for each entity that became visible or changed in such a way that necessitates updating the MeshInputUniform (e.g. changed transform).

Stores the IDs of entities that became invisible this frame.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum RenderMeshInstanceGpuQueue {
    None,
    CpuCulling {
        changed: Vec<(MainEntity, RenderMeshInstanceGpuBuilder)>,
        removed: Vec<MainEntity>,
    },
    GpuCulling {
        changed: Vec<(MainEntity, RenderMeshInstanceGpuBuilder, MeshCullingData)>,
        removed: Vec<MainEntity>,
    },
}
```

---

## Struct GltfPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/struct.GltfPlugin.html

**Contents:**
- Struct GltfPlugin Copy item path
- Fields§
- Implementations§
  - impl GltfPlugin
    - pub fn add_custom_vertex_attribute( self, name: &str, attribute: MeshVertexAttribute, ) -> GltfPlugin
      - Examples found in repository?
- Trait Implementations§
  - impl Default for GltfPlugin
    - fn default() -> GltfPlugin
  - impl Plugin for GltfPlugin

Adds support for glTF file loading to the app.

The default image sampler to lay glTF sampler data on top of.

Can be modified with the DefaultGltfImageSampler resource.

CAUTION: This is an experimental feature with known issues. Behavior may change in future versions.

How to convert glTF coordinates on import. Assuming glTF cameras, glTF lights, and glTF meshes had global identity transforms, their Bevy Transform::forward will be pointing in the following global directions:

The default is false.

Registry for custom vertex attributes.

To specify, use GltfPlugin::add_custom_vertex_attribute.

Register a custom vertex attribute so that it is recognized when loading a glTF file with the GltfLoader.

name must be the attribute name as found in the glTF data, which must start with an underscore. See this section of the glTF specification for additional details on custom attributes.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GltfPlugin {
    pub default_sampler: ImageSamplerDescriptor,
    pub use_model_forward_direction: bool,
    pub custom_vertex_attributes: HashMap<Box<str>, MeshVertexAttribute>,
}
```

Example 2 (unknown):
```unknown
24fn main() {
25    App::new()
26        .insert_resource(AmbientLight {
27            color: Color::WHITE,
28            brightness: 1.0 / 5.0f32,
29            ..default()
30        })
31        .add_plugins((
32            DefaultPlugins.set(
33                GltfPlugin::default()
34                    // Map a custom glTF attribute name to a `MeshVertexAttribute`.
35                    // The glTF file used here has an attribute name with *two*
36                    // underscores: __BARYCENTRIC
37                    // One is stripped to do the comparison here.
38                    .add
...
```

---

## Struct StandardMaterialKey Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.StandardMaterialKey.html

**Contents:**
- Struct StandardMaterialKey Copy item path
- Implementations§
  - impl StandardMaterialKey
    - pub const CULL_FRONT: StandardMaterialKey
    - pub const CULL_BACK: StandardMaterialKey
    - pub const NORMAL_MAP: StandardMaterialKey
    - pub const RELIEF_MAPPING: StandardMaterialKey
    - pub const DIFFUSE_TRANSMISSION: StandardMaterialKey
    - pub const SPECULAR_TRANSMISSION: StandardMaterialKey
    - pub const CLEARCOAT: StandardMaterialKey

The pipeline key for StandardMaterial, packed into 64 bits.

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

The bitwise or (|) of the bits in each flags v

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct StandardMaterialKey(/* private fields */);
```

---

## Struct DefaultOpaqueRendererMethod Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.DefaultOpaqueRendererMethod.html

**Contents:**
- Struct DefaultOpaqueRendererMethod Copy item path
- Implementations§
  - impl DefaultOpaqueRendererMethod
    - pub fn forward() -> DefaultOpaqueRendererMethod
    - pub fn deferred() -> DefaultOpaqueRendererMethod
      - Examples found in repository?
    - pub fn set_to_forward(&mut self)
      - Examples found in repository?
    - pub fn set_to_deferred(&mut self)
      - Examples found in repository?

Default render method used for opaque materials.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DefaultOpaqueRendererMethod(/* private fields */);
```

Example 2 (unknown):
```unknown
17fn main() {
18    App::new()
19        .insert_resource(DefaultOpaqueRendererMethod::deferred())
20        .insert_resource(DirectionalLightShadowMap { size: 4096 })
21        .add_plugins(DefaultPlugins)
22        .insert_resource(Pause(true))
23        .add_systems(Startup, (setup, setup_parallax))
24        .add_systems(Update, (animate_light_direction, switch_mode, spin))
25        .run();
26}
```

Example 3 (javascript):
```javascript
22fn main() {
23    #[cfg(not(target_arch = "wasm32"))]
24    let args: Args = argh::from_env();
25    #[cfg(target_arch = "wasm32")]
26    let args: Args = Args::from_args(&[], &[]).unwrap();
27
28    let mut app = App::new();
29    app.add_plugins(DefaultPlugins)
30        .insert_resource(AmbientLight::NONE);
31
32    if args.deferred {
33        app.insert_resource(DefaultOpaqueRendererMethod::deferred());
34    }
35
36    app.insert_resource(args)
37        .add_systems(Startup, setup)
38        .add_systems(Update, add_lightmaps_to_meshes)
39        .run();
40}
```

Example 4 (unknown):
```unknown
98fn main() {
99    // Enable deferred rendering, which is necessary for screen-space
100    // reflections at this time. Disable multisampled antialiasing, as deferred
101    // rendering doesn't support that.
102    App::new()
103        .insert_resource(DefaultOpaqueRendererMethod::deferred())
104        .init_resource::<AppSettings>()
105        .add_plugins(DefaultPlugins.set(WindowPlugin {
106            primary_window: Some(Window {
107                title: "Bevy Screen Space Reflections Example".into(),
108                ..default()
109            }),
110            ..default()
111  
...
```

---

## Struct GltfMeshName Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/struct.GltfMeshName.html

**Contents:**
- Struct GltfMeshName Copy item path
- Tuple Fields§
- Methods from Deref<Target = str>§
    - pub fn len(&self) -> usize
      - §Examples
    - pub fn is_empty(&self) -> bool
      - §Examples
    - pub fn is_char_boundary(&self, index: usize) -> bool
      - §Examples
    - pub fn floor_char_boundary(&self, index: usize) -> usize

The mesh name of a glTF primitive.

See the relevant glTF specification section.

Returns the length of self.

This length is in bytes, not chars or graphemes. In other words, it might not be what a human considers the length of the string.

Returns true if self has a length of zero bytes.

Checks that index-th byte is the first byte in a UTF-8 code point sequence or the end of the string.

The start and end of the string (when index == self.len()) are considered to be boundaries.

Returns false if index is greater than self.len().

Finds the closest x not exceeding index where is_char_boundary(x) is true.

This method can help you truncate a string so that it’s still valid UTF-8, but doesn’t exceed a given number of bytes. Note that this is done purely at the character level and can still visually split graphemes, even though the underlying characters aren’t split. For example, the emoji 🧑‍🔬 (scientist) could be split so that the string only includes 🧑 (person) instead.

Finds the closest x not below index where is_char_boundary(x) is true.

If index is greater than the length of the string, this returns the length of the string.

This method is the natural complement to floor_char_boundary. See that method for more details.

Converts a string slice to a byte slice. To convert the byte slice back into a string slice, use the from_utf8 function.

Converts a string slice to a raw pointer.

As string slices are a slice of bytes, the raw pointer points to a u8. This pointer will be pointing to the first byte of the string slice.

The caller must ensure that the returned pointer is never written to. If you need to mutate the contents of the string slice, use as_mut_ptr.

Returns a subslice of str.

This is the non-panicking alternative to indexing the str. Returns None whenever equivalent indexing operation would panic.

Returns an unchecked subslice of str.

This is the unchecked alternative to indexing the str.

Callers of this function are responsible that these preconditions are satisfied:

Failing that, the returned string slice may reference invalid memory or violate the invariants communicated by the str type.

Creates a string slice from another string slice, bypassing safety checks.

This is generally not recommended, use with caution! For a safe alternative see str and Index.

This new slice goes from begin to end, including begin but excluding end.

To get a mutable string slice instead, see the slice_mut_unchecked method.

Callers of this function 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct GltfMeshName(pub String);
```

Example 2 (javascript):
```javascript
let len = "foo".len();
assert_eq!(3, len);

assert_eq!("ƒoo".len(), 4); // fancy f!
assert_eq!("ƒoo".chars().count(), 3);
```

Example 3 (javascript):
```javascript
let s = "";
assert!(s.is_empty());

let s = "not empty";
assert!(!s.is_empty());
```

Example 4 (javascript):
```javascript
let s = "Löwe 老虎 Léopard";
assert!(s.is_char_boundary(0));
// start of `老`
assert!(s.is_char_boundary(6));
assert!(s.is_char_boundary(s.len()));

// second byte of `ö`
assert!(!s.is_char_boundary(2));

// third byte of `老`
assert!(!s.is_char_boundary(8));
```

---

## Struct PbrPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PbrPlugin.html

**Contents:**
- Struct PbrPlugin Copy item path
- Fields§
- Trait Implementations§
  - impl Default for PbrPlugin
    - fn default() -> PbrPlugin
  - impl Plugin for PbrPlugin
    - fn build(&self, app: &mut App)
    - fn finish(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn cleanup(&self, _app: &mut App)

Sets up the entire PBR infrastructure of bevy.

Controls if the prepass is enabled for the StandardMaterial. For more information about what a prepass is, see the bevy_core_pipeline::prepass docs.

Controls if DeferredPbrLightingPlugin is added.

Controls if GPU MeshUniform building is enabled.

This requires compute shader support and so will be forcibly disabled if the platform doesn’t support those.

Debugging flags that can optionally be set when constructing the renderer.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PbrPlugin {
    pub prepass_enabled: bool,
    pub add_default_deferred_lighting_plugin: bool,
    pub use_gpu_instance_buffer_builder: bool,
    pub debug_flags: RenderDebugFlags,
}
```

---

## Function ktx2_get_texture_format Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/fn.ktx2_get_texture_format.html

**Contents:**
- Function ktx2_get_texture_format Copy item path

bevy::imageFunction ktx2_get_texture_format Copy item pathSource pub fn ktx2_get_texture_format<Data>( ktx2: &Reader<Data>, is_srgb: bool, ) -> Result<TextureFormat, TextureError>where Data: AsRef<[u8]>,

**Examples:**

Example 1 (unknown):
```unknown
pub fn ktx2_get_texture_format<Data>(
    ktx2: &Reader<Data>,
    is_srgb: bool,
) -> Result<TextureFormat, TextureError>where
    Data: AsRef<[u8]>,
```

---

## Struct ClearColor Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.ClearColor.html

**Contents:**
- Struct ClearColor Copy item path
- Tuple Fields§
- Methods from Deref<Target = Color>§
    - pub const WHITE: Color
    - pub const BLACK: Color
    - pub const NONE: Color
    - pub fn to_linear(&self) -> LinearRgba
      - Examples found in repository?
    - pub fn to_srgba(&self) -> Srgba
      - Examples found in repository?

A Resource that stores the default color that cameras use to clear the screen between frames.

This color appears as the “background” color for simple apps, when there are portions of the screen with nothing rendered.

Individual cameras may use Camera.clear_color to specify a different clear color or opt out of clearing their viewport.

Return the color as a linear RGBA color.

Return the color as an SRGBA color.

Match the dark gray bevy website code block color by default.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ClearColor(pub Color);
```

Example 2 (javascript):
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

Example 3 (unknown):
```unknown
228fn spawn_light_textures(
229    commands: &mut Commands,
230    asset_server: &AssetServer,
231    meshes: &mut Assets<Mesh>,
232    materials: &mut Assets<StandardMaterial>,
233) {
234    commands.spawn((
235        SpotLight {
236            color: Color::srgb(1.0, 1.0, 0.8),
237            intensity: 10e6,
238            outer_angle: 0.25,
239            inner_angle: 0.25,
240            shadows_enabled: true,
241            ..default()
242        },
243        Transform::from_translation(Vec3::new(6.0, 1.0, 2.0)).looking_at(Vec3::ZERO, Vec3::Y),
244        SpotLightTexture {
245        
...
```

Example 4 (unknown):
```unknown
104fn update_colors(
105    keyboard_input: Res<ButtonInput<KeyCode>>,
106    mut config: ResMut<Wireframe2dConfig>,
107    mut wireframe_colors: Query<&mut Wireframe2dColor>,
108    mut text: Single<&mut Text>,
109) {
110    text.0 = format!(
111        "Controls
112---------------
113Z - Toggle global
114X - Change global color
115C - Change color of the circle wireframe
116
117Wireframe2dConfig
118-------------
119Global: {}
120Color: {:?}",
121        config.global,
122        config.default_color.to_srgba(),
123    );
124
125    // Toggle showing a wireframe on all meshes
126    if keyboa
...
```

---

## Struct DeferredFragmentShader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.DeferredFragmentShader.html

**Contents:**
- Struct DeferredFragmentShader Copy item path
- Trait Implementations§
  - impl Clone for DeferredFragmentShader
    - fn clone(&self) -> DeferredFragmentShader
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for DeferredFragmentShader
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for DeferredFragmentShader
    - fn default() -> DeferredFragmentShader
  - impl Hash for DeferredFragmentShader

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DeferredFragmentShader;
```

---

## Enum RenderTarget Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/enum.RenderTarget.html

**Contents:**
- Enum RenderTarget Copy item path
- Variants§
  - Window(WindowRef)
  - Image(ImageRenderTarget)
  - TextureView(ManualTextureViewHandle)
  - None
    - Fields
- Implementations§
  - impl RenderTarget
    - pub fn as_image(&self) -> Option<&Handle<Image>>

The “target” that a Camera will render to. For example, this could be a Window swapchain or an Image.

Window to which the camera’s view is rendered.

Image to which the camera’s view is rendered.

Texture View to which the camera’s view is rendered. Useful when the texture view needs to be created outside of Bevy, for example OpenXR.

The camera won’t render to any color target.

This is useful when you want a camera that only renders prepasses, for example a depth prepass. See the render_depth_to_texture example.

The physical size of the viewport.

Get a handle to the render target’s image, or None if the render target is another variant.

Normalize the render target down to a more concrete value, mostly used for equality comparisons.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum RenderTarget {
    Window(WindowRef),
    Image(ImageRenderTarget),
    TextureView(ManualTextureViewHandle),
    None {
        size: UVec2,
    },
}
```

Example 2 (javascript):
```javascript
174fn drive_diegetic_pointer(
175    mut cursor_last: Local<Vec2>,
176    mut raycast: MeshRayCast,
177    rays: Res<RayMap>,
178    cubes: Query<&Mesh3d, With<Cube>>,
179    ui_camera: Query<&Camera, With<Camera2d>>,
180    primary_window: Query<Entity, With<PrimaryWindow>>,
181    windows: Query<(Entity, &Window)>,
182    images: Res<Assets<Image>>,
183    manual_texture_views: Res<ManualTextureViews>,
184    mut window_events: MessageReader<WindowEvent>,
185    mut pointer_inputs: MessageWriter<PointerInput>,
186) -> Result {
187    // Get the size of the texture, so we can convert from dim
...
```

---

## Struct ExtractSchedule Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/struct.ExtractSchedule.html

**Contents:**
- Struct ExtractSchedule Copy item path
- Trait Implementations§
  - impl Clone for ExtractSchedule
    - fn clone(&self) -> ExtractSchedule
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for ExtractSchedule
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for ExtractSchedule
    - fn default() -> ExtractSchedule
  - impl Hash for ExtractSchedule

Schedule which extract data from the main world and inserts it into the render world.

This step should be kept as short as possible to increase the “pipelining potential” for running the next frame while rendering the current frame.

This schedule is run on the main world, but its buffers are not applied until it is returned to the render world.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ExtractSchedule;
```

---

## Struct MeshVertexBufferLayouts Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.MeshVertexBufferLayouts.html

**Contents:**
- Struct MeshVertexBufferLayouts Copy item path
- Implementations§
  - impl MeshVertexBufferLayouts
    - pub fn insert( &mut self, layout: MeshVertexBufferLayout, ) -> MeshVertexBufferLayoutRef
- Trait Implementations§
  - impl Clone for MeshVertexBufferLayouts
    - fn clone(&self) -> MeshVertexBufferLayouts
    - fn clone_from(&mut self, source: &Self)
  - impl Default for MeshVertexBufferLayouts
    - fn default() -> MeshVertexBufferLayouts

Stores the single copy of each mesh vertex buffer layout.

Inserts a new mesh vertex buffer layout in the store and returns a reference to it, reusing the existing reference if this mesh vertex buffer layout was already in the store.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshVertexBufferLayouts(/* private fields */);
```

---

## Enum VertexFormat Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/enum.VertexFormat.html

**Contents:**
- Enum VertexFormat Copy item path
- Variants§
  - Uint8 = 0
  - Uint8x2 = 1
  - Uint8x4 = 2
  - Sint8 = 3
  - Sint8x2 = 4
  - Sint8x4 = 5
  - Unorm8 = 6
  - Unorm8x2 = 7

Vertex Format for a VertexAttribute (input).

Corresponds to WebGPU GPUVertexFormat.

One unsigned byte (u8). u32 in shaders.

Two unsigned bytes (u8). vec2<u32> in shaders.

Four unsigned bytes (u8). vec4<u32> in shaders.

One signed byte (i8). i32 in shaders.

Two signed bytes (i8). vec2<i32> in shaders.

Four signed bytes (i8). vec4<i32> in shaders.

One unsigned byte (u8). [0, 255] converted to float [0, 1] f32 in shaders.

Two unsigned bytes (u8). [0, 255] converted to float [0, 1] vec2<f32> in shaders.

Four unsigned bytes (u8). [0, 255] converted to float [0, 1] vec4<f32> in shaders.

One signed byte (i8). [−127, 127] converted to float [−1, 1] f32 in shaders.

Two signed bytes (i8). [−127, 127] converted to float [−1, 1] vec2<f32> in shaders.

Four signed bytes (i8). [−127, 127] converted to float [−1, 1] vec4<f32> in shaders.

One unsigned short (u16). u32 in shaders.

Two unsigned shorts (u16). vec2<u32> in shaders.

Four unsigned shorts (u16). vec4<u32> in shaders.

One signed short (u16). i32 in shaders.

Two signed shorts (i16). vec2<i32> in shaders.

Four signed shorts (i16). vec4<i32> in shaders.

One unsigned short (u16). [0, 65535] converted to float [0, 1] f32 in shaders.

Two unsigned shorts (u16). [0, 65535] converted to float [0, 1] vec2<f32> in shaders.

Four unsigned shorts (u16). [0, 65535] converted to float [0, 1] vec4<f32> in shaders.

One signed short (i16). [−32767, 32767] converted to float [−1, 1] f32 in shaders.

Two signed shorts (i16). [−32767, 32767] converted to float [−1, 1] vec2<f32> in shaders.

Four signed shorts (i16). [−32767, 32767] converted to float [−1, 1] vec4<f32> in shaders.

One half-precision float (no Rust equiv). f32 in shaders.

Two half-precision floats (no Rust equiv). vec2<f32> in shaders.

Four half-precision floats (no Rust equiv). vec4<f32> in shaders.

One single-precision float (f32). f32 in shaders.

Two single-precision floats (f32). vec2<f32> in shaders.

Three single-precision floats (f32). vec3<f32> in shaders.

Four single-precision floats (f32). vec4<f32> in shaders.

One unsigned int (u32). u32 in shaders.

Two unsigned ints (u32). vec2<u32> in shaders.

Three unsigned ints (u32). vec3<u32> in shaders.

Four unsigned ints (u32). vec4<u32> in shaders.

One signed int (i32). i32 in shaders.

Two signed ints (i32). vec2<i32> in shaders.

Three signed ints (i32). vec3<i32> in shaders.

Four signed ints (i32). vec4<i32> in shaders.

One double-precision float (f64). f32 in shaders. Requires F

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub enum VertexFormat {
Show 45 variants    Uint8 = 0,
    Uint8x2 = 1,
    Uint8x4 = 2,
    Sint8 = 3,
    Sint8x2 = 4,
    Sint8x4 = 5,
    Unorm8 = 6,
    Unorm8x2 = 7,
    Unorm8x4 = 8,
    Snorm8 = 9,
    Snorm8x2 = 10,
    Snorm8x4 = 11,
    Uint16 = 12,
    Uint16x2 = 13,
    Uint16x4 = 14,
    Sint16 = 15,
    Sint16x2 = 16,
    Sint16x4 = 17,
    Unorm16 = 18,
    Unorm16x2 = 19,
    Unorm16x4 = 20,
    Snorm16 = 21,
    Snorm16x2 = 22,
    Snorm16x4 = 23,
    Float16 = 24,
    Float16x2 = 25,
    Float16x4 = 26,
    Float32 = 27,
    Float32x2 = 28,
    Float32x3 = 29,
    
...
```

Example 2 (javascript):
```javascript
217    fn specialize(
218        &self,
219        key: Self::Key,
220        layout: &MeshVertexBufferLayoutRef,
221    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
222        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;
223
224        descriptor.vertex.shader = self.shader.clone();
225        descriptor.vertex.buffers.push(VertexBufferLayout {
226            array_stride: size_of::<InstanceData>() as u64,
227            step_mode: VertexStepMode::Instance,
228            attributes: vec![
229                VertexAttribute {
230                  
...
```

---

## Struct OrthographicProjection Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.OrthographicProjection.html

**Contents:**
- Struct OrthographicProjection Copy item path
- §Examples
- Fields§
- Implementations§
  - impl OrthographicProjection
    - pub fn default_2d() -> OrthographicProjection
    - pub fn default_3d() -> OrthographicProjection
      - Examples found in repository?
- Trait Implementations§
  - impl CameraProjection for OrthographicProjection

Project a 3D space onto a 2D surface using parallel lines, i.e., unlike PerspectiveProjection, the size of objects remains the same regardless of their distance to the camera.

The volume contained in the projection is called the view frustum. Since the viewport is rectangular and projection lines are parallel, the view frustum takes the shape of a cuboid.

Note that the scale of the projection and the apparent size of objects are inversely proportional. As the size of the projection increases, the size of objects decreases.

Configure the orthographic projection to one world unit per 100 window pixels:

The distance of the near clipping plane in world units.

Objects closer than this will not be rendered.

The distance of the far clipping plane in world units.

Objects further than this will not be rendered.

Specifies the origin of the viewport as a normalized position from 0 to 1, where (0, 0) is the bottom left and (1, 1) is the top right. This determines where the camera’s position sits inside the viewport.

When the projection scales due to viewport resizing, the position of the camera, and thereby viewport_origin, remains at the same relative point.

Consequently, this is pivot point when scaling. With a bottom left pivot, the projection will expand upwards and to the right. With a top right pivot, the projection will expand downwards and to the left. Values in between will caused the projection to scale proportionally on each axis.

Defaults to (0.5, 0.5), which makes scaling affect opposite sides equally, keeping the center point of the viewport centered.

How the projection will scale to the viewport.

Defaults to ScalingMode::WindowSize, and works in concert with OrthographicProjection::scale to determine the final effect.

For simplicity, zooming should be done by changing OrthographicProjection::scale, rather than changing the parameters of the scaling mode.

Scales the projection.

As scale increases, the apparent size of objects decreases, and vice versa.

Note: scaling can be set by scaling_mode as well. This parameter scales on top of that.

This property is particularly useful in implementing zoom functionality.

Defaults to 1.0, which under standard settings corresponds to a 1:1 mapping of world units to rendered pixels. See ScalingMode::WindowSize for more information.

The area that the projection covers relative to viewport_origin.

Bevy’s camera_system automatically updates this value when the viewport is resized depending on Orthogr

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct OrthographicProjection {
    pub near: f32,
    pub far: f32,
    pub viewport_origin: Vec2,
    pub scaling_mode: ScalingMode,
    pub scale: f32,
    pub area: Rect,
}
```

Example 2 (javascript):
```javascript
let projection = Projection::Orthographic(OrthographicProjection {
    scaling_mode: ScalingMode::WindowSize,
    scale: 0.01,
    ..OrthographicProjection::default_2d()
});
```

Example 3 (javascript):
```javascript
263fn modify_projection(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Projection>) {
264    for mut projection in &mut query {
265        if keys.just_pressed(KeyCode::KeyO) {
266            match *projection {
267                Projection::Perspective(_) => {
268                    *projection = Projection::Orthographic(OrthographicProjection {
269                        scale: 0.002,
270                        ..OrthographicProjection::default_3d()
271                    });
272                }
273                _ => {
274                    *projection = Projection::Perspective(
...
```

Example 4 (unknown):
```unknown
13fn setup(
14    mut commands: Commands,
15    mut meshes: ResMut<Assets<Mesh>>,
16    mut materials: ResMut<Assets<StandardMaterial>>,
17) {
18    // camera
19    commands.spawn((
20        Camera3d::default(),
21        Projection::from(OrthographicProjection {
22            // 6 world units per pixel of window height.
23            scaling_mode: ScalingMode::FixedVertical {
24                viewport_height: 6.0,
25            },
26            ..OrthographicProjection::default_3d()
27        }),
28        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
29    ));
30
31  
...
```

---

## Struct HdrTextureLoaderSettings Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.HdrTextureLoaderSettings.html

**Contents:**
- Struct HdrTextureLoaderSettings Copy item path
- Fields§
- Trait Implementations§
  - impl Debug for HdrTextureLoaderSettings
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for HdrTextureLoaderSettings
    - fn default() -> HdrTextureLoaderSettings
  - impl<'de> Deserialize<'de> for HdrTextureLoaderSettings
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<HdrTextureLoaderSettings, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,
  - impl Serialize for HdrTextureLoaderSettings

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct HdrTextureLoaderSettings {
    pub asset_usage: RenderAssetUsages,
}
```

---

## Struct ImageRenderTarget Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.ImageRenderTarget.html

**Contents:**
- Struct ImageRenderTarget Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for ImageRenderTarget
    - fn clone(&self) -> ImageRenderTarget
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for ImageRenderTarget
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl From<Handle<Image>> for ImageRenderTarget
    - fn from(handle: Handle<Image>) -> ImageRenderTarget

A render target that renders to an Image.

The image to render to.

The scale factor of the render target image, corresponding to the scale factor for a window target. This should almost always be 1.0.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ImageRenderTarget {
    pub handle: Handle<Image>,
    pub scale_factor: FloatOrd,
}
```

---

## Struct PointLightTexture Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.PointLightTexture.html

**Contents:**
- Struct PointLightTexture Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for PointLightTexture
    - fn clone(&self) -> PointLightTexture
    - fn clone_from(&mut self, source: &Self)
  - impl Component for PointLightTexturewhere PointLightTexture: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

Add to a PointLight to add a light texture effect. A texture mask is applied to the light source to modulate its intensity, simulating patterns like window shadows, gobo/cookie effects, or soft falloffs.

The texture image. Only the R channel is read.

The cubemap layout. The image should be a packed cubemap in one of the formats described by the CubemapLayout enum.

Required Components: PointLight.

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components of the required components, recursively, in depth-first order.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PointLightTexture {
    pub image: Handle<Image>,
    pub cubemap_layout: CubemapLayout,
}
```

---

## Enum CapsuleUvProfile Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/enum.CapsuleUvProfile.html

**Contents:**
- Enum CapsuleUvProfile Copy item path
- Variants§
  - Aspect
  - Uniform
  - Fixed
- Trait Implementations§
  - impl Clone for CapsuleUvProfile
    - fn clone(&self) -> CapsuleUvProfile
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for CapsuleUvProfile

Manner in which UV coordinates are distributed vertically.

UV space is distributed by how much of the capsule consists of the hemispheres.

Hemispheres get UV space according to the ratio of latitudes to rings.

Upper third of the texture goes to the northern hemisphere, middle third to the cylinder and lower third to the southern one.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum CapsuleUvProfile {
    Aspect,
    Uniform,
    Fixed,
}
```

---

## Trait TextureFormatPixelInfo Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/trait.TextureFormatPixelInfo.html

**Contents:**
- Trait TextureFormatPixelInfo Copy item path
- Required Methods§
    - fn pixel_size(&self) -> Result<usize, TextureAccessError>
- Implementors§
  - impl TextureFormatPixelInfo for TextureFormat

Extends the wgpu TextureFormat with information about the pixel.

Returns the size of a pixel in bytes of the format. error with TextureAccessError::UnsupportedTextureFormat if the format is compressed.

**Examples:**

Example 1 (unknown):
```unknown
pub trait TextureFormatPixelInfo {
    // Required method
    fn pixel_size(&self) -> Result<usize, TextureAccessError>;
}
```

---

## Module render_resource Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/render_resource/index.html

**Contents:**
- Module render_resource Copy item path
- Modules§
- Structs§
- Enums§
- Constants§
- Statics§
- Traits§
- Functions§
- Type Aliases§
- Derive Macros§

bevy::renderModule render_resource Copy item pathSource Modules§binding_typesencaseresource_macrosStructs§BindGroupBind groups are responsible for binding render resources (e.g. buffers, textures, samplers) to a TrackedRenderPass. This makes them accessible in the pipeline (shaders) as uniforms.BindGroupDescriptorDescribes a group of bindings and the resources to be bound.BindGroupEntriesHelper for constructing bindgroups.BindGroupEntryAn element of a BindGroupDescriptor, consisting of a bindable resource and the slot to bind it to.BindGroupIdBindGroupLayoutBind group layouts define the interface of resources (e.g. buffers, textures, samplers) for a shader. The actual resource binding is done via a BindGroup.BindGroupLayoutDescriptorDescribes a BindGroupLayout.BindGroupLayoutEntriesBindGroupLayoutEntryDescribes a single binding inside a bind group.BindGroupLayoutEntryBuilderHelper for constructing bind group layouts.BindGroupLayoutIdBindingNumberThe index of the actual binding in the bind group.BindingResourcesA pair of binding index and binding resource, used as part of PreparedBindGroup and UnpreparedBindGroup.BindlessBufferDescriptorDescribes a bindless buffer.BindlessDescriptorInformation about the bindless resources in this object.BindlessIndexThe index in the bindless index table.BindlessIndexTableDescriptorDescribes the layout of the bindless index table, which maps bindless indices to indices within the binding arrays.BlasBottom Level Acceleration Structure (BLAS).BlasBuildEntryBuilds the given sets of geometry into the given Blas.BlasTriangleGeometryDefinition for a triangle geometry for a Bottom Level Acceleration Structure (BLAS).BlendComponentDescribes a blend component of a BlendState.BlendStateDescribe the blend state of a render pipeline, within ColorTargetState.BufferBufferAsyncErrorError occurred when trying to async map a buffer.BufferBindingDescribes the segment of a buffer to bind.BufferIdBufferInitDescriptorDescribes a Buffer when allocating.BufferSliceBufferUsagesDifferent ways that you can use a buffer.BufferVecLike RawBufferVec, but doesn’t require that the data type T be NoUninit.CachedComputePipelineIdIndex of a cached compute pipeline in a PipelineCache.CachedPipelineCachedRenderPipelineIdIndex of a cached render pipeline in a PipelineCache.ColorTargetStateDescribes the color state of a render pipeline.ColorWritesColor write mask. Disabled color channels will not be written to.CommandEncoderEncodes a series of GPU operations.Comp

*[Content truncated]*

---

## Struct ManualTextureViewHandle Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.ManualTextureViewHandle.html

**Contents:**
- Struct ManualTextureViewHandle Copy item path
- Tuple Fields§
- Trait Implementations§
  - impl Clone for ManualTextureViewHandle
    - fn clone(&self) -> ManualTextureViewHandle
    - fn clone_from(&mut self, source: &Self)
  - impl Component for ManualTextureViewHandlewhere ManualTextureViewHandle: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

A unique id that corresponds to a specific ManualTextureView in the ManualTextureViews collection.

See ManualTextureViews in bevy_camera for more details.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ManualTextureViewHandle(pub u32);
```

---

## Constant INDEX_BUFFER_ASSET_INDEX Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/constant.INDEX_BUFFER_ASSET_INDEX.html

**Contents:**
- Constant INDEX_BUFFER_ASSET_INDEX Copy item path

bevy::meshConstant INDEX_BUFFER_ASSET_INDEX Copy item pathSource pub const INDEX_BUFFER_ASSET_INDEX: u64 = 0; // 0u64

**Examples:**

Example 1 (javascript):
```javascript
pub const INDEX_BUFFER_ASSET_INDEX: u64 = 0; // 0u64
```

---

## Struct MaterialBindGroupSlot Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialBindGroupSlot.html

**Contents:**
- Struct MaterialBindGroupSlot Copy item path
- Tuple Fields§
- Methods from Deref<Target = u32>§
    - pub const MIN: u32 = 0u32
    - pub const MAX: u32 = 4_294_967_295u32
    - pub const BITS: u32 = 32u32
- Trait Implementations§
  - impl Clone for MaterialBindGroupSlot
    - fn clone(&self) -> MaterialBindGroupSlot
    - fn clone_from(&mut self, source: &Self)

The index of the slot containing material data within each material bind group.

In bindless mode, this slot is needed to locate the material data in each bind group, since multiple materials are packed into a single slab. In non-bindless mode, this slot is always 0.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialBindGroupSlot(pub u32);
```

---

## Enum HdrTextureLoaderError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.HdrTextureLoaderError.html

**Contents:**
- Enum HdrTextureLoaderError Copy item path
- Variants (Non-exhaustive)§
  - Io(Error)
  - Image(ImageError)
  - TextureAccess(TextureAccessError)
- Trait Implementations§
  - impl Debug for HdrTextureLoaderError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for HdrTextureLoaderError
    - fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[non_exhaustive]pub enum HdrTextureLoaderError {
    Io(Error),
    Image(ImageError),
    TextureAccess(TextureAccessError),
}
```

---

## Module experimental Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/experimental/index.html

**Contents:**
- Module experimental Copy item path
- Modules§

Experimental rendering features.

Experimental features are features with known problems, but are included nonetheless for testing purposes.

---

## Struct LateShadowPassNode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.LateShadowPassNode.html

**Contents:**
- Struct LateShadowPassNode Copy item path
- Trait Implementations§
  - impl Deref for LateShadowPassNode
    - type Target = ShadowPassNode
    - fn deref(&self) -> &<LateShadowPassNode as Deref>::Target
  - impl DerefMut for LateShadowPassNode
    - fn deref_mut(&mut self) -> &mut <LateShadowPassNode as Deref>::Target
  - impl FromWorld for LateShadowPassNode
    - fn from_world(world: &mut World) -> LateShadowPassNode
  - impl Node for LateShadowPassNode

The rendering node that renders meshes that became newly “visible” (so to speak) from a light this frame.

If occlusion culling for a light is disabled, then this node does nothing.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct LateShadowPassNode(/* private fields */);
```

---

## Module batching Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/batching/index.html

**Contents:**
- Module batching Copy item path
- Modules§
- Structs§
- Traits§
- Functions§

bevy::renderModule batching Copy item pathSource Modules§gpu_preprocessingBatching functionality when GPU preprocessing is in use.no_gpu_preprocessingBatching functionality when GPU preprocessing isn’t in use.Structs§NoAutomaticBatchingAdd this component to mesh entities to disable automatic batchingTraits§GetBatchDataA trait to support getting data used for batching draw commands via phase items.GetFullBatchDataA trait to support getting data used for batching draw commands via phase items.Functions§sort_binned_render_phaseSorts a render phase that uses bins.

---

## Struct MeshPipelineViewLayout Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshPipelineViewLayout.html

**Contents:**
- Struct MeshPipelineViewLayout Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for MeshPipelineViewLayout
    - fn clone(&self) -> MeshPipelineViewLayout
    - fn clone_from(&mut self, source: &Self)
- Auto Trait Implementations§
  - impl Freeze for MeshPipelineViewLayout
  - impl !RefUnwindSafe for MeshPipelineViewLayout
  - impl Send for MeshPipelineViewLayout

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshPipelineViewLayout {
    pub main_layout: BindGroupLayout,
    pub binding_array_layout: BindGroupLayout,
    pub empty_layout: BindGroupLayout,
    pub texture_count: usize,
}
```

---

## Struct SetMeshViewBindingArrayBindGroup Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.SetMeshViewBindingArrayBindGroup.html

**Contents:**
- Struct SetMeshViewBindingArrayBindGroup Copy item path
- Trait Implementations§
  - impl<P, const I: usize> RenderCommand<P> for SetMeshViewBindingArrayBindGroup<I>where P: PhaseItem,
    - type Param = ()
    - type ViewQuery = (&'static MeshViewBindGroup,)
    - type ItemQuery = ()
    - fn render<'w>( _item: &P, _: <<<SetMeshViewBindingArrayBindGroup<I> as RenderCommand<P>>::ViewQuery as QueryData>::ReadOnly as QueryData>::Item<'w, '_>, _entity: Option<()>, _: <<SetMeshViewBindingArrayBindGroup<I> as RenderCommand<P>>::Param as SystemParam>::Item<'w, '_>, pass: &mut TrackedRenderPass<'w>, ) -> RenderCommandResult
- Auto Trait Implementations§
  - impl<const I: usize> Freeze for SetMeshViewBindingArrayBindGroup<I>
  - impl<const I: usize> RefUnwindSafe for SetMeshViewBindingArrayBindGroup<I>

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (javascript):
```javascript
pub struct SetMeshViewBindingArrayBindGroup<const I: usize>;
```

---

## Module primitives Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/primitives/index.html

**Contents:**
- Module primitives Copy item path
- Structs§
- Enums§
- Constants§
- Traits§
- Functions§

bevy::cameraModule primitives Copy item pathSource Structs§AabbAn axis-aligned bounding box, defined by:CascadesFrustaCubeMapFaceCubemapFrustaFrustumA region of 3D space defined by the intersection of 6 HalfSpaces.HalfSpaceA region of 3D space, specifically an open set whose border is a bisecting 2D plane.SphereEnums§CubemapLayoutCubemap layout defines the order of images in a packed cubemap image.Constants§CUBE_MAP_FACESTraits§MeshAabbFunctions§face_index_to_name

---

## Struct PhysicalCameraParameters Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.PhysicalCameraParameters.html

**Contents:**
- Struct PhysicalCameraParameters Copy item path
- Fields§
- Implementations§
  - impl PhysicalCameraParameters
    - pub fn ev100(&self) -> f32
- Trait Implementations§
  - impl Clone for PhysicalCameraParameters
    - fn clone(&self) -> PhysicalCameraParameters
    - fn clone_from(&mut self, source: &Self)
  - impl Default for PhysicalCameraParameters

Parameters based on physical camera characteristics for calculating EV100 values for use with Exposure. This is also used for depth of field.

https://en.wikipedia.org/wiki/F-number

https://en.wikipedia.org/wiki/Shutter_speed

https://en.wikipedia.org/wiki/Film_speed

The height of the image sensor format in meters.

Focal length is derived from the FOV and this value. The default is 18.66mm, matching the Super 35 format, which is popular in cinema.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PhysicalCameraParameters {
    pub aperture_f_stops: f32,
    pub shutter_speed_s: f32,
    pub sensitivity_iso: f32,
    pub sensor_height: f32,
}
```

---

## Enum CameraOutputMode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/enum.CameraOutputMode.html

**Contents:**
- Enum CameraOutputMode Copy item path
- Variants§
  - Write
    - Fields
  - Skip
- Trait Implementations§
  - impl Clone for CameraOutputMode
    - fn clone(&self) -> CameraOutputMode
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for CameraOutputMode

Control how this Camera outputs once rendering is completed.

Writes the camera output to configured render target.

The blend state that will be used by the pipeline that writes the intermediate render textures to the final render target texture. If not set, the output will be written as-is, ignoring clear_color and the existing data in the final render target texture.

The clear color operation to perform on the final render target texture.

Skips writing the camera output to the configured render target. The output will remain in the Render Target’s “intermediate” textures, which a camera with a higher order should write to the render target using CameraOutputMode::Write. The “skip” mode can easily prevent render results from being displayed, or cause them to be lost. Only use this if you know what you are doing! In camera setups with multiple active cameras rendering to the same RenderTarget, the Skip mode can be used to remove unnecessary / redundant writes to the final output texture, removing unnecessary render passes.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum CameraOutputMode {
    Write {
        blend_state: Option<BlendState>,
        clear_color: ClearColorConfig,
    },
    Skip,
}
```

---

## Struct ShaderLoader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/shader/struct.ShaderLoader.html

**Contents:**
- Struct ShaderLoader Copy item path
- Trait Implementations§
  - impl AssetLoader for ShaderLoader
    - type Asset = Shader
    - type Settings = ShaderSettings
    - type Error = ShaderLoaderError
    - async fn load( &self, reader: &mut dyn Reader, settings: &<ShaderLoader as AssetLoader>::Settings, load_context: &mut LoadContext<'_>, ) -> Result<Shader, <ShaderLoader as AssetLoader>::Error>
    - fn extensions(&self) -> &[&str]
  - impl Default for ShaderLoader
    - fn default() -> ShaderLoader

Processes the asset in an asynchronous closure.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ShaderLoader;
```

---

## Struct MeshTag Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.MeshTag.html

**Contents:**
- Struct MeshTag Copy item path
- Tuple Fields§
- Methods from Deref<Target = u32>§
    - pub const MIN: u32 = 0u32
    - pub const MAX: u32 = 4_294_967_295u32
    - pub const BITS: u32 = 32u32
- Trait Implementations§
  - impl Clone for MeshTag
    - fn clone(&self) -> MeshTag
    - fn clone_from(&mut self, source: &Self)

A component that stores an arbitrary index used to identify the mesh instance when rendering.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshTag(pub u32);
```

---

## Struct Segment2dMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.Segment2dMeshBuilder.html

**Contents:**
- Struct Segment2dMeshBuilder Copy item path
- Fields§
- Implementations§
  - impl Segment2dMeshBuilder
    - pub const fn new(line: Segment2d) -> Segment2dMeshBuilder
- Trait Implementations§
  - impl MeshBuilder for Segment2dMeshBuilder
    - fn build(&self) -> Mesh
- Auto Trait Implementations§
  - impl Freeze for Segment2dMeshBuilder

A builder used for creating a Mesh with a Segment2d.

Creates a new Segment2dMeshBuilder from a given segment.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Segment2dMeshBuilder {
    pub segment: Segment2d,
}
```

---

## Trait VisitAssetDependencies Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/trait.VisitAssetDependencies.html

**Contents:**
- Trait VisitAssetDependencies Copy item path
- Required Methods§
    - fn visit_dependencies(&self, visit: &mut impl FnMut(UntypedAssetId))
- Dyn Compatibility§
- Implementations on Foreign Types§
  - impl VisitAssetDependencies for Option<UntypedHandle>
    - fn visit_dependencies(&self, visit: &mut impl FnMut(UntypedAssetId))
  - impl VisitAssetDependencies for ()
    - fn visit_dependencies(&self, _visit: &mut impl FnMut(UntypedAssetId))
  - impl<A> VisitAssetDependencies for Option<Handle<A>>where A: Asset,

This trait defines how to visit the dependencies of an asset. For example, a 3D model might require both textures and meshes to be loaded.

Note that this trait is automatically implemented when deriving Asset.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait VisitAssetDependencies {
    // Required method
    fn visit_dependencies(&self, visit: &mut impl FnMut(UntypedAssetId));
}
```

---

## Struct EarlyShadowPassNode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.EarlyShadowPassNode.html

**Contents:**
- Struct EarlyShadowPassNode Copy item path
- Trait Implementations§
  - impl Deref for EarlyShadowPassNode
    - type Target = ShadowPassNode
    - fn deref(&self) -> &<EarlyShadowPassNode as Deref>::Target
  - impl DerefMut for EarlyShadowPassNode
    - fn deref_mut(&mut self) -> &mut <EarlyShadowPassNode as Deref>::Target
  - impl FromWorld for EarlyShadowPassNode
    - fn from_world(world: &mut World) -> EarlyShadowPassNode
  - impl Node for EarlyShadowPassNode

The rendering node that renders meshes that were “visible” (so to speak) from a light last frame.

If occlusion culling for a light is disabled, then this node simply renders all meshes in range of the light.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct EarlyShadowPassNode(/* private fields */);
```

---

## Struct MeshLayouts Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshLayouts.html

**Contents:**
- Struct MeshLayouts Copy item path
- Fields§
- Implementations§
  - impl MeshLayouts
    - pub fn new( render_device: &RenderDevice, render_adapter: &RenderAdapter, ) -> MeshLayouts
    - pub fn model_only( &self, render_device: &RenderDevice, model: &BindingResource<'_>, ) -> BindGroup
    - pub fn lightmapped( &self, render_device: &RenderDevice, model: &BindingResource<'_>, lightmap_slab: &LightmapSlab, bindless_lightmaps: bool, ) -> BindGroup
    - pub fn skinned( &self, render_device: &RenderDevice, model: &BindingResource<'_>, current_skin: &Buffer, ) -> BindGroup
    - pub fn skinned_motion( &self, render_device: &RenderDevice, model: &BindingResource<'_>, current_skin: &Buffer, prev_skin: &Buffer, ) -> BindGroup
    - pub fn morphed( &self, render_device: &RenderDevice, model: &BindingResource<'_>, current_weights: &Buffer, targets: &TextureView, ) -> BindGroup

All possible BindGroupLayouts in bevy’s default mesh shader (mesh.wgsl).

The mesh model uniform (transform) and nothing else.

Includes the lightmap texture and uniform.

Also includes the uniform for skinning

Like MeshLayouts::skinned, but includes slots for the previous frame’s joint matrices, so that we can compute motion vectors.

Also includes the uniform and MorphAttributes for morph targets.

Like MeshLayouts::morphed, but includes a slot for the previous frame’s morph weights, so that we can compute motion vectors.

Also includes both uniforms for skinning and morph targets, also the morph target MorphAttributes binding.

Like MeshLayouts::morphed_skinned, but includes slots for the previous frame’s joint matrices and morph weights, so that we can compute motion vectors.

Prepare the layouts used by the default bevy Mesh.

Creates the bind group for skinned meshes with no morph targets.

Creates the bind group for skinned meshes with no morph targets, with the infrastructure to compute motion vectors.

current_skin is the buffer of joint matrices for this frame; prev_skin is the buffer for the previous frame. The latter is used for motion vector computation. If there is no such applicable buffer, current_skin and prev_skin will reference the same buffer.

Creates the bind group for meshes with no skins but morph targets.

Creates the bind group for meshes with no skins but morph targets, in addition to the infrastructure to compute motion vectors.

current_weights is the buffer of morph weights for this frame; prev_weights is the buffer for the previous frame. The latter is used for motion vector computation. If there is no such applicable buffer, current_weights and prev_weights will reference the same buffer.

Creates the bind group for meshes with skins and morph targets.

Creates the bind group for meshes with skins and morph targets, in addition to the infrastructure to compute motion vectors.

See the documentation for MeshLayouts::skinned_motion and MeshLayouts::morphed_motion above for more information about the current_skin, prev_skin, current_weights, and prev_weights buffers.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshLayouts {
    pub model_only: BindGroupLayout,
    pub lightmapped: BindGroupLayout,
    pub skinned: BindGroupLayout,
    pub skinned_motion: BindGroupLayout,
    pub morphed: BindGroupLayout,
    pub morphed_motion: BindGroupLayout,
    pub morphed_skinned: BindGroupLayout,
    pub morphed_skinned_motion: BindGroupLayout,
}
```

---

## Struct RenderAssetUsages Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.RenderAssetUsages.html

**Contents:**
- Struct RenderAssetUsages Copy item path
  - §Platform-specific
- Implementations§
  - impl RenderAssetUsages
    - pub const MAIN_WORLD: RenderAssetUsages
    - pub const RENDER_WORLD: RenderAssetUsages
  - impl RenderAssetUsages
    - pub const fn empty() -> RenderAssetUsages
    - pub const fn all() -> RenderAssetUsages
      - Examples found in repository?

Defines where the asset will be used.

If an asset is set to the RENDER_WORLD but not the MAIN_WORLD, the asset will be unloaded from the asset server once it’s been extracted and prepared in the render world.

Unloading the asset saves on memory, as for most cases it is no longer necessary to keep it in RAM once it’s been uploaded to the GPU’s VRAM. However, this means you can no longer access the asset from the CPU (via the Assets<T> resource) once unloaded (without re-loading it).

If you never need access to the asset from the CPU past the first frame it’s loaded on, or only need very infrequent access, then set this to RENDER_WORLD. Otherwise, set this to RENDER_WORLD | MAIN_WORLD.

If you have an asset that doesn’t actually need to end up in the render world, like an Image that will be decoded into another Image asset, use MAIN_WORLD only.

On Wasm, it is not possible for now to free reserved memory. To control memory usage, load assets in sequence and unload one before loading the next. See this discussion about memory management for more details.

The bit flag for the main world.

The bit flag for the render world.

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

The intersection of a source flags value with the complemen

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderAssetUsages(/* private fields */);
```

Example 2 (javascript):
```javascript
48fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
49    let bird_left = Bird::Normal;
50    let bird_right = Bird::Normal;
51    commands.spawn(Camera2d);
52
53    let texture_left = asset_server.load_with_settings(
54        bird_left.get_texture_path(),
55        // `RenderAssetUsages::all()` is already the default, so the line below could be omitted.
56        // It's helpful to know it exists, however.
57        //
58        // `RenderAssetUsages` tell Bevy whether to keep the data around:
59        //   - for the GPU (`RenderAssetUsages::RENDER_WORLD`),
60        //   -
...
```

Example 3 (javascript):
```javascript
25fn test(
26    mut commands: Commands,
27    mut images: ResMut<Assets<Image>>,
28    mut meshes: ResMut<Assets<Mesh>>,
29    mut materials: ResMut<Assets<StandardMaterial>>,
30) {
31    // Spawn a UI camera
32    commands.spawn(Camera3d::default());
33
34    // Set up an texture for the 3D camera to render to.
35    // The size of the texture will be based on the viewport's ui size.
36    let mut image = Image::new_uninit(
37        default(),
38        TextureDimension::D2,
39        TextureFormat::Bgra8UnormSrgb,
40        RenderAssetUsages::all(),
41    );
42    image.texture_descriptor.
...
```

Example 4 (javascript):
```javascript
48fn setup(
49    mut commands: Commands,
50    asset_server: Res<AssetServer>,
51    mut materials: ResMut<Assets<StandardMaterial>>,
52) {
53    let left_shape = Shape::Cube;
54    let right_shape = Shape::Cube;
55
56    // In normal use, you can call `asset_server.load`, however see below for an explanation of
57    // `RenderAssetUsages`.
58    let left_shape_model = asset_server.load_with_settings(
59        GltfAssetLabel::Primitive {
60            mesh: 0,
61            // This field stores an index to this primitive in its parent mesh. In this case, we
62            // want the first o
...
```

---

## Trait Material2d Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/sprite_render/trait.Material2d.html

**Contents:**
- Trait Material2d Copy item path
- §Example
- Provided Methods§
    - fn vertex_shader() -> ShaderRef
    - fn fragment_shader() -> ShaderRef
    - fn depth_bias(&self) -> f32
    - fn alpha_mode(&self) -> AlphaMode2d
    - fn specialize( descriptor: &mut RenderPipelineDescriptor, layout: &MeshVertexBufferLayoutRef, key: Material2dKey<Self>, ) -> Result<(), SpecializedMeshPipelineError>
- Dyn Compatibility§
- Implementors§

Materials are used alongside Material2dPlugin, Mesh2d, and MeshMaterial2d to spawn entities that are rendered with a specific Material2d type. They serve as an easy to use high level way to render Mesh2d entities with custom shader logic.

Materials must implement AsBindGroup to define how data will be transferred to the GPU and bound in shaders. AsBindGroup can be derived, which makes generating bindings straightforward. See the AsBindGroup docs for details.

Here is a simple Material2d implementation. The AsBindGroup derive has many features. To see what else is available, check out the AsBindGroup documentation.

In WGSL shaders, the material’s binding would look like this:

Returns this material’s vertex shader. If ShaderRef::Default is returned, the default mesh vertex shader will be used.

Returns this material’s fragment shader. If ShaderRef::Default is returned, the default mesh fragment shader will be used.

Add a bias to the view depth of the mesh which can be used to force a specific render order.

Customizes the default RenderPipelineDescriptor.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait Material2d:
    Sized
    + AsBindGroup
    + Asset
    + Clone {
    // Provided methods
    fn vertex_shader() -> ShaderRef { ... }
    fn fragment_shader() -> ShaderRef { ... }
    fn depth_bias(&self) -> f32 { ... }
    fn alpha_mode(&self) -> AlphaMode2d { ... }
    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> { ... }
}
```

Example 2 (unknown):
```unknown
#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct CustomMaterial {
    // Uniform bindings must implement `ShaderType`, which will be used to convert the value to
    // its shader-compatible equivalent. Most core math types already implement `ShaderType`.
    #[uniform(0)]
    color: LinearRgba,
    // Images can be bound as textures in shaders. If the Image's sampler is also needed, just
    // add the sampler attribute with a different binding index.
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
}

// All functions on `Material2d` have default impls. Y
...
```

Example 3 (wgsl):
```wgsl
struct CustomMaterial {
    color: vec4<f32>,
}

@group(2) @binding(0) var<uniform> material: CustomMaterial;
@group(2) @binding(1) var color_texture: texture_2d<f32>;
@group(2) @binding(2) var color_sampler: sampler;
```

---

## Enum OpaqueRendererMethod Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/enum.OpaqueRendererMethod.html

**Contents:**
- Enum OpaqueRendererMethod Copy item path
- Variants§
  - Forward
  - Deferred
  - Auto
- Trait Implementations§
  - impl Clone for OpaqueRendererMethod
    - fn clone(&self) -> OpaqueRendererMethod
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for OpaqueRendererMethod

Render method used for opaque materials.

The forward rendering main pass draws each mesh entity and shades it according to its corresponding material and the lights that affect it. Some render features like Screen Space Ambient Occlusion require running depth and normal prepasses, that are ‘deferred’-like prepasses over all mesh entities to populate depth and normal textures. This means that when using render features that require running prepasses, multiple passes over all visible geometry are required. This can be slow if there is a lot of geometry that cannot be batched into few draws.

Deferred rendering runs a prepass to gather not only geometric information like depth and normals, but also all the material properties like base color, emissive color, reflectance, metalness, etc, and writes them into a deferred ‘g-buffer’ texture. The deferred main pass is then a fullscreen pass that reads data from these textures and executes shading. This allows for one pass over geometry, but is at the cost of not being able to use MSAA, and has heavier bandwidth usage which can be unsuitable for low end mobile or other bandwidth-constrained devices.

If a material indicates OpaqueRendererMethod::Auto, DefaultOpaqueRendererMethod will be used.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum OpaqueRendererMethod {
    Forward,
    Deferred,
    Auto,
}
```

---

## Crate ui_render Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/ui_render/index.html

**Contents:**
- Crate ui_render Copy item path
- Modules§
- Structs§
- Enums§
- Functions§
- Type Aliases§

Provides rendering functionality for bevy_ui.

---

## Struct MaterialVertexShader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialVertexShader.html

**Contents:**
- Struct MaterialVertexShader Copy item path
- Trait Implementations§
  - impl Clone for MaterialVertexShader
    - fn clone(&self) -> MaterialVertexShader
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for MaterialVertexShader
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for MaterialVertexShader
    - fn default() -> MaterialVertexShader
  - impl Hash for MaterialVertexShader

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialVertexShader;
```

---

## Struct ErasedMaterialKeyVTable Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ErasedMaterialKeyVTable.html

**Contents:**
- Struct ErasedMaterialKeyVTable Copy item path
- Trait Implementations§
  - impl Debug for ErasedMaterialKeyVTable
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
- Auto Trait Implementations§
  - impl Freeze for ErasedMaterialKeyVTable
  - impl RefUnwindSafe for ErasedMaterialKeyVTable
  - impl Send for ErasedMaterialKeyVTable
  - impl Sync for ErasedMaterialKeyVTable
  - impl Unpin for ErasedMaterialKeyVTable

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ErasedMaterialKeyVTable { /* private fields */ }
```

---

## Enum PerimeterSegment Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/enum.PerimeterSegment.html

**Contents:**
- Enum PerimeterSegment Copy item path
- Variants§
  - Smooth
    - Fields
  - Flat
    - Fields
- Auto Trait Implementations§
  - impl Freeze for PerimeterSegment
  - impl RefUnwindSafe for PerimeterSegment
  - impl Send for PerimeterSegment

A type representing a segment of the perimeter of an extrudable mesh.

This segment of the perimeter will be shaded smooth.

This has the effect of rendering the segment’s faces with softened edges, so it is appropriate for curved shapes.

The normals for the vertices that are part of this segment will be calculated based on the positions of their neighbors. Each normal is interpolated between the normals of the two line segments connecting it with its neighbors. Closer vertices have a stronger effect on the normal than more distant ones.

Since the vertices corresponding to the first and last indices do not have two neighboring vertices, their normals must be provided manually.

The normal of the first vertex.

The normal of the last vertex.

A list of indices representing this segment of the perimeter of the mesh.

The indices must be ordered such that the outside of the mesh is to the right when walking along the vertices of the mesh in the order provided by the indices.

For geometry to be rendered, you must provide at least two indices.

This segment of the perimeter will be shaded flat.

This has the effect of rendering the segment’s faces with hard edges.

A list of indices representing this segment of the perimeter of the mesh.

The indices must be ordered such that the outside of the mesh is to the right when walking along the vertices of the mesh in the order provided by indices.

For geometry to be rendered, you must provide at least two indices.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum PerimeterSegment {
    Smooth {
        first_normal: Vec2,
        last_normal: Vec2,
        indices: Vec<u32>,
    },
    Flat {
        indices: Vec<u32>,
    },
}
```

---

## Struct MeshUniform Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshUniform.html

**Contents:**
- Struct MeshUniform Copy item path
- Fields§
- Implementations§
  - impl MeshUniform
    - pub fn new( mesh_transforms: &MeshTransforms, first_vertex_index: u32, material_bind_group_slot: MaterialBindGroupSlot, maybe_lightmap: Option<(LightmapSlotIndex, Rect)>, current_skin_index: Option<u32>, tag: Option<u32>, ) -> MeshUniform
      - Examples found in repository?
- Trait Implementations§
  - impl Clone for MeshUniform
    - fn clone(&self) -> MeshUniform
    - fn clone_from(&mut self, source: &Self)

The index of this mesh’s first vertex in the vertex buffer.

Multiple meshes can be packed into a single vertex buffer (see MeshAllocator). This value stores the offset of the first vertex in this mesh in that buffer.

The current skin index, or u32::MAX if there’s no skin.

The material and lightmap indices, packed into 32 bits.

Low 16 bits: index of the material inside the bind group data. High 16 bits: index of the lightmap in the binding array.

User supplied tag to identify this mesh instance.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshUniform {
    pub world_from_local: [Vec4; 3],
    pub previous_world_from_local: [Vec4; 3],
    pub local_from_world_transpose_a: [Vec4; 2],
    pub local_from_world_transpose_b: f32,
    pub flags: u32,
    pub lightmap_uv_rect: UVec2,
    pub first_vertex_index: u32,
    pub current_skin_index: u32,
    pub material_and_lightmap_bind_group_slot: u32,
    pub tag: u32,
    pub pad: u32,
}
```

Example 2 (javascript):
```javascript
405    fn get_binned_batch_data(
406        (mesh_instances, _render_assets, mesh_allocator): &SystemParamItem<Self::Param>,
407        main_entity: MainEntity,
408    ) -> Option<Self::BufferData> {
409        let RenderMeshInstances::CpuBuilding(ref mesh_instances) = **mesh_instances else {
410            error!(
411                "`get_binned_batch_data` should never be called in GPU mesh uniform building mode"
412            );
413            return None;
414        };
415        let mesh_instance = mesh_instances.get(&main_entity)?;
416        let first_vertex_index =
417            matc
...
```

---

## Struct RenderMeshInstanceFlags Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.RenderMeshInstanceFlags.html

**Contents:**
- Struct RenderMeshInstanceFlags Copy item path
- Implementations§
  - impl RenderMeshInstanceFlags
    - pub const SHADOW_CASTER: RenderMeshInstanceFlags
    - pub const AUTOMATIC_BATCHING: RenderMeshInstanceFlags
    - pub const HAS_PREVIOUS_TRANSFORM: RenderMeshInstanceFlags
    - pub const HAS_PREVIOUS_SKIN: RenderMeshInstanceFlags
    - pub const HAS_PREVIOUS_MORPH: RenderMeshInstanceFlags
  - impl RenderMeshInstanceFlags
    - pub const fn empty() -> RenderMeshInstanceFlags

Various useful flags for [RenderMeshInstance]s.

The mesh casts shadows.

The mesh can participate in automatic batching.

The mesh had a transform last frame and so is eligible for motion vector computation.

The mesh had a skin last frame and so that skin should be taken into account for motion vector computation.

The mesh had morph targets last frame and so they should be taken into account for motion vector computation.

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

The bitwise and (

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderMeshInstanceFlags(/* private fields */);
```

---

## Struct FullscreenShader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/core_pipeline/struct.FullscreenShader.html

**Contents:**
- Struct FullscreenShader Copy item path
- Implementations§
  - impl FullscreenShader
    - pub fn shader(&self) -> Handle<Shader>
    - pub fn to_vertex_state(&self) -> VertexState
      - Examples found in repository?
- Trait Implementations§
  - impl Clone for FullscreenShader
    - fn clone(&self) -> FullscreenShader
    - fn clone_from(&mut self, source: &Self)

A shader that renders to the whole screen. Useful for post-processing.

Gets the raw shader handle.

Creates a VertexState that uses the FullscreenShader to output a

from the vertex shader. The draw call should render one triangle: render_pass.draw(0..3, 0..1);

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct FullscreenShader(/* private fields */);
```

Example 2 (wgsl):
```wgsl
struct FullscreenVertexOutput {
    @builtin(position)
    position: vec4<f32>;
    @location(0)
    uv: vec2<f32>;
};
```

Example 3 (javascript):
```javascript
230fn init_post_process_pipeline(
231    mut commands: Commands,
232    render_device: Res<RenderDevice>,
233    asset_server: Res<AssetServer>,
234    fullscreen_shader: Res<FullscreenShader>,
235    pipeline_cache: Res<PipelineCache>,
236) {
237    // We need to define the bind group layout used for our pipeline
238    let layout = render_device.create_bind_group_layout(
239        "post_process_bind_group_layout",
240        &BindGroupLayoutEntries::sequential(
241            // The layout entries will only be visible in the fragment stage
242            ShaderStages::FRAGMENT,
243         
...
```

---

## Module texture Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/texture/index.html

**Contents:**
- Module texture Copy item path
- Structs§
- Functions§

bevy::renderModule texture Copy item pathSource Structs§CachedTextureA cached GPU Texture with corresponding TextureView.ColorAttachmentA wrapper for a CachedTexture that is used as a RenderPassColorAttachment.DefaultImageSamplerA rendering resource for the default image sampler which is set during renderer initialization.DepthAttachmentA wrapper for a TextureView that is used as a depth-only RenderPassDepthStencilAttachment.FallbackImageA RenderApp resource that contains the default “fallback image”, which can be used in situations where an image was not explicitly defined. The most common use case is AsBindGroup implementations (such as materials) that support optional textures.FallbackImageCubemapA RenderApp resource that contains a “cubemap fallback image”, which can be used in situations where an image was not explicitly defined. The most common use case is AsBindGroup implementations (such as materials) that support optional textures.FallbackImageFormatMsaaCacheA Cache of fallback textures that uses the sample count and TextureFormat as a keyFallbackImageMsaaFallbackImageZeroA RenderApp resource that contains a zero-filled “fallback image”, which can be used in place of FallbackImage, when a fully transparent or black fallback is required instead of fully opaque white.GpuImageThe GPU-representation of an Image. Consists of the Texture, its TextureView and the corresponding Sampler, and the texture’s size.ManualTextureViewA manually managed TextureView for use as a bevy_camera::RenderTarget.ManualTextureViewsResource that stores manually managed ManualTextureViews for use as a RenderTarget. This type dereferences to a HashMap<ManualTextureViewHandle, ManualTextureView>. To add a new texture view, pick a new ManualTextureViewHandle and insert it into the map. Then, to render to the view, set a Cameras target to RenderTarget::TextureView(handle).OutputColorAttachmentA wrapper for a TextureView that is used as a RenderPassColorAttachment for a view target’s final output texture.TextureCacheThis resource caches textures that are created repeatedly in the rendering process and are only required for one frame.TexturePluginFunctions§update_texture_cache_systemUpdates the TextureCache to only retains recently used textures.

---

## Struct RenderCascadesVisibleEntities Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.RenderCascadesVisibleEntities.html

**Contents:**
- Struct RenderCascadesVisibleEntities Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for RenderCascadesVisibleEntities
    - fn clone(&self) -> RenderCascadesVisibleEntities
    - fn clone_from(&mut self, source: &Self)
  - impl Component for RenderCascadesVisibleEntitieswhere RenderCascadesVisibleEntities: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

Map of view entity to the visible entities for each cascade frustum.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderCascadesVisibleEntities {
    pub entities: EntityHashMap<Vec<RenderVisibleMeshEntities>>,
}
```

---

## Struct StandardMaterial Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.StandardMaterial.html

**Contents:**
- Struct StandardMaterial Copy item path
- Fields§
    - §Notes
    - §Performance
  - §Notes
  - §Usage
    - §Limitations
    - §Performance
- Implementations§
  - impl StandardMaterial

A material with “standard” properties used in PBR lighting. Standard property values with pictures here: https://google.github.io/filament/notes/material_properties.html.

May be created directly from a Color or an Image.

The color of the surface of the material before lighting.

Doubles as diffuse albedo for non-metallic, specular for metallic and a mix for everything in between. If used together with a base_color_texture, this is factored into the final base color as base_color * base_color_texture_value.

Defaults to Color::WHITE.

The UV channel to use for the StandardMaterial::base_color_texture.

Defaults to UvChannel::Uv0.

The texture component of the material’s color before lighting. The actual pre-lighting color is base_color * this_texture.

See base_color for details.

You should set base_color to Color::WHITE (the default) if you want the texture to show as-is.

Setting base_color to something else than white will tint the texture. For example, setting base_color to pure red will tint the texture red.

Color the material “emits” to the camera.

This is typically used for monitor screens or LED lights. Anything that can be visible even in darkness.

The emissive color is added to what would otherwise be the material’s visible color. This means that for a light emissive value, in darkness, you will mostly see the emissive component.

The default emissive color is LinearRgba::BLACK, which doesn’t add anything to the material color.

Emissive strength is controlled by the value of the color channels, while the hue is controlled by their relative values.

As a result, channel values for emissive colors can exceed 1.0. For instance, a base_color of LinearRgba::rgb(1.0, 0.0, 0.0) represents the brightest red for objects that reflect light, but an emissive color like LinearRgba::rgb(1000.0, 0.0, 0.0) can be used to create intensely bright red emissive effects.

This results in a final luminance value when multiplied by the value of the greyscale emissive texture (which ranges from 0 for black to 1 for white). Luminance is a measure of the amount of light emitted per unit area, and can be thought of as the “brightness” of the effect. In Bevy, we treat these luminance values as the physical units of cd/m², aka nits.

Increasing the emissive strength of the color will impact visual effects like bloom, but it’s important to note that an emissive material won’t typically light up surrounding areas like a light source, it just adds a value to the color see

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct StandardMaterial {Show 60 fields
    pub base_color: Color,
    pub base_color_channel: UvChannel,
    pub base_color_texture: Option<Handle<Image>>,
    pub emissive: LinearRgba,
    pub emissive_exposure_weight: f32,
    pub emissive_channel: UvChannel,
    pub emissive_texture: Option<Handle<Image>>,
    pub perceptual_roughness: f32,
    pub metallic: f32,
    pub metallic_roughness_channel: UvChannel,
    pub metallic_roughness_texture: Option<Handle<Image>>,
    pub reflectance: f32,
    pub specular_tint: Color,
    pub diffuse_transmission: f32,
    pub diffuse_transmission_
...
```

Example 2 (javascript):
```javascript
fn load_normal_map(asset_server: Res<AssetServer>) {
    let normal_handle: Handle<Image> = asset_server.load_with_settings(
        "textures/parallax_example/cube_normal.png",
        // The normal map texture is in linear color space. Lighting won't look correct
        // if `is_srgb` is `true`, which is the default.
        |settings: &mut ImageLoaderSettings| settings.is_srgb = false,
    );
}
```

---

## Struct MaterialBindlessSlab Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialBindlessSlab.html

**Contents:**
- Struct MaterialBindlessSlab Copy item path
- Auto Trait Implementations§
  - impl Freeze for MaterialBindlessSlab
  - impl !RefUnwindSafe for MaterialBindlessSlab
  - impl Send for MaterialBindlessSlab
  - impl Sync for MaterialBindlessSlab
  - impl Unpin for MaterialBindlessSlab
  - impl !UnwindSafe for MaterialBindlessSlab
- Blanket Implementations§
  - impl<T> Any for Twhere T: 'static + ?Sized,

A single bind group and the bookkeeping necessary to allocate into it.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialBindlessSlab { /* private fields */ }
```

---

## Struct MeshletDeferredFragmentShader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshletDeferredFragmentShader.html

**Contents:**
- Struct MeshletDeferredFragmentShader Copy item path
- Trait Implementations§
  - impl Clone for MeshletDeferredFragmentShader
    - fn clone(&self) -> MeshletDeferredFragmentShader
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for MeshletDeferredFragmentShader
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for MeshletDeferredFragmentShader
    - fn default() -> MeshletDeferredFragmentShader
  - impl Hash for MeshletDeferredFragmentShader

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshletDeferredFragmentShader;
```

---

## Enum ShaderLoaderError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/shader/enum.ShaderLoaderError.html

**Contents:**
- Enum ShaderLoaderError Copy item path
- Variants (Non-exhaustive)§
  - Io(Error)
  - Parse(FromUtf8Error)
- Trait Implementations§
  - impl Debug for ShaderLoaderError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for ShaderLoaderError
    - fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>
  - impl Error for ShaderLoaderError

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[non_exhaustive]pub enum ShaderLoaderError {
    Io(Error),
    Parse(FromUtf8Error),
}
```

---

## Struct MaterialBindGroupIndex Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialBindGroupIndex.html

**Contents:**
- Struct MaterialBindGroupIndex Copy item path
- Tuple Fields§
- Methods from Deref<Target = u32>§
    - pub const MIN: u32 = 0u32
    - pub const MAX: u32 = 4_294_967_295u32
    - pub const BITS: u32 = 32u32
- Trait Implementations§
  - impl Clone for MaterialBindGroupIndex
    - fn clone(&self) -> MaterialBindGroupIndex
    - fn clone_from(&mut self, source: &Self)

The index of each material bind group.

In bindless mode, each bind group contains multiple materials. In non-bindless mode, each bind group contains only one material.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialBindGroupIndex(pub u32);
```

---

## Type Alias TextureAtlasBuilderResult Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/type.TextureAtlasBuilderResult.html

**Contents:**
- Type Alias TextureAtlasBuilderResult Copy item path
- Aliased Type§
- Variants§
  - Ok(T)
  - Err(TextureAtlasBuilderError)

Contains the success value

Contains the error value

**Examples:**

Example 1 (unknown):
```unknown
pub type TextureAtlasBuilderResult<T> = Result<T, TextureAtlasBuilderError>;
```

Example 2 (unknown):
```unknown
pub enum TextureAtlasBuilderResult<T> {
    Ok(T),
    Err(TextureAtlasBuilderError),
}
```

---

## Struct SkinnedMeshInverseBindposes Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/skinning/struct.SkinnedMeshInverseBindposes.html

**Contents:**
- Struct SkinnedMeshInverseBindposes Copy item path
- Methods from Deref<Target = [Mat4]>§
    - pub fn len(&self) -> usize
      - §Examples
    - pub fn is_empty(&self) -> bool
      - §Examples
    - pub fn first(&self) -> Option<&T>
      - §Examples
    - pub fn split_first(&self) -> Option<(&T, &[T])>
      - §Examples

Returns the number of elements in the slice.

Returns true if the slice has a length of 0.

Returns the first element of the slice, or None if it is empty.

Returns the first and all the rest of the elements of the slice, or None if it is empty.

Returns the last and all the rest of the elements of the slice, or None if it is empty.

Returns the last element of the slice, or None if it is empty.

Returns an array reference to the first N items in the slice.

If the slice is not at least N in length, this will return None.

Returns an array reference to the first N items in the slice and the remaining slice.

If the slice is not at least N in length, this will return None.

Returns an array reference to the last N items in the slice and the remaining slice.

If the slice is not at least N in length, this will return None.

Returns an array reference to the last N items in the slice.

If the slice is not at least N in length, this will return None.

Returns a reference to an element or subslice depending on the type of index.

Returns a reference to an element or subslice, without doing bounds checking.

For a safe alternative see get.

Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used.

You can think of this like .get(index).unwrap_unchecked(). It’s UB to call .get_unchecked(len), even if you immediately convert to a pointer. And it’s UB to call .get_unchecked(..len + 1), .get_unchecked(..=len), or similar.

Returns a raw pointer to the slice’s buffer.

The caller must ensure that the slice outlives the pointer this function returns, or else it will end up dangling.

The caller must also ensure that the memory the pointer (non-transitively) points to is never written to (except inside an UnsafeCell) using this pointer or any pointer derived from it. If you need to mutate the contents of the slice, use as_mut_ptr.

Modifying the container referenced by this slice may cause its buffer to be reallocated, which would also make any pointers to it invalid.

Returns the two raw pointers spanning the slice.

The returned range is half-open, which means that the end pointer points one past the last element of the slice. This way, an empty slice is represented by two equal pointers, and the difference between the two pointers represents the size of the slice.

See as_ptr for warnings on using these pointers. The end pointer requires extra caution, as it does not point to a valid element in the slice.

Th

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct SkinnedMeshInverseBindposes(/* private fields */);
```

Example 2 (javascript):
```javascript
let a = [1, 2, 3];
assert_eq!(a.len(), 3);
```

Example 3 (javascript):
```javascript
let a = [1, 2, 3];
assert!(!a.is_empty());

let b: &[i32] = &[];
assert!(b.is_empty());
```

Example 4 (javascript):
```javascript
let v = [10, 40, 30];
assert_eq!(Some(&10), v.first());

let w: &[i32] = &[];
assert_eq!(None, w.first());
```

---

## Struct CircularSegmentMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.CircularSegmentMeshBuilder.html

**Contents:**
- Struct CircularSegmentMeshBuilder Copy item path
- Fields§
- Implementations§
  - impl CircularSegmentMeshBuilder
    - pub fn new(segment: CircularSegment) -> CircularSegmentMeshBuilder
      - Examples found in repository?
    - pub const fn resolution(self, resolution: u32) -> CircularSegmentMeshBuilder
    - pub const fn uv_mode( self, uv_mode: CircularMeshUvMode, ) -> CircularSegmentMeshBuilder
      - Examples found in repository?
- Trait Implementations§

A builder used for creating a Mesh with a CircularSegment shape.

The resulting mesh will have a UV-map such that the center of the circle is at the center of the texture.

The number of vertices used for the arc portion of the segment mesh. The default is 32.

Creates a new CircularSegmentMeshBuilder from a given segment

Sets the number of vertices used for the segment mesh.

Sets the uv mode used for the segment mesh

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CircularSegmentMeshBuilder {
    pub segment: CircularSegment,
    pub resolution: u32,
    pub uv_mode: CircularMeshUvMode,
}
```

Example 2 (javascript):
```javascript
34fn setup(
35    mut commands: Commands,
36    asset_server: Res<AssetServer>,
37    mut meshes: ResMut<Assets<Mesh>>,
38    mut materials: ResMut<Assets<ColorMaterial>>,
39) {
40    let material = materials.add(asset_server.load("branding/icon.png"));
41
42    commands.spawn((
43        Camera2d,
44        Camera {
45            clear_color: ClearColorConfig::Custom(GRAY.into()),
46            ..default()
47        },
48    ));
49
50    const NUM_SLICES: i32 = 8;
51    const SPACING_X: f32 = 100.0;
52    const OFFSET_X: f32 = SPACING_X * (NUM_SLICES - 1) as f32 / 2.0;
53
54    // This draws 
...
```

Example 3 (javascript):
```javascript
34fn setup(
35    mut commands: Commands,
36    asset_server: Res<AssetServer>,
37    mut meshes: ResMut<Assets<Mesh>>,
38    mut materials: ResMut<Assets<ColorMaterial>>,
39) {
40    let material = materials.add(asset_server.load("branding/icon.png"));
41
42    commands.spawn((
43        Camera2d,
44        Camera {
45            clear_color: ClearColorConfig::Custom(GRAY.into()),
46            ..default()
47        },
48    ));
49
50    const NUM_SLICES: i32 = 8;
51    const SPACING_X: f32 = 100.0;
52    const OFFSET_X: f32 = SPACING_X * (NUM_SLICES - 1) as f32 / 2.0;
53
54    // This draws 
...
```

---

## Struct DeferredVertexShader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.DeferredVertexShader.html

**Contents:**
- Struct DeferredVertexShader Copy item path
- Trait Implementations§
  - impl Clone for DeferredVertexShader
    - fn clone(&self) -> DeferredVertexShader
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for DeferredVertexShader
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for DeferredVertexShader
    - fn default() -> DeferredVertexShader
  - impl Hash for DeferredVertexShader

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DeferredVertexShader;
```

---

## Struct MeshPipelineViewLayoutKey Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshPipelineViewLayoutKey.html

**Contents:**
- Struct MeshPipelineViewLayoutKey Copy item path
- Implementations§
  - impl MeshPipelineViewLayoutKey
    - pub const MULTISAMPLED: MeshPipelineViewLayoutKey
    - pub const DEPTH_PREPASS: MeshPipelineViewLayoutKey
    - pub const NORMAL_PREPASS: MeshPipelineViewLayoutKey
    - pub const MOTION_VECTOR_PREPASS: MeshPipelineViewLayoutKey
    - pub const DEFERRED_PREPASS: MeshPipelineViewLayoutKey
    - pub const OIT_ENABLED: MeshPipelineViewLayoutKey
  - impl MeshPipelineViewLayoutKey

A key that uniquely identifies a MeshPipelineViewLayout.

Used to generate all possible layouts for the mesh pipeline in generate_view_layouts, so special care must be taken to not add too many flags, as the number of possible layouts will grow exponentially.

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

Builds a unique label for each layout based on the flags

The bitwise and (&) of the bits in two flags values.

The bitwise and (&) of the bits in two flags values.

The bitwise or (|) o

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshPipelineViewLayoutKey(/* private fields */);
```

---

## Struct TetrahedronMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.TetrahedronMeshBuilder.html

**Contents:**
- Struct TetrahedronMeshBuilder Copy item path
- Trait Implementations§
  - impl Clone for TetrahedronMeshBuilder
    - fn clone(&self) -> TetrahedronMeshBuilder
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for TetrahedronMeshBuilder
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for TetrahedronMeshBuilder
    - fn default() -> TetrahedronMeshBuilder
  - impl FromArg for TetrahedronMeshBuilder

A builder used for creating a Mesh with a Tetrahedron shape.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct TetrahedronMeshBuilder { /* private fields */ }
```

---

## Struct GltfMaterialExtras Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/struct.GltfMaterialExtras.html

**Contents:**
- Struct GltfMaterialExtras Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for GltfMaterialExtras
    - fn clone(&self) -> GltfMaterialExtras
    - fn clone_from(&mut self, source: &Self)
  - impl Component for GltfMaterialExtraswhere GltfMaterialExtras: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

Additional untyped data that can be present on most glTF types at the material level.

See the relevant glTF specification section.

Content of the extra data.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GltfMaterialExtras {
    pub value: String,
}
```

---

## Struct MaterialDrawFunction Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialDrawFunction.html

**Contents:**
- Struct MaterialDrawFunction Copy item path
- Trait Implementations§
  - impl Clone for MaterialDrawFunction
    - fn clone(&self) -> MaterialDrawFunction
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for MaterialDrawFunction
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for MaterialDrawFunction
    - fn default() -> MaterialDrawFunction
  - impl DrawFunctionLabel for MaterialDrawFunctionwhere MaterialDrawFunction: 'static + Send + Sync + Clone + Eq + Debug + Hash,

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialDrawFunction;
```

---

## Function dds_format_to_texture_format Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/fn.dds_format_to_texture_format.html

**Contents:**
- Function dds_format_to_texture_format Copy item path

bevy::imageFunction dds_format_to_texture_format Copy item pathSource pub fn dds_format_to_texture_format( dds: &Dds, is_srgb: bool, ) -> Result<TextureFormat, TextureError>

**Examples:**

Example 1 (unknown):
```unknown
pub fn dds_format_to_texture_format(
    dds: &Dds,
    is_srgb: bool,
) -> Result<TextureFormat, TextureError>
```

---

## Enum TextureAccessError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.TextureAccessError.html

**Contents:**
- Enum TextureAccessError Copy item path
- Variants§
  - OutOfBounds
    - Fields
  - UnsupportedTextureFormat(TextureFormat)
  - WrongDimension
- Trait Implementations§
  - impl Debug for TextureAccessError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for TextureAccessError

An error that occurs when accessing specific pixels in a texture.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum TextureAccessError {
    OutOfBounds {
        x: u32,
        y: u32,
        z: u32,
    },
    UnsupportedTextureFormat(TextureFormat),
    WrongDimension,
}
```

---

## Struct GltfMaterialName Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/struct.GltfMaterialName.html

**Contents:**
- Struct GltfMaterialName Copy item path
- Tuple Fields§
- Methods from Deref<Target = str>§
    - pub fn len(&self) -> usize
      - §Examples
    - pub fn is_empty(&self) -> bool
      - §Examples
    - pub fn is_char_boundary(&self, index: usize) -> bool
      - §Examples
    - pub fn floor_char_boundary(&self, index: usize) -> usize

The material name of a glTF primitive.

See the relevant glTF specification section.

Returns the length of self.

This length is in bytes, not chars or graphemes. In other words, it might not be what a human considers the length of the string.

Returns true if self has a length of zero bytes.

Checks that index-th byte is the first byte in a UTF-8 code point sequence or the end of the string.

The start and end of the string (when index == self.len()) are considered to be boundaries.

Returns false if index is greater than self.len().

Finds the closest x not exceeding index where is_char_boundary(x) is true.

This method can help you truncate a string so that it’s still valid UTF-8, but doesn’t exceed a given number of bytes. Note that this is done purely at the character level and can still visually split graphemes, even though the underlying characters aren’t split. For example, the emoji 🧑‍🔬 (scientist) could be split so that the string only includes 🧑 (person) instead.

Finds the closest x not below index where is_char_boundary(x) is true.

If index is greater than the length of the string, this returns the length of the string.

This method is the natural complement to floor_char_boundary. See that method for more details.

Converts a string slice to a byte slice. To convert the byte slice back into a string slice, use the from_utf8 function.

Converts a string slice to a raw pointer.

As string slices are a slice of bytes, the raw pointer points to a u8. This pointer will be pointing to the first byte of the string slice.

The caller must ensure that the returned pointer is never written to. If you need to mutate the contents of the string slice, use as_mut_ptr.

Returns a subslice of str.

This is the non-panicking alternative to indexing the str. Returns None whenever equivalent indexing operation would panic.

Returns an unchecked subslice of str.

This is the unchecked alternative to indexing the str.

Callers of this function are responsible that these preconditions are satisfied:

Failing that, the returned string slice may reference invalid memory or violate the invariants communicated by the str type.

Creates a string slice from another string slice, bypassing safety checks.

This is generally not recommended, use with caution! For a safe alternative see str and Index.

This new slice goes from begin to end, including begin but excluding end.

To get a mutable string slice instead, see the slice_mut_unchecked method.

Callers of this funct

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct GltfMaterialName(pub String);
```

Example 2 (javascript):
```javascript
let len = "foo".len();
assert_eq!(3, len);

assert_eq!("ƒoo".len(), 4); // fancy f!
assert_eq!("ƒoo".chars().count(), 3);
```

Example 3 (javascript):
```javascript
let s = "";
assert!(s.is_empty());

let s = "not empty";
assert!(!s.is_empty());
```

Example 4 (javascript):
```javascript
let s = "Löwe 老虎 Léopard";
assert!(s.is_char_boundary(0));
// start of `老`
assert!(s.is_char_boundary(6));
assert!(s.is_char_boundary(s.len()));

// second byte of `ö`
assert!(!s.is_char_boundary(2));

// third byte of `老`
assert!(!s.is_char_boundary(8));
```

---

## Struct Viewport Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.Viewport.html

**Contents:**
- Struct Viewport Copy item path
- Fields§
- Implementations§
  - impl Viewport
    - pub fn clamp_to_size(&mut self, size: UVec2)
    - pub fn from_viewport_and_override( viewport: Option<&Viewport>, main_pass_resolution_override: Option<&MainPassResolutionOverride>, ) -> Option<Viewport>
      - Examples found in repository?
- Trait Implementations§
  - impl Clone for Viewport
    - fn clone(&self) -> Viewport

Render viewport configuration for the Camera component.

The viewport defines the area on the render target to which the camera renders its image. You can overlay multiple cameras in a single window using viewports to create effects like split screen, minimaps, and character viewers.

The physical position to render this viewport to within the RenderTarget of this Camera. (0,0) corresponds to the top-left corner

The physical size of the viewport rectangle to render to within the RenderTarget of this Camera. The origin of the rectangle is in the top-left corner.

The minimum and maximum depth to render (on a scale from 0.0 to 1.0).

Cut the viewport rectangle so that it lies inside a rectangle of the given size.

If either of the viewport’s position coordinates lies outside the given dimensions, it will be moved just inside first. If either of the given dimensions is zero, the position and size of the viewport rectangle will both be set to zero in that dimension.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Viewport {
    pub physical_position: UVec2,
    pub physical_size: UVec2,
    pub depth: Range<f32>,
}
```

Example 2 (javascript):
```javascript
584    fn run<'w>(
585        &self,
586        graph: &mut RenderGraphContext,
587        render_context: &mut RenderContext<'w>,
588        (camera, view, target, resolution_override): QueryItem<'w, '_, Self::ViewQuery>,
589        world: &'w World,
590    ) -> Result<(), NodeRunError> {
591        // First, we need to get our phases resource
592        let Some(stencil_phases) = world.get_resource::<ViewSortedRenderPhases<Stencil3d>>() else {
593            return Ok(());
594        };
595
596        // Get the view entity from the graph
597        let view_entity = graph.view_entity();
598
...
```

---

## Struct ExrTextureLoader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.ExrTextureLoader.html

**Contents:**
- Struct ExrTextureLoader Copy item path
- Trait Implementations§
  - impl AssetLoader for ExrTextureLoader
    - type Asset = Image
    - type Settings = ExrTextureLoaderSettings
    - type Error = ExrTextureLoaderError
    - async fn load( &self, reader: &mut dyn Reader, settings: &<ExrTextureLoader as AssetLoader>::Settings, _load_context: &mut LoadContext<'_>, ) -> Result<Image, <ExrTextureLoader as AssetLoader>::Error>
    - fn extensions(&self) -> &[&str]
  - impl Clone for ExrTextureLoader
    - fn clone(&self) -> ExrTextureLoader

Loads EXR textures as Texture assets

Processes the asset in an asynchronous closure.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ExrTextureLoader;
```

---

## Struct MeshPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.MeshPlugin.html

**Contents:**
- Struct MeshPlugin Copy item path
- Trait Implementations§
  - impl Default for MeshPlugin
    - fn default() -> MeshPlugin
  - impl Plugin for MeshPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str

Adds Mesh as an asset.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshPlugin;
```

---

## Struct AnnulusMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.AnnulusMeshBuilder.html

**Contents:**
- Struct AnnulusMeshBuilder Copy item path
- Fields§
- Implementations§
  - impl AnnulusMeshBuilder
    - pub fn new( inner_radius: f32, outer_radius: f32, resolution: u32, ) -> AnnulusMeshBuilder
    - pub fn resolution(self, resolution: u32) -> AnnulusMeshBuilder
- Trait Implementations§
  - impl Clone for AnnulusMeshBuilder
    - fn clone(&self) -> AnnulusMeshBuilder
    - fn clone_from(&mut self, source: &Self)

A builder for creating a Mesh with an Annulus shape.

The number of vertices used in constructing each concentric circle of the annulus mesh. The default is 32.

Create an AnnulusMeshBuilder with the given inner radius, outer radius, and angular vertex count.

Sets the number of vertices used in constructing the concentric circles of the annulus mesh.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AnnulusMeshBuilder {
    pub annulus: Annulus,
    pub resolution: u32,
}
```

---

## Struct EarlyGpuPreprocessNode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.EarlyGpuPreprocessNode.html

**Contents:**
- Struct EarlyGpuPreprocessNode Copy item path
- Trait Implementations§
  - impl FromWorld for EarlyGpuPreprocessNode
    - fn from_world(world: &mut World) -> EarlyGpuPreprocessNode
  - impl Node for EarlyGpuPreprocessNode
    - fn update(&mut self, world: &mut World)
    - fn run<'w>( &self, graph: &mut RenderGraphContext<'_>, render_context: &mut RenderContext<'w>, world: &'w World, ) -> Result<(), NodeRunError>
    - fn input(&self) -> Vec<SlotInfo>
    - fn output(&self) -> Vec<SlotInfo>
- Auto Trait Implementations§

The render node for the first mesh preprocessing pass.

This pass runs a compute shader to cull meshes outside the view frustum (if that wasn’t done by the CPU), cull meshes that weren’t visible last frame (if occlusion culling is on), transform them, and, if indirect drawing is on, populate indirect draw parameter metadata for the subsequent EarlyPrepassBuildIndirectParametersNode.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct EarlyGpuPreprocessNode { /* private fields */ }
```

---

## Struct MeshViewBindGroup Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshViewBindGroup.html

**Contents:**
- Struct MeshViewBindGroup Copy item path
- Fields§
- Trait Implementations§
  - impl Component for MeshViewBindGroupwhere MeshViewBindGroup: Send + Sync + 'static,
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
pub struct MeshViewBindGroup {
    pub main: BindGroup,
    pub binding_array: BindGroup,
    pub empty: BindGroup,
}
```

---

## Enum TextureAtlasBuilderError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.TextureAtlasBuilderError.html

**Contents:**
- Enum TextureAtlasBuilderError Copy item path
- Variants§
  - NotEnoughSpace
  - WrongFormat
  - UninitializedAtlas
  - UninitializedSourceTexture
  - TextureAccess(TextureAccessError)
- Trait Implementations§
  - impl Debug for TextureAtlasBuilderError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

Attempted to add a texture to an uninitialized atlas

Attempted to add an uninitialized texture to an atlas

A texture access error occurred

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum TextureAtlasBuilderError {
    NotEnoughSpace,
    WrongFormat,
    UninitializedAtlas,
    UninitializedSourceTexture,
    TextureAccess(TextureAccessError),
}
```

---

## Enum DynamicTextureAtlasBuilderError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.DynamicTextureAtlasBuilderError.html

**Contents:**
- Enum DynamicTextureAtlasBuilderError Copy item path
- Variants§
  - FailedToAllocateSpace
  - UninitializedAtlas
  - UninitializedSourceTexture
  - TextureAccess(TextureAccessError)
- Trait Implementations§
  - impl Debug for DynamicTextureAtlasBuilderError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for DynamicTextureAtlasBuilderError

An error produced by DynamicTextureAtlasBuilder when trying to add a new texture to a TextureAtlasLayout.

Unable to allocate space within the atlas for the new texture

Attempted to add a texture to an uninitialized atlas

Attempted to add an uninitialized texture to an atlas

A texture access error occurred

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum DynamicTextureAtlasBuilderError {
    FailedToAllocateSpace,
    UninitializedAtlas,
    UninitializedSourceTexture,
    TextureAccess(TextureAccessError),
}
```

---

## Struct Polyline2dMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.Polyline2dMeshBuilder.html

**Contents:**
- Struct Polyline2dMeshBuilder Copy item path
- Trait Implementations§
  - impl Clone for Polyline2dMeshBuilder
    - fn clone(&self) -> Polyline2dMeshBuilder
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for Polyline2dMeshBuilder
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for Polyline2dMeshBuilder
    - fn default() -> Polyline2dMeshBuilder
  - impl FromArg for Polyline2dMeshBuilder

A builder used for creating a Mesh with a Polyline2d shape.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Polyline2dMeshBuilder { /* private fields */ }
```

---

## Struct CustomProjection Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.CustomProjection.html

**Contents:**
- Struct CustomProjection Copy item path
- Implementations§
  - impl CustomProjection
    - pub fn get<P>(&self) -> Option<&P>where P: CameraProjection + Debug + Send + Sync + Clone + 'static,
    - pub fn get_mut<P>(&mut self) -> Option<&mut P>where P: CameraProjection + Debug + Send + Sync + Clone + 'static,
- Trait Implementations§
  - impl Clone for CustomProjection
    - fn clone(&self) -> CustomProjection
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for CustomProjection

Holds a dynamic CameraProjection trait object. Use Projection::custom() to construct a custom projection.

The contained dynamic object can be downcast into a static type using CustomProjection::get.

Returns a reference to the CameraProjection P.

Returns None if this dynamic object is not a projection of type P.

Returns a mutable reference to the CameraProjection P.

Returns None if this dynamic object is not a projection of type P.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CustomProjection { /* private fields */ }
```

Example 2 (javascript):
```javascript
// For simplicity's sake, use perspective as a custom projection:
let projection = Projection::custom(PerspectiveProjection::default());
let Projection::Custom(custom) = projection else { return };

// At this point the projection type is erased.
// We can use `get()` if we know what kind of projection we have.
let perspective = custom.get::<PerspectiveProjection>().unwrap();

assert_eq!(perspective.fov, PerspectiveProjection::default().fov);
```

Example 3 (javascript):
```javascript
// For simplicity's sake, use perspective as a custom projection:
let mut projection = Projection::custom(PerspectiveProjection::default());
let Projection::Custom(mut custom) = projection else { return };

// At this point the projection type is erased.
// We can use `get_mut()` if we know what kind of projection we have.
let perspective = custom.get_mut::<PerspectiveProjection>().unwrap();

assert_eq!(perspective.fov, PerspectiveProjection::default().fov);
perspective.fov = 1.0;
```

---

## Struct EarlyPrepassBuildIndirectParametersNode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.EarlyPrepassBuildIndirectParametersNode.html

**Contents:**
- Struct EarlyPrepassBuildIndirectParametersNode Copy item path
- Trait Implementations§
  - impl FromWorld for EarlyPrepassBuildIndirectParametersNode
    - fn from_world(world: &mut World) -> EarlyPrepassBuildIndirectParametersNode
  - impl Node for EarlyPrepassBuildIndirectParametersNode
    - fn update(&mut self, world: &mut World)
    - fn run<'w>( &self, _: &mut RenderGraphContext<'_>, render_context: &mut RenderContext<'w>, world: &'w World, ) -> Result<(), NodeRunError>
    - fn input(&self) -> Vec<SlotInfo>
    - fn output(&self) -> Vec<SlotInfo>
- Auto Trait Implementations§

The render node for the part of the indirect parameter building pass that draws the meshes visible from the previous frame.

This node runs a compute shader on the output of the EarlyGpuPreprocessNode in order to transform the IndirectParametersGpuMetadata into properly-formatted IndirectParametersIndexed and IndirectParametersNonIndexed.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct EarlyPrepassBuildIndirectParametersNode { /* private fields */ }
```

---

## Struct SubCameraView Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.SubCameraView.html

**Contents:**
- Struct SubCameraView Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for SubCameraView
    - fn clone(&self) -> SubCameraView
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for SubCameraView
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for SubCameraView
    - fn default() -> SubCameraView

Settings to define a camera sub view.

When Camera::sub_camera_view is Some, only the sub-section of the image defined by size and offset (relative to the full_size of the whole image) is projected to the cameras viewport.

Take the example of the following multi-monitor setup:

If each monitor is 1920x1080, the whole image will have a resolution of 3840x2160. For each monitor we can use a single camera with a viewport of the same size as the monitor it corresponds to. To ensure that the image is cohesive, we can use a different sub view on each camera:

However since only the ratio between the values is important, they could all be divided by 120 and still produce the same image. Camera D would for example have the following values: full_size = 32x18, size = 16x9, offset = 16,9

Size of the entire camera view

Offset of the sub camera

Size of the sub camera

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct SubCameraView {
    pub full_size: UVec2,
    pub offset: Vec2,
    pub size: UVec2,
}
```

Example 2 (css):
```css
┌───┬───┐
│ A │ B │
├───┼───┤
│ C │ D │
└───┴───┘
```

---

## Struct MeshInputUniform Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshInputUniform.html

**Contents:**
- Struct MeshInputUniform Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for MeshInputUniform
    - fn clone(&self) -> MeshInputUniform
    - fn clone_from(&mut self, source: &Self)
  - impl CreateFrom for MeshInputUniformwhere MeshInputUniform: ShaderType<ExtraMetadata = StructMetadata<12>>, [Vec4; 3]: for<'__> CreateFrom, UVec2: for<'__> CreateFrom, u32: for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom,
    - fn create_from<B>(reader: &mut Reader<B>) -> MeshInputUniformwhere B: BufferRef,
  - impl Debug for MeshInputUniform
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

Information that has to be transferred from CPU to GPU in order to produce the full MeshUniform.

This is essentially a subset of the fields in MeshUniform above.

Affine 4x3 matrix transposed to 3x4.

Four 16-bit unsigned normalized UV values packed into a UVec2:

The index of this mesh’s MeshInputUniform in the previous frame’s buffer, if applicable.

This is used for TAA. If not present, this will be u32::MAX.

The index of this mesh’s first vertex in the vertex buffer.

Multiple meshes can be packed into a single vertex buffer (see MeshAllocator). This value stores the offset of the first vertex in this mesh in that buffer.

The index of this mesh’s first index in the index buffer, if any.

Multiple meshes can be packed into a single index buffer (see MeshAllocator). This value stores the offset of the first index in this mesh in that buffer.

If this mesh isn’t indexed, this value is ignored.

For an indexed mesh, the number of indices that make it up; for a non-indexed mesh, the number of vertices in it.

The current skin index, or u32::MAX if there’s no skin.

The material and lightmap indices, packed into 32 bits.

Low 16 bits: index of the material inside the bind group data. High 16 bits: index of the lightmap in the binding array.

The number of the frame on which this MeshInputUniform was built.

This is used to validate the previous transform and skin. If this MeshInputUniform wasn’t updated on this frame, then we know that neither this mesh’s transform nor that of its joints have been updated on this frame, and therefore the transforms of both this mesh and its joints must be identical to those for the previous frame.

User supplied tag to identify this mesh instance.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct MeshInputUniform {
    pub world_from_local: [Vec4; 3],
    pub lightmap_uv_rect: UVec2,
    pub flags: u32,
    pub previous_input_index: u32,
    pub first_vertex_index: u32,
    pub first_index_index: u32,
    pub index_count: u32,
    pub current_skin_index: u32,
    pub material_and_lightmap_bind_group_slot: u32,
    pub timestamp: u32,
    pub tag: u32,
    pub pad: u32,
}
```

Example 2 (text):
```text
<--- MSB                   LSB --->
                        +---- min v ----+ +---- min u ----+
    lightmap_uv_rect.x: vvvvvvvv vvvvvvvv uuuuuuuu uuuuuuuu,
                        +---- max v ----+ +---- max u ----+
    lightmap_uv_rect.y: VVVVVVVV VVVVVVVV UUUUUUUU UUUUUUUU,

(MSB: most significant bit; LSB: least significant bit.)
```

---

## Enum ScalingMode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/enum.ScalingMode.html

**Contents:**
- Enum ScalingMode Copy item path
- §Examples
- Variants§
  - WindowSize
  - Fixed
    - Fields
  - AutoMin
    - Fields
  - AutoMax
    - Fields

Scaling mode for OrthographicProjection.

The effect of these scaling modes are combined with the OrthographicProjection::scale property.

For example, if the scaling mode is ScalingMode::Fixed { width: 100.0, height: 300 } and the scale is 2.0, the projection will be 200 world units wide and 600 world units tall.

Configure the orthographic projection to two world units per window height:

Match the viewport size.

With a scale of 1, lengths in world units will map 1:1 with the number of pixels used to render it. For example, if we have a 64x64 sprite with a Transform::scale of 1.0, no custom size and no inherited scale, the sprite will be 64 world units wide and 64 world units tall. When rendered with OrthographicProjection::scaling_mode set to WindowSize when the window scale factor is 1 the sprite will be rendered at 64 pixels wide and 64 pixels tall.

Changing any of these properties will multiplicatively affect the final size.

Manually specify the projection’s size, ignoring window resizing. The image will stretch.

Arguments describe the area of the world that is shown (in world units).

Keeping the aspect ratio while the axes can’t be smaller than given minimum.

Arguments are in world units.

Keeping the aspect ratio while the axes can’t be bigger than given maximum.

Arguments are in world units.

Keep the projection’s height constant; width will be adjusted to match aspect ratio.

The argument is the desired height of the projection in world units.

Keep the projection’s width constant; height will be adjusted to match aspect ratio.

The argument is the desired width of the projection in world units.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ScalingMode {
    WindowSize,
    Fixed {
        width: f32,
        height: f32,
    },
    AutoMin {
        min_width: f32,
        min_height: f32,
    },
    AutoMax {
        max_width: f32,
        max_height: f32,
    },
    FixedVertical {
        viewport_height: f32,
    },
    FixedHorizontal {
        viewport_width: f32,
    },
}
```

Example 2 (javascript):
```javascript
let projection = Projection::Orthographic(OrthographicProjection {
   scaling_mode: ScalingMode::FixedVertical { viewport_height: 2.0 },
   ..OrthographicProjection::default_2d()
});
```

---

## Macro define_atomic_id Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/macro.define_atomic_id.html

**Contents:**
- Macro define_atomic_id Copy item path

bevy::renderMacro define_atomic_id Copy item pathSource macro_rules! define_atomic_id { ($atomic_id_type:ident) => { ... }; }

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! define_atomic_id {
    ($atomic_id_type:ident) => { ... };
}
```

---

## Struct VertexAttributeDescriptor Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.VertexAttributeDescriptor.html

**Contents:**
- Struct VertexAttributeDescriptor Copy item path
- Fields§
- Implementations§
  - impl VertexAttributeDescriptor
    - pub const fn new( shader_location: u32, id: MeshVertexAttributeId, name: &'static str, ) -> VertexAttributeDescriptor
- Auto Trait Implementations§
  - impl Freeze for VertexAttributeDescriptor
  - impl RefUnwindSafe for VertexAttributeDescriptor
  - impl Send for VertexAttributeDescriptor
  - impl Sync for VertexAttributeDescriptor

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct VertexAttributeDescriptor {
    pub shader_location: u32,
    pub id: MeshVertexAttributeId,
    /* private fields */
}
```

---

## Function queue_material_meshes Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/fn.queue_material_meshes.html

**Contents:**
- Function queue_material_meshes Copy item path

For each view, iterates over all the meshes visible from that view and adds them to BinnedRenderPhases or SortedRenderPhases as appropriate.

**Examples:**

Example 1 (unknown):
```unknown
pub fn queue_material_meshes(
    render_materials: Res<'_, ErasedRenderAssets<PreparedMaterial>>,
    render_mesh_instances: Res<'_, RenderMeshInstances>,
    render_material_instances: Res<'_, RenderMaterialInstances>,
    mesh_allocator: Res<'_, MeshAllocator>,
    gpu_preprocessing_support: Res<'_, GpuPreprocessingSupport>,
    opaque_render_phases: ResMut<'_, ViewBinnedRenderPhases<Opaque3d>>,
    alpha_mask_render_phases: ResMut<'_, ViewBinnedRenderPhases<AlphaMask3d>>,
    transmissive_render_phases: ResMut<'_, ViewSortedRenderPhases<Transmissive3d>>,
    transparent_render_phases: ResM
...
```

---

## Struct CylinderMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.CylinderMeshBuilder.html

**Contents:**
- Struct CylinderMeshBuilder Copy item path
- Fields§
- Implementations§
  - impl CylinderMeshBuilder
    - pub fn new(radius: f32, height: f32, resolution: u32) -> CylinderMeshBuilder
    - pub const fn resolution(self, resolution: u32) -> CylinderMeshBuilder
      - Examples found in repository?
    - pub const fn segments(self, segments: u32) -> CylinderMeshBuilder
    - pub const fn without_caps(self) -> CylinderMeshBuilder
    - pub const fn anchor(self, anchor: CylinderAnchor) -> CylinderMeshBuilder

A builder used for creating a Mesh with a Cylinder shape.

The number of vertices used for the top and bottom of the cylinder.

The number of segments along the height of the cylinder. Must be greater than 0 for geometry to be generated.

If set to true, the cylinder caps (flat circle faces) are built, otherwise the mesh will be a shallow tube

The anchor point for the cylinder mesh, defaults to the midpoint between the top and bottom caps

Creates a new CylinderMeshBuilder from the given radius, a height, and a resolution used for the top and bottom.

Sets the number of vertices used for the top and bottom of the cylinder.

Sets the number of segments along the height of the cylinder. Must be greater than 0 for geometry to be generated.

Ignore the cylinder caps, making the mesh a shallow tube instead

Sets a custom anchor point for the mesh

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CylinderMeshBuilder {
    pub cylinder: Cylinder,
    pub resolution: u32,
    pub segments: u32,
    pub caps: bool,
    pub anchor: CylinderAnchor,
}
```

Example 2 (javascript):
```javascript
60fn setup(
61    mut commands: Commands,
62    mut meshes: ResMut<Assets<Mesh>>,
63    mut materials: ResMut<Assets<StandardMaterial>>,
64    asset_server: Res<AssetServer>,
65) {
66    let icosphere_mesh = meshes.add(Sphere::new(0.9).mesh().ico(7).unwrap());
67    let cube_mesh = meshes.add(Cuboid::new(0.7, 0.7, 0.7));
68    let plane_mesh = meshes.add(Plane3d::default().mesh().size(2.0, 2.0));
69    let cylinder_mesh = meshes.add(Cylinder::new(0.5, 2.0).mesh().resolution(50));
70
71    // Cube #1
72    commands.spawn((
73        Mesh3d(cube_mesh.clone()),
74        MeshMaterial3d(materials.
...
```

---

## Enum ClearColorConfig Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/enum.ClearColorConfig.html

**Contents:**
- Enum ClearColorConfig Copy item path
- Variants§
  - Default
  - Custom(Color)
  - None
- Trait Implementations§
  - impl Clone for ClearColorConfig
    - fn clone(&self) -> ClearColorConfig
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for ClearColorConfig

For a camera, specifies the color used to clear the viewport before rendering or when writing to the final render target texture.

The clear color is taken from the world’s ClearColor resource.

The given clear color is used, overriding the ClearColor resource defined in the world.

No clear color is used: the camera will simply draw on top of anything already in the viewport.

This can be useful when multiple cameras are rendering to the same viewport.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ClearColorConfig {
    Default,
    Custom(Color),
    None,
}
```

---

## Struct MeshPipeline Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshPipeline.html

**Contents:**
- Struct MeshPipeline Copy item path
- Fields§
- Implementations§
  - impl MeshPipeline
    - pub fn get_image_texture<'a>( &'a self, gpu_images: &'a RenderAssets<GpuImage>, handle_option: &Option<Handle<Image>>, ) -> Option<(&'a TextureView, &'a Sampler)>
    - pub fn get_view_layout( &self, layout_key: MeshPipelineViewLayoutKey, ) -> &MeshPipelineViewLayout
      - Examples found in repository?
- Trait Implementations§
  - impl Clone for MeshPipeline
    - fn clone(&self) -> MeshPipeline

All data needed to construct a pipeline for rendering 3D meshes.

A reference to all the mesh pipeline view layouts.

The shader asset handle.

MeshUniforms are stored in arrays in buffers. If storage buffers are available, they are used and this will be None, otherwise uniform buffers will be used with batches of this many MeshUniforms, stored at dynamic offsets within the uniform buffer. Use code like this in custom shaders:

Whether binding arrays (a.k.a. bindless textures) are usable on the current render device.

This affects whether reflection probes can be used.

Whether clustered decals are usable on the current render device.

Whether skins will use uniform buffers on account of storage buffers being unavailable on this platform.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshPipeline {
    pub view_layouts: MeshPipelineViewLayouts,
    pub dummy_white_gpu_image: GpuImage,
    pub clustered_forward_buffer_binding_type: BufferBindingType,
    pub mesh_layouts: MeshLayouts,
    pub shader: Handle<Shader>,
    pub per_object_buffer_batch_size: Option<u32>,
    pub binding_arrays_are_usable: bool,
    pub clustered_decals_are_usable: bool,
    pub skins_use_uniform_buffers: bool,
}
```

Example 2 (wgsl):
```wgsl
##ifdef PER_OBJECT_BUFFER_BATCH_SIZE
@group(1) @binding(0) var<uniform> mesh: array<Mesh, #{PER_OBJECT_BUFFER_BATCH_SIZE}u>;
##else
@group(1) @binding(0) var<storage> mesh: array<Mesh>;
##endif // PER_OBJECT_BUFFER_BATCH_SIZE
```

Example 3 (javascript):
```javascript
180    fn specialize(
181        &self,
182        key: Self::Key,
183        layout: &MeshVertexBufferLayoutRef,
184    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
185        // We will only use the position of the mesh in our shader so we only need to specify that
186        let mut vertex_attributes = Vec::new();
187        if layout.0.contains(Mesh::ATTRIBUTE_POSITION) {
188            // Make sure this matches the shader location
189            vertex_attributes.push(Mesh::ATTRIBUTE_POSITION.at_shader_location(0));
190        }
191        // This will automatical
...
```

Example 4 (javascript):
```javascript
186    fn specialize(
187        &self,
188        mesh_key: Self::Key,
189        layout: &MeshVertexBufferLayoutRef,
190    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
191        // Define the vertex attributes based on a standard bevy [`Mesh`]
192        let mut vertex_attributes = Vec::new();
193        if layout.0.contains(Mesh::ATTRIBUTE_POSITION) {
194            // Make sure this matches the shader location
195            vertex_attributes.push(Mesh::ATTRIBUTE_POSITION.at_shader_location(0));
196        }
197        if layout.0.contains(Mesh::ATTRIBUTE_COLOR) 
...
```

---

## Struct StandardMaterialFlags Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.StandardMaterialFlags.html

**Contents:**
- Struct StandardMaterialFlags Copy item path
- Implementations§
  - impl StandardMaterialFlags
    - pub const BASE_COLOR_TEXTURE: StandardMaterialFlags
    - pub const EMISSIVE_TEXTURE: StandardMaterialFlags
    - pub const METALLIC_ROUGHNESS_TEXTURE: StandardMaterialFlags
    - pub const OCCLUSION_TEXTURE: StandardMaterialFlags
    - pub const DOUBLE_SIDED: StandardMaterialFlags
    - pub const UNLIT: StandardMaterialFlags
    - pub const TWO_COMPONENT_NORMAL_MAP: StandardMaterialFlags

Bitflags info about the material a shader is currently rendering. This is accessible in the shader in the StandardMaterialUniform

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

The bitwise exclusive-or (^) of the bit

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct StandardMaterialFlags(/* private fields */);
```

---

## Struct MainBuildIndirectParametersNode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MainBuildIndirectParametersNode.html

**Contents:**
- Struct MainBuildIndirectParametersNode Copy item path
- Trait Implementations§
  - impl FromWorld for MainBuildIndirectParametersNode
    - fn from_world(world: &mut World) -> MainBuildIndirectParametersNode
  - impl Node for MainBuildIndirectParametersNode
    - fn update(&mut self, world: &mut World)
    - fn run<'w>( &self, _: &mut RenderGraphContext<'_>, render_context: &mut RenderContext<'w>, world: &'w World, ) -> Result<(), NodeRunError>
    - fn input(&self) -> Vec<SlotInfo>
    - fn output(&self) -> Vec<SlotInfo>
- Auto Trait Implementations§

The render node for the part of the indirect parameter building pass that draws all meshes, both those that are newly-visible on this frame and those that were visible last frame.

This node runs a compute shader on the output of the EarlyGpuPreprocessNode and LateGpuPreprocessNode in order to transform the IndirectParametersGpuMetadata into properly-formatted IndirectParametersIndexed and IndirectParametersNonIndexed.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MainBuildIndirectParametersNode { /* private fields */ }
```

---

## Struct MaterialBindingId Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialBindingId.html

**Contents:**
- Struct MaterialBindingId Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for MaterialBindingId
    - fn clone(&self) -> MaterialBindingId
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for MaterialBindingId
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for MaterialBindingId
    - fn default() -> MaterialBindingId

The location of a material (either bindless or non-bindless) within the slabs.

The index of the bind group (slab) where the GPU data is located.

The slot within that bind group.

Non-bindless materials will always have a slot of 0.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialBindingId {
    pub group: MaterialBindGroupIndex,
    pub slot: MaterialBindGroupSlot,
}
```

---

## Struct SpotLightTexture Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.SpotLightTexture.html

**Contents:**
- Struct SpotLightTexture Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for SpotLightTexture
    - fn clone(&self) -> SpotLightTexture
    - fn clone_from(&mut self, source: &Self)
  - impl Component for SpotLightTexturewhere SpotLightTexture: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

Add to a SpotLight to add a light texture effect. A texture mask is applied to the light source to modulate its intensity, simulating patterns like window shadows, gobo/cookie effects, or soft falloffs.

The texture image. Only the R channel is read. Note the border of the image should be entirely black to avoid leaking light.

Required Components: SpotLight.

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components of the required components, recursively, in depth-first order.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct SpotLightTexture {
    pub image: Handle<Image>,
}
```

---

## Enum AtmosphereMode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/enum.AtmosphereMode.html

**Contents:**
- Enum AtmosphereMode Copy item path
- Variants§
  - LookupTexture = 0
  - Raymarched = 1
- Trait Implementations§
  - impl Clone for AtmosphereMode
    - fn clone(&self) -> AtmosphereMode
    - fn clone_from(&mut self, source: &Self)
  - impl Default for AtmosphereMode
    - fn default() -> AtmosphereMode

Selects how the atmosphere is rendered. Choose based on scene scale and volumetric shadow quality, and based on performance needs.

High-performance solution tailored to scenes that are mostly inside of the atmosphere. Uses a set of lookup textures to approximate scattering integration. Slightly less accurate for very long-distance/space views (lighting precision tapers as the camera moves far from the scene origin) and for sharp volumetric (cloud/fog) shadows.

Slower, more accurate rendering method for any type of scene. Integrates the scattering numerically with raymarching and produces sharp volumetric (cloud/fog) shadows. Best for cinematic shots, planets seen from orbit, and scenes requiring accurate long-distance lighting.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[repr(u32)]pub enum AtmosphereMode {
    LookupTexture = 0,
    Raymarched = 1,
}
```

---

## Struct Capsule2dMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.Capsule2dMeshBuilder.html

**Contents:**
- Struct Capsule2dMeshBuilder Copy item path
- Fields§
- Implementations§
  - impl Capsule2dMeshBuilder
    - pub fn new(radius: f32, length: f32, resolution: u32) -> Capsule2dMeshBuilder
    - pub const fn resolution(self, resolution: u32) -> Capsule2dMeshBuilder
- Trait Implementations§
  - impl Clone for Capsule2dMeshBuilder
    - fn clone(&self) -> Capsule2dMeshBuilder
    - fn clone_from(&mut self, source: &Self)

A builder used for creating a Mesh with a Capsule2d shape.

The number of vertices used for one hemicircle. The total number of vertices for the capsule mesh will be two times the resolution.

Creates a new Capsule2dMeshBuilder from a given radius, length, and the number of vertices used for one hemicircle. The total number of vertices for the capsule mesh will be two times the resolution.

Sets the number of vertices used for one hemicircle. The total number of vertices for the capsule mesh will be two times the resolution.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Capsule2dMeshBuilder {
    pub capsule: Capsule2d,
    pub resolution: u32,
}
```

---

## Struct MaterialPipelineSpecializer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialPipelineSpecializer.html

**Contents:**
- Struct MaterialPipelineSpecializer Copy item path
- Trait Implementations§
  - impl SpecializedMeshPipeline for MaterialPipelineSpecializer
    - type Key = ErasedMaterialPipelineKey
    - fn specialize( &self, key: <MaterialPipelineSpecializer as SpecializedMeshPipeline>::Key, layout: &MeshVertexBufferLayoutRef, ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError>
- Auto Trait Implementations§
  - impl Freeze for MaterialPipelineSpecializer
  - impl !RefUnwindSafe for MaterialPipelineSpecializer
  - impl Send for MaterialPipelineSpecializer
  - impl Sync for MaterialPipelineSpecializer

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialPipelineSpecializer { /* private fields */ }
```

---

## Struct MeshTransforms Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshTransforms.html

**Contents:**
- Struct MeshTransforms Copy item path
- Fields§
- Trait Implementations§
  - impl Component for MeshTransformswhere MeshTransforms: Send + Sync + 'static,
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
pub struct MeshTransforms {
    pub world_from_local: Affine3,
    pub previous_world_from_local: Affine3,
    pub flags: u32,
}
```

---

## Module render_phase Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/render_phase/index.html

**Contents:**
- Module render_phase Copy item path
- Structs§
- Enums§
- Traits§
- Functions§
- Type Aliases§
- Derive Macros§

The modular rendering abstraction responsible for queuing, preparing, sorting and drawing entities as part of separate render phases.

In Bevy each view (camera, or shadow-casting light, etc.) has one or multiple render phases (e.g. opaque, transparent, shadow, etc). They are used to queue entities for rendering. Multiple phases might be required due to different sorting/batching behaviors (e.g. opaque: front to back, transparent: back to front) or because one phase depends on the rendered texture of the previous phase (e.g. for screen-space reflections).

To draw an entity, a corresponding PhaseItem has to be added to one or multiple of these render phases for each view that it is visible in. This must be done in the RenderSystems::Queue. After that the render phase sorts them in the RenderSystems::PhaseSort. Finally the items are rendered using a single TrackedRenderPass, during the RenderSystems::Render.

Therefore each phase item is assigned a Draw function. These set up the state of the TrackedRenderPass (i.e. select the RenderPipeline, configure the BindGroups, etc.) and then issue a draw call, for the corresponding item.

The Draw function trait can either be implemented directly or such a function can be created by composing multiple RenderCommands.

---

## Struct RenderDebugFlags Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/struct.RenderDebugFlags.html

**Contents:**
- Struct RenderDebugFlags Copy item path
- Implementations§
  - impl RenderDebugFlags
    - pub const ALLOW_COPIES_FROM_INDIRECT_PARAMETERS: RenderDebugFlags
  - impl RenderDebugFlags
    - pub const fn empty() -> RenderDebugFlags
    - pub const fn all() -> RenderDebugFlags
    - pub const fn bits(&self) -> u8
    - pub const fn from_bits(bits: u8) -> Option<RenderDebugFlags>
    - pub const fn from_bits_truncate(bits: u8) -> RenderDebugFlags

Debugging flags that can optionally be set when constructing the renderer.

If true, this sets the COPY_SRC flag on indirect draw parameters so that they can be read back to CPU.

This is a debugging feature that may reduce performance. It primarily exists for the occlusion_culling example.

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

The bitwise or (|) of the bits in two flags va

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderDebugFlags(/* private fields */);
```

---

## Crate image Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/index.html

**Contents:**
- Crate image Copy item path
- Modules§
- Structs§
- Enums§
- Constants§
- Traits§
- Functions§
- Type Aliases§

bevyCrate image Copy item pathSource Modules§preludeStructs§CompressedImageFormatSupportFor defining which compressed image formats are supported. This will be initialized from available device features in finish() of the bevy RenderPlugin, but is left for the user to specify if not using the RenderPlugin, or the WGPU backend.CompressedImageFormatsCompressedImageSaverDynamicTextureAtlasBuilderHelper utility to update TextureAtlasLayout on the fly.ExrTextureLoaderLoads EXR textures as Texture assetsExrTextureLoaderSettingsFileTextureErrorAn error that occurs when loading a texture from a file.HdrTextureLoaderLoads HDR textures as Texture assetsHdrTextureLoaderSettingsImageAn image, optimized for usage in rendering.ImageLoaderLoader for images that can be read by the image crate.ImageLoaderSettingsSettings for loading an Image using an ImageLoader.ImagePluginAdds the Image as an asset and makes sure that they are extracted and prepared for the GPU.ImageSamplerDescriptorIndicates to an ImageLoader how an Image should be sampled.SerializedImageA version of Image suitable for serializing for short-term transfer.TextureAtlasAn index into a TextureAtlasLayout, which corresponds to a specific section of a texture.TextureAtlasBuilderA builder which is used to create a texture atlas from many individual sprites.TextureAtlasLayoutStores a map used to lookup the position of a texture in a TextureAtlas. This can be used to either use and look up a specific section of a texture, or animate frame-by-frame as a sprite sheet.TextureAtlasPluginAdds support for texture atlases.TextureAtlasSourcesStores a mapping from sub texture handles to the related area index.Enums§CompressedImageSaverErrorDataFormatDynamicTextureAtlasBuilderErrorAn error produced by DynamicTextureAtlasBuilder when trying to add a new texture to a TextureAtlasLayout.ExrTextureLoaderErrorPossible errors that can be produced by ExrTextureLoaderHdrTextureLoaderErrorImageAddressModeHow edges should be handled in texture addressing.ImageCompareFunctionComparison function used for depth and stencil operations.ImageFilterModeTexel mixing mode when sampling between texels.ImageFormatImageFormatSettingHow to determine an image’s format when loading.ImageLoaderErrorAn error when loading an image using ImageLoader.ImageSamplerUsed in Image, this determines what image sampler to use when rendering. The default setting, ImageSampler::Default, will read the sampler from the ImagePlugin at setup. Setting this to ImageSa

*[Content truncated]*

---

## Struct TextureUsages Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/render_resource/struct.TextureUsages.html

**Contents:**
- Struct TextureUsages Copy item path
- Implementations§
  - impl TextureUsages
    - pub const COPY_SRC: TextureUsages
    - pub const COPY_DST: TextureUsages
    - pub const TEXTURE_BINDING: TextureUsages
    - pub const STORAGE_BINDING: TextureUsages
    - pub const RENDER_ATTACHMENT: TextureUsages
    - pub const STORAGE_ATOMIC: TextureUsages
  - impl TextureUsages

Different ways that you can use a texture.

The usages determine what kind of memory the texture is allocated from and what actions the texture can partake in.

Corresponds to WebGPU GPUTextureUsageFlags.

Allows a texture to be the source in a [CommandEncoder::copy_texture_to_buffer] or [CommandEncoder::copy_texture_to_texture] operation.

Allows a texture to be the destination in a [CommandEncoder::copy_buffer_to_texture], [CommandEncoder::copy_texture_to_texture], or [Queue::write_texture] operation.

Allows a texture to be a BindingType::Texture in a bind group.

Allows a texture to be a BindingType::StorageTexture in a bind group.

Allows a texture to be an output attachment of a render pass.

Allows a texture to be used with image atomics. Requires Features::TEXTURE_ATOMIC.

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

Yield a set of contained flags value

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct TextureUsages(/* private fields */);
```

Example 2 (javascript):
```javascript
369fn setup_environment_map_usage(cubemaps: Res<Cubemaps>, mut images: ResMut<Assets<Image>>) {
370    if let Some(image) = images.get_mut(&cubemaps.specular_environment_map)
371        && !image
372            .texture_descriptor
373            .usage
374            .contains(TextureUsages::COPY_SRC)
375    {
376        image.texture_descriptor.usage |= TextureUsages::COPY_SRC;
377    }
378}
```

---

## Struct ScreenSpaceReflectionsPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ScreenSpaceReflectionsPlugin.html

**Contents:**
- Struct ScreenSpaceReflectionsPlugin Copy item path
- Trait Implementations§
  - impl Plugin for ScreenSpaceReflectionsPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str
    - fn is_unique(&self) -> bool
- Auto Trait Implementations§

Enables screen-space reflections for a camera.

Screen-space reflections are currently only supported with deferred rendering.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ScreenSpaceReflectionsPlugin;
```

---

## Struct CameraProjectionPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.CameraProjectionPlugin.html

**Contents:**
- Struct CameraProjectionPlugin Copy item path
- Trait Implementations§
  - impl Default for CameraProjectionPlugin
    - fn default() -> CameraProjectionPlugin
  - impl Plugin for CameraProjectionPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str

Adds Camera driver systems for a given projection type.

If you are using bevy_pbr, then you need to add PbrProjectionPlugin along with this.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CameraProjectionPlugin;
```

---

## Enum IcosphereError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/enum.IcosphereError.html

**Contents:**
- Enum IcosphereError Copy item path
- Variants§
  - TooManyVertices
    - Fields
- Trait Implementations§
  - impl Clone for IcosphereError
    - fn clone(&self) -> IcosphereError
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for IcosphereError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

An error when creating an icosphere Mesh from a SphereMeshBuilder.

The icosphere has too many vertices.

The number of subdivisions used. 79 is the largest allowed value for a mesh to be generated.

The number of vertices generated. 65535 is the largest allowed value for a mesh to be generated.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum IcosphereError {
    TooManyVertices {
        subdivisions: u32,
        number_of_resulting_points: u32,
    },
}
```

---

## Struct BuildIndirectParametersPipelineKey Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.BuildIndirectParametersPipelineKey.html

**Contents:**
- Struct BuildIndirectParametersPipelineKey Copy item path
- Implementations§
  - impl BuildIndirectParametersPipelineKey
    - pub const INDEXED: BuildIndirectParametersPipelineKey
    - pub const MULTI_DRAW_INDIRECT_COUNT_SUPPORTED: BuildIndirectParametersPipelineKey
    - pub const OCCLUSION_CULLING: BuildIndirectParametersPipelineKey
    - pub const EARLY_PHASE: BuildIndirectParametersPipelineKey
    - pub const LATE_PHASE: BuildIndirectParametersPipelineKey
    - pub const MAIN_PHASE: BuildIndirectParametersPipelineKey
  - impl BuildIndirectParametersPipelineKey

Specifies variants of the indirect parameter building shader.

Whether the indirect parameter building shader is processing indexed meshes (those that have index buffers).

This defines INDEXED in the shader.

Whether the GPU and driver supports multi_draw_indirect_count.

This defines MULTI_DRAW_INDIRECT_COUNT_SUPPORTED in the shader.

Whether GPU two-phase occlusion culling is in use.

This #define’s OCCLUSION_CULLING in the shader.

Whether this is the early phase of GPU two-phase occlusion culling.

This #define’s EARLY_PHASE in the shader.

Whether this is the late phase of GPU two-phase occlusion culling.

This #define’s LATE_PHASE in the shader.

Whether this is the phase that runs after the early and late phases, and right before the main drawing logic, when GPU two-phase occlusion culling is in use.

This #define’s MAIN_PHASE in the shader.

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

The bitwise negation (!) of the bits in a fl

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct BuildIndirectParametersPipelineKey(/* private fields */);
```

---

## Struct GltfLoaderSettings Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/struct.GltfLoaderSettings.html

**Contents:**
- Struct GltfLoaderSettings Copy item path
- §Example
- Fields§
- Trait Implementations§
  - impl Default for GltfLoaderSettings
    - fn default() -> GltfLoaderSettings
  - impl<'de> Deserialize<'de> for GltfLoaderSettings
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<GltfLoaderSettings, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,
  - impl Serialize for GltfLoaderSettings
    - fn serialize<__S>( &self, __serializer: __S, ) -> Result<<__S as Serializer>::Ok, <__S as Serializer>::Error>where __S: Serializer,

Specifies optional settings for processing gltfs at load time. By default, all recognized contents of the gltf will be loaded.

To load a gltf but exclude the cameras, replace a call to asset_server.load("my.gltf") with

If empty, the gltf mesh nodes will be skipped.

Otherwise, nodes will be loaded and retained in RAM/VRAM according to the active flags.

If empty, the gltf materials will be skipped.

Otherwise, materials will be loaded and retained in RAM/VRAM according to the active flags.

If true, the loader will spawn cameras for gltf camera nodes.

If true, the loader will spawn lights for gltf light nodes.

If true, the loader will load AnimationClip assets, and also add AnimationTarget and AnimationPlayer components to hierarchies affected by animation. Requires the bevy_animation feature.

If true, the loader will include the root of the gltf root node.

Overrides the default sampler. Data from sampler node is added on top of that.

If None, uses the global default which is stored in the DefaultGltfImageSampler resource.

If true, the loader will ignore sampler data from gltf and use the default sampler.

CAUTION: This is an experimental feature with known issues. Behavior may change in future versions.

How to convert glTF coordinates on import. Assuming glTF cameras, glTF lights, and glTF meshes had global unit transforms, their Bevy Transform::forward will be pointing in the following global directions:

If None, uses the global default set by GltfPlugin::use_model_forward_direction.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GltfLoaderSettings {
    pub load_meshes: RenderAssetUsages,
    pub load_materials: RenderAssetUsages,
    pub load_cameras: bool,
    pub load_lights: bool,
    pub load_animations: bool,
    pub include_source: bool,
    pub default_sampler: Option<ImageSamplerDescriptor>,
    pub override_sampler: bool,
    pub use_model_forward_direction: Option<bool>,
}
```

Example 2 (javascript):
```javascript
let gltf_handle: Handle<Gltf> = asset_server.load_with_settings(
    "my.gltf",
    |s: &mut GltfLoaderSettings| {
        s.load_cameras = false;
    }
);
```

---

## Struct MaterialPipelineKey Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialPipelineKey.html

**Contents:**
- Struct MaterialPipelineKey Copy item path
- Fields§
- Auto Trait Implementations§
  - impl<M> Freeze for MaterialPipelineKey<M>where <M as AsBindGroup>::Data: Freeze,
  - impl<M> RefUnwindSafe for MaterialPipelineKey<M>where <M as AsBindGroup>::Data: RefUnwindSafe,
  - impl<M> Send for MaterialPipelineKey<M>
  - impl<M> Sync for MaterialPipelineKey<M>
  - impl<M> Unpin for MaterialPipelineKey<M>where <M as AsBindGroup>::Data: Unpin,
  - impl<M> UnwindSafe for MaterialPipelineKey<M>where <M as AsBindGroup>::Data: UnwindSafe,
- Blanket Implementations§

A key uniquely identifying a specialized MaterialPipeline.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialPipelineKey<M>where
    M: Material,{
    pub mesh_key: MeshPipelineKey,
    pub bind_group_data: <M as AsBindGroup>::Data,
}
```

---

## Struct MeshRenderPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshRenderPlugin.html

**Contents:**
- Struct MeshRenderPlugin Copy item path
- Fields§
- Implementations§
  - impl MeshRenderPlugin
    - pub fn new(debug_flags: RenderDebugFlags) -> MeshRenderPlugin
- Trait Implementations§
  - impl Plugin for MeshRenderPlugin
    - fn build(&self, app: &mut App)
    - fn finish(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool

Provides support for rendering 3D meshes.

Whether we’re building MeshUniforms on GPU.

This requires compute shader support and so will be forcibly disabled if the platform doesn’t support those.

Debugging flags that can optionally be set when constructing the renderer.

Creates a new MeshRenderPlugin with the given debug flags.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshRenderPlugin {
    pub use_gpu_instance_buffer_builder: bool,
    pub debug_flags: RenderDebugFlags,
}
```

---

## Struct GltfMeshExtras Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/struct.GltfMeshExtras.html

**Contents:**
- Struct GltfMeshExtras Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for GltfMeshExtras
    - fn clone(&self) -> GltfMeshExtras
    - fn clone_from(&mut self, source: &Self)
  - impl Component for GltfMeshExtraswhere GltfMeshExtras: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

Additional untyped data that can be present on most glTF types at the mesh level.

See the relevant glTF specification section.

Content of the extra data.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GltfMeshExtras {
    pub value: String,
}
```

---

## Crate render Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/index.html

**Contents:**
- Crate render Copy item path
- §Useful Environment Variables
- Modules§
- Macros§
- Structs§
- Enums§
- Functions§
- Type Aliases§

Both bevy_render and wgpu have a number of environment variable options for changing the runtime behavior of both crates. Many of these may be useful in development or release environments.

---

## Struct ConicalFrustumMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.ConicalFrustumMeshBuilder.html

**Contents:**
- Struct ConicalFrustumMeshBuilder Copy item path
- Fields§
- Implementations§
  - impl ConicalFrustumMeshBuilder
    - pub const fn new( radius_top: f32, radius_bottom: f32, height: f32, resolution: u32, ) -> ConicalFrustumMeshBuilder
    - pub const fn resolution(self, resolution: u32) -> ConicalFrustumMeshBuilder
    - pub const fn segments(self, segments: u32) -> ConicalFrustumMeshBuilder
- Trait Implementations§
  - impl Clone for ConicalFrustumMeshBuilder
    - fn clone(&self) -> ConicalFrustumMeshBuilder

A builder used for creating a Mesh with a ConicalFrustum shape.

The ConicalFrustum shape.

The number of vertices used for the top and bottom of the conical frustum.

The number of horizontal lines subdividing the lateral surface of the conical frustum.

Creates a new ConicalFrustumMeshBuilder from the given top and bottom radii, a height, and a resolution used for the top and bottom.

Sets the number of vertices used for the top and bottom of the conical frustum.

Sets the number of horizontal lines subdividing the lateral surface of the conical frustum.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ConicalFrustumMeshBuilder {
    pub frustum: ConicalFrustum,
    pub resolution: u32,
    pub segments: u32,
}
```

---

## Struct GltfPrimitive Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/struct.GltfPrimitive.html

**Contents:**
- Struct GltfPrimitive Copy item path
- Fields§
- Implementations§
  - impl GltfPrimitive
    - pub fn new( gltf_mesh: &Mesh<'_>, gltf_primitive: &Primitive<'_>, mesh: Handle<Mesh>, material: Option<Handle<StandardMaterial>>, extras: Option<GltfExtras>, material_extras: Option<GltfExtras>, ) -> GltfPrimitive
    - pub fn asset_label(&self) -> GltfAssetLabel
- Trait Implementations§
  - impl Clone for GltfPrimitive
    - fn clone(&self) -> GltfPrimitive
    - fn clone_from(&mut self, source: &Self)

Part of a GltfMesh that consists of a Mesh, an optional StandardMaterial and GltfExtras.

See the relevant glTF specification section.

Index of the primitive inside the mesh

Index of the parent GltfMesh of this primitive

Computed name for a primitive - either a user defined primitive name from gLTF or a generated name from index

Topology to be rendered.

Material to apply to the mesh.

Additional data of the material.

Create a primitive extracting name and index from glTF def

Subasset label for this primitive within its parent GltfMesh within the gLTF parent asset.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GltfPrimitive {
    pub index: usize,
    pub parent_mesh_index: usize,
    pub name: String,
    pub mesh: Handle<Mesh>,
    pub material: Option<Handle<StandardMaterial>>,
    pub extras: Option<GltfExtras>,
    pub material_extras: Option<GltfExtras>,
}
```

---

## Struct MeshBindGroupPair Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshBindGroupPair.html

**Contents:**
- Struct MeshBindGroupPair Copy item path
- Auto Trait Implementations§
  - impl Freeze for MeshBindGroupPair
  - impl !RefUnwindSafe for MeshBindGroupPair
  - impl Send for MeshBindGroupPair
  - impl Sync for MeshBindGroupPair
  - impl Unpin for MeshBindGroupPair
  - impl !UnwindSafe for MeshBindGroupPair
- Blanket Implementations§
  - impl<T> Any for Twhere T: 'static + ?Sized,

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshBindGroupPair { /* private fields */ }
```

---

## Struct Triangle2dMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.Triangle2dMeshBuilder.html

**Contents:**
- Struct Triangle2dMeshBuilder Copy item path
- Implementations§
  - impl Triangle2dMeshBuilder
    - pub const fn new(a: Vec2, b: Vec2, c: Vec2) -> Triangle2dMeshBuilder
- Trait Implementations§
  - impl Clone for Triangle2dMeshBuilder
    - fn clone(&self) -> Triangle2dMeshBuilder
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for Triangle2dMeshBuilder
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

A builder used for creating a Mesh with a Triangle2d shape.

Creates a new Triangle2dMeshBuilder from the points a, b, and c.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Triangle2dMeshBuilder { /* private fields */ }
```

---

## Struct ExtrusionBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.ExtrusionBuilder.html

**Contents:**
- Struct ExtrusionBuilder Copy item path
- Fields§
- Implementations§
  - impl<P> ExtrusionBuilder<P>where P: Primitive2d + Meshable, <P as Meshable>::Output: Extrudable,
    - pub fn new(base_shape: &P, depth: f32) -> ExtrusionBuilder<P>
    - pub fn segments(self, segments: usize) -> ExtrusionBuilder<P>
  - impl ExtrusionBuilder<Circle>
    - pub fn resolution(self, resolution: u32) -> ExtrusionBuilder<Circle>
  - impl ExtrusionBuilder<Ellipse>
    - pub fn resolution(self, resolution: u32) -> ExtrusionBuilder<Ellipse>

A builder used for creating a Mesh with an Extrusion shape.

Create a new ExtrusionBuilder<P> from a given base_shape and the full depth of the extrusion.

Sets the number of segments along the depth of the extrusion. Must be greater than 0 for the geometry of the mantel to be generated.

Sets the number of vertices used for the circle mesh at each end of the extrusion.

Sets the number of vertices used for the ellipse mesh at each end of the extrusion.

Sets the number of vertices used in constructing the concentric circles of the annulus mesh at each end of the extrusion.

Sets the number of vertices used for each hemicircle at the ends of the extrusion.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ExtrusionBuilder<P>where
    P: Primitive2d + Meshable,
    <P as Meshable>::Output: Extrudable,{
    pub base_builder: <P as Meshable>::Output,
    pub half_depth: f32,
    pub segments: usize,
}
```

---

## Enum PhasePreprocessBindGroups Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/enum.PhasePreprocessBindGroups.html

**Contents:**
- Enum PhasePreprocessBindGroups Copy item path
- Variants§
  - Direct(BindGroup)
  - IndirectFrustumCulling
    - Fields
  - IndirectOcclusionCulling
    - Fields
- Trait Implementations§
  - impl Clone for PhasePreprocessBindGroups
    - fn clone(&self) -> PhasePreprocessBindGroups

The compute shader bind group for the mesh preprocessing step for a single render phase on a single view.

The bind group used for the single invocation of the compute shader when indirect drawing is not being used.

Because direct drawing doesn’t require splitting the meshes into indexed and non-indexed meshes, there’s only one bind group in this case.

The bind groups used for the compute shader when indirect drawing is being used, but occlusion culling isn’t being used.

Because indirect drawing requires splitting the meshes into indexed and non-indexed meshes, there are two bind groups here.

The bind group for indexed meshes.

The bind group for non-indexed meshes.

The bind groups used for the compute shader when indirect drawing is being used, but occlusion culling isn’t being used.

Because indirect drawing requires splitting the meshes into indexed and non-indexed meshes, and because occlusion culling requires splitting this phase into early and late versions, there are four bind groups here.

The bind group for indexed meshes during the early mesh preprocessing phase.

The bind group for non-indexed meshes during the early mesh preprocessing phase.

The bind group for indexed meshes during the late mesh preprocessing phase.

The bind group for non-indexed meshes during the late mesh preprocessing phase.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum PhasePreprocessBindGroups {
    Direct(BindGroup),
    IndirectFrustumCulling {
        indexed: Option<BindGroup>,
        non_indexed: Option<BindGroup>,
    },
    IndirectOcclusionCulling {
        early_indexed: Option<BindGroup>,
        early_non_indexed: Option<BindGroup>,
        late_indexed: Option<BindGroup>,
        late_non_indexed: Option<BindGroup>,
    },
}
```

---

## Struct PreprocessPipelines Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PreprocessPipelines.html

**Contents:**
- Struct PreprocessPipelines Copy item path
- Fields§
- Trait Implementations§
  - impl FromWorld for PreprocessPipelines
    - fn from_world(world: &mut World) -> PreprocessPipelines
  - impl Resource for PreprocessPipelineswhere PreprocessPipelines: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for PreprocessPipelines
  - impl !RefUnwindSafe for PreprocessPipelines
  - impl Send for PreprocessPipelines

The compute shader pipelines for the GPU mesh preprocessing and indirect parameter building passes.

The pipeline used for CPU culling. This pipeline doesn’t populate indirect parameter metadata.

The pipeline used for mesh preprocessing when GPU frustum culling is in use, but occlusion culling isn’t.

This pipeline populates indirect parameter metadata.

The pipeline used for the first phase of occlusion culling.

This pipeline culls, transforms meshes, and populates indirect parameter metadata.

The pipeline used for the second phase of occlusion culling.

This pipeline culls, transforms meshes, and populates indirect parameter metadata.

The pipeline that builds indirect draw parameters for indexed meshes, when frustum culling is enabled but occlusion culling isn’t enabled.

The pipeline that builds indirect draw parameters for non-indexed meshes, when frustum culling is enabled but occlusion culling isn’t enabled.

Compute shader pipelines for the early prepass phase that draws meshes visible in the previous frame.

Compute shader pipelines for the late prepass phase that draws meshes that weren’t visible in the previous frame, but became visible this frame.

Compute shader pipelines for the main color phase.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PreprocessPipelines {
    pub direct_preprocess: PreprocessPipeline,
    pub gpu_frustum_culling_preprocess: PreprocessPipeline,
    pub early_gpu_occlusion_culling_preprocess: PreprocessPipeline,
    pub late_gpu_occlusion_culling_preprocess: PreprocessPipeline,
    pub gpu_frustum_culling_build_indexed_indirect_params: BuildIndirectParametersPipeline,
    pub gpu_frustum_culling_build_non_indexed_indirect_params: BuildIndirectParametersPipeline,
    pub early_phase: PreprocessPhasePipelines,
    pub late_phase: PreprocessPhasePipelines,
    pub main_phase: PreprocessPhasePipelines
...
```

---

## Struct MaterialBindGroupNonBindlessAllocator Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialBindGroupNonBindlessAllocator.html

**Contents:**
- Struct MaterialBindGroupNonBindlessAllocator Copy item path
- Auto Trait Implementations§
  - impl Freeze for MaterialBindGroupNonBindlessAllocator
  - impl !RefUnwindSafe for MaterialBindGroupNonBindlessAllocator
  - impl Send for MaterialBindGroupNonBindlessAllocator
  - impl Sync for MaterialBindGroupNonBindlessAllocator
  - impl Unpin for MaterialBindGroupNonBindlessAllocator
  - impl !UnwindSafe for MaterialBindGroupNonBindlessAllocator
- Blanket Implementations§
  - impl<T> Any for Twhere T: 'static + ?Sized,

The allocator that stores bind groups for non-bindless materials.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialBindGroupNonBindlessAllocator { /* private fields */ }
```

---

## Struct ScreenSpaceReflectionsPipeline Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ScreenSpaceReflectionsPipeline.html

**Contents:**
- Struct ScreenSpaceReflectionsPipeline Copy item path
- Trait Implementations§
  - impl SpecializedRenderPipeline for ScreenSpaceReflectionsPipeline
    - type Key = ScreenSpaceReflectionsPipelineKey
    - fn specialize( &self, key: <ScreenSpaceReflectionsPipeline as SpecializedRenderPipeline>::Key, ) -> RenderPipelineDescriptor
  - impl Resource for ScreenSpaceReflectionsPipelinewhere ScreenSpaceReflectionsPipeline: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for ScreenSpaceReflectionsPipeline
  - impl !RefUnwindSafe for ScreenSpaceReflectionsPipeline
  - impl Send for ScreenSpaceReflectionsPipeline

Information relating to the render pipeline for the screen space reflections shader.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ScreenSpaceReflectionsPipeline { /* private fields */ }
```

---

## Struct MaterialProperties Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialProperties.html

**Contents:**
- Struct MaterialProperties Copy item path
- Fields§
- Implementations§
  - impl MaterialProperties
    - pub fn get_shader(&self, label: impl ShaderLabel) -> Option<Handle<Shader>>
    - pub fn add_shader(&mut self, label: impl ShaderLabel, shader: Handle<Shader>)
      - Examples found in repository?
    - pub fn get_draw_function( &self, label: impl DrawFunctionLabel, ) -> Option<DrawFunctionId>
    - pub fn add_draw_function( &mut self, label: impl DrawFunctionLabel, draw_function: DrawFunctionId, )
      - Examples found in repository?

Common Material properties, calculated for a specific material instance.

Is this material should be rendered by the deferred renderer when. AlphaMode::Opaque or AlphaMode::Mask

The AlphaMode of this material.

The bits in the MeshPipelineKey for this material.

These are precalculated so that we can just “or” them together in queue_material_meshes.

Add a bias to the view depth of the mesh which can be used to force a specific render order for meshes with equal depth, to avoid z-fighting. The bias is in depth-texture units so large values may be needed to overcome small depth differences.

Whether the material would like to read from ViewTransmissionTexture.

This allows taking color output from the Opaque3d pass as an input, (for screen-space transmission) but requires rendering to take place in a separate Transmissive3d pass.

Backing array is a size of 4 because the StandardMaterial needs 4 draw functions by default

Backing array is a size of 3 because the StandardMaterial has 3 custom shaders (frag, prepass_frag, deferred_frag) which is the most common use case

Whether this material actually uses bindless resources, taking the platform support (or lack thereof) of bindless resources into account.

The key for this material, typically a bitfield of flags that are used to modify the pipeline descriptor used for this material.

Whether shadows are enabled for this material

Whether prepass is enabled for this material

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialProperties {Show 14 fields
    pub render_method: OpaqueRendererMethod,
    pub alpha_mode: AlphaMode,
    pub mesh_pipeline_key_bits: MeshPipelineKey,
    pub depth_bias: f32,
    pub reads_view_transmission_texture: bool,
    pub render_phase_type: RenderPhaseType,
    pub material_layout: Option<BindGroupLayout>,
    pub draw_functions: SmallVec<[(Interned<dyn DrawFunctionLabel>, DrawFunctionId); 4]>,
    pub shaders: SmallVec<[(Interned<dyn ShaderLabel>, Handle<Shader>); 3]>,
    pub bindless: bool,
    pub specialize: Option<fn(&MaterialPipeline, &mut RenderPipelineDesc
...
```

Example 2 (javascript):
```javascript
135    fn prepare_asset(
136        source_asset: Self::SourceAsset,
137        asset_id: AssetId<Self::SourceAsset>,
138        (
139            opaque_draw_functions,
140            material_layout,
141            asset_server,
142            bind_group_allocators,
143            render_material_bindings,
144            gpu_images,
145            image_material_sampler,
146        ): &mut SystemParamItem<Self::Param>,
147    ) -> std::result::Result<Self::ErasedAsset, PrepareAssetError<Self::SourceAsset>> {
148        let material_layout = material_layout.0.clone();
149        let draw_funct
...
```

Example 3 (javascript):
```javascript
135    fn prepare_asset(
136        source_asset: Self::SourceAsset,
137        asset_id: AssetId<Self::SourceAsset>,
138        (
139            opaque_draw_functions,
140            material_layout,
141            asset_server,
142            bind_group_allocators,
143            render_material_bindings,
144            gpu_images,
145            image_material_sampler,
146        ): &mut SystemParamItem<Self::Param>,
147    ) -> std::result::Result<Self::ErasedAsset, PrepareAssetError<Self::SourceAsset>> {
148        let material_layout = material_layout.0.clone();
149        let draw_funct
...
```

---

## Module gpu_readback Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/gpu_readback/index.html

**Contents:**
- Module gpu_readback Copy item path
- Structs§
- Enums§

bevy::renderModule gpu_readback Copy item pathSource Structs§GpuReadbackPluginA plugin that enables reading back gpu buffers and textures to the cpu.ReadbackCompleteAn event that is triggered when a gpu readback is complete.Enums§ReadbackA component that registers the wrapped handle for gpu readback, either a texture or a buffer.

---

## Function ktx2_dfd_header_to_texture_format Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/fn.ktx2_dfd_header_to_texture_format.html

**Contents:**
- Function ktx2_dfd_header_to_texture_format Copy item path

bevy::imageFunction ktx2_dfd_header_to_texture_format Copy item pathSource pub fn ktx2_dfd_header_to_texture_format( data_format_descriptor: &DfdBlockHeaderBasic, sample_information: &[SampleInformation], is_srgb: bool, ) -> Result<TextureFormat, TextureError>

**Examples:**

Example 1 (unknown):
```unknown
pub fn ktx2_dfd_header_to_texture_format(
    data_format_descriptor: &DfdBlockHeaderBasic,
    sample_information: &[SampleInformation],
    is_srgb: bool,
) -> Result<TextureFormat, TextureError>
```

---

## Struct ShadowBatchSetKey Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ShadowBatchSetKey.html

**Contents:**
- Struct ShadowBatchSetKey Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for ShadowBatchSetKey
    - fn clone(&self) -> ShadowBatchSetKey
    - fn clone_from(&mut self, source: &Self)
  - impl Hash for ShadowBatchSetKey
    - fn hash<__H>(&self, state: &mut __H)where __H: Hasher,
    - fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,
  - impl Ord for ShadowBatchSetKey

Information that must be identical in order to place opaque meshes in the same batch set.

A batch set is a set of batches that can be multi-drawn together, if multi-draw is in use.

The identifier of the render pipeline.

The function used to draw.

The ID of a bind group specific to the material.

In the case of PBR, this is the MaterialBindGroupIndex.

The ID of the slab of GPU memory that contains vertex data.

For non-mesh items, you can fill this with 0 if your items can be multi-drawn, or with a unique value if they can’t.

The ID of the slab of GPU memory that contains index data, if present.

For non-mesh items, you can safely fill this with None.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ShadowBatchSetKey {
    pub pipeline: CachedRenderPipelineId,
    pub draw_function: DrawFunctionId,
    pub material_bind_group_index: Option<u32>,
    pub vertex_slab: SlabId,
    pub index_slab: Option<SlabId>,
}
```

---

## Struct MeshDeserializer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.MeshDeserializer.html

**Contents:**
- Struct MeshDeserializer Copy item path
- Implementations§
  - impl MeshDeserializer
    - pub fn new() -> MeshDeserializer
    - pub fn add_custom_vertex_attribute( &mut self, name: &str, attribute: MeshVertexAttribute, ) -> &mut MeshDeserializer
    - pub fn deserialize(&self, serialized_mesh: SerializedMesh) -> Mesh
- Trait Implementations§
  - impl Default for MeshDeserializer
    - fn default() -> MeshDeserializer
- Auto Trait Implementations§

Use to specify extra options when deserializing a SerializedMesh into a Mesh.

Create a new MeshDeserializer.

Register a custom vertex attribute to the deserializer. Custom vertex attributes that were not added with this method will be ignored while deserializing.

Deserialize a SerializedMesh into a Mesh.

See the documentation for SerializedMesh for caveats.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshDeserializer { /* private fields */ }
```

---

## Struct RegularPolygonMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.RegularPolygonMeshBuilder.html

**Contents:**
- Struct RegularPolygonMeshBuilder Copy item path
- Implementations§
  - impl RegularPolygonMeshBuilder
    - pub const fn new(circumradius: f32, sides: u32) -> RegularPolygonMeshBuilder
      - §Panics
- Trait Implementations§
  - impl Clone for RegularPolygonMeshBuilder
    - fn clone(&self) -> RegularPolygonMeshBuilder
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for RegularPolygonMeshBuilder

A builder used for creating a Mesh with a RegularPolygon shape.

Creates a new RegularPolygonMeshBuilder from the radius of a circumcircle and a number of sides.

Panics in debug mode if circumradius is negative, or if sides is less than 3.

Returns the default RegularPolygonMeshBuilder with six sides (a hexagon) and a circumradius of 0.5.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RegularPolygonMeshBuilder { /* private fields */ }
```

---

## Struct CircularSectorMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.CircularSectorMeshBuilder.html

**Contents:**
- Struct CircularSectorMeshBuilder Copy item path
- Fields§
- Implementations§
  - impl CircularSectorMeshBuilder
    - pub fn new(sector: CircularSector) -> CircularSectorMeshBuilder
      - Examples found in repository?
    - pub const fn resolution(self, resolution: u32) -> CircularSectorMeshBuilder
    - pub const fn uv_mode( self, uv_mode: CircularMeshUvMode, ) -> CircularSectorMeshBuilder
      - Examples found in repository?
- Trait Implementations§

A builder used for creating a Mesh with a CircularSector shape.

The resulting mesh will have a UV-map such that the center of the circle is at the center of the texture.

The number of vertices used for the arc portion of the sector mesh. The default is 32.

Creates a new CircularSectorMeshBuilder from a given sector

Sets the number of vertices used for the sector mesh.

Sets the uv mode used for the sector mesh

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CircularSectorMeshBuilder {
    pub sector: CircularSector,
    pub resolution: u32,
    pub uv_mode: CircularMeshUvMode,
}
```

Example 2 (javascript):
```javascript
34fn setup(
35    mut commands: Commands,
36    asset_server: Res<AssetServer>,
37    mut meshes: ResMut<Assets<Mesh>>,
38    mut materials: ResMut<Assets<ColorMaterial>>,
39) {
40    let material = materials.add(asset_server.load("branding/icon.png"));
41
42    commands.spawn((
43        Camera2d,
44        Camera {
45            clear_color: ClearColorConfig::Custom(GRAY.into()),
46            ..default()
47        },
48    ));
49
50    const NUM_SLICES: i32 = 8;
51    const SPACING_X: f32 = 100.0;
52    const OFFSET_X: f32 = SPACING_X * (NUM_SLICES - 1) as f32 / 2.0;
53
54    // This draws 
...
```

Example 3 (javascript):
```javascript
34fn setup(
35    mut commands: Commands,
36    asset_server: Res<AssetServer>,
37    mut meshes: ResMut<Assets<Mesh>>,
38    mut materials: ResMut<Assets<ColorMaterial>>,
39) {
40    let material = materials.add(asset_server.load("branding/icon.png"));
41
42    commands.spawn((
43        Camera2d,
44        Camera {
45            clear_color: ClearColorConfig::Custom(GRAY.into()),
46            ..default()
47        },
48    ));
49
50    const NUM_SLICES: i32 = 8;
51    const SPACING_X: f32 = 100.0;
52    const OFFSET_X: f32 = SPACING_X * (NUM_SLICES - 1) as f32 / 2.0;
53
54    // This draws 
...
```

---

## Struct TextureAtlasPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.TextureAtlasPlugin.html

**Contents:**
- Struct TextureAtlasPlugin Copy item path
- Trait Implementations§
  - impl Plugin for TextureAtlasPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)
    - fn name(&self) -> &str
    - fn is_unique(&self) -> bool
- Auto Trait Implementations§

Adds support for texture atlases.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct TextureAtlasPlugin;
```

---

## Enum VertexAttributeValues Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/enum.VertexAttributeValues.html

**Contents:**
- Enum VertexAttributeValues Copy item path
- Variants§
  - Float32(Vec<f32>)
  - Sint32(Vec<i32>)
  - Uint32(Vec<u32>)
  - Float32x2(Vec<[f32; 2]>)
  - Sint32x2(Vec<[i32; 2]>)
  - Uint32x2(Vec<[u32; 2]>)
  - Float32x3(Vec<[f32; 3]>)
  - Sint32x3(Vec<[i32; 3]>)

Contains an array where each entry describes a property of a single vertex. Matches the VertexFormats.

Returns the number of vertices in this VertexAttributeValues. For a single mesh, all of the VertexAttributeValues must have the same length.

Returns true if there are no vertices in this VertexAttributeValues.

Returns the values as float triples if possible.

Flattens the VertexAttributeValues into a sequence of bytes. This is useful for serialization and sending to the GPU.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum VertexAttributeValues {
Show 28 variants    Float32(Vec<f32>),
    Sint32(Vec<i32>),
    Uint32(Vec<u32>),
    Float32x2(Vec<[f32; 2]>),
    Sint32x2(Vec<[i32; 2]>),
    Uint32x2(Vec<[u32; 2]>),
    Float32x3(Vec<[f32; 3]>),
    Sint32x3(Vec<[i32; 3]>),
    Uint32x3(Vec<[u32; 3]>),
    Float32x4(Vec<[f32; 4]>),
    Sint32x4(Vec<[i32; 4]>),
    Uint32x4(Vec<[u32; 4]>),
    Sint16x2(Vec<[i16; 2]>),
    Snorm16x2(Vec<[i16; 2]>),
    Uint16x2(Vec<[u16; 2]>),
    Unorm16x2(Vec<[u16; 2]>),
    Sint16x4(Vec<[i16; 4]>),
    Snorm16x4(Vec<[i16; 4]>),
    Uint16x4(Vec<[u16; 4]>),
    Unorm16x4(
...
```

Example 2 (javascript):
```javascript
281fn spawn_small_cubes(
282    commands: &mut Commands,
283    meshes: &mut Assets<Mesh>,
284    materials: &mut Assets<StandardMaterial>,
285) {
286    // Add the cube mesh.
287    let small_cube = meshes.add(Cuboid::new(
288        SMALL_CUBE_SIZE,
289        SMALL_CUBE_SIZE,
290        SMALL_CUBE_SIZE,
291    ));
292
293    // Add the cube material.
294    let small_cube_material = materials.add(StandardMaterial {
295        base_color: SILVER.into(),
296        ..default()
297    });
298
299    // Create the entity that the small cubes will be parented to. This is the
300    // entity tha
...
```

---

## Struct ConeMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.ConeMeshBuilder.html

**Contents:**
- Struct ConeMeshBuilder Copy item path
- Fields§
- Implementations§
  - impl ConeMeshBuilder
    - pub const fn new(radius: f32, height: f32, resolution: u32) -> ConeMeshBuilder
    - pub const fn resolution(self, resolution: u32) -> ConeMeshBuilder
    - pub const fn anchor(self, anchor: ConeAnchor) -> ConeMeshBuilder
- Trait Implementations§
  - impl Clone for ConeMeshBuilder
    - fn clone(&self) -> ConeMeshBuilder

A builder used for creating a Mesh with a Cone shape.

The number of vertices used for the base of the cone.

The anchor point for the cone mesh, defaults to the midpoint between the tip of the cone and the center of its base

Creates a new ConeMeshBuilder from a given radius, height, and number of vertices used for the base of the cone.

Sets the number of vertices used for the base of the cone.

Sets a custom anchor point for the mesh

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ConeMeshBuilder {
    pub cone: Cone,
    pub resolution: u32,
    pub anchor: ConeAnchor,
}
```

---

## Struct PrepassPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PrepassPlugin.html

**Contents:**
- Struct PrepassPlugin Copy item path
- Fields§
- Implementations§
  - impl PrepassPlugin
    - pub fn new(debug_flags: RenderDebugFlags) -> PrepassPlugin
- Trait Implementations§
  - impl Plugin for PrepassPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)

Sets up the prepasses for a material.

This depends on the PrepassPipelinePlugin.

Debugging flags that can optionally be set when constructing the renderer.

Creates a new PrepassPlugin with the given debug flags.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PrepassPlugin {
    pub debug_flags: RenderDebugFlags,
}
```

---

## Struct PlaneMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.PlaneMeshBuilder.html

**Contents:**
- Struct PlaneMeshBuilder Copy item path
- Fields§
- Implementations§
  - impl PlaneMeshBuilder
    - pub fn new(normal: Dir3, size: Vec2) -> PlaneMeshBuilder
    - pub fn from_size(size: Vec2) -> PlaneMeshBuilder
    - pub fn from_length(length: f32) -> PlaneMeshBuilder
    - pub fn normal(self, normal: Dir3) -> PlaneMeshBuilder
      - Examples found in repository?
    - pub fn size(self, width: f32, height: f32) -> PlaneMeshBuilder

A builder used for creating a Mesh with a Plane3d shape.

The number of subdivisions in the mesh.

0 - is the original plane geometry, the 4 points in the XZ plane.

1 - is split by 1 line in the middle of the plane on both the X axis and the Z axis, resulting in a plane with 4 quads / 8 triangles.

2 - is a plane split by 2 lines on both the X and Z axes, subdividing the plane into 3 equal sections along each axis, resulting in a plane with 9 quads / 18 triangles.

Creates a new PlaneMeshBuilder from a given normal and size.

Creates a new PlaneMeshBuilder from the given size, with the normal pointing upwards.

Creates a new PlaneMeshBuilder from the given length, with the normal pointing upwards, and the resulting PlaneMeshBuilder being a square.

Sets the normal of the plane, aka the direction the plane is facing.

Sets the size of the plane mesh.

Sets the subdivisions of the plane mesh.

0 - is the original plane geometry, the 4 points in the XZ plane.

1 - is split by 1 line in the middle of the plane on both the X axis and the Z axis, resulting in a plane with 4 quads / 8 triangles.

2 - is a plane split by 2 lines on both the X and Z axes, subdividing the plane into 3 equal sections along each axis, resulting in a plane with 9 quads / 18 triangles.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PlaneMeshBuilder {
    pub plane: Plane3d,
    pub subdivisions: u32,
}
```

Example 2 (javascript):
```javascript
28fn setup_environment(
29    mut commands: Commands,
30    mut mesh_assets: ResMut<Assets<Mesh>>,
31    mut material_assets: ResMut<Assets<StandardMaterial>>,
32) {
33    let description = "(left to right)\n\
34        0: Normal skinned mesh.\n\
35        1: Mesh asset is missing skinning attributes.\n\
36        2: One joint entity is missing.\n\
37        3: Mesh entity is missing SkinnedMesh component.";
38
39    commands.spawn((
40        Text::new(description),
41        Node {
42            position_type: PositionType::Absolute,
43            top: px(12),
44            left: px(12),
45 
...
```

Example 3 (unknown):
```unknown
45fn setup(
46    mut commands: Commands,
47    mut meshes: ResMut<Assets<Mesh>>,
48    mut materials: ResMut<Assets<StandardMaterial>>,
49) {
50    // plane
51    commands.spawn((
52        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
53        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
54        Ground,
55    ));
56
57    // light
58    commands.spawn((
59        DirectionalLight::default(),
60        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
61    ));
62
63    // camera
64    commands.spawn((
65        Camera3d::default(),
66   
...
```

Example 4 (javascript):
```javascript
185fn setup_scene(
186    mut commands: Commands,
187    mut meshes: ResMut<Assets<Mesh>>,
188    mut materials: ResMut<Assets<StandardMaterial>>,
189) {
190    // Camera
191    commands.spawn((
192        Camera3d::default(),
193        Transform::from_xyz(10.0, 10.0, 15.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
194    ));
195
196    // Light
197    commands.spawn((
198        DirectionalLight {
199            shadows_enabled: true,
200            ..default()
201        },
202        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
203    ));
204
205    /
...
```

---

## Struct TextureAtlasBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.TextureAtlasBuilder.html

**Contents:**
- Struct TextureAtlasBuilder Copy item path
- Implementations§
  - impl<'a> TextureAtlasBuilder<'a>
    - pub fn initial_size(&mut self, size: UVec2) -> &mut TextureAtlasBuilder<'a>
    - pub fn max_size(&mut self, size: UVec2) -> &mut TextureAtlasBuilder<'a>
    - pub fn format(&mut self, format: TextureFormat) -> &mut TextureAtlasBuilder<'a>
    - pub fn auto_format_conversion( &mut self, auto_format_conversion: bool, ) -> &mut TextureAtlasBuilder<'a>
    - pub fn add_texture( &mut self, image_id: Option<AssetId<Image>>, texture: &'a Image, ) -> &mut TextureAtlasBuilder<'a>
      - Examples found in repository?
    - pub fn padding(&mut self, padding: UVec2) -> &mut TextureAtlasBuilder<'a>

A builder which is used to create a texture atlas from many individual sprites.

Sets the initial size of the atlas in pixels.

Sets the max size of the atlas in pixels.

Sets the texture format for textures in the atlas.

Control whether the added texture should be converted to the atlas format, if different.

Adds a texture to be copied to the texture atlas.

Optionally an asset id can be passed that can later be used with the texture layout to retrieve the index of this texture. The insertion order will reflect the index of the added texture in the finished texture atlas.

Sets the amount of padding in pixels to add between the textures in the texture atlas.

The x value provide will be added to the right edge, while the y value will be added to the bottom edge.

Consumes the builder, and returns the newly created texture atlas and the associated atlas layout.

Assigns indices to the textures based on the insertion order. Internally it copies all rectangles from the textures and copies them into a new texture.

If there is not enough space in the atlas texture, an error will be returned. It is then recommended to make a larger sprite sheet.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct TextureAtlasBuilder<'a> { /* private fields */ }
```

Example 2 (javascript):
```javascript
217fn create_texture_atlas(
218    folder: &LoadedFolder,
219    padding: Option<UVec2>,
220    sampling: Option<ImageSampler>,
221    textures: &mut ResMut<Assets<Image>>,
222) -> (TextureAtlasLayout, TextureAtlasSources, Handle<Image>) {
223    // Build a texture atlas using the individual sprites
224    let mut texture_atlas_builder = TextureAtlasBuilder::default();
225    texture_atlas_builder.padding(padding.unwrap_or_default());
226    for handle in folder.handles.iter() {
227        let id = handle.id().typed_unchecked::<Image>();
228        let Some(texture) = textures.get(id) else {
2
...
```

Example 3 (javascript):
```javascript
217fn create_texture_atlas(
218    folder: &LoadedFolder,
219    padding: Option<UVec2>,
220    sampling: Option<ImageSampler>,
221    textures: &mut ResMut<Assets<Image>>,
222) -> (TextureAtlasLayout, TextureAtlasSources, Handle<Image>) {
223    // Build a texture atlas using the individual sprites
224    let mut texture_atlas_builder = TextureAtlasBuilder::default();
225    texture_atlas_builder.padding(padding.unwrap_or_default());
226    for handle in folder.handles.iter() {
227        let id = handle.id().typed_unchecked::<Image>();
228        let Some(texture) = textures.get(id) else {
2
...
```

Example 4 (javascript):
```javascript
fn my_system(mut textures: ResMut<Assets<Image>>, mut layouts: ResMut<Assets<TextureAtlasLayout>>) {
    // Declare your builder
    let mut builder = TextureAtlasBuilder::default();
    // Customize it
    // ...
    // Build your texture and the atlas layout
    let (atlas_layout, atlas_sources, texture) = builder.build().unwrap();
    let texture = textures.add(texture);
    let layout = layouts.add(atlas_layout);
}
```

---

## Struct MeshletFragmentShader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshletFragmentShader.html

**Contents:**
- Struct MeshletFragmentShader Copy item path
- Trait Implementations§
  - impl Clone for MeshletFragmentShader
    - fn clone(&self) -> MeshletFragmentShader
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for MeshletFragmentShader
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for MeshletFragmentShader
    - fn default() -> MeshletFragmentShader
  - impl Hash for MeshletFragmentShader

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshletFragmentShader;
```

---

## Struct RenderMeshInstanceGpuQueues Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.RenderMeshInstanceGpuQueues.html

**Contents:**
- Struct RenderMeshInstanceGpuQueues Copy item path
- Methods from Deref<Target = Parallel<RenderMeshInstanceGpuQueue>>§
    - pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T>
    - pub fn clear(&mut self)
    - pub fn scope_or<R>( &self, create: impl FnOnce() -> T, f: impl FnOnce(&mut T) -> R, ) -> R
    - pub fn borrow_local_mut_or(&self, create: impl FnOnce() -> T) -> impl DerefMut
    - pub fn scope<R>(&self, f: impl FnOnce(&mut T) -> R) -> R
    - pub fn borrow_local_mut(&self) -> impl DerefMut
      - Examples found in repository?
    - pub fn drain(&mut self) -> impl Iterator<Item = T>

The per-thread queues containing mesh instances, populated during the extract phase.

These are filled in extract_meshes_for_gpu_building and consumed in collect_meshes_for_gpu_building.

Gets a mutable iterator over all of the per-thread queues.

Clears all of the stored thread local values.

Retrieves the thread-local value for the current thread and runs f on it.

If there is no thread-local value, it will be initialized to the result of create.

Mutably borrows the thread-local value.

If there is no thread-local value, it will be initialized to the result of create.

Retrieves the thread-local value for the current thread and runs f on it.

If there is no thread-local value, it will be initialized to its default.

Mutably borrows the thread-local value.

If there is no thread-local value, it will be initialized to its default.

Drains all enqueued items from all threads and returns an iterator over them.

Unlike Vec::drain, this will piecemeal remove chunks of the data stored. If iteration is terminated part way, the rest of the enqueued items in the same chunk will be dropped, and the rest of the undrained elements will remain.

The ordering is not guaranteed.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderMeshInstanceGpuQueues(/* private fields */);
```

Example 2 (unknown):
```unknown
258fn check_entities_needing_specialization(
259    needs_specialization: Query<
260        Entity,
261        (
262            Or<(
263                Changed<Mesh3d>,
264                AssetChanged<Mesh3d>,
265                Changed<ImageMaterial3d>,
266                AssetChanged<ImageMaterial3d>,
267            )>,
268            With<ImageMaterial3d>,
269        ),
270    >,
271    mut par_local: Local<Parallel<Vec<Entity>>>,
272    mut entities_needing_specialization: ResMut<EntitiesNeedingSpecialization<ImageMaterial>>,
273) {
274    entities_needing_specialization.clear();
275
276  
...
```

---

## Struct DynamicTextureAtlasBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.DynamicTextureAtlasBuilder.html

**Contents:**
- Struct DynamicTextureAtlasBuilder Copy item path
- Implementations§
  - impl DynamicTextureAtlasBuilder
    - pub fn new(size: UVec2, padding: u32) -> DynamicTextureAtlasBuilder
      - §Arguments
    - pub fn add_texture( &mut self, atlas_layout: &mut TextureAtlasLayout, texture: &Image, atlas_texture: &mut Image, ) -> Result<usize, DynamicTextureAtlasBuilderError>
      - §Arguments
- Auto Trait Implementations§
  - impl Freeze for DynamicTextureAtlasBuilder
  - impl RefUnwindSafe for DynamicTextureAtlasBuilder

Helper utility to update TextureAtlasLayout on the fly.

Helpful in cases when texture is created procedurally, e.g: in a font glyph TextureAtlasLayout, only add the Image texture for letters to be rendered.

Create a new DynamicTextureAtlasBuilder

Add a new texture to atlas_layout.

It is the user’s responsibility to pass in the correct TextureAtlasLayout. Also, the asset that atlas_texture_handle points to must have a usage matching RenderAssetUsages::MAIN_WORLD.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DynamicTextureAtlasBuilder { /* private fields */ }
```

---

## Struct FileTextureError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.FileTextureError.html

**Contents:**
- Struct FileTextureError Copy item path
- Trait Implementations§
  - impl Debug for FileTextureError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for FileTextureError
    - fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>
  - impl Error for FileTextureError
    - fn source(&self) -> Option<&(dyn Error + 'static)>
    - fn description(&self) -> &str
    - fn cause(&self) -> Option<&dyn Error>

An error that occurs when loading a texture from a file.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct FileTextureError { /* private fields */ }
```

---

## Struct MeshPipelineViewLayouts Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshPipelineViewLayouts.html

**Contents:**
- Struct MeshPipelineViewLayouts Copy item path
- Tuple Fields§
- Implementations§
  - impl MeshPipelineViewLayouts
    - pub fn get_view_layout( &self, layout_key: MeshPipelineViewLayoutKey, ) -> &MeshPipelineViewLayout
- Trait Implementations§
  - impl Clone for MeshPipelineViewLayouts
    - fn clone(&self) -> MeshPipelineViewLayouts
    - fn clone_from(&mut self, source: &Self)
  - impl Deref for MeshPipelineViewLayouts

Stores the view layouts for every combination of pipeline keys.

This is wrapped in an Arc so that it can be efficiently cloned and placed inside specializable pipeline types.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshPipelineViewLayouts(pub Arc<[MeshPipelineViewLayout; 64]>);
```

---

## Struct MeshletMeshLoader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/experimental/meshlet/struct.MeshletMeshLoader.html

**Contents:**
- Struct MeshletMeshLoader Copy item path
- Trait Implementations§
  - impl AssetLoader for MeshletMeshLoader
    - type Asset = MeshletMesh
    - type Settings = ()
    - type Error = MeshletMeshSaveOrLoadError
    - async fn load( &self, reader: &mut dyn Reader, _settings: &(), _load_context: &mut LoadContext<'_>, ) -> Result<MeshletMesh, MeshletMeshSaveOrLoadError>
    - fn extensions(&self) -> &[&str]
- Auto Trait Implementations§
  - impl Freeze for MeshletMeshLoader

An AssetLoader for .meshlet_mesh MeshletMesh assets.

Processes the asset in an asynchronous closure.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshletMeshLoader;
```

---

## Struct TextureAtlas Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.TextureAtlas.html

**Contents:**
- Struct TextureAtlas Copy item path
- Fields§
- Implementations§
  - impl TextureAtlas
    - pub fn texture_rect( &self, texture_atlases: &Assets<TextureAtlasLayout>, ) -> Option<URect>
    - pub fn with_index(self, index: usize) -> TextureAtlas
    - pub fn with_layout(self, layout: Handle<TextureAtlasLayout>) -> TextureAtlas
- Trait Implementations§
  - impl Clone for TextureAtlas
    - fn clone(&self) -> TextureAtlas

An index into a TextureAtlasLayout, which corresponds to a specific section of a texture.

It stores a handle to TextureAtlasLayout and the index of the current section of the atlas. The texture atlas contains various sections of a given texture, allowing users to have a single image file for either sprite animation or global mapping. You can change the texture index of the atlas to animate the sprite or display only a section of the texture for efficient rendering of related game objects.

Check the following examples for usage:

Texture atlas layout handle

Texture atlas section index

Retrieves the current texture URect of the sprite sheet according to the section index

Returns this TextureAtlas with the specified index.

Returns this TextureAtlas with the specified TextureAtlasLayout handle.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct TextureAtlas {
    pub layout: Handle<TextureAtlasLayout>,
    pub index: usize,
}
```

---

## Struct RhombusMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.RhombusMeshBuilder.html

**Contents:**
- Struct RhombusMeshBuilder Copy item path
- Implementations§
  - impl RhombusMeshBuilder
    - pub const fn new( horizontal_diagonal: f32, vertical_diagonal: f32, ) -> RhombusMeshBuilder
      - §Panics
- Trait Implementations§
  - impl Clone for RhombusMeshBuilder
    - fn clone(&self) -> RhombusMeshBuilder
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for RhombusMeshBuilder

A builder for creating a Mesh with an Rhombus shape.

Creates a new RhombusMeshBuilder from a horizontal and vertical diagonal size.

Panics in debug mode if horizontal_diagonal or vertical_diagonal is negative.

Returns the default RhombusMeshBuilder with a half-horizontal and half-vertical diagonal of 0.5.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RhombusMeshBuilder { /* private fields */ }
```

---

## Struct TorusMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.TorusMeshBuilder.html

**Contents:**
- Struct TorusMeshBuilder Copy item path
- Fields§
- Implementations§
  - impl TorusMeshBuilder
    - pub fn new(inner_radius: f32, outer_radius: f32) -> TorusMeshBuilder
    - pub const fn minor_resolution(self, resolution: usize) -> TorusMeshBuilder
    - pub const fn major_resolution(self, resolution: usize) -> TorusMeshBuilder
    - pub const fn angle_range(self, range: RangeInclusive<f32>) -> TorusMeshBuilder
- Trait Implementations§
  - impl Clone for TorusMeshBuilder

A builder used for creating a Mesh with a Torus shape.

The number of vertices used for each circular segment in the ring or tube of the torus.

The number of segments used for the main ring of the torus.

A resolution of 4 would make the torus appear rectangular, while a resolution of 32 resembles a circular ring.

Optional angle range in radians, defaults to a full circle (0.0..=2 * PI)

Creates a new TorusMeshBuilder from an inner and outer radius.

The inner radius is the radius of the hole, and the outer radius is the radius of the entire object.

Sets the number of vertices used for each circular segment in the ring or tube of the torus.

Sets the number of segments used for the main ring of the torus.

A resolution of 4 would make the torus appear rectangular, while a resolution of 32 resembles a circular ring.

Sets a custom angle range in radians instead of a full circle

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct TorusMeshBuilder {
    pub torus: Torus,
    pub minor_resolution: usize,
    pub major_resolution: usize,
    pub angle_range: RangeInclusive<f32>,
}
```

---

## Struct MaterialBindGroupAllocators Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialBindGroupAllocators.html

**Contents:**
- Struct MaterialBindGroupAllocators Copy item path
- Methods from Deref<Target = HashMap<TypeId, MaterialBindGroupAllocator, NoOpHash>>§
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
pub struct MaterialBindGroupAllocators(/* private fields */);
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

## Struct PreparedMaterial Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PreparedMaterial.html

**Contents:**
- Struct PreparedMaterial Copy item path
- Fields§
- Auto Trait Implementations§
  - impl Freeze for PreparedMaterial
  - impl !RefUnwindSafe for PreparedMaterial
  - impl Send for PreparedMaterial
  - impl Sync for PreparedMaterial
  - impl Unpin for PreparedMaterial
  - impl !UnwindSafe for PreparedMaterial
- Blanket Implementations§

Data prepared for a Material instance.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PreparedMaterial {
    pub binding: MaterialBindingId,
    pub properties: Arc<MaterialProperties>,
}
```

---

## Struct PrepassFragmentShader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PrepassFragmentShader.html

**Contents:**
- Struct PrepassFragmentShader Copy item path
- Trait Implementations§
  - impl Clone for PrepassFragmentShader
    - fn clone(&self) -> PrepassFragmentShader
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for PrepassFragmentShader
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for PrepassFragmentShader
    - fn default() -> PrepassFragmentShader
  - impl Hash for PrepassFragmentShader

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PrepassFragmentShader;
```

---

## Struct RenderAdapterInfo Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/renderer/struct.RenderAdapterInfo.html

**Contents:**
- Struct RenderAdapterInfo Copy item path
- Tuple Fields§
- Trait Implementations§
  - impl Clone for RenderAdapterInfo
    - fn clone(&self) -> RenderAdapterInfo
    - fn clone_from(&mut self, source: &Self)
  - impl Deref for RenderAdapterInfo
    - type Target = WgpuWrapper<AdapterInfo>
    - fn deref(&self) -> &<RenderAdapterInfo as Deref>::Target
  - impl DerefMut for RenderAdapterInfo

The AdapterInfo of the adapter in use by the renderer.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderAdapterInfo(pub WgpuWrapper<AdapterInfo>);
```

---

## Trait CameraProjection Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/trait.CameraProjection.html

**Contents:**
- Trait CameraProjection Copy item path
- Required Methods§
    - fn get_clip_from_view(&self) -> Mat4
    - fn get_clip_from_view_for_sub(&self, sub_view: &SubCameraView) -> Mat4
    - fn update(&mut self, width: f32, height: f32)
    - fn far(&self) -> f32
    - fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [Vec3A; 8]
- Provided Methods§
    - fn compute_frustum(&self, camera_transform: &GlobalTransform) -> Frustum
- Implementors§

Describes a type that can generate a projection matrix, allowing it to be added to a Camera’s Projection component.

Once implemented, the projection can be added to a camera using Projection::custom.

The projection will be automatically updated as the render area is resized. This is useful when, for example, a projection type has a field like fov that should change when the window width is changed but not when the height changes.

This trait is implemented by bevy’s built-in projections PerspectiveProjection and OrthographicProjection.

Generate the projection matrix.

Generate the projection matrix for a SubCameraView.

When the area this camera renders to changes dimensions, this method will be automatically called. Use this to update any projection properties that depend on the aspect ratio or dimensions of the render area.

The far plane distance of the projection.

The eight corners of the camera frustum, as defined by this projection.

The corners should be provided in the following order: first the bottom right, top right, top left, bottom left for the near plane, then similar for the far plane.

Compute camera frustum for camera with given projection and transform.

This code is called by update_frusta system for each camera to update its frustum.

**Examples:**

Example 1 (unknown):
```unknown
pub trait CameraProjection {
    // Required methods
    fn get_clip_from_view(&self) -> Mat4;
    fn get_clip_from_view_for_sub(&self, sub_view: &SubCameraView) -> Mat4;
    fn update(&mut self, width: f32, height: f32);
    fn far(&self) -> f32;
    fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [Vec3A; 8];

    // Provided method
    fn compute_frustum(&self, camera_transform: &GlobalTransform) -> Frustum { ... }
}
```

---

## Module render_graph Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/render_graph/index.html

**Contents:**
- Module render_graph Copy item path
- Structs§
- Enums§
- Traits§
- Type Aliases§
- Derive Macros§

bevy::renderModule render_graph Copy item pathSource Structs§CameraDriverNodeEdgesA collection of input and output Edges for a Node.EmptyNodeA Node without any inputs, outputs and subgraphs, which does nothing when run. Used (as a label) to bundle multiple dependencies into one inside the RenderGraph.GraphInputThe label for the input node of a graph. Used to connect other nodes to it.GraphInputNodeA Node which acts as an entry point for a RenderGraph with custom inputs. It has the same input and output slots and simply copies them over when run.NodeStateThe internal representation of a Node, with all data required by the RenderGraph.RenderGraphThe render graph configures the modular and re-usable render logic.RenderGraphContextThe context with all graph information required to run a Node. This context is created for each node by the render graph runner.RunGraphOnViewNodeA RenderGraph Node that runs the configured subgraph once. This makes it easier to insert sub-graph runs into a graph.RunSubGraphA command that signals the graph runner to run the sub graph corresponding to the sub_graph with the specified inputs next.SlotInfoThe internal representation of a slot, which specifies its SlotType and name.SlotInfosA collection of input or output SlotInfos for a NodeState.ViewNodeRunnerThis Node can be used to run any ViewNode. It will take care of updating the view query in update() and running the query in run().Enums§EdgeAn edge, which connects two Nodes in a RenderGraph.EdgeExistenceInputSlotErrorNodeRunErrorOutputSlotErrorRenderGraphErrorRunSubGraphErrorSlotLabelA SlotLabel is used to reference a slot by either its name or index inside the RenderGraph.SlotTypeDescribes the render resources created (output) or used (input) by the render Nodes.SlotValueA value passed between render Nodes. Corresponds to the SlotType specified in the RenderGraph.Traits§DynEqAn object safe version of Eq. This trait is automatically implemented for any 'static type that implements Eq.IntoRenderNodeArrayNodeA render node that can be added to a RenderGraph.RenderGraphExtAdds common RenderGraph operations to SubApp (and App).RenderLabelA strongly-typed class of labels used to identify a Node in a render graph.RenderSubGraphA strongly-typed class of labels used to identify a [SubGraph] in a render graph.ViewNodeThis trait should be used instead of the Node trait when making a render node that runs on a view.Type Aliases§InternedRenderLabelA shorthand for Interned<dyn RenderLabel>.In

*[Content truncated]*

---

## Struct VertexBufferLayout Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.VertexBufferLayout.html

**Contents:**
- Struct VertexBufferLayout Copy item path
- Fields§
- Implementations§
  - impl VertexBufferLayout
    - pub fn from_vertex_formats<T>( step_mode: VertexStepMode, vertex_formats: T, ) -> VertexBufferLayoutwhere T: IntoIterator<Item = VertexFormat>,
      - Examples found in repository?
    - pub fn offset_locations_by(self, location: u32) -> VertexBufferLayout
- Trait Implementations§
  - impl Clone for VertexBufferLayout
    - fn clone(&self) -> VertexBufferLayout

Describes how the vertex buffer is interpreted.

The stride, in bytes, between elements of this buffer.

How often this vertex buffer is “stepped” forward.

The list of attributes which comprise a single vertex.

Creates a new densely packed VertexBufferLayout from an iterator of vertex formats. Iteration order determines the shader_location and offset of the VertexAttributes. The first iterated item will have a shader_location and offset of zero. The array_stride is the sum of the size of the iterated VertexFormats (in bytes).

Returns a VertexBufferLayout with the shader location of every attribute offset by location.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct VertexBufferLayout {
    pub array_stride: u64,
    pub step_mode: VertexStepMode,
    pub attributes: Vec<VertexAttribute>,
}
```

Example 2 (javascript):
```javascript
153    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
154        // Customize how to store the meshes' vertex attributes in the vertex buffer
155        // Our meshes only have position and color
156        let formats = vec![
157            // Position
158            VertexFormat::Float32x3,
159            // Color
160            VertexFormat::Uint32,
161        ];
162
163        let vertex_layout =
164            VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, formats);
165
166        let format = match key.contains(Mesh2dPipelineKey::HDR) {
167           
...
```

---

## Constant TEXTURE_ASSET_INDEX Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/constant.TEXTURE_ASSET_INDEX.html

**Contents:**
- Constant TEXTURE_ASSET_INDEX Copy item path

bevy::imageConstant TEXTURE_ASSET_INDEX Copy item pathSource pub const TEXTURE_ASSET_INDEX: u64 = 0; // 0u64

**Examples:**

Example 1 (javascript):
```javascript
pub const TEXTURE_ASSET_INDEX: u64 = 0; // 0u64
```

---

## Enum TextureFormat Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/render_resource/enum.TextureFormat.html

**Contents:**
- Enum TextureFormat Copy item path
- Variants§
  - R8Unorm
  - R8Snorm
  - R8Uint
  - R8Sint
  - R16Uint
  - R16Sint
  - R16Unorm
  - R16Snorm

Format in which a texture’s texels are stored in GPU memory.

Certain formats additionally specify a conversion. When these formats are used in a shader, the conversion automatically takes place when loading from or storing to the texture.

Corresponds to WebGPU GPUTextureFormat.

Red channel only. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.

Red channel only. 8 bit integer per channel. [−127, 127] converted to/from float [−1, 1] in shader.

Red channel only. 8 bit integer per channel. Unsigned in shader.

Red channel only. 8 bit integer per channel. Signed in shader.

Red channel only. 16 bit integer per channel. Unsigned in shader.

Red channel only. 16 bit integer per channel. Signed in shader.

Red channel only. 16 bit integer per channel. [0, 65535] converted to/from float [0, 1] in shader.

Features::TEXTURE_FORMAT_16BIT_NORM must be enabled to use this texture format.

Red channel only. 16 bit integer per channel. [−32767, 32767] converted to/from float [−1, 1] in shader.

Features::TEXTURE_FORMAT_16BIT_NORM must be enabled to use this texture format.

Red channel only. 16 bit float per channel. Float in shader.

Red and green channels. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.

Red and green channels. 8 bit integer per channel. [−127, 127] converted to/from float [−1, 1] in shader.

Red and green channels. 8 bit integer per channel. Unsigned in shader.

Red and green channels. 8 bit integer per channel. Signed in shader.

Red channel only. 32 bit integer per channel. Unsigned in shader.

Red channel only. 32 bit integer per channel. Signed in shader.

Red channel only. 32 bit float per channel. Float in shader.

Red and green channels. 16 bit integer per channel. Unsigned in shader.

Red and green channels. 16 bit integer per channel. Signed in shader.

Red and green channels. 16 bit integer per channel. [0, 65535] converted to/from float [0, 1] in shader.

Features::TEXTURE_FORMAT_16BIT_NORM must be enabled to use this texture format.

Red and green channels. 16 bit integer per channel. [−32767, 32767] converted to/from float [−1, 1] in shader.

Features::TEXTURE_FORMAT_16BIT_NORM must be enabled to use this texture format.

Red and green channels. 16 bit float per channel. Float in shader.

Red, green, blue, and alpha channels. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.

Red, green, blue, and alpha channels. 8 bit integer per channel. Srgb-col

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub enum TextureFormat {
Show 76 variants    R8Unorm,
    R8Snorm,
    R8Uint,
    R8Sint,
    R16Uint,
    R16Sint,
    R16Unorm,
    R16Snorm,
    R16Float,
    Rg8Unorm,
    Rg8Snorm,
    Rg8Uint,
    Rg8Sint,
    R32Uint,
    R32Sint,
    R32Float,
    Rg16Uint,
    Rg16Sint,
    Rg16Unorm,
    Rg16Snorm,
    Rg16Float,
    Rgba8Unorm,
    Rgba8UnormSrgb,
    Rgba8Snorm,
    Rgba8Uint,
    Rgba8Sint,
    Bgra8Unorm,
    Bgra8UnormSrgb,
    Rgb9e5Ufloat,
    Rgb10a2Uint,
    Rgb10a2Unorm,
    Rg11b10Ufloat,
    R64Uint,
    Rg32Uint,
    Rg32Sint,
    Rg32Float,
    Rgba16Uint,
  
...
```

Example 2 (javascript):
```javascript
339    fn run(
340        &self,
341        _graph: &mut RenderGraphContext,
342        render_context: &mut RenderContext,
343        world: &World,
344    ) -> Result<(), NodeRunError> {
345        let image_copiers = world.get_resource::<ImageCopiers>().unwrap();
346        let gpu_images = world
347            .get_resource::<RenderAssets<bevy::render::texture::GpuImage>>()
348            .unwrap();
349
350        for image_copier in image_copiers.iter() {
351            if !image_copier.enabled() {
352                continue;
353            }
354
355            let src_image = gpu_images
...
```

Example 3 (javascript):
```javascript
339    fn run(
340        &self,
341        _graph: &mut RenderGraphContext,
342        render_context: &mut RenderContext,
343        world: &World,
344    ) -> Result<(), NodeRunError> {
345        let image_copiers = world.get_resource::<ImageCopiers>().unwrap();
346        let gpu_images = world
347            .get_resource::<RenderAssets<bevy::render::texture::GpuImage>>()
348            .unwrap();
349
350        for image_copier in image_copiers.iter() {
351            if !image_copier.enabled() {
352                continue;
353            }
354
355            let src_image = gpu_images
...
```

---

## Struct CuboidMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.CuboidMeshBuilder.html

**Contents:**
- Struct CuboidMeshBuilder Copy item path
- Trait Implementations§
  - impl Clone for CuboidMeshBuilder
    - fn clone(&self) -> CuboidMeshBuilder
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for CuboidMeshBuilder
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for CuboidMeshBuilder
    - fn default() -> CuboidMeshBuilder
  - impl FromArg for CuboidMeshBuilder

A builder used for creating a Mesh with a Cuboid shape.

Returns the default CuboidMeshBuilder with a width, height, and depth of 1.0.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CuboidMeshBuilder { /* private fields */ }
```

---

## Module storage Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/storage/index.html

**Contents:**
- Module storage Copy item path
- Structs§

bevy::renderModule storage Copy item pathSource Structs§GpuShaderStorageBufferA storage buffer that is prepared as a RenderAsset and uploaded to the GPU.ShaderStorageBufferA storage buffer that is prepared as a RenderAsset and uploaded to the GPU.StoragePluginAdds ShaderStorageBuffer as an asset that is extracted and uploaded to the GPU.

---

## Struct MissingVertexAttributeError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.MissingVertexAttributeError.html

**Contents:**
- Struct MissingVertexAttributeError Copy item path
- Fields§
- Trait Implementations§
  - impl Debug for MissingVertexAttributeError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for MissingVertexAttributeError
    - fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>
  - impl Error for MissingVertexAttributeError
    - fn source(&self) -> Option<&(dyn Error + 'static)>
    - fn description(&self) -> &str

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MissingVertexAttributeError {
    pub pipeline_type: Option<&'static str>,
    /* private fields */
}
```

---

## Module diagnostic Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/diagnostic/index.html

**Contents:**
- Module diagnostic Copy item path
- Structs§
- Traits§

Infrastructure for recording render diagnostics.

For more info, see RenderDiagnosticsPlugin.

---

## Struct SetMeshViewBindGroup Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.SetMeshViewBindGroup.html

**Contents:**
- Struct SetMeshViewBindGroup Copy item path
- Trait Implementations§
  - impl<P, const I: usize> RenderCommand<P> for SetMeshViewBindGroup<I>where P: PhaseItem,
    - type Param = ()
    - type ViewQuery = (&'static ViewUniformOffset, &'static ViewLightsUniformOffset, &'static ViewFogUniformOffset, &'static ViewLightProbesUniformOffset, &'static ViewScreenSpaceReflectionsUniformOffset, &'static ViewEnvironmentMapUniformOffset, &'static MeshViewBindGroup, Option<&'static OrderIndependentTransparencySettingsOffset>)
    - type ItemQuery = ()
    - fn render<'w>( _item: &P, _: <<<SetMeshViewBindGroup<I> as RenderCommand<P>>::ViewQuery as QueryData>::ReadOnly as QueryData>::Item<'w, '_>, _entity: Option<()>, _: <<SetMeshViewBindGroup<I> as RenderCommand<P>>::Param as SystemParam>::Item<'w, '_>, pass: &mut TrackedRenderPass<'w>, ) -> RenderCommandResult
- Auto Trait Implementations§
  - impl<const I: usize> Freeze for SetMeshViewBindGroup<I>
  - impl<const I: usize> RefUnwindSafe for SetMeshViewBindGroup<I>

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (javascript):
```javascript
pub struct SetMeshViewBindGroup<const I: usize>;
```

---

## Struct ComputedCameraValues Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.ComputedCameraValues.html

**Contents:**
- Struct ComputedCameraValues Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for ComputedCameraValues
    - fn clone(&self) -> ComputedCameraValues
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for ComputedCameraValues
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for ComputedCameraValues
    - fn default() -> ComputedCameraValues

Holds internally computed Camera values.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ComputedCameraValues {
    pub clip_from_view: Mat4,
    pub target_info: Option<RenderTargetInfo>,
    pub old_viewport_size: Option<UVec2>,
    pub old_sub_camera_view: Option<SubCameraView>,
}
```

---

## Crate core_pipeline Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/core_pipeline/index.html

**Contents:**
- Crate core_pipeline Copy item path
- Modules§
- Structs§

bevyCrate core_pipeline Copy item pathSource Modules§blitcore_2dcore_3ddeferredexperimentalExperimental rendering features.oitOrder Independent Transparency (OIT) for 3d rendering. See OrderIndependentTransparencyPlugin for more details.prepassRun a prepass before the main pass to generate depth, normals, and/or motion vectors textures, sometimes called a thin g-buffer. These textures are useful for various screen-space effects and reducing overdraw in the main pass.tonemappingupscalingStructs§CorePipelinePluginFullscreenShaderA shader that renders to the whole screen. Useful for post-processing.SkyboxAdds a skybox to a 3D camera, based on a cubemap texture.

---

## Struct HdrTextureLoader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.HdrTextureLoader.html

**Contents:**
- Struct HdrTextureLoader Copy item path
- Trait Implementations§
  - impl AssetLoader for HdrTextureLoader
    - type Asset = Image
    - type Settings = HdrTextureLoaderSettings
    - type Error = HdrTextureLoaderError
    - async fn load( &self, reader: &mut dyn Reader, settings: &<HdrTextureLoader as AssetLoader>::Settings, _load_context: &mut LoadContext<'_>, ) -> Result<Image, <HdrTextureLoader as AssetLoader>::Error>
    - fn extensions(&self) -> &[&str]
  - impl Clone for HdrTextureLoader
    - fn clone(&self) -> HdrTextureLoader

Loads HDR textures as Texture assets

Processes the asset in an asynchronous closure.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct HdrTextureLoader;
```

---

## Struct MeshMaterial3d Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshMaterial3d.html

**Contents:**
- Struct MeshMaterial3d Copy item path
- §Example
- Tuple Fields§
- Methods from Deref<Target = Handle<M>>§
    - pub fn id(&self) -> AssetId<A>
      - Examples found in repository?
    - pub fn path(&self) -> Option<&AssetPath<'static>>
    - pub fn is_uuid(&self) -> bool
    - pub fn is_strong(&self) -> bool
- Trait Implementations§

A material used for rendering a Mesh3d.

See Material for general information about 3D materials and how to implement your own materials.

Returns the AssetId of this Asset.

Returns the path if this is (1) a strong handle and (2) the asset has a path

Returns true if this is a uuid handle.

Returns true if this is a strong handle.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshMaterial3d<M>(pub Handle<M>)
where
    M: Material;
```

Example 2 (unknown):
```unknown
// Spawn an entity with a mesh using `StandardMaterial`.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: RED.into(),
            ..Default::default()
        })),
    ));
}
```

Example 3 (javascript):
```javascript
112    fn as_asset_id(&self) -> AssetId<Self::Asset> {
113        self.0.id()
114    }
115}
116
117#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
118struct ImageMaterial {
119    image: Handle<Image>,
120}
121
122impl ErasedRenderAsset for ImageMaterial {
123    type SourceAsset = ImageMaterial;
124    type ErasedAsset = PreparedMaterial;
125    type Param = (
126        SRes<DrawFunctions<Opaque3d>>,
127        SRes<ImageMaterialBindGroupLayout>,
128        SRes<AssetServer>,
129        SResMut<MaterialBindGroupAllocators>,
130        SResMut<RenderMaterialBindings>,
131        SRes<Re
...
```

Example 4 (javascript):
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

---

## Struct CircleMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.CircleMeshBuilder.html

**Contents:**
- Struct CircleMeshBuilder Copy item path
- Fields§
- Implementations§
  - impl CircleMeshBuilder
    - pub const fn new(radius: f32, resolution: u32) -> CircleMeshBuilder
    - pub const fn resolution(self, resolution: u32) -> CircleMeshBuilder
- Trait Implementations§
  - impl Clone for CircleMeshBuilder
    - fn clone(&self) -> CircleMeshBuilder
    - fn clone_from(&mut self, source: &Self)

A builder used for creating a Mesh with a Circle shape.

The number of vertices used for the circle mesh. The default is 32.

Creates a new CircleMeshBuilder from a given radius and vertex count.

Sets the number of vertices used for the circle mesh.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CircleMeshBuilder {
    pub circle: Circle,
    pub resolution: u32,
}
```

---

## Struct ManualTextureView Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/texture/struct.ManualTextureView.html

**Contents:**
- Struct ManualTextureView Copy item path
- Fields§
- Implementations§
  - impl ManualTextureView
    - pub fn with_default_format( texture_view: TextureView, size: UVec2, ) -> ManualTextureView
- Trait Implementations§
  - impl Clone for ManualTextureView
    - fn clone(&self) -> ManualTextureView
    - fn clone_from(&mut self, source: &Self)
  - impl Component for ManualTextureViewwhere ManualTextureView: Send + Sync + 'static,

A manually managed TextureView for use as a bevy_camera::RenderTarget.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ManualTextureView {
    pub texture_view: TextureView,
    pub size: UVec2,
    pub format: TextureFormat,
}
```

---

## Module globals Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/globals/index.html

**Contents:**
- Module globals Copy item path
- Structs§

bevy::renderModule globals Copy item pathSource Structs§GlobalsBufferThe buffer containing the GlobalsUniformGlobalsPluginGlobalsUniformContains global values useful when writing shaders. Currently only contains values related to time.

---

## Crate picking Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/picking/index.html

**Contents:**
- Crate picking Copy item path
  - §Overview
  - §Expressive Events
  - §Modularity
      - §Mix and Match Hit Testing Backends
      - §Input Agnostic
  - §Robustness
- §Getting Started
      - §Next Steps
- §The Picking Pipeline

This crate provides ‘picking’ capabilities for the Bevy game engine, allowing pointers to interact with entities using hover, click, and drag events.

In the simplest case, this plugin allows you to click on things in the scene. However, it also allows you to express more complex interactions, like detecting when a touch input drags a UI element and drops it on a 3d mesh rendered to a different camera.

Pointer events bubble up the entity hierarchy and can be used with observers, allowing you to succinctly express rich interaction behaviors by attaching pointer callbacks to entities:

At its core, this crate provides a robust abstraction for computing picking state regardless of pointing devices, or what you are hit testing against. It is designed to work with any input, including mouse, touch, pens, or virtual pointers controlled by gamepads.

Although the events in this module (see events) can be listened to with normal MessageReaders, using observers is often more expressive, with less boilerplate. This is because observers allow you to attach event handling logic to specific entities, as well as make use of event bubbling.

When events are generated, they bubble up the entity hierarchy starting from their target, until they reach the root or bubbling is halted with a call to On::propagate. See Observer for details.

This allows you to run callbacks when any children of an entity are interacted with, and leads to succinct, expressive code:

The plugin attempts to handle all the hard parts for you, all you need to do is tell it when a pointer is hitting any entities. Multiple backends can be used at the same time! Use this simple API to write your own backend in about 100 lines of code.

Picking provides a generic Pointer abstraction, which is useful for reacting to many different types of input devices. Pointers can be controlled with anything, whether it’s the included mouse or touch inputs, or a custom gamepad input system you write yourself to control a virtual pointer.

In addition to these features, this plugin also correctly handles multitouch, multiple windows, multiple cameras, viewports, and render layers. Using this as a library allows you to write a picking backend that can interoperate with any other picking backend.

TODO: This section will need to be re-written once more backends are introduced.

To learn more, take a look at the examples in the examples. You can read the next section to understand how the plugin works.

This plugin is des

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
world.spawn(MyComponent)
    .observe(|mut event: On<Pointer<Click>>| {
        // Read the underlying pointer event data
        println!("Pointer {:?} was just clicked!", event.pointer_id);
        // Stop the event from bubbling up the entity hierarchy
        event.propagate(false);
    });
```

Example 2 (javascript):
```javascript
fn setup(mut commands: Commands) {
    commands.spawn(Transform::default())
        // Spawn your entity here, e.g. a `Mesh3d`.
        // When dragged, mutate the `Transform` component on the dragged target entity:
        .observe(|drag: On<Pointer<Drag>>, mut transforms: Query<&mut Transform>| {
            let mut transform = transforms.get_mut(drag.entity).unwrap();
            transform.rotate_local_y(drag.delta.x / 50.0);
        })
        .observe(|click: On<Pointer<Click>>, mut commands: Commands| {
            println!("Entity {} goes BOOM!", click.entity);
            commands.enti
...
```

---

## Trait Meshable Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/trait.Meshable.html

**Contents:**
- Trait Meshable Copy item path
- Required Associated Types§
    - type Output: MeshBuilder
- Required Methods§
    - fn mesh(&self) -> Self::Output
- Implementors§
  - impl Meshable for Annulus
    - type Output = AnnulusMeshBuilder
  - impl Meshable for Capsule2d
    - type Output = Capsule2dMeshBuilder

A trait for shapes that can be turned into a Mesh.

The output of Self::mesh. This will be a MeshBuilder used for creating a Mesh.

Creates a Mesh for a shape.

**Examples:**

Example 1 (unknown):
```unknown
pub trait Meshable {
    type Output: MeshBuilder;

    // Required method
    fn mesh(&self) -> Self::Output;
}
```

---

## Struct DirectionalLightTexture Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.DirectionalLightTexture.html

**Contents:**
- Struct DirectionalLightTexture Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for DirectionalLightTexture
    - fn clone(&self) -> DirectionalLightTexture
    - fn clone_from(&mut self, source: &Self)
  - impl Component for DirectionalLightTexturewhere DirectionalLightTexture: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

Add to a DirectionalLight to add a light texture effect. A texture mask is applied to the light source to modulate its intensity, simulating patterns like window shadows, gobo/cookie effects, or soft falloffs.

The texture image. Only the R channel is read.

Whether to tile the image infinitely, or use only a single tile centered at the light’s translation

Required Components: DirectionalLight.

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components of the required components, recursively, in depth-first order.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DirectionalLightTexture {
    pub image: Handle<Image>,
    pub tiled: bool,
}
```

---

## Struct MeshVertexAttribute Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.MeshVertexAttribute.html

**Contents:**
- Struct MeshVertexAttribute Copy item path
- Fields§
- Implementations§
  - impl MeshVertexAttribute
    - pub const fn new( name: &'static str, id: u64, format: VertexFormat, ) -> MeshVertexAttribute
      - Examples found in repository?
    - pub const fn at_shader_location( &self, shader_location: u32, ) -> VertexAttributeDescriptor
      - Examples found in repository?
- Trait Implementations§
  - impl Clone for MeshVertexAttribute

The friendly name of the vertex attribute

The unique id of the vertex attribute. This will also determine sort ordering when generating vertex buffers. Built-in / standard attributes will use “close to zero” indices. When in doubt, use a random / very large u64 to avoid conflicts.

The format of the vertex attribute.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshVertexAttribute {
    pub name: &'static str,
    pub id: MeshVertexAttributeId,
    pub format: VertexFormat,
}
```

Example 2 (javascript):
```javascript
26const ATTRIBUTE_BLEND_COLOR: MeshVertexAttribute =
27    MeshVertexAttribute::new("BlendColor", 988540917, VertexFormat::Float32x4);
```

Example 3 (javascript):
```javascript
21const ATTRIBUTE_BARYCENTRIC: MeshVertexAttribute =
22    MeshVertexAttribute::new("Barycentric", 2137464976, VertexFormat::Float32x3);
```

Example 4 (javascript):
```javascript
49fn star(
50    mut commands: Commands,
51    // We will add a new Mesh for the star being created
52    mut meshes: ResMut<Assets<Mesh>>,
53) {
54    // Let's define the mesh for the object we want to draw: a nice star.
55    // We will specify here what kind of topology is used to define the mesh,
56    // that is, how triangles are built from the vertices. We will use a
57    // triangle list, meaning that each vertex of the triangle has to be
58    // specified. We set `RenderAssetUsages::RENDER_WORLD`, meaning this mesh
59    // will not be accessible in future frames from the `meshes` r
...
```

---

## Enum MeshBindGroups Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/enum.MeshBindGroups.html

**Contents:**
- Enum MeshBindGroups Copy item path
- Variants§
  - CpuPreprocessing(MeshPhaseBindGroups)
  - GpuPreprocessing(HashMap<TypeId, MeshPhaseBindGroups, NoOpHash>)
- Trait Implementations§
  - impl Resource for MeshBindGroupswhere MeshBindGroups: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for MeshBindGroups
  - impl !RefUnwindSafe for MeshBindGroups
  - impl Send for MeshBindGroups

All bind groups for meshes currently loaded.

The bind groups for the meshes for the entire scene, if GPU mesh preprocessing isn’t in use.

A mapping from the type ID of a phase (e.g. Opaque3d) to the mesh bind groups for that phase.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum MeshBindGroups {
    CpuPreprocessing(MeshPhaseBindGroups),
    GpuPreprocessing(HashMap<TypeId, MeshPhaseBindGroups, NoOpHash>),
}
```

---

## Module primitives Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/primitives/index.html

**Contents:**
- Module primitives Copy item path
- Structs§
- Enums§
- Traits§

Mesh generation for primitive shapes.

Primitives that support meshing implement the Meshable trait. Calling mesh will return either a Mesh or a builder that can be used to specify shape-specific configuration for creating the Mesh.

**Examples:**

Example 1 (javascript):
```javascript
// Create circle mesh with default configuration
let circle = meshes.add(Circle { radius: 25.0 });

// Specify number of vertices
let circle = meshes.add(Circle { radius: 25.0 }.mesh().resolution(64));
```

---

## Enum SphereKind Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/enum.SphereKind.html

**Contents:**
- Enum SphereKind Copy item path
- Variants§
  - Ico
    - Fields
  - Uv
    - Fields
- Trait Implementations§
  - impl Clone for SphereKind
    - fn clone(&self) -> SphereKind
    - fn clone_from(&mut self, source: &Self)

A type of sphere mesh.

An icosphere, a spherical mesh that consists of similar sized triangles.

The number of subdivisions applied. The number of faces quadruples with each subdivision.

A UV sphere, a spherical mesh that consists of quadrilaterals apart from triangles at the top and bottom.

The number of longitudinal sectors, aka the horizontal resolution.

The number of latitudinal stacks, aka the vertical resolution.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum SphereKind {
    Ico {
        subdivisions: u32,
    },
    Uv {
        sectors: u32,
        stacks: u32,
    },
}
```

---

## Struct Mesh3d Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.Mesh3d.html

**Contents:**
- Struct Mesh3d Copy item path
- §Example
- Tuple Fields§
- Methods from Deref<Target = Handle<Mesh>>§
    - pub fn id(&self) -> AssetId<A>
      - Examples found in repository?
    - pub fn path(&self) -> Option<&AssetPath<'static>>
    - pub fn is_uuid(&self) -> bool
    - pub fn is_strong(&self) -> bool
- Trait Implementations§

A component for 3D meshes. Requires a MeshMaterial3d to be rendered, commonly using a StandardMaterial.

Returns the AssetId of this Asset.

Returns the path if this is (1) a strong handle and (2) the asset has a path

Returns true if this is a uuid handle.

Returns true if this is a strong handle.

Required Components: Transform.

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components of the required components, recursively, in depth-first order.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Mesh3d(pub Handle<Mesh>);
```

Example 2 (unknown):
```unknown
// Spawn an entity with a mesh using `StandardMaterial`.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: RED.into(),
            ..Default::default()
        })),
    ));
}
```

Example 3 (javascript):
```javascript
112    fn as_asset_id(&self) -> AssetId<Self::Asset> {
113        self.0.id()
114    }
115}
116
117#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
118struct ImageMaterial {
119    image: Handle<Image>,
120}
121
122impl ErasedRenderAsset for ImageMaterial {
123    type SourceAsset = ImageMaterial;
124    type ErasedAsset = PreparedMaterial;
125    type Param = (
126        SRes<DrawFunctions<Opaque3d>>,
127        SRes<ImageMaterialBindGroupLayout>,
128        SRes<AssetServer>,
129        SResMut<MaterialBindGroupAllocators>,
130        SResMut<RenderMaterialBindings>,
131        SRes<Re
...
```

Example 4 (javascript):
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

---

## Struct LateGpuPreprocessNode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.LateGpuPreprocessNode.html

**Contents:**
- Struct LateGpuPreprocessNode Copy item path
- Trait Implementations§
  - impl FromWorld for LateGpuPreprocessNode
    - fn from_world(world: &mut World) -> LateGpuPreprocessNode
  - impl Node for LateGpuPreprocessNode
    - fn update(&mut self, world: &mut World)
    - fn run<'w>( &self, _: &mut RenderGraphContext<'_>, render_context: &mut RenderContext<'w>, world: &'w World, ) -> Result<(), NodeRunError>
    - fn input(&self) -> Vec<SlotInfo>
    - fn output(&self) -> Vec<SlotInfo>
- Auto Trait Implementations§

The render node for the second mesh preprocessing pass.

This pass runs a compute shader to cull meshes outside the view frustum (if that wasn’t done by the CPU), cull meshes that were neither visible last frame nor visible this frame (if occlusion culling is on), transform them, and, if indirect drawing is on, populate the indirect draw parameter metadata for the subsequent LatePrepassBuildIndirectParametersNode.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct LateGpuPreprocessNode { /* private fields */ }
```

---

## Struct SunDisk Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.SunDisk.html

**Contents:**
- Struct SunDisk Copy item path
- Fields§
- Implementations§
  - impl SunDisk
    - pub const EARTH: SunDisk
    - pub const OFF: SunDisk
- Trait Implementations§
  - impl Clone for SunDisk
    - fn clone(&self) -> SunDisk
    - fn clone_from(&mut self, source: &Self)

Add to a DirectionalLight to control rendering of the visible solar disk in the sky. Affects only the disk’s appearance, not the light’s illuminance or shadows. Requires a bevy::pbr::Atmosphere component on a Camera3d to have any effect.

By default, the atmosphere is rendered with SunDisk::EARTH, which approximates the apparent size and brightness of the Sun as seen from Earth. You can also disable the sun disk entirely with SunDisk::OFF.

In order to cause the sun to “glow” and light up the surrounding sky, enable bloom in your post-processing pipeline by adding a Bloom component to your camera.

The angular size (diameter) of the sun disk in radians, as observed from the scene.

Multiplier for the brightness of the sun disk.

0.0 disables the disk entirely (atmospheric scattering still occurs), 1.0 is the default physical intensity, and values >1.0 overexpose it.

Earth-like parameters for the sun disk.

Uses the mean apparent size (~32 arcminutes) of the Sun at 1 AU distance with default intensity.

Keeps scattering and directional light illumination, but hides the disk itself.

Required Components: DirectionalLight.

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components of the required components, recursively, in depth-first order.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct SunDisk {
    pub angular_size: f32,
    pub intensity: f32,
}
```

---

## Enum RenderPhaseType Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/enum.RenderPhaseType.html

**Contents:**
- Enum RenderPhaseType Copy item path
- Variants§
  - Opaque
  - AlphaMask
  - Transmissive
  - Transparent
- Trait Implementations§
  - impl Clone for RenderPhaseType
    - fn clone(&self) -> RenderPhaseType
    - fn clone_from(&mut self, source: &Self)

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum RenderPhaseType {
    Opaque,
    AlphaMask,
    Transmissive,
    Transparent,
}
```

---

## Struct ErasedMaterialPipelineKey Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ErasedMaterialPipelineKey.html

**Contents:**
- Struct ErasedMaterialPipelineKey Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for ErasedMaterialPipelineKey
    - fn clone(&self) -> ErasedMaterialPipelineKey
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for ErasedMaterialPipelineKey
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Hash for ErasedMaterialPipelineKey
    - fn hash<__H>(&self, state: &mut __H)where __H: Hasher,

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ErasedMaterialPipelineKey {
    pub mesh_key: MeshPipelineKey,
    pub material_key: ErasedMaterialKey,
    pub type_id: TypeId,
}
```

---

## Struct CameraMainTextureUsages Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.CameraMainTextureUsages.html

**Contents:**
- Struct CameraMainTextureUsages Copy item path
- Tuple Fields§
- Implementations§
  - impl CameraMainTextureUsages
    - pub fn with(self, usages: TextureUsages) -> CameraMainTextureUsages
      - Examples found in repository?
- Trait Implementations§
  - impl Clone for CameraMainTextureUsages
    - fn clone(&self) -> CameraMainTextureUsages
    - fn clone_from(&mut self, source: &Self)

This component lets you control the TextureUsages field of the main texture generated for the camera

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CameraMainTextureUsages(pub TextureUsages);
```

Example 2 (javascript):
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

## Struct StandardMaterialUniform Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.StandardMaterialUniform.html

**Contents:**
- Struct StandardMaterialUniform Copy item path
- Fields§
- Trait Implementations§
  - impl AsBindGroupShaderType<StandardMaterialUniform> for StandardMaterial
    - fn as_bind_group_shader_type( &self, images: &RenderAssets<GpuImage>, ) -> StandardMaterialUniform
  - impl Clone for StandardMaterialUniform
    - fn clone(&self) -> StandardMaterialUniform
    - fn clone_from(&mut self, source: &Self)
  - impl CreateFrom for StandardMaterialUniformwhere StandardMaterialUniform: ShaderType<ExtraMetadata = StructMetadata<23>>, Vec4: for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom, Mat3: for<'__> CreateFrom, Vec3: for<'__> CreateFrom, f32: for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom, Vec2: for<'__> CreateFrom, u32: for<'__> CreateFrom + for<'__> CreateFrom + for<'__> CreateFrom,
    - fn create_from<B>(reader: &mut Reader<B>) -> StandardMaterialUniformwhere B: BufferRef,

The GPU representation of the uniform data of a StandardMaterial.

Doubles as diffuse albedo for non-metallic, specular for metallic and a mix for everything in between.

Color white light takes after traveling through the attenuation distance underneath the material surface

The transform applied to the UVs corresponding to ATTRIBUTE_UV_0 on the mesh before sampling. Default is identity.

Specular intensity for non-metals on a linear scale of [0.0, 1.0] defaults to 0.5 which is mapped to 4% reflectance in the shader

Linear perceptual roughness, clamped to [0.089, 1.0] in the shader Defaults to minimum of 0.089

From [0.0, 1.0], dielectric to pure metallic

Amount of diffuse light transmitted through the material

Amount of specular light transmitted through the material

Thickness of the volume underneath the material surface

How far light travels through the volume underneath the material surface before being absorbed

The StandardMaterialFlags accessible in the wgsl shader.

When the alpha mode mask flag is set, any base color alpha above this cutoff means fully opaque, and any below means fully transparent.

The depth of the StandardMaterial::depth_map to apply.

In how many layers to split the depth maps for Steep parallax mapping.

If your parallax_depth_scale is >0.1 and you are seeing jaggy edges, increase this value. However, this incurs a performance cost.

The exposure (brightness) level of the lightmap, if present.

Using ParallaxMappingMethod::Relief, how many additional steps to use at most to find the depth value.

ID for specifying which deferred lighting pass should be used for rendering this material, if any.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct StandardMaterialUniform {Show 23 fields
    pub base_color: Vec4,
    pub emissive: Vec4,
    pub attenuation_color: Vec4,
    pub uv_transform: Mat3,
    pub reflectance: Vec3,
    pub roughness: f32,
    pub metallic: f32,
    pub diffuse_transmission: f32,
    pub specular_transmission: f32,
    pub thickness: f32,
    pub ior: f32,
    pub attenuation_distance: f32,
    pub clearcoat: f32,
    pub clearcoat_perceptual_roughness: f32,
    pub anisotropy_strength: f32,
    pub anisotropy_rotation: Vec2,
    pub flags: u32,
    pub alpha_cutoff: f32,
    pub parallax_depth_scale: f
...
```

---

## Function propagate_gizmos Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/fn.propagate_gizmos.html

**Contents:**
- Function propagate_gizmos Copy item path

Propagate the contextual gizmo into the Update storage for rendering.

This should be before GizmoMeshSystems.

**Examples:**

Example 1 (unknown):
```unknown
pub fn propagate_gizmos<Config, Clear>(
    update_storage: ResMut<'_, GizmoStorage<Config, ()>>,
    contextual_storage: Res<'_, GizmoStorage<Config, Clear>>,
)where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
```

---

## Module settings Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/settings/index.html

**Contents:**
- Module settings Copy item path
- Structs§
- Enums§
- Functions§

bevy::renderModule settings Copy item pathSource Structs§BackendsRepresents the backends that wgpu will use.InstanceFlagsInstance debugging flags.RenderResourcesWgpuFeaturesFeatures that are not guaranteed to be supported.WgpuLimitsRepresents the sets of limits an adapter/device supports.WgpuSettingsProvides configuration for renderer initialization. Use RenderDevice::features, RenderDevice::limits, and the RenderAdapterInfo resource to get runtime information about the actual adapter, backend, features, and limits. NOTE: Backends::DX12, Backends::METAL, and Backends::VULKAN are enabled by default for non-web and the best choice is automatically selected. Web using the webgl feature uses Backends::GL. NOTE: If you want to use Backends::GL in a native app on Windows and/or macOS, you must use ANGLE and enable the gles feature. This is because wgpu requires EGL to create a GL context without a window and only ANGLE supports that.Enums§Dx12CompilerSelects which DX12 shader compiler to use.Gles3MinorVersionSelects which OpenGL ES 3 minor version to request.MemoryHintsHints to the device about the memory allocation strategy.PowerPreferencePower Preference when choosing a physical adapter.RenderCreationAn enum describing how the renderer will initialize resources. This is used when creating the RenderPlugin.WgpuSettingsPriorityConfigures the priority used when automatically configuring the features/limits of wgpu.Functions§settings_priority_from_envGet a features/limits priority from the environment variable WGPU_SETTINGS_PRIO

---

## Trait MeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/trait.MeshBuilder.html

**Contents:**
- Trait MeshBuilder Copy item path
- Required Methods§
    - fn build(&self) -> Mesh
- Implementors§
  - impl MeshBuilder for AnnulusMeshBuilder
  - impl MeshBuilder for Capsule2dMeshBuilder
  - impl MeshBuilder for Capsule3dMeshBuilder
  - impl MeshBuilder for CircleMeshBuilder
  - impl MeshBuilder for CircularSectorMeshBuilder
  - impl MeshBuilder for CircularSegmentMeshBuilder

A trait used to build Meshes from a configuration

Builds a Mesh based on the configuration in self.

**Examples:**

Example 1 (unknown):
```unknown
pub trait MeshBuilder {
    // Required method
    fn build(&self) -> Mesh;
}
```

---

## Enum PrimitiveTopology Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/enum.PrimitiveTopology.html

**Contents:**
- Enum PrimitiveTopology Copy item path
- Variants§
  - PointList = 0
  - LineList = 1
  - LineStrip = 2
  - TriangleList = 3
  - TriangleStrip = 4
- Implementations§
  - impl PrimitiveTopology
    - pub fn is_strip(&self) -> bool

Primitive type the input mesh is composed of.

Corresponds to WebGPU GPUPrimitiveTopology.

Vertex data is a list of points. Each vertex is a new point.

Vertex data is a list of lines. Each pair of vertices composes a new line.

Vertices 0 1 2 3 create two lines 0 1 and 2 3

Vertex data is a strip of lines. Each set of two adjacent vertices form a line.

Vertices 0 1 2 3 create three lines 0 1, 1 2, and 2 3.

Vertex data is a list of triangles. Each set of 3 vertices composes a new triangle.

Vertices 0 1 2 3 4 5 create two triangles 0 1 2 and 3 4 5

Vertex data is a triangle strip. Each set of three adjacent vertices form a triangle.

Vertices 0 1 2 3 4 5 create four triangles 0 1 2, 2 1 3, 2 3 4, and 4 3 5

Returns true for strip topologies.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub enum PrimitiveTopology {
    PointList = 0,
    LineList = 1,
    LineStrip = 2,
    TriangleList = 3,
    TriangleStrip = 4,
}
```

---

## Struct SpecializedMaterialViewPipelineCache Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.SpecializedMaterialViewPipelineCache.html

**Contents:**
- Struct SpecializedMaterialViewPipelineCache Copy item path
- Methods from Deref<Target = HashMap<MainEntity, (Tick, CachedRenderPipelineId), EntityHash>>§
    - pub fn hasher(&self) -> &S
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn keys(&self) -> Keys<'_, K, V> ⓘ
      - §Examples
      - Examples found in repository?
    - pub fn values(&self) -> Values<'_, K, V> ⓘ
      - §Examples

Stores the cached render pipeline ID for each entity in a single view, as well as the last time it was changed.

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

Shrinks the capacity of the map with a lower limit. It will drop down no lower than 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct SpecializedMaterialViewPipelineCache { /* private fields */ }
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

## Function triangle_normal Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/fn.triangle_normal.html

**Contents:**
- Function triangle_normal Copy item path

Compute the normal of a face made of three points: a, b, and c.

**Examples:**

Example 1 (unknown):
```unknown
pub fn triangle_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3]
```

---

## Struct SpecializedShadowMaterialViewPipelineCache Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.SpecializedShadowMaterialViewPipelineCache.html

**Contents:**
- Struct SpecializedShadowMaterialViewPipelineCache Copy item path
- Methods from Deref<Target = HashMap<MainEntity, (Tick, CachedRenderPipelineId), EntityHash>>§
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
pub struct SpecializedShadowMaterialViewPipelineCache { /* private fields */ }
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

## Struct Camera3dDepthTextureUsage Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.Camera3dDepthTextureUsage.html

**Contents:**
- Struct Camera3dDepthTextureUsage Copy item path
- Tuple Fields§
- Trait Implementations§
  - impl Clone for Camera3dDepthTextureUsage
    - fn clone(&self) -> Camera3dDepthTextureUsage
    - fn clone_from(&mut self, source: &Self)
  - impl<'de> Deserialize<'de> for Camera3dDepthTextureUsage
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<Camera3dDepthTextureUsage, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,
  - impl From<Camera3dDepthTextureUsage> for TextureUsages
    - fn from(value: Camera3dDepthTextureUsage) -> TextureUsages

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Camera3dDepthTextureUsage(pub u32);
```

---

## Struct EllipseMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.EllipseMeshBuilder.html

**Contents:**
- Struct EllipseMeshBuilder Copy item path
- Fields§
- Implementations§
  - impl EllipseMeshBuilder
    - pub const fn new( half_width: f32, half_height: f32, resolution: u32, ) -> EllipseMeshBuilder
    - pub const fn resolution(self, resolution: u32) -> EllipseMeshBuilder
- Trait Implementations§
  - impl Clone for EllipseMeshBuilder
    - fn clone(&self) -> EllipseMeshBuilder
    - fn clone_from(&mut self, source: &Self)

A builder used for creating a Mesh with an Ellipse shape.

The number of vertices used for the ellipse mesh. The default is 32.

Creates a new EllipseMeshBuilder from a given half width and half height and a vertex count.

Sets the number of vertices used for the ellipse mesh.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct EllipseMeshBuilder {
    pub ellipse: Ellipse,
    pub resolution: u32,
}
```

---

## Struct Image Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.Image.html

**Contents:**
- Struct Image Copy item path
  - §Remote Inspection
- Fields§
    - §Field Usage Notes
    - §Field Usage Notes
- Implementations§
  - impl Image
    - pub fn new( size: Extent3d, dimension: TextureDimension, data: Vec<u8>, format: TextureFormat, asset_usage: RenderAssetUsages, ) -> Image
      - §Panics
    - pub fn new_uninit( size: Extent3d, dimension: TextureDimension, format: TextureFormat, asset_usage: RenderAssetUsages, ) -> Image

An image, optimized for usage in rendering.

To transmit an Image between two running Bevy apps, e.g. through BRP, use SerializedImage. This type is only meant for short-term transmission between same versions and should not be stored anywhere.

Raw pixel data. If the image is being used as a storage texture which doesn’t need to be initialized by the CPU, then this should be None. Otherwise, it should always be Some.

For texture data with layers and mips, this field controls how wgpu interprets the buffer layout.

Use TextureDataOrder::default() for all other cases.

Describes the data layout of the GPU texture. For example, whether a texture contains 1D/2D/3D data, and what the format of the texture data is.

The ImageSampler to use during rendering.

Describes how the GPU texture should be interpreted. For example, 2D image data could be read as plain 2D, an array texture of layers of 2D with the same dimensions (and the number of layers in that case), a cube map, an array of cube maps, etc.

Whether this image should be copied on the GPU when resized.

Creates a new image from raw binary data and the corresponding metadata.

Panics if the length of the data, volume of the size and the size of the format do not match.

Exactly the same as Image::new, but doesn’t initialize the image

A transparent white 1x1x1 image.

Contrast to Image::default, which is opaque.

Creates a new uninitialized 1x1x1 image

Creates a new image from raw binary data and the corresponding metadata, by filling the image data with the pixel data repeated multiple times.

Panics if the size of the format is not a multiple of the length of the pixel data.

Create a new zero-filled image with a given size, which can be rendered to. Useful for mirrors, UI, or exporting images for example. This is primarily for use as a render target for a Camera. See RenderTarget::Image.

For Standard Dynamic Range (SDR) images you can use TextureFormat::Rgba8UnormSrgb. For High Dynamic Range (HDR) images you can use TextureFormat::Rgba16Float.

The default TextureUsages are TEXTURE_BINDING, COPY_DST, RENDER_ATTACHMENT.

The default RenderAssetUsages is MAIN_WORLD | RENDER_WORLD so that it is accessible from the CPU and GPU. You can customize this by changing the asset_usage field.

Returns the width of a 2D image.

Returns the height of a 2D image.

Returns the aspect ratio (width / height) of a 2D image.

Returns the size of a 2D image as f32.

Returns the size of a 2D image.

Resizes the image to

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Image {
    pub data: Option<Vec<u8>>,
    pub data_order: TextureDataOrder,
    pub texture_descriptor: TextureDescriptor<Option<&'static str>, &'static [TextureFormat]>,
    pub sampler: ImageSampler,
    pub texture_view_descriptor: Option<TextureViewDescriptor<Option<&'static str>>>,
    pub asset_usage: RenderAssetUsages,
    pub copy_on_resize: bool,
}
```

Example 2 (javascript):
```javascript
349    fn from_world(world: &mut World) -> Self {
350        let mut images = world.resource_mut::<Assets<Image>>();
351
352        // Create a new 32-bit floating point depth texture.
353        let mut depth_image = Image::new_uninit(
354            Extent3d {
355                width: DEPTH_TEXTURE_SIZE,
356                height: DEPTH_TEXTURE_SIZE,
357                depth_or_array_layers: 1,
358            },
359            TextureDimension::D2,
360            TextureFormat::Depth32Float,
361            RenderAssetUsages::default(),
362        );
363
364        // Create a sampler. Note 
...
```

Example 3 (javascript):
```javascript
25fn test(
26    mut commands: Commands,
27    mut images: ResMut<Assets<Image>>,
28    mut meshes: ResMut<Assets<Mesh>>,
29    mut materials: ResMut<Assets<StandardMaterial>>,
30) {
31    // Spawn a UI camera
32    commands.spawn(Camera3d::default());
33
34    // Set up an texture for the 3D camera to render to.
35    // The size of the texture will be based on the viewport's ui size.
36    let mut image = Image::new_uninit(
37        default(),
38        TextureDimension::D2,
39        TextureFormat::Bgra8UnormSrgb,
40        RenderAssetUsages::all(),
41    );
42    image.texture_descriptor.
...
```

Example 4 (javascript):
```javascript
70fn setup(
71    mut commands: Commands,
72    mut images: ResMut<Assets<Image>>,
73    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
74) {
75    // Create a storage buffer with some data
76    let buffer: Vec<u32> = (0..BUFFER_LEN as u32).collect();
77    let mut buffer = ShaderStorageBuffer::from(buffer);
78    // We need to enable the COPY_SRC usage so we can copy the buffer to the cpu
79    buffer.buffer_description.usage |= BufferUsages::COPY_SRC;
80    let buffer = buffers.add(buffer);
81
82    // Create a storage texture with some data
83    let size = Extent3d {
84        width: B
...
```

---

## Crate shader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/shader/index.html

**Contents:**
- Crate shader Copy item path
- Modules§
- Macros§
- Structs§
- Enums§
- Type Aliases§

bevyCrate shader Copy item pathSource Modules§preludeThe shader prelude.Macros§load_shader_libraryInline shader as an embedded_asset and load it permanently.Structs§ShaderAn “unprocessed” shader. It can contain preprocessor directives.ShaderCacheShaderIdShaderLoaderShaderResolverShaderSettingsSettings for loading shaders.Enums§PipelineCacheErrorType of error returned by a PipelineCache when the creation of a GPU pipeline object failed.ShaderCacheSourceSource of a shader module.ShaderDefValShaderImportShaderLoaderErrorShaderRefA reference to a shader asset.ShaderReflectErrorSourceValidateShaderDescribes whether or not to perform runtime checks on shaders. Runtime checks can be enabled for safety at the cost of speed. By default no runtime checks will be performed.Type Aliases§CachedPipelineId

---

## Struct SerializedMesh Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.SerializedMesh.html

**Contents:**
- Struct SerializedMesh Copy item path
- Implementations§
  - impl SerializedMesh
    - pub fn from_mesh(mesh: Mesh) -> SerializedMesh
    - pub fn into_mesh(self) -> Mesh
- Trait Implementations§
  - impl Clone for SerializedMesh
    - fn clone(&self) -> SerializedMesh
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for SerializedMesh

A version of Mesh suitable for serializing for short-term transfer.

Mesh does not implement Serialize / Deserialize because it is made with the renderer in mind. It is not a general-purpose mesh implementation, and its internals are subject to frequent change. As such, storing a Mesh on disk is highly discouraged.

But there are still some valid use cases for serializing a Mesh, namely transferring meshes between processes. To support this, you can create a SerializedMesh from a Mesh with SerializedMesh::from_mesh, and then deserialize it with SerializedMesh::deserialize. The caveats are:

Create a SerializedMesh from a Mesh. See the documentation for SerializedMesh for caveats.

Create a Mesh from a SerializedMesh. See the documentation for SerializedMesh for caveats.

Use MeshDeserializer if you need to pass extra options to the deserialization process, such as specifying custom vertex attributes.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct SerializedMesh { /* private fields */ }
```

---

## Struct Mesh2d Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.Mesh2d.html

**Contents:**
- Struct Mesh2d Copy item path
- §Example
- Tuple Fields§
- Methods from Deref<Target = Handle<Mesh>>§
    - pub fn id(&self) -> AssetId<A>
      - Examples found in repository?
    - pub fn path(&self) -> Option<&AssetPath<'static>>
    - pub fn is_uuid(&self) -> bool
    - pub fn is_strong(&self) -> bool
- Trait Implementations§

A component for 2D meshes. Requires a MeshMaterial2d to be rendered, commonly using a ColorMaterial.

Returns the AssetId of this Asset.

Returns the path if this is (1) a strong handle and (2) the asset has a path

Returns true if this is a uuid handle.

Returns true if this is a strong handle.

Required Components: Transform.

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components of the required components, recursively, in depth-first order.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Mesh2d(pub Handle<Mesh>);
```

Example 2 (unknown):
```unknown
// Spawn an entity with a mesh using `ColorMaterial`.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
    ));
}
```

Example 3 (javascript):
```javascript
112    fn as_asset_id(&self) -> AssetId<Self::Asset> {
113        self.0.id()
114    }
115}
116
117#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
118struct ImageMaterial {
119    image: Handle<Image>,
120}
121
122impl ErasedRenderAsset for ImageMaterial {
123    type SourceAsset = ImageMaterial;
124    type ErasedAsset = PreparedMaterial;
125    type Param = (
126        SRes<DrawFunctions<Opaque3d>>,
127        SRes<ImageMaterialBindGroupLayout>,
128        SRes<AssetServer>,
129        SResMut<MaterialBindGroupAllocators>,
130        SResMut<RenderMaterialBindings>,
131        SRes<Re
...
```

Example 4 (javascript):
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

---

## Struct ScreenSpaceReflections Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ScreenSpaceReflections.html

**Contents:**
- Struct ScreenSpaceReflections Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for ScreenSpaceReflections
    - fn clone(&self) -> ScreenSpaceReflections
    - fn clone_from(&mut self, source: &Self)
  - impl Component for ScreenSpaceReflectionswhere ScreenSpaceReflections: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

Add this component to a camera to enable screen-space reflections (SSR).

Screen-space reflections currently require deferred rendering in order to appear. Therefore, they also need the DepthPrepass and DeferredPrepass components, which are inserted automatically.

SSR currently performs no roughness filtering for glossy reflections, so only very smooth surfaces will reflect objects in screen space. You can adjust the perceptual_roughness_threshold in order to tune the threshold below which screen-space reflections will be traced.

As with all screen-space techniques, SSR can only reflect objects on screen. When objects leave the camera, they will disappear from reflections. An alternative that doesn’t suffer from this problem is the combination of a LightProbe and EnvironmentMapLight. The advantage of SSR is that it can reflect all objects, not just static ones.

SSR is an approximation technique and produces artifacts in some situations. Hand-tuning the settings in this component will likely be useful.

Screen-space reflections are presently unsupported on WebGL 2 because of a bug whereby Naga doesn’t generate correct GLSL when sampling depth buffers, which is required for screen-space raymarching.

The maximum PBR roughness level that will enable screen space reflections.

When marching the depth buffer, we only have 2.5D information and don’t know how thick surfaces are. We shall assume that the depth buffer fragments are cuboids with a constant thickness defined by this parameter.

The number of steps to be taken at regular intervals to find an initial intersection. Must not be zero.

Higher values result in higher-quality reflections, because the raymarching shader is less likely to miss objects. However, they take more GPU time.

Exponent to be applied in the linear part of the march.

A value of 1.0 will result in equidistant steps, and higher values will compress the earlier steps, and expand the later ones. This might be desirable in order to get more detail close to objects.

For optimal performance, this should be a small unsigned integer, such as 1 or 2.

Number of steps in a bisection (binary search) to perform once the linear search has found an intersection. Helps narrow down the hit, increasing the chance of the secant method finding an accurate hit point.

Approximate the root position using the secant method—by solving for line-line intersection between the ray approach rate and the surface gradient.

Required Components: DepthPrepass, D

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct ScreenSpaceReflections {
    pub perceptual_roughness_threshold: f32,
    pub thickness: f32,
    pub linear_steps: u32,
    pub linear_march_exponent: f32,
    pub bisection_steps: u32,
    pub use_secant: bool,
}
```

---

## Enum Camera3dDepthLoadOp Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/enum.Camera3dDepthLoadOp.html

**Contents:**
- Enum Camera3dDepthLoadOp Copy item path
- Variants§
  - Clear(f32)
  - Load
- Trait Implementations§
  - impl Clone for Camera3dDepthLoadOp
    - fn clone(&self) -> Camera3dDepthLoadOp
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for Camera3dDepthLoadOp
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

The depth clear operation to perform for the main 3d pass.

Clear with a specified value. Note that 0.0 is the far plane due to bevy’s use of reverse-z projections.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum Camera3dDepthLoadOp {
    Clear(f32),
    Load,
}
```

---

## Constant MATERIAL_BIND_GROUP_INDEX Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/constant.MATERIAL_BIND_GROUP_INDEX.html

**Contents:**
- Constant MATERIAL_BIND_GROUP_INDEX Copy item path

bevy::pbrConstant MATERIAL_BIND_GROUP_INDEX Copy item pathSource pub const MATERIAL_BIND_GROUP_INDEX: usize = 3; // 3usize

**Examples:**

Example 1 (javascript):
```javascript
pub const MATERIAL_BIND_GROUP_INDEX: usize = 3; // 3usize
```

---

## Module taa Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/anti_alias/taa/index.html

**Contents:**
- Module taa Copy item path
- Structs§

bevy::anti_aliasModule taa Copy item pathSource Structs§TemporalAntiAliasHistoryTexturesTemporalAntiAliasNodeRender bevy_render::render_graph::Node used by temporal anti-aliasing.TemporalAntiAliasPipelineIdTemporalAntiAliasPluginPlugin for temporal anti-aliasing.TemporalAntiAliasingComponent to apply temporal anti-aliasing to a 3D camera.

---

## Struct ShaderSettings Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/shader/struct.ShaderSettings.html

**Contents:**
- Struct ShaderSettings Copy item path
- Fields§
- Trait Implementations§
  - impl Debug for ShaderSettings
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for ShaderSettings
    - fn default() -> ShaderSettings
  - impl<'de> Deserialize<'de> for ShaderSettings
    - fn deserialize<__D>( __deserializer: __D, ) -> Result<ShaderSettings, <__D as Deserializer<'de>>::Error>where __D: Deserializer<'de>,
  - impl Serialize for ShaderSettings

Settings for loading shaders.

The #define specified for this shader.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ShaderSettings {
    pub shader_defs: Vec<ShaderDefVal>,
}
```

---

## Struct RenderPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/struct.RenderPlugin.html

**Contents:**
- Struct RenderPlugin Copy item path
- Fields§
- Trait Implementations§
  - impl Default for RenderPlugin
    - fn default() -> RenderPlugin
  - impl Plugin for RenderPlugin
    - fn build(&self, app: &mut App)
    - fn ready(&self, app: &App) -> bool
    - fn finish(&self, app: &mut App)
    - fn cleanup(&self, _app: &mut App)

Contains the default Bevy rendering backend based on wgpu.

Rendering is done in a SubApp, which exchanges data with the main app between main schedule iterations.

Rendering can be executed between iterations of the main schedule, or it can be executed in parallel with main schedule when PipelinedRenderingPlugin is enabled.

If true, disables asynchronous pipeline compilation. This has no effect on macOS, Wasm, iOS, or without the multi_threaded feature.

Debugging flags that can optionally be set when constructing the renderer.

Initializes the renderer, sets up the RenderSystems and creates the rendering sub-app.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderPlugin {
    pub render_creation: RenderCreation,
    pub synchronous_pipeline_compilation: bool,
    pub debug_flags: RenderDebugFlags,
}
```

---

## Struct MeshletPrepassFragmentShader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshletPrepassFragmentShader.html

**Contents:**
- Struct MeshletPrepassFragmentShader Copy item path
- Trait Implementations§
  - impl Clone for MeshletPrepassFragmentShader
    - fn clone(&self) -> MeshletPrepassFragmentShader
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for MeshletPrepassFragmentShader
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for MeshletPrepassFragmentShader
    - fn default() -> MeshletPrepassFragmentShader
  - impl Hash for MeshletPrepassFragmentShader

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MeshletPrepassFragmentShader;
```

---

## Struct SpecializedPrepassMaterialPipelineCache Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.SpecializedPrepassMaterialPipelineCache.html

**Contents:**
- Struct SpecializedPrepassMaterialPipelineCache Copy item path
- Methods from Deref<Target = HashMap<RetainedViewEntity, SpecializedPrepassMaterialViewPipelineCache>>§
    - pub fn hasher(&self) -> &S
    - pub fn capacity(&self) -> usize
      - §Examples
    - pub fn keys(&self) -> Keys<'_, K, V> ⓘ
      - §Examples
      - Examples found in repository?
    - pub fn values(&self) -> Values<'_, K, V> ⓘ
      - §Examples

Stores the SpecializedPrepassMaterialViewPipelineCache for each view.

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

Shrinks the capacity of the map with a lower limit. It will drop down no lower than the supplied limit while maintaining the i

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct SpecializedPrepassMaterialPipelineCache { /* private fields */ }
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

## Struct Camera3d Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.Camera3d.html

**Contents:**
- Struct Camera3d Copy item path
- Fields§
      - §Notes
- Trait Implementations§
  - impl Clone for Camera3d
    - fn clone(&self) -> Camera3d
    - fn clone_from(&mut self, source: &Self)
  - impl Component for Camera3dwhere Camera3d: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable

A 3D camera component. Enables the main 3D render graph for a Camera.

The camera coordinate space is right-handed X-right, Y-up, Z-back. This means “forward” is -Z.

The depth clear operation to perform for the main 3d pass.

The texture usages for the depth texture created for the main 3d pass.

How many individual steps should be performed in the Transmissive3d pass.

Roughly corresponds to how many “layers of transparency” are rendered for screen space specular transmissive objects. Each step requires making one additional texture copy, so it’s recommended to keep this number to a reasonably low value. Defaults to 1.

The quality of the screen space specular transmission blur effect, applied to whatever’s “behind” transmissive objects when their roughness is greater than 0.0.

Higher qualities are more GPU-intensive.

Note: You can get better-looking results at any quality level by enabling TAA. See: TemporalAntiAliasPlugin

Required Components: Camera, Projection.

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components of the required components, recursively, in depth-first order.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Camera3d {
    pub depth_load_op: Camera3dDepthLoadOp,
    pub depth_texture_usages: Camera3dDepthTextureUsage,
    pub screen_space_specular_transmission_steps: usize,
    pub screen_space_specular_transmission_quality: ScreenSpaceTransmissionQuality,
}
```

---

## Enum CylinderAnchor Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/enum.CylinderAnchor.html

**Contents:**
- Enum CylinderAnchor Copy item path
- Variants§
  - MidPoint
  - Top
  - Bottom
- Trait Implementations§
  - impl Clone for CylinderAnchor
    - fn clone(&self) -> CylinderAnchor
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for CylinderAnchor

Anchoring options for CylinderMeshBuilder

Midpoint between the top and bottom caps of the cylinder

The center of the top circle cap

The center of the bottom circle cap

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum CylinderAnchor {
    MidPoint,
    Top,
    Bottom,
}
```

---

## Struct PreprocessPipeline Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PreprocessPipeline.html

**Contents:**
- Struct PreprocessPipeline Copy item path
- Fields§
- Trait Implementations§
  - impl SpecializedComputePipeline for PreprocessPipeline
    - type Key = PreprocessPipelineKey
    - fn specialize( &self, key: <PreprocessPipeline as SpecializedComputePipeline>::Key, ) -> ComputePipelineDescriptor
- Auto Trait Implementations§
  - impl Freeze for PreprocessPipeline
  - impl !RefUnwindSafe for PreprocessPipeline
  - impl Send for PreprocessPipeline

The pipeline for the GPU mesh preprocessing shader.

The bind group layout for the compute shader.

The shader asset handle.

The pipeline ID for the compute shader.

This gets filled in prepare_preprocess_pipelines.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PreprocessPipeline {
    pub bind_group_layout: BindGroupLayout,
    pub shader: Handle<Shader>,
    pub pipeline_id: Option<CachedComputePipelineId>,
}
```

---

## Struct BaseMeshPipelineKey Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.BaseMeshPipelineKey.html

**Contents:**
- Struct BaseMeshPipelineKey Copy item path
- Implementations§
  - impl BaseMeshPipelineKey
    - pub const MORPH_TARGETS: BaseMeshPipelineKey
  - impl BaseMeshPipelineKey
    - pub const fn empty() -> BaseMeshPipelineKey
    - pub const fn all() -> BaseMeshPipelineKey
    - pub const fn bits(&self) -> u64
    - pub const fn from_bits(bits: u64) -> Option<BaseMeshPipelineKey>
    - pub const fn from_bits_truncate(bits: u64) -> BaseMeshPipelineKey

Our base mesh pipeline key bits start from the highest bit and go downward. The PBR mesh pipeline key bits start from the lowest bit and go upward. This allows the PBR bits in the downstream crate bevy_pbr to coexist in the same field without any shifts.

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

The bitwise or (|) of the bits

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct BaseMeshPipelineKey(/* private fields */);
```

---

## Crate sprite_render Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/sprite_render/index.html

**Contents:**
- Crate sprite_render Copy item path
- Modules§
- Structs§
- Enums§
- Constants§
- Traits§
- Functions§
- Type Aliases§

Provides 2D sprite rendering functionality.

---

## Struct MeshCullingData Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshCullingData.html

**Contents:**
- Struct MeshCullingData Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for MeshCullingData
    - fn clone(&self) -> MeshCullingData
    - fn clone_from(&mut self, source: &Self)
  - impl CreateFrom for MeshCullingDatawhere MeshCullingData: ShaderType<ExtraMetadata = StructMetadata<2>>, Vec4: for<'__> CreateFrom + for<'__> CreateFrom,
    - fn create_from<B>(reader: &mut Reader<B>) -> MeshCullingDatawhere B: BufferRef,
  - impl Default for MeshCullingData
    - fn default() -> MeshCullingData

Information about each mesh instance needed to cull it on GPU.

This consists of its axis-aligned bounding box (AABB).

The 3D center of the AABB in model space, padded with an extra unused float value.

The 3D extents of the AABB in model space, divided by two, padded with an extra unused float value.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[repr(C)]pub struct MeshCullingData {
    pub aabb_center: Vec4,
    pub aabb_half_extents: Vec4,
}
```

---

## Struct AmbientLight Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.AmbientLight.html

**Contents:**
- Struct AmbientLight Copy item path
- §Examples
- Fields§
- Implementations§
  - impl AmbientLight
    - pub const NONE: AmbientLight
- Trait Implementations§
  - impl Clone for AmbientLight
    - fn clone(&self) -> AmbientLight
    - fn clone_from(&mut self, source: &Self)

An ambient light, which lights the entire scene equally.

This resource is inserted by the LightPlugin and by default it is set to a low ambient light.

It can also be added to a camera to override the resource (or default) ambient for that camera only.

Make ambient light slightly brighter:

A direct scale factor multiplied with color before being passed to the shader.

After applying this multiplier, the resulting value should be in units of cd/m^2.

Whether this ambient light has an effect on meshes with lightmaps.

Set this to false if your lightmap baking tool bakes the ambient light into the lightmaps, to avoid rendering that light twice.

By default, this is set to true.

Required Components: Camera.

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components of the required components, recursively, in depth-first order.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AmbientLight {
    pub color: Color,
    pub brightness: f32,
    pub affects_lightmapped_meshes: bool,
}
```

Example 2 (unknown):
```unknown
fn setup_ambient_light(mut ambient_light: ResMut<AmbientLight>) {
   ambient_light.brightness = 100.0;
}
```

---

## Module wireframe Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/wireframe/index.html

**Contents:**
- Module wireframe Copy item path
- Structs§
- Functions§
- Type Aliases§

bevy::pbrModule wireframe Copy item pathSource Structs§ExtractedWireframeColorMesh3dWireframeNoWireframeDisables wireframe rendering for any entity it is attached to. It will ignore the WireframeConfig global setting.RenderWireframeInstancesRenderWireframeMaterialSetWireframe3dPushConstantsSpecializedWireframePipelineCacheStores the SpecializedWireframeViewPipelineCache for each view.SpecializedWireframeViewPipelineCacheStores the cached render pipeline ID for each entity in a single view, as well as the last time it was changed.WireframeEnables wireframe rendering for any entity it is attached to. It will ignore the WireframeConfig global setting.Wireframe3dWireframe3dBatchSetKeyWireframe3dBinKeyData that must be identical in order to batch phase items together.Wireframe3dPipelineWireframeColorSets the color of the Wireframe of the entity it is attached to.WireframeConfigWireframeEntitiesNeedingSpecializationWireframeEntitySpecializationTicksWireframeMaterialWireframePluginA Plugin that draws wireframes.Functions§check_wireframe_entities_needing_specializationextract_wireframe_entities_needing_specializationextract_wireframe_materialsinit_wireframe_3d_pipelinespecialize_wireframesType Aliases§DrawWireframe3d

---

## Struct CameraPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/camera/struct.CameraPlugin.html

**Contents:**
- Struct CameraPlugin Copy item path
- Trait Implementations§
  - impl Default for CameraPlugin
    - fn default() -> CameraPlugin
  - impl Plugin for CameraPlugin
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
pub struct CameraPlugin;
```

---

## Module core_2d Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/core_pipeline/core_2d/index.html

**Contents:**
- Module core_2d Copy item path
- Modules§
- Structs§
- Constants§
- Functions§

bevy::core_pipelineModule core_2d Copy item pathSource Modules§graphStructs§AlphaMask2dAlpha mask 2D BinnedPhaseItems.AlphaMask2dBinKeyData that must be identical in order to batch phase items together.BatchSetKey2d2D meshes aren’t currently multi-drawn together, so this batch set key only stores whether the mesh is indexed.Core2dPluginMainOpaquePass2dNodeA bevy_render::render_graph::Node that runs the Opaque2d ViewBinnedRenderPhases and AlphaMask2d ViewBinnedRenderPhasesMainTransparentPass2dNodeOpaque2dOpaque 2D BinnedPhaseItems.Opaque2dBinKeyData that must be identical in order to batch phase items together.Transparent2dTransparent 2D SortedPhaseItems.Constants§CORE_2D_DEPTH_FORMATFunctions§extract_core_2d_camera_phasesprepare_core_2d_depth_textures

---

## Struct MaterialFragmentShader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialFragmentShader.html

**Contents:**
- Struct MaterialFragmentShader Copy item path
- Trait Implementations§
  - impl Clone for MaterialFragmentShader
    - fn clone(&self) -> MaterialFragmentShader
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for MaterialFragmentShader
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Default for MaterialFragmentShader
    - fn default() -> MaterialFragmentShader
  - impl Hash for MaterialFragmentShader

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialFragmentShader;
```

---

## Struct RectangleMeshBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/struct.RectangleMeshBuilder.html

**Contents:**
- Struct RectangleMeshBuilder Copy item path
- Implementations§
  - impl RectangleMeshBuilder
    - pub const fn new(width: f32, height: f32) -> RectangleMeshBuilder
      - §Panics
- Trait Implementations§
  - impl Clone for RectangleMeshBuilder
    - fn clone(&self) -> RectangleMeshBuilder
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for RectangleMeshBuilder

A builder used for creating a Mesh with a Rectangle shape.

Creates a new RectangleMeshBuilder from a full width and height.

Panics in debug mode if width or height is negative.

Returns the default RectangleMeshBuilder with a half-width and half-height of 0.5.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RectangleMeshBuilder { /* private fields */ }
```

---

## Struct CascadeShadowConfigBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/struct.CascadeShadowConfigBuilder.html

**Contents:**
- Struct CascadeShadowConfigBuilder Copy item path
- Fields§
- Implementations§
  - impl CascadeShadowConfigBuilder
    - pub fn build(&self) -> CascadeShadowConfig
      - Examples found in repository?
- Trait Implementations§
  - impl Default for CascadeShadowConfigBuilder
    - fn default() -> CascadeShadowConfigBuilder
  - impl From<CascadeShadowConfigBuilder> for CascadeShadowConfig

Builder for CascadeShadowConfig.

The number of shadow cascades. More cascades increases shadow quality by mitigating perspective aliasing - a phenomenon where areas nearer the camera are covered by fewer shadow map texels than areas further from the camera, causing blocky looking shadows.

This does come at the cost increased rendering overhead, however this overhead is still less than if you were to use fewer cascades and much larger shadow map textures to achieve the same quality level.

In case rendered geometry covers a relatively narrow and static depth relative to camera, it may make more sense to use fewer cascades and a higher resolution shadow map texture as perspective aliasing is not as much an issue. Be sure to adjust minimum_distance and maximum_distance appropriately.

The minimum shadow distance, which can help improve the texel resolution of the first cascade. Areas nearer to the camera than this will likely receive no shadows.

NOTE: Due to implementation details, this usually does not impact shadow quality as much as first_cascade_far_bound and maximum_distance. At many view frustum field-of-views, the texel resolution of the first cascade is dominated by the width / height of the view frustum plane at first_cascade_far_bound rather than the depth of the frustum from minimum_distance to first_cascade_far_bound.

The maximum shadow distance. Areas further from the camera than this will likely receive no shadows.

Sets the far bound of the first cascade, relative to the view origin. In-between cascades will be exponentially spaced relative to the maximum shadow distance. NOTE: This is ignored if there is only one cascade, the maximum distance takes precedence.

Sets the overlap proportion between cascades. The overlap is used to make the transition from one cascade’s shadow map to the next less abrupt by blending between both shadow maps.

Returns the cascade config as specified by this builder.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct CascadeShadowConfigBuilder {
    pub num_cascades: usize,
    pub minimum_distance: f32,
    pub maximum_distance: f32,
    pub first_cascade_far_bound: f32,
    pub overlap_proportion: f32,
}
```

Example 2 (unknown):
```unknown
98fn setup_camera_and_environment(
99    mut commands: Commands,
100    mut meshes: ResMut<Assets<Mesh>>,
101    mut materials: ResMut<Assets<StandardMaterial>>,
102) {
103    // Camera
104    commands.spawn((
105        Camera3d::default(),
106        Transform::from_xyz(100.0, 100.0, 150.0).looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
107    ));
108
109    // Plane
110    commands.spawn((
111        Mesh3d(meshes.add(Plane3d::default().mesh().size(500000.0, 500000.0))),
112        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
113    ));
114
115    // Light
116    commands.spa
...
```

Example 3 (unknown):
```unknown
89fn spawn_scene(commands: &mut Commands, asset_server: &AssetServer) {
90    // Spawn the main scene.
91    commands.spawn(SceneRoot(asset_server.load(
92        GltfAssetLabel::Scene(0).from_asset("models/TonemappingTest/TonemappingTest.gltf"),
93    )));
94
95    // Spawn the flight helmet.
96    commands.spawn((
97        SceneRoot(
98            asset_server
99                .load(GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf")),
100        ),
101        Transform::from_xyz(0.5, 0.0, -0.5).with_rotation(Quat::from_rotation_y(-0.15 * PI)),
102    ));
103
104  
...
```

Example 4 (unknown):
```unknown
344fn add_basic_scene(commands: &mut Commands, asset_server: &AssetServer) {
345    // Spawn the main scene.
346    commands.spawn(SceneRoot(asset_server.load(
347        GltfAssetLabel::Scene(0).from_asset("models/TonemappingTest/TonemappingTest.gltf"),
348    )));
349
350    // Spawn the flight helmet.
351    commands.spawn((
352        SceneRoot(
353            asset_server
354                .load(GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf")),
355        ),
356        Transform::from_xyz(0.5, 0.0, -0.5).with_rotation(Quat::from_rotation_y(-0.15 * PI)),
357  
...
```

---

## Function mark_3d_meshes_as_changed_if_their_assets_changed Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/mesh/fn.mark_3d_meshes_as_changed_if_their_assets_changed.html

**Contents:**
- Function mark_3d_meshes_as_changed_if_their_assets_changed Copy item path

A system that marks a Mesh3d as changed if the associated Mesh asset has changed.

This is needed because the systems that extract meshes, such as extract_meshes_for_gpu_building, write some metadata about the mesh (like the location within each slab) into the GPU structures that they build that needs to be kept up to date if the contents of the mesh change.

**Examples:**

Example 1 (unknown):
```unknown
pub fn mark_3d_meshes_as_changed_if_their_assets_changed(
    meshes_3d: Query<'_, '_, &mut Mesh3d>,
    mesh_asset_events: MessageReader<'_, '_, AssetEvent<Mesh>>,
)
```

---

## Struct PrepassPipeline Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PrepassPipeline.html

**Contents:**
- Struct PrepassPipeline Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for PrepassPipeline
    - fn clone(&self) -> PrepassPipeline
    - fn clone_from(&mut self, source: &Self)
  - impl Resource for PrepassPipelinewhere PrepassPipeline: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for PrepassPipeline
  - impl !RefUnwindSafe for PrepassPipeline

Whether skins will use uniform buffers on account of storage buffers being unavailable on this platform.

Whether binding arrays (a.k.a. bindless textures) are usable on the current render device.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PrepassPipeline {
    pub view_layout_motion_vectors: BindGroupLayout,
    pub view_layout_no_motion_vectors: BindGroupLayout,
    pub mesh_layouts: MeshLayouts,
    pub empty_layout: BindGroupLayout,
    pub default_prepass_shader: Handle<Shader>,
    pub skins_use_uniform_buffers: bool,
    pub depth_clip_control_supported: bool,
    pub binding_arrays_are_usable: bool,
    pub material_pipeline: MaterialPipeline,
}
```

---

## Struct DrawMesh Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.DrawMesh.html

**Contents:**
- Struct DrawMesh Copy item path
- Trait Implementations§
  - impl<P> RenderCommand<P> for DrawMeshwhere P: PhaseItem,
    - type Param = (Res<'static, RenderAssets<RenderMesh>>, Res<'static, RenderMeshInstances>, Res<'static, IndirectParametersBuffers>, Res<'static, PipelineCache>, Res<'static, MeshAllocator>, Option<Res<'static, PreprocessPipelines>>, Res<'static, GpuPreprocessingSupport>)
    - type ViewQuery = Has<PreprocessBindGroups>
    - type ItemQuery = ()
    - fn render<'w>( item: &P, has_preprocess_bind_group: <<<DrawMesh as RenderCommand<P>>::ViewQuery as QueryData>::ReadOnly as QueryData>::Item<'_, '_>, _item_query: Option<()>, _: <<DrawMesh as RenderCommand<P>>::Param as SystemParam>::Item<'w, '_>, pass: &mut TrackedRenderPass<'w>, ) -> RenderCommandResult
- Auto Trait Implementations§
  - impl Freeze for DrawMesh
  - impl RefUnwindSafe for DrawMesh

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DrawMesh;
```

---

## Struct RenderMeshInstanceGpuBuilder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.RenderMeshInstanceGpuBuilder.html

**Contents:**
- Struct RenderMeshInstanceGpuBuilder Copy item path
- Fields§
- Auto Trait Implementations§
  - impl Freeze for RenderMeshInstanceGpuBuilder
  - impl RefUnwindSafe for RenderMeshInstanceGpuBuilder
  - impl Send for RenderMeshInstanceGpuBuilder
  - impl Sync for RenderMeshInstanceGpuBuilder
  - impl Unpin for RenderMeshInstanceGpuBuilder
  - impl UnwindSafe for RenderMeshInstanceGpuBuilder
- Blanket Implementations§

Information that is gathered during the parallel portion of mesh extraction when GPU mesh uniform building is enabled.

From this, the MeshInputUniform and RenderMeshInstanceGpu are prepared.

Data that will be placed on the RenderMeshInstanceGpu.

The current transform.

Four 16-bit unsigned normalized UV values packed into a UVec2:

The index of the previous mesh input.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct RenderMeshInstanceGpuBuilder {
    pub shared: RenderMeshInstanceShared,
    pub world_from_local: Affine3,
    pub lightmap_uv_rect: UVec2,
    pub previous_input_index: Option<NonMaxU32>,
    pub mesh_flags: MeshFlags,
}
```

Example 2 (text):
```text
<--- MSB                   LSB --->
                        +---- min v ----+ +---- min u ----+
    lightmap_uv_rect.x: vvvvvvvv vvvvvvvv uuuuuuuu uuuuuuuu,
                        +---- max v ----+ +---- max u ----+
    lightmap_uv_rect.y: VVVVVVVV VVVVVVVV UUUUUUUU UUUUUUUU,

(MSB: most significant bit; LSB: least significant bit.)
```

---

## Struct MaterialPlugin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialPlugin.html

**Contents:**
- Struct MaterialPlugin Copy item path
- Fields§
- Trait Implementations§
  - impl<M> Default for MaterialPlugin<M>where M: Material,
    - fn default() -> MaterialPlugin<M>
  - impl<M> Plugin for MaterialPlugin<M>where M: Material, <M as AsBindGroup>::Data: PartialEq + Eq + Hash + Clone,
    - fn build(&self, app: &mut App)
    - fn ready(&self, _app: &App) -> bool
    - fn finish(&self, _app: &mut App)
    - fn cleanup(&self, _app: &mut App)

Adds the necessary ECS resources and render logic to enable rendering entities using the given Material asset type.

Controls if the prepass is enabled for the Material. For more information about what a prepass is, see the bevy_core_pipeline::prepass docs.

When it is enabled, it will automatically add the PrepassPlugin required to make the prepass work on this Material.

Controls if shadows are enabled for the Material.

Debugging flags that can optionally be set when constructing the renderer.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialPlugin<M>where
    M: Material,{
    pub prepass_enabled: bool,
    pub shadows_enabled: bool,
    pub debug_flags: RenderDebugFlags,
    pub _marker: PhantomData<M>,
}
```

---

## Struct LatePrepassBuildIndirectParametersNode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.LatePrepassBuildIndirectParametersNode.html

**Contents:**
- Struct LatePrepassBuildIndirectParametersNode Copy item path
- Trait Implementations§
  - impl FromWorld for LatePrepassBuildIndirectParametersNode
    - fn from_world(world: &mut World) -> LatePrepassBuildIndirectParametersNode
  - impl Node for LatePrepassBuildIndirectParametersNode
    - fn update(&mut self, world: &mut World)
    - fn run<'w>( &self, _: &mut RenderGraphContext<'_>, render_context: &mut RenderContext<'w>, world: &'w World, ) -> Result<(), NodeRunError>
    - fn input(&self) -> Vec<SlotInfo>
    - fn output(&self) -> Vec<SlotInfo>
- Auto Trait Implementations§

The render node for the part of the indirect parameter building pass that draws the meshes that are potentially visible on this frame but weren’t visible on the previous frame.

This node runs a compute shader on the output of the LateGpuPreprocessNode in order to transform the IndirectParametersGpuMetadata into properly-formatted IndirectParametersIndexed and IndirectParametersNonIndexed.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct LatePrepassBuildIndirectParametersNode { /* private fields */ }
```

---

## Struct MaterialPipeline Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MaterialPipeline.html

**Contents:**
- Struct MaterialPipeline Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for MaterialPipeline
    - fn clone(&self) -> MaterialPipeline
    - fn clone_from(&mut self, source: &Self)
  - impl Resource for MaterialPipelinewhere MaterialPipeline: Send + Sync + 'static,
- Auto Trait Implementations§
  - impl Freeze for MaterialPipeline
  - impl !RefUnwindSafe for MaterialPipeline

Render pipeline data for a given Material.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MaterialPipeline {
    pub mesh_pipeline: MeshPipeline,
}
```

---

## Struct SetMeshBindGroup Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.SetMeshBindGroup.html

**Contents:**
- Struct SetMeshBindGroup Copy item path
- Trait Implementations§
  - impl<P, const I: usize> RenderCommand<P> for SetMeshBindGroup<I>where P: PhaseItem,
    - type Param = (Res<'static, RenderDevice>, Res<'static, MeshBindGroups>, Res<'static, RenderMeshInstances>, Res<'static, SkinUniforms>, Res<'static, MorphIndices>, Res<'static, RenderLightmaps>)
    - type ViewQuery = Has<MotionVectorPrepass>
    - type ItemQuery = ()
    - fn render<'w>( item: &P, has_motion_vector_prepass: bool, _item_query: Option<()>, _: <<SetMeshBindGroup<I> as RenderCommand<P>>::Param as SystemParam>::Item<'w, '_>, pass: &mut TrackedRenderPass<'w>, ) -> RenderCommandResult
- Auto Trait Implementations§
  - impl<const I: usize> Freeze for SetMeshBindGroup<I>
  - impl<const I: usize> RefUnwindSafe for SetMeshBindGroup<I>

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (javascript):
```javascript
pub struct SetMeshBindGroup<const I: usize>;
```

---

## Struct Render Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/render/struct.Render.html

**Contents:**
- Struct Render Copy item path
- Implementations§
  - impl Render
    - pub fn base_schedule() -> Schedule
- Trait Implementations§
  - impl Clone for Render
    - fn clone(&self) -> Render
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for Render
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

The main render schedule.

Sets up the base structure of the rendering Schedule.

The sets defined in this enum are configured to run in order.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Render;
```

---

## Struct DistanceFog Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.DistanceFog.html

**Contents:**
- Struct DistanceFog Copy item path
  - §Falloff
  - §Example
  - §Material Override
- Fields§
- Trait Implementations§
  - impl Clone for DistanceFog
    - fn clone(&self) -> DistanceFog
    - fn clone_from(&mut self, source: &Self)
  - impl Component for DistanceFogwhere DistanceFog: Send + Sync + 'static,

Configures the “classic” computer graphics distance fog effect, in which objects appear progressively more covered in atmospheric haze the further away they are from the camera. Affects meshes rendered via the PBR StandardMaterial.

The rate at which fog intensity increases with distance is controlled by the falloff mode. Currently, the following fog falloff modes are supported:

Once enabled for a specific camera, the fog effect can also be disabled for individual StandardMaterial instances via the fog_enabled flag.

The color of the fog effect.

Tip: The alpha channel of the color can be used to “modulate” the fog effect without changing the fog falloff mode or parameters.

Color used to modulate the influence of directional light colors on the fog, where the view direction aligns with each directional light direction, producing a “glow” or light dispersion effect. (e.g. around the sun)

Use Color::NONE to disable the effect.

The exponent applied to the directional light alignment calculation. A higher value means a more concentrated “glow”.

Determines which falloff mode to use, and its parameters.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DistanceFog {
    pub color: Color,
    pub directional_light_color: Color,
    pub directional_light_exponent: f32,
    pub falloff: FogFalloff,
}
```

Example 2 (unknown):
```unknown
commands.spawn((
    // Setup your camera as usual
    Camera3d::default(),
    // Add fog to the same entity
    DistanceFog {
        color: Color::WHITE,
        falloff: FogFalloff::Exponential { density: 1e-3 },
        ..Default::default()
    },
));
```

---
