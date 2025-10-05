# rCandle - Rust Migration Specification

## Project Overview

**rCandle** is a Rust reimplementation of the C++ Candle application, a GRBL controller application with G-Code visualizer. This migration aims to modernize the codebase while maintaining feature parity with the original Qt-based application.

### Source Application
- **Name**: Candle
- **Version**: 10.10.2
- **Repository**: https://github.com/Denvi/Candle
- **Language**: C++ with Qt5 framework
- **Lines of Code**: ~1,319 source files in C++

### Purpose
The Candle program is designed for controlling CNC machines equipped with GRBL firmware using a PC. It supports 3-axis milling machines and laser plotters with comprehensive G-Code manipulation and visualization capabilities.

## Core Requirements

### 1. Functional Requirements

#### 1.1 GRBL Communication
- Serial port communication with GRBL controllers
- Support for alternative connection types:
  - Serial port (primary)
  - Telnet/TCP
  - WebSocket
- Real-time status monitoring
- Command sending and response handling
- Support for GRBL v0.9, v1.1, and newer versions

#### 1.2 G-Code Program Management
- Load G-Code files from disk
- Edit G-Code programs in-application
- Save modified G-Code files
- Syntax validation
- Line-by-line execution control
- Program state management (running, paused, stopped)

#### 1.3 G-Code Visualization
- 3D OpenGL-based visualization of toolpaths
- Real-time progress indication during machining
- Display of:
  - Tool path lines
  - Current tool position
  - Work coordinate system origin
  - Machine bounds
  - Height map overlay (when applicable)
- Camera controls (pan, zoom, rotate)
- Multiple view modes

#### 1.4 CNC Machine Control
- **Manual Control (Jog)**
  - XYZ axis movement via keyboard/buttons
  - Configurable jog step sizes
  - Continuous jog mode
  - Feed rate adjustment
  
- **Work Coordinate System**
  - Zero axis operations (X, Y, Z individually or together)
  - Coordinate display (machine and work coordinates)
  - Coordinate system offset management

- **Spindle Control**
  - Spindle on/off
  - Speed control (RPM)
  - Override controls

- **Feed Rate Override**
  - Real-time feed rate adjustment
  - Spindle speed override
  - Rapid traverse override

#### 1.5 Height Map Functionality
- Surface scanning for height compensation
- Grid-based height measurement
- Height map creation from probe operations
- G-Code transformation based on height map
- Height map visualization
- Save/load height maps
- Interpolation between measured points

#### 1.6 Console Interface
- Command history
- Direct GRBL command input
- Response logging
- Filter and search capabilities

#### 1.7 Settings Management
- Connection settings (port, baud rate)
- Visualization settings (colors, line widths)
- Machine limits configuration
- Default directories
- UI preferences
- Profile management (save/load settings profiles)

#### 1.8 User Commands
- Configurable custom command buttons
- Macro support
- Script execution

### 2. Architecture Components

#### 2.1 Core Modules

##### Connection Module
- **Responsibility**: Abstract interface for GRBL communication
- **Rust Crates**: 
  - `serialport` for serial communication
  - `tokio` for async I/O
  - `tokio-tungstenite` for WebSocket support
- **Components**:
  - Connection trait (abstract interface)
  - SerialConnection implementation
  - TelnetConnection implementation
  - WebSocketConnection implementation
  - Connection manager for lifecycle management

##### Parser Module
- **Responsibility**: G-Code parsing and manipulation
- **Components**:
  - G-Code lexer/tokenizer
  - G-Code parser (command extraction)
  - G-Code preprocessor (arc expansion, units conversion)
  - Line segment representation
  - Point segment representation
  - Arc properties calculator
  - View parser (for visualization data extraction)

##### Visualization Module (Renderer)
- **Responsibility**: 3D rendering of toolpaths and machine state
- **Rust Crates**:
  - `wgpu` (modern, safe graphics API - preferred over OpenGL)
  - Alternative: `glium` or `glow` for OpenGL support
  - `egui` or `iced` for UI integration with 3D viewport
- **Components**:
  - Shader management
  - Vertex buffer management
  - Camera controller
  - Drawable traits and implementations:
    - GCodeDrawer (toolpath rendering)
    - ToolDrawer (current tool position)
    - OriginDrawer (coordinate system origin)
    - HeightMapDrawers (border, grid, interpolation)
    - SelectionDrawer
    - MachineBoundsDrawer

##### State Management Module
- **Responsibility**: Application and machine state tracking
- **Components**:
  - Machine state (idle, run, hold, alarm, etc.)
  - Work coordinates tracking
  - Machine coordinates tracking
  - Program execution state
  - Settings/configuration storage
  - Parser state for G-Code modality

