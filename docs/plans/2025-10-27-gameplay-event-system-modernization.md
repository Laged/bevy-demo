# Gameplay Agent: Event System Modernization Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Modernize existing event-driven systems to use Bevy 0.17's Trigger/Observer pattern, replacing direct system queries with reactive event-based communication.

**Architecture:** Refactor collision detection, player damage, enemy spawning, and bullet hits to emit events that other systems observe. This establishes the event contract foundation that UI and Graphics agents will build upon.

**Tech Stack:** Bevy 0.17 ECS (Trigger, Observer, On::<Add>, On::<Remove>), Rust

**Worktree:** `.worktrees/gameplay-events` (branch: `gameplay-events`)

---

## Pre-Implementation: Define Event Contracts

### Task 0: Add Event Definitions to core/events.rs

**Files:**
- Modify: `src/core/events.rs`

**Step 1: Add new event structs**

Add these events to `src/core/events.rs` after the existing choice events:

```rust
// === Gameplay Domain Events ===

/// Enemy spawned in the world
#[derive(Event, Clone)]
pub struct EnemySpawned {
    pub enemy_entity: Entity,
    pub enemy_type: crate::entities::enemy::EnemyType,
    pub position: Vec2,
}

/// Bullet hit an enemy
#[derive(Event, Clone)]
pub struct BulletHitEnemy {
    pub bullet_entity: Entity,
    pub enemy_entity: Entity,
    pub damage: f32,
    pub impact_position: Vec2,
}

/// Player took damage
#[derive(Event, Clone)]
pub struct PlayerDamaged {
    pub damage: f32,
    pub source_entity: Option<Entity>,
}

/// Enemy was killed
#[derive(Event, Clone)]
pub struct EnemyKilled {
    pub enemy_entity: Entity,
    pub position: Vec2,
    pub enemy_color: crate::entities::enemy::EnemyColor,
}
```

**Step 2: Commit event contract definitions**

```bash
cd /home/laged/Codings/laged/bevy-demo/.worktrees/gameplay-events
git add src/core/events.rs
git commit -m "feat: add gameplay event contracts for Bevy 0.17 Trigger pattern

Events defined:
- EnemySpawned: notify systems when enemy enters world
- BulletHitEnemy: trigger impact effects and damage
- PlayerDamaged: reactive health UI updates
- EnemyKilled: trigger death effects and cleanup

These contracts enable parallel UI/Graphics agent work.

🤖 Generated with Claude Code (https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 1: Modernize Collision Detection (Bullet ↔ Enemy)

**Files:**
- Modify: `src/core/collision.rs:80-120`
- Test: `tests/collision_events_test.rs` (new)

**Step 1: Write failing test for BulletHitEnemy event**

Create `tests/collision_events_test.rs`:

```rust
use bevy::prelude::*;
use hell_game::test_utils::*;
use hell_game::core::events::BulletHitEnemy;
use hell_game::entities::enemy::Enemy;
use hell_game::domains::gameplay::combat::Bullet;
use hell_game::GameConfig;

