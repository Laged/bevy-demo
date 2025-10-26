# Crates-Rs-Bevy - Other

**Pages:** 49

---

## Module retained Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/retained/index.html

**Contents:**
- Module retained Copy item path
- Structs§

This module is for ‘retained’ alternatives to the ‘immediate mode’ Gizmos system parameter.

---

## Module arcs Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/arcs/index.html

**Contents:**
- Module arcs Copy item path
- Structs§

Additional GizmoBuffer Functions – Arcs

Includes the implementation of GizmoBuffer::arc_2d, and assorted support items.

---

## Module rounded_corners Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/feathers/rounded_corners/index.html

**Contents:**
- Module rounded_corners Copy item path
- Enums§

Mechanism for specifying which corners of a widget are rounded, used for segmented buttons and control groups.

---

## Module primitives Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/primitives/index.html

**Contents:**
- Module primitives Copy item path
- Modules§

A module for rendering each of the 2D and 3D bevy_math::primitives with Gizmos.

---

## Module graph Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/graph/index.html

**Contents:**
- Module graph Copy item path
- Enums§

bevy::pbrModule graph Copy item pathSource Enums§NodePbrRender graph nodes specific to 3D PBR rendering.

---

## Module resources Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/resources/index.html

**Contents:**
- Module resources Copy item path
- Structs§

bevy::pbrModule resources Copy item pathSource Structs§AtmosphereSamplersAtmosphereTexturesAtmosphereTransformAtmosphereTransformsAtmosphereTransformsOffset

---

## Crate pbr Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/index.html

**Contents:**
- Crate pbr Copy item path
- Modules§
- Structs§
- Enums§
- Constants§
- Traits§
- Functions§
- Type Aliases§

bevyCrate pbr Copy item pathSource Modules§decalDecal rendering.deferredenvironment_mapEnvironment maps and reflection probes.experimentalExperimental features that are not yet finished. Please report any issues you encounter!generateLike EnvironmentMapLight, but filtered in realtime from a cubemap.graphirradiance_volumeIrradiance volumes, also known as voxel global illumination.preludeThe PBR prelude.resourceswireframeStructs§AtmosphereThis component describes the atmosphere of a planet, and when added to a camera will enable atmospheric scattering for that camera. This is only compatible with HDR cameras.AtmosphereSettingsThis component controls the resolution of the atmosphere LUTs, and how many samples are used when computing them.BluenoiseA resource that stores the spatio-temporal blue noise texture.BuildIndirectParametersBindGroupsThe bind groups for the compute shaders that reset indirect draw counts and build indirect parameters.BuildIndirectParametersPipelineThe pipeline for the indirect parameter building shader.BuildIndirectParametersPipelineKeySpecifies variants of the indirect parameter building shader.ClearIndirectParametersMetadataNodeThe render node that clears out the GPU-side indirect metadata buffers.ClusteredDecalPluginA plugin that adds support for clustered decals.DefaultOpaqueRendererMethodDefault render method used for opaque materials.DeferredDrawFunctionDeferredFragmentShaderDeferredVertexShaderDistanceFogConfigures the “classic” computer graphics distance fog effect, in which objects appear progressively more covered in atmospheric haze the further away they are from the camera. Affects meshes rendered via the PBR StandardMaterial.DrawMeshEarlyGpuPreprocessNodeThe render node for the first mesh preprocessing pass.EarlyPrepassBuildIndirectParametersNodeThe render node for the part of the indirect parameter building pass that draws the meshes visible from the previous frame.EarlyShadowPassNodeThe rendering node that renders meshes that were “visible” (so to speak) from a light last frame.EntitiesNeedingSpecializationEntitySpecializationTicksEnvironmentMapUniformThe uniform struct extracted from EnvironmentMapLight. Will be available for use in the Environment Map shader.EnvironmentMapUniformBufferA GPU buffer that stores the environment map settings for each view.ErasedMaterialKeyErasedMaterialKeyVTableErasedMaterialPipelineKeyExtendedMaterialA material that extends a base Material with additional shaders and data.ExtractedClusterC

*[Content truncated]*

---

## Module smaa Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/anti_alias/smaa/index.html

**Contents:**
- Module smaa Copy item path
- Structs§
- Enums§
- Functions§

Subpixel morphological antialiasing (SMAA).

