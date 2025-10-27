# Bevy 0.17 Features Reference

> **For Star Streams:** This guide prioritizes **Headless Widgets**, **ViewportNode**, and **Tilemap Chunks** as high-synergy features for your auto-battler architecture. All examples show integration with the headless testing infrastructure.

## Quick Reference Matrix

| Priority | Feature | Category | Key APIs | Status | See Section |
|----------|---------|----------|----------|--------|-------------|
| 🔴 **HIGH** | **Headless Widgets** | UI | `Button`, `Slider`, `Checkbox` | Experimental | UI - Headless Widgets |
| 🔴 **HIGH** | **ViewportNode** | UI | `ViewportNode` camera rendering | Stable | UI - ViewportNode |
| 🔴 **HIGH** | **Tilemap Chunk Rendering** | Assets | `TilemapChunk` component | Stable | Assets - Tileamps |
| 🟡 **MEDIUM** | Event/Observer Overhaul | ECS | `Event`, `Trigger`, `On` | Stable | ECS - Events/Observers |
| 🟡 **MEDIUM** | Component Lifecycle Events | ECS | `On::<Add>`, `On::<Remove>` | Stable | ECS - Lifecycle Events |
| 🟡 **MEDIUM** | UI Gradients | UI | `BackgroundGradient`, `BorderGradient` | Stable | UI - Gradients |
| 🟡 **MEDIUM** | Val Helper Functions | UI | `px()`, `percent()`, `vw()` | Stable | UI - Val Helpers |
| 🟡 **MEDIUM** | Component Propagation | ECS | `Propagate<C>`, `HierarchyPropagatePlugin` | Stable | ECS - Propagation |
| ⚪ **LOW** | Bevy Solari: Raytraced Lighting | Rendering | `EnvironmentMapLight` | Experimental | Rendering - Solari |
| ⚪ **LOW** | DLSS Support | Rendering | `Dlss` component | Stable | Rendering - DLSS |
| ⚪ **LOW** | Realtime Environment Maps | Rendering | `EnvironmentMapPlugin` | Stable | Rendering - Env Maps |
| ⚪ **LOW** | Text2d Drop Shadows | UI | `TextShadow` component | Stable | UI - Text Effects |
| ⚪ **LOW** | Reflect Auto-Registration | Serialization | `#[derive(Reflect)]` | Stable | Reflection - Auto-Reg |
| ⚪ **LOW** | Hot Patching Systems | Development | `hotpatching` feature | Experimental | Development - Hot Patching |

---

## HIGH PRIORITY: Features with Headless Testing Synergy

### UI System - Headless Widgets (Experimental)

**What it is:** Unstyled, standardized widget primitives (`Button`, `Slider`, `Checkbox`, `RadioButton`, `Scrollbar`) with observer integration, designed for custom UI frameworks and editor tooling.

**Why it matters for Star Streams:**
- **Headless testable:** Widgets work in headless mode with no rendering overhead
- **Menu system:** Perfect foundation for pause menu, settings, game-over screens
- **Minimal dependencies:** Separate from visual styling, making tests fast
- **Observer-native:** Integrates seamlessly with Bevy's new event system

**Architecture & Motivation:**
Separates widget behavior from styling, enabling custom theming and reducing boilerplate for widget-heavy applications. The event-based design makes widgets testable without rendering.

**Key Components:**
- `Button` - Interactive button primitive (no styling)
- `Slider` - Range selection with value change events
- `Checkbox` / `RadioButton` - Boolean/enum selection
- `Scrollbar` - Scroll area navigation
- `Hovered`, `Pressed`, `Checked` - State tracking components

**Key Events:**
- `Activate` - Widget interaction (button press, checkbox toggle)
- `ValueChange` - Slider/selection changes with new value

**Code Pattern - Basic Button:**
```rust
commands.spawn((
    Button::default(),
    Node {
        width: Val::px(100.0),
        height: Val::px(40.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
)).observe(|trigger: In<Activate>, mut commands: Commands| {
    println!("Button pressed!");
    commands.trigger(PlayerPausedEvent);
});
```

