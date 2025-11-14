# System Dependencies

Some examples in this repository require external system libraries or tools to be installed. This document lists those dependencies and how to install them.

## Required for Specific Examples

### gRPC Example (`04-networking/07-grpc`)

**Dependency:** Protocol Buffers Compiler (`protoc`)

**Installation:**

```bash
# macOS
brew install protobuf

# Ubuntu/Debian
sudo apt-get install protobuf-compiler

# Windows
# Download from https://github.com/protocolbuffers/protobuf/releases
# Extract and add protoc.exe to your PATH
```

**Verify:**
```bash
protoc --version
```

### Tauri App Example (`11-gui/05-tauri-app`)

**Dependencies:** GTK3, WebKit2GTK, and related libraries (Linux only)

**Installation:**

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y \
  libgtk-3-dev \
  libwebkit2gtk-4.0-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

# Fedora
sudo dnf install gtk3-devel webkit2gtk3-devel libappindicator-gtk3-devel librsvg2-devel

# Arch
sudo pacman -S gtk3 webkit2gtk libappindicator-gtk3 librsvg
```

**Note:** On macOS and Windows, Tauri uses native webview components, so no additional dependencies are needed.

### Other GUI Examples

Most GUI examples (egui, iced, slint) work without additional system dependencies on all platforms. However, some may benefit from system libraries:

- **Linux:** May require X11 or Wayland development libraries
- **macOS:** No additional dependencies
- **Windows:** No additional dependencies

## CI/CD

The GitHub Actions CI workflow automatically installs these dependencies on Ubuntu runners:

- `libgtk-3-dev`
- `libwebkit2gtk-4.0-dev`
- `libayatana-appindicator3-dev`
- `librsvg2-dev`
- `protobuf-compiler`

## Troubleshooting

### "package 'XYZ' not found" errors

This typically means a required system library is not installed. Check the error message for the package name (e.g., `gobject-2.0`, `webkit2gtk-4.0`) and install the corresponding development package.

### pkg-config errors

Some crates use `pkg-config` to find system libraries. If you see pkg-config errors:

1. Ensure the library is installed
2. Ensure `pkg-config` itself is installed
3. Check that `PKG_CONFIG_PATH` includes the directory containing the `.pc` file

```bash
# Install pkg-config
# Ubuntu/Debian
sudo apt-get install pkg-config

# macOS
brew install pkg-config
```

### Windows-specific issues

On Windows, some examples may require additional setup:

1. For GTK-based apps, you may need to install GTK for Windows
2. For protobuf, download the Windows binaries and add to PATH
3. Some examples may work better with MSVC toolchain than GNU toolchain

## Platform-Specific Notes

### Linux

Most dependencies are available through package managers. Development packages typically end with `-dev` (Debian/Ubuntu) or `-devel` (Fedora/RHEL).

### macOS

Use Homebrew for most dependencies. Some GUI frameworks may require Xcode Command Line Tools:

```bash
xcode-select --install
```

### Windows

Many Rust GUI frameworks work out of the box on Windows. For those requiring C libraries, consider using:

- MSYS2 for Unix-like libraries
- vcpkg for Windows-native libraries
- Pre-built binaries from official sources
