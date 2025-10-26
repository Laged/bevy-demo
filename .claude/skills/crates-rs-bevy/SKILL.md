---
name: crates-rs-bevy
description: Bevy game engine (v0.17.2) - comprehensive reference for Rust game development using Bevy's ECS architecture, rendering pipeline, input systems, and game engine APIs.
---

# Bevy Game Engine (v0.17.2) Skill

Your complete reference for building games and interactive applications with Bevy, a data-driven game engine for Rust.

## When to Use This Skill

Use this skill when:
- **Building games** with Bevy (2D or 3D)
- **Implementing game systems** (physics, input, UI, animation)
- **Working with ECS** (Entity Component System architecture)
- **Setting up rendering pipelines** (cameras, materials, shaders)
- **Handling input** (keyboard, mouse, gamepad)
- **Managing game state** and world organization
- **Loading and managing assets** (textures, models, audio)
- **Creating UI** interfaces and menus
- **Optimizing performance** of game code

## Quick Reference

### 1. Basic App Setup
```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Sprite::default(),
        Transform::default(),
        Velocity { x: 1.0, y: 0.0 },
    ));
}

fn movement(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

#[derive(Component)]
struct Velocity { x: f32, y: f32 }
```

### 2. ECS Fundamentals
- **Entity**: A unique object in the game world
- **Component**: Data attached to entities (position, health, sprite, etc.)
- **System**: Logic that reads/writes components and runs each frame
- **Query**: Safe way to filter and access specific components
- **Resource**: Global state shared across systems

### 3. Common Patterns

**Spawning an Entity:**
```rust
commands.spawn((
    Sprite::default(),
    Transform::from_xyz(0.0, 0.0, 0.0),
    Health { value: 100 },
));
```

**Querying Components:**
```rust
fn update_enemies(
    mut query: Query<(&mut Transform, &Health), With<Enemy>>
) {
    for (mut transform, health) in &mut query {
        if health.value <= 0 {
            // Handle death
        }
    }
}
```

**Using Resources:**
```rust
#[derive(Resource)]
struct GameState {
    score: u32,
    level: u32,
}

fn increment_score(mut game_state: ResMut<GameState>) {
    game_state.score += 10;
}
```

**Input Handling:**
```rust
fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if keyboard.pressed(KeyCode::ArrowRight) {
        // Move right
    }
    if mouse.just_pressed(MouseButton::Left) {
        // Handle click
    }
}
```

### 4. Key Bevy Concepts

| Concept | Purpose |
|---------|---------|
| **Plugin** | Reusable bundle of systems and resources |
| **Schedule** | Determines when systems run (Startup, Update, etc.) |
| **Stage** | Groups of systems that run sequentially |
| **Bundle** | Template for spawning related components together |
| **Asset** | External data (textures, models, audio, fonts) |
| **Material** | Defines how a mesh should be rendered |
| **Camera** | Viewport into the game world |
| **Transform** | Position, rotation, scale in 3D space |

## Major Reference Sections

This skill includes **2000 pages** of comprehensive documentation organized into 20 categories:

### Core Systems
- **quickstart.md** - Getting started, basic setup, tutorials (1.7MB)
- **core.md** - App lifecycle, plugins, scheduling (48KB)
- **api_reference.md** - Complete API index (280KB)

### Game Architecture
- **ecs.md** - Entity Component System deep dive (256KB, 7.6K lines)
- **hierarchy.md** - Parent-child entity relationships
- **state.md** - Game state management

### Graphics & Rendering
- **rendering.md** - Full rendering pipeline (500KB, 15.5K lines)
- **math.md** - Vectors, transforms, quaternions (276KB, 10K lines)
- **asset.md** - Asset loading and management (148KB)

### Gameplay
- **input.md** - Keyboard, mouse, gamepad input (40KB)
- **animation.md** - Skeletal and curve-based animation (28KB)
- **window.md** - Window management and display settings (60KB)
- **timing.md** - Delta time, timers, time scaling
- **text.md** - Text rendering and fonts (20KB)

### User Interface
- **ui.md** - UI framework and widgets (4KB)

### Audio & Media
- **audio.md** - Sound and music systems (28KB)

### Development Tools
- **diagnostics.md** - Profiling and performance metrics (20KB)
- **utils.md** - Utility types and helpers (32KB)

### Data & Scenes
- **scene.md** - Scene serialization and loading (8KB)
- **other.md** - Miscellaneous modules (36KB)

## Navigation Guide

### I'm a Beginner
1. Start with **quickstart.md** for basic tutorials
2. Read the "Quick Reference" section above
3. Explore **core.md** to understand plugins and scheduling
4. Follow the example code patterns

### I Need Specific Features
1. Check the "Major Reference Sections" table to find the relevant category
2. Use `view references/[category].md` to read detailed documentation
3. Look for code examples and implementation patterns
4. Check the **api_reference.md** for complete type definitions

### I'm Debugging an Issue
1. Check **core.md** for system execution order
2. Review **ecs.md** if it's component/query related
3. Check **rendering.md** for display issues
4. See **diagnostics.md** for performance profiling

### I Want Best Practices
1. Read **ecs.md** for proper entity/component architecture
2. Check **core.md** for system organization
3. Review **rendering.md** for performance tips
4. Explore examples in **quickstart.md**

## Common Tasks

### Create a Simple 2D Game
```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .add_systems(Update, spawn_enemies)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Sprite::from_image(asset_server.load("player.png")),
        Transform::default(),
        Player,
    ));
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut();
    if keyboard.pressed(KeyCode::ArrowRight) {
        transform.translation.x += 5.0;
    }
}

#[derive(Component)]
struct Player;
```

### Handle Events
```rust
fn handle_collision(
    mut collision_reader: EventReader<CollisionEvent>,
) {
    for event in collision_reader.read() {
        println!("Collision: {:?}", event);
    }
}
```

### Load Assets
```rust
#[derive(Resource)]
struct GameAssets {
    player_texture: Handle<Image>,
    player_sound: Handle<AudioSource>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let assets = GameAssets {
        player_texture: asset_server.load("textures/player.png"),
        player_sound: asset_server.load("sounds/jump.mp3"),
    };
    commands.insert_resource(assets);
}
```

## Performance Tips

1. **Use Queries Efficiently** - Filter with `With<T>` and `Without<T>` to avoid unnecessary data
2. **Batch Operations** - Group similar operations together
3. **Profile with Diagnostics** - Use `diagnostics.md` tools to identify bottlenecks
4. **Cache Expensive Computations** - Store results in components or resources
5. **Use Systems Correctly** - Understand schedule ordering from `core.md`

## Key Resources

- **Official Bevy Book**: https://bevyengine.org/learn/book/
- **Bevy GitHub**: https://github.com/bevyengine/bevy
- **Bevy Discord**: Community support and discussions
- **This Skill**: Complete API reference from docs.rs v0.17.2

## Quick Lookup

**Finding a Type:** Check `api_reference.md` for alphabetical listing

**Finding a System:** Check `core.md` for built-in systems; search `quickstart.md` for examples

**Finding an Example:** Browse `quickstart.md` (1.7MB of examples)

## Summary

Bevy is a powerful, data-driven game engine using ECS architecture for flexible and efficient game development. This skill provides:

✅ **2000+ pages** of official documentation
✅ **20 reference categories** covering all major systems
✅ **Complete API reference** for all public types
✅ **Code examples** from official Bevy repository
✅ **Best practices** and common patterns

---

**Bevy Version:** 0.17.2
**Documentation Source:** docs.rs
**Skill Status:** Production Ready ✅