##### UI Module
- **Responsibility**: User interface
- **Rust Crates**: 
  - Primary option: `iced` (pure Rust, cross-platform, Elm-inspired)
  - Alternative: `egui` (immediate mode, great for tools)
  - Alternative: `Slint` (declarative UI, Qt-like)
- **Components**:
  - Main window
  - G-Code editor widget
  - Console widget
  - Control panels:
    - State panel
    - Control panel (jog, spindle)
    - Coordinate system panel
    - Override panel
    - Height map panel
    - User commands panel
  - Settings dialog
  - About dialog
  - File dialogs

##### Height Map Module
- **Responsibility**: Surface height mapping and compensation
- **Components**:
  - Height map data structure (2D grid)
  - Probe operation coordinator
  - Interpolation algorithms (bilinear, bicubic)
  - G-Code transformer (apply height compensation)
  - Height map I/O (save/load)

##### Script Module
- **Responsibility**: Custom scripting and automation
- **Rust Crates**:
  - `rhai` (embedded scripting language for Rust)
  - Alternative: `mlua` (Lua bindings)
- **Components**:
  - Script engine integration
  - API bindings for application functions
  - Script variable management

#### 2.2 Data Flow

```
User Input → UI Module → State Management
                ↓
          Connection Module ↔ GRBL Controller
                ↓
          Parser Module
                ↓
          Visualization Module
```

### 3. Technology Stack

#### 3.1 Core Language
- **Rust** (latest stable)
- Edition 2021

#### 3.2 Key Dependencies

**Communication**
- `serialport = "4.3"` - Serial port access
- `tokio = { version = "1.35", features = ["full"] }` - Async runtime
- `tokio-tungstenite = "0.21"` - WebSocket client

**UI Framework** (Choose one)
- `iced = "0.12"` (Recommended - pure Rust, reactive)
- `egui = "0.27"` (Alternative - immediate mode)
- `slint = "1.4"` (Alternative - declarative)

**Graphics/Rendering**
- `wgpu = "0.19"` - Modern graphics API
- `bytemuck = "1.14"` - Safe casting for vertex data
- `glam = "0.27"` - Vector/matrix math

**Parsing**
- `nom = "7.1"` - Parser combinator library (for G-Code parsing)
- `regex = "1.10"` - Regular expressions

**File I/O**
- `serde = { version = "1.0", features = ["derive"] }` - Serialization
- `serde_json = "1.0"` - JSON support for configs
- `toml = "0.8"` - TOML support for configs

**Scripting**
- `rhai = "1.17"` - Embedded scripting engine

**Logging**
- `tracing = "0.1"` - Structured logging
- `tracing-subscriber = "0.3"` - Log formatting

**Error Handling**
- `thiserror = "1.0"` - Error type derivation
- `anyhow = "1.0"` - Error handling for applications

**Math & Interpolation**
- `nalgebra = "0.32"` - Linear algebra (for height map interpolation)

### 4. Project Structure