**Code Pattern - Interactive Slider:**
```rust
commands.spawn((
    Slider::new(0.0..=1.0, 0.5),
    Node {
        width: Val::px(200.0),
        height: Val::px(20.0),
        ..default()
    },
)).observe(|trigger: In<ValueChange>, mut query: Query<&Slider>| {
    if let Ok(slider) = query.get(trigger.entity()) {
        println!("Volume: {}", slider.value);
    }
});
```

**Headless Testing Pattern:**
```rust
use hell_game::test_utils::*;

#[test]
fn test_pause_menu_button() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    init_for_testing(&mut app, &config);

    // Spawn pause menu button (no rendering needed)
    let mut commands = app.world_mut().commands();
    let button_entity = commands.spawn(Button::default()).id();

    // Simulate click by triggering Activate event
    app.world_mut().trigger_targets(Activate, [button_entity]);
    app.update();

    // Verify pause state changed
    let game_state = app.world().resource::<State<GameState>>();
    assert_eq!(*game_state.get(), GameState::Paused);
}

#[test]
fn test_slider_value_change() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    init_for_testing(&mut app, &config);

    // Spawn volume slider (headless - no visual render)
    let mut commands = app.world_mut().commands();
    let slider_entity = commands.spawn((
        Slider::new(0.0..=1.0, 0.5),
    )).id();

    // Trigger value change
    app.world_mut().trigger_targets(ValueChange { value: 0.8 }, [slider_entity]);
    app.update();

    // Verify settings updated
    let settings = app.world().resource::<GameSettings>();
    assert_eq!(settings.volume, 0.8);
}
```

**Integration for Star Streams:**

| Use Case | Widget | Benefits | Headless Test |
|----------|--------|----------|-----------------|
| **Pause Menu** | Button + hierarchy | Clean state transitions | Trigger Activate, check GameState |
| **Settings Panel** | Slider | Volume, difficulty, particle count | ValueChange event, check resources |
| **Enemy Wave Select** | RadioButton | Choose starting wave (future expansion) | Trigger Activate per option |
| **Difficulty Selector** | Checkbox grid | Select enemy types to enable | Trigger per checkbox, verify config |

**See also:** crates-rs-bevy `bevy::ui`

---

### UI System - ViewportNode: Render Cameras to UI (Stable)

**What it is:** `ViewportNode` component renders a camera's output directly into a UI node, with support for picking through the rendered content.

**Why it matters for Star Streams:**
- **Minimap system:** Render zoomed-out world view into UI corner for tactical overview
- **Enemy preview panel:** Show upcoming wave composition with rendered sprites
- **Headless compatible:** Camera setup works in headless mode (no rendering, just data)
- **Tactical depth:** Enables player to see arena state before engaging
- **Synergy:** Works perfectly with headless testing - test camera logic, render visuals

**Motivation:**
Previously, showing a preview required duplicate rendering logic or hacky workarounds. ViewportNode enables clean, performant embedded camera views.

**Key APIs:**
- `ViewportNode { camera_entity }` - Specify which camera to render
- Picking works through rendered viewport
- Supports any camera type (2D, 3D, orthographic)

**Code Pattern - Minimap:**
```rust
// Create minimap camera
let minimap_camera = commands.spawn((
    Camera2d {
        // Zoomed out view
        ..default()
    },
    Transform::from_translation(Vec3::new(0.0, 0.0, 100.0))
        .with_scale(Vec3::splat(0.2)), // Zoom out 5x
)).id();

// Create UI node showing minimap
commands.spawn((
    Node {
        position_type: PositionType::Absolute,
        right: Val::px(10.0),
        bottom: Val::px(10.0),
        width: Val::px(200.0),
        height: Val::px(200.0),
        border: UiRect::all(Val::px(2.0)),
        ..default()
    },
    BorderColor(Color::srgb(0.8, 0.8, 0.8)),
    ViewportNode {
        camera_entity: minimap_camera,
    },
));
```

