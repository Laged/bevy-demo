# Crates-Rs-Bevy - Animation

**Pages:** 17

---

## Struct AnimationPlayer Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/struct.AnimationPlayer.html

**Contents:**
- Struct AnimationPlayer Copy item path
- Implementations§
  - impl AnimationPlayer
    - pub fn start(&mut self, animation: NodeIndex) -> &mut ActiveAnimation
    - pub fn play(&mut self, animation: NodeIndex) -> &mut ActiveAnimation
      - Examples found in repository?
    - pub fn stop(&mut self, animation: NodeIndex) -> &mut AnimationPlayer
    - pub fn stop_all(&mut self) -> &mut AnimationPlayer
    - pub fn playing_animations( &self, ) -> impl Iterator<Item = (&NodeIndex, &ActiveAnimation)>
      - Examples found in repository?

Automatically added to any root animations of a scene when it is spawned.

Start playing an animation, restarting it if necessary.

Start playing an animation, unless the requested animation is already playing.

Stops playing the given animation, removing it from the list of playing animations.

Stops all currently-playing animations.

Iterates through all animations that this AnimationPlayer is currently playing.

Iterates through all animations that this AnimationPlayer is currently playing, mutably.

Returns true if the animation is currently playing or paused, or false if the animation is stopped.

Check if all playing animations have finished, according to the repetition behavior.

Check if all playing animations are paused.

Pause all playing animations.

Resume all active animations.

Rewinds all active animations.

Multiplies the speed of all active animations by the given factor.

Seeks all active animations forward or backward by the same amount.

To seek forward, pass a positive value; to seek negative, pass a negative value. Values below 0.0 or beyond the end of the animation clip are clamped appropriately.

Returns the ActiveAnimation associated with the given animation node if it’s currently playing.

If the animation isn’t currently active, returns None.

Returns a mutable reference to the ActiveAnimation associated with the given animation node if it’s currently active.