```
rCandle/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── .gitignore
├── docs/
│   ├── architecture.md
│   ├── user_manual.md
│   └── development.md
├── src/
│   ├── main.rs                    # Application entry point
│   ├── lib.rs                     # Library root
│   ├── connection/
│   │   ├── mod.rs                 # Connection trait
│   │   ├── serial.rs              # Serial port implementation
│   │   ├── telnet.rs              # Telnet implementation
│   │   └── websocket.rs           # WebSocket implementation
│   ├── parser/
│   │   ├── mod.rs                 # Parser module root
│   │   ├── gcode_parser.rs        # G-Code parsing
│   │   ├── gcode_preprocessor.rs  # Preprocessing utilities
│   │   ├── line_segment.rs        # Line segment representation
│   │   ├── point_segment.rs       # Point segment representation
│   │   ├── arc_properties.rs      # Arc calculations
│   │   └── view_parser.rs         # Extract visualization data
│   ├── renderer/
│   │   ├── mod.rs                 # Renderer module root
│   │   ├── camera.rs              # Camera controller
│   │   ├── shader.rs              # Shader management
│   │   ├── vertex.rs              # Vertex structures
│   │   ├── drawers/
│   │   │   ├── mod.rs
│   │   │   ├── gcode.rs           # G-Code path rendering
│   │   │   ├── tool.rs            # Tool position
│   │   │   ├── origin.rs          # Origin marker
│   │   │   ├── heightmap.rs       # Height map visualization
│   │   │   ├── selection.rs       # Selection box
│   │   │   └── machine_bounds.rs  # Machine limits
│   │   └── scene.rs               # Scene management
│   ├── state/
│   │   ├── mod.rs                 # State module root
│   │   ├── machine.rs             # Machine state
│   │   ├── program.rs             # Program execution state
│   │   ├── coordinates.rs         # Coordinate systems
│   │   └── settings.rs            # Application settings
│   ├── heightmap/
│   │   ├── mod.rs                 # Height map module root
│   │   ├── grid.rs                # Height map data structure
│   │   ├── interpolation.rs       # Interpolation algorithms
│   │   ├── probe.rs               # Probing operations
│   │   └── transform.rs           # G-Code transformation
│   ├── script/
│   │   ├── mod.rs                 # Script module root
│   │   ├── engine.rs              # Script engine integration
│   │   └── bindings.rs            # API bindings
│   ├── ui/
│   │   ├── mod.rs                 # UI module root
│   │   ├── main_window.rs         # Main application window
│   │   ├── widgets/
│   │   │   ├── mod.rs
│   │   │   ├── gcode_editor.rs    # G-Code editor widget
│   │   │   ├── console.rs         # Console widget
│   │   │   ├── viewport.rs        # 3D viewport widget
│   │   │   └── controls.rs        # Control panels
│   │   ├── dialogs/
│   │   │   ├── mod.rs
│   │   │   ├── settings.rs        # Settings dialog
│   │   │   └── about.rs           # About dialog
│   │   └── theme.rs               # UI theming
│   ├── grbl/
│   │   ├── mod.rs                 # GRBL-specific logic
│   │   ├── commands.rs            # GRBL command construction
│   │   ├── responses.rs           # Response parsing
│   │   └── protocol.rs            # GRBL protocol handling
│   └── utils/
│       ├── mod.rs
│       ├── file_io.rs             # File operations
│       └── logger.rs              # Logging setup
├── tests/
│   ├── integration/
│   │   ├── parser_tests.rs
│   │   └── connection_tests.rs
│   └── common/
│       └── mod.rs
├── examples/
│   ├── simple_viewer.rs           # Simple G-Code viewer
│   └── serial_test.rs             # Serial connection test
├── assets/
│   ├── shaders/
│   │   ├── vertex.wgsl
│   │   └── fragment.wgsl
│   ├── icons/
│   └── fonts/
└── resources/
    └── sample_gcode/
        ├── simple.nc
        └── complex.nc
```

### 5. Implementation Phases

#### Phase 1: Foundation (Weeks 1-2)
- Project scaffolding with Cargo
- Basic project structure
- Logging infrastructure
- Configuration management
- Error types definition
- Basic CLI interface for testing

**Deliverables**:
- Compiling skeleton application
- Configuration loading/saving
- Logging to file and console

#### Phase 2: G-Code Parser (Weeks 3-4)
- G-Code lexer/tokenizer
- G-Code parser implementation
- Line segment and arc representations
- Unit tests for parser
- Preprocessing utilities (coordinate transformation, arc expansion)

**Deliverables**:
- Parse standard G-Code files
- Extract all commands and parameters
- Convert to internal representation
- Pass comprehensive parser test suite

#### Phase 3: Connection Module (Weeks 5-6)
- Connection trait definition
- Serial port implementation
- Async I/O with Tokio
- GRBL protocol handling
- Command/response queue
- Connection state management

**Deliverables**:
- Connect to GRBL via serial port
- Send commands and receive responses
- Parse GRBL status messages
- Handle real-time commands

#### Phase 4: State Management (Week 7)
- Machine state tracking
- Coordinate system management
- Program execution state
- Settings storage and retrieval

**Deliverables**:
- Track machine position and status
- Manage work/machine coordinates
- Store and load application settings

#### Phase 5: Visualization Core (Weeks 8-10)
- WGPU setup and initialization
- Camera controller
- Shader pipeline
- Basic vertex rendering
- G-Code path visualization
- Tool position rendering

**Deliverables**:
- Display loaded G-Code in 3D
- Interactive camera controls
- Real-time tool position updates

#### Phase 6: UI Framework (Weeks 11-13)
- Main window layout
- G-Code editor widget
- Console widget
- Control panels
- Settings dialog
- File dialogs
- Integration with 3D viewport

**Deliverables**:
- Functional GUI application
- Load/save G-Code files
- Send commands to GRBL
- View machine status
- Manual jog controls

#### Phase 7: Height Map (Weeks 14-15)
- Height map data structure
- Probing sequence implementation
- Interpolation algorithms
- G-Code transformation with height compensation
- Height map visualization
- Save/load functionality

