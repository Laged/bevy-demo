#!/usr/bin/env bash
# Diagnostic script to check worktree state

echo "=== Graphics Tilemap Worktree Diagnostics ==="
echo ""

cd /home/laged/Codings/laged/bevy-demo/.worktrees/graphics-tilemap

echo "1. Git Status:"
git status
echo ""

echo "2. Modified files:"
git diff --name-status
echo ""

echo "3. Staged files:"
git diff --cached --name-status
echo ""

echo "4. Check if tilemap files exist:"
ls -la src/domains/graphics/tilemap.rs 2>/dev/null && echo "✓ tilemap.rs exists" || echo "✗ tilemap.rs MISSING"
ls -la tests/tilemap_test.rs 2>/dev/null && echo "✓ tilemap_test.rs exists" || echo "✗ tilemap_test.rs MISSING"
ls -la src/domains/graphics/mod.rs 2>/dev/null && echo "✓ mod.rs exists" || echo "✗ mod.rs MISSING"
echo ""

echo "5. Check tilemap.rs first 10 lines:"
head -10 src/domains/graphics/tilemap.rs 2>/dev/null || echo "Cannot read tilemap.rs"
echo ""

echo "6. Run single test to see compilation errors:"
cargo test test_tilemap_zones_headless_creation --no-run 2>&1 | head -30
echo ""

echo "7. Check test imports:"
grep "^use " tests/tilemap_test.rs | head -10
echo ""