**Code Pattern - Enemy Wave Preview:**
```rust
// Create preview camera (renders enemy wave spawning positions)
let preview_camera = commands.spawn((
    Camera2d::default(),
    Transform::from_translation(Vec3::new(0.0, 0.0, 50.0)),
)).id();

// UI panel showing next wave
commands.spawn((
    Node {
        position_type: PositionType::Absolute,
        left: Val::px(10.0),
        top: Val::px(10.0),
        width: Val::px(300.0),
        height: Val::px(150.0),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::px(10.0)),
        ..default()
    },
    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
    ViewportNode {
        camera_entity: preview_camera,
    },
)).with_child((
    Text::new("Next Wave"),
    TextFont::from_font_size(24.0),
));
```

**Headless Testing Pattern:**
```rust
#[test]
fn test_minimap_camera_setup() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    init_for_testing(&mut app, &config);

    // Spawn minimap camera (works in headless - just entity setup)
    let mut commands = app.world_mut().commands();
    let minimap_camera = commands.spawn(Camera2d::default()).id();

    // Spawn minimap UI node
    let minimap_ui = commands.spawn(ViewportNode {
        camera_entity: minimap_camera,
    }).id();

    app.update();

    // Verify camera and UI node exist
    assert!(app.world().entities().contains(minimap_camera));
    assert!(app.world().entities().contains(minimap_ui));
}

#[test]
fn test_wave_preview_rendering() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    init_for_testing(&mut app, &config);

    // Setup preview camera
    let mut commands = app.world_mut().commands();
    let preview_cam = commands.spawn(Camera2d::default()).id();

    // Spawn enemies that will appear in preview
    let enemies = spawn_test_enemies(&mut commands, 5, 300.0, &config);

    // Create viewport node
    commands.spawn(ViewportNode {
        camera_entity: preview_cam,
    });

    app.update();

    // Verify enemy positions are correct for preview
    let enemy_query = app.world().query::<&Transform>();
    assert_eq!(enemy_query.iter(app.world()).count(), 5);
}
```

**Integration for Star Streams:**

| System | Purpose | Camera Setup | Headless Benefit |
|--------|---------|--------------|------------------|
| **Minimap** | Real-time arena overview | 5x zoomed out from player | Test camera culling, positioning |
| **Wave Preview** | Show next enemy wave | Orthographic view at spawn positions | Verify enemy spawn patterns |
| **Tactical Display** | Alternative camera angle | Side view showing height differences | Test multiple concurrent cameras |
| **Replay Viewer** | Show past game segment | Playback of recorded positions | Bench camera switch performance |

**See also:** crates-rs-bevy `bevy::ui`

---

### Assets - Tilemap Chunk Rendering (Stable)

**What it is:** Performant tilemap rendering via `TilemapChunk` component supporting tile indexing, per-tile visibility, color tinting, and automatic batching.

**Why it matters for Star Streams:**
- **Future grid expansion:** If expanding to hex/grid-based arena, chunks provide efficient rendering
- **Headless testable:** Tile logic (placement, removal, tinting) works in headless mode
- **Performance baseline:** Benchmark chunk rendering impact via headless tests
- **Environmental storytelling:** Use tilemap for decorative arena background
- **Synergy:** Test tile logic in headless, render visuals in standard mode

**Motivation:**
2D grid-based games need efficient tilemap systems; chunking reduces draw calls and enables large maps without performance penalties.

**Key APIs:**
- `TilemapChunk { size, tiles }` - Define tilemap chunk
- Per-tile visibility and color tinting
- Automatic batching for performance
- Index-based tile selection

**Code Pattern - Basic Tilemap:**
```rust
// Create 16x16 tile chunk (e.g., stone floor)
let mut tiles = vec![0; 16 * 16]; // All tiles index 0

commands.spawn((
    TilemapChunk {
        size: UVec2::new(16, 16),
        tiles,
    },
    Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)), // Behind entities
    GlobalTransform::default(),
));
```