**Deliverables**:
- Create height maps through probing
- Apply height compensation to G-Code
- Visualize height maps in 3D viewport

#### Phase 8: Advanced Features (Weeks 16-17)
- Scripting engine integration
- User command macros
- Additional connection types (Telnet, WebSocket)
- Advanced visualization features
- Override controls

**Deliverables**:
- Execute custom scripts
- User-defined command buttons
- Alternative connection methods

#### Phase 9: Polish & Testing (Weeks 18-20)
- Comprehensive integration testing
- UI/UX refinements
- Performance optimization
- Documentation completion
- Bug fixes
- Cross-platform testing (Windows, Linux, macOS)

**Deliverables**:
- Stable, tested application
- Complete user manual
- Installation packages
- Developer documentation

### 6. Key Design Decisions

#### 6.1 Async vs Sync Architecture
- Use **async/await** with Tokio for all I/O operations
- Connection module uses async channels for command/response
- UI runs on main thread with periodic polling or message passing

#### 6.2 UI Framework Selection
**Recommendation: Iced**
- Pros: Pure Rust, reactive architecture, good 3D integration, active development
- Cons: Smaller ecosystem than Qt, fewer pre-built widgets
- Alternative egui considered for its maturity and immediate-mode simplicity

#### 6.3 Graphics API
**Recommendation: WGPU**
- Pros: Modern API, safe, cross-platform (Vulkan/Metal/DX12/OpenGL), future-proof
- Cons: Requires shader rewriting from OpenGL
- Shaders written in WGSL (WebGPU Shading Language)

#### 6.4 Error Handling Strategy
- Use `thiserror` for library error types
- Use `anyhow` for application-level error handling
- Propagate errors up to UI layer for user-friendly messages
- Log all errors with context using `tracing`

#### 6.5 Concurrency Model
- UI thread: main event loop
- Connection thread: async runtime for serial/network I/O
- Parser thread: can be on main thread or spawned as needed
- Renderer: main thread (most graphics APIs require this)
- Use channels (`tokio::sync::mpsc`) for cross-thread communication

#### 6.6 State Management Pattern
- Centralized state in `State` struct
- Immutable updates where possible
- Use of interior mutability (`Arc<Mutex<>>` or `Arc<RwLock<>>`) where needed
- State changes trigger UI updates through message passing

### 7. Testing Strategy

#### 7.1 Unit Tests
- Parser: extensive test suite with various G-Code samples
- Connection: mock serial port for testing
- Height map: interpolation accuracy tests
- State management: state transition tests

#### 7.2 Integration Tests
- End-to-end G-Code loading and visualization
- Connection → Parser → Renderer pipeline
- Settings persistence

#### 7.3 Manual Testing
- Real GRBL controller testing
- Various G-Code files (from Fusion 360, FreeCAD, etc.)
- Cross-platform testing
- Performance testing with large files

### 8. Documentation Requirements

#### 8.1 User Documentation
- Installation guide (per platform)
- User manual (based on original Candle manual)
- Quickstart guide
- FAQ
- Troubleshooting guide

#### 8.2 Developer Documentation
- Architecture overview
- Module documentation (rustdoc)
- Contributing guide
- Build instructions
- API documentation

### 9. Performance Considerations

- Large G-Code files (>100k lines) should load within 2 seconds
- Visualization should maintain 60 FPS for files up to 1 million line segments
- Serial communication should have <10ms latency
- UI should remain responsive during long operations (use async/threading)
- Memory usage should be reasonable (<500MB for typical operations)

### 10. Cross-Platform Support

#### 10.1 Target Platforms
- **Primary**: Linux (Ubuntu 20.04+, Arch, Fedora)
- **Primary**: Windows 10/11
- **Secondary**: macOS 12+

#### 10.2 Platform-Specific Considerations
- Serial port permissions (Linux: dialout group)
- File dialog styling
- Keyboard shortcuts (Ctrl vs Cmd)
- Path separators
- Binary packaging (AppImage, MSI installer, .app bundle)

### 11. Migration Notes from C++/Qt

#### 11.1 Qt to Rust Mappings
- `QSerialPort` → `serialport` crate
- `QSettings` → `serde` + JSON/TOML
- `QString` → `String` or `&str`
- `QVector3D` → `glam::Vec3`
- `QOpenGLWidget` → `wgpu` rendering context
- `QThread` → `tokio::task` or `std::thread`
- `QTimer` → `tokio::time::interval`
- Signals/Slots → Channels or callback functions

