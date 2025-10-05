# rCandle Project Status

## Overview
rCandle is a Rust-based GRBL controller application with G-Code visualization, migrated from the reference C++ Candle application. It uses the egui framework for the user interface and targets Windows, Linux, and macOS.

## Current Status: Phase 8 - Advanced Features Implementation Complete, Build Fixed

### What's Working

#### Core Functionality
- **G-Code Parser**: Complete implementation with tokenization, parsing, and validation
- **G-Code Preprocessor**: Arc interpolation, feedrate management, and coordinate transformation
- **3D Renderer**: WGPU-based visualization with camera controls (rotate, pan, zoom)
- **Serial Communication**: Full serialport implementation with connection management
- **GRBL Protocol**: Command formatting, queue management, and response parsing infrastructure
- **Settings Management**: Configuration loading, saving, validation, and persistence
- **Build System**: ✅ **Fixed!** Now compiles successfully without errors

#### User Interface
- **Main Window**: Multi-panel layout with menu bar, status bar, and central visualization
- **G-Code Editor**: Syntax highlighting, line numbers, search/replace
- **Console Widget**: Command history, colored output (info, warning, error, sent, received)
- **Control Panel**: Connection management, jog controls, machine state display
- **3D Visualization**: Real-time toolpath rendering with camera manipulation
- **File Operations**: Open G-Code files with validation
- **Program Execution**: Run/Pause/Stop/Reset controls, progress tracking, step mode ✨ NEW
- **Settings Dialog**: Comprehensive configuration UI with 5 categories ✨ NEW
- **Theming System**: Dark/light mode with dynamic font sizing ✨ NEW

#### Connection Infrastructure
- **Serial Port**: FTDI/USB serial with configurable baud rates
- **Connection Manager**: Lifecycle management, automatic status queries, command queue
- **Multiple Transports**: Serial, Telnet, and WebSocket support (infrastructure ready)

#### Recent Additions (Phase 8 - Advanced Features)
- **Scripting Engine**: Full Rhai integration with API for machine control, status queries, and program control
- **User Commands**: Customizable command buttons with default library (spindle, coolant, safety commands)
- **Override Controls**: Real-time feed rate, spindle speed, and rapid override support
- **View Presets**: 7 predefined camera views (Isometric, Top, Front, Right, Left, Back, Bottom)
- **Error Handling**: Extended error types for script execution
- **Unit Tests**: Comprehensive tests for overrides and view presets

### Current Limitations

#### Build Status: RESOLVED ✅
**Status**: Fixed as of January 2025

The compilation errors from the egui 0.28 and WGPU 0.20 upgrade have been resolved:
- ✅ Added missing `compilation_options` fields to WGPU pipeline states
- ✅ Removed non-existent `with_focused()` method
- ✅ Fixed deprecated API usage (clamp_range → range)
- ✅ Cleaned up unused mutable variables
- ✅ Dev build: Successful (124MB binary)
- ✅ All 133 unit tests: Passing

See `COMPILATION_FIX_SUMMARY.md` for detailed information.

#### UI Interaction: RESOLVED ✅
**Status**: Fixed as of January 2025

**Issue Resolved**: The UI interaction issue was caused by outdated egui/eframe versions (0.27.x) and WGPU compatibility problems. Upgrading to egui 0.28, eframe 0.28, and WGPU 0.20, along with fixing API compatibility issues, has resolved all interaction problems.

**Current Status**:
- ✅ Mouse interactions working (clicks, drags, scrolling)
- ✅ Keyboard input working (text entry, shortcuts)
- ✅ Menu selections responsive
- ✅ All UI controls functional
- ✅ 3D visualization camera controls working

The application is now fully interactive and ready for comprehensive feature testing.

#### Other Limitations
- **Connection Manager Storage**: After successful connection, the manager needs to be stored and reused
- **Response Handling**: No active loop to receive and process GRBL responses
- **Status Updates**: Machine state, position, and other status not being parsed and displayed
- **Async Communication**: Console messages from async tasks not showing up in UI
- **Testing**: Limited testing without working UI interaction

### Technical Architecture

#### Module Structure
```
rcandle/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── lib.rs                  # Library root
│   ├── connection/            # Connection management
│   │   ├── manager.rs         # Connection lifecycle
│   │   ├── serial.rs          # Serial port implementation
│   │   ├── telnet.rs          # Telnet implementation
│   │   └── websocket.rs       # WebSocket implementation
│   ├── grbl/                  # GRBL protocol
│   │   ├── commands.rs        # Command formatting
│   │   ├── responses.rs       # Response parsing
│   │   ├── queue.rs           # Command queue
│   │   ├── realtime.rs        # Real-time commands
│   │   └── overrides.rs       # Override controls (Phase 8) ✨
│   ├── parser/                # G-Code parsing
│   │   ├── tokenizer.rs       # Lexical analysis
│   │   ├── parser.rs          # Syntax parsing
│   │   ├── preprocessor.rs    # Arc interpolation, etc.
│   │   └── types.rs           # AST definitions
│   ├── renderer/              # 3D visualization
│   │   ├── camera.rs          # Camera system
│   │   ├── line_renderer.rs   # Line drawing
│   │   ├── view_presets.rs    # View presets (Phase 8) ✨
│   │   └── shaders/           # WGSL shaders
│   ├── script/                # Scripting engine (Phase 8) ✨
│   │   ├── mod.rs             # Main module
│   │   ├── api.rs             # Script API
│   │   ├── executor.rs        # Script executor
│   │   └── user_commands.rs   # User command system
│   ├── state/                 # Application state
│   │   ├── app.rs             # App state management
│   │   ├── execution.rs       # Execution state
│   │   └── shared.rs          # Shared state primitives
│   ├── settings/              # Configuration
│   │   └── mod.rs             # Settings management
│   ├── ui/                    # User interface
│   │   ├── app.rs             # Main app window
│   │   ├── panels.rs          # Panel layouts
│   │   └── widgets.rs         # Custom widgets
│   └── utils/                 # Utilities
│       ├── error.rs           # Error types
│       └── logging.rs         # Logging setup
└── Cargo.toml                 # Dependencies
```

