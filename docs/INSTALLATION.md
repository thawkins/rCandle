# rCandle Installation Guide

**Version**: 0.1.0-alpha  
**Last Updated**: January 2025

## Table of Contents

1. [System Requirements](#system-requirements)
2. [Windows Installation](#windows-installation)
3. [Linux Installation](#linux-installation)
4. [macOS Installation](#macos-installation)
5. [Building from Source](#building-from-source)
6. [Post-Installation Setup](#post-installation-setup)
7. [Updating rCandle](#updating-rcandle)
8. [Uninstallation](#uninstallation)

## System Requirements

### Minimum Requirements
- **CPU**: Dual-core 2.0 GHz or faster
- **RAM**: 4 GB
- **Storage**: 100 MB free space
- **Graphics**: GPU with Vulkan, DirectX 12, or Metal support
- **USB**: One available USB port for GRBL controller
- **Display**: 1024x768 minimum resolution

### Recommended Requirements
- **CPU**: Quad-core 2.5 GHz or faster
- **RAM**: 8 GB
- **Storage**: 500 MB free space (for logs and cached files)
- **Graphics**: Dedicated GPU with updated drivers
- **Display**: 1920x1080 or higher

### Supported Operating Systems
- **Windows**: Windows 10 (build 1809+) or Windows 11
- **Linux**: Ubuntu 20.04+, Fedora 34+, Debian 11+, Arch Linux (current)
- **macOS**: macOS 11 (Big Sur) or later

## Windows Installation

### Method 1: Installer (Recommended)

1. **Download the Installer**
   - Visit the releases page
   - Download `rcandle-windows-x64-installer.exe`
   - Verify checksum (recommended)

2. **Run the Installer**
   - Double-click the downloaded file
   - If SmartScreen appears, click "More info" → "Run anyway"
   - Follow the installation wizard
   - Choose installation directory (default: `C:\Program Files\rCandle`)
   - Select Start Menu folder
   - Click "Install"

3. **Launch rCandle**
   - From Start Menu: `rCandle`
   - From Desktop shortcut (if selected)
   - From installation directory

### Method 2: Portable Version

1. **Download the Archive**
   - Download `rcandle-windows-x64-portable.zip`
   - Verify checksum

2. **Extract the Archive**
   - Extract to your preferred location
   - No installation required

3. **Run rCandle**
   - Double-click `rcandle.exe`
   - Create shortcut if desired

### Required Drivers

**USB Serial Drivers**:
- Most USB-serial adapters install drivers automatically
- For FTDI devices: Download from [FTDI website](https://ftdichip.com/drivers/)
- For CH340/CH341: Usually auto-installed by Windows Update

**Graphics Drivers**:
- Update graphics drivers to latest version
- NVIDIA: [GeForce Experience](https://www.nvidia.com/en-us/geforce/geforce-experience/)
- AMD: [AMD Software](https://www.amd.com/en/support)
- Intel: [Intel Driver & Support Assistant](https://www.intel.com/content/www/us/en/support/intel-driver-support-assistant.html)

### Troubleshooting Windows Installation

**"Windows protected your PC" message**:
- Click "More info"
- Click "Run anyway"
- This appears because the app isn't signed (will be addressed in future release)

**Installation fails**:
- Run installer as Administrator (right-click → "Run as administrator")
- Temporarily disable antivirus
- Ensure sufficient disk space
- Check Windows Event Viewer for errors

## Linux Installation

### Method 1: AppImage (Recommended for most distros)

1. **Download AppImage**
   ```bash
   wget https://github.com/yourusername/rCandle/releases/download/v0.1.0/rcandle-linux-x86_64.AppImage
   ```

2. **Make Executable**
   ```bash
   chmod +x rcandle-linux-x86_64.AppImage
   ```

3. **Run rCandle**
   ```bash
   ./rcandle-linux-x86_64.AppImage
   ```

4. **Optional: Install FUSE** (if not already installed)
   ```bash
   # Ubuntu/Debian
   sudo apt install fuse libfuse2
   
   # Fedora
   sudo dnf install fuse fuse-libs
   
   # Arch
   sudo pacman -S fuse2
   ```

### Method 2: Debian/Ubuntu Package

1. **Download .deb Package**
   ```bash
   wget https://github.com/yourusername/rCandle/releases/download/v0.1.0/rcandle_0.1.0_amd64.deb
   ```

2. **Install Package**
   ```bash
   sudo dpkg -i rcandle_0.1.0_amd64.deb
   sudo apt-get install -f  # Install dependencies if needed
   ```

3. **Launch rCandle**
   ```bash
   rcandle
   # Or from application menu
   ```

### Method 3: Arch Linux (AUR)

1. **Install from AUR**
   ```bash
   yay -S rcandle-bin
   # Or with paru
   paru -S rcandle-bin
   ```

2. **Launch rCandle**
   ```bash
   rcandle
   ```

### Post-Installation (Linux)

**Serial Port Permissions**:
```bash
# Add user to dialout group (required for serial port access)
sudo usermod -a -G dialout $USER

# Log out and log back in for changes to take effect
# Or use: newgrp dialout
```

**Verify Serial Port Access**:
```bash
ls -l /dev/ttyUSB* /dev/ttyACM*
# Should show your user in the group
```

**Graphics Setup**:
```bash
# Install Vulkan support (if not already installed)
# Ubuntu/Debian
sudo apt install vulkan-tools vulkan-icd

# Fedora
sudo dnf install vulkan-tools vulkan-loader

# Arch
sudo pacman -S vulkan-tools vulkan-icd-loader

# Verify Vulkan
vulkaninfo | grep "deviceName"
```

### Desktop Integration (AppImage)

**Add to Application Menu**:
```bash
# Download and run AppImageLauncher
# Or manually create .desktop file
mkdir -p ~/.local/share/applications
cat > ~/.local/share/applications/rcandle.desktop << EOF
[Desktop Entry]
Type=Application
Name=rCandle
Comment=GRBL CNC Controller
Exec=/path/to/rcandle-linux-x86_64.AppImage
Icon=/path/to/icon.png
Terminal=false
Categories=Development;Engineering;
EOF
```

## macOS Installation

### Method 1: DMG Installer (Recommended)

1. **Download DMG**
   - Download `rcandle-macos-universal.dmg`
   - Verify checksum

2. **Mount and Install**
   - Double-click the DMG file
   - Drag rCandle to Applications folder
   - Eject the DMG

3. **First Launch**
   - Open Applications folder
   - Right-click rCandle → Open (first time only)
   - Click "Open" in security dialog
   - Subsequent launches can use double-click

### Method 2: Homebrew

```bash
# Tap repository
brew tap yourusername/rcandle

# Install rCandle
brew install rcandle

# Launch
rcandle
```

### Post-Installation (macOS)

**Allow Unsigned Application**:
- System Preferences → Security & Privacy
- Click lock to make changes
- Allow rCandle to run
- May need to approve on first launch

**Serial Port Drivers**:
- Most USB-serial adapters work automatically
- For FTDI: Download from [FTDI website](https://ftdichip.com/drivers/)
- For CH340: Download from manufacturer

**Graphics Requirements**:
- Requires Metal support (macOS 11+)
- Update macOS to latest version
- No additional driver installation needed

## Building from Source

### Prerequisites

**Install Rust** (1.75 or later):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Platform-Specific Dependencies**:

**Linux**:
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install build-essential pkg-config libudev-dev

# Fedora
sudo dnf install gcc pkg-config libudev-devel

# Arch
sudo pacman -S base-devel pkg-config udev
```

**Windows**:
- Install [Visual Studio 2019 or later](https://visualstudio.microsoft.com/)
- Select "Desktop development with C++"
- Or install [Build Tools for Visual Studio](https://visualstudio.microsoft.com/downloads/)

**macOS**:
```bash
xcode-select --install
```

### Build Steps

1. **Clone Repository**
   ```bash
   git clone https://github.com/yourusername/rCandle.git
   cd rCandle
   ```

2. **Build Release Version**
   ```bash
   cargo build --release
   ```
   
   Build time: 5-15 minutes depending on system

3. **Run rCandle**
   ```bash
   ./target/release/rcandle  # Linux/macOS
   target\release\rcandle.exe  # Windows
   ```

4. **Install System-Wide** (Optional)
   ```bash
   # Linux
   sudo cp target/release/rcandle /usr/local/bin/
   
   # macOS
   sudo cp target/release/rcandle /usr/local/bin/
   ```

### Development Build

For development with faster compilation:
```bash
cargo build
./target/debug/rcandle
```

### Build Troubleshooting

**Compilation Errors**:
- Ensure Rust version is 1.75+: `rustc --version`
- Update Rust: `rustup update`
- Clean and rebuild: `cargo clean && cargo build --release`

**Linking Errors**:
- Install missing system libraries
- Check platform-specific requirements above
- Verify CMake is installed: `cmake --version`

## Post-Installation Setup

### First Run

On first launch, rCandle will:
1. Create configuration directory
2. Generate default settings
3. Create log directory
4. May prompt for permissions (graphics, serial ports)

### Configuration Locations

**Windows**:
- Config: `%APPDATA%\rCandle\config.json`
- Logs: `%APPDATA%\rCandle\logs\`
- Scripts: `%APPDATA%\rCandle\scripts\`

**Linux**:
- Config: `~/.config/rCandle/config.json`
- Logs: `~/.local/share/rCandle/logs/`
- Scripts: `~/.local/share/rCandle/scripts/`

**macOS**:
- Config: `~/Library/Application Support/rCandle/config.json`
- Logs: `~/Library/Application Support/rCandle/logs/`
- Scripts: `~/Library/Application Support/rCandle/scripts/`

### Verify Installation

1. **Launch Application**
   - Application window should open
   - UI should be responsive

2. **Check Graphics**
   - 3D view should display grid and axes
   - Camera controls should work

3. **Check Serial Ports**
   - Connect GRBL controller
   - Port should appear in dropdown

4. **Test Basic Functions**
   - Open a sample G-Code file
   - Verify 3D visualization works
   - Check console displays messages

## Updating rCandle

### Installer Method

1. Download latest installer
2. Run installer (will upgrade existing installation)
3. Your settings and configurations are preserved

### AppImage Method

1. Download new AppImage
2. Replace old AppImage
3. Make executable: `chmod +x rcandle-new.AppImage`

### Package Manager Method

```bash
# Debian/Ubuntu
sudo apt update
sudo apt upgrade rcandle

# Arch (AUR)
yay -Syu rcandle-bin

# Homebrew (macOS)
brew upgrade rcandle
```

### From Source

```bash
cd rCandle
git pull
cargo build --release
```

### Settings Migration

Settings are automatically migrated between versions when possible. Backup your config file before major updates:

```bash
# Linux/macOS
cp ~/.config/rCandle/config.json ~/.config/rCandle/config.json.backup

# Windows
copy %APPDATA%\rCandle\config.json %APPDATA%\rCandle\config.json.backup
```

## Uninstallation

### Windows

**Installer Version**:
1. Control Panel → Programs → Uninstall a program
2. Select rCandle
3. Click Uninstall

**Portable Version**:
- Simply delete the rCandle folder

**Remove User Data** (optional):
- Delete `%APPDATA%\rCandle\` folder

### Linux

**AppImage**:
- Delete the AppImage file
- Remove desktop integration files if created

**Debian Package**:
```bash
sudo apt remove rcandle
```

**AUR**:
```bash
yay -R rcandle-bin
```

**Remove User Data** (optional):
```bash
rm -rf ~/.config/rCandle
rm -rf ~/.local/share/rCandle
```

### macOS

1. Drag rCandle from Applications to Trash
2. Empty Trash

**Remove User Data** (optional):
```bash
rm -rf ~/Library/Application\ Support/rCandle
rm -rf ~/Library/Preferences/rCandle
rm -rf ~/Library/Caches/rCandle
```

## Troubleshooting Installation

### Installation Fails

- Check system requirements
- Verify sufficient disk space
- Check antivirus/firewall settings
- Try running installer as administrator (Windows)
- Check installation logs

### Application Won't Start

- Update graphics drivers
- Check error messages in terminal/logs
- Verify all dependencies installed
- Try software rendering mode
- See [Troubleshooting Guide](TROUBLESHOOTING.md)

### Serial Port Not Working

- Install USB-serial drivers
- Check permissions (Linux: dialout group)
- Verify device is connected
- Try different USB port/cable

## Getting Help

If you encounter issues during installation:

1. Check [Troubleshooting Guide](TROUBLESHOOTING.md)
2. Review [User Guide](USER_GUIDE.md)
3. Search existing GitHub issues
4. Create new issue with:
   - Operating system and version
   - Installation method used
   - Error messages
   - System specifications

## Additional Resources

- **User Guide**: Complete usage instructions
- **Troubleshooting**: Common problems and solutions
- **Keyboard Shortcuts**: Complete shortcut reference
- **GitHub Issues**: Bug reports and feature requests
- **Discussions**: Community support and tips

---

**Installation complete? Check out the [User Guide](USER_GUIDE.md) to get started!**