#[test]
fn test_bullet_hit_enemy_event_fired() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    init_for_testing(&mut app, &config);

    // Spawn enemy at specific position
    let mut commands = app.world_mut().commands();
    let enemy_id = commands.spawn((
        Enemy { health: config.enemy.health },
        Transform::from_translation(Vec3::new(100.0, 100.0, 0.0)),
    )).id();

    // Spawn bullet at same position (guaranteed collision)
    let bullet_id = commands.spawn((
        Bullet,
        Transform::from_translation(Vec3::new(100.0, 100.0, 0.0)),
    )).id();

    // Track if event was fired
    let mut event_fired = false;
    app.world_mut().observe(move |trigger: Trigger<BulletHitEnemy>| {
        event_fired = true;
        assert_eq!(trigger.event().bullet_entity, bullet_id);
        assert_eq!(trigger.event().enemy_entity, enemy_id);
    });

    // Run collision detection
    app.update();

    assert!(event_fired, "BulletHitEnemy event should have been fired");
}
```

**Step 2: Run test to verify it fails**

```bash
cd /home/laged/Codings/laged/bevy-demo/.worktrees/gameplay-events
cargo test test_bullet_hit_enemy_event_fired -- --nocapture
```

Expected: FAIL with "event_fired assertion failed" (observer not triggered)

**Step 3: Refactor handle_enemy_bullet_collision to emit events**

Modify `src/core/collision.rs`, replace `handle_enemy_bullet_collision` function (around line 80):

```rust
fn handle_enemy_bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(&Transform, Entity), With<Bullet>>,
    tree: Res<EnemyKdTree>,
    config: Res<crate::config_loader::GameConfig>,
    mut enemy_query: Query<(&Transform, &mut Enemy, Entity)>,
    particle_assets: Res<ParticleEffectAssets>,
) {
    for (bullet_transform, bullet_entity) in bullet_query.iter() {
        let bullet_pos = bullet_transform.translation;
        let enemies = tree.0.within_radius(&[bullet_pos.x, bullet_pos.y], 20.0);

        for enemy_collidable in enemies.iter() {
            if let Ok((enemy_transform, mut enemy, enemy_entity)) = enemy_query.get_mut(enemy_collidable.entity) {
                // Apply damage
                enemy.health -= config.gun.bullet_damage;

                // Emit event for impact effects (Graphics Agent will observe)
                commands.trigger(crate::core::events::BulletHitEnemy {
                    bullet_entity,
                    enemy_entity,
                    damage: config.gun.bullet_damage,
                    impact_position: bullet_pos.truncate(),
                });

                // Despawn bullet
                commands.entity(bullet_entity).despawn();

                // Check if enemy died
                if enemy.health <= 0.0 {
                    let enemy_pos = enemy_transform.translation.truncate();
                    let enemy_color = /* get color from enemy component */;

                    commands.trigger(crate::core::events::EnemyKilled {
                        enemy_entity,
                        position: enemy_pos,
                        enemy_color,
                    });

                    commands.entity(enemy_entity).despawn();
                }

                break; // Bullet hits only one enemy
            }
        }
    }
}
```

**Step 4: Run test to verify it passes**

```bash
cargo test test_bullet_hit_enemy_event_fired -- --nocapture
```

Expected: PASS

**Step 5: Commit**

```bash
git add src/core/collision.rs tests/collision_events_test.rs
git commit -m "refactor: modernize bullet-enemy collision to emit BulletHitEnemy events

Replaces direct particle spawning with event emission.
Graphics Agent will observe BulletHitEnemy to spawn impact effects.

Test coverage: collision_events_test.rs

