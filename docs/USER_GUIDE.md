# rCandle User Guide

**Version**: 0.1.0-alpha  
**Last Updated**: January 2025

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [User Interface Overview](#user-interface-overview)
4. [Connecting to Your CNC Machine](#connecting-to-your-cnc-machine)
5. [Loading and Editing G-Code](#loading-and-editing-g-code)
6. [3D Visualization](#3d-visualization)
7. [Machine Control](#machine-control)
8. [Program Execution](#program-execution)
9. [Settings and Configuration](#settings-and-configuration)
10. [Advanced Features](#advanced-features)
11. [Tips and Best Practices](#tips-and-best-practices)

## Introduction

rCandle is a modern GRBL controller application designed for controlling CNC machines running GRBL firmware. It provides comprehensive G-Code manipulation, real-time 3D visualization, and intuitive machine control.

### What is GRBL?

GRBL is open-source CNC firmware that runs on Arduino-based controllers. It interprets G-Code commands and controls stepper motors for CNC machines including mills, routers, lathes, and laser cutters.

### Key Features

- **G-Code Editor**: Load, edit, and validate G-Code files
- **3D Visualization**: Real-time toolpath rendering with camera controls
- **Serial Communication**: Direct connection to GRBL controllers
- **Machine Control**: Jog, home, and zero position controls
- **Program Execution**: Run, pause, stop, and step through programs
- **Scripting**: Automate tasks with Rhai scripts
- **User Commands**: Create custom command buttons
- **Override Controls**: Real-time feed rate and spindle speed adjustment

## Getting Started

### System Requirements

- **Operating System**: Windows 10/11, Linux (Ubuntu 20.04+), or macOS 11+
- **Graphics**: WGPU-compatible graphics driver
- **USB Port**: For serial connection to GRBL controller
- **Memory**: 4GB RAM minimum, 8GB recommended
- **Storage**: 100MB free disk space

### Installation

1. Download the appropriate version for your platform
2. Extract the archive to your preferred location
3. Run the executable: `rcandle` (Linux/macOS) or `rcandle.exe` (Windows)

### First Launch

On first launch, rCandle will create a configuration directory:
- **Windows**: `%APPDATA%\rCandle\`
- **Linux**: `~/.config/rCandle/`
- **macOS**: `~/Library/Application Support/rCandle/`

Default settings will be created automatically.

## User Interface Overview

### Main Window Layout

The rCandle interface consists of several panels:

#### Top Menu Bar
- **File**: Open, save G-Code files, exit application
- **Edit**: Cut, copy, paste, find and replace
- **Machine**: Connection, homing, and control functions
- **Program**: Execution controls (run, pause, stop)
- **Tools**: Scripts, user commands, settings
- **View**: Camera presets, theme switching, panel visibility
- **Help**: User guide, about dialog

#### Left Panel: File and Connection
- **File Information**: Current file name and line count
- **Connection Controls**: Port selection, connect/disconnect
- **Machine Status**: Current machine state display

#### Center Panel: 3D Visualization
- **Toolpath Display**: Real-time 3D rendering of G-Code
- **Camera Controls**: Rotate, pan, zoom with mouse
- **View Presets**: Quick camera positioning buttons
- **Grid and Axes**: Visual reference guides

#### Right Panel: Controls
- **Jog Controls**: Manual machine positioning
  - Axis buttons (X+/-, Y+/-, Z+/-)
  - Distance and speed settings
  - Home and zero buttons
- **Program Execution**: Run, pause, stop, step controls
- **Override Controls**: Feed rate, spindle, and rapid overrides
- **User Commands**: Custom command buttons

#### Bottom Panel: Editor and Console
- **G-Code Editor Tab**: 
  - Syntax highlighting
  - Line numbers
  - Search and replace functionality
- **Console Tab**:
  - Command history
  - Colored output (info, warnings, errors)
  - Direct GRBL command input

#### Status Bar
- Connection status indicator
- Current position (work and machine coordinates)
- Machine state (Idle, Run, Hold, etc.)
- Feed rate and spindle speed

## Connecting to Your CNC Machine

### Hardware Setup

1. Connect your GRBL controller to your computer via USB
2. Power on your CNC machine
3. Ensure FTDI/USB drivers are installed (usually automatic)

### Establishing Connection

1. Click the **Port** dropdown in the left panel
2. Select your GRBL controller from the list
3. Choose the appropriate **Baud Rate** (typically 115200)
4. Click **Connect**

### Connection Status

- **Disconnected**: Red indicator, no connection
- **Connecting**: Yellow indicator, establishing connection
- **Connected**: Green indicator, ready for commands

### Troubleshooting Connection Issues

- **Port not listed**: Check USB cable, try different port
- **Permission denied** (Linux): Add user to dialout group
- **Connection fails**: Verify baud rate matches GRBL settings
- **Intermittent connection**: Check cable quality, try lower baud rate

## Loading and Editing G-Code

### Opening G-Code Files

1. Click **File** → **Open** or press `Ctrl+O`
2. Navigate to your G-Code file (.nc, .gcode, .tap)
3. Click **Open**

The file will be loaded, parsed, and visualized in the 3D panel.

### G-Code Editor

The editor provides:
- **Line Numbers**: For easy reference
- **Syntax Highlighting**: Commands, parameters, comments
- **Search**: `Ctrl+F` to find text
- **Replace**: `Ctrl+H` to find and replace

### Validation

rCandle automatically validates G-Code on loading:
- **Syntax Errors**: Highlighted in red
- **Warnings**: Displayed in console
- **Line Numbers**: Shown for easy debugging

### Saving Files

1. Make your edits
2. Click **File** → **Save** or press `Ctrl+S`
3. Choose location and filename

## 3D Visualization

### Camera Controls

**Mouse**:
- **Left Click + Drag**: Rotate view around toolpath
- **Right Click + Drag**: Pan view left/right/up/down
- **Scroll Wheel**: Zoom in/out
- **Middle Click**: Reset to default view

**View Presets**:
- **Isometric**: 45° angle view (default)
- **Top**: View from above (XY plane)
- **Front**: View from front (XZ plane)
- **Right**: View from right side (YZ plane)
- **Left**: View from left side
- **Back**: View from back
- **Bottom**: View from below

### Toolpath Display

- **Rapid Moves**: Shown in one color (typically gray)
- **Feed Moves**: Shown in another color (typically blue/green)
- **Current Position**: Highlighted during execution
- **Grid**: Reference grid at Z=0
- **Axes**: X (red), Y (green), Z (blue)

### Zoom to Fit

Press `F` or click the **Fit** button to automatically frame the entire toolpath.

## Machine Control

### Homing

Homing establishes machine coordinates:
1. Ensure machine is clear of obstacles
2. Click **Home All** button
3. Machine will move to home position (typically X0, Y0, Z max)
4. Wait for "Idle" state

**Safety**: Machine will move at rapid speed to home switches.

### Jogging

Manual positioning of the machine:

1. **Select Distance**: Choose step size (0.1, 1, 10, 100mm)
2. **Select Speed**: Choose jog speed (slow, medium, fast)
3. **Click Axis Button**: X+, X-, Y+, Y-, Z+, Z-
4. **Keyboard**: Use arrow keys for XY, Page Up/Down for Z

**Continuous Jog**: Hold the button for continuous movement.

### Zeroing Work Coordinates

Set current position as work zero:
1. Jog to desired zero position
2. Click **Zero X**, **Zero Y**, or **Zero Z**
3. Or click **Zero All** to zero all axes

This sets the work coordinate system (G54) origin.

### Coordinate Systems

- **Machine Coordinates (MPos)**: Absolute position from home
- **Work Coordinates (WPos)**: Position relative to work zero
- **G54-G59**: Multiple work coordinate systems supported

## Program Execution

### Running a Program

1. Load G-Code file
2. Ensure machine is connected and homed
3. Position tool at safe starting position
4. Click **Run** button or press `Space`

### Execution Controls

- **Run/Resume**: Start or continue program execution
- **Pause**: Temporarily pause (feed hold)
- **Stop**: Stop execution and reset
- **Reset**: Reset to beginning without stopping machine
- **Step**: Execute single line (step mode)

### Progress Monitoring

- **Progress Bar**: Visual completion indicator
- **Line Counter**: Current line / Total lines
- **Percentage**: Completion percentage
- **Elapsed Time**: Time since program start
- **Estimated Remaining**: Calculated time to completion

### Step Mode

For debugging or careful execution:
1. Enable **Step Mode** checkbox
2. Click **Step** to execute one line at a time
3. Review each movement before proceeding

### Override Controls

Adjust speeds during execution without stopping:

**Feed Rate Override** (10-200%):
- Affects cutting feed rates
- Use slider or +/- buttons
- Does not affect rapids

**Spindle Override** (10-200%):
- Affects spindle RPM
- Real-time adjustment
- Requires GRBL 1.1+

**Rapid Override** (25%, 50%, 100%):
- Affects rapid movement speed
- For safety during first runs
- Toggle between preset values

## Settings and Configuration

### Accessing Settings

Click **Tools** → **Settings** or press `Ctrl+,`

### Settings Categories

#### General
- **Units**: Metric (mm) or Imperial (inches)
- **Language**: Interface language
- **Theme**: Light or dark mode

#### Connection
- **Default Port**: Auto-select last used port
- **Baud Rate**: Default communication speed
- **Connection Timeout**: Seconds before timeout
- **Auto-reconnect**: Attempt reconnection on disconnect

#### Visualization
- **Grid Size**: Grid spacing in units
- **Show Grid**: Toggle grid visibility
- **Show Axes**: Toggle axes visibility
- **Anti-aliasing**: Smooth line rendering
- **Background Color**: 3D view background
- **Rapid Color**: Color for G0 moves
- **Feed Color**: Color for G1/G2/G3 moves

#### Jog
- **Default Distance**: Initial jog step size
- **Default Speed**: Initial jog speed
- **Keyboard Shortcuts**: Enable keyboard jogging
- **Continuous Mode**: Hold button for continuous jog

#### UI
- **Font Size**: Interface text size
- **Console History**: Lines to keep in console
- **Auto-scroll**: Scroll console automatically
- **Panel Layout**: Save/restore panel sizes

### Saving Settings

Changes are applied immediately and persisted to disk.

## Advanced Features

### Scripting with Rhai

Automate common tasks with scripts:

1. Click **Tools** → **Scripts**
2. Click **New Script**
3. Write your script using Rhai syntax
4. Save and execute

**Example Script**:
```rhai
// Home machine and zero
send_command("$H");
wait(5000);
zero_axis("Z");
print("Machine ready!");
```

### User Commands

Create custom command buttons:

1. Click **Tools** → **User Commands**
2. Click **Add Command**
3. Define command details:
   - Name: Display text
   - Commands: GRBL commands to send
   - Category: Organization
   - Keyboard Shortcut: Optional hotkey
4. Save

**Common Commands**:
- Spindle On/Off: `M3 S12000` / `M5`
- Coolant On/Off: `M8` / `M9`
- Tool Change: `M6 T1`

### Macros

Record sequences of commands for playback:

1. Start macro recording
2. Perform actions
3. Stop recording
4. Save macro
5. Playback when needed

## Tips and Best Practices

### Safety First
- **Always home** before running programs
- **Test programs** with feed override at 50% first
- **Keep emergency stop** within reach
- **Monitor first runs** closely
- **Start above work** to avoid crashes

### Workflow Optimization
- **Save work zeros** in G54-G59 for repeat jobs
- **Use view presets** to quickly check toolpath
- **Create user commands** for common operations
- **Write scripts** for setup routines
- **Enable step mode** for unfamiliar programs

### File Management
- **Organize G-Code** in project folders
- **Name files** descriptively
- **Keep backups** of working files
- **Comment your code** for future reference
- **Version control** for iterative designs

### Troubleshooting
- **Check console** for error messages
- **Verify work zero** before starting
- **Test jog controls** before loading program
- **Restart connection** if commands lag
- **Update GRBL firmware** if features missing

### Performance
- **Close unused programs** for better performance
- **Reduce console history** on slower systems
- **Disable anti-aliasing** if visualization is slow
- **Split large files** if loading is slow

## Getting Help

### Resources
- **User Manual**: This guide
- **Keyboard Shortcuts**: Help → Keyboard Shortcuts
- **FAQ**: Help → Frequently Asked Questions
- **Troubleshooting Guide**: Help → Troubleshooting

### Community
- **GitHub Issues**: Report bugs and request features
- **Discussions**: Ask questions and share tips
- **Wiki**: Community-contributed guides

### Support
For technical support, check the GitHub repository or consult the troubleshooting guide.

---

**Happy Machining!**

*This guide is for rCandle v0.1.0-alpha. Features and interface may change in future versions.*