SMAA is a 2011 antialiasing technique that takes an aliased image and smooths out the jaggies, making edges smoother. It’s been used in numerous games and has become a staple postprocessing technique. Compared to MSAA, SMAA has the advantage of compatibility with deferred rendering and reduction of GPU memory bandwidth. Compared to FXAA, SMAA has the advantage of improved quality, but the disadvantage of reduced performance. Compared to TAA, SMAA has the advantage of stability and lack of ghosting artifacts, but has the disadvantage of not supporting temporal accumulation, which have made SMAA less popular when advanced photorealistic rendering features are used in recent years.

To use SMAA, add Smaa to a bevy_camera::Camera. In a pinch, you can simply use the default settings (via the Default trait) for a high-quality, high-performance appearance. When using SMAA, you will likely want set bevy_render::view::Msaa to bevy_render::view::Msaa::Off for every camera using SMAA.

Those who have used SMAA in other engines should be aware that Bevy doesn’t yet support the following more advanced features of SMAA:

The temporal variant.

Depth- and chroma-based edge detection.

Predicated thresholding.

Compatibility with SSAA and MSAA.

---

## Crate tasks Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/tasks/index.html

**Contents:**
- Crate tasks Copy item path
- §Bevy Tasks
  - §Usage
  - §no_std Support
- Modules§
- Structs§
- Traits§
- Functions§
- Type Aliases§

A refreshingly simple task executor for bevy. :)

This is a simple threadpool with minimal dependencies. The main usecase is a scoped fork-join, i.e. spawning tasks from a single thread and having that thread await the completion of those tasks. This is intended specifically for bevy as a lighter alternative to rayon for this specific usecase. There are also utilities for generating the tasks from a slice of data. This library is intended for games and makes no attempt to ensure fairness or ordering of spawned tasks.

It is based on async-executor, a lightweight executor that allows the end user to manage their own threads. async-executor is based on async-task, a core piece of async-std.

In order to be able to optimize task execution in multi-threaded environments, bevy provides three different thread pools via which tasks of different kinds can be spawned. (The same API is used in single-threaded environments, even if execution is limited to a single thread. This currently applies to Wasm targets.) The determining factor for what kind of work should go in each pool is latency requirements:

For CPU-intensive work (tasks that generally spin until completion) we have a standard ComputeTaskPool and an AsyncComputeTaskPool. Work that does not need to be completed to present the next frame should go to the AsyncComputeTaskPool.

For IO-intensive work (tasks that spend very little time in a “woken” state) we have an IoTaskPool whose tasks are expected to complete very quickly. Generally speaking, they should just await receiving data from somewhere (i.e. disk) and signal other systems when the data is ready for consumption. (likely via channels)

To enable no_std support in this crate, you will need to disable default features, and enable the edge_executor and critical-section features.

---

## Crate gizmos Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/index.html

**Contents:**
- Crate gizmos Copy item path
- §Example
- Modules§
- Structs§
- Enums§
- Traits§
- Functions§
- Type Aliases§

This crate adds an immediate mode drawing api to Bevy for visual debugging.

See the documentation on Gizmos for more examples.

**Examples:**

Example 1 (unknown):
```unknown
fn system(mut gizmos: Gizmos) {
    gizmos.line(Vec3::ZERO, Vec3::X, GREEN);
}
```

---

## Module ci_testing Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/dev_tools/ci_testing/index.html

**Contents:**
- Module ci_testing Copy item path
- Structs§
- Enums§

Utilities for testing in CI environments.

---

## Module palettes Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/palettes/index.html

**Contents:**
- Module palettes Copy item path
- Modules§

Color palettes consisting of collections of const colors.

---

## Crate feathers Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/feathers/index.html

**Contents:**
- Crate feathers Copy item path
  - §Warning: Experimental!
- Modules§
- Structs§

bevy_feathers is a collection of styled and themed widgets for building editors and inspectors.

The aesthetic choices made here are designed with a future Bevy Editor in mind, but this crate is deliberately exposed to the public to allow the broader ecosystem to easily create tooling for themselves and others that fits cohesively together.

While it may be tempting to use this crate for your game’s UI, it’s deliberately not intended for that. We’ve opted for a clean, functional style, and prioritized consistency over customization. That said, if you like what you see, it can be a helpful learning tool. Consider copying this code into your own project, and refining the styles and abstractions provided to meet your needs.

