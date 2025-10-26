# Crates-Rs-Bevy - Utils

**Pages:** 20

---

## Enum UntypedAssetId Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.UntypedAssetId.html

**Contents:**
- Enum UntypedAssetId Copy item path
- Variants§
  - Index
    - Fields
  - Uuid
    - Fields
- Implementations§
  - impl UntypedAssetId
    - pub fn typed_unchecked<A>(self) -> AssetId<A>where A: Asset,
      - Examples found in repository?

An “untyped” / “generic-less” Asset identifier that behaves much like AssetId, but stores the Asset type information at runtime instead of compile-time. This increases the size of the type, but it enables storing asset ids across asset types together and enables comparisons between them.

A small / efficient runtime identifier that can be used to efficiently look up an asset stored in Assets. This is the “default” identifier used for assets. The alternative(s) (ex: UntypedAssetId::Uuid) will only be used if assets are explicitly registered that way.

An identifier that records the underlying asset type.

The unstable, opaque index of the asset.

A stable-across-runs / const asset identifier. This will only be used if an asset is explicitly registered in Assets with one.

An identifier that records the underlying asset type.

The UUID provided during asset registration.

Converts this to a “typed” AssetId without checking the stored type to see if it matches the target A Asset type. This should only be called if you are absolutely certain the asset type matches the stored type. And even then, you should consider using UntypedAssetId::typed_debug_checked instead.

Converts this to a “typed” AssetId. When compiled in debug-mode it will check to see if the stored type matches the target A Asset type. When compiled in release-mode, this check will be skipped.

Panics if compiled in debug mode and the TypeId of A does not match the stored TypeId.

Converts this to a “typed” AssetId.

Panics if the TypeId of A does not match the stored type id.

Try to convert this to a “typed” AssetId.

