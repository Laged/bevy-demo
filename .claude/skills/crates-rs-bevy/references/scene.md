# Crates-Rs-Bevy - Scene

**Pages:** 4

---

## Struct SerializedImage Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/image/struct.SerializedImage.html

**Contents:**
- Struct SerializedImage Copy item path
- Implementations§
  - impl SerializedImage
    - pub fn from_image(image: Image) -> SerializedImage
    - pub fn into_image(self) -> Image
- Trait Implementations§
  - impl Clone for SerializedImage
    - fn clone(&self) -> SerializedImage
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for SerializedImage

A version of Image suitable for serializing for short-term transfer.

Image does not implement Serialize / Deserialize because it is made with the renderer in mind. It is not a general-purpose image implementation, and its internals are subject to frequent change. As such, storing an Image on disk is highly discouraged. Use an existing image asset format such as .png instead!

But there are still some valid use cases for serializing an Image, namely transferring images between processes. To support this, you can create a SerializedImage from an Image with SerializedImage::from_image, and then deserialize it with SerializedImage::into_image.

Creates a new SerializedImage from an Image.

Create an Image from a SerializedImage.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct SerializedImage { /* private fields */ }
```

---

## Struct GltfSceneExtras Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/struct.GltfSceneExtras.html

**Contents:**
- Struct GltfSceneExtras Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for GltfSceneExtras
    - fn clone(&self) -> GltfSceneExtras
    - fn clone_from(&mut self, source: &Self)
  - impl Component for GltfSceneExtraswhere GltfSceneExtras: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

Additional untyped data that can be present on most glTF types at the scene level.

See the relevant glTF specification section.

Content of the extra data.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GltfSceneExtras {
    pub value: String,
}
```

---

## Struct GltfExtras Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/struct.GltfExtras.html

**Contents:**
- Struct GltfExtras Copy item path
- Fields§
- Trait Implementations§
  - impl Clone for GltfExtras
    - fn clone(&self) -> GltfExtras
    - fn clone_from(&mut self, source: &Self)
  - impl Component for GltfExtraswhere GltfExtras: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )

Additional untyped data that can be present on most glTF types at the primitive level.

See the relevant glTF specification section.

Content of the extra data.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GltfExtras {
    pub value: String,
}
```

---

## Struct SceneInstance Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/scene/struct.SceneInstance.html

**Contents:**
- Struct SceneInstance Copy item path
- Trait Implementations§
  - impl Component for SceneInstancewhere SceneInstance: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior
    - fn on_add() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_insert() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_replace() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>

InstanceId of a spawned scene. It can be used with the SceneSpawner to interact with the spawned scene.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct SceneInstance(/* private fields */);
```

---
