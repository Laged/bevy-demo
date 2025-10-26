# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs` bootstraps the Bevy app; keep wiring here and move gameplay logic into modules such as `player.rs`, `enemy.rs`, and `world.rs`.
- Share systems via `src/lib.rs`; rendering and UI lives in `camera.rs`, `animation.rs`, and `gui.rs`, while collisions stay in `collision.rs`.
- Runtime defaults live in `src/configs.rs`; update knobs there instead of scattering literals.
- Store sprites, fonts, and audio in `assets/`; `target/` is build output and must stay untracked.

## Build, Test, and Development Commands
- `cargo run` launches the shooter with live asset reload for rapid iteration.
- `cargo build --release` produces an optimized binary ready for demos or profiling.
- `cargo test` runs unit and integration suites; execute it before every push.
- `cargo fmt --all` applies rustfmt rules; pair with `cargo clippy --all-targets --all-features` to catch ECS mistakes.
- `nix develop` enters the pinned toolchain when working inside the Nix flow.

## Coding Style & Naming Conventions
- rustfmt defaults (4-space indents, trailing commas) are mandatory; never hand-format.
- Use `snake_case` for modules and functions, `CamelCase` for structs/resources/components, and `SCREAMING_SNAKE_CASE` for constants.
- Group related systems behind a `pub fn plugin()` export so `main.rs` stays declarative.

## Testing Guidelines
- Place unit tests in `#[cfg(test)] mod tests` blocks at the bottom of the file they cover.
- Use `App::new()` with minimal plugin registration; mock resources instead of loading assets.
- Add integration tests under `tests/` for cross-system behaviour and document any manual verification in the PR.

## Commit & Pull Request Guidelines
- Follow conventional prefixes (`feat:`, `fix:`, `docs:`) with an imperative summary; keep commits focused.
- PRs need a short overview, command output (`cargo test`, relevant `cargo run` scenario), and visuals when gameplay changes.
- Link related issues (`Fixes #123`) and list follow-up tasks so maintainers can plan review.