**Code Pattern - Destructible Tiles:**
```rust
// Track tile damage
#[derive(Component)]
struct TileDamage {
    damage: [f32; 256], // For 16x16 chunk
}

fn handle_bullet_hit_tile(
    mut bullets: RemovedComponents<Bullet>,
    mut chunks: Query<(&Transform, &mut TilemapChunk, &mut TileDamage)>,
) {
    for bullet_entity in bullets.read() {
        // Find tile at bullet position
        if let Some((chunk_transform, mut chunk, mut damage)) = chunks.iter_mut().next() {
            let local_pos = bullet_pos - chunk_transform.translation.xy();
            let tile_x = (local_pos.x / TILE_SIZE) as usize;
            let tile_y = (local_pos.y / TILE_SIZE) as usize;

            if tile_x < 16 && tile_y < 16 {
                let tile_idx = tile_y * 16 + tile_x;
                damage.damage[tile_idx] += 10.0;

                if damage.damage[tile_idx] > 100.0 {
                    chunk.tiles[tile_idx] = 1; // Damaged tile sprite
                }
            }
        }
    }
}
```

**Code Pattern - Tinted Arena Zones:**
```rust
// Create arena floor with colored regions
#[derive(Component)]
struct ArenaFloor;

fn setup_arena_floor(mut commands: Commands) {
    // Safe zone (green)
    commands.spawn((
        TilemapChunk {
            size: UVec2::new(16, 16),
            tiles: vec![0; 256], // Green floor tile
        },
        Transform::from_translation(Vec3::new(-200.0, 0.0, -1.0)),
        GlobalTransform::default(),
        ArenaFloor,
    ));

    // Danger zone (red)
    commands.spawn((
        TilemapChunk {
            size: UVec2::new(16, 16),
            tiles: vec![1; 256], // Red floor tile
        },
        Transform::from_translation(Vec3::new(200.0, 0.0, -1.0)),
        GlobalTransform::default(),
        ArenaFloor,
    ));
}
```

**Headless Testing Pattern:**
```rust
#[test]
fn test_tilemap_chunk_creation() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    init_for_testing(&mut app, &config);

    // Spawn tilemap chunk (works in headless - no rendering)
    let mut commands = app.world_mut().commands();
    let tiles = vec![0; 16 * 16];
    commands.spawn(TilemapChunk {
        size: UVec2::new(16, 16),
        tiles,
    });

    app.update();

    // Verify chunk exists and is accessible
    let chunk_count = app.world()
        .query::<&TilemapChunk>()
        .iter(app.world())
        .count();
    assert_eq!(chunk_count, 1);
}

#[test]
fn test_tile_damage_system() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    init_for_testing(&mut app, &config);

    // Spawn destructible tilemap
    let mut commands = app.world_mut().commands();
    commands.spawn((
        TilemapChunk {
            size: UVec2::new(16, 16),
            tiles: vec![0; 256], // All intact
        },
        TileDamage {
            damage: [0.0; 256],
        },
    ));

    // Simulate multiple bullet hits on same tile
    for _ in 0..12 {
        let mut chunk_query = app.world_mut().query::<&mut TileDamage>();
        for mut damage in chunk_query.iter_mut(app.world_mut()) {
            damage.damage[0] += 10.0;
        }
    }

    // Verify tile is destroyed when damage exceeds threshold
    let tile_damage = app.world()
        .query::<&TileDamage>()
        .iter(app.world())
        .next()
        .unwrap();

    assert!(tile_damage.damage[0] >= 100.0);
}

#[test]
fn test_tilemap_performance_baseline() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    init_for_testing(&mut app, &config);

    // Spawn multiple tilemap chunks (benchmark in headless)
    let mut commands = app.world_mut().commands();
    for x in 0..10 {
        for y in 0..10 {
            commands.spawn((
                TilemapChunk {
                    size: UVec2::new(16, 16),
                    tiles: vec![0; 256],
                },
                Transform::from_translation(Vec3::new(
                    x as f32 * 256.0,
                    y as f32 * 256.0,
                    -1.0,
                )),
            ));
        }
    }

    // Run 60 frames and measure performance
    let start = std::time::Instant::now();
    run_frames(&mut app, 60);
    let elapsed = start.elapsed();

    println!("100 chunks, 60 frames: {:?}", elapsed);
    // Headless should complete in <100ms
    assert!(elapsed.as_millis() < 100);
}
```

