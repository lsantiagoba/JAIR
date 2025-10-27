#!/bin/bash

# FLATPAK_TO_SNAP.sh - Script to convert Flatpak build from GNOME Builder to Snap package

set -e  # Exit on error

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}======================================${NC}"
echo -e "${BLUE}  Flatpak to Snap Converter${NC}"
echo -e "${BLUE}======================================${NC}"
echo ""

# Variables
APP_NAME="jair"
APP_VERSION="1.0.0"
ARCH=$(dpkg --print-architecture)
FLATPAK_STAGING="$HOME/.cache/gnome-builder/projects/JAIR/flatpak/staging/x86_64-main/files"
DEB_DIR="debian_package"
SNAP_NAME="${APP_NAME}_${APP_VERSION}_${ARCH}.snap"

# Check if flatpak staging directory exists
if [ ! -d "$FLATPAK_STAGING" ]; then
    echo -e "${RED}❌ Error: Flatpak staging directory not found!${NC}"
    echo -e "${YELLOW}Expected: $FLATPAK_STAGING${NC}"
    echo -e "${YELLOW}Please build the project in GNOME Builder first.${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Found Flatpak staging directory${NC}"
echo ""

# Clean previous builds
echo -e "${BLUE}🧹 Cleaning previous builds...${NC}"
rm -rf "$DEB_DIR" "${APP_NAME}_${APP_VERSION}_${ARCH}.deb" 2>/dev/null || true

# Create debian package structure from flatpak
echo -e "${BLUE}📦 Creating .deb package structure from Flatpak...${NC}"
mkdir -p "$DEB_DIR/DEBIAN"
mkdir -p "$DEB_DIR/usr"

# Copy files from Flatpak staging
echo -e "${BLUE}📥 Copying files from Flatpak staging...${NC}"
if [ -d "$FLATPAK_STAGING/bin" ]; then
    echo "  - Copying binaries..."
    cp -r "$FLATPAK_STAGING/bin" "$DEB_DIR/usr/"
fi

if [ -d "$FLATPAK_STAGING/share" ]; then
    echo "  - Copying shared files..."
    cp -r "$FLATPAK_STAGING/share" "$DEB_DIR/usr/"
fi

if [ -d "$FLATPAK_STAGING/lib" ]; then
    echo "  - Copying libraries..."
    cp -r "$FLATPAK_STAGING/lib" "$DEB_DIR/usr/"
fi

# Verify essential files
echo -e "${BLUE}🔍 Verifying essential files...${NC}"
MISSING_FILES=()

if [ ! -f "$DEB_DIR/usr/bin/jair" ]; then
    MISSING_FILES+=("usr/bin/jair")
fi

if [ ! -f "$DEB_DIR/usr/share/jair/jair.gresource" ]; then
    MISSING_FILES+=("usr/share/jair/jair.gresource")
fi

if [ ${#MISSING_FILES[@]} -ne 0 ]; then
    echo -e "${RED}❌ Missing essential files:${NC}"
    for file in "${MISSING_FILES[@]}"; do
        echo -e "${RED}  - $file${NC}"
    done
    exit 1
fi

echo -e "${GREEN}✅ All essential files present${NC}"

# Create control file
echo -e "${BLUE}📝 Creating control file...${NC}"
cat > "$DEB_DIR/DEBIAN/control" << EOF
Package: $APP_NAME
Version: $APP_VERSION
Section: graphics
Priority: optional
Architecture: $ARCH
Depends: libgtk-4-1, libadwaita-1-0, libglib2.0-0, libgdk-pixbuf-2.0-0, libpango-1.0-0, libcairo2, libjpeg-turbo8 | libjpeg8, libpng16-16, libwebp7
Maintainer: LSB <contact@lsb.codes>
Homepage: https://github.com/lsantiagoba/jair
Description: JAIR - Image Resizer
 Image Resizer for Android, iOS, Web, and Social Media.
 .
 JAIR is a powerful and intuitive GTK4/Libadwaita
 application for resizing images. Perfect for developers and designers who need
 to quickly resize images for multiple platforms.
 .
 Features:
  * Smart presets for Android, iOS, Web, and Social Media
  * Batch processing support
  * Custom dimensions
  * PNG and JPEG output formats
  * Modern GNOME/GTK4 interface
  * Multi-language support (English and Spanish)
EOF

# Create postinst script
cat > "$DEB_DIR/DEBIAN/postinst" << 'EOF'
#!/bin/bash
set -e

# Compile GSettings schemas
if [ -d /usr/share/glib-2.0/schemas ]; then
    glib-compile-schemas /usr/share/glib-2.0/schemas || true
fi

# Update icon cache
if [ -d /usr/share/icons/hicolor ]; then
    gtk-update-icon-cache -f -t /usr/share/icons/hicolor 2>/dev/null || true
fi

# Update desktop database
if [ -x /usr/bin/update-desktop-database ]; then
    update-desktop-database -q /usr/share/applications || true
fi

exit 0
EOF

chmod 755 "$DEB_DIR/DEBIAN/postinst"

# Create postrm script
cat > "$DEB_DIR/DEBIAN/postrm" << 'EOF'
#!/bin/bash
set -e

if [ "$1" = "remove" ] || [ "$1" = "purge" ]; then
    # Update icon cache
    if [ -d /usr/share/icons/hicolor ]; then
        gtk-update-icon-cache -f -t /usr/share/icons/hicolor 2>/dev/null || true
    fi

    # Update desktop database
    if [ -x /usr/bin/update-desktop-database ]; then
        update-desktop-database -q /usr/share/applications || true
    fi
fi

exit 0
EOF

chmod 755 "$DEB_DIR/DEBIAN/postrm"

# Calculate installed size
INSTALLED_SIZE=$(du -sk "$DEB_DIR" | cut -f1)
echo "Installed-Size: $INSTALLED_SIZE" >> "$DEB_DIR/DEBIAN/control"

# Build the .deb package
echo -e "${BLUE}🔧 Building .deb package...${NC}"
dpkg-deb --build --root-owner-group "$DEB_DIR" "${APP_NAME}_${APP_VERSION}_${ARCH}.deb"

echo -e "${GREEN}✅ .deb package created successfully!${NC}"
echo ""

# Now build the snap from the .deb
echo -e "${BLUE}📦 Building Snap package...${NC}"

# Check if snapcraft is installed
if ! command -v snapcraft &> /dev/null; then
    echo -e "${RED}❌ Error: snapcraft is not installed${NC}"
    echo -e "${YELLOW}Install it with: sudo snap install snapcraft --classic${NC}"
    exit 1
fi

# Build the snap
snapcraft --verbose

if [ -f "$SNAP_NAME" ]; then
    echo ""
    echo -e "${GREEN}✅ Snap package created successfully!${NC}"
    echo -e "${GREEN}📦 File: $SNAP_NAME${NC}"
    echo ""
    echo -e "${BLUE}To install the snap, run:${NC}"
    echo -e "${YELLOW}  sudo snap install --dangerous $SNAP_NAME${NC}"
else
    echo -e "${RED}❌ Error: Snap package was not created${NC}"
    exit 1
fi

# Clean up temporary files
echo ""
echo -e "${BLUE}🧹 Cleaning temporary files...${NC}"
rm -rf "$DEB_DIR"

echo ""
echo -e "${GREEN}✅ Conversion complete!${NC}"
echo -e "${GREEN}📦 .deb file: ${APP_NAME}_${APP_VERSION}_${ARCH}.deb${NC}"
echo -e "${GREEN}📦 Snap file: $SNAP_NAME${NC}"