All that said, this crate is still experimental and unfinished! It will change in breaking ways, and there will be both bugs and limitations.

Please report issues, submit fixes and propose changes. Thanks for stress-testing; let’s build something better together.

---

## Module experimental Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/experimental/index.html

**Contents:**
- Module experimental Copy item path
- Modules§

Experimental features that are not yet finished. Please report any issues you encounter!

Expect bugs, missing features, compatibility issues, low performance, and/or future breaking changes.

---

## Crate reflect Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/reflect/index.html

**Contents:**
- Crate reflect Copy item path
- §The Reflect and PartialReflect traits
  - §Converting between PartialReflect and Reflect
- §Implementing Reflect
  - §Requirements
- §The Reflection Subtraits
  - §Opaque Types
- §Dynamic Types
  - §Patching
  - §FromReflect

Reflection is a powerful tool provided within many programming languages that allows for meta-programming: using information about the program to affect the program. In other words, reflection allows us to inspect the program itself, its syntax, and its type information at runtime.

This crate adds this missing reflection functionality to Rust. Though it was made with the Bevy game engine in mind, it’s a general-purpose solution that can be used in any Rust project.

At a very high level, this crate allows you to:

It’s important to note that because of missing features in Rust, there are some limitations with this crate.

At the root of bevy_reflect is the PartialReflect trait.

Its purpose is to allow dynamic introspection of values, following Rust’s type system through a system of subtraits.

Its primary purpose is to allow all implementors to be passed around as a dyn PartialReflect trait object in one of the following forms:

This allows values of types implementing PartialReflect to be operated upon completely dynamically (at a small runtime cost).

Building on PartialReflect is the Reflect trait.

PartialReflect is a supertrait of Reflect so any type implementing Reflect implements PartialReflect by definition. dyn Reflect trait objects can be used similarly to dyn PartialReflect, but Reflect is also often used in trait bounds (like T: Reflect).

The distinction between PartialReflect and Reflect is summarized in the following:

Since T: Reflect implies T: PartialReflect, conversion from a dyn Reflect to a dyn PartialReflect trait object (upcasting) is infallible and can be performed with one of the following methods. Note that these are temporary while the language feature for dyn upcasting coercion is experimental:

For conversion in the other direction — downcasting dyn PartialReflect to dyn Reflect — there are fallible methods:

Additionally, FromReflect::from_reflect can be used to convert a dyn PartialReflect to a concrete type which implements Reflect.

Implementing Reflect (and PartialReflect) is easily done using the provided derive macro:

This will automatically generate the implementation of Reflect for any struct or enum.

It will also generate other very important trait implementations used for reflection:

We can implement Reflect on any type that satisfies both of the following conditions:

Additionally, using the derive macro on enums requires a third condition to be met:

