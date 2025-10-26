# Crates-Rs-Bevy - Asset

**Pages:** 86

---

## Struct ImageLoader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.ImageLoader.html

**Contents:**
- Struct ImageLoader Copy item path
- Implementations§
  - impl ImageLoader
    - pub const SUPPORTED_FORMATS: &'static [ImageFormat]
    - pub const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str]
    - pub fn new(supported_compressed_formats: CompressedImageFormats) -> ImageLoader
- Trait Implementations§
  - impl AssetLoader for ImageLoader
    - type Asset = Image
    - type Settings = ImageLoaderSettings

Loader for images that can be read by the image crate.

Full list of supported formats.

Gets the list of file extensions for all formats.

Creates a new image loader that supports the provided formats.

Processes the asset in an asynchronous closure.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ImageLoader { /* private fields */ }
```

---

## Enum UntypedAssetConversionError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.UntypedAssetConversionError.html

**Contents:**
- Enum UntypedAssetConversionError Copy item path
- Variants (Non-exhaustive)§
  - TypeIdMismatch
    - Fields
- Trait Implementations§
  - impl Clone for UntypedAssetConversionError
    - fn clone(&self) -> UntypedAssetConversionError
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for UntypedAssetConversionError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

Errors preventing the conversion of to/from an UntypedHandle and a Handle.

Caused when trying to convert an UntypedHandle into a Handle of the wrong type.

The expected TypeId of the Handle being converted to.

The TypeId of the UntypedHandle being converted from.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[non_exhaustive]pub enum UntypedAssetConversionError {
    TypeIdMismatch {
        expected: TypeId,
        found: TypeId,
    },
}
```

---

## Macro load_embedded_asset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/macro.load_embedded_asset.html

**Contents:**
- Macro load_embedded_asset Copy item path
- §Syntax
- §Usage

Load an embedded asset.

This is useful if the embedded asset in question is not publicly exposed, but you need to use it internally.

This macro takes two arguments and an optional third one:

The advantage compared to using directly AssetServer::load is:

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! load_embedded_asset {
    (@get: $path: literal, $provider: expr) => { ... };
    ($provider: expr, $path: literal, $settings: expr) => { ... };
    ($provider: expr, $path: literal) => { ... };
}
```

---

## Struct AddAsyncError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.AddAsyncError.html

**Contents:**
- Struct AddAsyncError Copy item path
- Trait Implementations§
  - impl Clone for AddAsyncError
    - fn clone(&self) -> AddAsyncError
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for AddAsyncError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for AddAsyncError
    - fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>
  - impl Error for AddAsyncError

An error that occurs while resolving an asset added by add_async.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AddAsyncError { /* private fields */ }
```

---

## Struct LoadedFolder Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.LoadedFolder.html

**Contents:**
- Struct LoadedFolder Copy item path
- Fields§
- Trait Implementations§
  - impl TypePath for LoadedFolder
    - fn type_path() -> &'static str
    - fn short_type_path() -> &'static str
    - fn type_ident() -> Option<&'static str>
    - fn crate_name() -> Option<&'static str>
    - fn module_path() -> Option<&'static str>
  - impl VisitAssetDependencies for LoadedFolder

A “loaded folder” containing handles for all assets stored in a given AssetPath.

This is produced by AssetServer::load_folder.

The handles of all assets stored in the folder.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct LoadedFolder {
    pub handles: Vec<UntypedHandle>,
}
```

---

## Struct ImageSamplerDescriptor Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.ImageSamplerDescriptor.html

**Contents:**
- Struct ImageSamplerDescriptor Copy item path
- Fields§
- Implementations§
  - impl ImageSamplerDescriptor
    - pub fn linear() -> ImageSamplerDescriptor
      - Examples found in repository?
    - pub fn nearest() -> ImageSamplerDescriptor
    - pub fn as_wgpu(&self) -> SamplerDescriptor<Option<&str>>
- Trait Implementations§
  - impl Clone for ImageSamplerDescriptor

Indicates to an ImageLoader how an Image should be sampled.

As this type is part of the ImageLoaderSettings, it will be serialized to an image asset .meta file which might require a migration in case of a breaking change.

This types mirrors SamplerDescriptor, but that might change in future versions.

How to deal with out of bounds accesses in the u (i.e. x) direction.

How to deal with out of bounds accesses in the v (i.e. y) direction.

How to deal with out of bounds accesses in the w (i.e. z) direction.

How to filter the texture when it needs to be magnified (made larger).

How to filter the texture when it needs to be minified (made smaller).

How to filter between mip map levels

Minimum level of detail (i.e. mip level) to use.

Maximum level of detail (i.e. mip level) to use.

If this is enabled, this is a comparison sampler using the given comparison function.

Must be at least 1. If this is not 1, all filter modes must be linear.

Border color to use when address_mode is ImageAddressMode::ClampToBorder.

Returns a sampler descriptor with Linear min and mag filters

Returns a sampler descriptor with Nearest min and mag filters

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ImageSamplerDescriptor {
    pub label: Option<String>,
    pub address_mode_u: ImageAddressMode,
    pub address_mode_v: ImageAddressMode,
    pub address_mode_w: ImageAddressMode,
    pub mag_filter: ImageFilterMode,
    pub min_filter: ImageFilterMode,
    pub mipmap_filter: ImageFilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: Option<ImageCompareFunction>,
    pub anisotropy_clamp: u16,
    pub border_color: Option<ImageSamplerBorderColor>,
}
```

Example 2 (javascript):
```javascript
355fn uv_debug_texture() -> Image {
356    use bevy::{asset::RenderAssetUsages, render::render_resource::*};
357    const TEXTURE_SIZE: usize = 7;
358
359    let mut palette = [
360        164, 164, 164, 255, 168, 168, 168, 255, 153, 153, 153, 255, 139, 139, 139, 255, 153, 153,
361        153, 255, 177, 177, 177, 255, 159, 159, 159, 255,
362    ];
363
364    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
365    for y in 0..TEXTURE_SIZE {
366        let offset = TEXTURE_SIZE * y * 4;
367        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
368       
...
```

---

## Struct BuildIndirectParametersPipeline Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.BuildIndirectParametersPipeline.html

**Contents:**
- Struct BuildIndirectParametersPipeline Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for BuildIndirectParametersPipeline
    - fn clone(&self) -> BuildIndirectParametersPipeline
    - fn clone_from(&mut self, source: &Self)
  - impl SpecializedComputePipeline for BuildIndirectParametersPipeline
    - type Key = BuildIndirectParametersPipelineKey
    - fn specialize( &self, key: <BuildIndirectParametersPipeline as SpecializedComputePipeline>::Key, ) -> ComputePipelineDescriptor
- Auto Trait Implementations§

The pipeline for the indirect parameter building shader.

The bind group layout for the compute shader.

The shader asset handle.

The pipeline ID for the compute shader.

This gets filled in prepare_preprocess_pipelines.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct BuildIndirectParametersPipeline {
    pub bind_group_layout: BindGroupLayout,
    pub shader: Handle<Shader>,
    pub pipeline_id: Option<CachedComputePipelineId>,
}
```

---

## Derive Macro Asset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/derive.Asset.html

**Contents:**
- Derive Macro Asset Copy item path

Implement the Asset trait.

**Examples:**

Example 1 (unknown):
```unknown
#[derive(Asset)]
{
    // Attributes available to this derive:
    #[dependency]
}
```

---

## Trait DirectAssetAccessExt Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/trait.DirectAssetAccessExt.html

**Contents:**
- Trait DirectAssetAccessExt Copy item path
- Required Methods§
    - fn add_asset<A>(&mut self, asset: impl Into<A>) -> Handle<A>where A: Asset,
    - fn load_asset<'a, A>(&self, path: impl Into<AssetPath<'a>>) -> Handle<A>where A: Asset,
    - fn load_asset_with_settings<'a, A, S>( &self, path: impl Into<AssetPath<'a>>, settings: impl Fn(&mut S) + Send + Sync + 'static, ) -> Handle<A>where A: Asset, S: Settings,
- Dyn Compatibility§
- Implementors§
  - impl DirectAssetAccessExt for World

An extension trait for methods for working with assets directly from a World.

Insert an asset similarly to Assets::add.

Load an asset similarly to AssetServer::load.

Load an asset with settings, similarly to AssetServer::load_with_settings.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait DirectAssetAccessExt {
    // Required methods
    fn add_asset<A>(&mut self, asset: impl Into<A>) -> Handle<A>
       where A: Asset;
    fn load_asset<'a, A>(&self, path: impl Into<AssetPath<'a>>) -> Handle<A>
       where A: Asset;
    fn load_asset_with_settings<'a, A, S>(
        &self,
        path: impl Into<AssetPath<'a>>,
        settings: impl Fn(&mut S) + Send + Sync + 'static,
    ) -> Handle<A>
       where A: Asset,
             S: Settings;
}
```

---

## Struct AssetHandleProvider Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.AssetHandleProvider.html

**Contents:**
- Struct AssetHandleProvider Copy item path
- Implementations§
  - impl AssetHandleProvider
    - pub fn reserve_handle(&self) -> UntypedHandle
- Trait Implementations§
  - impl Clone for AssetHandleProvider
    - fn clone(&self) -> AssetHandleProvider
    - fn clone_from(&mut self, source: &Self)
- Auto Trait Implementations§
  - impl Freeze for AssetHandleProvider

Provides Handle and UntypedHandle for a specific asset type. This should only be used for one specific asset type.

Reserves a new strong UntypedHandle (with a new UntypedAssetId). The stored Asset TypeId in the UntypedHandle will match the Asset TypeId assigned to this AssetHandleProvider.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AssetHandleProvider { /* private fields */ }
```

---

## Struct AudioLoader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/struct.AudioLoader.html

**Contents:**
- Struct AudioLoader Copy item path
- Trait Implementations§
  - impl AssetLoader for AudioLoader
    - type Asset = AudioSource
    - type Settings = ()
    - type Error = Error
    - async fn load( &self, reader: &mut dyn Reader, _settings: &<AudioLoader as AssetLoader>::Settings, _load_context: &mut LoadContext<'_>, ) -> Result<AudioSource, <AudioLoader as AssetLoader>::Error>
    - fn extensions(&self) -> &[&str]
  - impl Default for AudioLoader
    - fn default() -> AudioLoader

Loads files as AudioSource Assets

This asset loader supports different audio formats based on the enable Bevy features. The feature bevy/vorbis enables loading from .ogg files and is enabled by default. Other file endings can be loaded from with additional features: .mp3 with bevy/mp3 .flac with bevy/flac .wav with bevy/wav

Processes the asset in an asynchronous closure.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AudioLoader;
```

---

## Module meta Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/meta/index.html

**Contents:**
- Module meta Copy item path
- Structs§
- Enums§
- Constants§
- Traits§
- Type Aliases§

bevy::assetModule meta Copy item pathSource Structs§AssetMetaAsset metadata that informs how an Asset should be handled by the asset system.AssetMetaMinimalThis is a minimal counterpart to AssetMeta that exists to speed up (or enable) serialization in cases where the whole AssetMeta isn’t necessary.ProcessDependencyInfoInformation about a dependency used to process an asset. This is used to determine whether an asset’s “process dependency” has changed.ProcessedInfoInfo produced by the AssetProcessor for a given processed asset. This is used to determine if an asset source file (or its dependencies) has changed.ProcessedInfoMinimalThis is a minimal counterpart to ProcessedInfo that exists to speed up serialization in cases where the whole ProcessedInfo isn’t necessary.Enums§AssetActionConfigures how an asset source file should be handled by the asset system.AssetActionMinimalThis is a minimal counterpart to AssetAction that exists to speed up (or enable) serialization in cases where the whole AssetAction isn’t necessary.Constants§META_FORMAT_VERSIONTraits§AssetMetaDynA dynamic type-erased counterpart to AssetMeta that enables passing around and interacting with AssetMeta without knowing its type.SettingsSettings used by the asset system, such as by AssetLoader, Process, and AssetSaverType Aliases§AssetHashMetaTransform

---

## Trait AssetLoader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/trait.AssetLoader.html

**Contents:**
- Trait AssetLoader Copy item path
- Required Associated Types§
    - type Asset: Asset
    - type Settings: Settings + Default + Serialize + for<'a> Deserialize<'a>
    - type Error: Into<BevyError>
- Required Methods§
    - fn load( &self, reader: &mut dyn Reader, settings: &Self::Settings, load_context: &mut LoadContext<'_>, ) -> impl ConditionalSendFuture
- Provided Methods§
    - fn extensions(&self) -> &[&str]
- Dyn Compatibility§

Loads an Asset from a given byte Reader. This can accept AssetLoader::Settings, which configure how the Asset should be loaded.

This trait is generally used in concert with AssetReader to load assets from a byte source.

For a complementary version of this trait that can save assets, see AssetSaver.

The top level Asset loaded by this AssetLoader.

The settings type used by this AssetLoader.

The type of error which could be encountered by this loader.

Asynchronously loads AssetLoader::Asset (and any other labeled assets) from the bytes provided by Reader.

Returns a list of extensions supported by this AssetLoader, without the preceding dot. Note that users of this AssetLoader may choose to load files with a non-matching extension.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

The () loader should never be called. This implementation exists to make the meta format nicer to work with.

**Examples:**

Example 1 (unknown):
```unknown
pub trait AssetLoader:
    Send
    + Sync
    + 'static {
    type Asset: Asset;
    type Settings: Settings + Default + Serialize + for<'a> Deserialize<'a>;
    type Error: Into<BevyError>;

    // Required method
    fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> impl ConditionalSendFuture;

    // Provided method
    fn extensions(&self) -> &[&str] { ... }
}
```

---

## Module transformer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/transformer/index.html

**Contents:**
- Module transformer Copy item path
- Structs§
- Traits§

bevy::assetModule transformer Copy item pathSource Structs§IdentityAssetTransformerAn identity AssetTransformer which infallibly returns the input Asset on transformation.]TransformedAssetAn Asset (and any “sub assets”) intended to be transformedTransformedSubAssetA labeled sub-asset of TransformedAssetTraits§AssetTransformerTransforms an Asset of a given AssetTransformer::AssetInput type to an Asset of AssetTransformer::AssetOutput type.

---

## Macro load_internal_asset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/macro.load_internal_asset.html

**Contents:**
- Macro load_internal_asset Copy item path

