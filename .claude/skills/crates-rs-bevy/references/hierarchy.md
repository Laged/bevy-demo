# Crates-Rs-Bevy - Hierarchy

**Pages:** 3

---

## Crate transform Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/transform/index.html

**Contents:**
- Crate transform Copy item path
- §Bevy Transform
- Modules§
- Structs§
- Enums§
- Traits§

This crate contains types and functions associated with the Transform component.

---

## Struct PreviousGlobalTransform Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.PreviousGlobalTransform.html

**Contents:**
- Struct PreviousGlobalTransform Copy item path
- Tuple Fields§
- Trait Implementations§
  - impl Component for PreviousGlobalTransformwhere PreviousGlobalTransform: Send + Sync + 'static,
    - const STORAGE_TYPE: StorageType = bevy_ecs::component::StorageType::Table
    - type Mutability = Mutable
    - fn register_required_components( _requiree: ComponentId, required_components: &mut RequiredComponentsRegistrator<'_, '_>, )
    - fn clone_behavior() -> ComponentCloneBehavior
    - fn on_add() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>
    - fn on_insert() -> Option<for<'w> fn(DeferredWorld<'w>, HookContext)>

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct PreviousGlobalTransform(pub Affine3A);
```

---

## Struct GltfNode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/struct.GltfNode.html

**Contents:**
- Struct GltfNode Copy item path
- Fields§
- Implementations§
  - impl GltfNode
    - pub fn new( node: &Node<'_>, children: Vec<Handle<GltfNode>>, mesh: Option<Handle<GltfMesh>>, transform: Transform, skin: Option<Handle<GltfSkin>>, extras: Option<GltfExtras>, ) -> GltfNode
    - pub fn with_animation_root(self, is_animation_root: bool) -> GltfNode
    - pub fn asset_label(&self) -> GltfAssetLabel
- Trait Implementations§
  - impl Clone for GltfNode
    - fn clone(&self) -> GltfNode

A glTF node with all of its child nodes, its GltfMesh, Transform, its optional GltfSkin and an optional GltfExtras.

See the relevant glTF specification section.

Index of the node inside the scene

Computed name for a node - either a user defined node name from gLTF or a generated name from index

Direct children of the node.

Is this node used as an animation root

Create a node extracting name and index from glTF def

Create a node with animation root mark

Subasset label for this node within the gLTF parent asset.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GltfNode {
    pub index: usize,
    pub name: String,
    pub children: Vec<Handle<GltfNode>>,
    pub mesh: Option<Handle<GltfMesh>>,
    pub skin: Option<Handle<GltfSkin>>,
    pub transform: Transform,
    pub is_animation_root: bool,
    pub extras: Option<GltfExtras>,
}
```

---