Since PartialReflect is meant to cover any and every type, t

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
#[derive(Reflect)]
struct MyStruct {
  foo: i32
}
```

Example 2 (javascript):
```javascript
let my_struct: Box<dyn Struct> = Box::new(MyStruct {
  foo: 123
});
let foo: &dyn PartialReflect = my_struct.field("foo").unwrap();
assert_eq!(Some(&123), foo.try_downcast_ref::<i32>());
```

Example 3 (javascript):
```javascript
let my_tuple: Box<dyn PartialReflect> = Box::new((1, 2, 3));
let my_tuple = my_tuple.reflect_ref().as_tuple().unwrap();
assert_eq!(3, my_tuple.field_len());
```

Example 4 (javascript):
```javascript
let mut data = DynamicStruct::default();
data.insert("foo", 123_i32);
assert_eq!(Some(&123), data.field("foo").unwrap().try_downcast_ref::<i32>())
```

---

## Crate tracing Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/tracing/index.html

**Contents:**
- Crate tracing Copy item path
- §Overview
- §Core Concepts
  - §Spans
  - §Events
  - §Subscribers
- §Usage
  - §Recording Spans and Events
    - §Spans
    - §Events

A scoped, structured logging and diagnostics system.

tracing is a framework for instrumenting Rust programs to collect structured, event-based diagnostic information.

In asynchronous systems like Tokio, interpreting traditional log messages can often be quite challenging. Since individual tasks are multiplexed on the same thread, associated events and log lines are intermixed making it difficult to trace the logic flow. tracing expands upon logging-style diagnostics by allowing libraries and applications to record structured events with additional information about temporality and causality — unlike a log message, a span in tracing has a beginning and end time, may be entered and exited by the flow of execution, and may exist within a nested tree of similar spans. In addition, tracing spans are structured, with the ability to record typed data as well as textual messages.

The tracing crate provides the APIs necessary for instrumenting libraries and applications to emit trace data.

Compiler support: requires rustc 1.63+

The core of tracing’s API is composed of spans, events and subscribers. We’ll cover these in turn.

To record the flow of execution through a program, tracing introduces the concept of spans. Unlike a log line that represents a moment in time, a span represents a period of time with a beginning and an end. When a program begins executing in a context or performing a unit of work, it enters that context’s span, and when it stops executing in that context, it exits the span. The span in which a thread is currently executing is referred to as that thread’s current span.

The span module’s documentation provides further details on how to use spans.

Warning: In asynchronous code that uses async/await syntax, Span::enter may produce incorrect traces if the returned drop guard is held across an await point. See the method documentation for details.

An Event represents a moment in time. It signifies something that happened while a trace was being recorded. Events are comparable to the log records emitted by unstructured logging code, but unlike a typical log line, an Event may occur within the context of a span.

In general, events should be used to represent points in time within a span — a request returned with a given status code, n new items were taken from a queue, and so on.

The Event struct documentation provides further details on using events.

As Spans and Events occur, they are recorded or aggregated by implementations of the Subs

*[Content truncated]*

**Examples:**

Example 1 (javascript):
```javascript
use tracing::{span, Level};
let span = span!(Level::TRACE, "my_span");
// `enter` returns a RAII guard which, when dropped, exits the span. this
// indicates that we are in the span for the current lexical scope.
let _enter = span.enter();
// perform some work in the context of `my_span`...
```

Example 2 (unknown):
```unknown
Span::enter
```

Example 3 (javascript):
```javascript
use tracing::{event, span, Level};

// records an event outside of any span context:
event!(Level::INFO, "something happened");

let span = span!(Level::INFO, "my_span");
let _guard = span.enter();