Returns the stored TypeId of the referenced Asset.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum UntypedAssetId {
    Index {
        type_id: TypeId,
        index: AssetIndex,
    },
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

---

## Crate uuid Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/uuid/index.html

**Contents:**
- Crate uuid Copy item path
- §Getting started
- §Working with different UUID versions
  - §Which UUID version should I use?
- §Other features
- §Unstable features
- §Building for other targets
  - §WebAssembly
  - §Embedded
- §Examples

Generate and parse universally unique identifiers (UUIDs).

Here’s an example of a UUID:

A UUID is a unique 128-bit value, stored as 16 octets, and regularly formatted as a hex string in five groups. UUIDs are used to assign unique identifiers to entities without requiring a central allocating authority.

They are particularly useful in distributed systems, though can be used in disparate areas, such as databases and network protocols. Typically a UUID is displayed in a readable string form as a sequence of hexadecimal digits, separated into groups by hyphens.

The uniqueness property is not strictly guaranteed, however for all practical purposes, it can be assumed that an unintentional collision would be extremely unlikely.

UUIDs have a number of standardized encodings that are specified in RFC 9562.

Add the following to your Cargo.toml:

When you want a UUID, you can generate one:

If you have a UUID value, you can use its string literal form inline:

This library supports all standardized methods for generating UUIDs through individual Cargo features.

By default, this crate depends on nothing but the Rust standard library and can parse and format UUIDs, but cannot generate them. Depending on the kind of UUID you’d like to work with, there are Cargo features that enable generating them:

This library also includes a Builder type that can be used to help construct UUIDs of any version without any additional dependencies or features. It’s a lower-level API than Uuid that can be used when you need control over implicit requirements on things like a source of randomness.

If you just want to generate unique identifiers then consider version 4 (v4) UUIDs. If you want to use UUIDs as database keys or need to sort them then consider version 7 (v7) UUIDs. Other versions should generally be avoided unless there’s an existing need for them.

Some UUID versions supersede others. Prefer version 6 over version 1 and version 5 over version 3.

Other crate features can also be useful beyond the version support:

Some features are unstable. They may be incomplete or depend on other unstable libraries. These include:

Unstable features may break between minor releases.

To allow unstable features, you’ll need to enable the Cargo feature as normal, but also pass an additional flag through your environment to opt-in to unstable uuid features:

For WebAssembly, enable the js feature:

For embedded targets without the standard library, you’ll need to disable default feat

*[Content truncated]*

**Examples:**

Example 1 (text):
```text
67e55044-10b1-426f-9247-bb680e5fe0c8
```

Example 2 (toml):
```toml
[dependencies.uuid]
version = "1.18.1"
# Lets you generate random UUIDs
features = [
    "v4",
]
```

Example 3 (javascript):
```javascript
use uuid::Uuid;

let id = Uuid::new_v4();
```

Example 4 (javascript):
```javascript
use uuid::{uuid, Uuid};

const ID: Uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
```

---

## Struct HashMap Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/platform/collections/struct.HashMap.html

**Contents:**
- Struct HashMap Copy item path
- Implementations§
  - impl<K, V> HashMap<K, V>
    - pub const fn new() -> HashMap<K, V>
      - §Examples
    - pub fn with_capacity(capacity: usize) -> HashMap<K, V>
      - §Examples
  - impl<K, V, S> HashMap<K, V, S>
    - pub const fn with_hasher(hash_builder: S) -> HashMap<K, V, S>
      - §Examples

New-type for HashMap with FixedHasher as the default hashing provider. Can be trivially converted to and from a hashbrown HashMap using From.

A new-type is used instead of a type alias due to critical methods like new being incompatible with Bevy’s choice of default hasher.

Unlike hashbrown::HashMap, HashMap defaults to FixedHasher instead of RandomState. This provides determinism by default with an acceptable compromise to denial of service resistance in the context of a game engine.

Creates an empty HashMap.

Refer to new for further details.

Creates an empty HashMap with the specified capacity.

Refer to with_capacity for further details.

Creates an empty HashMap which will use the given hash builder to hash keys.

Refer to with_hasher for further details.

Creates an empty HashMap with the specified capacity, using hash_builder to hash the keys.

Refer to with_capacity_and_hasher for further details.

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

Clears the map, removing all key-value pairs. Keeps the al

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct HashMap<K, V, S = FixedHasher>(/* private fields */);
```

Example 2 (javascript):
```javascript
// Creates a HashMap with zero capacity.
let map = HashMap::new();
```

Example 3 (javascript):
```javascript
// Creates a HashMap with capacity for at least 5 entries.
let map = HashMap::with_capacity(5);
```

Example 4 (javascript):
```javascript
// Creates a HashMap with the provided hasher.
let map = HashMap::with_hasher(SomeHasher);
```

---

## Module cfg Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/utils/cfg/index.html

**Contents:**
- Module cfg Copy item path

Configuration information for this crate.

---

## Struct Parallel Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/utils/struct.Parallel.html

**Contents:**
- Struct Parallel Copy item path
- Implementations§
  - impl<T> Parallel<T>where T: Send,
    - pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T>
    - pub fn clear(&mut self)
    - pub fn scope_or<R>( &self, create: impl FnOnce() -> T, f: impl FnOnce(&mut T) -> R, ) -> R
    - pub fn borrow_local_mut_or(&self, create: impl FnOnce() -> T) -> impl DerefMut
  - impl<T> Parallel<T>where T: Default + Send,
    - pub fn scope<R>(&self, f: impl FnOnce(&mut T) -> R) -> R
    - pub fn borrow_local_mut(&self) -> impl DerefMut

A cohesive set of thread-local values of a given type.

Mutable references can be fetched if T: Default via Parallel::scope.

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

Collect all enqueued items from all threads and appends them to the end of a single Vec.

The ordering is not guaranteed.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Parallel<T>where
    T: Send,{ /* private fields */ }
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

Example 3 (unknown):
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

## Type Alias TypeIdMap Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/utils/type.TypeIdMap.html

**Contents:**
- Type Alias TypeIdMap Copy item path
- Aliased Type§

A specialized hashmap type with Key of TypeId Iteration order only depends on the order of insertions and deletions.

**Examples:**

Example 1 (unknown):
```unknown
pub type TypeIdMap<V> = HashMap<TypeId, V, NoOpHash>;
```

Example 2 (unknown):
```unknown
pub struct TypeIdMap<V>(/* private fields */);
```

---

## Type Alias PreHashMap Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/utils/type.PreHashMap.html

**Contents:**
- Type Alias PreHashMap Copy item path
- Aliased Type§

A HashMap pre-configured to use Hashed keys and PassHash passthrough hashing. Iteration order only depends on the order of insertions and deletions.

**Examples:**

Example 1 (unknown):
```unknown
pub type PreHashMap<K, V> = HashMap<Hashed<K>, V, PassHash>;
```

Example 2 (unknown):
```unknown
pub struct PreHashMap<K, V>(/* private fields */);
```

---

## Constant DEFAULT_MAX_HISTORY_LENGTH Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/diagnostic/constant.DEFAULT_MAX_HISTORY_LENGTH.html

**Contents:**
- Constant DEFAULT_MAX_HISTORY_LENGTH Copy item path

Default max history length for new diagnostics.

**Examples:**

Example 1 (javascript):
```javascript
pub const DEFAULT_MAX_HISTORY_LENGTH: usize = 120; // 120usize
```

---

## Macro once Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/utils/macro.once.html

**Contents:**
- Macro once Copy item path

Call some expression only once per call site.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! once {
    ($expression:expr) => { ... };
}
```

---

## Crate utils Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/utils/index.html

**Contents:**
- Crate utils Copy item path
- Modules§
- Macros§
- Structs§
- Traits§
- Functions§
- Type Aliases§

General utilities for first-party Bevy engine crates.

---

## Constant DEFAULT_FILTER Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/constant.DEFAULT_FILTER.html

**Contents:**
- Constant DEFAULT_FILTER Copy item path

The default LogPlugin EnvFilter.

**Examples:**

Example 1 (javascript):
```javascript
pub const DEFAULT_FILTER: &'static str;
```

---

## Trait PreHashMapExt Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/utils/trait.PreHashMapExt.html

**Contents:**
- Trait PreHashMapExt Copy item path
- Required Methods§
    - fn get_or_insert_with<F>(&mut self, key: &Hashed<K>, func: F) -> &mut Vwhere F: FnOnce() -> V,
- Dyn Compatibility§
- Implementors§
  - impl<K, V> PreHashMapExt<K, V> for HashMap<Hashed<K>, V, PassHash>where K: Hash + Eq + PartialEq + Clone,

Extension methods intended to add functionality to PreHashMap.

Tries to get or insert the value for the given key using the pre-computed hash first. If the PreHashMap does not already contain the key, it will clone it and insert the value returned by func.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait PreHashMapExt<K, V> {
    // Required method
    fn get_or_insert_with<F>(&mut self, key: &Hashed<K>, func: F) -> &mut V
       where F: FnOnce() -> V;
}
```

---

## Trait BevyDefault Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/trait.BevyDefault.html

**Contents:**
- Trait BevyDefault Copy item path
- Required Methods§
    - fn bevy_default() -> Self
- Dyn Compatibility§
- Implementors§
  - impl BevyDefault for TextureFormat

Trait used to provide default values for Bevy-external types that do not implement Default.

Returns the default value for a type.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait BevyDefault {
    // Required method
    fn bevy_default() -> Self;
}
```

---

## Enum WriteDefaultMetaError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/enum.WriteDefaultMetaError.html

**Contents:**
- Enum WriteDefaultMetaError Copy item path
- Variants§
  - MissingAssetLoader(MissingAssetLoaderForExtensionError)
  - MissingAssetSource(MissingAssetSourceError)
  - MissingAssetWriter(MissingAssetWriterError)
  - FailedToWriteMeta(AssetWriterError)
  - MetaAlreadyExists
  - IoErrorFromExistingMetaCheck(Arc<Error>)
  - HttpErrorFromExistingMetaCheck(u16)
- Trait Implementations§

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum WriteDefaultMetaError {
    MissingAssetLoader(MissingAssetLoaderForExtensionError),
    MissingAssetSource(MissingAssetSourceError),
    MissingAssetWriter(MissingAssetWriterError),
    FailedToWriteMeta(AssetWriterError),
    MetaAlreadyExists,
    IoErrorFromExistingMetaCheck(Arc<Error>),
    HttpErrorFromExistingMetaCheck(u16),
}
```

---

## Macro uuid_handle Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/macro.uuid_handle.html

**Contents:**
- Macro uuid_handle Copy item path
- §Examples

Creates a Handle from a string literal containing a UUID.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! uuid_handle {
    ($uuid:expr) => { ... };
}
```

Example 2 (javascript):
```javascript
const IMAGE: Handle<Image> = uuid_handle!("1347c9b7-c46a-48e7-b7b8-023a354b7cac");
```

---

## Trait TypeIdMapExt Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/utils/trait.TypeIdMapExt.html

**Contents:**
- Trait TypeIdMapExt Copy item path
- §Examples
- Required Methods§
    - fn insert_type<T>(&mut self, v: V) -> Option<V>where T: 'static + ?Sized,
    - fn get_type<T>(&self) -> Option<&V>where T: 'static + ?Sized,
    - fn get_type_mut<T>(&mut self) -> Option<&mut V>where T: 'static + ?Sized,
    - fn remove_type<T>(&mut self) -> Option<V>where T: 'static + ?Sized,
    - fn entry_type<T>(&mut self) -> Entry<'_, TypeId, V, NoOpHash>where T: 'static + ?Sized,
- Dyn Compatibility§
- Implementors§

Extension trait to make use of TypeIdMap more ergonomic.

Each function on this trait is a trivial wrapper for a function on HashMap, replacing a TypeId key with a generic parameter T.

Inserts a value for the type T.

If the map did not previously contain this key then None is returned, otherwise the value for this key is updated and the old value returned.

Returns a reference to the value for type T, if one exists.

Returns a mutable reference to the value for type T, if one exists.

Removes type T from the map, returning the value for this key if it was previously present.

Gets the type T’s entry in the map for in-place manipulation.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait TypeIdMapExt<V> {
    // Required methods
    fn insert_type<T>(&mut self, v: V) -> Option<V>
       where T: 'static + ?Sized;
    fn get_type<T>(&self) -> Option<&V>
       where T: 'static + ?Sized;
    fn get_type_mut<T>(&mut self) -> Option<&mut V>
       where T: 'static + ?Sized;
    fn remove_type<T>(&mut self) -> Option<V>
       where T: 'static + ?Sized;
    fn entry_type<T>(&mut self) -> Entry<'_, TypeId, V, NoOpHash>
       where T: 'static + ?Sized;
}
```

Example 2 (javascript):
```javascript
use bevy_utils::TypeIdMapExt;

struct MyType;

// Using the built-in `HashMap` functions requires manually looking up `TypeId`s.
let mut map = TypeIdMap::default();
map.insert(TypeId::of::<MyType>(), 7);
assert_eq!(map.get(&TypeId::of::<MyType>()), Some(&7));

// Using `TypeIdMapExt` functions does the lookup for you.
map.insert_type::<MyType>(7);
assert_eq!(map.get_type::<MyType>(), Some(&7));
```

---

## Struct Assets Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/asset/struct.Assets.html

**Contents:**
- Struct Assets Copy item path
- Implementations§
  - impl<A> Assets<A>where A: Asset,
    - pub fn get_handle_provider(&self) -> AssetHandleProvider
    - pub fn reserve_handle(&self) -> Handle<A>
    - pub fn insert( &mut self, id: impl Into<AssetId<A>>, asset: A, ) -> Result<(), InvalidGenerationError>
      - Examples found in repository?
    - pub fn get_or_insert_with( &mut self, id: impl Into<AssetId<A>>, insert_fn: impl FnOnce() -> A, ) -> Result<&mut A, InvalidGenerationError>
    - pub fn contains(&self, id: impl Into<AssetId<A>>) -> bool
    - pub fn add(&mut self, asset: impl Into<A>) -> Handle<A>

Stores Asset values identified by their AssetId.

Assets identified by AssetId::Index will be stored in a “dense” vec-like storage. This is more efficient, but it means that the assets can only be identified at runtime. This is the default behavior.

Assets identified by AssetId::Uuid will be stored in a hashmap. This is less efficient, but it means that the assets can be referenced at compile time.

This tracks (and queues) AssetEvent events whenever changes to the collection occur. To check whether the asset used by a given component has changed (due to a change in the handle or the underlying asset) use the AssetChanged query filter.

Retrieves an AssetHandleProvider capable of reserving new Handle values for assets that will be stored in this collection.

Reserves a new Handle for an asset that will be stored in this collection.

Inserts the given asset, identified by the given id. If an asset already exists for id, it will be replaced.

Note: This will never return an error for UUID asset IDs.

Retrieves an Asset stored for the given id if it exists. If it does not exist, it will be inserted using insert_fn.

Note: This will never return an error for UUID asset IDs.

Returns true if the id exists in this collection. Otherwise it returns false.

Adds the given asset and allocates a new strong Handle for it.

Upgrade an AssetId into a strong Handle that will prevent asset drop.

Returns None if the provided id is not part of this Assets collection. For example, it may have been dropped earlier.

Retrieves a reference to the Asset with the given id, if it exists. Note that this supports anything that implements Into<AssetId<A>>, which includes Handle and AssetId.

Retrieves a mutable reference to the Asset with the given id, if it exists. Note that this supports anything that implements Into<AssetId<A>>, which includes Handle and AssetId.

Retrieves a mutable reference to the Asset with the given id, if it exists.

This is the same as Assets::get_mut except it doesn’t emit AssetEvent::Modified.

Removes (and returns) the Asset with the given id, if it exists. Note that this supports anything that implements Into<AssetId<A>>, which includes Handle and AssetId.

Removes (and returns) the Asset with the given id, if it exists. This skips emitting AssetEvent::Removed. Note that this supports anything that implements Into<AssetId<A>>, which includes Handle and AssetId.

This is the same as Assets::remove except it doesn’t emit AssetEvent::Removed.

Returns tr

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct Assets<A>where
    A: Asset,{ /* private fields */ }
```

Example 2 (javascript):
```javascript
226fn resize_image(
227    image_mesh: Query<(&MeshMaterial3d<StandardMaterial>, &Mesh3d), With<HDRViewer>>,
228    materials: Res<Assets<StandardMaterial>>,
229    mut meshes: ResMut<Assets<Mesh>>,
230    images: Res<Assets<Image>>,
231    mut image_event_reader: MessageReader<AssetEvent<Image>>,
232) {
233    for event in image_event_reader.read() {
234        let (AssetEvent::Added { id } | AssetEvent::Modified { id }) = event else {
235            continue;
236        };
237
238        for (mat_h, mesh_h) in &image_mesh {
239            let Some(mat) = materials.get(mat_h) else {
240      
...
```

Example 3 (javascript):
```javascript
97fn setup(mut assets: ResMut<Assets<SineAudio>>, mut commands: Commands) {
98    // add a `SineAudio` to the asset server so that it can be played
99    let audio_handle = assets.add(SineAudio {
100        frequency: 440., // this is the frequency of A4
101    });
102    commands.spawn(AudioPlayer(audio_handle));
103}
```

Example 4 (unknown):
```unknown
12fn setup(
13    mut commands: Commands,
14    mut meshes: ResMut<Assets<Mesh>>,
15    mut materials: ResMut<Assets<ColorMaterial>>,
16) {
17    commands.spawn(Camera2d);
18
19    commands.spawn((
20        Mesh2d(meshes.add(Rectangle::default())),
21        MeshMaterial2d(materials.add(Color::from(PURPLE))),
22        Transform::default().with_scale(Vec3::splat(128.)),
23    ));
24}
```

---

## Struct DefaultSpatialScale Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/struct.DefaultSpatialScale.html

**Contents:**
- Struct DefaultSpatialScale Copy item path
- Tuple Fields§
- Trait Implementations§
  - impl Clone for DefaultSpatialScale
    - fn clone(&self) -> DefaultSpatialScale
    - fn clone_from(&mut self, source: &Self)
  - impl Default for DefaultSpatialScale
    - fn default() -> DefaultSpatialScale
  - impl FromArg for DefaultSpatialScale
    - type This<'from_arg> = DefaultSpatialScale

The default scale factor applied to the positions of audio sources and listeners for spatial audio. Can be overridden for individual sounds in PlaybackSettings.

You may need to adjust this scale to fit your world’s units.

Default is Vec3::ONE.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct DefaultSpatialScale(pub SpatialScale);
```

---

## Function default Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/utils/fn.default.html

**Contents:**
- Function default Copy item path
      - Examples found in repository?

An ergonomic abbreviation for Default::default() to make initializing structs easier.

This is especially helpful when combined with “struct update syntax”.

**Examples:**

Example 1 (unknown):
```unknown
pub fn default<T>() -> Twhere
    T: Default,
```

Example 2 (javascript):
```javascript
use bevy_utils::default;

#[derive(Default)]
struct Foo {
  a: usize,
  b: usize,
  c: usize,
}

// Normally you would initialize a struct with defaults using "struct update syntax"
// combined with `Default::default()`. This example sets `Foo::a` to 10 and the remaining
// values to their defaults.
let foo = Foo {
  a: 10,
  ..Default::default()
};

// But now you can do this, which is equivalent:
let foo = Foo {
  a: 10,
  ..default()
};
```

Example 3 (javascript):
```javascript
72    fn default() -> Self {
73        Self::Global(default())
74    }
75}
76
77/// Buttons consist of three parts: the button itself, a label child, and a
78/// value child. This specifies one of the three entities.
79#[derive(Clone, Copy, PartialEq, Component)]
80enum ColorGradingOptionWidgetType {
81    /// The parent button.
82    Button,
83    /// The label of the button.
84    Label,
85    /// The numerical value that the button displays.
86    Value,
87}
88
89#[derive(Clone, Copy, Component)]
90struct ColorGradingOptionWidget {
91    widget_type: ColorGradingOptionWidgetType,
92    opti
...
```

Example 4 (unknown):
```unknown
91    fn new(quantize_steps: u32) -> Self {
92        Self {
93            quantize_steps,
94            ..default()
95        }
96    }
```

---

## Struct OnDrop Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/utils/struct.OnDrop.html

**Contents:**
- Struct OnDrop Copy item path
- §Examples
- Implementations§
  - impl<F> OnDrop<F>where F: FnOnce(),
    - pub fn new(callback: F) -> OnDrop<F>
- Trait Implementations§
  - impl<F> Drop for OnDrop<F>where F: FnOnce(),
    - fn drop(&mut self)
- Auto Trait Implementations§
  - impl<F> Freeze for OnDrop<F>where F: Freeze,

A type which calls a function when dropped. This can be used to ensure that cleanup code is run even in case of a panic.

Note that this only works for panics that unwind – any code within OnDrop will be skipped if a panic does not unwind. In most cases, this will just work.

Returns an object that will invoke the specified callback when dropped.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct OnDrop<F>where
    F: FnOnce(),{ /* private fields */ }
```

Example 2 (javascript):
```javascript
// This will print a message when the variable `_catch` gets dropped,
// even if a panic occurs before we reach the end of this scope.
// This is similar to a `try ... catch` block in languages such as C++.
let _catch = OnDrop::new(|| log("Oops, a panic occurred and this function didn't complete!"));

// Some code that may panic...
// ...

// Make sure the message only gets printed if a panic occurs.
// If we remove this line, then the message will be printed regardless of whether a panic occurs
// -- similar to a `try ... finally` block.
core::mem::forget(_catch);
```

---
