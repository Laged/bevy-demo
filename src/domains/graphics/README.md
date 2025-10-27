# Graphics Domain

**Maintainer:** Graphics Agent

## Responsibilities
- Particle effects (trails, impacts, death effects)
- Sprite animations
- Tilemap arena backgrounds
- Visual polish

## Modules
- `particles.rs` - Particle effect system (ParticleEffectsPlugin)
- `animation.rs` - Sprite animation system (AnimationPlugin)
- `tilemap.rs` - Arena background zone system (TilemapPlugin)

## Dependencies
**Reads from:**
- `core::state::GameState`
- `entities::player::{Player, PlayerState}`
- `entities::enemy::{Enemy, EnemyColor}`
- `core::resources::{CursorPosition, GlobalTextureAtlas}`
- `domains::gameplay::config::loader::GameConfig`
- `entities::world::GameEntity` (for cleanup marker)

**Writes to:**
- Sprite rendering components
- Tilemap zone entities (ArenaZone, TilemapTile)

## Contact Points
For inter-domain communication, use events in `core::events`.
