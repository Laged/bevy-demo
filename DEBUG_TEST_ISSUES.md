# Debug: Test Discovery Issues

## Problem Observed

From the script output:
```
Running unittests src/lib.rs (target/debug/deps/hell_game-5d539ac43ecdd8a6)
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out

Running tests/tilemap_test.rs (target/debug/deps/tilemap_test-6d9b71ac84f88bfa)
running 1 test
test test_tilemap_performance_impact ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out
```

**Issue**: 4 out of 5 tests are "filtered out" in tilemap_test.rs

## Likely Causes

### 1. Test Compilation Issues
When tests fail to compile, cargo marks them as "filtered out" rather than showing compilation errors with `cargo test`.

### 2. Missing Test Helper Functions
The tests import:
```rust
use hell_game::domains::testing::harness::create_headless_app;
use hell_game::domains::testing::simulation::{run_frames, set_state};
```

These might not exist or have wrong signatures in the graphics-tilemap worktree.

### 3. Import Path Issues
Tests use `hell_game::state::GameState` which should be `hell_game::core::state::GameState` or rely on re-exports.

## How to Debug

### Step 1: Check if tests compile
```bash
cd /home/laged/Codings/laged/bevy-demo/.worktrees/graphics-tilemap
cargo test --no-run 2>&1 | head -50
```

This will show compilation errors without running tests.

### Step 2: Check test helper availability
```bash
cd /home/laged/Codings/laged/bevy-demo/.worktrees/graphics-tilemap
cargo test --test tilemap_test --no-run
```

### Step 3: Run specific test with verbose output
```bash
cd /home/laged/Codings/laged/bevy-demo/.worktrees/graphics-tilemap
cargo test test_tilemap_zones_headless_creation -- --exact --nocapture
```

### Step 4: Check what test_utils provides
The tests might need a different import. Check if `test_utils` exists:
```bash
grep -r "pub mod test_utils" src/
grep -r "pub fn create_headless_app" src/
```

## Expected Files That Might Be Missing

Based on test imports, these should exist:
- `src/domains/testing/harness.rs` with `create_headless_app()`
- `src/domains/testing/simulation.rs` with `run_frames()` and `set_state()`
- Proper re-exports in `src/lib.rs`

## Quick Fix Options

### Option A: Check main repo for test helpers
The main `dev` branch likely has these test utilities. They might need to be copied to the worktree.

### Option B: Update test imports
If test helpers are in a different location, update the imports in `tests/tilemap_test.rs`.

### Option C: Verify Cargo.toml
Make sure the test dependencies are correct and the test is being discovered.

## Git Commit Issue

The git commit failed because files weren't staged. The worktree might be in a different state than expected. Check:

```bash
cd /home/laged/Codings/laged/bevy-demo/.worktrees/graphics-tilemap
git status
git diff --stat
```

See what files actually changed vs. what the script expects to commit.
