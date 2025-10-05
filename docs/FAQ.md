# rCandle Frequently Asked Questions (FAQ)

**Version**: 0.1.0-alpha  
**Last Updated**: January 2025

## Table of Contents

- [General Questions](#general-questions)
- [Compatibility](#compatibility)
- [Features and Functionality](#features-and-functionality)
- [Installation and Setup](#installation-and-setup)
- [Usage Questions](#usage-questions)
- [Troubleshooting](#troubleshooting)
- [Development and Contributing](#development-and-contributing)

## General Questions

### What is rCandle?

rCandle is a modern GRBL controller application written in Rust. It's a reimplementation of the popular Candle CNC controller with improved performance, safety, and maintainability. It provides comprehensive G-Code editing, 3D visualization, and machine control for CNC mills, routers, and laser cutters running GRBL firmware.

### Why "rCandle"?

The "r" stands for Rust, the programming language used to build it. It's a modern reimplementation of the original Candle application.

### Is rCandle free?

Yes! rCandle is open-source software licensed under GPL-3.0, the same license as the original Candle. It's free to use, modify, and distribute.

### How is rCandle different from the original Candle?

- **Language**: Written in Rust instead of C++/Qt
- **Graphics**: Uses modern WGPU (Vulkan/Metal/DX12) instead of OpenGL 2.0
- **UI Framework**: Uses egui (immediate mode) instead of Qt
- **Memory Safety**: Rust's compiler guarantees prevent many common bugs
- **Scripting**: More powerful scripting engine (Rhai)
- **Performance**: Optimized for modern systems

### Is rCandle production-ready?

rCandle is currently in alpha (v0.1.0). While most core features work well, it's recommended to:
- Test thoroughly before using on production work
- Keep the original Candle or another controller as backup
- Report any issues you encounter

### What does "alpha" mean?

Alpha means the software is feature-complete but still undergoing testing. There may be bugs, and features might change before the stable 1.0 release.

## Compatibility

### What CNC machines work with rCandle?

rCandle works with any CNC machine running GRBL firmware, including:
- 3-axis CNC mills
- CNC routers
- Laser engravers/cutters
- Plasma cutters (with GRBL support)
- 3D printers (running GRBL)

### What GRBL versions are supported?

rCandle supports:
- GRBL 1.1 (recommended)
- GRBL 0.9
- GRBL 1.0
- grblHAL
- Some features require GRBL 1.1+

### What G-Code formats are supported?

rCandle supports standard G-Code formats:
- `.nc` files
- `.gcode` files
- `.tap` files
- `.txt` files (if containing G-Code)

### Can I use rCandle with [insert controller brand]?

If your controller runs GRBL firmware, yes! Common compatible controllers:
- Arduino + GRBL Shield
- Arduino Uno + CNC Shield
- Arduino Mega + RAMPS
- grblHAL boards
- Protoneer CNC boards
- Woodpecker boards
- Many Chinese 3-axis boards

### What operating systems are supported?

- Windows 10 (build 1809+) and Windows 11
- Linux (Ubuntu 20.04+, Fedora, Debian, Arch)
- macOS 11 (Big Sur) and later

### Can I run rCandle on Raspberry Pi?

Not currently. rCandle requires:
- WGPU-compatible GPU (Vulkan/Metal/DX12)
- Sufficient RAM and CPU
- Raspberry Pi 4 *might* work with Vulkan drivers, but untested

### Does rCandle work with non-GRBL firmwares?

No. rCandle is specifically designed for GRBL. For other firmwares:
- Marlin: Use Pronterface, OctoPrint
- Mach3/4: Use their respective software
- LinuxCNC: Use Axis or other LinuxCNC UIs

## Features and Functionality

### Can I edit G-Code in rCandle?

Yes! rCandle includes a full G-Code editor with:
- Syntax highlighting
- Line numbers
- Find and replace
- Real-time visualization

### Does rCandle support 4-axis or 5-axis machines?

Currently, rCandle supports 3-axis machines (X, Y, Z). Support for additional axes may come in future versions.

### Can I use rCandle for laser engraving?

Yes! GRBL 1.1+ includes laser mode. rCandle supports:
- M3/M4/M5 spindle/laser commands
- Variable power (S parameter)
- Laser mode enable/disable

### Does rCandle support tool changes?

Basic tool change support is included (M6 command recognition), but advanced tool management (tool library, automatic tool length measurement) is planned for future releases.

### Can I create G-Code in rCandle?

No. rCandle is a controller and editor, not a CAM software. Create G-Code with:
- CAM software (Fusion 360, FreeCAD, etc.)
- Online CAM tools (JSCut, MakerCAM)
- Text editor for simple patterns

### What scripting language does rCandle use?

rCandle uses Rhai, a simple scripting language similar to JavaScript/Rust. Scripts can automate common tasks, create custom macros, and interact with the machine.

### Can I customize keyboard shortcuts?

Customizable keyboard shortcuts are planned for a future release. Currently, shortcuts are fixed.

### Does rCandle support probing?

Basic probing command support exists, but advanced probing features (work piece measuring, tool length, etc.) are planned for future releases.

## Installation and Setup

### Do I need to install drivers?

**Windows**: Usually automatic, but you may need:
- FTDI drivers for FTDI USB-serial adapters
- CH340 drivers for Chinese Arduino clones

**Linux**: No drivers needed, but you must add your user to the `dialout` group

**macOS**: May need FTDI driver for some adapters

### Why can't I access the serial port?

**Windows**: 
- Check Device Manager for COM port
- Close other programs using the port

**Linux**: 
- Run: `sudo usermod -a -G dialout $USER`
- Log out and back in

**macOS**:
- Check System Information for USB device
- May need driver installation

### How much disk space does rCandle need?

- Application: ~10-20 MB
- Configuration/logs: ~1-10 MB
- Recommended free space: 100 MB

### Can I run rCandle from a USB drive?

Yes! Use the portable version (Windows) or AppImage (Linux). Your settings will be stored on the USB drive.

### Does rCandle require internet access?

No. rCandle works completely offline once installed.

## Usage Questions

### How do I connect to my CNC machine?

1. Connect controller via USB
2. Select port from dropdown
3. Choose baud rate (usually 115200)
4. Click "Connect"

### What baud rate should I use?

Most GRBL controllers use **115200**. If connection fails, try:
- 115200 (most common)
- 57600
- 38400
- 19200

Check your GRBL settings with `$$` command.

### How do I home my machine?

1. Ensure machine has clear path to home
2. Click "Home All" button
3. Wait for completion
4. Machine should be in "Idle" state

**Note**: Requires limit switches and homing enabled in GRBL (`$22=1`)

### What's the difference between WPos and MPos?

- **MPos (Machine Position)**: Absolute position from home switches
- **WPos (Work Position)**: Position relative to work zero (G54-G59)

Set work zero with "Zero X/Y/Z" buttons.

### How do I pause a running program?

Click **Pause** button or press `P`. The machine will decelerate to a stop (feed hold). Click **Resume** to continue.

### Can I stop a program mid-execution?

Yes! Click **Stop** button or press `S`. This sends a soft reset to GRBL, stopping all movement immediately.

### What are override controls?

Override controls let you adjust speeds in real-time without stopping:
- **Feed Override**: 10-200% of programmed feed rate
- **Spindle Override**: 10-200% of programmed spindle speed
- **Rapid Override**: 25%, 50%, or 100% of rapid speed

Great for testing or slowing down difficult cuts.

### How do I know if my G-Code has errors?

rCandle validates G-Code when loading. Errors appear:
- Highlighted in red in the editor
- Displayed in the console
- Prevent program execution

### Can I send custom commands to GRBL?

Yes! Use the console:
1. Switch to Console tab
2. Type GRBL command (e.g., `$$`, `$H`, `G0 X10`)
3. Press Enter

### What do the different machine states mean?

- **Idle**: Ready for commands
- **Run**: Executing G-Code
- **Hold**: Paused (feed hold active)
- **Alarm**: Error condition, needs clearing
- **Door**: Safety door open (if enabled)
- **Check**: G-Code check mode (no movement)

### How do I clear an alarm?

1. Determine cause (check console for alarm code)
2. Fix the problem (move machine, reset limit switch, etc.)
3. Send unlock command: `$X`
4. Home machine if needed

## Troubleshooting

### Why is my toolpath not showing in 3D view?

- Press `F` to zoom to fit
- Check file actually contains movement commands
- Verify "Show Toolpath" is enabled
- Try loading a known-good test file

### Why are commands not being sent to GRBL?

- Check connection status (should be green)
- Verify machine state (must be Idle or Run)
- If in Alarm, send `$X` to unlock
- Check console for error messages

### Why is the 3D view slow or choppy?

- Update graphics drivers
- Disable anti-aliasing in settings
- Reduce console history limit
- Close other applications
- Try software rendering mode

### Application won't start. What should I do?

1. Check system requirements
2. Update graphics drivers
3. Run from terminal to see error messages
4. Check [Troubleshooting Guide](TROUBLESHOOTING.md)

### Why does jogging move the wrong distance?

- Verify units match (mm vs inches)
- Check jog distance setting
- Ensure machine is homed
- Verify GRBL steps/mm calibration (`$100-$102`)

### Can I recover from a crash mid-program?

GRBL doesn't support resume from arbitrary positions. Options:
1. Edit G-Code to start from safe line
2. Manually jog to position and restart
3. Use work coordinate system to resume

Prevention: Save work coordinates, test programs first, use feed override.

## Development and Contributing

### Is the source code available?

Yes! rCandle is open source on GitHub: https://github.com/yourusername/rCandle

### Can I contribute to rCandle?

Absolutely! Contributions are welcome:
- Bug reports and feature requests
- Code contributions (pull requests)
- Documentation improvements
- Testing and feedback

### What language is rCandle written in?

Rust 2021 edition. You'll need Rust 1.75+ to build from source.

### How do I report a bug?

1. Check if issue already reported on GitHub
2. Gather information (version, OS, error messages, logs)
3. Create detailed issue with steps to reproduce
4. Include relevant files (G-Code, config, logs)

### Can I request new features?

Yes! File a feature request on GitHub with:
- Clear description of feature
- Use case / why it's needed
- Examples or mockups if applicable

### How often is rCandle updated?

During alpha/beta phase, updates are irregular. After 1.0 release, we plan regular updates with bug fixes and new features.

### Will rCandle support my favorite feature from [other software]?

Maybe! File a feature request. Priority is given to:
- Safety-critical features
- Features that benefit many users
- Features aligned with project goals

### Can I use rCandle in my commercial project?

Yes! GPL-3.0 allows commercial use. However, if you modify rCandle, you must:
- Share your modifications under GPL-3.0
- Attribute the original project
- Include license and copyright notices

### How can I support rCandle development?

- Use rCandle and provide feedback
- Report bugs with detailed information
- Contribute code or documentation
- Help others in discussions
- Sponsor the project (if sponsorship enabled)
- Spread the word about rCandle

## Still Have Questions?

If your question isn't answered here:

1. Check the [User Guide](USER_GUIDE.md)
2. Read the [Troubleshooting Guide](TROUBLESHOOTING.md)
3. Search [GitHub Issues](https://github.com/yourusername/rCandle/issues)
4. Ask in [GitHub Discussions](https://github.com/yourusername/rCandle/discussions)
5. Create a new issue if it's a bug or feature request

---

**This FAQ is updated regularly based on user questions. Suggestions for new questions are welcome!**