**Integration for Star Streams:**

| Feature | Current | Future Expansion | Headless Testing |
|---------|---------|------------------|------------------|
| **Arena Background** | Free-form sprites | Tilemap layers | Test tile placement logic |
| **Destructible Terrain** | Not implemented | Tilemap chunks with damage | Bench damage calculation |
| **Grid Expansion** | No grid | Hex/square grid arena | Test tile-to-entity mapping |
| **Environmental Zones** | None | Colored danger zones | Verify zone boundaries |

**See also:** crates-rs-bevy `bevy::sprite`

---

## MEDIUM PRIORITY: ECS & Core Features

### ECS - Event/Observer Overhaul (Stable)

**What it is:** Complete redesign of Bevy's event system with:
- `Event` trait with associated `Trigger` implementations
- `EntityEvent` trait for entity-targeted events
- Unified `world.trigger()` API
- `PropagateEntityTrigger` for event bubbling through hierarchies

**Why it matters:** Decouples event producers from consumers, enabling flexible reactive systems and cleaner headless testing.

**Before/After (0.16 → 0.17):**
```rust
// 0.16: EventReader pattern
fn handle_collision(mut collision_events: EventReader<CollisionEvent>) {
    for event in collision_events.read() {
        // Process event
    }
}

// 0.17: Observer pattern
fn handle_collision(trigger: Trigger<CollisionEvent>) {
    // Automatically called when event fires
    // Cleaner, more composable
}
```

**Key Components:**
- `#[derive(Event)]` - Derive macro for events
- `Trigger<E>` - Observer trigger in systems
- `On::<E>` - Lifecycle-based triggering (e.g., `On::<Add>`)
- `PropagateEntityTrigger` - Bubble events up entity hierarchy

**Code Pattern - Enemy Spawn Reaction:**
```rust
#[derive(Event)]
struct EnemySpawned { enemy_id: Entity }

fn react_to_enemy_spawn(trigger: Trigger<EnemySpawned>, mut commands: Commands) {
    let enemy_id = trigger.event().enemy_id;
    // Play spawn animation, sound effect
    commands.entity(enemy_id).insert(AnimationState::Idle);
}

// Trigger the event
fn spawn_enemies(mut commands: Commands, mut world: &mut World) {
    let enemy = commands.spawn(/* ... */).id();
    world.trigger_targets(EnemySpawned { enemy_id: enemy }, [enemy]);
}
```

**Headless Testing Pattern:**
```rust
#[test]
fn test_enemy_spawn_event() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    init_for_testing(&mut app, &config);

    // Trigger enemy spawn event
    let mut commands = app.world_mut().commands();
    let enemy_id = commands.spawn(Enemy {
        health: config.enemy.health,
    }).id();

    app.world_mut().trigger_targets(EnemySpawned { enemy_id }, [enemy_id]);
    app.update();

    // Verify reaction occurred (animation state changed)
    let anim_state = app.world()
        .query::<&AnimationState>()
        .iter(app.world())
        .next();

    assert!(anim_state.is_some());
}
```

**See also:** crates-rs-bevy `bevy::ecs::event`

---

### ECS - Component Lifecycle Events (Stable)

**What it is:** Renamed lifecycle events (`Add`, `Remove`, `Insert`, `Delete`) triggered as `EntityEvent`s when components are added/removed/modified on entities.

**Motivation:** Replace manual `Changed<T>` queries with reactive, hierarchical event handling.

**Key Events:**
- `On::<Add>` - Component added to entity
- `On::<Remove>` - Component removed
- `On::<Insert>` - Component inserted (with previous value)
- `On::<Delete>` - Component deleted

**Code Pattern - HUD Update on Health Change:**
```rust
fn on_player_health_change(
    trigger: Trigger<On<Insert>>,
    mut query: Query<&Health, With<Player>>,
    mut hud_query: Query<&mut Text, With<HealthDisplay>>,
) {
    if let Ok(health) = query.get(trigger.entity()) {
        if let Ok(mut text) = hud_query.single_mut() {
            text.sections[0].value = format!("HP: {:.0}", health.0);
        }
    }
}
```

