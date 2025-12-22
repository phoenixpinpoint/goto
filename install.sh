#!/bin/bash

# Install script for goto
# Usage: ./install.sh [--debug]

set -e

BUILD_TYPE="release"
TARGET_DIR="target/release"

# Check for --debug flag
if [[ "$1" == "--debug" ]]; then
    BUILD_TYPE="debug"
    TARGET_DIR="target/debug"
fi

echo "Building goto ($BUILD_TYPE)..."
if [[ "$BUILD_TYPE" == "release" ]]; then
    cargo build --release
else
    cargo build
fi

# Create ~/.local/bin if it doesn't exist
mkdir -p ~/.local/bin

echo "Installing goto to ~/.local/bin..."
cp "$TARGET_DIR/goto" ~/.local/bin/

echo ""
echo "✓ goto installed successfully!"
echo ""
echo "Make sure ~/.local/bin is in your PATH and you've sourced the shell script:"
echo "  For zsh: source $(pwd)/scripts/goto.zsh"
echo "  For bash: source $(pwd)/scripts/goto.bash"
echo ""
echo "Add one of the above lines to your ~/.zshrc or ~/.bashrc"