Loads an “internal” asset by embedding the string stored in the given path_str and associates it with the given handle.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! load_internal_asset {
    ($app: ident, $handle: expr, $path_str: expr, $loader: expr) => { ... };
    ($app: ident, $handle: ident, $path_str: expr, $loader: expr $(, $param:expr)+) => { ... };
}
```

---

## Crate text Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/text/index.html

**Contents:**
- Crate text Copy item path
- §Font
- §TextPipeline
- Modules§
- Structs§
- Enums§
- Constants§
- Traits§
- Functions§
- Type Aliases§

This crate provides the tools for positioning and rendering text in Bevy.

Fonts contain information for drawing glyphs, which are shapes that typically represent a single character, but in some cases part of a “character” (grapheme clusters) or more than one character (ligatures).

A font face is part of a font family, and is distinguished by its style (e.g. italic), its weight (e.g. bold) and its stretch (e.g. condensed).

In Bevy, Fonts are loaded by the FontLoader as assets.

The TextPipeline resource does all of the heavy lifting for rendering text.

UI Text is first measured by creating a TextMeasureInfo in TextPipeline::create_text_measure, which is called by the measure_text_system system of bevy_ui.

Note that text measurement is only relevant in a UI context.

With the actual text bounds defined, the bevy_ui::widget::text::text_system system (in a UI context) or bevy_sprite::text2d::update_text2d_layout system (in a 2d world space context) passes it into TextPipeline::queue_text, which:

---

## Struct UnknownTyped Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.UnknownTyped.html

**Contents:**
- Struct UnknownTyped Copy item path
- Auto Trait Implementations§
  - impl Freeze for UnknownTyped
  - impl RefUnwindSafe for UnknownTyped
  - impl Send for UnknownTyped
  - impl Sync for UnknownTyped
  - impl Unpin for UnknownTyped
  - impl UnwindSafe for UnknownTyped
- Blanket Implementations§
  - impl<T> Any for Twhere T: 'static + ?Sized,

NestedLoader does not know what type of asset it will be loading.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct UnknownTyped(/* private fields */);
```

---

## Enum GltfAssetLabel Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/enum.GltfAssetLabel.html

**Contents:**
- Enum GltfAssetLabel Copy item path
- Variants§
  - Scene(usize)
  - Node(usize)
  - Mesh(usize)
  - Primitive
    - Fields
  - MorphTarget
    - Fields
  - Texture(usize)

Labels that can be used to load part of a glTF

You can use GltfAssetLabel::from_asset to add it to an asset path

Or when formatting a string for the path

Scene{}: glTF Scene as a Bevy Scene

Node{}: glTF Node as a GltfNode

Mesh{}: glTF Mesh as a GltfMesh

Mesh{}/Primitive{}: glTF Primitive as a Bevy Mesh

Index of the mesh for this primitive

Index of this primitive in its parent mesh

Mesh{}/Primitive{}/MorphTargets: Morph target animation data for a glTF Primitive as a Bevy Image

Index of the mesh for this primitive

Index of this primitive in its parent mesh

Texture{}: glTF Texture as a Bevy Image

Material{}: glTF Material as a Bevy StandardMaterial

Index of this material

Used to set the Face of the material, useful if it is used with negative scale

DefaultMaterial: glTF’s default Material as a Bevy StandardMaterial

Animation{}: glTF Animation as Bevy AnimationClip

Skin{}: glTF mesh skin as GltfSkin

Skin{}/InverseBindMatrices: glTF mesh skin matrices as Bevy SkinnedMeshInverseBindposes

Add this label to an asset path

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum GltfAssetLabel {
    Scene(usize),
    Node(usize),
    Mesh(usize),
    Primitive {
        mesh: usize,
        primitive: usize,
    },
    MorphTarget {
        mesh: usize,
        primitive: usize,
    },
    Texture(usize),
    Material {
        index: usize,
        is_scale_inverted: bool,
    },
    DefaultMaterial,
    Animation(usize),
    Skin(usize),
    InverseBindMatrices(usize),
}
```

Example 2 (javascript):
```javascript
fn load_gltf_scene(asset_server: Res<AssetServer>) {
    let gltf_scene: Handle<Scene> = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf"));
}
```

Example 3 (javascript):
```javascript
fn load_gltf_scene(asset_server: Res<AssetServer>) {
    let gltf_scene: Handle<Scene> = asset_server.load(format!("models/FlightHelmet/FlightHelmet.gltf#{}", GltfAssetLabel::Scene(0)));
}
```

Example 4 (javascript):
```javascript
fn load_gltf_scene(asset_server: Res<AssetServer>) {
    let gltf_scene: Handle<Scene> = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf"));
}
```

---

## Struct GltfLoader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/struct.GltfLoader.html

**Contents:**
- Struct GltfLoader Copy item path
- Fields§
- Implementations§
  - impl GltfLoader
    - pub async fn load_gltf<'a, 'b, 'c>( loader: &GltfLoader, bytes: &'a [u8], load_context: &'b mut LoadContext<'c>, settings: &'b GltfLoaderSettings, ) -> Result<Gltf, GltfError>
- Trait Implementations§
  - impl AssetLoader for GltfLoader
    - type Asset = Gltf
    - type Settings = GltfLoaderSettings
    - type Error = GltfError

Loads glTF files with all of their data as their corresponding bevy representations.

List of compressed image formats handled by the loader.

Custom vertex attributes that will be recognized when loading a glTF file.

Keys must be the attribute names as found in the glTF data, which must start with an underscore. See this section of the glTF specification for additional details on custom attributes.

Arc to default ImageSamplerDescriptor.

How to convert glTF coordinates on import. Assuming glTF cameras, glTF lights, and glTF meshes had global identity transforms, their Bevy Transform::forward will be pointing in the following global directions:

The default is false.

Loads an entire glTF file.

Processes the asset in an asynchronous closure.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GltfLoader {
    pub supported_compressed_formats: CompressedImageFormats,
    pub custom_vertex_attributes: HashMap<Box<str>, MeshVertexAttribute>,
    pub default_sampler: Arc<Mutex<ImageSamplerDescriptor>>,
    pub default_use_model_forward_direction: bool,
}
```

---

## Struct MissingAssetLoaderForTypeIdError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.MissingAssetLoaderForTypeIdError.html

**Contents:**
- Struct MissingAssetLoaderForTypeIdError Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for MissingAssetLoaderForTypeIdError
    - fn clone(&self) -> MissingAssetLoaderForTypeIdError
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for MissingAssetLoaderForTypeIdError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for MissingAssetLoaderForTypeIdError
    - fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>

An error that occurs when an AssetLoader is not registered for a given Asset TypeId.

The type ID that was not found.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MissingAssetLoaderForTypeIdError {
    pub type_id: TypeId,
}
```

---

## Macro load_internal_binary_asset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/macro.load_internal_binary_asset.html

**Contents:**
- Macro load_internal_binary_asset Copy item path

Loads an “internal” binary asset by embedding the bytes stored in the given path_str and associates it with the given handle.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! load_internal_binary_asset {
    ($app: ident, $handle: expr, $path_str: expr, $loader: expr) => { ... };
}
```

---

## Crate gltf Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/index.html

**Contents:**
- Crate gltf Copy item path
- §Quick Start
- §Loading parts of a glTF asset
  - §Using Gltf
  - §Asset Labels
- Modules§
- Structs§
- Enums§

Plugin providing an AssetLoader and type definitions for loading glTF 2.0 (a standard 3D scene definition format) files in Bevy.

The glTF 2.0 specification defines the format of the glTF files.

Here’s how to spawn a simple glTF scene

If you want to access part of the asset, you can load the entire Gltf using the AssetServer. Once the Handle<Gltf> is loaded you can then use it to access named parts of it.

The glTF loader let’s you specify labels that let you target specific parts of the glTF.

Be careful when using this feature, if you misspell a label it will simply ignore it without warning.

You can use GltfAssetLabel to ensure you are using the correct label.

**Examples:**

Example 1 (unknown):
```unknown
fn spawn_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        // This is equivalent to "models/FlightHelmet/FlightHelmet.gltf#Scene0"
        // The `#Scene0` label here is very important because it tells bevy to load the first scene in the glTF file.
        // If this isn't specified bevy doesn't know which part of the glTF file to load.
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf"))),
        // You can use the transform to give it a position
        Transform::from_xyz(2.0, 0.0, -5.0),
...
```

Example 2 (javascript):
```javascript
// Holds the scene handle
#[derive(Resource)]
struct HelmetScene(Handle<Gltf>);

fn load_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf = asset_server.load("models/FlightHelmet/FlightHelmet.gltf");
    commands.insert_resource(HelmetScene(gltf));
}