**See also:** crates-rs-bevy `bevy::ecs::event`

---

### ECS - Component Propagation (Stable)

**What it is:** `HierarchyPropagatePlugin` for automatic tree-wide component synchronization using `Propagate<C>`, `PropagateStop<C>`, `PropagateOver<C>` markers.

**Motivation:** Reduces boilerplate for components that should cascade down entity hierarchies.

**Key Components:**
- `Propagate<C>` - Cascade C to all descendants
- `PropagateStop<C>` - Stop propagation at this entity
- `PropagateOver<C>` - Custom propagation logic

**Code Pattern - Enemy Damage Flash:**
```rust
// When enemy takes damage, flash all child sprites the same color
commands.entity(enemy).insert((
    Tint::RED,
    Propagate::<Tint>::default(),
)); // All children auto-tint
```

**See also:** crates-rs-bevy `bevy::hierarchy`

---

### UI - Gradients (Stable)

**What it is:** `BackgroundGradient` and `BorderGradient` components supporting linear, conic, and radial gradient types with color stops.

**Code Pattern - Health Bar Gradient:**
```rust
commands.spawn(Node {
    width: Val::px(200.0),
    height: Val::px(20.0),
    ..default()
}).insert(BackgroundGradient::linear(
    Rect::new(0.0, 0.0, 1.0, 0.0),
    vec![Color::srgb(0.0, 1.0, 0.0), Color::srgb(1.0, 0.0, 0.0)], // Green to red
));
```

**See also:** crates-rs-bevy `bevy::ui`

---

### UI - Val Helper Functions (Stable)

**What it is:** Shorthand constructors (`px()`, `percent()`, `vw()`, `vh()`, `vmin()`, `vmax()`) and fluent `UiRect` builders.

**Before/After:**
```rust
// Before 0.17
width: Val::Px(100.0)
padding: UiRect {
    left: Val::Px(10.0),
    right: Val::Px(10.0),
    top: Val::Px(5.0),
    bottom: Val::Px(5.0),
}

// After 0.17
width: px(100.0)
padding: UiRect::all(px(5.0)).with_left(px(10.0)).with_right(px(10.0))
```

**See also:** crates-rs-bevy `bevy::ui::Val`

---

## LOW PRIORITY: Rendering & Advanced Features

### Rendering - Bevy Solari: Raytraced Lighting (Experimental)

**What it is:** Real-time GPU-accelerated raytracing for physically-accurate direct and indirect lighting.

**Integration for Star Streams:**
- **Current:** Not applicable (2D auto-battler)
- **Future:** Test on high-end PC for advanced lighting effects
- **See also:** crates-rs-bevy `bevy::pbr`

---

### Rendering - DLSS Support (Stable)

**What it is:** NVIDIA-exclusive anti-aliasing + upscaling technology.

**Integration for Star Streams:**
- **Applies:** If targeting high-end PC with RTX GPUs
- **Benefit:** Smoother performance for intensive scenes
- **See also:** crates-rs-bevy `bevy::render::camera`

---

### UI - Text Effects (Stable)

**What it is:** `TextShadow` component for drop shadows and `TextBackgroundColor` for span backgrounds.

**Code Pattern:**
```rust
commands.spawn((
    Text2d::new("Score: 1000"),
    TextShadow {
        offset: Vec2::new(2.0, 2.0),
        color: Color::BLACK,
    },
));
```

**See also:** crates-rs-bevy `bevy::text`

---

### Serialization - Reflect Auto-Registration (Stable)

**What it is:** Types with `#[derive(Reflect)]` automatically registered; eliminates manual `app.register_type::<T>()` boilerplate.

**Integration for Star Streams:**
- **Components:** `#[derive(Reflect)]` on `Enemy`, `Player`, `Health` - automatic registration
- **Benefit:** Easier serialization/deserialization for save systems
- **See also:** crates-rs-bevy `bevy::reflect`

---