If the animation isn’t currently active, returns None.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AnimationPlayer { /* private fields */ }
```

Example 2 (unknown):
```unknown
512fn play_animations(
513    mut commands: Commands,
514    assets: Res<ExampleAssets>,
515    mut players: Query<(Entity, &mut AnimationPlayer), Without<AnimationGraphHandle>>,
516) {
517    for (entity, mut player) in players.iter_mut() {
518        commands
519            .entity(entity)
520            .insert(AnimationGraphHandle(assets.fox_animation_graph.clone()));
521        player.play(assets.fox_animation_node).repeat();
522    }
523}
```

Example 3 (javascript):
```javascript
379fn init_animations(
380    mut commands: Commands,
381    mut query: Query<(Entity, &mut AnimationPlayer)>,
382    animation_graph: Res<ExampleAnimationGraph>,
383    mut done: Local<bool>,
384) {
385    if *done {
386        return;
387    }
388
389    for (entity, mut player) in query.iter_mut() {
390        commands.entity(entity).insert((
391            AnimationGraphHandle(animation_graph.0.clone()),
392            ExampleAnimationWeights::default(),
393        ));
394        for &node_index in &CLIP_NODE_INDICES {
395            player.play(node_index.into()).repeat();
396        }
39
...
```

Example 4 (javascript):
```javascript
58fn play_animation_when_ready(
59    scene_ready: On<SceneInstanceReady>,
60    mut commands: Commands,
61    children: Query<&Children>,
62    animations_to_play: Query<&AnimationToPlay>,
63    mut players: Query<&mut AnimationPlayer>,
64) {
65    if let Ok(animation_to_play) = animations_to_play.get(scene_ready.entity) {
66        for child in children.iter_descendants(scene_ready.entity) {
67            if let Ok(mut player) = players.get_mut(child) {
68                player.play(animation_to_play.index).repeat();
69
70                commands
71                    .entity(child)
72      
...
```

---

## Struct ActiveAnimation Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/struct.ActiveAnimation.html

**Contents:**
- Struct ActiveAnimation Copy item path
- Implementations§
  - impl ActiveAnimation
    - pub fn is_finished(&self) -> bool
    - pub fn replay(&mut self)
      - Examples found in repository?
    - pub fn weight(&self) -> f32
    - pub fn set_weight(&mut self, weight: f32) -> &mut ActiveAnimation
      - Examples found in repository?
    - pub fn pause(&mut self) -> &mut ActiveAnimation

An animation that an AnimationPlayer is currently either playing or was playing, but is presently paused.

A stopped animation is considered no longer active.

Check if the animation has finished, based on its repetition behavior and the number of times it has repeated.

Note: An animation with RepeatAnimation::Forever will never finish.

Reset back to the initial state as if no time has elapsed.

Returns the current weight of this animation.

Sets the weight of this animation.

Unpause the animation.

Returns true if this animation is currently paused.

Note that paused animations are still ActiveAnimations.

Sets the repeat mode for this playing animation.

Marks this animation as repeating forever.

Returns the repeat mode assigned to this active animation.

Returns the number of times this animation has completed.

Returns true if the animation is playing in reverse.

Returns the speed of the animation playback.

Sets the speed of the animation playback.

Returns the amount of time the animation has been playing.

Returns the seek time of the animation.

This is nonnegative and no more than the clip duration.

Seeks to a specific time in the animation.

This will not trigger events between the current time and seek_time. Use seek_to if this is desired.

Seeks to a specific time in the animation.

Note that any events between the current time and seek_time will be triggered on the next update. Use set_seek_time if this is undesired.

Seeks to the beginning of the animation.

Note that any events between the current time and 0.0 will be triggered on the next update. Use set_seek_time if this is undesired.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct ActiveAnimation { /* private fields */ }
```

Example 2 (javascript):
```javascript
128fn keyboard_control(
129    keyboard_input: Res<ButtonInput<KeyCode>>,
130    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
131    animations: Res<Animations>,
132    mut current_animation: Local<usize>,
133) {
134    for (mut player, mut transitions) in &mut animation_players {
135        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
136            continue;
137        };
138
139        if keyboard_input.just_pressed(KeyCode::Space) {
140            let playing_animation = player.animation_mut(playing_animation_index)
...
```

Example 3 (javascript):
```javascript
453fn sync_weights(mut query: Query<(&mut AnimationPlayer, &ExampleAnimationWeights)>) {
454    for (mut animation_player, animation_weights) in query.iter_mut() {
455        for (&animation_node_index, &animation_weight) in CLIP_NODE_INDICES
456            .iter()
457            .zip(animation_weights.weights.iter())
458        {
459            // If the animation happens to be no longer active, restart it.
460            if !animation_player.is_playing_animation(animation_node_index.into()) {
461                animation_player.play(animation_node_index.into());
462            }
463
464     
...
```

Example 4 (javascript):
```javascript
285    fn pause_animation_frame(
286        scene_ready: On<SceneInstanceReady>,
287        children: Query<&Children>,
288        mut commands: Commands,
289        animation: Res<Animation>,
290        mut players: Query<(Entity, &mut AnimationPlayer)>,
291    ) {
292        for child in children.iter_descendants(scene_ready.entity) {
293            if let Ok((entity, mut player)) = players.get_mut(child) {
294                let mut transitions = AnimationTransitions::new();
295                transitions
296                    .play(&mut player, animation.animation, Duration::ZERO)
297    
...
```

---

## Struct AnimationClip Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/struct.AnimationClip.html

**Contents:**
- Struct AnimationClip Copy item path
- Implementations§
  - impl AnimationClip
    - pub fn curves( &self, ) -> &HashMap<AnimationTargetId, Vec<VariableCurve>, NoOpHash>
    - pub fn curves_mut( &mut self, ) -> &mut HashMap<AnimationTargetId, Vec<VariableCurve>, NoOpHash>
    - pub fn curves_for_target( &self, target_id: AnimationTargetId, ) -> Option<&Vec<VariableCurve>>
    - pub fn curves_for_target_mut( &mut self, target_id: AnimationTargetId, ) -> Option<&mut Vec<VariableCurve>>
    - pub fn duration(&self) -> f32
    - pub fn set_duration(&mut self, duration_sec: f32)
      - Examples found in repository?

A list of VariableCurves and the AnimationTargetIds to which they apply.

Because animation clips refer to targets by UUID, they can target any AnimationTarget with that ID.

VariableCurves for each animation target. Indexed by the AnimationTargetId.

Get mutable references of VariableCurves for each animation target. Indexed by the AnimationTargetId.

Gets the curves for a single animation target.

Returns None if this clip doesn’t animate the target.

Gets mutable references of the curves for a single animation target.

Returns None if this clip doesn’t animate the target.

Duration of the clip, represented in seconds.

Set the duration of the clip in seconds.

Adds an AnimationCurve to an AnimationTarget named by an AnimationTargetId.

If the curve extends beyond the current duration of this clip, this method lengthens this clip to include the entire time span that the curve covers.

For example, a curve with domain [2, 5] will extend the clip to cover [0, 5] when added and will produce the same output on the entire interval [0, 2] because these time values all get clamped to 2.

By contrast, a curve with domain [-10, ∞] will never extend the clip duration when added and will be sampled only on [0, duration], ignoring all negative time values.

Like add_curve_to_target, but adding a VariableCurve directly.

Under normal circumstances, that method is generally more convenient.

Add an EntityEvent with no AnimationTarget to this AnimationClip.

The event will be cloned and triggered on the AnimationPlayer entity once the time (in seconds) is reached in the animation.

See also add_event_to_target.

Add an EntityEvent to an AnimationTarget named by an AnimationTargetId.

The event will be cloned and triggered on the entity matching the target once the time (in seconds) is reached in the animation.

Use add_event instead if you don’t have a specific target.

Add an event function with no AnimationTarget to this AnimationClip.

The func will trigger on the AnimationPlayer entity once the time (in seconds) is reached in the animation.

For a simpler EntityEvent-based alternative, see AnimationClip::add_event. See also add_event_to_target.

Add an event function to an AnimationTarget named by an AnimationTargetId.

The func will trigger on the entity matching the target once the time (in seconds) is reached in the animation.

For a simpler EntityEvent-based alternative, see AnimationClip::add_event_to_target. Use add_event instead if you don’t have a specific 

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub struct AnimationClip { /* private fields */ }
```

Example 2 (javascript):
```javascript
37fn setup(
38    mut commands: Commands,
39    mut animations: ResMut<Assets<AnimationClip>>,
40    mut graphs: ResMut<Assets<AnimationGraph>>,
41) {
42    // Camera
43    commands.spawn((
44        Camera2d,
45        Camera {
46            clear_color: ClearColorConfig::Custom(BLACK.into()),
47            ..Default::default()
48        },
49        Bloom {
50            intensity: 0.4,
51            ..Bloom::NATURAL
52        },
53    ));
54
55    // The text that will be changed by animation events.
56    commands.spawn((
57        MessageText,
58        Text2d::default(),
59        TextFo
...
```

Example 3 (javascript):
```javascript
36    fn create(
37        animation_graphs: &mut Assets<AnimationGraph>,
38        animation_clips: &mut Assets<AnimationClip>,
39    ) -> AnimationInfo {
40        // Create an ID that identifies the text node we're going to animate.
41        let animation_target_name = Name::new("Text");
42        let animation_target_id = AnimationTargetId::from_name(&animation_target_name);
43
44        // Allocate an animation clip.
45        let mut animation_clip = AnimationClip::default();
46
47        // Create a curve that animates font size.
48        animation_clip.add_curve_to_target(
49        
...
```

Example 4 (javascript):
```javascript
93    fn create(
94        animation_graphs: &mut Assets<AnimationGraph>,
95        animation_clips: &mut Assets<AnimationClip>,
96    ) -> AnimationInfo {
97        // Create an ID that identifies the text node we're going to animate.
98        let animation_target_name = Name::new("Cube");
99        let animation_target_id = AnimationTargetId::from_name(&animation_target_name);
100
101        // Allocate an animation clip.
102        let mut animation_clip = AnimationClip::default();
103
104        // Each leg of the translation motion should take 3 seconds.
105        let animation_domain =
...
```

---

## Module gltf_curves Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/gltf_curves/index.html

**Contents:**
- Module gltf_curves Copy item path
- Structs§
- Enums§

Concrete curve structures used to load glTF curves into the animation system.

---

## Function advance_animations Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/fn.advance_animations.html

**Contents:**
- Function advance_animations Copy item path

A system that advances the time for all playing animations.

**Examples:**

Example 1 (unknown):
```unknown
pub fn advance_animations(
    time: Res<'_, Time>,
    animation_clips: Res<'_, Assets<AnimationClip>>,
    animation_graphs: Res<'_, Assets<AnimationGraph>>,
    players: Query<'_, '_, (&mut AnimationPlayer, &AnimationGraphHandle)>,
)
```

---

## Crate animation Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/index.html

**Contents:**
- Crate animation Copy item path
- Modules§
- Macros§
- Structs§
- Enums§
- Statics§
- Traits§
- Functions§
- Type Aliases§
- Derive Macros§

Animation for the game engine Bevy

---

## Module graph Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/graph/index.html

**Contents:**
- Module graph Copy item path
- Structs§
- Enums§
- Type Aliases§

The animation graph, which allows animations to be blended together.

---

## Module animation_curves Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/animation_curves/index.html

**Contents:**
- Module animation_curves Copy item path
- §Overview
- §Making animation curves
  - §Animated Fields
  - §Animatable Properties
  - §Custom AnimationCurve and AnimationCurveEvaluator
- Structs§
- Enums§
- Traits§

The AnimationCurve trait and adaptors that allow curves to implement it.

The flow of curves into the animation system generally begins with something that implements the Curve trait. Let’s imagine, for example, that we have some Curve<Vec3> that we want to use to animate something. That could be defined in a number of different ways, but let’s imagine that we’ve defined it using a function:

Okay, so we have a curve, but the animation system also needs to know, in some way, how the values from this curve should actually be used. That is, it needs to know what to animate! That’s what AnimationCurve is for. In particular, what we need to do is take our curve and turn it into an AnimationCurve which will be usable by the animation system.

For instance, let’s imagine that we want to use the Vec3 output from our curve to animate the translation component of a Transform. For this, there is the adaptor AnimatableCurve, which wraps any Curve and AnimatableProperty and turns it into an AnimationCurve that will use the given curve to animate the entity’s property:

And finally, this AnimationCurve needs to be added to an AnimationClip in order to actually animate something. This is what that looks like:

The overview showed one example, but in general there are a few different ways of going from a Curve, which produces time-related data of some kind, to an AnimationCurve, which knows how to apply that data to an entity.

The animated_field macro (which returns an AnimatedField), in combination with AnimatableCurve is the easiest way to make an animation curve (see the example above).

This will select a field on a component and pass it to a Curve with a type that matches the field.

Animation of arbitrary aspects of entities can be accomplished using AnimatableProperty in conjunction with AnimatableCurve. See the documentation there for details.

This is the lowest-level option with the most control, but it is also the most complicated.

**Examples:**

Example 1 (javascript):
```javascript
let wobble_curve = FunctionCurve::new(
    Interval::UNIT,
    |t| { vec3(t.cos(), 0.0, 0.0) },
);
```

Example 2 (javascript):
```javascript
let wobble_animation = AnimatableCurve::new(animated_field!(Transform::translation), wobble_curve);
```

Example 3 (javascript):
```javascript
let mut animation_clip = AnimationClip::default();
animation_clip.add_curve_to_target(
    animation_target_id,
    wobble_animation,
);
```

---

## Struct AnimationTargetId Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/struct.AnimationTargetId.html

**Contents:**
- Struct AnimationTargetId Copy item path
- Tuple Fields§
- Implementations§
  - impl AnimationTargetId
    - pub fn from_names<'a>( names: impl Iterator<Item = &'a Name>, ) -> AnimationTargetId
      - Examples found in repository?
    - pub fn from_name(name: &Name) -> AnimationTargetId
      - Examples found in repository?
- Trait Implementations§
  - impl Clone for AnimationTargetId

A unique UUID for an animation target (e.g. bone in a skinned mesh).

The AnimationClip asset and the AnimationTarget component both use this to refer to targets (e.g. bones in a skinned mesh) to be animated.

When importing an armature or an animation clip, asset loaders typically use the full path name from the armature to the bone to generate these UUIDs. The ID is unique to the full path name and based only on the names. So, for example, any imported armature with a bone at the root named Hips will assign the same AnimationTargetId to its root bone. Likewise, any imported animation clip that animates a root bone named Hips will reference the same AnimationTargetId. Any animation is playable on any armature as long as the bone names match, which allows for easy animation retargeting.

Note that asset loaders generally use the full path name to generate the AnimationTargetId. Thus a bone named Chest directly connected to a bone named Hips will have a different ID from a bone named Chest that’s connected to a bone named Stomach.

Creates a new AnimationTargetId by hashing a list of names.

Typically, this will be the path from the animation root to the animation target (e.g. bone) that is to be animated.

Creates a new AnimationTargetId by hashing a single name.

Creates a new AnimationTargetId by hashing a list of strings.

Typically, this will be the path from the animation root to the animation target (e.g. bone) that is to be animated.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AnimationTargetId(pub Uuid);
```

Example 2 (javascript):
```javascript
340fn setup_animation_graph_once_loaded(
341    mut commands: Commands,
342    asset_server: Res<AssetServer>,
343    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
344    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
345    targets: Query<(Entity, &AnimationTarget)>,
346) {
347    for (entity, mut player) in &mut players {
348        // Load the animation clip from the glTF file.
349        let mut animation_graph = AnimationGraph::new();
350        let blend_node = animation_graph.add_additive_blend(1.0, animation_graph.root);
351
352        let animation
...
```

Example 3 (javascript):
```javascript
22fn setup(
23    mut commands: Commands,
24    mut meshes: ResMut<Assets<Mesh>>,
25    mut materials: ResMut<Assets<StandardMaterial>>,
26    mut animations: ResMut<Assets<AnimationClip>>,
27    mut graphs: ResMut<Assets<AnimationGraph>>,
28) {
29    // Camera
30    commands.spawn((
31        Camera3d::default(),
32        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
33    ));
34
35    // Light
36    commands.spawn((
37        PointLight {
38            intensity: 500_000.0,
39            ..default()
40        },
41        Transform::from_xyz(0.0, 2.5, 0.0),
42    ));

...
```

Example 4 (javascript):
```javascript
36    fn create(
37        animation_graphs: &mut Assets<AnimationGraph>,
38        animation_clips: &mut Assets<AnimationClip>,
39    ) -> AnimationInfo {
40        // Create an ID that identifies the text node we're going to animate.
41        let animation_target_name = Name::new("Text");
42        let animation_target_id = AnimationTargetId::from_name(&animation_target_name);
43
44        // Allocate an animation clip.
45        let mut animation_clip = AnimationClip::default();
46
47        // Create a curve that animates font size.
48        animation_clip.add_curve_to_target(
49        
...
```

---

## Static ANIMATION_TARGET_NAMESPACE Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/static.ANIMATION_TARGET_NAMESPACE.html

**Contents:**
- Static ANIMATION_TARGET_NAMESPACE Copy item path

The UUID namespace of animation targets (e.g. bones).

**Examples:**

Example 1 (unknown):
```unknown
pub static ANIMATION_TARGET_NAMESPACE: Uuid
```

---

## Module animatable Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/animatable/index.html

**Contents:**
- Module animatable Copy item path
- Structs§
- Traits§
- Functions§

Traits and type for interpolating between values.

---

## Enum RepeatAnimation Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/enum.RepeatAnimation.html

**Contents:**
- Enum RepeatAnimation Copy item path
- Variants§
  - Never
  - Count(u32)
  - Forever
- Trait Implementations§
  - impl Clone for RepeatAnimation
    - fn clone(&self) -> RepeatAnimation
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for RepeatAnimation

Repetition behavior of an animation.

The animation will finish after running once.

The animation will finish after running “n” times.

The animation will never finish.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum RepeatAnimation {
    Never,
    Count(u32),
    Forever,
}
```

---

## Function animate_targets Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/fn.animate_targets.html

**Contents:**
- Function animate_targets Copy item path

A system that modifies animation targets (e.g. bones in a skinned mesh) according to the currently-playing animations.

**Examples:**

Example 1 (unknown):
```unknown
pub fn animate_targets(
    par_commands: ParallelCommands<'_, '_>,
    clips: Res<'_, Assets<AnimationClip>>,
    graphs: Res<'_, Assets<AnimationGraph>>,
    threaded_animation_graphs: Res<'_, ThreadedAnimationGraphs>,
    players: Query<'_, '_, (&AnimationPlayer, &AnimationGraphHandle)>,
    targets: Query<'_, '_, (Entity, &AnimationTarget, EntityMutExcept<'_, '_, (AnimationTarget, AnimationPlayer, AnimationGraphHandle)>)>,
    animation_evaluation_state: Local<'_, ThreadLocal<RefCell<AnimationEvaluationState>>>,
)
```

---

## Module transition Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/transition/index.html

**Contents:**
- Module transition Copy item path
- Structs§
- Functions§

Animation transitions.

Please note that this is an unstable temporary API. It may be replaced by a state machine in the future.

---

## Macro animated_field Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/macro.animated_field.html

**Contents:**
- Macro animated_field Copy item path

Returns an AnimatedField with a given $component and $field.

This can be used in the following way:

**Examples:**

Example 1 (javascript):
```javascript
macro_rules! animated_field {
    ($component:ident::$field:tt) => { ... };
}
```

Example 2 (javascript):
```javascript
#[derive(Component, Reflect)]
struct Transform {
    translation: Vec3,
}

let field = animated_field!(Transform::translation);

#[derive(Component, Reflect)]
struct Color(Srgba);

let tuple_field = animated_field!(Color::0);
```

---

## Struct VariableCurve Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/struct.VariableCurve.html

**Contents:**
- Struct VariableCurve Copy item path
- Tuple Fields§
- Implementations§
  - impl VariableCurve
    - pub fn new(animation_curve: impl AnimationCurve) -> VariableCurve
- Trait Implementations§
  - impl Clone for VariableCurve
    - fn clone(&self) -> VariableCurve
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for VariableCurve

Contains an animation curve which is used to animate a property of an entity.

Create a new VariableCurve from an animation curve.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct VariableCurve(pub Box<dyn AnimationCurve>);
```

---

## Type Alias AnimationCurves Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/animation/type.AnimationCurves.html

**Contents:**
- Type Alias AnimationCurves Copy item path
- Aliased Type§

A mapping from AnimationTargetId (e.g. bone in a skinned mesh) to the animation curves.

**Examples:**

Example 1 (unknown):
```unknown
pub type AnimationCurves = HashMap<AnimationTargetId, Vec<VariableCurve>, NoOpHash>;
```

Example 2 (unknown):
```unknown
pub struct AnimationCurves(/* private fields */);
```

---
