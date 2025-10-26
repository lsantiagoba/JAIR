#!/bin/bash

set -e

echo "========================================="
echo "  JAIR Snap Build Script"
echo "========================================="
echo ""

# Variables
APP_NAME="jair"
APP_VERSION="1.0.0"
ARCH=$(dpkg --print-architecture)
DEB_FILE="${APP_NAME}_${APP_VERSION}_${ARCH}.deb"

# Check if snapcraft is installed
if ! command -v snapcraft &> /dev/null; then
    echo "❌ Snapcraft is not installed."
    echo "Installing snapcraft..."
    sudo snap install snapcraft --classic
fi

echo "✅ Snapcraft is installed"
echo ""

# Build .deb first if it doesn't exist
if [ ! -f "$DEB_FILE" ]; then
    echo "📦 .deb file not found. Building it first..."
    echo ""
    ./BUILD_DEB.sh
    echo ""
    echo "✅ .deb package created successfully"
    echo ""
else
    echo "✅ Found existing .deb file: $DEB_FILE"
    echo ""
fi

# Clean previous builds
echo "🧹 Cleaning previous snap builds..."
snapcraft clean 2>/dev/null || true
rm -f *.snap 2>/dev/null || true

echo ""
echo "🔨 Building the snap from .deb..."
echo "This may take several minutes..."
echo ""

# Build the snap
snapcraft

echo ""
echo "========================================="
echo "  ✅ Build completed!"
echo "========================================="
echo ""

# Show the generated file
SNAP_FILE=$(ls -1 *.snap 2>/dev/null | head -n1)

if [ -n "$SNAP_FILE" ]; then
    echo "📦 Snap generated: $SNAP_FILE"
    echo "📊 Size: $(du -h "$SNAP_FILE" | cut -f1)"
    echo ""
    echo "To install locally:"
    echo "  sudo snap install $SNAP_FILE --dangerous"
    echo ""
    echo "To run after installation:"
    echo "  jair"
    echo ""
else
    echo "❌ Snap file not found"
    exit 1
fi
