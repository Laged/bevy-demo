# Gameplay Domain

**Maintainer:** Gameplay Agent

## Responsibilities
- Game balance and configuration
- Combat logic (guns, bullets)
- Entity spawning and AI
- State machine coordination

## Modules
- `config/` - Game configuration (constants, loader)
- `combat.rs` - Gun and bullet systems (GunPlugin)

## Dependencies
**Reads from:**
- `core::state::GameState`
- `entities::player::*`
- `entities::enemy::*`

**Writes to:**
- `core::state` - State transitions
- Entity spawning/despawning

## Contact Points
For inter-domain communication, use events in `core::events`.
