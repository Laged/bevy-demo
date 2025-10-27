#!/usr/bin/env bash

echo "=== Debugging UI Viewport ==="
cd /home/laged/Codings/laged/bevy-demo/.worktrees/ui-viewport

echo "1. Checking if it compiles..."
cargo check 2>&1 | head -50

echo ""
echo "2. Trying to build tests..."
cargo test --no-run 2>&1 | head -50

echo ""
echo "3. If build succeeded, list test names..."
cargo test --list 2>&1 | head -20
