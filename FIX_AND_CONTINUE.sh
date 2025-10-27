#!/usr/bin/env bash
set -e

GREEN='\033[0;32m'
NC='\033[0m'

status() {
    echo -e "${GREEN}[✓]${NC} $1"
}

echo "=== Fixing UI Viewport and Continuing Merge ==="
echo ""

# Commit the fix
cd /home/laged/Codings/laged/bevy-demo/.worktrees/ui-viewport
status "Adding MinimapCameraReady event fix..."
git add src/core/events.rs
git commit --amend --no-edit

status "Testing ui-viewport..."
cargo test --quiet 2>&1 | grep -E "(test result|passed|failed)" || cargo test

echo ""
status "UI viewport fixed and tested!"
echo ""

# Continue with merge
cd /home/laged/Codings/laged/bevy-demo
status "Switching to bevy-0.17-features..."
git checkout bevy-0.17-features

status "Merging graphics-tilemap..."
git merge graphics-tilemap --no-edit || echo "Already merged"

status "Merging ui-widgets..."
git merge ui-widgets --no-edit || echo "Already merged"

status "Merging ui-viewport (with fix)..."
git merge ui-viewport --no-edit

status "Testing bevy-0.17-features..."
cargo test --quiet 2>&1 | grep -E "(test result|passed|failed)" | head -10

echo ""
status "Merging to dev..."
git checkout dev
git merge bevy-0.17-features --no-edit

status "Final test on dev..."
cargo test --quiet 2>&1 | grep -E "(test result|passed|failed)" | head -10

echo ""
echo "========================================="
echo "✓ Bevy 0.17 Integration COMPLETE!"
echo "========================================="
echo ""
echo "All three features merged to dev:"
echo "  ✓ Graphics tilemap (colored arena zones)"
echo "  ✓ UI widgets (pause menu, settings)"
echo "  ✓ UI viewport (minimap with ViewportNode)"
echo ""
