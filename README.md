# JAIR - Image Resizer

A universal image resizer for Android, iOS, Web, and Social Media built with GTK4 and Rust.

## Overview

JAIR is a powerful desktop application that helps developers and designers resize images for multiple platforms. Whether you're developing for Android, iOS, building websites, or managing social media, JAIR generates all the required image sizes automatically with professional quality.

**What makes JAIR special?**
- **Smart Presets**: Instantly resize images to platform-specific dimensions without remembering exact sizes
- **Custom Sizes**: Need a specific dimension? Just enter your custom width and height
- **Batch Processing**: Process multiple images at once, saving hours of manual work
- **Professional Quality**: Uses advanced Lanczos3 filtering for crystal-clear results
- **Native Experience**: Built with GTK4 and libadwaita for a beautiful, fast, and responsive interface
- **Developer-Friendly**: Organized output structure with properly named folders ready for your projects

## Features

### 🤖 Android Platform
- **Launcher Icons** (Legacy) - ldpi through xxxhdpi
- **Adaptive Icons** - Modern Android icon format
- **Play Store Graphics** - Store icon, Feature graphic, Promo graphic
- **Screenshots** - Phone, 7" Tablet, 10" Tablet formats
- **TV Banner** - For Android TV apps
- **Complete Android Pack** - All formats in one go

