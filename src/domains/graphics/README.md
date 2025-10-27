# Graphics Domain

**Maintainer:** Graphics Agent

## Responsibilities
- Particle effects (trails, impacts, death effects)
- Sprite animations
- Visual polish

## Modules
- `particles.rs` - Particle effect system (ParticleEffectsPlugin)
- `animation.rs` - Sprite animation system (AnimationPlugin)

## Dependencies
**Reads from:**
- `core::state::GameState`
- `entities::player::{Player, PlayerState}`
- `entities::enemy::{Enemy, EnemyColor}`
- `core::resources::CursorPosition`

**Writes to:**
- Sprite rendering components

## Contact Points
For inter-domain communication, use events in `core::events`.