#### 11.2 Architecture Differences
- Replace Qt's object hierarchy with Rust's ownership model
- Replace signal/slot with message passing (channels)
- Replace Qt's event loop with async runtime + UI framework's loop
- Replace QML/UI files with native Rust UI code

#### 11.3 Features to Preserve
- All GRBL communication protocols
- G-Code parsing behavior (including edge cases)
- Height map algorithms
- Visualization appearance and controls
- Keyboard shortcuts
- Settings structure

#### 11.4 Features to Improve
- More robust error handling
- Better async I/O performance
- Reduced memory allocations in hot paths
- Plugin system architecture (better than original)
- Modern, more maintainable codebase

### 12. Security Considerations

- Validate all G-Code input (prevent code injection in scripts)
- Sanitize file paths (prevent path traversal)
- Secure serial port access (don't expose to scripts without sandboxing)
- No hardcoded credentials for network connections
- Safe handling of untrusted G-Code files

### 13. Licensing

- Target license: **GPLv3** (same as original Candle)
- All dependencies must be GPL-compatible
- Document all third-party licenses in LICENSES.md

### 14. Success Criteria

The rCandle migration will be considered successful when:

1. ✅ Can connect to GRBL controllers via serial port
2. ✅ Can load and display G-Code files in 3D
3. ✅ Can send G-Code commands and receive status updates
4. ✅ Manual jog controls work correctly
5. ✅ Height map creation and compensation works
6. ✅ Settings are persisted between sessions
7. ✅ Runs on Windows and Linux with similar performance
8. ✅ Can handle G-Code files of at least 100,000 lines
9. ✅ UI is responsive and intuitive
10. ✅ All original Candle features are implemented
11. ✅ Comprehensive test coverage (>70% for core modules)
12. ✅ Complete user documentation
13. ✅ At least as fast as original C++ version

### 15. Risk Assessment

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| UI framework immaturity | High | Medium | Choose well-established framework (Iced/egui), have fallback options |
| Graphics API complexity | High | Medium | Start with simple rendering, iterate; extensive testing |
| Serial communication issues | High | Low | Use well-tested crate, comprehensive error handling |
| Performance degradation | Medium | Low | Profile early and often, optimize hot paths |
| G-Code parser bugs | High | Medium | Extensive test suite with real-world files |
| Platform compatibility | Medium | Medium | Test on all platforms regularly, use CI/CD |
| Scope creep | Medium | High | Strict adherence to specification, phase-based approach |

### 16. Future Enhancements (Post-Migration)

- Network-based machine control (multiple clients)
- Camera integration for machine monitoring
- DRO (Digital Read Out) support
- Touchscreen optimization
- Cloud backup of G-Code and settings
- Advanced simulation (collision detection)
- Tool library management
- Multiple language support
- Mobile companion app
- Plugin marketplace

### 17. References

- Original Candle repository: https://github.com/Denvi/Candle
- Original user manual: https://github.com/Denvi/Candle/blob/master/doc/help_en.html
- GRBL documentation: https://github.com/gnea/grbl/wiki
- Rust serialport crate: https://docs.rs/serialport/
- Iced UI framework: https://github.com/iced-rs/iced
- WGPU graphics: https://wgpu.rs/

---

## Appendix A: Example G-Code

```gcode
; Simple toolpath example
G21         ; Metric units
G90         ; Absolute positioning
G0 Z5.000   ; Rapid move to safe height
G0 X0.000 Y0.000  ; Move to start
G1 Z-1.000 F100   ; Plunge
G1 X10.000 F500   ; Cut line
G1 Y10.000        ; Cut line
G1 X0.000         ; Cut line
G1 Y0.000         ; Cut line
G0 Z5.000         ; Retract
M30               ; End program
```

## Appendix B: GRBL Status Report Format

```
<Idle|MPos:0.000,0.000,0.000|FS:0,0|WCO:0.000,0.000,0.000>
```

Components:
- State: Idle, Run, Hold, Alarm, Check, Home
- MPos: Machine position (X, Y, Z)
- FS: Feed rate and spindle speed
- WCO: Work coordinate offset

## Appendix C: Keyboard Shortcuts (to implement)

- `Ctrl+O`: Open file
- `Ctrl+S`: Save file
- `Ctrl+R`: Run program
- `Ctrl+P`: Pause/Resume
- `Ctrl+Q`: Stop
- `Arrow keys`: Jog (with modifiers for step size)
- `Page Up/Down`: Jog Z axis
- `Home`: Home machine
- `Space`: Hold/Resume
- `Esc`: Reset/Abort

---

**Document Version**: 1.0  
**Last Updated**: 2024  
**Status**: Initial Specification