// records an event within "my_span".
event!(Level::DEBUG, "something happened inside my_span");
```

Example 4 (toml):
```toml
[dependencies]
tracing = "0.1"
```

---

## Module light Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/light/index.html

**Contents:**
- Module light Copy item path
- Structs§
- Enums§

A module adding debug visualization of PointLights, SpotLights and DirectionalLights.

---

## Module color_difference Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/color_difference/index.html

**Contents:**
- Module color_difference Copy item path
- Traits§

Module for calculating distance between two colors in the same color space.

---

## Module arrows Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/arrows/index.html

**Contents:**
- Module arrows Copy item path
- Structs§

Additional GizmoBuffer Functions – Arrows

Includes the implementation of GizmoBuffer::arrow and GizmoBuffer::arrow_2d, and assorted support items.

---

## Module generate Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/generate/index.html

**Contents:**
- Module generate Copy item path
- Structs§
- Enums§
- Functions§

Like EnvironmentMapLight, but filtered in realtime from a cubemap.

An environment map needs to be processed to be able to support uses beyond a simple skybox, such as reflections, and ambient light contribution. This process is called filtering, and can either be done ahead of time (prefiltering), or in realtime, although at a reduced quality. Prefiltering is preferred, but not always possible: sometimes you only gain access to an environment map at runtime, for whatever reason. Typically this is from realtime reflection probes, but can also be from other sources.

In any case, Bevy supports both modes of filtering. This module provides realtime filtering via bevy_light::GeneratedEnvironmentMapLight. For prefiltered environment maps, see bevy_light::EnvironmentMapLight. These components are intended to be added to a camera.

---

## Module theme Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/feathers/theme/index.html

**Contents:**
- Module theme Copy item path
- Structs§

A framework for theming.

---

## Module circles Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/circles/index.html

**Contents:**
- Module circles Copy item path
- Structs§

Additional GizmoBuffer Functions – Circles

Includes the implementation of GizmoBuffer::circle and GizmoBuffer::circle_2d, and assorted support items.

---

## Crate color Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/color/index.html

**Contents:**
- Crate color Copy item path
- §Color Space Usage
- §Conversion
- §Other Utilities
- §Example
- Modules§
- Structs§
- Enums§
- Traits§

Representations of colors in various color spaces.

This crate provides a number of color representations, including:

Each of these color spaces is represented as a distinct Rust type.

Rendering engines typically use linear RGBA colors, which allow for physically accurate lighting calculations. However, linear RGBA colors are not perceptually uniform, because both human eyes and computer monitors have non-linear responses to light. “Standard” RGBA represents an industry-wide compromise designed to encode colors in a way that looks good to humans in as few bits as possible, but it is not suitable for lighting calculations.

Most image file formats and scene graph formats use standard RGBA, because graphic design tools are intended to be used by humans. However, 3D lighting calculations operate in linear RGBA, so it is important to convert standard colors to linear before sending them to the GPU. Most Bevy APIs will handle this conversion automatically, but if you are writing a custom shader, you will need to do this conversion yourself.

HSL and LCH are “cylindrical” color spaces, which means they represent colors as a combination of hue, saturation, and lightness (or chroma). These color spaces are useful for working with colors in an artistic way - for example, when creating gradients or color palettes. A gradient in HSL space from red to violet will produce a rainbow. The LCH color space is more perceptually accurate than HSL, but is less intuitive to work with.

HSV and HWB are very closely related to HSL in their derivation, having identical definitions for hue. Where HSL uses saturation and lightness, HSV uses a slightly modified definition of saturation, and an analog of lightness in the form of value. In contrast, HWB instead uses whiteness and blackness parameters, which can be used to lighten and darken a particular hue respectively.

Oklab and Oklch are perceptually uniform color spaces that are designed to be used for tasks such as image processing. They are not as widely used as the other color spaces, but are useful for tasks such as color correction and image analysis, where it is important to be able to do things like change color saturation without causing hue shifts.

XYZ is a foundational space commonly used in the definition of other more modern color spaces. The space is more formally known as CIE 1931, where the x and z axes represent a form of chromaticity, while y defines an illuminance level.

See also the Wikipedia article on col

*[Content truncated]*

**Examples:**

Example 1 (javascript):
```javascript
let color = Srgba::rgb(0.5, 0.5, 0.5);

// Using From explicitly
let linear_color = LinearRgba::from(color);

// Using Into
let linear_color: LinearRgba = color.into();
```

Example 2 (javascript):
```javascript
use bevy_color::{Srgba, Hsla};

let srgba = Srgba::new(0.5, 0.2, 0.8, 1.0);
let hsla: Hsla = srgba.into();

println!("Srgba: {:?}", srgba);
println!("Hsla: {:?}", hsla);
```

---

## Crate platform Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/platform/index.html

**Contents:**
- Crate platform Copy item path
- Modules§

Platform compatibility support for first-party Bevy engine crates.

---

## Module dark_theme Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/feathers/dark_theme/index.html

**Contents:**
- Module dark_theme Copy item path
- Functions§

The standard bevy_feathers dark theme.

---

## Crate gilrs Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gilrs/index.html

**Contents:**
- Crate gilrs Copy item path
- Structs§
- Type Aliases§

Systems and type definitions for gamepad handling in Bevy.

This crate is built on top of GilRs, a library that handles abstracting over platform-specific gamepad APIs.

---

## Module grid Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/grid/index.html

**Contents:**
- Module grid Copy item path
- Structs§

Additional GizmoBuffer Functions – Grids

Includes the implementation of GizmoBuffer::grid and GizmoBuffer::grid_2d. and assorted support items.

---

## Module config Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/config/index.html

**Contents:**
- Module config Copy item path
- Structs§
- Enums§
- Traits§
- Derive Macros§

A module for the GizmoConfig<T> Resource.

---

## Module rounded_box Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/rounded_box/index.html

**Contents:**
- Module rounded_box Copy item path
- Structs§

Additional GizmoBuffer Functions – Rounded cuboids and rectangles

Includes the implementation of GizmoBuffer::rounded_rect, GizmoBuffer::rounded_rect_2d and GizmoBuffer::rounded_cuboid. and assorted support items.

---

## Module curves Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/curves/index.html

**Contents:**
- Module curves Copy item path

Additional GizmoBuffer Functions – Curves

Includes the implementation of GizmoBuffer::curve_2d, GizmoBuffer::curve_3d and assorted support items.

---

## Module cluster Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/light/cluster/index.html

**Contents:**
- Module cluster Copy item path
- Modules§
- Structs§
- Enums§
- Functions§

Spatial clustering of objects, currently just point and spot lights.

---

## Crate solari Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/solari/index.html

**Contents:**
- Crate solari Copy item path
- Modules§
- Structs§

Provides raytraced lighting.

See SolariPlugins for more info.

---

## Macro error_once Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/macro.error_once.html

**Contents:**
- Macro error_once Copy item path

Call error! once per call site.

Useful for logging within systems which are called every frame.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! error_once {
    ($($arg:tt)+) => { ... };
}
```

