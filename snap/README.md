# JAIR Snap Package

## Installation

### From local file
```bash
sudo snap install jair_1.0.0_amd64.snap --dangerous
```

### From Snapcraft Store (when published)
```bash
sudo snap install jair
```

## Building

### Prerequisites
```bash
sudo snap install snapcraft --classic
```

### Build the snap
```bash
snapcraft
```

This will create the `jair_1.0.0_amd64.snap` file.

## Permissions

The snap requires the following permissions:

- **home**: Access to your home directory to read/save images
- **removable-media**: Access to removable devices (USB, etc.)
- **desktop**: Desktop integration
- **wayland/x11**: Support for graphical servers
- **gsettings**: Save application preferences

### Connect permissions manually (if needed)
```bash
sudo snap connect jair:home
sudo snap connect jair:removable-media
```

## Usage

```bash
# Run the application
jair

# View logs
snap logs jair

# View logs in real-time
snap logs -f jair

# With debug enabled
RUST_LOG=debug jair
```

## Uninstallation

```bash
sudo snap remove jair
```

## Publishing to Snapcraft Store

1. Create account at https://snapcraft.io
2. Login:
```bash
snapcraft login
```

3. Register the name:
```bash
snapcraft register jair
```

4. Upload the snap:
```bash
snapcraft upload jair_1.0.0_amd64.snap --release=edge
```

5. Promote to stable channel:
```bash
snapcraft release jair <revision> stable
```

## Snap Structure

```
snap/
├── snapcraft.yaml       # Main snap configuration
├── hooks/
│   ├── install         # Installation hook
│   └── configure       # Configuration hook
└── gui/
    └── icon.svg        # Application icon
```

## Multi-architecture Support

The snap supports:
- amd64 (x86_64)
- arm64 (aarch64)

## Known Issues

If you encounter file permission issues, make sure permissions are connected:
```bash
snap connections jair
```

## Development

For local development, you can use:
```bash
snapcraft --use-lxd  # If you prefer LXD
snapcraft --debug    # For debugging
snapcraft clean      # Clean previous builds
```