fn spawn_gltf_objects(
    mut commands: Commands,
    helmet_scene: Res<HelmetScene>,
    gltf_assets: Res<Assets<Gltf>>,
    mut loaded: Local<bool>,
) {
    // Only do this once
    if *loaded {
        return;
    }
    // Wait until the scene is loaded
    let Some(gltf) = gltf_assets.get(&helmet_scene.0) else {
  
...
```

---

## Enum UntypedHandle Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.UntypedHandle.html

**Contents:**
- Enum UntypedHandle Copy item path
- Variants§
  - Strong(Arc<StrongHandle>)
  - Uuid
    - Fields
- Implementations§
  - impl UntypedHandle
    - pub fn id(&self) -> UntypedAssetId
      - Examples found in repository?
    - pub fn path(&self) -> Option<&AssetPath<'static>>

An untyped variant of Handle, which internally stores the Asset type information at runtime as a TypeId instead of encoding it in the compile-time type. This allows handles across Asset types to be stored together and compared.

See Handle for more information.

A strong handle, which will keep the referenced Asset alive until all strong handles are dropped.

A UUID handle, which does not keep the referenced Asset alive.

An identifier that records the underlying asset type.

The UUID provided during asset registration.

Returns the UntypedAssetId for the referenced asset.

Returns the path if this is (1) a strong handle and (2) the asset has a path

Returns the TypeId of the referenced Asset.

Converts to a typed Handle. This will not check if the target Handle type matches.

Converts to a typed Handle. This will check the type when compiled with debug asserts, but it will not check if the target Handle type matches in release builds. Use this as an optimization when you want some degree of validation at dev-time, but you are also very certain that the type actually matches.

Converts to a typed Handle. This will panic if the internal TypeId does not match the given asset type A

Converts to a typed Handle. This will panic if the internal TypeId does not match the given asset type A

The “meta transform” for the strong handle. This will only be Some if the handle is strong and there is a meta transform associated with it.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum UntypedHandle {
    Strong(Arc<StrongHandle>),
    Uuid {
        type_id: TypeId,
        uuid: Uuid,
    },
}
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

---

## Macro weak_handle Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/macro.weak_handle.html

**Contents:**
- Macro weak_handle Copy item path

bevy::assetMacro weak_handle Copy item pathSource macro_rules! weak_handle { ($uuid:expr) => { ... }; }👎Deprecated: Use uuid_handle! instead

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! weak_handle {
    ($uuid:expr) => { ... };
}
```

---

## Enum UntypedAssetIdConversionError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.UntypedAssetIdConversionError.html

**Contents:**
- Enum UntypedAssetIdConversionError Copy item path
- Variants (Non-exhaustive)§
  - TypeIdMismatch
    - Fields
- Trait Implementations§
  - impl Clone for UntypedAssetIdConversionError
    - fn clone(&self) -> UntypedAssetIdConversionError
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for UntypedAssetIdConversionError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

Errors preventing the conversion of to/from an UntypedAssetId and an AssetId.

Caused when trying to convert an UntypedAssetId into an AssetId of the wrong type.

The TypeId of the asset that we are trying to convert to.

The TypeId of the asset that we are trying to convert from.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[non_exhaustive]pub enum UntypedAssetIdConversionError {
    TypeIdMismatch {
        expected: TypeId,
        found: TypeId,
    },
}
```

---

## Enum Handle Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.Handle.html

**Contents:**
- Enum Handle Copy item path
- Variants§
  - Strong(Arc<StrongHandle>)
  - Uuid(Uuid, PhantomData<fn() -> A>)
- Implementations§
  - impl<A> Handle<A>where A: Asset,
    - pub fn id(&self) -> AssetId<A>
      - Examples found in repository?
    - pub fn path(&self) -> Option<&AssetPath<'static>>
    - pub fn is_uuid(&self) -> bool

A handle to a specific Asset of type A. Handles act as abstract “references” to assets, whose data are stored in the Assets<A> resource, avoiding the need to store multiple copies of the same data.

If a Handle is Handle::Strong, the Asset will be kept alive until the Handle is dropped. If a Handle is Handle::Uuid, it does not necessarily reference a live Asset, nor will it keep assets alive.

Modifying a handle will change which existing asset is referenced, but modifying the asset (by mutating the Assets resource) will change the asset for all handles referencing it.

Handle can be cloned. If a Handle::Strong is cloned, the referenced Asset will not be freed until all instances of the Handle are dropped.

Handle::Strong, via StrongHandle also provides access to useful Asset metadata, such as the AssetPath (if it exists).

A “strong” reference to a live (or loading) Asset. If a Handle is Handle::Strong, the Asset will be kept alive until the Handle is dropped. Strong handles also provide access to additional asset metadata.

A reference to an Asset using a stable-across-runs / const identifier. Dropping this handle will not result in the asset being dropped.

Returns the AssetId of this Asset.

Returns the path if this is (1) a strong handle and (2) the asset has a path

Returns true if this is a uuid handle.

Returns true if this is a strong handle.

Converts this Handle to an “untyped” / “generic-less” UntypedHandle, which stores the Asset type information inside UntypedHandle. This will return UntypedHandle::Strong for Handle::Strong and UntypedHandle::Uuid for Handle::Uuid.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum Handle<A>where
    A: Asset,{
    Strong(Arc<StrongHandle>),
    Uuid(Uuid, PhantomData<fn() -> A>),
}
```

Example 2 (javascript):
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

Example 3 (javascript):
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

Example 4 (javascript):
```javascript
169fn print_counts(
170    time: Res<Time>,
171    mut timer: Local<PrintingTimer>,
172    texts: Query<&ViewVisibility, With<Text2d>>,
173    atlases: Res<FontAtlasSets>,
174    font: Res<FontHandle>,
175) {
176    timer.tick(time.delta());
177    if !timer.just_finished() {
178        return;
179    }
180
181    let num_atlases = atlases
182        .get(font.0.id())
183        .map(|set| set.iter().map(|atlas| atlas.1.len()).sum())
184        .unwrap_or(0);
185
186    let visible_texts = texts.iter().filter(|visibility| visibility.get()).count();
187
188    info!(
189        "Texts: {} Visib
...
```

---

## Struct AssetPath Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.AssetPath.html

**Contents:**
- Struct AssetPath Copy item path
- Implementations§
  - impl<'a> AssetPath<'a>
    - pub fn parse(asset_path: &'a str) -> AssetPath<'a>
      - §Panics
    - pub fn try_parse( asset_path: &'a str, ) -> Result<AssetPath<'a>, ParseAssetPathError>
    - pub fn from_path_buf(path_buf: PathBuf) -> AssetPath<'a>
    - pub fn from_path(path: &'a Path) -> AssetPath<'a>
      - Examples found in repository?
    - pub fn source(&self) -> &AssetSourceId<'_>

Represents a path to an asset in a “virtual filesystem”.

Asset paths consist of three main parts:

Asset paths are generally constructed (and visualized) as strings:

AssetPath implements From for &'static str, &'static Path, and &'a String, which allows us to optimize the static cases. This means that the common case of asset_server.load("my_scene.scn") when it creates and clones internal owned AssetPaths. This also means that you should use AssetPath::parse in cases where &str is the explicit type.

Creates a new AssetPath from a string in the asset path format:

Prefer From<'static str> for static strings, as this will prevent allocations and reference counting for AssetPath::into_owned.

Panics if the asset path is in an invalid format. Use AssetPath::try_parse for a fallible variant

Creates a new AssetPath from a string in the asset path format:

Prefer From<'static str> for static strings, as this will prevent allocations and reference counting for AssetPath::into_owned.

This will return a ParseAssetPathError if asset_path is in an invalid format.

Creates a new AssetPath from a PathBuf.

Creates a new AssetPath from a Path.

Gets the “asset source”, if one was defined. If none was defined, the default source will be used.

Gets the “sub-asset label”.

Gets the “sub-asset label”.

Gets the path to the asset in the “virtual filesystem”.

Gets the path to the asset in the “virtual filesystem” without a label (if a label is currently set).

Removes a “sub-asset label” from this AssetPath, if one was set.

Takes the “sub-asset label” from this AssetPath, if one was set.

Returns this asset path with the given label. This will replace the previous label if it exists.

Returns this asset path with the given asset source. This will replace the previous asset source if it exists.

Returns an AssetPath for the parent folder of this path, if there is a parent folder in the path.

Converts this into an “owned” value. If internally a value is borrowed, it will be cloned into an “owned Arc”. If internally a value is a static reference, the static reference will be used unchanged. If internally a value is an “owned Arc”, it will remain unchanged.

Clones this into an “owned” value. If internally a value is borrowed, it will be cloned into an “owned Arc”. If internally a value is a static reference, the static reference will be used unchanged. If internally a value is an “owned Arc”, the Arc will be cloned.

Resolves a relative asset path via concatenation. The 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct AssetPath<'a> { /* private fields */ }
```

Example 2 (javascript):
```javascript
// This loads the `my_scene.scn` base asset from the default asset source.
let scene: Handle<Scene> = asset_server.load("my_scene.scn");

// This loads the `PlayerMesh` labeled asset from the `my_scene.scn` base asset in the default asset source.
let mesh: Handle<Mesh> = asset_server.load("my_scene.scn#PlayerMesh");

// This loads the `my_scene.scn` base asset from a custom 'remote' asset source.
let scene: Handle<Scene> = asset_server.load("remote://my_scene.scn");
```

Example 3 (javascript):
```javascript
30fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
31    commands.spawn(Camera2d);
32
33    // Now we can load the asset using our new asset source.
34    //
35    // The actual file path relative to workspace root is
36    // "examples/asset/files/bevy_pixel_light.png".
37    let path = Path::new("bevy_pixel_light.png");
38    let source = AssetSourceId::from("example_files");
39    let asset_path = AssetPath::from_path(path).with_source(source);
40
41    // You could also parse this URL-like string representation for the asset
42    // path.
43    assert_eq!(asset_path, "ex
...
```

Example 4 (javascript):
```javascript
35fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
36    commands.spawn(Camera2d);
37
38    // Each example is its own crate (with name from [[example]] in Cargo.toml).
39    let crate_name = "embedded_asset";
40
41    // The actual file path relative to workspace root is
42    // "examples/asset/files/bevy_pixel_light.png".
43    //
44    // We omit the "examples/asset" from the embedded_asset! call and replace it
45    // with the crate name.
46    let path = Path::new(crate_name).join("files/bevy_pixel_light.png");
47    let source = AssetSourceId::from("embedded");
48    
...
```

---

## Crate ron Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/ron/index.html

**Contents:**
- Crate ron Copy item path
- §Rusty Object Notation
  - §Example
  - §RON syntax overview
  - §Specification
  - §Why RON?
    - §Example in JSON
    - §Same example in RON
  - §Quickstart
    - §Cargo.toml

RON is a simple readable data serialization format that looks similar to Rust syntax. It’s designed to support all of Serde’s data model, so structs, enums, tuples, arrays, generic maps, and primitive values.

Note: Serde’s data model represents fixed-size Rust arrays as tuple (instead of as list)

RON also supports several extensions, which are documented here.

RON’s formal and complete grammar is available here.

There also is a very basic, work in progress specification available on the wiki page.

Note the following advantages of RON over JSON:

RON requires struct, enum, and variant names to be valid Rust identifiers and will reject invalid ones created by #[serde(rename = "...")] at serialization / deserialization time.

RON is not designed to be a fully self-describing format (unlike JSON) and is thus not guaranteed to work when deserialize_any is used instead of its typed alternatives. In particular, the following Serde attributes only have limited support:

While data structures with any of these attributes should generally roundtrip through RON, some restrictions apply 3 and their textual representation may not always match your expectation:

Furthermore, serde imposes the following restrictions for data to roundtrip:

While RON offers a best-effort implementation for #[serde(flatten)], it may be unsupported in further cases and combinations not listed above. These limitations stem primarily from serde rather than RON. Enumerating all such cases based on serde’s behavior is nontrivial, so the lists above are not exhaustive.

Please file a new issue if you come across a use case which is not listed among the above restrictions but still breaks.

While RON guarantees roundtrips like Rust -> RON -> Rust for Rust types using non-deserialize_any-based implementations, RON does not yet make any guarantees about roundtrips through ron::Value. For instance, even when RON -> Rust works, RON -> ron::Value -> Rust, or RON -> ron::Value -> RON -> Rust may not work. We plan on improving ron::Value in an upcoming version of RON, though this work is partially blocked on serde#1183.

RON is dual-licensed under Apache-2.0 and MIT.

Any contribution intentionally submitted for inclusion in the work must be provided under the same dual-license terms.

Deserialising an internally, adjacently, or un-tagged enum requires detecting serde’s internal serde::__private::de::content::Content content type so that RON can describe the deserialised data structure in serde’s i

*[Content truncated]*

**Examples:**

Example 1 (ron):
```ron
GameConfig( // optional struct name
    window_size: (800, 600),
    window_title: "PAC-MAN",
    fullscreen: false,

    mouse_sensitivity: 1.4,
    key_bindings: {
        "up": Up,
        "down": Down,
        "left": Left,
        "right": Right,

        // Uncomment to enable WASD controls
        /*
        "W": Up,
        "S": Down,
        "A": Left,
        "D": Right,
        */
    },

    difficulty_options: (
        start_difficulty: Easy,
        adaptive: false,
    ),
)
```

Example 2 (json):
```json
{
   "materials": {
        "metal": {
            "reflectivity": 1.0
        },
        "plastic": {
            "reflectivity": 0.5
        }
   },
   "entities": [
        {
            "name": "hero",
            "material": "metal"
        },
        {
            "name": "monster",
            "material": "plastic"
        }
   ]
}
```

Example 3 (ron):
```ron
Scene( // class name is optional
    materials: { // this is a map
        "metal": (
            reflectivity: 1.0,
        ),
        "plastic": (
            reflectivity: 0.5,
        ),
    },
    entities: [ // this is an array
        (
            name: "hero",
            material: "metal",
        ),
        (
            name: "monster",
            material: "plastic",
        ),
    ],
)
```

Example 4 (toml):
```toml
[dependencies]
ron = "0.8"
serde = { version = "1", features = ["derive"] }
```

---

## Trait Asset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/trait.Asset.html

**Contents:**
- Trait Asset Copy item path
- Dyn Compatibility§
- Implementations on Foreign Types§
  - impl Asset for ()
- Implementors§
  - impl Asset for FrametimeGraphMaterial
  - impl Asset for GltfMesh
  - impl Asset for GltfNode
  - impl Asset for GltfPrimitive
  - impl Asset for GltfSkin

Declares that this type is an asset, which can be loaded and managed by the AssetServer and stored in Assets collections.

Generally, assets are large, complex, and/or expensive to load from disk, and are often authored by artists or designers.

TypePath is largely used for diagnostic purposes, and should almost always be implemented by deriving Reflect on your type. VisitAssetDependencies is used to track asset dependencies, and an implementation is automatically generated when deriving Asset.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait Asset:
    VisitAssetDependencies
    + TypePath
    + Send
    + Sync
    + 'static { }
```

---

## Struct Immediate Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.Immediate.html

**Contents:**
- Struct Immediate Copy item path
- Auto Trait Implementations§
  - impl<'builder, 'reader> Freeze for Immediate<'builder, 'reader>
  - impl<'builder, 'reader> !RefUnwindSafe for Immediate<'builder, 'reader>
  - impl<'builder, 'reader> Send for Immediate<'builder, 'reader>
  - impl<'builder, 'reader> Sync for Immediate<'builder, 'reader>
  - impl<'builder, 'reader> Unpin for Immediate<'builder, 'reader>
  - impl<'builder, 'reader> !UnwindSafe for Immediate<'builder, 'reader>
- Blanket Implementations§
  - impl<T> Any for Twhere T: 'static + ?Sized,

NestedLoader will immediately load an asset when requested.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Immediate<'builder, 'reader> { /* private fields */ }
```

---

## Trait AsAssetId Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/trait.AsAssetId.html

**Contents:**
- Trait AsAssetId Copy item path
- Required Associated Types§
    - type Asset: Asset
- Required Methods§
    - fn as_asset_id(&self) -> AssetId<Self::Asset>
- Dyn Compatibility§
- Implementors§
  - impl AsAssetId for SkinnedMesh
    - type Asset = SkinnedMeshInverseBindposes
  - impl AsAssetId for Mesh3dWireframe

A trait for components that can be used as asset identifiers, e.g. handle wrappers.

The underlying asset type.

Retrieves the asset id from this component.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait AsAssetId: Component {
    type Asset: Asset;

    // Required method
    fn as_asset_id(&self) -> AssetId<Self::Asset>;
}
```

---

## Trait AssetSaver Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/saver/trait.AssetSaver.html

**Contents:**
- Trait AssetSaver Copy item path
- Required Associated Types§
    - type Asset: Asset
    - type Settings: Settings + Default + Serialize + for<'a> Deserialize<'a>
    - type OutputLoader: AssetLoader
    - type Error: Into<Box<dyn Error + Sync + Send>>
- Required Methods§
    - fn save( &self, writer: &mut (dyn AsyncWrite + Unpin + Sync + Send + 'static), asset: SavedAsset<'_, Self::Asset>, settings: &Self::Settings, ) -> impl ConditionalSendFuture
- Dyn Compatibility§
- Implementors§

Saves an Asset of a given AssetSaver::Asset type. AssetSaver::OutputLoader will then be used to load the saved asset in the final deployed application. The saver should produce asset bytes in a format that AssetSaver::OutputLoader can read.

This trait is generally used in concert with AssetWriter to write assets as bytes.

For a complementary version of this trait that can load assets, see AssetLoader.

The top level Asset saved by this AssetSaver.

The settings type used by this AssetSaver.

The type of AssetLoader used to load this Asset

The type of error which could be encountered by this saver.

Saves the given runtime Asset by writing it to a byte format using writer. The passed in settings can influence how the asset is saved.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait AssetSaver:
    Send
    + Sync
    + 'static {
    type Asset: Asset;
    type Settings: Settings + Default + Serialize + for<'a> Deserialize<'a>;
    type OutputLoader: AssetLoader;
    type Error: Into<Box<dyn Error + Sync + Send>>;

    // Required method
    fn save(
        &self,
        writer: &mut (dyn AsyncWrite + Unpin + Sync + Send + 'static),
        asset: SavedAsset<'_, Self::Asset>,
        settings: &Self::Settings,
    ) -> impl ConditionalSendFuture;
}
```

---

## Struct AssetsMutIterator Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.AssetsMutIterator.html

**Contents:**
- Struct AssetsMutIterator Copy item path
- Trait Implementations§
  - impl<'a, A> Iterator for AssetsMutIterator<'a, A>where A: Asset,
    - type Item = (AssetId<A>, &'a mut A)
    - fn next(&mut self) -> Option<<AssetsMutIterator<'a, A> as Iterator>::Item>
    - fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,
    - fn size_hint(&self) -> (usize, Option<usize>)
    - fn count(self) -> usizewhere Self: Sized,
    - fn last(self) -> Option<Self::Item>where Self: Sized,
    - fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>>

A mutable iterator over Assets.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AssetsMutIterator<'a, A>where
    A: Asset,{ /* private fields */ }
```

---

## Struct MissingAssetLoaderForExtensionError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.MissingAssetLoaderForExtensionError.html

**Contents:**
- Struct MissingAssetLoaderForExtensionError Copy item path
- Trait Implementations§
  - impl Clone for MissingAssetLoaderForExtensionError
    - fn clone(&self) -> MissingAssetLoaderForExtensionError
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for MissingAssetLoaderForExtensionError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for MissingAssetLoaderForExtensionError
    - fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>
  - impl Error for MissingAssetLoaderForExtensionError

An error that occurs when an AssetLoader is not registered for a given extension.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MissingAssetLoaderForExtensionError { /* private fields */ }
```

---

## Struct LoadContext Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.LoadContext.html

**Contents:**
- Struct LoadContext Copy item path
- Implementations§
  - impl<'a> LoadContext<'a>
    - pub fn begin_labeled_asset(&self) -> LoadContext<'_>
    - pub fn labeled_asset_scope<A, E>( &mut self, label: String, load: impl FnOnce(&mut LoadContext<'_>) -> Result<A, E>, ) -> Result<Handle<A>, E>where A: Asset,
    - pub fn add_labeled_asset<A>(&mut self, label: String, asset: A) -> Handle<A>where A: Asset,
      - §Warning
    - pub fn add_loaded_labeled_asset<A>( &mut self, label: impl Into<CowArc<'static, str>>, loaded_asset: LoadedAsset<A>, ) -> Handle<A>where A: Asset,
    - pub fn has_labeled_asset<'b>(&self, label: impl Into<CowArc<'b, str>>) -> bool
    - pub fn finish<A>(self, value: A) -> LoadedAsset<A>where A: Asset,

A context that provides access to assets in AssetLoaders, tracks dependencies, and collects asset load state.

Any asset state accessed by LoadContext will be tracked and stored for use in dependency events and asset preprocessing.

Begins a new labeled asset load. Use the returned LoadContext to load dependencies for the new asset and call LoadContext::finish to finalize the asset load. When finished, make sure you call LoadContext::add_loaded_labeled_asset to add the results back to the parent context. Prefer LoadContext::labeled_asset_scope when possible, which will automatically add the labeled LoadContext back to the parent context. LoadContext::begin_labeled_asset exists largely to enable parallel asset loading.

See AssetPath for more on labeled assets.

Creates a new LoadContext for the given label. The load function is responsible for loading an Asset of type A. load will be called immediately and the result will be used to finalize the LoadContext, resulting in a new LoadedAsset, which is registered under the label label.

This exists to remove the need to manually call LoadContext::begin_labeled_asset and then manually register the result with LoadContext::add_loaded_labeled_asset.

See AssetPath for more on labeled assets.

This will add the given asset as a “labeled Asset” with the label label.

This will not assign dependencies to the given asset. If adding an asset with dependencies generated from calls such as LoadContext::load, use LoadContext::labeled_asset_scope or LoadContext::begin_labeled_asset to generate a new LoadContext to track the dependencies for the labeled asset.

See AssetPath for more on labeled assets.

Add a LoadedAsset that is a “labeled sub asset” of the root path of this load context. This can be used in combination with LoadContext::begin_labeled_asset to parallelize sub asset loading.

See AssetPath for more on labeled assets.

Returns true if an asset with the label label exists in this context.

See AssetPath for more on labeled assets.

“Finishes” this context by populating the final Asset value.

Gets the source path for this load context.

Gets the source asset path for this load context.

Reads the asset at the given path and returns its bytes

Returns a handle to an asset of type A with the label label. This LoadContext must produce an asset of the given type and the given label or the dependencies of this asset will never be considered “fully loaded”. However you can call this method before or after adding th

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct LoadContext<'a> { /* private fields */ }
```

Example 2 (javascript):
```javascript
let mut handles = Vec::new();
for i in 0..2 {
    let labeled = load_context.begin_labeled_asset();
    handles.push(std::thread::spawn(move || {
        (i.to_string(), labeled.finish(Image::default()))
    }));
}

for handle in handles {
    let (label, loaded_asset) = handle.join().unwrap();
    load_context.add_loaded_labeled_asset(label, loaded_asset);
}
```

Example 3 (javascript):
```javascript
43    async fn load(
44        &self,
45        reader: &mut dyn Reader,
46        _settings: &(),
47        load_context: &mut LoadContext<'_>,
48    ) -> Result<Self::Asset, Self::Error> {
49        let compressed_path = load_context.path();
50        let file_name = compressed_path
51            .file_name()
52            .ok_or(GzAssetLoaderError::IndeterminateFilePath)?
53            .to_string_lossy();
54        let uncompressed_file_name = file_name
55            .strip_suffix(".gz")
56            .ok_or(GzAssetLoaderError::IndeterminateFilePath)?;
57        let contained_path = compres
...
```

Example 4 (javascript):
```javascript
141    async fn load(
142        &self,
143        reader: &mut dyn Reader,
144        _settings: &Self::Settings,
145        load_context: &mut LoadContext<'_>,
146    ) -> Result<CoolText, Self::Error> {
147        let mut bytes = Vec::new();
148        reader.read_to_end(&mut bytes).await?;
149        let ron: CoolTextRon = ron::de::from_bytes(&bytes)?;
150        let mut base_text = ron.text;
151        for embedded in ron.embedded_dependencies {
152            let loaded = load_context
153                .loader()
154                .immediate()
155                .load::<Text>(&embedded)
...
```

---

## Struct AssetLoaderError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.AssetLoaderError.html

**Contents:**
- Struct AssetLoaderError Copy item path
- Implementations§
  - impl AssetLoaderError
    - pub fn path(&self) -> &AssetPath<'static>
    - pub fn error(&self) -> &BevyError
- Trait Implementations§
  - impl Clone for AssetLoaderError
    - fn clone(&self) -> AssetLoaderError
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for AssetLoaderError

An error that can occur during asset loading.

The path of the asset that failed to load.

The error the loader reported when attempting to load the asset.

If you know the type of the error the asset loader returned, you can use BevyError::downcast_ref() to get it.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AssetLoaderError { /* private fields */ }
```

---

## Struct GizmoAsset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/struct.GizmoAsset.html

**Contents:**
- Struct GizmoAsset Copy item path
- Implementations§
  - impl GizmoAsset
    - pub fn new() -> GizmoAsset
      - Examples found in repository?
    - pub fn config_typeid(&self) -> TypeId
- Methods from Deref<Target = GizmoBuffer<ErasedGizmoConfigGroup, ()>>§
    - pub fn arc_2d( &mut self, isometry: impl Into<Isometry2d>, arc_angle: f32, radius: f32, color: impl Into<Color>, ) -> Arc2dBuilder<'_, Config, Clear>
      - §Arguments
      - §Example

A collection of gizmos.

Has the same gizmo drawing API as Gizmos.

Create a new GizmoAsset.

The type of the gizmo’s configuration group.

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

Draw a circle in 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct GizmoAsset { /* private fields */ }
```

Example 2 (javascript):
```javascript
23fn setup(
24    mut commands: Commands,
25    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
26    mut meshes: ResMut<Assets<Mesh>>,
27    mut materials: ResMut<Assets<StandardMaterial>>,
28) {
29    let mut gizmo = GizmoAsset::new();
30
31    // When drawing a lot of static lines a Gizmo component can have
32    // far better performance than the Gizmos system parameter,
33    // but the system parameter will perform better for smaller lines that update often.
34
35    // A sphere made out of 30_000 lines!
36    gizmo
37        .sphere(Isometry3d::IDENTITY, 0.5, CRIMSON)
38        .resolutio
...
```

Example 3 (unknown):
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

Example 4 (javascript):
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

---

## Struct AssetSource Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/io/struct.AssetSource.html

**Contents:**
- Struct AssetSource Copy item path
- Implementations§
  - impl AssetSource
    - pub fn build() -> AssetSourceBuilder
      - Examples found in repository?
    - pub fn id(&self) -> AssetSourceId<'static>
    - pub fn reader(&self) -> &(dyn ErasedAssetReader + 'static)
    - pub fn writer( &self, ) -> Result<&(dyn ErasedAssetWriter + 'static), MissingAssetWriterError>
    - pub fn processed_reader( &self, ) -> Result<&(dyn ErasedAssetReader + 'static), MissingProcessedAssetReaderError>
    - pub fn processed_writer( &self, ) -> Result<&(dyn ErasedAssetWriter + 'static), MissingProcessedAssetWriterError>

A collection of unprocessed and processed AssetReader, AssetWriter, and AssetWatcher instances for a specific asset source, identified by an AssetSourceId.

Starts building a new AssetSource.

Returns this source’s id.

Return’s this source’s unprocessed AssetReader.

Return’s this source’s unprocessed AssetWriter, if it exists.

Return’s this source’s processed AssetReader, if it exists.

Return’s this source’s processed AssetWriter, if it exists.

Return’s this source’s unprocessed event receiver, if the source is currently watching for changes.

Return’s this source’s processed event receiver, if the source is currently watching for changes.

Returns true if the assets in this source should be processed.

Returns a builder function for this platform’s default AssetReader. path is the relative path to the asset root.

Returns a builder function for this platform’s default AssetWriter. path is the relative path to the asset root. This will return None if this platform does not support writing assets by default.

Returns the default non-existent AssetWatcher warning for the current platform.

Returns a builder function for this platform’s default AssetWatcher. path is the relative path to the asset root. This will return None if this platform does not support watching assets by default. file_debounce_time is the amount of time to wait (and debounce duplicate events) before returning an event. Higher durations reduce duplicates but increase the amount of time before a change event is processed. If the duration is set too low, some systems might surface events before their filesystem has the changes.

This will cause processed AssetReader futures (such as AssetReader::read) to wait until the AssetProcessor has finished processing the requested asset.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AssetSource { /* private fields */ }
```

Example 2 (unknown):
```unknown
42    fn build(&self, app: &mut App) {
43        app.register_asset_source(
44            AssetSourceId::Default,
45            AssetSource::build().with_reader(|| {
46                Box::new(CustomAssetReader(
47                    // This is the default reader for the current platform
48                    AssetSource::get_default_reader("assets".to_string())(),
49                ))
50            }),
51        );
52    }
```

Example 3 (unknown):
```unknown
42    fn build(&self, app: &mut App) {
43        app.register_asset_source(
44            AssetSourceId::Default,
45            AssetSource::build().with_reader(|| {
46                Box::new(CustomAssetReader(
47                    // This is the default reader for the current platform
48                    AssetSource::get_default_reader("assets".to_string())(),
49                ))
50            }),
51        );
52    }
```

---

## Trait ErasedAssetLoader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/trait.ErasedAssetLoader.html

**Contents:**
- Trait ErasedAssetLoader Copy item path
- Required Methods§
    - fn load<'a>( &'a self, reader: &'a mut dyn Reader, meta: &'a (dyn AssetMetaDyn + 'static), load_context: LoadContext<'a>, ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<ErasedLoadedAsset, BevyError>> + 'a>>
    - fn extensions(&self) -> &[&str]
    - fn deserialize_meta( &self, meta: &[u8], ) -> Result<Box<dyn AssetMetaDyn>, DeserializeMetaError>
    - fn default_meta(&self) -> Box<dyn AssetMetaDyn>
    - fn type_name(&self) -> &'static str
    - fn type_id(&self) -> TypeId
    - fn asset_type_name(&self) -> &'static str
    - fn asset_type_id(&self) -> TypeId

Provides type-erased access to an AssetLoader.

Asynchronously loads the asset(s) from the bytes provided by Reader.

Returns a list of extensions supported by this asset loader, without the preceding dot.

Deserializes metadata from the input meta bytes into the appropriate type (erased as Box<dyn AssetMetaDyn>).

Returns the default meta value for the AssetLoader (erased as Box<dyn AssetMetaDyn>).

Returns the type name of the AssetLoader.

Returns the TypeId of the AssetLoader.

Returns the type name of the top-level Asset loaded by the AssetLoader.

Returns the TypeId of the top-level Asset loaded by the AssetLoader.

**Examples:**

Example 1 (unknown):
```unknown
pub trait ErasedAssetLoader:
    Send
    + Sync
    + 'static {
    // Required methods
    fn load<'a>(
        &'a self,
        reader: &'a mut dyn Reader,
        meta: &'a (dyn AssetMetaDyn + 'static),
        load_context: LoadContext<'a>,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<ErasedLoadedAsset, BevyError>> + 'a>>;
    fn extensions(&self) -> &[&str];
    fn deserialize_meta(
        &self,
        meta: &[u8],
    ) -> Result<Box<dyn AssetMetaDyn>, DeserializeMetaError>;
    fn default_meta(&self) -> Box<dyn AssetMetaDyn>;
    fn type_name(&self) -> &'static str;
 
...
```

---

## Module handle_or_path Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/feathers/handle_or_path/index.html

**Contents:**
- Module handle_or_path Copy item path
- Enums§

Provides a way to specify assets either by handle or by path.

---

## Struct StrongHandle Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.StrongHandle.html

**Contents:**
- Struct StrongHandle Copy item path
- Trait Implementations§
  - impl Debug for StrongHandle
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Drop for StrongHandle
    - fn drop(&mut self)
  - impl TypePath for StrongHandle
    - fn type_path() -> &'static str
    - fn short_type_path() -> &'static str
    - fn type_ident() -> Option<&'static str>

The internal “strong” Asset handle storage for Handle::Strong and UntypedHandle::Strong. When this is dropped, the Asset will be freed. It also stores some asset metadata for easy access from handles.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct StrongHandle { /* private fields */ }
```

---

## Struct AssetServer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.AssetServer.html

**Contents:**
- Struct AssetServer Copy item path
- Implementations§
  - impl AssetServer
    - pub fn new( sources: AssetSources, mode: AssetServerMode, watching_for_changes: bool, unapproved_path_mode: UnapprovedPathMode, ) -> AssetServer
    - pub fn new_with_meta_check( sources: AssetSources, mode: AssetServerMode, meta_check: AssetMetaCheck, watching_for_changes: bool, unapproved_path_mode: UnapprovedPathMode, ) -> AssetServer
    - pub fn get_source<'a>( &self, source: impl Into<AssetSourceId<'a>>, ) -> Result<&AssetSource, MissingAssetSourceError>
    - pub fn watching_for_changes(&self) -> bool
    - pub fn register_loader<L>(&self, loader: L)where L: AssetLoader,
    - pub fn register_asset<A>(&self, assets: &Assets<A>)where A: Asset,
    - pub async fn get_asset_loader_with_extension( &self, extension: &str, ) -> Result<Arc<dyn ErasedAssetLoader>, MissingAssetLoaderForExtensionError>

Loads and tracks the state of Asset values from a configured AssetReader. This can be used to kick off new asset loads and retrieve their current load states.

The general process to load an asset is:

AssetServer can be cloned. It is backed by an Arc so clones will share state. Clones can be freely used in parallel.

Create a new instance of AssetServer. If watch_for_changes is true, the AssetReader storage will watch for changes to asset sources and hot-reload them.

Create a new instance of AssetServer. If watch_for_changes is true, the AssetReader storage will watch for changes to asset sources and hot-reload them.

Retrieves the AssetSource for the given source.

Returns true if the AssetServer watches for changes.

Registers a new AssetLoader. AssetLoaders must be registered before they can be used.

Registers a new Asset type. Asset types must be registered before assets of that type can be loaded.

Returns the registered AssetLoader associated with the given extension, if it exists.

Returns the registered AssetLoader associated with the given core::any::type_name, if it exists.

Retrieves the default AssetLoader for the given path, if one can be found.

Retrieves the default AssetLoader for the given Asset TypeId, if one can be found.

Retrieves the default AssetLoader for the given Asset type, if one can be found.

Begins loading an Asset of type A stored at path. This will not block on the asset load. Instead, it returns a “strong” Handle. When the Asset is loaded (and enters LoadState::Loaded), it will be added to the associated Assets resource.

Note that if the asset at this path is already loaded, this function will return the existing handle, and will not waste work spawning a new load task.

In case the file path contains a hashtag (#), the path must be specified using Path or AssetPath because otherwise the hashtag would be interpreted as separator between the file path and the label. For example:

Furthermore, if you need to load a file with a hashtag in its name and a label, you can manually construct an AssetPath.

You can check the asset’s load state by reading AssetEvent events, calling AssetServer::load_state, or checking the Assets storage to see if the Asset exists yet.

The asset load will fail and an error will be printed to the logs if the asset stored at path is not of type A.

Same as load, but you can load assets from unapproved paths if AssetPlugin::unapproved_path_mode is Deny.

See UnapprovedPathMode and AssetPath::is_una

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct AssetServer { /* private fields */ }
```

Example 2 (unknown):
```unknown
// `#path` is a label.
asset_server.load("some/file#path");

// `#path` is part of the file name.
asset_server.load(Path::new("some/file#path"));
```

Example 3 (unknown):
```unknown
asset_server.load(AssetPath::from_path(Path::new("some/file#path")).with_label("subasset"));
```

Example 4 (unknown):
```unknown
116fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
117    commands.spawn(Sprite::from_image(asset_server.load("branding/icon.png")));
118}
```

---

## Type Alias TrackAssets Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/type.TrackAssets.html

**Contents:**
- Type Alias TrackAssets Copy item path
- Aliased Type§

Deprecated alias for AssetTrackingSystems.

**Examples:**

Example 1 (unknown):
```unknown
pub type TrackAssets = AssetTrackingSystems;
```

Example 2 (unknown):
```unknown
pub struct TrackAssets;
```

---

## Struct ReflectHandle Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.ReflectHandle.html

**Contents:**
- Struct ReflectHandle Copy item path
- Implementations§
  - impl ReflectHandle
    - pub fn asset_type_id(&self) -> TypeId
    - pub fn downcast_handle_untyped( &self, handle: &(dyn Any + 'static), ) -> Option<UntypedHandle>
    - pub fn typed(&self, handle: UntypedHandle) -> Box<dyn Reflect>
- Trait Implementations§
  - impl Clone for ReflectHandle
    - fn clone(&self) -> ReflectHandle
    - fn clone_from(&mut self, source: &Self)

Reflect type data struct relating a Handle<T> back to the T asset type.

Say you want to look up the asset values of a list of handles when you have access to their &dyn Reflect form. Assets can be looked up in the world using ReflectAsset, but how do you determine which ReflectAsset to use when only looking at the handle? ReflectHandle is stored in the type registry on each Handle<T> type, so you can use ReflectHandle::asset_type_id to look up the ReflectAsset type data on the corresponding T asset type:

The TypeId of the asset

A way to go from a Handle<T> in a dyn Any to a UntypedHandle

A way to go from a UntypedHandle to a Handle<T> in a Box<dyn Reflect>. Equivalent of UntypedHandle::typed.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ReflectHandle { /* private fields */ }
```

Example 2 (javascript):
```javascript
use bevy_asset::{ReflectHandle, ReflectAsset};

let handles: Vec<&dyn Reflect> = unimplemented!();
for handle in handles {
    let reflect_handle = type_registry.get_type_data::<ReflectHandle>(handle.type_id()).unwrap();
    let reflect_asset = type_registry.get_type_data::<ReflectAsset>(reflect_handle.asset_type_id()).unwrap();

    let handle = reflect_handle.downcast_handle_untyped(handle.as_any()).unwrap();
    let value = reflect_asset.get(world, &handle).unwrap();
    println!("{value:?}");
}
```

---

## Trait AssetContainer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/trait.AssetContainer.html

**Contents:**
- Trait AssetContainer Copy item path
- Required Methods§
    - fn insert(self: Box<Self>, id: UntypedAssetId, world: &mut World)
    - fn asset_type_name(&self) -> &'static str
- Implementations§
  - impl dyn AssetContainer
    - pub fn is<__T>(&self) -> boolwhere __T: AssetContainer,
    - pub fn downcast<__T>( self: Box<dyn AssetContainer>, ) -> Result<Box<__T>, Box<dyn AssetContainer>>where __T: AssetContainer,
    - pub fn downcast_rc<__T>( self: Rc<dyn AssetContainer>, ) -> Result<Rc<__T>, Rc<dyn AssetContainer>>where __T: AssetContainer,
    - pub fn downcast_ref<__T>(&self) -> Option<&__T>where __T: AssetContainer,

A type erased container for an Asset value that is capable of inserting the Asset into a World’s Assets collection.

Returns true if the trait object wraps an object of type __T.

Returns a boxed object from a boxed trait object if the underlying object is of type __T. Returns the original boxed trait if it isn’t.

Returns an Rc-ed object from an Rc-ed trait object if the underlying object is of type __T. Returns the original Rc-ed trait if it isn’t.

Returns a reference to the object within the trait object if it is of type __T, or None if it isn’t.

Returns a mutable reference to the object within the trait object if it is of type __T, or None if it isn’t.

**Examples:**

Example 1 (unknown):
```unknown
pub trait AssetContainer:
    Downcast
    + Any
    + Send
    + Sync
    + 'static {
    // Required methods
    fn insert(self: Box<Self>, id: UntypedAssetId, world: &mut World);
    fn asset_type_name(&self) -> &'static str;
}
```

---

## Struct LoadedUntypedAsset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.LoadedUntypedAsset.html

**Contents:**
- Struct LoadedUntypedAsset Copy item path
- Fields§
- Trait Implementations§
  - impl TypePath for LoadedUntypedAsset
    - fn type_path() -> &'static str
    - fn short_type_path() -> &'static str
    - fn type_ident() -> Option<&'static str>
    - fn crate_name() -> Option<&'static str>
    - fn module_path() -> Option<&'static str>
  - impl VisitAssetDependencies for LoadedUntypedAsset

A “loaded asset” containing the untyped handle for an asset stored in a given AssetPath.

The handle to the loaded asset.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct LoadedUntypedAsset {
    pub handle: UntypedHandle,
}
```

---

## Struct AssetMeta Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/meta/struct.AssetMeta.html

**Contents:**
- Struct AssetMeta Copy item path
- Fields§
- Implementations§
  - impl<L, P> AssetMeta<L, P>where L: AssetLoader, P: Process,
    - pub fn new( asset: AssetAction<<L as AssetLoader>::Settings, <P as Process>::Settings>, ) -> AssetMeta<L, P>
    - pub fn deserialize( bytes: &[u8], ) -> Result<AssetMeta<L, P>, DeserializeMetaError>
- Trait Implementations§
  - impl<L, P> AssetMetaDyn for AssetMeta<L, P>where L: AssetLoader, P: Process,
    - fn loader_settings(&self) -> Option<&(dyn Settings + 'static)>
    - fn loader_settings_mut(&mut self) -> Option<&mut (dyn Settings + 'static)>

Asset metadata that informs how an Asset should be handled by the asset system.

L is the AssetLoader (if one is configured) for the AssetAction. This can be () if it is not required. P is the Process processor, if one is configured for the AssetAction. This can be () if it is not required.

The version of the meta format being used. This will change whenever a breaking change is made to the meta format.

Information produced by the AssetProcessor after processing this asset. This will only exist alongside processed versions of assets. You should not manually set it in your asset source files.

How to handle this asset in the asset system. See AssetAction.

Deserializes the given serialized byte representation of the asset meta.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AssetMeta<L, P>where
    L: AssetLoader,
    P: Process,{
    pub meta_format_version: String,
    pub processed_info: Option<ProcessedInfo>,
    pub asset: AssetAction<<L as AssetLoader>::Settings, <P as Process>::Settings>,
}
```

---

## Struct FontLoader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/text/struct.FontLoader.html

**Contents:**
- Struct FontLoader Copy item path
- Trait Implementations§
  - impl AssetLoader for FontLoader
    - type Asset = Font
    - type Settings = ()
    - type Error = FontLoaderError
    - async fn load( &self, reader: &mut dyn Reader, _settings: &(), _load_context: &mut LoadContext<'_>, ) -> Result<Font, <FontLoader as AssetLoader>::Error>
    - fn extensions(&self) -> &[&str]
  - impl Default for FontLoader
    - fn default() -> FontLoader

An AssetLoader for Fonts, for use by the AssetServer

Processes the asset in an asynchronous closure.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct FontLoader;
```

---

## Struct ErasedLoadedAsset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.ErasedLoadedAsset.html

**Contents:**
- Struct ErasedLoadedAsset Copy item path
- Implementations§
  - impl ErasedLoadedAsset
    - pub fn take<A>(self) -> Option<A>where A: Asset,
      - Examples found in repository?
    - pub fn get<A>(&self) -> Option<&A>where A: Asset,
    - pub fn asset_type_id(&self) -> TypeId
    - pub fn asset_type_name(&self) -> &'static str
    - pub fn get_labeled( &self, label: impl Into<CowArc<'static, str>>, ) -> Option<&ErasedLoadedAsset>
    - pub fn iter_labels(&self) -> impl Iterator<Item = &str>

A “type erased / boxed” counterpart to LoadedAsset. This is used in places where the loaded type is not statically known.

Cast (and take ownership) of the Asset value of the given type. This will return Some if the stored type matches A and None if it does not.

Retrieves a reference to the internal Asset type, if it matches the type A. Otherwise returns None.

Retrieves the TypeId of the stored Asset type.

Retrieves the type_name of the stored Asset type.

Returns the ErasedLoadedAsset for the given label, if it exists.

Iterate over all labels for “labeled assets” in the loaded asset

Cast this loaded asset as the given type. If the type does not match, the original type-erased asset is returned.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ErasedLoadedAsset { /* private fields */ }
```

Example 2 (javascript):
```javascript
115fn decompress<T: Component + From<Handle<A>>, A: Asset>(
116    mut commands: Commands,
117    asset_server: Res<AssetServer>,
118    mut compressed_assets: ResMut<Assets<GzAsset>>,
119    query: Query<(Entity, &Compressed<A>)>,
120) {
121    for (entity, Compressed { compressed, .. }) in query.iter() {
122        let Some(GzAsset { uncompressed }) = compressed_assets.remove(compressed) else {
123            continue;
124        };
125
126        let uncompressed = uncompressed.take::<A>().unwrap();
127
128        commands
129            .entity(entity)
130            .remove::<Compressed<A
...
```

---

## Macro embedded_path Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/macro.embedded_path.html

**Contents:**
- Macro embedded_path Copy item path

Returns the Path for a given embedded asset. This is used internally by embedded_asset and can be used to get a Path that matches the AssetPath used by that asset.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! embedded_path {
    ($path_str: expr) => { ... };
    ($source_path: expr, $path_str: expr) => { ... };
}
```

---

## Trait AsyncReadExt Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/trait.AsyncReadExt.html

**Contents:**
- Trait AsyncReadExt Copy item path
- Provided Methods§
    - fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> ReadFuture<'a, Self> ⓘwhere Self: Unpin,
      - §Examples
    - fn read_vectored<'a>( &'a mut self, bufs: &'a mut [IoSliceMut<'a>], ) -> ReadVectoredFuture<'a, Self> ⓘwhere Self: Unpin,
    - fn read_to_end<'a>( &'a mut self, buf: &'a mut Vec<u8>, ) -> ReadToEndFuture<'a, Self> ⓘwhere Self: Unpin,
      - §Examples
    - fn read_to_string<'a>( &'a mut self, buf: &'a mut String, ) -> ReadToStringFuture<'a, Self> ⓘwhere Self: Unpin,
      - §Examples
    - fn read_exact<'a>(&'a mut self, buf: &'a mut [u8]) -> ReadExactFuture<'a, Self> ⓘwhere Self: Unpin,

Extension trait for AsyncRead.

Reads some bytes from the byte stream.

On success, returns the total number of bytes read.

If the return value is Ok(n), then it must be guaranteed that 0 <= n <= buf.len(). A nonzero n value indicates that the buffer has been filled with n bytes of data. If n is 0, then it can indicate one of two scenarios:

Like read(), except it reads into a slice of buffers.

Data is copied to fill each buffer in order, with the final buffer possibly being only partially filled. This method must behave same as a single call to read() with the buffers concatenated would.

Reads the entire contents and appends them to a Vec.

On success, returns the total number of bytes read.

Reads the entire contents and appends them to a String.

On success, returns the total number of bytes read.

Reads the exact number of bytes required to fill buf.

Creates an adapter which will read at most limit bytes from it.

This method returns a new instance of AsyncRead which will read at most limit bytes, after which it will always return Ok(0) indicating EOF.

Converts this AsyncRead into a Stream of bytes.

The returned type implements Stream where Item is io::Result<u8>.

Creates an adapter which will chain this stream with another.

The returned AsyncRead instance will first read all bytes from this reader until EOF is found, and then continue with next.

Boxes the reader and changes its type to dyn AsyncRead + Send + 'a.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait AsyncReadExt: AsyncRead {
    // Provided methods
    fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> ReadFuture<'a, Self> ⓘ
       where Self: Unpin { ... }
    fn read_vectored<'a>(
        &'a mut self,
        bufs: &'a mut [IoSliceMut<'a>],
    ) -> ReadVectoredFuture<'a, Self> ⓘ
       where Self: Unpin { ... }
    fn read_to_end<'a>(
        &'a mut self,
        buf: &'a mut Vec<u8>,
    ) -> ReadToEndFuture<'a, Self> ⓘ
       where Self: Unpin { ... }
    fn read_to_string<'a>(
        &'a mut self,
        buf: &'a mut String,
    ) -> ReadToStringFuture<'a, Self> ⓘ
       
...
```

Example 2 (javascript):
```javascript
use futures_lite::io::{AsyncReadExt, BufReader};

let input: &[u8] = b"hello";
let mut reader = BufReader::new(input);

let mut buf = vec![0; 1024];
let n = reader.read(&mut buf).await?;
```

Example 3 (javascript):
```javascript
use futures_lite::io::{AsyncReadExt, Cursor};

let mut reader = Cursor::new(vec![1, 2, 3]);
let mut contents = Vec::new();

let n = reader.read_to_end(&mut contents).await?;
assert_eq!(n, 3);
assert_eq!(contents, [1, 2, 3]);
```

Example 4 (javascript):
```javascript
use futures_lite::io::{AsyncReadExt, Cursor};

let mut reader = Cursor::new(&b"hello");
let mut contents = String::new();

let n = reader.read_to_string(&mut contents).await?;
assert_eq!(n, 5);
assert_eq!(contents, "hello");
```

---

## Enum SceneLoaderError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/scene/enum.SceneLoaderError.html

**Contents:**
- Enum SceneLoaderError Copy item path
- Variants (Non-exhaustive)§
  - Io(Error)
  - RonSpannedError(SpannedError)
- Trait Implementations§
  - impl Debug for SceneLoaderError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for SceneLoaderError
    - fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>
  - impl Error for SceneLoaderError

Possible errors that can be produced by SceneLoader

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[non_exhaustive]pub enum SceneLoaderError {
    Io(Error),
    RonSpannedError(SpannedError),
}
```

---

## Struct Deferred Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.Deferred.html

**Contents:**
- Struct Deferred Copy item path
- Auto Trait Implementations§
  - impl Freeze for Deferred
  - impl RefUnwindSafe for Deferred
  - impl Send for Deferred
  - impl Sync for Deferred
  - impl Unpin for Deferred
  - impl UnwindSafe for Deferred
- Blanket Implementations§
  - impl<T> Any for Twhere T: 'static + ?Sized,

NestedLoader will create and return asset handles immediately, but only actually load the asset later.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Deferred(/* private fields */);
```

---

## Enum LoadDirectError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.LoadDirectError.html

**Contents:**
- Enum LoadDirectError Copy item path
- Variants§
  - RequestedSubasset(AssetPath<'static>)
  - LoadError
    - Fields
- Trait Implementations§
  - impl Debug for LoadDirectError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for LoadDirectError
    - fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>

An error that occurs when attempting to call NestedLoader::load which is configured to work immediately.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum LoadDirectError {
    RequestedSubasset(AssetPath<'static>),
    LoadError {
        dependency: AssetPath<'static>,
        error: AssetLoadError,
    },
}
```

---

## Trait AsyncWriteExt Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/trait.AsyncWriteExt.html

**Contents:**
- Trait AsyncWriteExt Copy item path
- Provided Methods§
    - fn write<'a>(&'a mut self, buf: &'a [u8]) -> WriteFuture<'a, Self> ⓘwhere Self: Unpin,
      - §Examples
    - fn write_vectored<'a>( &'a mut self, bufs: &'a [IoSlice<'a>], ) -> WriteVectoredFuture<'a, Self> ⓘwhere Self: Unpin,
    - fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> WriteAllFuture<'a, Self> ⓘwhere Self: Unpin,
      - §Examples
      - Examples found in repository?
    - fn flush(&mut self) -> FlushFuture<'_, Self> ⓘwhere Self: Unpin,
      - §Examples

Extension trait for AsyncWrite.

Writes some bytes into the byte stream.

Returns the number of bytes written from the start of the buffer.

If the return value is Ok(n) then it must be guaranteed that 0 <= n <= buf.len(). A return value of 0 typically means that the underlying object is no longer able to accept bytes and will likely not be able to in the future as well, or that the provided buffer is empty.

Like write(), except that it writes a slice of buffers.

Data is copied from each buffer in order, with the final buffer possibly being only partially consumed. This method must behave same as a call to write() with the buffers concatenated would.

Writes an entire buffer into the byte stream.

This method will keep calling write() until there is no more data to be written or an error occurs. It will not return before the entire buffer is successfully written or an error occurs.

Flushes the stream to ensure that all buffered contents reach their destination.

Boxes the writer and changes its type to dyn AsyncWrite + Send + 'a.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait AsyncWriteExt: AsyncWrite {
    // Provided methods
    fn write<'a>(&'a mut self, buf: &'a [u8]) -> WriteFuture<'a, Self> ⓘ
       where Self: Unpin { ... }
    fn write_vectored<'a>(
        &'a mut self,
        bufs: &'a [IoSlice<'a>],
    ) -> WriteVectoredFuture<'a, Self> ⓘ
       where Self: Unpin { ... }
    fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> WriteAllFuture<'a, Self> ⓘ
       where Self: Unpin { ... }
    fn flush(&mut self) -> FlushFuture<'_, Self> ⓘ
       where Self: Unpin { ... }
    fn close(&mut self) -> CloseFuture<'_, Self> ⓘ
       where Self: Unpin { .
...
```

Example 2 (javascript):
```javascript
use futures_lite::io::{AsyncWriteExt, BufWriter};

let mut output = Vec::new();
let mut writer = BufWriter::new(&mut output);

let n = writer.write(b"hello").await?;
```

Example 3 (javascript):
```javascript
use futures_lite::io::{AsyncWriteExt, BufWriter};

let mut output = Vec::new();
let mut writer = BufWriter::new(&mut output);

let n = writer.write_all(b"hello").await?;
```

Example 4 (unknown):
```unknown
217    async fn save(
218        &self,
219        writer: &mut Writer,
220        asset: SavedAsset<'_, Self::Asset>,
221        _settings: &Self::Settings,
222    ) -> Result<TextSettings, Self::Error> {
223        writer.write_all(asset.text.as_bytes()).await?;
224        Ok(TextSettings::default())
225    }
```

---

## Enum AssetMetaCheck Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.AssetMetaCheck.html

**Contents:**
- Enum AssetMetaCheck Copy item path
- Variants§
  - Always
  - Paths(HashSet<AssetPath<'static>>)
  - Never
- Trait Implementations§
  - impl Clone for AssetMetaCheck
    - fn clone(&self) -> AssetMetaCheck
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for AssetMetaCheck

Configures how / if meta files will be checked. If an asset’s meta file is not checked, the default meta for the asset will be used.

Always check if assets have meta files. If the meta does not exist, the default meta will be used.

Only look up meta files for the provided paths. The default meta will be used for any paths not contained in this set.

Never check if assets have meta files and always use the default meta. If meta files exist, they will be ignored and the default meta will be used.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum AssetMetaCheck {
    Always,
    Paths(HashSet<AssetPath<'static>>),
    Never,
}
```

---

## Enum AssetMode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.AssetMode.html

**Contents:**
- Enum AssetMode Copy item path
- Variants§
  - Unprocessed
  - Processed
- Trait Implementations§
  - impl Debug for AssetMode
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
- Auto Trait Implementations§
  - impl Freeze for AssetMode
  - impl RefUnwindSafe for AssetMode

Controls whether or not assets are pre-processed before being loaded.

This setting is controlled by setting AssetPlugin::mode.

When building on web, asset preprocessing can cause problems due to the lack of filesystem access. See bevy#10157 for context.

Loads assets from their AssetSource’s default AssetReader without any “preprocessing”.

Assets will be “pre-processed”. This enables assets to be imported / converted / optimized ahead of time.

Assets will be read from their unprocessed AssetSource (defaults to the assets folder), processed according to their AssetMeta, and written to their processed AssetSource (defaults to the imported_assets/Default folder).

By default, this assumes the processor has already been run. It will load assets from their final processed AssetReader.

When developing an app, you should enable the asset_processor cargo feature, which will run the asset processor at startup. This should generally be used in combination with the file_watcher cargo feature, which enables hot-reloading of assets that have changed. When both features are enabled, changes to “original/source assets” will be detected, the asset will be re-processed, and then the final processed asset will be hot-reloaded in the app.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum AssetMode {
    Unprocessed,
    Processed,
}
```

---

## Struct NestedLoader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.NestedLoader.html

**Contents:**
- Struct NestedLoader Copy item path
- §Loader state
  - §Typing
  - §Load mode
- §Load kickoff
- §Lifetimes
- Implementations§
  - impl<'ctx, 'builder, T, M> NestedLoader<'ctx, 'builder, T, M>where T: Typing, M: Mode,
    - pub fn with_settings<S>( self, settings: impl Fn(&mut S) + Send + Sync + 'static, ) -> NestedLoader<'ctx, 'builder, T, M>where S: Settings,
      - Examples found in repository?

A builder for loading nested assets inside a LoadContext.

The type parameters T and M determine how this will load assets:

T: the typing of this loader. How do we know what type of asset to load?

See StaticTyped (the default), DynamicTyped, and UnknownTyped.

M: the load mode. Do we want to load this asset right now (in which case you will have to await the operation), or do we just want a Handle, and leave the actual asset loading to later?

See Deferred (the default) and Immediate.

When configuring this builder, you can freely switch between these modes via functions like deferred and immediate.

To inform the loader of what type of asset to load:

in StaticTyped: statically providing a type parameter A: Asset to load.

This is the simplest way to get a Handle<A> to the loaded asset, as long as you know the type of A at compile time.

in DynamicTyped: providing the TypeId of the asset at runtime.

If you know the type ID of the asset at runtime, but not at compile time, use with_dynamic_type followed by load to start loading an asset of that type. This lets you get an UntypedHandle (via Deferred), or a ErasedLoadedAsset (via Immediate).

in UnknownTyped: loading either a type-erased version of the asset (ErasedLoadedAsset), or a handle to a handle of the actual asset (LoadedUntypedAsset).

If you have no idea what type of asset you will be loading (not even at runtime with a TypeId), use this.

To inform the loader how you want to load the asset:

in Deferred: when you request to load the asset, you get a Handle for it, but the actual loading won’t be completed until later.

Use this if you only need a Handle or UntypedHandle.

in Immediate: the load request will load the asset right then and there, waiting until the asset is fully loaded and giving you access to it.

Note that this requires you to await a future, so you must be in an async context to use direct loading. In an asset loader, you will be in an async context.

Use this if you need the value of another asset in order to load the current asset. For example, if you are deriving a new asset from the referenced asset, or you are building a collection of assets. This will add the path of the asset as a “load dependency”.

If the current loader is used in a Process “asset preprocessor”, such as a LoadTransformAndSave preprocessor, changing a “load dependency” will result in re-processing of the asset.

If the current context is a normal AssetServer::load, an actual asset load will be kicked of

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct NestedLoader<'ctx, 'builder, T, M> { /* private fields */ }
```

Example 2 (javascript):
```javascript
141    async fn load(
142        &self,
143        reader: &mut dyn Reader,
144        _settings: &Self::Settings,
145        load_context: &mut LoadContext<'_>,
146    ) -> Result<CoolText, Self::Error> {
147        let mut bytes = Vec::new();
148        reader.read_to_end(&mut bytes).await?;
149        let ron: CoolTextRon = ron::de::from_bytes(&bytes)?;
150        let mut base_text = ron.text;
151        for embedded in ron.embedded_dependencies {
152            let loaded = load_context
153                .loader()
154                .immediate()
155                .load::<Text>(&embedded)
...
```

Example 3 (javascript):
```javascript
43    async fn load(
44        &self,
45        reader: &mut dyn Reader,
46        _settings: &(),
47        load_context: &mut LoadContext<'_>,
48    ) -> Result<Self::Asset, Self::Error> {
49        let compressed_path = load_context.path();
50        let file_name = compressed_path
51            .file_name()
52            .ok_or(GzAssetLoaderError::IndeterminateFilePath)?
53            .to_string_lossy();
54        let uncompressed_file_name = file_name
55            .strip_suffix(".gz")
56            .ok_or(GzAssetLoaderError::IndeterminateFilePath)?;
57        let contained_path = compres
...
```

Example 4 (javascript):
```javascript
141    async fn load(
142        &self,
143        reader: &mut dyn Reader,
144        _settings: &Self::Settings,
145        load_context: &mut LoadContext<'_>,
146    ) -> Result<CoolText, Self::Error> {
147        let mut bytes = Vec::new();
148        reader.read_to_end(&mut bytes).await?;
149        let ron: CoolTextRon = ron::de::from_bytes(&bytes)?;
150        let mut base_text = ron.text;
151        for embedded in ron.embedded_dependencies {
152            let loaded = load_context
153                .loader()
154                .immediate()
155                .load::<Text>(&embedded)
...
```

---

## Module io Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/io/index.html

**Contents:**
- Module io Copy item path
- Modules§
- Structs§
- Enums§
- Constants§
- Traits§
- Type Aliases§

bevy::assetModule io Copy item pathSource Modules§embeddedfilememoryprocessor_gatedwebStructs§AssetSourceA collection of unprocessed and processed AssetReader, AssetWriter, and AssetWatcher instances for a specific asset source, identified by an AssetSourceId.AssetSourceBuilderMetadata about an “asset source”, such as how to construct the AssetReader and AssetWriter for the source, and whether or not the source is processed.AssetSourceBuildersA Resource that hold (repeatable) functions capable of producing new AssetReader and AssetWriter instances for a given asset source.AssetSourcesA collection of AssetSources.MissingAssetSourceErrorAn error returned when an AssetSource does not exist for a given id.MissingAssetWriterErrorAn error returned when an AssetWriter does not exist for a given id.MissingProcessedAssetReaderErrorAn error returned when a processed AssetReader does not exist for a given id.MissingProcessedAssetWriterErrorAn error returned when a processed AssetWriter does not exist for a given id.SeekForwardFutureSliceReaderAn AsyncRead implementation capable of reading a [&[u8]].StackFutureA wrapper that stores a future in space allocated by the containerVecReaderAn AsyncRead implementation capable of reading a Vec<u8>.Enums§AssetReaderErrorErrors that occur while loading assets.AssetSourceEventAn “asset source change event” that occurs whenever asset (or asset metadata) is created/added/removedAssetSourceIdA reference to an “asset source”, which maps to an AssetReader and/or AssetWriter.AssetWriterErrorErrors that occur while loading assets.Constants§STACK_FUTURE_SIZEThe maximum size of a future returned from Reader::read_to_end. This is large enough to fit ten references.Traits§AssetReaderPerforms read operations on an asset storage. AssetReader exposes a “virtual filesystem” API, where asset bytes and asset metadata bytes are both stored and accessible for a given path. This trait is not object safe, if needed use a dyn ErasedAssetReader instead.AssetReaderFutureA future that returns a value or an AssetReaderErrorAssetWatcherA handle to an “asset watcher” process, that will listen for and emit AssetSourceEvent values for as long as AssetWatcher has not been dropped.AssetWriterPreforms write operations on an asset storage. AssetWriter exposes a “virtual filesystem” API, where asset bytes and asset metadata bytes are both stored and accessible for a given path. This trait is not object safe, if needed use a dyn ErasedAssetWriter instead.AsyncSeek

*[Content truncated]*

---

## Trait AssetWriter Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/io/trait.AssetWriter.html

**Contents:**
- Trait AssetWriter Copy item path
- Required Methods§
    - fn write<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture
    - fn write_meta<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture
    - fn remove<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture
    - fn remove_meta<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture
    - fn rename<'a>( &'a self, old_path: &'a Path, new_path: &'a Path, ) -> impl ConditionalSendFuture
    - fn rename_meta<'a>( &'a self, old_path: &'a Path, new_path: &'a Path, ) -> impl ConditionalSendFuture
    - fn create_directory<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture
    - fn remove_directory<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture

Preforms write operations on an asset storage. AssetWriter exposes a “virtual filesystem” API, where asset bytes and asset metadata bytes are both stored and accessible for a given path. This trait is not object safe, if needed use a dyn ErasedAssetWriter instead.

This trait defines asset-agnostic mechanisms to write bytes to a storage system. For the per-asset-type saving/loading logic, see AssetSaver and AssetLoader.

For a complementary version of this trait that can read assets from storage, see AssetReader.

Writes the full asset bytes at the provided path.

Writes the full asset meta bytes at the provided path. This should not include storage specific extensions like .meta.

Removes the asset stored at the given path.

Removes the asset meta stored at the given path. This should not include storage specific extensions like .meta.

Renames the asset at old_path to new_path

Renames the asset meta for the asset at old_path to new_path. This should not include storage specific extensions like .meta.

Creates a directory at the given path, including all parent directories if they do not already exist.

Removes the directory at the given path, including all assets and directories in that directory.

Removes the directory at the given path, but only if it is completely empty. This will return an error if the directory is not empty.

Removes all assets (and directories) in this directory, resulting in an empty directory.

Writes the asset bytes to the given path.

Writes the asset meta bytes to the given path.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait AssetWriter:
    Send
    + Sync
    + 'static {
    // Required methods
    fn write<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture;
    fn write_meta<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture;
    fn remove<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture;
    fn remove_meta<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture;
    fn rename<'a>(
        &'a self,
        old_path: &'a Path,
        new_path: &'a Path,
    ) -> impl ConditionalSendFuture;
    fn rename_meta<'a>(
        &'a self,
        old_path: &'a Path,
        n
...
```

---

## Enum FontLoaderError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/text/enum.FontLoaderError.html

**Contents:**
- Enum FontLoaderError Copy item path
- Variants (Non-exhaustive)§
  - Content(FaceParsingError)
  - Io(Error)
- Trait Implementations§
  - impl Debug for FontLoaderError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for FontLoaderError
    - fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>
  - impl Error for FontLoaderError

Possible errors that can be produced by FontLoader

The contents that could not be parsed

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[non_exhaustive]pub enum FontLoaderError {
    Content(FaceParsingError),
    Io(Error),
}
```

---

## Struct AssetIndex Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.AssetIndex.html

**Contents:**
- Struct AssetIndex Copy item path
- Implementations§
  - impl AssetIndex
    - pub fn to_bits(self) -> u64
    - pub fn from_bits(bits: u64) -> AssetIndex
- Trait Implementations§
  - impl Clone for AssetIndex
    - fn clone(&self) -> AssetIndex
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for AssetIndex

A generational runtime-only identifier for a specific Asset stored in Assets. This is optimized for efficient runtime usage and is not suitable for identifying assets across app runs.

Convert the AssetIndex into an opaque blob of bits to transport it in circumstances where carrying a strongly typed index isn’t possible.

The result of this function should not be relied upon for anything except putting it back into AssetIndex::from_bits to recover the index.

Convert an opaque u64 acquired from AssetIndex::to_bits back into an AssetIndex. This should not be used with any inputs other than those derived from AssetIndex::to_bits, as there are no guarantees for what will happen with such inputs.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AssetIndex { /* private fields */ }
```

---

## Enum DeserializeMetaError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.DeserializeMetaError.html

**Contents:**
- Enum DeserializeMetaError Copy item path
- Variants§
  - DeserializeSettings(SpannedError)
  - DeserializeMinimal(SpannedError)
- Trait Implementations§
  - impl Clone for DeserializeMetaError
    - fn clone(&self) -> DeserializeMetaError
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for DeserializeMetaError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

An error that occurs while deserializing AssetMeta.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum DeserializeMetaError {
    DeserializeSettings(SpannedError),
    DeserializeMinimal(SpannedError),
}
```

---

## Struct ReflectAsset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.ReflectAsset.html

**Contents:**
- Struct ReflectAsset Copy item path
- Implementations§
  - impl ReflectAsset
    - pub fn handle_type_id(&self) -> TypeId
    - pub fn assets_resource_type_id(&self) -> TypeId
    - pub fn get<'w>( &self, world: &'w World, asset_id: impl Into<UntypedAssetId>, ) -> Option<&'w (dyn Reflect + 'static)>
    - pub fn get_mut<'w>( &self, world: &'w mut World, asset_id: impl Into<UntypedAssetId>, ) -> Option<&'w mut (dyn Reflect + 'static)>
    - pub unsafe fn get_unchecked_mut<'w>( &self, world: UnsafeWorldCell<'w>, asset_id: impl Into<UntypedAssetId>, ) -> Option<&'w mut (dyn Reflect + 'static)>
      - §Safety
    - pub fn add( &self, world: &mut World, value: &(dyn PartialReflect + 'static), ) -> UntypedHandle

Type data for the TypeRegistry used to operate on reflected Assets.

This type provides similar methods to Assets<T> like get, add and remove, but can be used in situations where you don’t know which asset type T you want until runtime.

ReflectAsset can be obtained via TypeRegistration::data if the asset was registered using register_asset_reflect.

The TypeId of the Handle<T> for this asset

The TypeId of the Assets<T> resource

Equivalent of Assets::get

Equivalent of Assets::get_mut

Equivalent of Assets::get_mut, but works with an UnsafeWorldCell.

Only use this method when you have ensured that you are the only one with access to the Assets resource of the asset type. Furthermore, this does not allow you to have look up two distinct handles, you can only have at most one alive at the same time. This means that this is not allowed:

This method does not prevent you from having two mutable pointers to the same data, violating Rust’s aliasing rules. To avoid this:

Equivalent of Assets::add

Equivalent of Assets::insert

Equivalent of Assets::remove

Equivalent of Assets::len

Equivalent of Assets::is_empty

Equivalent of Assets::ids

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ReflectAsset { /* private fields */ }
```

Example 2 (javascript):
```javascript
let unsafe_world_cell = world.as_unsafe_world_cell();
let a = unsafe { reflect_asset.get_unchecked_mut(unsafe_world_cell, &handle_1).unwrap() };
let b = unsafe { reflect_asset.get_unchecked_mut(unsafe_world_cell, &handle_2).unwrap() };
// ^ not allowed, two mutable references through the same asset resource, even though the
// handles are distinct

println!("a = {a:?}, b = {b:?}");
```

---

## Enum AssetServerMode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.AssetServerMode.html

**Contents:**
- Enum AssetServerMode Copy item path
- Variants§
  - Unprocessed
  - Processed
- Trait Implementations§
  - impl Clone for AssetServerMode
    - fn clone(&self) -> AssetServerMode
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for AssetServerMode
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

The “asset mode” the server is currently in.

This server loads unprocessed assets.

This server loads processed assets.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum AssetServerMode {
    Unprocessed,
    Processed,
}
```

---

## Enum ImageLoaderError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/enum.ImageLoaderError.html

**Contents:**
- Enum ImageLoaderError Copy item path
- Variants (Non-exhaustive)§
  - Io(Error)
  - FileTexture(FileTextureError)
- Trait Implementations§
  - impl Debug for ImageLoaderError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for ImageLoaderError
    - fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>
  - impl Error for ImageLoaderError

An error when loading an image using ImageLoader.

An error occurred while trying to load the image bytes.

An error occurred while trying to decode the image bytes.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[non_exhaustive]pub enum ImageLoaderError {
    Io(Error),
    FileTexture(FileTextureError),
}
```

---

## Crate asset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/index.html

**Contents:**
- Crate asset Copy item path
  - §Loading assets
- §Modifying entities that use assets
- §Hot reloading assets
- §Procedural asset creation
  - §Handles and reference counting
- §Asset dependencies
- §Custom asset types
- Modules§
- Macros§

In the context of game development, an “asset” is a piece of content that is loaded from disk and displayed in the game. Typically, these are authored by artists and designers (in contrast to code), are relatively large in size, and include everything from textures and models to sounds and music to levels and scripts.

This presents two main challenges:

These problems play into each other, for if assets are expensive to store in memory, then larger game worlds will need to load them from disk as needed, ideally without a loading screen.

As is common in Rust, non-blocking asset loading is done using async, with background tasks used to load assets while the game is running. Bevy coordinates these tasks using the AssetServer resource, storing each loaded asset in a strongly-typed Assets<T> collection (also a resource). Handles serve as an id-based reference to entries in the Assets collection, allowing them to be cheaply shared between systems, and providing a way to initialize objects (generally entities) before the required assets are loaded. In short: Handles are not the assets themselves, they just tell how to look them up!

The AssetServer is the main entry point for loading assets. Typically, you’ll use the AssetServer::load method to load an asset from disk, which returns a Handle. Note that this method does not attempt to reload the asset if it has already been loaded: as long as at least one handle has not been dropped, calling AssetServer::load on the same path will return the same handle. The handle that’s returned can be used to instantiate various Components that require asset data to function, which will then be spawned into the world as part of an entity.

To avoid assets “popping” into existence, you may want to check that all of the required assets are loaded before transitioning to a new scene. This can be done by checking the LoadState of the asset handle using AssetServer::is_loaded_with_dependencies, which will be true when the asset is ready to use.

Keep track of what you’re waiting on by using a HashSet of asset handles or similar data structure, which iterate over and poll in your update loop, and transition to the new scene once all assets are loaded. Bevy’s built-in states system can be very helpful for this!

If we later want to change the asset data a given component uses (such as changing an entity’s material), we have three options:

The first option is the most common: just query for the component that holds the handle, and 

*[Content truncated]*

---

## Enum InvalidGenerationError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.InvalidGenerationError.html

**Contents:**
- Enum InvalidGenerationError Copy item path
- Variants§
  - Occupied
    - Fields
  - Removed
    - Fields
- Trait Implementations§
  - impl Debug for InvalidGenerationError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for InvalidGenerationError

An error returned when an AssetIndex has an invalid generation.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum InvalidGenerationError {
    Occupied {
        index: AssetIndex,
        current_generation: u32,
    },
    Removed {
        index: AssetIndex,
    },
}
```

---

## Struct DynamicTyped Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.DynamicTyped.html

**Contents:**
- Struct DynamicTyped Copy item path
- Auto Trait Implementations§
  - impl Freeze for DynamicTyped
  - impl RefUnwindSafe for DynamicTyped
  - impl Send for DynamicTyped
  - impl Sync for DynamicTyped
  - impl Unpin for DynamicTyped
  - impl UnwindSafe for DynamicTyped
- Blanket Implementations§
  - impl<T> Any for Twhere T: 'static + ?Sized,

NestedLoader has been configured with info on what type of asset to load at runtime.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DynamicTyped { /* private fields */ }
```

---

## Struct ResetIndirectBatchSetsPipeline Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ResetIndirectBatchSetsPipeline.html

**Contents:**
- Struct ResetIndirectBatchSetsPipeline Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for ResetIndirectBatchSetsPipeline
    - fn clone(&self) -> ResetIndirectBatchSetsPipeline
    - fn clone_from(&mut self, source: &Self)
  - impl SpecializedComputePipeline for ResetIndirectBatchSetsPipeline
    - type Key = ()
    - fn specialize( &self, _: <ResetIndirectBatchSetsPipeline as SpecializedComputePipeline>::Key, ) -> ComputePipelineDescriptor
- Auto Trait Implementations§

The pipeline for the batch set count reset shader.

This shader resets the indirect batch set count to 0 for each view. It runs in between every phase (early, late, and main).

The bind group layout for the compute shader.

The shader asset handle.

The pipeline ID for the compute shader.

This gets filled in prepare_preprocess_pipelines.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ResetIndirectBatchSetsPipeline {
    pub bind_group_layout: BindGroupLayout,
    pub shader: Handle<Shader>,
    pub pipeline_id: Option<CachedComputePipelineId>,
}
```

---

## Enum WaitForAssetError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.WaitForAssetError.html

**Contents:**
- Enum WaitForAssetError Copy item path
- Variants§
  - NotLoaded
  - Failed(Arc<AssetLoadError>)
  - DependencyFailed(Arc<AssetLoadError>)
- Trait Implementations§
  - impl Clone for WaitForAssetError
    - fn clone(&self) -> WaitForAssetError
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for WaitForAssetError

An error when attempting to wait asynchronously for an Asset to load.

The asset is not being loaded; waiting for it is meaningless.

The asset failed to load.

A dependency of the asset failed to load.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum WaitForAssetError {
    NotLoaded,
    Failed(Arc<AssetLoadError>),
    DependencyFailed(Arc<AssetLoadError>),
}
```

---

## Struct StaticTyped Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.StaticTyped.html

**Contents:**
- Struct StaticTyped Copy item path
- Auto Trait Implementations§
  - impl Freeze for StaticTyped
  - impl RefUnwindSafe for StaticTyped
  - impl Send for StaticTyped
  - impl Sync for StaticTyped
  - impl Unpin for StaticTyped
  - impl UnwindSafe for StaticTyped
- Blanket Implementations§
  - impl<T> Any for Twhere T: 'static + ?Sized,

NestedLoader will be provided the type of asset as a type parameter on load.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct StaticTyped(/* private fields */);
```

---

## Enum AssetId Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.AssetId.html

**Contents:**
- Enum AssetId Copy item path
- Variants§
  - Index
    - Fields
  - Uuid
    - Fields
- Implementations§
  - impl<A> AssetId<A>where A: Asset,
    - pub const DEFAULT_UUID: Uuid
    - pub const INVALID_UUID: Uuid

A unique runtime-only identifier for an Asset. This is cheap to Copy/Clone and is not directly tied to the lifetime of the Asset. This means it can point to an Asset that no longer exists.

For an identifier tied to the lifetime of an asset, see Handle.

For an “untyped” / “generic-less” id, see UntypedAssetId.

A small / efficient runtime identifier that can be used to efficiently look up an asset stored in Assets. This is the “default” identifier used for assets. The alternative(s) (ex: AssetId::Uuid) will only be used if assets are explicitly registered that way.

The unstable, opaque index of the asset.

A marker to store the type information of the asset.

A stable-across-runs / const asset identifier. This will only be used if an asset is explicitly registered in Assets with one.

The UUID provided during asset registration.

The uuid for the default AssetId. It is valid to assign a value to this in Assets and by convention (where appropriate) assets should support this pattern.

This asset id should never be valid. Assigning a value to this in Assets will produce undefined behavior, so don’t do it!

Returns an AssetId with Self::INVALID_UUID, which should never be assigned to.

Converts this to an “untyped” / “generic-less” Asset identifier that stores the type information inside the UntypedAssetId.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum AssetId<A>where
    A: Asset,{
    Index {
        index: AssetIndex,
        marker: PhantomData<fn() -> A>,
    },
    Uuid {
        uuid: Uuid,
    },
}
```

Example 2 (javascript):
```javascript
215fn queue_custom_phase_item(
216    pipeline_cache: Res<PipelineCache>,
217    mut pipeline: ResMut<CustomPhasePipeline>,
218    mut opaque_render_phases: ResMut<ViewBinnedRenderPhases<Opaque3d>>,
219    opaque_draw_functions: Res<DrawFunctions<Opaque3d>>,
220    views: Query<(&ExtractedView, &RenderVisibleEntities, &Msaa)>,
221    mut next_tick: Local<Tick>,
222) {
223    let draw_custom_phase_item = opaque_draw_functions
224        .read()
225        .id::<DrawCustomPhaseItemCommands>();
226
227    // Render phases are per-view, so we need to iterate over all views so that
228    // the en
...
```

Example 3 (javascript):
```javascript
230fn extract_image_materials(
231    mut material_instances: ResMut<RenderMaterialInstances>,
232    changed_meshes_query: Extract<
233        Query<
234            (Entity, &ViewVisibility, &ImageMaterial3d),
235            Or<(Changed<ViewVisibility>, Changed<ImageMaterial3d>)>,
236        >,
237    >,
238) {
239    let last_change_tick = material_instances.current_change_tick;
240
241    for (entity, view_visibility, material) in &changed_meshes_query {
242        if view_visibility.get() {
243            material_instances.instances.insert(
244                entity.into(),
245           
...
```

Example 4 (javascript):
```javascript
215fn queue_custom_phase_item(
216    pipeline_cache: Res<PipelineCache>,
217    mut pipeline: ResMut<CustomPhasePipeline>,
218    mut opaque_render_phases: ResMut<ViewBinnedRenderPhases<Opaque3d>>,
219    opaque_draw_functions: Res<DrawFunctions<Opaque3d>>,
220    views: Query<(&ExtractedView, &RenderVisibleEntities, &Msaa)>,
221    mut next_tick: Local<Tick>,
222) {
223    let draw_custom_phase_item = opaque_draw_functions
224        .read()
225        .id::<DrawCustomPhaseItemCommands>();
226
227    // Render phases are per-view, so we need to iterate over all views so that
228    // the en
...
```

---

## Struct SceneLoader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/scene/struct.SceneLoader.html

**Contents:**
- Struct SceneLoader Copy item path
- Trait Implementations§
  - impl AssetLoader for SceneLoader
    - type Asset = DynamicScene
    - type Settings = ()
    - type Error = SceneLoaderError
    - async fn load( &self, reader: &mut dyn Reader, _settings: &(), _load_context: &mut LoadContext<'_>, ) -> Result<<SceneLoader as AssetLoader>::Asset, <SceneLoader as AssetLoader>::Error>
    - fn extensions(&self) -> &[&str]
  - impl Debug for SceneLoader
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

Asset loader for a Bevy dynamic scene (.scn / .scn.ron).

The loader handles assets serialized with DynamicScene::serialize.

Processes the asset in an asynchronous closure.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct SceneLoader { /* private fields */ }
```

---

## Enum AssetLoadError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.AssetLoadError.html

**Contents:**
- Enum AssetLoadError Copy item path
- Variants§
  - RequestedHandleTypeMismatch
    - Fields
  - MissingAssetLoader
    - Fields
  - MissingAssetLoaderForExtension(MissingAssetLoaderForExtensionError)
  - MissingAssetLoaderForTypeName(MissingAssetLoaderForTypeNameError)
  - MissingAssetLoaderForTypeIdError(MissingAssetLoaderForTypeIdError)
  - AssetReaderError(AssetReaderError)

An error that occurs during an Asset load.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum AssetLoadError {
Show 16 variants    RequestedHandleTypeMismatch {
        path: AssetPath<'static>,
        requested: TypeId,
        actual_asset_name: &'static str,
        loader_name: &'static str,
    },
    MissingAssetLoader {
        loader_name: Option<String>,
        asset_type_id: Option<TypeId>,
        extension: Option<String>,
        asset_path: Option<String>,
    },
    MissingAssetLoaderForExtension(MissingAssetLoaderForExtensionError),
    MissingAssetLoaderForTypeName(MissingAssetLoaderForTypeNameError),
    MissingAssetLoaderForTypeIdError(MissingAssetLoaderFo
...
```

---

## Module processor Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/processor/index.html

**Contents:**
- Module processor Copy item path
- §Usage
- §Defining asset processors
- Structs§
- Enums§
- Traits§

Asset processing in Bevy is a framework for automatically transforming artist-authored assets into the format that best suits the needs of your particular game.

You can think of the asset processing system as a “build system” for assets. When an artist adds a new asset to the project or an asset is changed (assuming asset hot reloading is enabled), the asset processing system will automatically perform the specified processing steps on the asset. This can include things like creating lightmaps for baked lighting, compressing a .wav file to an .ogg, or generating mipmaps for a texture.

Taken together, this means that the original asset plus the processing steps should be enough to regenerate the final asset. While it may be possible to manually edit the final asset, this should be discouraged. Final post-processed assets should generally not be version-controlled, except to save developer time when recomputing heavy asset processing steps.

Asset processing can be enabled or disabled in AssetPlugin by setting the AssetMode. Enable Bevy’s file_watcher feature to automatically watch for changes to assets and reprocess them.

To register a new asset processor, use AssetProcessor::register_processor. To set the default asset processor for a given extension, use AssetProcessor::set_default_processor. In most cases, these methods will be called directly on App using the AssetApp extension trait.

If a default asset processor is set, assets with a matching extension will be processed using that processor before loading.

For an end-to-end example, check out the examples in the examples/asset/processing directory of the Bevy repository.

Bevy provides two different ways to define new asset processors:

In most cases, LoadTransformAndSave should be sufficient.

---

## Module saver Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/saver/index.html

**Contents:**
- Module saver Copy item path
- Structs§
- Traits§

bevy::assetModule saver Copy item pathSource Structs§SavedAssetAn Asset (and any labeled “sub assets”) intended to be saved.Traits§AssetSaverSaves an Asset of a given AssetSaver::Asset type. AssetSaver::OutputLoader will then be used to load the saved asset in the final deployed application. The saver should produce asset bytes in a format that AssetSaver::OutputLoader can read.ErasedAssetSaverA type-erased dynamic variant of AssetSaver that allows callers to save assets without knowing the actual type of the AssetSaver.

---

## Enum ParseAssetPathError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.ParseAssetPathError.html

**Contents:**
- Enum ParseAssetPathError Copy item path
- Variants§
  - InvalidSourceSyntax
  - InvalidLabelSyntax
  - MissingSource
  - MissingLabel
- Trait Implementations§
  - impl Debug for ParseAssetPathError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for ParseAssetPathError

An error that occurs when parsing a string type to create an AssetPath fails, such as during AssetPath::parse.

Error that occurs when the AssetPath::source section of a path string contains the AssetPath::label delimiter #. E.g. bad#source://file.test.

Error that occurs when the AssetPath::label section of a path string contains the AssetPath::source delimiter ://. E.g. source://file.test#bad://label.

Error that occurs when a path string has an AssetPath::source delimiter :// with no characters preceding it. E.g. ://file.test.

Error that occurs when a path string has an AssetPath::label delimiter # with no characters succeeding it. E.g. file.test#

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ParseAssetPathError {
    InvalidSourceSyntax,
    InvalidLabelSyntax,
    MissingSource,
    MissingLabel,
}
```

---

## Enum ReadAssetBytesError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.ReadAssetBytesError.html

**Contents:**
- Enum ReadAssetBytesError Copy item path
- Variants§
  - DeserializeMetaError(DeserializeMetaError)
  - AssetReaderError(AssetReaderError)
  - MissingAssetSourceError(MissingAssetSourceError)
  - MissingProcessedAssetReaderError(MissingProcessedAssetReaderError)
  - Io
    - Fields
  - MissingAssetHash
- Trait Implementations§

An error produced when calling LoadContext::read_asset_bytes

Encountered an I/O error while loading an asset.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum ReadAssetBytesError {
    DeserializeMetaError(DeserializeMetaError),
    AssetReaderError(AssetReaderError),
    MissingAssetSourceError(MissingAssetSourceError),
    MissingProcessedAssetReaderError(MissingProcessedAssetReaderError),
    Io {
        path: PathBuf,
        source: Error,
    },
    MissingAssetHash,
}
```

---

## Trait Settings Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/meta/trait.Settings.html

**Contents:**
- Trait Settings Copy item path
- Implementations§
  - impl dyn Settings
    - pub fn is<__T>(&self) -> boolwhere __T: Settings,
    - pub fn downcast<__T>( self: Box<dyn Settings>, ) -> Result<Box<__T>, Box<dyn Settings>>where __T: Settings,
    - pub fn downcast_rc<__T>( self: Rc<dyn Settings>, ) -> Result<Rc<__T>, Rc<dyn Settings>>where __T: Settings,
    - pub fn downcast_ref<__T>(&self) -> Option<&__T>where __T: Settings,
    - pub fn downcast_mut<__T>(&mut self) -> Option<&mut __T>where __T: Settings,
- Implementors§
  - impl<T> Settings for Twhere T: 'static + Send + Sync,

Settings used by the asset system, such as by AssetLoader, Process, and AssetSaver

Returns true if the trait object wraps an object of type __T.

Returns a boxed object from a boxed trait object if the underlying object is of type __T. Returns the original boxed trait if it isn’t.

Returns an Rc-ed object from an Rc-ed trait object if the underlying object is of type __T. Returns the original Rc-ed trait if it isn’t.

Returns a reference to the object within the trait object if it is of type __T, or None if it isn’t.

Returns a mutable reference to the object within the trait object if it is of type __T, or None if it isn’t.

**Examples:**

Example 1 (unknown):
```unknown
pub trait Settings:
    Downcast
    + Send
    + Sync
    + 'static { }
```

---

## Struct MissingAssetLoaderForTypeNameError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.MissingAssetLoaderForTypeNameError.html

**Contents:**
- Struct MissingAssetLoaderForTypeNameError Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for MissingAssetLoaderForTypeNameError
    - fn clone(&self) -> MissingAssetLoaderForTypeNameError
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for MissingAssetLoaderForTypeNameError
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
  - impl Display for MissingAssetLoaderForTypeNameError
    - fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>

An error that occurs when an AssetLoader is not registered for a given core::any::type_name.

The type name that was not found.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct MissingAssetLoaderForTypeNameError {
    pub type_name: String,
}
```

---

## Constant SAMPLER_ASSET_INDEX Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/constant.SAMPLER_ASSET_INDEX.html

**Contents:**
- Constant SAMPLER_ASSET_INDEX Copy item path

bevy::imageConstant SAMPLER_ASSET_INDEX Copy item pathSource pub const SAMPLER_ASSET_INDEX: u64 = 1; // 1u64

**Examples:**

Example 1 (javascript):
```javascript
pub const SAMPLER_ASSET_INDEX: u64 = 1; // 1u64
```

---

## Struct LoadedAsset Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.LoadedAsset.html

**Contents:**
- Struct LoadedAsset Copy item path
- Implementations§
  - impl<A> LoadedAsset<A>where A: Asset,
    - pub fn new_with_dependencies(value: A) -> LoadedAsset<A>
    - pub fn take(self) -> A
    - pub fn get(&self) -> &A
      - Examples found in repository?
    - pub fn get_labeled( &self, label: impl Into<CowArc<'static, str>>, ) -> Option<&ErasedLoadedAsset>
    - pub fn iter_labels(&self) -> impl Iterator<Item = &str>
- Trait Implementations§

The successful result of an AssetLoader::load call. This contains the loaded “root” asset and any other “labeled” assets produced by the loader. It also holds the input AssetMeta (if it exists) and tracks dependencies:

Create a new loaded asset. This will use VisitAssetDependencies to populate dependencies.

Cast (and take ownership) of the Asset value of the given type.

Retrieves a reference to the internal Asset type.

Returns the ErasedLoadedAsset for the given label, if it exists.

Iterate over all labels for “labeled assets” in the loaded asset

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct LoadedAsset<A>where
    A: Asset,{ /* private fields */ }
```

Example 2 (javascript):
```javascript
141    async fn load(
142        &self,
143        reader: &mut dyn Reader,
144        _settings: &Self::Settings,
145        load_context: &mut LoadContext<'_>,
146    ) -> Result<CoolText, Self::Error> {
147        let mut bytes = Vec::new();
148        reader.read_to_end(&mut bytes).await?;
149        let ron: CoolTextRon = ron::de::from_bytes(&bytes)?;
150        let mut base_text = ron.text;
151        for embedded in ron.embedded_dependencies {
152            let loaded = load_context
153                .loader()
154                .immediate()
155                .load::<Text>(&embedded)
...
```

---

## Trait AssetReader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/io/trait.AssetReader.html

**Contents:**
- Trait AssetReader Copy item path
- Required Methods§
    - fn read<'a>(&'a self, path: &'a Path) -> impl AssetReaderFuture + Reader + 'a
      - §Note for implementors
    - fn read_meta<'a>( &'a self, path: &'a Path, ) -> impl AssetReaderFuture + Reader + 'a
    - fn read_directory<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture
    - fn is_directory<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture
- Provided Methods§
    - fn read_meta_bytes<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture
- Dyn Compatibility§

Performs read operations on an asset storage. AssetReader exposes a “virtual filesystem” API, where asset bytes and asset metadata bytes are both stored and accessible for a given path. This trait is not object safe, if needed use a dyn ErasedAssetReader instead.

This trait defines asset-agnostic mechanisms to read bytes from a storage system. For the per-asset-type saving/loading logic, see AssetSaver and AssetLoader.

For a complementary version of this trait that can write assets to storage, see AssetWriter.

Returns a future to load the full file data at the provided path.

The preferred style for implementing this method is an async fn returning an opaque type.

Returns a future to load the full file data at the provided path.

Returns an iterator of directory entry names at the provided path.

Returns true if the provided path points to a directory.

Reads asset metadata bytes at the given path into a Vec<u8>. This is a convenience function that wraps AssetReader::read_meta by default.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait AssetReader:
    Send
    + Sync
    + 'static {
    // Required methods
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> impl AssetReaderFuture + Reader + 'a;
    fn read_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> impl AssetReaderFuture + Reader + 'a;
    fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> impl ConditionalSendFuture;
    fn is_directory<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture;

    // Provided method
    fn read_meta_bytes<'a>(
        &'a self,
        path: &'a Path,
    ) -> impl Condi
...
```

Example 2 (unknown):
```unknown
impl AssetReader for MyReader {
    async fn read<'a>(&'a self, path: &'a Path) -> Result<impl Reader + 'a, AssetReaderError> {
        // ...
    }
}
```

---

## Constant TRANSPARENT_IMAGE_HANDLE Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/constant.TRANSPARENT_IMAGE_HANDLE.html

**Contents:**
- Constant TRANSPARENT_IMAGE_HANDLE Copy item path

A handle to a 1 x 1 transparent white image.

Like Handle<Image>::default, this is a handle to a fallback image asset. While that handle points to an opaque white 1 x 1 image, this handle points to a transparent 1 x 1 white image.

**Examples:**

Example 1 (javascript):
```javascript
pub const TRANSPARENT_IMAGE_HANDLE: Handle<Image>;
```

---

## Trait Reader Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/io/trait.Reader.html

**Contents:**
- Trait Reader Copy item path
- Provided Methods§
    - fn read_to_end<'a>( &'a mut self, buf: &'a mut Vec<u8>, ) -> StackFuture<'a, Result<usize, Error>, bevy_asset::::io::Reader::read_to_end::{constant#0}> ⓘ
      - §Note for implementors
      - Examples found in repository?
- Trait Implementations§
  - impl Reader for Box<dyn Reader + '_>
    - fn read_to_end<'a>( &'a mut self, buf: &'a mut Vec<u8>, ) -> StackFuture<'a, Result<usize, Error>, bevy_asset::::io::{impl#7}::read_to_end::{constant#0}> ⓘ
- Implementations on Foreign Types§
  - impl Reader for File

A type returned from AssetReader::read, which is used to read the contents of a file (or virtual file) corresponding to an asset.

This is essentially a trait alias for types implementing AsyncRead and AsyncSeekForward. The only reason a blanket implementation is not provided for applicable types is to allow implementors to override the provided implementation of Reader::read_to_end.

Reads the entire contents of this reader and appends them to a vec.

You should override the provided implementation if you can fill up the buffer more efficiently than the default implementation, which calls poll_read repeatedly to fill up the buffer 32 bytes at a time.

**Examples:**

Example 1 (unknown):
```unknown
pub trait Reader:
    AsyncRead
    + AsyncSeekForward
    + Unpin
    + Send
    + Sync {
    // Provided method
    fn read_to_end<'a>(
        &'a mut self,
        buf: &'a mut Vec<u8>,
    ) -> StackFuture<'a, Result<usize, Error>, bevy_asset::::io::Reader::read_to_end::{constant#0}> ⓘ { ... }
}
```

Example 2 (javascript):
```javascript
39    async fn load(
40        &self,
41        reader: &mut dyn Reader,
42        _settings: &(),
43        _load_context: &mut LoadContext<'_>,
44    ) -> Result<Self::Asset, Self::Error> {
45        let mut bytes = Vec::new();
46        reader.read_to_end(&mut bytes).await?;
47        let custom_asset = ron::de::from_bytes::<CustomAsset>(&bytes)?;
48        Ok(custom_asset)
49    }
50
51    fn extensions(&self) -> &[&str] {
52        &["custom"]
53    }
54}
55
56#[derive(Asset, TypePath, Debug)]
57struct Blob {
58    bytes: Vec<u8>,
59}
60
61#[derive(Default)]
62struct BlobAssetLoader;
63
6
...
```

Example 3 (javascript):
```javascript
84    async fn load(
85        &self,
86        reader: &mut dyn Reader,
87        settings: &TextSettings,
88        _load_context: &mut LoadContext<'_>,
89    ) -> Result<Text, Self::Error> {
90        let mut bytes = Vec::new();
91        reader.read_to_end(&mut bytes).await?;
92        let value = if let Some(ref text) = settings.text_override {
93            text.clone()
94        } else {
95            String::from_utf8(bytes).unwrap()
96        };
97        Ok(Text(value))
98    }
99
100    fn extensions(&self) -> &[&str] {
101        &["txt"]
102    }
103}
104
105#[derive(Serialize, De
...
```

Example 4 (javascript):
```javascript
43    async fn load(
44        &self,
45        reader: &mut dyn Reader,
46        _settings: &(),
47        load_context: &mut LoadContext<'_>,
48    ) -> Result<Self::Asset, Self::Error> {
49        let compressed_path = load_context.path();
50        let file_name = compressed_path
51            .file_name()
52            .ok_or(GzAssetLoaderError::IndeterminateFilePath)?
53            .to_string_lossy();
54        let uncompressed_file_name = file_name
55            .strip_suffix(".gz")
56            .ok_or(GzAssetLoaderError::IndeterminateFilePath)?;
57        let contained_path = compres
...
```

---
