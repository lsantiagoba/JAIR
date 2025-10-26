#!/bin/bash

# BUILD_DEB.sh - Script to build a .deb package for JAIR

set -e  # Exit on error

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}======================================${NC}"
echo -e "${BLUE}  JAIR - Debian Package Builder${NC}"
echo -e "${BLUE}======================================${NC}"
echo ""

# Variables
APP_NAME="jair"
APP_VERSION="1.0.0"
ARCH=$(dpkg --print-architecture)
BUILD_DIR="build"
DEB_DIR="debian_package"
INSTALL_PREFIX="/usr"

# Check dependencies
echo -e "${BLUE}🔍 Checking dependencies...${NC}"

DEPENDENCIES=(
    "meson"
    "ninja-build"
    "cargo"
    "rustc"
    "pkg-config"
    "libgtk-4-dev"
    "libadwaita-1-dev"
    "gettext"
)

MISSING_DEPS=()
for dep in "${DEPENDENCIES[@]}"; do
    if ! dpkg -l | grep -q "^ii  $dep"; then
        MISSING_DEPS+=("$dep")
    fi
done

if [ ${#MISSING_DEPS[@]} -ne 0 ]; then
    echo -e "${YELLOW}⚠️  Missing dependencies: ${MISSING_DEPS[*]}${NC}"
    echo -e "${YELLOW}Installing dependencies...${NC}"
    sudo apt update
    sudo apt install -y "${MISSING_DEPS[@]}"
fi

echo -e "${GREEN}✅ All dependencies are installed${NC}"
echo ""

# Clean previous builds
echo -e "${BLUE}🧹 Cleaning previous builds...${NC}"
rm -rf "$BUILD_DIR" "$DEB_DIR" "${APP_NAME}_${APP_VERSION}_${ARCH}.deb"

# Configure with meson
echo -e "${BLUE}⚙️  Configuring project with Meson...${NC}"
meson setup "$BUILD_DIR" --prefix="$INSTALL_PREFIX" --buildtype=release

# Compile
echo -e "${BLUE}🔨 Compiling...${NC}"
meson compile -C "$BUILD_DIR"

# Create debian package structure
echo -e "${BLUE}📦 Creating .deb package structure...${NC}"
mkdir -p "$DEB_DIR/DEBIAN"
mkdir -p "$DEB_DIR$INSTALL_PREFIX"

# Install to temporary directory
echo -e "${BLUE}📥 Installing files to temporary directory...${NC}"
DESTDIR="$(pwd)/$DEB_DIR" meson install -C "$BUILD_DIR"

# Create control file
echo -e "${BLUE}📝 Creating control file...${NC}"
cat > "$DEB_DIR/DEBIAN/control" << EOF
Package: $APP_NAME
Version: $APP_VERSION
Section: graphics
Priority: optional
Architecture: $ARCH
Depends: libgtk-4-1, libadwaita-1-0, libglib2.0-0, libgdk-pixbuf-2.0-0, libpango-1.0-0, libcairo2, libjpeg-turbo8 | libjpeg8, libpng16-16, libwebp7
Maintainer: LSB <lsantiagoba@example.com>
Homepage: https://github.com/lsantiagoba/jair
Description: JAIR -  Image Resizer
 Image Resizer for Android, iOS, Web, and Social Media.
 .
 JAIR ( is a powerful and intuitive GTK4/Libadwaita
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

# Create postinst script to update caches
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

# Create postrm script for cleanup
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

# Clean up
echo -e "${BLUE}🧹 Cleaning temporary files...${NC}"
rm -rf "$DEB_DIR"

# Verify the package
echo ""
echo -e "${GREEN}✅ .deb package created successfully!${NC}"
echo -e "${GREEN}📦 File: ${APP_NAME}_${APP_VERSION}_${ARCH}.deb${NC}"
echo ""
echo -e "${BLUE}📊 Package information:${NC}"
dpkg-deb --info "${APP_NAME}_${APP_VERSION}_${ARCH}.deb"
echo ""
echo -e "${BLUE}📁 Package contents:${NC}"
dpkg-deb --contents "${APP_NAME}_${APP_VERSION}_${ARCH}.deb"
echo ""
echo -e "${GREEN}To install the package, run:${NC}"
echo -e "${YELLOW}  sudo dpkg -i ${APP_NAME}_${APP_VERSION}_${ARCH}.deb${NC}"
echo -e "${YELLOW}  sudo apt-get install -f  ${NC}${BLUE}# If dependencies are missing${NC}"
