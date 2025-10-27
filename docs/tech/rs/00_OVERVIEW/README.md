# Overview (Tech)

## Architectural Snapshot
- `src/main.rs` wires Bevy `App` with modular plugins; keep gameplay logic in dedicated modules under `src/`.
- Shared systems live in `src/lib.rs`; resource defaults in `src/configs.rs`.
- Asset pipeline pulls from `assets/` with live reload during `cargo run`.

## Responsibilities
- Document how plugins compose (load order, dependencies, feature flags).
- Track cross-cutting concerns (save/load, analytics, tooling).
- Note build targets, platform constraints, and performance budgets.

## Outstanding Work
- Identify missing automation (CI, validation scripts).
- Capture refactor opportunities to keep module boundaries clean.
- List external crates or Bevy versions to watch for upgrades.
