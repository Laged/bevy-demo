# Tilemap Test Compilation Fix

## Issue Found

The tilemap tests were showing "4 filtered out" because 4 out of 5 tests failed to compile.

**Error**: Invalid syntax in query generic type parameter
```rust
// Line 128 in tests/tilemap_test.rs - INVALID SYNTAX
.query::<(&TilemapTile, &hell_game::entities::world::GameEntity)>()
```

**Problem**: You cannot use full module paths inside generic type parameters. Rust requires imported types.

## Fix Applied

### 1. Added import at top of test file:
```rust
use hell_game::entities::world::GameEntity;
```

### 2. Updated query to use imported type:
```rust
// Changed from:
.query::<(&TilemapTile, &hell_game::entities::world::GameEntity)>()

// To:
.query::<(&TilemapTile, &GameEntity)>()
```

## Files Modified

- `.worktrees/graphics-tilemap/tests/tilemap_test.rs`
  - Added `GameEntity` import (line 12)
  - Fixed query syntax (line 129)

## Expected Result

All 5 tilemap tests should now compile and run:
1. ✅ `test_tilemap_zones_headless_creation`
2. ✅ `test_zone_assignment_by_distance`
3. ✅ `test_tilemap_cleanup_on_state_exit`
4. ✅ `test_tilemap_performance_impact`
5. ✅ `test_tile_grid_alignment`

## Next Step

Run the integration script again:

```bash
cd /home/laged/Codings/laged/bevy-demo
./COMPLETE_BEVY_0.17_INTEGRATION.sh
```

All tests in graphics-tilemap should now pass (no more "filtered out").
