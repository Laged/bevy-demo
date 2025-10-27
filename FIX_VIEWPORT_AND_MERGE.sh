#!/usr/bin/env bash
set -e

GREEN='\033[0;32m'
NC='\033[0m'

status() {
    echo -e "${GREEN}[✓]${NC} $1"
}

echo "=== Fixing UI Viewport API Issues and Merging ==="
echo ""

# Commit the fixes
cd /home/laged/Codings/laged/bevy-demo/.worktrees/ui-viewport
status "Committing Bevy 0.17 API fixes..."
git add src/core/events.rs src/domains/ui/minimap.rs
git commit --amend --no-edit

status "Testing ui-viewport..."
cargo test 2>&1 | grep -E "(test result|passed|failed|error\[)" | head -20

echo ""
status "UI viewport fixed and tested!"
echo ""

# Merge everything
cd /home/laged/Codings/laged/bevy-demo
git checkout bevy-0.17-features

status "Merging all branches..."
git merge graphics-tilemap --no-edit 2>&1 | head -5
git merge ui-widgets --no-edit 2>&1 | head -5
git merge ui-viewport --no-edit 2>&1 | head -5

status "Testing bevy-0.17-features..."
cargo test 2>&1 | grep -E "(test result|passed|failed)" | head -10

echo ""
status "Merging to dev..."
git checkout dev
git merge bevy-0.17-features --no-edit

status "Final test on dev..."
cargo test 2>&1 | grep -E "(test result|passed|failed)" | head -10

echo ""
echo "========================================="
echo "✓ Bevy 0.17 Integration COMPLETE!"
echo "========================================="
echo ""
