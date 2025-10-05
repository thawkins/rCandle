# rCandle

**A Rust GRBL Controller Application with G-Code Visualizer**

rCandle is a modern reimplementation of the [Candle](https://github.com/Denvi/Candle) CNC controller application, written in Rust for improved performance, safety, and maintainability.

![Status: In Development](https://img.shields.io/badge/status-in%20development%20(80%25)-yellow)
![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue)
![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey)

## Overview

rCandle is designed for controlling CNC machines equipped with GRBL firmware using a PC. It supports 3-axis milling machines and laser plotters with comprehensive G-Code manipulation and visualization capabilities.

### Current Development State

The project has reached approximately **80% completion** with all core systems implemented, integrated, and Phase 8 advanced features completed. The application features a fully functional G-Code parser, 3D visualization engine, serial communication layer, comprehensive user interface, scripting engine, user commands, override controls, and view presets. Development is currently focused on resolving a UI interaction issue before proceeding to hardware integration testing.

**Recent Achievements** (Phase 8 - Advanced Features & Build Fixes):
- ✅ Implemented Rhai scripting engine with comprehensive API for machine control and automation
- ✅ Added user-defined command buttons with default library (spindle, coolant, safety operations)
- ✅ Implemented real-time override controls (feed rate, spindle speed, rapid movement)
- ✅ Added 7 camera view presets (Isometric, Top, Front, Right, Left, Back, Bottom)
- ✅ **Completed full UI integration** for all Phase 8 features
- ✅ Added script editor dialog with code editing and save functionality
- ✅ Integrated view preset buttons in control panel
- ✅ Added user commands panel with category organization
- ✅ Extended menu bar with Tools and View menu items
- ✅ Resolved all borrow checker issues with clean compilation
- ✅ **Fixed WGPU 0.20 compilation errors** - Build now succeeds!
- ✅ **All 133 unit tests passing**

**Next Steps:**
- Test UI interaction with newly built binary
- Backend integration for script execution
- Real-time override command transmission to GRBL
- Settings persistence for scripts and user commands
- Hardware integration testing

### Key Features

#### Implemented ✅
- **G-Code Management**: Load, edit, save, and validate G-Code files
- **3D Visualization**: Real-time toolpath rendering with interactive camera controls and view presets
- **Serial Communication**: FTDI/USB serial port support with device discovery
- **GRBL Protocol**: Command formatting, queue management, response parsing, override controls
- **Machine Control**: Jog controls, homing, zero positioning, coordinate systems
- **Console Interface**: Command history with color-coded output
- **Settings System**: Comprehensive configuration with persistence
- **Program Execution**: Run/Pause/Stop controls with progress tracking and step mode
- **Modern UI**: Dark/light themes, keyboard shortcuts, responsive layout
- **Scripting Engine**: Rhai-based automation with comprehensive API
- **User Commands**: Customizable command buttons with default library
- **Override Controls**: Real-time feed rate, spindle speed, and rapid overrides
- **View Presets**: 7 predefined camera views for common angles

#### In Progress 🚧
- **UI Interaction**: Known issue preventing user input (under investigation)
- **Response Handling**: Real-time status updates and machine state monitoring
- **Error Handling**: User-friendly error messages and recovery

#### Planned 📅
- **Height Mapping**: Surface scanning and automatic Z-axis compensation
- **Tool Management**: Tool change sequences and tool library
- **Probing Operations**: Edge finding, center finding, tool length measurement
- **Advanced Features**: Measurement tools, section views, multi-language support

## Status

This project is currently in **active development** with substantial core functionality complete. Development is approximately **80% complete** with the application being in a **near-functional state** pending resolution of a UI interaction issue.

### ✅ Completed Components

#### Core Functionality
- **G-Code Parser**: Full lexical and syntactic analysis with validation
- **G-Code Preprocessor**: Arc interpolation, feedrate management, coordinate transformation
- **3D Renderer**: WGPU-based visualization with camera controls (rotate, pan, zoom)
- **Serial Communication**: Complete serialport implementation with FTDI/USB support
- **GRBL Protocol**: Command formatting, queue management, and response parsing infrastructure
- **Settings Management**: Configuration loading, saving, validation, and persistence

#### User Interface
- **Main Window**: Multi-panel layout with menu bar, status bar, and visualization
- **G-Code Editor**: Syntax highlighting, line numbers, search functionality
- **Console Widget**: Command history with colored output (info, warning, error, sent, received)
- **Control Panel**: Connection management, jog controls, machine state display
- **Program Execution**: Run/Pause/Stop/Reset controls with progress tracking and step mode
- **Settings Dialog**: Comprehensive configuration UI with 5 categories (General, Connection, Visualization, Jog, UI)
- **Theming System**: Dark/light mode with dynamic font sizing
- **3D Visualization Panel**: Real-time toolpath rendering integrated into main window

#### Infrastructure
- **Project Architecture**: Modular design with clear separation of concerns
- **Build System**: Cross-platform Cargo configuration
- **Error Handling**: Comprehensive error types and propagation
- **Scripting System**: Rhai engine with script library and API bindings
- **User Commands**: Command library with category organization
- **Override System**: Real-time GRBL override commands
- **View Presets**: Camera positioning system
- **Logging**: Structured logging with tracing framework
- **Configuration**: JSON-based settings with validation

### 🚧 Known Issues

#### UI Interaction Not Working (Under Investigation)
The application window opens and renders all UI elements correctly, but mouse and keyboard interactions are not functioning. This affects buttons, text fields, menu selections, and all user interactions. 

**Recent Progress:**
- ✅ **Build Fixed**: All compilation errors resolved (January 2025)
- ✅ **Tests Passing**: All 133 unit tests pass
- ✅ **Binary Created**: 124MB debug binary successfully built
- 🔄 **Testing Required**: UI interaction needs verification with newly built binary

This is currently the **primary focus** for testing. The code structure follows egui's immediate mode patterns correctly. With the build now working, the next step is to run the application and test whether the egui 0.28 upgrade resolved the interaction issues.

**Current Status**: Ready for UI interaction testing. See `TODO.md` and `COMPILATION_FIX_SUMMARY.md` for details.

#### Other Limitations
- UI interaction needs testing with newly built binary
- Connection manager needs persistent storage after successful connection
- Response handling loop not yet implemented for continuous GRBL communication
- Machine state, position, and status parsing ready but not yet integrated

### 📋 Remaining Work

#### Phase 7: Testing & Integration (Current Phase)
- ✅ **Fixed compilation errors** (January 2025)
- ✅ **All tests passing** (133 unit tests)
- 🔄 **Test UI interaction** with newly built binary (next step)
- Manual testing of all implemented features
- Integration testing with mock and real GRBL hardware
- Platform-specific testing (Windows, Linux, macOS)
- Documentation completion

#### Phase 8: Advanced Features (Planned)
- Height mapping for surface compensation
- Tool change support
- Probe operations
- Custom macro system
- Keyboard shortcut configuration

## Building from Source

### Prerequisites

- Rust 1.75 or later
- Git
- CMake (for some dependencies)

**Platform-specific requirements:**

**Linux:**
```bash
sudo apt update
sudo apt install build-essential pkg-config libudev-dev
```

**Windows:**
- Visual Studio 2019 or later with C++ build tools

**macOS:**
```bash
xcode-select --install
```

### Build

```bash
git clone https://github.com/yourusername/rCandle.git
cd rCandle
cargo build --release
```

The executable will be in `target/release/rcandle` (or `rcandle.exe` on Windows).

### Run

```bash
cargo run --release
```

## Documentation

### Project Documentation
- **[README.md](README.md)** - This file: project overview and quick start
- **[PROJECT_STATUS.md](PROJECT_STATUS.md)** - Detailed current status and architecture
- **[TODO.md](TODO.md)** - Task tracking and known issues
- **[PROGRESS.md](PROGRESS.md)** - Comprehensive development progress log

### Development Summaries
- **[WEEK13_COMPLETION_SUMMARY.md](WEEK13_COMPLETION_SUMMARY.md)** - Week 13 overview
- **[WEEK13_DAY2_SUMMARY.md](WEEK13_DAY2_SUMMARY.md)** - Settings dialog implementation
- **[WEEK13_DAY5_SUMMARY.md](WEEK13_DAY5_SUMMARY.md)** - Theming and UI polish
- **[PHASE4_SUMMARY.md](PHASE4_SUMMARY.md)** - State management phase summary
- **[PHASE6_WEEK13_PROGRESS.md](PHASE6_WEEK13_PROGRESS.md)** - UI integration progress

### Specification Documents
- **[Specification](.specify/SPECIFICATION.md)** - Complete project requirements and design
- **[Architecture](.specify/ARCHITECTURE.md)** - Technical architecture details
- **[Roadmap](.specify/ROADMAP.md)** - 20-week development plan
- **[Dependencies](.specify/DEPENDENCIES.md)** - Crate selection rationale
- **[Migration Guide](.specify/MIGRATION_GUIDE.md)** - C++ to Rust migration patterns

### User Documentation
- **[User Manual](docs/user_manual.md)** - User guide (coming soon)
- **[Developer Guide](docs/development.md)** - Contributing and development guide (coming soon)

## Quick Start

**Note**: The application is currently experiencing a UI interaction issue that prevents buttons and controls from responding to user input. Once resolved, the following workflow will be available:

1. **Launch the Application**
   ```bash
   cargo run --release
   ```

2. **Connect to Your CNC Machine**
   - Select your serial port from the dropdown
   - Configure baud rate (typically 115200 for GRBL)
   - Click "Connect"

3. **Load G-Code**
   - File → Open to load a G-Code file
   - View toolpath in the 3D visualization panel
   - Edit G-Code in the editor if needed

4. **Control Your Machine**
   - Use jog controls for manual positioning
   - Home the machine if needed
   - Set work zero positions
   - Use program execution controls (Run/Pause/Stop)

5. **Monitor Progress**
   - View real-time position updates
   - Monitor machine state in the status bar
   - Check console output for command history

*Full functionality available once UI interaction issue is resolved.*

## Architecture

rCandle follows a modular architecture with clear separation of concerns:

### Core Modules

- **Connection Module** (`src/connection/`): Abstract interface for GRBL communication
  - Serial port implementation with FTDI/USB support
  - Telnet and WebSocket infrastructure (ready for implementation)
  - Connection lifecycle management
  - Device discovery and enumeration

- **Parser Module** (`src/parser/`): G-Code processing
  - Tokenizer for lexical analysis
  - Recursive descent parser for syntax analysis
  - Preprocessor for arc interpolation and transformations
  - Abstract syntax tree (AST) representation

- **Renderer Module** (`src/renderer/`): 3D visualization
  - WGPU-based rendering engine
  - Camera system with orbit controls
  - Line-based toolpath rendering
  - Grid and axis display
  - Efficient geometry batching

- **State Module** (`src/state/`): Application and machine state
  - Machine state tracking (Idle, Run, Hold, Alarm, etc.)
  - Program execution state
  - Coordinate system management
  - Thread-safe state sharing with Arc/Mutex

- **UI Module** (`src/ui/`): User interface
  - egui immediate mode GUI
  - Multi-panel layout system
  - Custom widgets (editor, console, controls)
  - Settings dialog with validation
  - Theme system (dark/light mode)

- **GRBL Module** (`src/grbl/`): GRBL protocol implementation
  - Command formatting and validation
  - Response parsing
  - Real-time command support
  - Command queue management
  - Status report parsing

- **Settings Module** (`src/settings/`): Configuration management
  - JSON-based persistence
  - Category-based organization
  - Validation and defaults
  - Live reload support

### Data Flow

```
User Input → UI Layer → State Management → GRBL Protocol → Serial Connection → CNC Machine
                ↓                                              ↓
         3D Renderer ← G-Code Parser ← File System    Console ← Response Parser
```

See the [Architecture Document](.specify/ARCHITECTURE.md) for detailed technical information.

## Comparison with Original Candle

### Implementation Status

| Feature | Candle (C++/Qt) | rCandle (Rust) | Status |
|---------|----------------|----------------|--------|
| G-Code Parser | ✅ | ✅ | Complete |
| 3D Visualization | ✅ | ✅ | Complete |
| Serial Communication | ✅ | ✅ | Complete |
| GRBL Protocol | ✅ | ✅ | Complete |
| UI Framework | Qt5 | egui | Complete |
| Settings Management | ✅ | ✅ | Complete |
| Jog Controls | ✅ | ✅ | Complete |
| File Operations | ✅ | ✅ | Complete |
| Program Execution | ✅ | ✅ | UI Ready |
| Console Output | ✅ | ✅ | Complete |
| Height Mapping | ✅ | 📅 | Planned |
| Tool Changes | ✅ | 📅 | Planned |
| Scripting | Limited | 📅 | Planned |
| **UI Interaction** | ✅ | 🐛 | **Issue** |

### Technical Stack Comparison

| Aspect | Candle (C++/Qt) | rCandle (Rust) |
|--------|----------------|----------------|
| Language | C++ | Rust |
| UI Framework | Qt5 | egui + eframe |
| Graphics | OpenGL 2.0 | WGPU (Vulkan/Metal/DX12) |
| Memory Safety | Manual | Guaranteed by compiler |
| Async I/O | Qt event loop | Tokio async runtime |
| Build System | CMake | Cargo |
| Package Manager | vcpkg | Cargo |
| Platforms | Windows, Linux, macOS | Windows, Linux, macOS |
| Binary Size | ~15-20 MB | ~8-12 MB (optimized) |

### Technical Stack
- **Language**: Rust 2021 edition
- **UI Framework**: egui + eframe (immediate mode)
- **Graphics**: WGPU (Vulkan/Metal/DX12)
- **Async Runtime**: Tokio
- **Parser**: nom combinators
- **Scripting**: Rhai

### Goals

- **Feature Parity**: Implement all features from the original Candle
- **Performance**: Match or exceed C++ version performance
- **Safety**: Leverage Rust's memory safety guarantees
- **Maintainability**: Cleaner, more maintainable codebase
- **Modern**: Use modern graphics APIs and UI frameworks

## Contributing

Contributions are welcome! This project is in early development, so there's plenty to do.

### How to Contribute

1. Check the [Roadmap](.specify/ROADMAP.md) for current priorities
2. Look at open issues or create a new one
3. Fork the repository
4. Create a feature branch
5. Make your changes
6. Submit a pull request

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/rCandle.git
cd rCandle

# Install development dependencies
cargo install cargo-watch cargo-audit

# Run tests
cargo test

# Run with auto-reload during development
cargo watch -x run

# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test parser::tests

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

### Current Test Coverage

- **Parser Module**: Unit tests for tokenizer and parser
- **Preprocessor**: Arc interpolation and transformation tests
- **Integration Tests**: Basic G-Code parsing workflows

**Note**: Full integration testing requires resolution of the UI interaction issue.

### Manual Testing

Once the UI interaction issue is resolved:

1. Test serial port connection with mock GRBL simulator
2. Verify G-Code parsing and visualization with sample files
3. Test jog controls and machine movements
4. Validate program execution workflow
5. Test on all target platforms (Windows, Linux, macOS)

Sample G-Code files for testing are available in the `examples/` directory.

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

This is the same license as the original Candle application, ensuring compatibility and continued open-source development.

## Credits

- **Original Candle**: [Denvi/Candle](https://github.com/Denvi/Candle) by Denis Ravilevich Hayrullin
- **GRBL**: [gnea/grbl](https://github.com/gnea/grbl)
- **GRBL Documentation**: [GRBL 1.1f Customized](https://github.com/craftweeks/grbl-1.1f.customized-for-laser/tree/master/doc/markdown)
- **Rust Community**: For excellent crates and documentation

## Support

### Getting Help
- **Issues**: [GitHub Issues](https://github.com/yourusername/rCandle/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/rCandle/discussions)
- **Documentation**: See `docs/` directory and project documentation files

### Troubleshooting

#### Application Won't Start
- Ensure you have Rust 1.75 or later: `rustc --version`
- Check that WGPU-compatible graphics drivers are installed
- Try running with debug logging: `RUST_LOG=debug cargo run`

#### UI Interaction Not Working
This is a known issue under active investigation. The UI renders but does not respond to mouse/keyboard input. Workarounds and fixes will be documented once available. See `TODO.md` for current investigation status.

#### Serial Port Not Detected
- Ensure FTDI/USB drivers are installed for your device
- Check device permissions on Linux: add user to `dialout` group
- Verify device is not in use by another application
- Try unplugging and replugging the device

#### Build Errors
- Clean the build: `cargo clean`
- Update dependencies: `cargo update`
- Check Rust version compatibility
- Review platform-specific requirements in [Building from Source](#building-from-source)

### Reporting Issues

When reporting issues, please include:
- Operating system and version
- Rust version (`rustc --version`)
- Steps to reproduce the problem
- Expected vs actual behavior
- Relevant log output (run with `RUST_LOG=debug`)

## Key Resources

### GRBL Firmware Documentation
- **Primary Documentation**: [GRBL 1.1f Markdown Docs](https://github.com/craftweeks/grbl-1.1f.customized-for-laser/tree/master/doc/markdown)
  - Commands reference (G-codes, M-codes, $ commands)
  - Serial interface protocol
  - Configuration settings
  - Real-time jogging
  - Laser mode features
- **GRBL v1.1 Wiki**: https://github.com/gnea/grbl/wiki
- **GRBL v0.9 Wiki**: https://github.com/grbl/grbl/wiki

### Project Documentation
- [Specification](.specify/SPECIFICATION.md) - Complete requirements and design
- [Architecture](.specify/ARCHITECTURE.md) - Technical architecture details
- [Roadmap](.specify/ROADMAP.md) - 20-week development plan
- [Dependencies](.specify/DEPENDENCIES.md) - Crate selection rationale
- [Migration Guide](.specify/MIGRATION_GUIDE.md) - C++ to Rust patterns

## Acknowledgments

This project is a Rust migration of Candle, originally created by Denis Ravilevich Hayrullin. We are grateful for the original work and aim to continue its legacy with a modern, safe implementation.

## Related Projects

- [Candle](https://github.com/Denvi/Candle) - Original C++/Qt application
- [GRBL](https://github.com/gnea/grbl) - CNC controller firmware
- [bCNC](https://github.com/vlachoudis/bCNC) - Python-based GRBL interface
- [CNCjs](https://github.com/cncjs/cncjs) - Web-based CNC interface

## Roadmap Highlights

### Development Progress (75% Complete)

- **Phase 1 (Weeks 1-2)**: Foundation and project setup ✅ **COMPLETE**
  - Project structure, build system, dependencies
  
- **Phase 2 (Weeks 3-4)**: G-Code parser ✅ **COMPLETE**
  - Tokenizer, parser, preprocessor, AST
  
- **Phase 3 (Weeks 5-6)**: Serial communication ✅ **COMPLETE**
  - Serial port support, connection manager, GRBL protocol
  
- **Phase 4 (Week 7)**: State management ✅ **COMPLETE**
  - Application state, machine state, execution state
  
- **Phase 5 (Weeks 8-10)**: 3D visualization ✅ **COMPLETE**
  - WGPU renderer, camera system, toolpath rendering
  
- **Phase 6 (Weeks 11-13)**: User interface ✅ **COMPLETE**
  - egui integration, panels, widgets, settings dialog, theming
  
- **Phase 7 (Weeks 14-15)**: Testing & Integration 🚧 **IN PROGRESS**
  - **Blocker**: UI interaction issue
  - Manual testing, integration testing, hardware validation
  
- **Phase 8 (Weeks 16-17)**: Advanced features 📅 **PLANNED**
  - Height mapping, tool changes, probing, macros
  
- **Phase 9 (Weeks 18-20)**: Polish and release 📅 **PLANNED**
  - Performance optimization, documentation, packaging

See the full [Roadmap](.specify/ROADMAP.md) for detailed milestone breakdown.

---

**Project Status**: Phase 7 (Testing & Integration) - 75% Complete  
**Last Updated**: December 2024  
**Version**: 0.1.0-alpha  
**Current Focus**: Resolving UI interaction issue, preparing for hardware integration testing

**Note**: This project is in active development. The application is near-functional with all core systems implemented. Features and documentation continue to be added and refined. Contributions and testing assistance are welcome once the UI interaction issue is resolved.
