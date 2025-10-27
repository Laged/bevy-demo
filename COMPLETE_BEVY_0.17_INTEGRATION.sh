#!/usr/bin/env bash
# Complete Bevy 0.17 Integration - Manual Execution Script
# Run this from the repository root

set -e  # Exit on error

echo "========================================="
echo "Bevy 0.17 Integration Completion Script"
echo "========================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print status
status() {
    echo -e "${GREEN}[✓]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[!]${NC} $1"
}

error() {
    echo -e "${RED}[✗]${NC} $1"
}

# Step 1: Test Graphics Tilemap
echo "Step 1: Testing graphics-tilemap branch..."
cd /home/laged/Codings/laged/bevy-demo/.worktrees/graphics-tilemap

status "Running tests in graphics-tilemap..."
cargo test -- --nocapture || { error "Graphics tilemap tests failed!"; exit 1; }

status "Checking performance baseline..."
cargo test test_tilemap_performance_impact -- --nocapture || { error "Performance test failed!"; exit 1; }

status "Committing graphics-tilemap..."
git add src/domains/graphics/tilemap.rs tests/tilemap_test.rs src/domains/graphics/mod.rs src/lib.rs src/domains/graphics/README.md
git commit -m "feat(graphics): add colored arena zones with tilemap system

Implements sprite grid approach with distance-based zones:
- Safe zone (green): center within 300 units
- Danger zone (red): 300-800 units from center
- Neutral zone (gray): beyond 800 units
- ~3,276 tile entities covering 3000x2500 world
- Full headless mode support
- Performance benchmarks (≥15k fps maintained)

🤖 Generated with Claude Code (https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"

status "graphics-tilemap complete!"
echo ""

# Step 2: Test UI Widgets
echo "Step 2: Testing ui-widgets branch..."
cd /home/laged/Codings/laged/bevy-demo/.worktrees/ui-widgets

status "Running tests in ui-widgets..."
cargo test -- --nocapture || { error "UI widgets tests failed!"; exit 1; }

status "Checking performance baseline..."
cargo test test_ui_widgets_performance_baseline -- --nocapture || { error "Performance test failed!"; exit 1; }

status "Committing ui-widgets..."
git add src/domains/ui/pause_menu.rs src/domains/ui/settings_panel.rs src/domains/ui/mod.rs tests/ui_widgets_test.rs
git commit -m "feat(ui): add headless widgets for pause menu and settings panel

Implements Bevy 0.17 widget primitives:
- Pause menu with Button widgets (resume/quit)
- Settings panel with Sliders (volume, difficulty, particles)
- Full headless mode support (PluginMode::Headless)
- Event-driven updates (PauseMenuToggled, SettingsChanged)
- GameSettings resource for persistent configuration
- Performance baseline maintained (≥15k fps)

🤖 Generated with Claude Code (https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"

status "ui-widgets complete!"
echo ""

# Step 3: Test UI Viewport
echo "Step 3: Testing ui-viewport branch..."
cd /home/laged/Codings/laged/bevy-demo/.worktrees/ui-viewport

status "Running tests in ui-viewport..."
cargo test -- --nocapture || { error "UI viewport tests failed!"; exit 1; }

status "Checking performance baseline..."
cargo test test_viewport_system_performance_baseline -- --nocapture || { error "Performance test failed!"; exit 1; }

status "Committing ui-viewport..."
git add src/domains/ui/minimap.rs src/domains/ui/mod.rs tests/viewport_test.rs
git commit -m "feat(ui): add minimap using Bevy 0.17 ViewportNode

Implements tactical overview system:
- MinimapCamera (5x zoom out, follows player)
- ViewportNode rendering camera to UI corner (bottom-right)
- Camera position tracking system
- MinimapCameraReady event emission
- Full headless mode support (camera logic, no rendering)
- Performance baseline maintained (≥15k fps)

🤖 Generated with Claude Code (https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"

status "ui-viewport complete!"
echo ""

# Step 4: Merge all branches to bevy-0.17-features
echo "Step 4: Merging branches to bevy-0.17-features..."
cd /home/laged/Codings/laged/bevy-demo

status "Switching to bevy-0.17-features branch..."
git checkout bevy-0.17-features

status "Merging graphics-tilemap..."
git merge graphics-tilemap --no-edit
cargo test -- --nocapture || { error "Tests failed after graphics-tilemap merge!"; exit 1; }

status "Merging ui-viewport..."
git merge ui-viewport --no-edit
cargo test -- --nocapture || { error "Tests failed after ui-viewport merge!"; exit 1; }

status "Merging ui-widgets..."
git merge ui-widgets --no-edit
cargo test -- --nocapture || { error "Tests failed after ui-widgets merge!"; exit 1; }

status "All branches merged successfully!"
echo ""

# Step 5: UI Polish (optional - can be done later)
echo "Step 5: UI Polish..."
warn "UI polish is optional and can be done in a separate session"
warn "Skipping for now - Phase 6 can be implemented later"
echo ""

# Step 6: Final merge to dev
echo "Step 6: Final merge to dev..."
status "Switching to dev branch..."
git checkout dev

status "Merging bevy-0.17-features to dev..."
git merge bevy-0.17-features --no-edit

status "Running final test suite..."
cargo test -- --nocapture || { error "Final tests failed!"; exit 1; }

status "Checking final performance baseline..."
cargo test test_headless_app_creation -- --nocapture || { error "Final performance test failed!"; exit 1; }

echo ""
echo "========================================="
echo "✓ Bevy 0.17 Integration COMPLETE!"
echo "========================================="
echo ""
echo "Summary:"
echo "  - Graphics tilemap: ✓ Merged"
echo "  - UI widgets (pause menu, settings): ✓ Merged"
echo "  - UI viewport (minimap): ✓ Merged"
echo "  - All tests passing: ✓"
echo "  - Performance baseline maintained: ✓"
echo ""
echo "Optional next steps:"
echo "  1. Implement UI polish (Phase 6) for gradients and Val helpers"
echo "  2. Clean up worktrees:"
echo "     git worktree remove .worktrees/graphics-tilemap"
echo "     git worktree remove .worktrees/ui-widgets"
echo "     git worktree remove .worktrees/ui-viewport"
echo "  3. Delete merged branches:"
echo "     git branch -d graphics-tilemap ui-widgets ui-viewport"
echo ""
