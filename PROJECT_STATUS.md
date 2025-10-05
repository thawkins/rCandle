# rCandle Project Status

## Overview
rCandle is a Rust-based GRBL controller application with G-Code visualization, migrated from the reference C++ Candle application. It uses the egui framework for the user interface and targets Windows, Linux, and macOS.

## Current Status: Phase 6 - Integration and Feature Completion

### What's Working

#### Core Functionality
- **G-Code Parser**: Complete implementation with tokenization, parsing, and validation
- **G-Code Preprocessor**: Arc interpolation, feedrate management, and coordinate transformation
- **3D Renderer**: WGPU-based visualization with camera controls (rotate, pan, zoom)
- **Serial Communication**: Full serialport implementation with connection management
- **GRBL Protocol**: Command formatting, queue management, and response parsing infrastructure
- **Settings Management**: Configuration loading, saving, and validation

#### User Interface
- **Main Window**: Multi-panel layout with menu bar, status bar, and central visualization
- **G-Code Editor**: Syntax highlighting, line numbers, search/replace
- **Console Widget**: Command history, colored output (info, warning, error, sent, received)
- **Control Panel**: Connection management, jog controls, machine state display
- **3D Visualization**: Real-time toolpath rendering with camera manipulation
- **File Operations**: Open G-Code files with validation

#### Connection Infrastructure
- **Serial Port**: FTDI/USB serial with configurable baud rates
- **Connection Manager**: Lifecycle management, automatic status queries, command queue
- **Multiple Transports**: Serial, Telnet, and WebSocket support (infrastructure ready)

### Current Limitations

#### Critical Issue: UI Interaction
**Status**: Under Investigation

The UI renders correctly and displays all elements, but mouse and keyboard interactions are not working. This affects:
- Button clicks
- Text field input
- Menu selections
- All user interactions

This is the primary blocker for testing the application. The code structure appears correct per egui's immediate mode patterns. Possible causes include event loop configuration, viewport setup, or platform-specific issues.

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
│   │   └── realtime.rs        # Real-time commands
│   ├── parser/                # G-Code parsing
│   │   ├── tokenizer.rs       # Lexical analysis
│   │   ├── parser.rs          # Syntax parsing
│   │   ├── preprocessor.rs    # Arc interpolation, etc.
│   │   └── types.rs           # AST definitions
│   ├── renderer/              # 3D visualization
│   │   ├── camera.rs          # Camera system
│   │   ├── line_renderer.rs   # Line drawing
│   │   └── shaders/           # WGSL shaders
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

#### Phase 6: Integration (Current)
- UI wiring
- Command flow
- Status monitoring
- **Blocked by**: UI interaction issue

#### Phase 7: Testing & Polish (Planned)
- Fix UI interaction
- Integration testing
- Platform-specific testing
- Documentation

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
- Fix UI interaction (highest priority)
- Complete status monitoring display
- Implement program execution controls
- Add error handling and user feedback
- Test with real GRBL hardware

#### Long Term Goals
- Advanced features (probing, tool changes, WCS)
- Custom macro system
- G-Code optimization
- Multi-language support
- Comprehensive documentation

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

### Resources

#### Reference Documentation
- Original Candle: https://github.com/Denvi/Candle
- User Manual: https://github.com/Denvi/Candle/blob/master/doc/help_en.html
- GRBL Documentation: https://github.com/craftweeks/grbl-1.1f.customized-for-laser/tree/master/doc/markdown
- egui Documentation: https://docs.rs/egui/
- WGPU Documentation: https://docs.rs/wgpu/

#### Community Resources
- egui Forum Discussion: https://users.rust-lang.org/t/how-to-use-the-button-pressed-and-released-events-in-egui/104106
- GRBL Wiki: https://github.com/gnea/grbl/wiki

### License
GPL-3.0 (matching original Candle project)

### Acknowledgments
- Original Candle application by Denvi
- GRBL firmware project
- Rust community and crate maintainers

---

**Last Updated**: 2024-12-19
**Version**: 0.1.0 (Development)
**Status**: Phase 6 - Integration (UI interaction issue blocking progress)
