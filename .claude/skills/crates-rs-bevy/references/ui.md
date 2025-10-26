# Crates-Rs-Bevy - Ui

**Pages:** 3

---

## Struct GltfSkin Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gltf/struct.GltfSkin.html

**Contents:**
- Struct GltfSkin Copy item path
- Fields§
- Implementations§
  - impl GltfSkin
    - pub fn new( skin: &Skin<'_>, joints: Vec<Handle<GltfNode>>, inverse_bind_matrices: Handle<SkinnedMeshInverseBindposes>, extras: Option<GltfExtras>, ) -> GltfSkin
    - pub fn asset_label(&self) -> GltfAssetLabel
- Trait Implementations§
  - impl Clone for GltfSkin
    - fn clone(&self) -> GltfSkin
    - fn clone_from(&mut self, source: &Self)

A glTF skin with all of its joint nodes, SkinnedMeshInversiveBindposes and an optional GltfExtras.

See the relevant glTF specification section.

Index of the skin inside the scene

Computed name for a skin - either a user defined skin name from gLTF or a generated name from index

All the nodes that form this skin.

Inverse-bind matrices of this skin.

Create a skin extracting name and index from glTF def

Subasset label for this skin within the gLTF parent asset.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GltfSkin {
    pub index: usize,
    pub name: String,
    pub joints: Vec<Handle<GltfNode>>,
    pub inverse_bind_matrices: Handle<SkinnedMeshInverseBindposes>,
    pub extras: Option<GltfExtras>,
}
```

---

## Struct ShadowPassNode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ShadowPassNode.html

**Contents:**
- Struct ShadowPassNode Copy item path
- Trait Implementations§
  - impl FromWorld for ShadowPassNode
    - fn from_world(world: &mut World) -> ShadowPassNode
- Auto Trait Implementations§
  - impl Freeze for ShadowPassNode
  - impl !RefUnwindSafe for ShadowPassNode
  - impl Send for ShadowPassNode
  - impl Sync for ShadowPassNode
  - impl Unpin for ShadowPassNode

Encapsulates rendering logic shared between the early and late shadow pass nodes.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ShadowPassNode { /* private fields */ }
```

---

## Struct ClearIndirectParametersMetadataNode Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/struct.ClearIndirectParametersMetadataNode.html

**Contents:**
- Struct ClearIndirectParametersMetadataNode Copy item path
- Trait Implementations§
  - impl Default for ClearIndirectParametersMetadataNode
    - fn default() -> ClearIndirectParametersMetadataNode
  - impl Node for ClearIndirectParametersMetadataNode
    - fn run<'w>( &self, _: &mut RenderGraphContext<'_>, render_context: &mut RenderContext<'w>, world: &'w World, ) -> Result<(), NodeRunError>
    - fn input(&self) -> Vec<SlotInfo>
    - fn output(&self) -> Vec<SlotInfo>
    - fn update(&mut self, _world: &mut World)
- Auto Trait Implementations§

The render node that clears out the GPU-side indirect metadata buffers.

This is only used when indirect drawing is enabled.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ClearIndirectParametersMetadataNode;
```

---