#### Key Dependencies
- **egui/eframe**: Immediate mode GUI framework
- **wgpu**: Graphics API for 3D rendering
- **serialport**: Cross-platform serial communication
- **tokio**: Async runtime
- **nom**: Parser combinators for G-Code
- **nalgebra**: Linear algebra for 3D math
- **tracing**: Structured logging

### Development Timeline

#### Phase 1: Project Setup ✅
- Project structure
- Build system configuration
- Initial dependencies

#### Phase 2: Core Parser ✅
- G-Code tokenizer
- Parser implementation
- AST definitions
- Preprocessor

#### Phase 3: Renderer ✅
- WGPU initialization
- Camera system
- Line rendering pipeline
- Shaders

#### Phase 4: UI Framework ✅
- egui integration
- Main window layout
- Editor widget
- Console widget

#### Phase 5: Connection Layer ✅
- Serial port support
- Connection manager
- GRBL protocol
- Command queue

#### Phase 6: Integration and UI Completion ✅ (95% Complete)
- ✅ UI wiring
- ✅ Command flow
- ✅ Program execution controls
- ✅ Settings dialog
- ✅ Theme system
- ✅ UI polish
- ⏸ Status monitoring (blocked by interaction issue)
- **Blocker**: UI interaction issue

#### Phase 7: Testing & Hardware Integration (Pending)
- ❌ Fix UI interaction issue (critical blocker)
- ⏸ Manual UI testing (blocked)
- ⏸ Integration testing (blocked)
- ⏸ Platform-specific testing (blocked)
- ⏸ Hardware testing with GRBL (blocked)
- Documentation (in progress)

#### Phase 8: Advanced Features ✅ (Complete - Infrastructure)
- ✅ Scripting engine with Rhai integration
- ✅ User-defined command buttons
- ✅ Override controls (feed rate, spindle, rapid)
- ✅ View presets for camera
- ⏸ UI integration for advanced features (pending)
- ⏸ Alternative connections complete (deferred, infrastructure exists)

### Next Steps

#### Immediate Priorities
1. **Resolve UI Interaction Issue**
   - Debug egui event handling
   - Test minimal egui example
   - Check platform-specific issues
   - Review eframe configuration

2. **Complete Connection Integration**
   - Store ConnectionManager after connection
   - Implement response handling loop
   - Parse and display status updates
   - Handle connection errors gracefully

3. **Testing Infrastructure**
   - Unit tests for parser
   - Integration tests for connection
   - Mock GRBL for testing

#### Short Term Goals
- Fix UI interaction (CRITICAL - highest priority)
- Test all implemented features manually
- ~~Complete status monitoring display~~ (implementation ready, needs testing)
- ~~Implement program execution controls~~ ✅ DONE
- ~~Add settings dialog~~ ✅ DONE
- ~~Implement theming system~~ ✅ DONE
- Add error handling and user feedback
- Test with real GRBL hardware

#### Medium Term Goals
- Complete connection integration
- Implement response handling loop
- Parse and display real-time status
- Add advanced GRBL features
- Performance optimization
- Cross-platform testing

#### Long Term Goals
- Advanced features (probing, tool changes, WCS)
- Custom macro system
- G-Code optimization
- Multi-language support
- Comprehensive user documentation
- Plugin architecture

### Build & Run

#### Prerequisites
- Rust 1.75 or later
- WGPU-compatible graphics drivers

#### Build
```bash
cd /home/thawkins/projects/rCandle
cargo build --release
```

#### Run
```bash
cargo run --release
```

#### Development Build
```bash
cargo build
cargo run
```

### Contributing

#### Code Style
- Follow Rust standard conventions
- Use `rustfmt` for formatting
- Run `clippy` for linting
- Add documentation for public APIs

#### Testing
- Add unit tests for new functionality
- Integration tests for major features
- Test on multiple platforms when possible

#### Documentation
- Update TODO.md for task tracking
- Update this STATUS.md for major changes
- Add inline documentation for complex code

#### Project Documentation
- `README.md` - Project overview and quick start
- `PROJECT_STATUS.md` - This document (current status)
- `TODO.md` - Task tracking and known issues
- `PROGRESS.md` - Detailed development progress
- `WEEK13_COMPLETION_SUMMARY.md` - Week 13 overview
- `WEEK13_DAY2_SUMMARY.md` - Settings dialog details
- `WEEK13_DAY5_SUMMARY.md` - Theming & polish details

#### Reference Documentation
- Original Candle: https://github.com/Denvi/Candle
- User Manual: https://github.com/Denvi/Candle/blob/master/doc/help_en.html
- GRBL Documentation: https://github.com/craftweeks/grbl-1.1f.customized-for-laser/tree/master/doc/markdown
- egui Documentation: https://docs.rs/egui/
- WGPU Documentation: https://docs.rs/wgpu/

#### Community Resources
- egui Forum Discussion: https://users.rust-lang.org/t/how-to-use-the-button-pressed-and-released-events-in-egui/104106
- GRBL Wiki: https://github.com/gnea/grbl/wiki

---

**Last Updated**: 2024-12-19
**Version**: 0.1.0 (Development)
**Status**: Phase 8 Complete - Advanced Features Infrastructure
**Progress**: Core functionality complete, UI interaction issue blocking testing, Phase 8 advanced features implemented