---

## Module cross Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/cross/index.html

**Contents:**
- Module cross Copy item path

Additional GizmoBuffer Functions – Crosses

Includes the implementation of GizmoBuffer::cross and GizmoBuffer::cross_2d, and assorted support items.

---

## Module palette Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/feathers/palette/index.html

**Contents:**
- Module palette Copy item path
- Constants§

The Feathers standard color palette.

---

## Module gizmos Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/gizmos/index.html

**Contents:**
- Module gizmos Copy item path
- Structs§

A module for the Gizmos SystemParam.

---

## Crate anti_alias Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/anti_alias/index.html

**Contents:**
- Crate anti_alias Copy item path
- Modules§
- Structs§

bevyCrate anti_alias Copy item pathSource Modules§contrast_adaptive_sharpeningfxaasmaaSubpixel morphological antialiasing (SMAA).taaStructs§AntiAliasPluginAdds fxaa, smaa, taa, contrast aware sharpening, and optional dlss support.

---

## Macro once Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/macro.once.html

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

## Module controls Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/feathers/controls/index.html

**Contents:**
- Module controls Copy item path
- Structs§
- Enums§
- Functions§

Meta-module containing all feathers controls (widgets that are interactive).

---

## Module cursor Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/feathers/cursor/index.html

**Contents:**
- Module cursor Copy item path
- Structs§
- Enums§

Provides a way to automatically set the mouse cursor based on hovered entity.

---

## Crate post_process Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/post_process/index.html

**Contents:**
- Crate post_process Copy item path
- Modules§
- Structs§

bevyCrate post_process Copy item pathSource Modules§auto_exposurebloomdofDepth of field, a postprocessing effect that simulates camera focus.effect_stackMiscellaneous built-in postprocessing effects.motion_blurPer-object, per-pixel motion blur.msaa_writebackStructs§PostProcessPluginAdds bloom, motion blur, depth of field, and chromatic aberration support.

---

## Module aabb Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/gizmos/aabb/index.html

**Contents:**
- Module aabb Copy item path
- Structs§

A module adding debug visualization of Aabbs.

---

## Module tokens Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/feathers/tokens/index.html

**Contents:**
- Module tokens Copy item path
- Constants§

Design tokens used by Feathers themes.

The term “design token” is commonly used in UX design to mean the smallest unit of a theme, similar in concept to a CSS variable. Each token represents an assignment of a color or value to a specific visual aspect of a widget, such as background or border.

---

## Macro info_once Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/macro.info_once.html

**Contents:**
- Macro info_once Copy item path

Call info! once per call site.