### Development - Hot Patching Systems (Experimental)

**What it is:** Rust system reloading via dynamic compilation (<1 second reload).

**Setup:**
1. Add `hotpatching` feature to Cargo.toml
2. Install: `cargo install bevy_dx`
3. Run: `bevy_dx watch`
4. Make changes to system code → automatic reload

**Integration for Star Streams:**
- **Rapid iteration:** Tweak enemy AI, particle effects without restart
- **Limitations:** Only systems; component/resource changes require restart
- **Setup:** `cargo install bevy_dx && bevy_dx watch`
- **See also:** crates-rs-bevy

---

## Integration Checklist for Star Streams

### Phase 1: Headless Testing Refactor (In Progress)
- [ ] Complete headless testing infrastructure (from implementation plan)
- [ ] Verify all plugins support `PluginMode::Headless`
- [ ] Test utilities working (`run_frames`, `spawn_test_enemies`, etc.)

### Phase 2: Headless Widgets Integration (Next Priority)
- [ ] Implement pause menu with `Button` widgets
- [ ] Add settings panel with `Slider` for volume/difficulty
- [ ] Create wave selector with `RadioButton`/`Checkbox`
- [ ] Write headless tests for all widget interactions
- [ ] Implement visual styling for widgets in standard mode

### Phase 3: ViewportNode Integration
- [ ] Implement minimap camera tracking player
- [ ] Render minimap to UI corner via `ViewportNode`
- [ ] Test camera positioning in headless mode
- [ ] Add wave preview panel with `ViewportNode`
- [ ] Benchmark multiple concurrent viewport nodes

### Phase 4: Tilemap Expansion (Optional)
- [ ] Design arena tilemap layout
- [ ] Implement `TilemapChunk` background layer
- [ ] Create destructible tile system
- [ ] Benchmark tilemap rendering performance via headless tests
- [ ] Add visual feedback for tile destruction

### Phase 5: Event System Modernization
- [ ] Refactor collision events to use `Trigger<CollisionEvent>`
- [ ] Replace `EventReader` loops with `Trigger` observers
- [ ] Use `On::<Add>` for component lifecycle reactions
- [ ] Simplify health/damage event handling

### Phase 6: UI Polish
- [ ] Add gradient backgrounds to health bars
- [ ] Use `Val` helpers to clean up HUD layout code
- [ ] Add text shadows to score/damage numbers
- [ ] Ensure responsive UI across window sizes

---

## Testing Strategy for New Features

**Headless Testing Workflow:**

```rust
// 1. Create headless app
let mut app = create_headless_app(config.clone());

// 2. Initialize to InGame state
init_for_testing(&mut app, &config);

// 3. Spawn entities needed for test
let mut commands = app.world_mut().commands();
let enemies = spawn_test_enemies(&mut commands, 10, 500.0, &config);

// 4. Trigger events or run simulation
app.world_mut().trigger_targets(EnemySpawned { enemy_id: enemies[0] }, [enemies[0]]);
run_frames(&mut app, 60);

// 5. Assert expected outcomes
let query = app.world().query::<&Enemy>();
assert_eq!(query.iter(app.world()).count(), expected_count);
```

**For each new feature, ask:**
1. ✅ Can it be tested in headless mode?
2. ✅ Does it integrate with observer pattern?
3. ✅ Can it be benchmarked via `run_frames`?
4. ✅ Do visual components work in standard mode?

---

## References

- **Official Blog:** https://bevy.org/news/bevy-0-17/
- **Bevy Examples:** https://github.com/bevyengine/bevy/tree/latest/examples
- **API Reference:** Use crates-rs-bevy skill for detailed API documentation
- **Migration Guide:** https://bevyengine.org/learn/migration-guides/

---

## Notes for Future Updates

This document synthesizes Bevy 0.17 features with focus on Star Streams development priorities:
1. Headless widgets, viewport node, tilemap chunks for high synergy
2. ECS events/lifecycle for reactive systems
3. UI polish features for menu/HUD
4. Advanced rendering reserved for future expansion

Last updated: 2025-10-27 (post-brainstorming session)