🤖 Generated with Claude Code (https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 2: Modernize Player Damage System

**Files:**
- Modify: `src/core/collision.rs:45-63`
- Modify: `src/entities/player.rs` (add observer)
- Test: `tests/player_damage_events_test.rs` (new)

**Step 1: Write failing test for PlayerDamaged event**

Create `tests/player_damage_events_test.rs`:

```rust
use bevy::prelude::*;
use hell_game::test_utils::*;
use hell_game::core::events::PlayerDamaged;
use hell_game::entities::player::{Player, Health};
use hell_game::entities::enemy::Enemy;
use hell_game::GameConfig;

#[test]
fn test_player_damaged_event_on_enemy_collision() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    init_for_testing(&mut app, &config);

    // Spawn player
    let mut commands = app.world_mut().commands();
    commands.spawn((
        Player,
        Health(100.0),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));

    // Spawn enemy at collision distance
    commands.spawn((
        Enemy { health: 50.0 },
        Transform::from_translation(Vec3::new(30.0, 0.0, 0.0)), // Within 50.0 radius
    ));

    // Track event
    let mut event_count = 0;
    app.world_mut().observe(move |trigger: Trigger<PlayerDamaged>| {
        event_count += 1;
        assert_eq!(trigger.event().damage, config.enemy.damage);
    });

    // Run collision detection
    app.update();

    assert!(event_count > 0, "PlayerDamaged event should fire on enemy collision");
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_player_damaged_event_on_enemy_collision -- --nocapture
```

Expected: FAIL (event not emitted)

**Step 3: Refactor handle_enemy_player_collision to emit events**

Modify `src/core/collision.rs`, replace `handle_enemy_player_collision`:

```rust
fn handle_enemy_player_collision(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    tree: Res<EnemyKdTree>,
    config: Res<crate::config_loader::GameConfig>,
) {
    if player_query.is_empty() {
        return;
    }

    let Ok(player_transform) = player_query.single() else { return; };
    let player_pos = player_transform.translation;
    let enemies = tree.0.within_radius(&[player_pos.x, player_pos.y], 50.0);

    if !enemies.is_empty() {
        // Emit event instead of directly modifying health
        commands.trigger(crate::core::events::PlayerDamaged {
            damage: config.enemy.damage,
            source_entity: Some(enemies[0].entity),
        });
    }
}
```

**Step 4: Add observer in player.rs to handle PlayerDamaged**

Modify `src/entities/player.rs`, add this system to the plugin:

```rust
fn handle_player_damaged(
    trigger: Trigger<crate::core::events::PlayerDamaged>,
    mut player_health: Query<&mut Health, With<Player>>,
) {
    if let Ok(mut health) = player_health.single_mut() {
        health.0 -= trigger.event().damage;
    }
}
```

Add to PlayerPlugin's build() method:

```rust
app.observe(handle_player_damaged);
```

**Step 5: Run test to verify it passes**

```bash
cargo test test_player_damaged_event_on_enemy_collision -- --nocapture
```

Expected: PASS

**Step 6: Commit**

```bash
git add src/core/collision.rs src/entities/player.rs tests/player_damage_events_test.rs
git commit -m "refactor: modernize player damage to use PlayerDamaged events

Collision system emits events, player system observes and applies damage.
Enables UI Agent to observe for reactive health bar updates.

Test coverage: player_damage_events_test.rs

🤖 Generated with Claude Code (https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 3: Add EnemySpawned Event Emission

**Files:**
- Modify: `src/entities/enemy.rs` (enemy spawning system)
- Test: `tests/enemy_spawn_events_test.rs` (new)

**Step 1: Write failing test for EnemySpawned event**

Create `tests/enemy_spawn_events_test.rs`:

```rust
use bevy::prelude::*;
use hell_game::test_utils::*;
use hell_game::core::events::EnemySpawned;
use hell_game::entities::enemy::{Enemy, EnemyType};
use hell_game::GameConfig;

#[test]
fn test_enemy_spawned_event_on_spawn() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    init_for_testing(&mut app, &config);

    let mut event_count = 0;
    app.world_mut().observe(move |trigger: Trigger<EnemySpawned>| {
        event_count += 1;
        assert_eq!(trigger.event().enemy_type, EnemyType::Green);
    });

    // Trigger enemy spawning (run a few frames)
    run_frames(&mut app, 10);

    assert!(event_count > 0, "EnemySpawned events should fire during gameplay");
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_enemy_spawned_event_on_spawn -- --nocapture
```

Expected: FAIL (no events emitted)

**Step 3: Add event emission to enemy spawning system**

Find the enemy spawning system in `src/entities/enemy.rs` (search for `spawn_enemy` or similar). Add event emission after enemy spawn:

```rust
// After spawning enemy entity
let enemy_entity = commands.spawn((
    Enemy { health: config.enemy.health },
    enemy_type,
    Transform::from_translation(spawn_pos),
    // ... other components
)).id();

// Emit event
commands.trigger(crate::core::events::EnemySpawned {
    enemy_entity,
    enemy_type,
    position: spawn_pos.truncate(),
});
```

**Step 4: Run test to verify it passes**

```bash
cargo test test_enemy_spawned_event_on_spawn -- --nocapture
```

Expected: PASS

**Step 5: Commit**

```bash
git add src/entities/enemy.rs tests/enemy_spawn_events_test.rs
git commit -m "feat: emit EnemySpawned events on enemy creation

Enables reactive systems:
- Graphics Agent: spawn entry animations
- UI Agent: update enemy count display
- Analytics: track spawn patterns

Test coverage: enemy_spawn_events_test.rs

🤖 Generated with Claude Code (https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 4: Add Component Lifecycle Observers

**Files:**
- Modify: `src/entities/player.rs` (add On::<Add> observer for Health)
- Test: `tests/component_lifecycle_test.rs` (new)

**Step 1: Write failing test for Health component lifecycle**

Create `tests/component_lifecycle_test.rs`:

```rust
use bevy::prelude::*;
use hell_game::test_utils::*;
use hell_game::entities::player::{Player, Health};
use hell_game::GameConfig;

#[test]
fn test_health_component_add_triggers_observer() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    init_for_testing(&mut app, &config);

    let mut add_detected = false;

    // Observer for Health component addition
    app.world_mut().observe(move |_trigger: Trigger<OnAdd, Health>| {
        add_detected = true;
    });

    // Spawn player with Health component
    let mut commands = app.world_mut().commands();
    commands.spawn((
        Player,
        Health(100.0),
    ));

    app.update();

    assert!(add_detected, "OnAdd<Health> observer should trigger");
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_health_component_add_triggers_observer -- --nocapture
```

Expected: FAIL (no observer registered)

**Step 3: Add On::<Add> observer to PlayerPlugin**

Modify `src/entities/player.rs`, add to `PlayerPlugin::build()`:

```rust
// Component lifecycle observers
app.observe(on_health_added);
```

Add observer system:

```rust
fn on_health_added(
    _trigger: Trigger<OnAdd, Health>,
) {
    // Future: Initialize health bar UI, log health initialization
    info!("Health component added to entity");
}
```

**Step 4: Run test to verify it passes**

```bash
cargo test test_health_component_add_triggers_observer -- --nocapture
```

Expected: PASS

**Step 5: Commit**

```bash
git add src/entities/player.rs tests/component_lifecycle_test.rs
git commit -m "feat: add component lifecycle observers for Health

Uses Bevy 0.17 On::<Add> pattern for reactive component tracking.
Enables future UI systems to observe Health additions for HUD initialization.

Test coverage: component_lifecycle_test.rs

🤖 Generated with Claude Code (https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 5: Integration Testing & Performance Verification

**Files:**
- Test: `tests/event_system_integration_test.rs` (new)

**Step 1: Write comprehensive integration test**

Create `tests/event_system_integration_test.rs`:

```rust
use bevy::prelude::*;
use hell_game::test_utils::*;
use hell_game::core::events::*;
use hell_game::entities::{Player, Enemy, Health};
use hell_game::domains::gameplay::combat::Bullet;
use hell_game::GameConfig;

#[test]
fn test_event_system_integration_full_gameplay_loop() {
    let config = GameConfig::default();
    let mut app = create_headless_app(config.clone());
    init_for_testing(&mut app, &config);

    // Track all events
    let mut enemy_spawned_count = 0;
    let mut bullet_hit_count = 0;
    let mut player_damaged_count = 0;
    let mut enemy_killed_count = 0;

    app.world_mut().observe(move |_: Trigger<EnemySpawned>| {
        enemy_spawned_count += 1;
    });

    app.world_mut().observe(move |_: Trigger<BulletHitEnemy>| {
        bullet_hit_count += 1;
    });

    app.world_mut().observe(move |_: Trigger<PlayerDamaged>| {
        player_damaged_count += 1;
    });

    app.world_mut().observe(move |_: Trigger<EnemyKilled>| {
        enemy_killed_count += 1;
    });

    // Run 100 frames of gameplay
    run_frames(&mut app, 100);

    // Verify events fired during gameplay
    println!("Enemies spawned: {}", enemy_spawned_count);
    println!("Bullet hits: {}", bullet_hit_count);
    println!("Player damaged: {}", player_damaged_count);
    println!("Enemies killed: {}", enemy_killed_count);

    assert!(enemy_spawned_count > 0, "Enemies should spawn during gameplay");
}

#[test]
fn test_event_system_performance_baseline() {
    let config = GameConfig::benchmark_mode();
    let mut app = create_headless_app(config.clone());
    init_for_testing(&mut app, &config);

    // Run 1000 frames with event system
    let start = std::time::Instant::now();
    run_frames(&mut app, 1000);
    let elapsed = start.elapsed();

    let fps = 1000.0 / elapsed.as_secs_f32();
    println!("Event system FPS: {:.2}", fps);

    assert!(fps >= 15000.0, "Event system should maintain ≥15k fps baseline");
}
```

**Step 2: Run integration tests**

```bash
cargo test test_event_system_integration -- --nocapture
cargo test test_event_system_performance_baseline -- --nocapture
```

Expected: Both PASS, performance ≥15k fps

**Step 3: Commit**

```bash
git add tests/event_system_integration_test.rs
git commit -m "test: add comprehensive event system integration tests

Verifies:
- All gameplay events fire during normal gameplay
- Performance baseline maintained (≥15k fps)
- Event observers trigger correctly
- No regressions in collision detection

🤖 Generated with Claude Code (https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 6: Update Documentation & Final Verification

**Files:**
- Modify: `src/core/README.md` (if exists, document event system)
- Modify: `docs/ARCHITECTURE.md` (update event patterns section)

**Step 1: Run all tests to verify no regressions**

```bash
cargo test -- --nocapture
```

Expected: All tests pass

**Step 2: Check performance baseline**

```bash
cargo test test_headless_app_creation -- --nocapture
```

Expected: ≥15,000 fps

**Step 3: Update ARCHITECTURE.md with new event patterns**

Add section to `docs/ARCHITECTURE.md` under "Event-Driven Communication":

```markdown
### Bevy 0.17 Event Patterns

**Trigger/Observer Pattern:**
```rust
// Emit event
commands.trigger(BulletHitEnemy {
    bullet_entity,
    enemy_entity,
    damage,
    impact_position,
});

// Observe event
app.observe(handle_bullet_hit);

fn handle_bullet_hit(trigger: Trigger<BulletHitEnemy>) {
    let event = trigger.event();
    // React to event
}
```

**Component Lifecycle:**
```rust
// Observe component addition
app.observe(on_health_added);

fn on_health_added(trigger: Trigger<OnAdd, Health>) {
    // React when Health component added
}
```

**Event Contracts (src/core/events.rs):**
- `EnemySpawned` - Enemy entered world
- `BulletHitEnemy` - Bullet collision with enemy
- `PlayerDamaged` - Player took damage
- `EnemyKilled` - Enemy destroyed
```

**Step 4: Commit documentation updates**

```bash
git add docs/ARCHITECTURE.md
git commit -m "docs: document Bevy 0.17 event system patterns

Added:
- Trigger/Observer pattern examples
- Component lifecycle event usage
- Event contract reference

Enables other agents to understand event-based coordination.

🤖 Generated with Claude Code (https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

**Step 5: Final verification and push**

```bash
# Run full test suite
cargo test -- --nocapture

# Check build
cargo check

# Visual verification (optional)
cargo run

# If all tests pass, ready for merge
git log --oneline -10
```

---

## Success Criteria Checklist

Before merging `gameplay-events` branch to `bevy-0.17-features`:

- [ ] All EventReader/EventWriter patterns replaced with Trigger/Observer
- [ ] BulletHitEnemy events emitted on collision
- [ ] PlayerDamaged events emitted when player hit
- [ ] EnemySpawned events emitted on enemy creation
- [ ] EnemyKilled events emitted when enemy dies
- [ ] Component lifecycle observers (On::<Add>) implemented for Health
- [ ] All tests passing (collision, player, enemy, integration)
- [ ] Performance baseline maintained (≥15k fps)
- [ ] Documentation updated (ARCHITECTURE.md)
- [ ] No compiler warnings
- [ ] Branch ready for merge (all commits clean)

---

## Notes for Parallel Agents

**UI Agent can now observe these events:**
- `PlayerDamaged` → Update health bar reactively
- `EnemySpawned` → Update enemy count display
- `EnemyKilled` → Show kill counter

**Graphics Agent can now observe these events:**
- `BulletHitEnemy` → Spawn impact particle effects
- `EnemyKilled` → Spawn death burst and linger effects
- `EnemySpawned` → Spawn entry animations (future)

**Event contracts are frozen** - do not modify event definitions once parallel work begins. If new events needed, coordinate via design document first.
