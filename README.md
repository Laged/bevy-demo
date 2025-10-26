# Star Stream Autocode Lab

This project is built on the excellent [bones-ai/bevy-2d-shooter](https://github.com/bones-ai/bevy-2d-shooter) starter template, keeping its performant Bevy/Rust foundations while repurposing the codebase as a training ground for large language models. The new mission is to teach autonomous coding agents how to build a complete game by following the `.md` design blueprints in this repository.

![screenshot](/screenshot.png)

## What We're Building
- **Game Concept**: *Star Streams* is a Y2K-futurist auto-battler where human drone swarms battle mythic Inari fox-spirits across a hex-grid arena.
- **Learning Focus**: Agents learn to translate rich design docs into systems code—covering combat math (DPS, EHP, Power Level), AI behaviours, encounter pacing, and UX polish.
- **Documentation-Driven**: The `docs/` directory contains the complete design canon, indexed in `docs/00_INDEX.md` and broken into lore, combat, gameplay, MVP roadmaps, and technical specs.

## Getting Started
```bash
cargo run
```

## Key References
- `docs/06_GAME_MVP/06_03_IMPLEMENTATION_ROADMAP.md` – week-by-week showcase plan for agents.
- `docs/02_COMBAT_SYSTEM/` – DPS/EHP formulas and matchup simulations that drive balance.
- `docs/04_GAMEPLAY_SYSTEMS/` – hex-grid movement, AI state machines, and auto-battler rules.
- `src/` – Bevy implementation; `src/configs.rs` holds tweakable runtime values.

## Dev Loop for Agents
1. Study the relevant design doc section.
2. Implement or extend the matching Bevy system.
3. Validate behaviour via `cargo test` (where available) and `cargo run`.
4. Document findings or gaps for the next agent.

## Controls & Credits
- `WASD` moves the player drone; mouse wheel adjusts zoom.
- Assets: [0x72 Dungeon Tileset II](https://0x72.itch.io/dungeontileset-ii)
- Font: [Monogram](https://datagoblin.itch.io/monogram)
