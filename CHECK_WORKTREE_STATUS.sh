#!/usr/bin/env bash
# Check the actual status of each worktree

echo "=== Graphics Tilemap Worktree ==="
cd /home/laged/Codings/laged/bevy-demo/.worktrees/graphics-tilemap
echo "Git status:"
git status --short
echo ""
echo "Files in src/domains/graphics/:"
ls -la src/domains/graphics/ 2>/dev/null || echo "Directory doesn't exist"
echo ""
echo "Files in tests/:"
ls -la tests/ 2>/dev/null || echo "Directory doesn't exist"
echo ""
echo "Recent commits:"
git log --oneline -5
echo ""
echo "Changes not staged:"
git diff --stat
echo ""
echo "=========================================="
echo ""

echo "=== UI Widgets Worktree ==="
cd /home/laged/Codings/laged/bevy-demo/.worktrees/ui-widgets
echo "Git status:"
git status --short
echo ""
echo "Files in src/domains/ui/:"
ls -la src/domains/ui/ 2>/dev/null || echo "Directory doesn't exist"
echo ""
echo "Recent commits:"
git log --oneline -5
echo ""
echo "=========================================="
echo ""

echo "=== UI Viewport Worktree ==="
cd /home/laged/Codings/laged/bevy-demo/.worktrees/ui-viewport
echo "Git status:"
git status --short
echo ""
echo "Files in src/domains/ui/:"
ls -la src/domains/ui/ 2>/dev/null || echo "Directory doesn't exist"
echo ""
echo "Recent commits:"
git log --oneline -5
echo ""