### 🍎 iOS / Apple Platform
- **App Icons** - iPhone, iPad, iPad Pro, App Store (1024×1024)
- **Spotlight & Settings Icons** - All required sizes
- **Notification Icons** - For iOS notifications
- **Screenshots** - iPhone (6.7", 6.5", 5.5") and iPad (12.9", 11")
- **Apple Watch Icons** - Complete set for watchOS
- **macOS Icons** - From 16×16 to 1024×1024
- **Complete Apple Pack** - iOS + watchOS + macOS

### 🌐 Web & Social Media
- **Social Profile Pictures** - Facebook, Twitter, Instagram, LinkedIn, YouTube
- **Social Covers/Banners** - For all major platforms
- **Social Media Posts** - Instagram, Facebook, Twitter, LinkedIn, Pinterest, TikTok
- **Web Favicons** - From 16×16 to 256×256
- **Open Graph Images** - For social sharing
- **HD Resolutions** - 720p, 1080p, 1440p, 4K
- **Blog Images** - Featured images, inline images, thumbnails
- **E-commerce** - Product images from mini to zoom quality
- **Email Newsletter** - Header, banner, and thumbnail sizes

### ⚡ General Features
- **Multi-Platform Support**: Switch between Android, iOS, Web/Social, and Custom presets
- **Custom Dimensions**: Enter any width and height for one-off or specialized sizes
- **Batch Processing**: Process multiple images at once
- **High-Quality Resizing**: Uses Lanczos3 filter for optimal quality
- **Output Formats**: Choose between PNG (lossless) or JPEG (smaller files)
- **Organized Output**: Creates separate folders for each size variant
- **Progress Tracking**: Real-time progress bar and status updates
- **Modern UI**: Built with GTK4 and libadwaita for a native GNOME experience
- **Flexible Workflow**: Use presets, custom sizes, or combine both in one operation

## Prerequisites

Before building JAIR, you need to install the following dependencies:

### Ubuntu/Debian

```bash
sudo apt install -y \
    libgtk-4-dev \
    libadwaita-1-dev \
    meson \
    ninja-build \
    rustc \
    cargo \
    gettext
```

### Fedora

```bash
sudo dnf install -y \
    gtk4-devel \
    libadwaita-devel \
    meson \
    ninja-build \
    rust \
    cargo \
    gettext
```

### Arch Linux

```bash
sudo pacman -S \
    gtk4 \
    libadwaita \
    meson \
    ninja \
    rust \
    gettext
```

## Building

### Option 1: Using GNOME Builder (Recommended)

1. Open GNOME Builder
2. Click "Clone Repository" or "Open Project"
3. Select the JAIR project directory
4. Click the "Build" button (hammer icon) or press Ctrl+Shift+B
5. Click "Run" button (play icon) or press Ctrl+F5

GNOME Builder will automatically handle all dependencies through Flatpak.

### Option 2: Building Manually

1. Clone the repository or navigate to the project directory

2. Setup the build directory:
```bash
meson setup builddir
```

3. Compile the project:
```bash
meson compile -C builddir
```

4. Run the application:
```bash
./builddir/src/jair
```

**Note:** For manual building, you must install the development libraries first (see Prerequisites section).

## Installation

### Option 1: Install from Snap

```bash
sudo snap install jair_1.0.0_amd64.snap --dangerous
```

After installation, you need to connect the `mount-observe` permission to allow JAIR to access your file system when selecting images:

```bash
sudo snap connect jair:mount-observe :mount-observe
```

This permission is required for the file picker dialog to work properly.

Then run from your application menu or with:
```bash
jair
```

### Option 2: Install from DEB Package

```bash
sudo dpkg -i jair_1.0.0_amd64.deb
```

### Option 3: Install from Source

To install JAIR system-wide from source:

```bash
meson install -C builddir
```

Then you can run it from your application menu or with:
```bash
jair
```

## Usage

1. **Select Platform**: Choose your target platform (Android, iOS/Apple, or Web & Social Media)

2. **Select Preset**: Choose which format sizes you need from the dropdown menu
   - Android: Launcher icons, Play Store graphics, screenshots, etc.
   - iOS/Apple: App icons, screenshots, Watch icons, macOS icons, etc.
   - Web & Social: Profile pictures, posts, favicons, blog images, etc.

3. **Add Images**: Click "Add Images" to select one or more images to resize

4. **Choose Format**: Toggle between PNG (lossless) and JPEG (smaller files) output format

5. **Process Images**: Click "Resize Images" and select an output directory

6. **Done!**: Your images will be resized and organized in folders by size

## Project Structure

```
JAIR/
├── src/
│   ├── main.rs              # Application entry point
│   ├── application.rs       # GTK Application setup
│   ├── window.rs            # Main window implementation
│   ├── window.ui            # GTK UI definition
│   ├── models/
│   │   ├── mod.rs           # Models module
│   │   ├── size.rs          # Size struct definition
│   │   └── android_sizes.rs # Android preset definitions
│   └── services/
│       ├── mod.rs           # Services module
│       └── processor.rs     # Image processing logic
├── data/                    # Desktop files and icons
├── po/                      # Translations
├── meson.build             # Build configuration
└── Cargo.toml              # Rust dependencies
```

## Dependencies

### Rust Crates

- **gtk4** (0.7) - GTK 4 bindings for Rust
- **libadwaita** (0.5) - GNOME libadwaita bindings
- **image** (0.24) - Image processing library
- **rayon** (1.7) - Data parallelism for batch processing
- **anyhow** (1.0) - Error handling
- **gettext-rs** (0.7) - Internationalization support

## Output Structure

When you process images, JAIR creates the following directory structure:

```
output_directory/
├── mdpi/
│   └── your-image-mdpi.png
├── hdpi/
│   └── your-image-hdpi.png
├── xhdpi/
│   └── your-image-xhdpi.png
├── xxhdpi/
│   └── your-image-xxhdpi.png
├── xxxhdpi/
│   └── your-image-xxxhdpi.png
└── feature-graphic/
    └── your-image-feature-graphic.png
```

Each folder contains the resized images at the appropriate dimensions for that Android density/format.

## Platform Format Reference

### 🤖 Android Formats

**Launcher Icons (Legacy)**
- ldpi: 36×36 | mdpi: 48×48 | hdpi: 72×72
- xhdpi: 96×96 | xxhdpi: 144×144 | xxxhdpi: 192×192

**Adaptive Icons**
- mdpi: 81×81 | hdpi: 108×108 | xhdpi: 162×162
- xxhdpi: 216×216 | xxxhdpi: 324×324

**Play Store Graphics**
- Store Icon: 512×512
- Feature Graphic: 1024×500
- Promo Graphic: 180×120
- TV Banner: 1280×720

**Screenshots**
- Phone: 1080×1920 (portrait), 1920×1080 (landscape)
- 7" Tablet: 1200×1920 (portrait), 1920×1200 (landscape)
- 10" Tablet: 1600×2560 (portrait), 2560×1600 (landscape)

### 🍎 iOS / Apple Formats

**App Icons**
- iPhone: 120×120 (@2x), 180×180 (@3x)
- iPad: 76×76 (@1x), 152×152 (@2x)
- iPad Pro: 167×167 (@2x)
- App Store: 1024×1024

**Screenshots**
- iPhone 6.7": 1290×2796 (portrait), 2796×1290 (landscape)
- iPhone 6.5": 1242×2688 (portrait), 2688×1242 (landscape)
- iPhone 5.5": 1242×2208 (portrait), 2208×1242 (landscape)
- iPad 12.9": 2048×2732 (portrait), 2732×2048 (landscape)
- iPad 11": 1668×2388 (portrait), 2388×1668 (landscape)

**Apple Watch Icons**
- Notification: 48×48, 55×55
- Home Screen: 80×80, 88×88, 92×92, 100×100
- Short Look: 172×172, 196×196, 216×216
- App Store: 1024×1024

**macOS Icons**
- 16×16 (@1x/@2x), 32×32 (@1x/@2x)
- 128×128 (@1x/@2x), 256×256 (@1x/@2x)
- 512×512 (@1x/@2x)

### 🌐 Web & Social Media Formats

**Social Profile Pictures**
- Facebook: 180×180 | Twitter: 400×400
- Instagram: 320×320 | LinkedIn: 300×300
- YouTube: 800×800

**Social Covers/Banners**
- Facebook: 820×312
- Twitter: 1500×500
- LinkedIn: 1584×396
- YouTube: 2560×1440

**Social Media Posts**
- Instagram Square: 1080×1080
- Instagram Portrait: 1080×1350
- Instagram Landscape: 1080×608
- Instagram/Facebook Story: 1080×1920
- Facebook Post: 1200×630
- Twitter Post: 1200×675
- LinkedIn Post: 1200×627
- Pinterest Pin: 1000×1500
- TikTok Video: 1080×1920

**Web Formats**
- Favicons: 16×16, 32×32, 48×48, 64×64, 128×128, 256×256
- Open Graph: 1200×630 (general)
- HD Resolutions: 720p, 1080p, 1440p, 4K

**E-commerce**
- Product Zoom: 2000×2000
- Product Large: 1000×1000
- Product Medium: 500×500
- Product Thumbnail: 250×250
- Product Mini: 100×100

## License

MIT License

Copyright (c) 2025 Leandro Santiago

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

If you encounter any issues or have questions, please open an issue on the project repository.