Useful for logging within systems which are called every frame.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! info_once {
    ($($arg:tt)+) => { ... };
}
```

---

## Module environment_map Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/environment_map/index.html

**Contents:**
- Module environment_map Copy item path
- Structs§

Environment maps and reflection probes.

An environment map consists of a pair of diffuse and specular cubemaps that together reflect the static surrounding area of a region in space. When available, the PBR shader uses these to apply diffuse light and calculate specular reflections.

Environment maps come in two flavors, depending on what other components the entities they’re attached to have:

If attached to a view, they represent the objects located a very far distance from the view, in a similar manner to a skybox. Essentially, these view environment maps represent a higher-quality replacement for AmbientLight for outdoor scenes. The indirect light from such environment maps are added to every point of the scene, including interior enclosed areas.

If attached to a bevy_light::LightProbe, environment maps represent the immediate surroundings of a specific location in the scene. These types of environment maps are known as reflection probes.

Typically, environment maps are static (i.e. “baked”, calculated ahead of time) and so only reflect fixed static geometry. The environment maps must be pre-filtered into a pair of cubemaps, one for the diffuse component and one for the specular component, according to the split-sum approximation. To pre-filter your environment map, you can use the glTF IBL Sampler or its artist-friendly UI. The diffuse map uses the Lambertian distribution, while the specular map uses the GGX distribution.

The Khronos Group has several pre-filtered environment maps available for you to use.

Currently, reflection probes (i.e. environment maps attached to light probes) use binding arrays (also known as bindless textures) and consequently aren’t supported on WebGL2 or WebGPU. Reflection probes are also unsupported if GLSL is in use, due to naga limitations. Environment maps attached to views are, however, supported on all platforms.

---

## Module irradiance_volume Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/pbr/irradiance_volume/index.html

**Contents:**
- Module irradiance_volume Copy item path
  - §Why ambient cubes?

Irradiance volumes, also known as voxel global illumination.

An irradiance volume is a cuboid voxel region consisting of regularly-spaced precomputed samples of diffuse indirect light. They’re ideal if you have a dynamic object such as a character that can move about static non-moving geometry such as a level in a game, and you want that dynamic object to be affected by the light bouncing off that static geometry.

To use irradiance volumes, you need to precompute, or bake, the indirect light in your scene. Bevy doesn’t currently come with a way to do this. Fortunately, Blender provides a baking tool as part of the Eevee renderer, and its irradiance volumes are compatible with those used by Bevy. The bevy-baked-gi project provides a tool, export-blender-gi, that can extract the baked irradiance volumes from the Blender .blend file and package them up into a .ktx2 texture for use by the engine. See the documentation in the bevy-baked-gi project for more details on this workflow.

Like all light probes in Bevy, irradiance volumes are 1×1×1 cubes, centered on the origin, that can be arbitrarily scaled, rotated, and positioned in a scene with the bevy_transform::components::Transform component. The 3D voxel grid will be stretched to fill the interior of the cube, with linear interpolation, and the illumination from the irradiance volume will apply to all fragments within that bounding region.

Bevy’s irradiance volumes are based on Valve’s ambient cubes as used in Half-Life 2 (Mitchell 2006, slide 27). These encode a single color of light from the six 3D cardinal directions and blend the sides together according to the surface normal. For an explanation of why ambient cubes were chosen over spherical harmonics, see Why ambient cubes? below.

If you wish to use a tool other than export-blender-gi to produce the irradiance volumes, you’ll need to pack the irradiance volumes in the following format. The irradiance volume of resolution (Rx, Ry, Rz) is expected to be a 3D texture of dimensions (Rx, 2Ry, 3Rz). The unnormalized texture coordinate (s, t, p) of the voxel at coordinate (x, y, z) with side S ∈ {-X, +X, -Y, +Y, -Z, +Z} is as follows:

Visually, in a left-handed coordinate system with Y up, viewed from the right, the 3D texture looks like a stacked series of voxel grids, one for each cube side, in this order:

A terminology note: Other engines may refer to irradiance volumes as voxel global illumination, VXGI, or simply as light probes. Sometimes light pr

*[Content truncated]*

**Examples:**

Example 1 (text):
```text
s = x

t = y + ⎰  0 if S ∈ {-X, -Y, -Z}
        ⎱ Ry if S ∈ {+X, +Y, +Z}

        ⎧   0 if S ∈ {-X, +X}
p = z + ⎨  Rz if S ∈ {-Y, +Y}
        ⎩ 2Rz if S ∈ {-Z, +Z}
```

---

## Macro warn_once Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/macro.warn_once.html

**Contents:**
- Macro warn_once Copy item path

Call warn! once per call site.

Useful for logging within systems which are called every frame.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! warn_once {
    ($($arg:tt)+) => { ... };
}
```

---

## Crate dev_tools Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/dev_tools/index.html

**Contents:**
- Crate dev_tools Copy item path
- Modules§
- Structs§

This crate provides additional utilities for the Bevy game engine, focused on improving developer experience.

---

## Macro trace_once Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/log/macro.trace_once.html

**Contents:**
- Macro trace_once Copy item path

Call trace! once per call site.

Useful for logging within systems which are called every frame.

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! trace_once {
    ($($arg:tt)+) => { ... };
}
```

---
