# rCandle Development Roadmap

## Timeline Overview

**Total Duration**: 20 weeks (approximately 5 months)  
**Team Size**: 1-2 developers  
**Start Date**: TBD  
**Target Completion**: TBD + 20 weeks

---

## Phase 1: Foundation (Weeks 1-2)

### Goals
Establish project infrastructure, tooling, and basic scaffolding

### Tasks

#### Week 1
- [x] **Day 1-2**: Project Setup
  - [x] Initialize Cargo workspace
  - [x] Set up Git repository with appropriate .gitignore
  - [ ] Configure CI/CD pipeline (GitHub Actions)
  - [x] Set up code formatting (rustfmt) and linting (clippy)
  - [x] Create project directory structure
  - [x] Write initial README.md and CONTRIBUTING.md

- [x] **Day 3-4**: Core Infrastructure
  - [x] Implement logging infrastructure (tracing + tracing-subscriber)
  - [x] Set up configuration management (serde + config file parsing)
  - [x] Define core error types (thiserror)
  - [x] Create basic CLI interface for testing (clap)
  - [x] Set up unit test framework and patterns

- [ ] **Day 5**: Documentation
  - [x] Write architecture documentation
  - [ ] Set up rustdoc configuration
  - [ ] Create developer onboarding guide
  - [ ] Document build process for all platforms

#### Week 2
- [x] **Day 1-2**: Settings System
  - [x] Define Settings struct
  - [x] Implement TOML/JSON serialization
  - [x] Create default configuration
  - [x] Implement settings validation
  - [x] Add settings loading/saving functionality
  - [x] Write unit tests for settings

- [x] **Day 3-4**: State Management Foundation
  - [x] Define AppState structure
  - [x] Define MachineState structure
  - [x] Implement state synchronization primitives (Arc<RwLock>)
  - [x] Create state update patterns
  - [x] Write state management tests

- [x] **Day 5**: Integration & Testing
  - [x] Verify all components compile
  - [x] Run initial test suite
  - [ ] Set up code coverage reporting
  - [x] Address any compilation warnings
  - [ ] Document Phase 1 progress

### Deliverables
- ✅ Compiling skeleton application
- ✅ Configuration loading/saving working
- ✅ Logging to file and console operational
- ✅ CI/CD pipeline active
- ✅ Core error handling in place
- ✅ Basic test infrastructure

### Success Criteria
- All code compiles without warnings
- Tests pass with >80% coverage
- Can load and save configuration files
- Logging works correctly at different levels

---

## Phase 2: G-Code Parser (Weeks 3-4)

### Goals
Implement robust G-Code parsing with comprehensive test coverage

### Tasks

#### Week 3
- [ ] **Day 1-2**: Lexer/Tokenizer
  - [ ] Implement G-Code tokenizer using nom
  - [ ] Handle comments (parentheses and semicolon)
  - [ ] Parse commands (G, M, T, etc.)
  - [ ] Parse parameters (X, Y, Z, F, S, etc.)
  - [ ] Handle line numbers and checksums
  - [ ] Write tokenizer tests with edge cases

- [ ] **Day 3-4**: Parser
  - [ ] Implement command parsing
  - [ ] Handle modal groups correctly
  - [ ] Implement parser state (G90/G91, units, etc.)
  - [ ] Parse parameter values (handle expressions if needed)
  - [ ] Error recovery and reporting
  - [ ] Write parser tests

- [ ] **Day 5**: Segment Generation
  - [ ] Define segment types (Line, Arc, Rapid)
  - [ ] Implement coordinate transformation
  - [ ] Handle relative/absolute modes
  - [ ] Write segment generation tests

#### Week 4
- [ ] **Day 1-2**: Preprocessor
  - [ ] Implement arc expansion (G2/G3 to line segments)
  - [ ] Unit conversion (G20/G21)
  - [ ] Coordinate system transformation
  - [ ] Remove unnecessary rapids
  - [ ] Write preprocessor tests

- [ ] **Day 3**: Arc Properties
  - [ ] Implement arc center calculation (I, J, K parameters)
  - [ ] Implement arc radius calculation (R parameter)
  - [ ] Handle full circles
  - [ ] Validate arc parameters
  - [ ] Write arc calculation tests

- [ ] **Day 4**: Integration & Testing
  - [ ] Test with real G-Code files (from Fusion 360, FreeCAD)
  - [ ] Test with edge cases (empty lines, comments only)
  - [ ] Test with malformed G-Code
  - [ ] Performance testing with large files
  - [ ] Fix bugs discovered during testing

- [ ] **Day 5**: Documentation
  - [ ] Document parser API
  - [ ] Write usage examples
  - [ ] Document supported G-Code subset
  - [ ] Document known limitations

### Deliverables
- ✅ Fully functional G-Code parser
- ✅ Parse standard G-Code files accurately
- ✅ Extract all commands and parameters
- ✅ Convert to internal segment representation
- ✅ Comprehensive test suite (>90% coverage)

### Success Criteria
- Parse 100+ real-world G-Code files without errors
- Handle files up to 100k lines in <2 seconds
- All unit tests pass
- Edge cases handled gracefully

---

## Phase 3: Connection Module (Weeks 5-6)

### Goals
Implement serial communication with GRBL controllers

### Tasks

#### Week 5
- [ ] **Day 1-2**: Connection Trait & Serial Implementation
  - [ ] Define Connection trait
  - [ ] Implement SerialConnection with tokio_serial
  - [ ] Handle port opening/closing
  - [ ] Implement async send/receive
  - [ ] Handle connection errors and reconnection
  - [ ] Write serial connection tests (with mocks)

- [ ] **Day 3-4**: GRBL Protocol Handling
  - [ ] Study GRBL protocol documentation (interface.md, commands.md)
  - [ ] Implement GRBL command formatting
  - [ ] Parse GRBL responses (ok, error:X)
  - [ ] Parse GRBL status reports (<...>)
  - [ ] Parse GRBL alarms and error messages
  - [ ] Handle real-time commands (?, !, ~, 0x18)
  - [ ] Implement GRBL settings ($$) parsing
  - [ ] Write protocol parsing tests

- [ ] **Day 5**: Command Queue
  - [ ] Implement command queue (bounded channel)
  - [ ] Handle command acknowledgments
  - [ ] Implement flow control (wait for "ok")
  - [ ] Handle command timeouts
  - [ ] Write queue management tests

#### Week 6
- [ ] **Day 1-2**: Connection Manager
  - [ ] Implement ConnectionManager
  - [ ] Manage connection lifecycle
  - [ ] Coordinate command sending and response receiving
  - [ ] Broadcast status updates
  - [ ] Handle disconnections gracefully
  - [ ] Write connection manager tests

- [ ] **Day 3**: Alternative Connections
  - [ ] Implement TelnetConnection (basic)
  - [ ] Implement WebSocketConnection (basic)
  - [ ] Write tests for alternative connections

- [ ] **Day 4**: Integration & Testing
  - [ ] Test with real GRBL controller (Arduino + GRBL)
  - [ ] Test reconnection scenarios
  - [ ] Test error handling
  - [ ] Performance testing (command throughput)
  - [ ] Fix bugs discovered during testing

- [ ] **Day 5**: Documentation
  - [ ] Document connection API
  - [ ] Write usage examples
  - [ ] Document GRBL protocol support
  - [ ] Document troubleshooting steps

### Deliverables
- ✅ Connect to GRBL via serial port
- ✅ Send commands and receive responses reliably
- ✅ Parse GRBL status messages correctly
- ✅ Handle real-time commands
- ✅ Graceful error handling and reconnection

### Success Criteria
- Successfully connect to GRBL controller
- Send 1000 commands without errors
- Parse all GRBL responses correctly
- Maintain connection stability for 1+ hour

---

## Phase 4: State Management (Week 7)

### Goals
Implement comprehensive state tracking for machine and program

### Tasks

- [ ] **Day 1**: Machine State
  - [ ] Implement MachineState structure
  - [ ] Track machine status (Idle, Run, Hold, etc.)
  - [ ] Track positions (machine, work, offsets)
  - [ ] Track spindle state
  - [ ] Track feed rate and overrides
  - [ ] Write state management tests

- [ ] **Day 2**: Coordinate Systems
  - [ ] Implement coordinate system management
  - [ ] Handle work coordinate offsets (G54-G59)
  - [ ] Implement coordinate transformation utilities
  - [ ] Write coordinate system tests

- [ ] **Day 3**: Program State
  - [ ] Implement ProgramState structure
  - [ ] Track current line execution
  - [ ] Track execution state (Running, Paused, etc.)
  - [ ] Calculate progress percentage
  - [ ] Write program state tests

- [ ] **Day 4**: State Updates & Synchronization
  - [ ] Implement state update mechanisms
  - [ ] Add change notification (pub/sub pattern)
  - [ ] Handle concurrent state access safely
  - [ ] Write synchronization tests

- [ ] **Day 5**: Integration & Documentation
  - [ ] Integrate state management with connection module
  - [ ] Test state updates from GRBL responses
  - [ ] Document state management API
  - [ ] Write usage examples

### Deliverables
- ✅ Track machine position and status accurately
- ✅ Manage work/machine coordinates
- ✅ Store and load application settings
- ✅ Thread-safe state access

### Success Criteria
- State accurately reflects GRBL controller state
- No race conditions in state access
- State persists correctly between sessions

---

## Phase 5: Visualization Core (Weeks 8-10)

### Goals
Implement 3D visualization of toolpaths using WGPU

### Tasks

#### Week 8
- [ ] **Day 1-2**: WGPU Setup
  - [ ] Initialize WGPU device and surface
  - [ ] Set up render pipeline
  - [ ] Implement basic vertex shader
  - [ ] Implement basic fragment shader
  - [ ] Render a simple triangle (verification)

- [ ] **Day 3-4**: Camera System
  - [ ] Implement Camera structure
  - [ ] Implement view and projection matrices
  - [ ] Implement camera controls (orbit, pan, zoom)
  - [ ] Handle mouse/keyboard input for camera
  - [ ] Write camera tests

- [ ] **Day 5**: Vertex Management
  - [ ] Define Vertex structure
  - [ ] Implement vertex buffer management
  - [ ] Implement index buffer management
  - [ ] Write buffer management tests

#### Week 9
- [ ] **Day 1-2**: G-Code Path Rendering
  - [ ] Implement GCodeDrawer
  - [ ] Convert segments to vertex data
  - [ ] Implement line rendering
  - [ ] Implement coloring by feed rate/type
  - [ ] Handle large toolpaths efficiently
  - [ ] Write rendering tests

- [ ] **Day 3**: Additional Drawers
  - [ ] Implement ToolDrawer (current position)
  - [ ] Implement OriginDrawer (coordinate origin)
  - [ ] Implement GridDrawer (reference grid)
  - [ ] Write drawer tests

- [ ] **Day 4**: Scene Management
  - [ ] Implement Scene structure
  - [ ] Manage multiple drawables
  - [ ] Implement transform hierarchy
  - [ ] Handle drawable updates
  - [ ] Write scene management tests

- [ ] **Day 5**: Rendering Optimization
  - [ ] Implement frustum culling
  - [ ] Optimize vertex buffer updates
  - [ ] Profile rendering performance
  - [ ] Implement level-of-detail if needed

#### Week 10
- [ ] **Day 1-2**: Advanced Rendering
  - [ ] Implement BoundsDrawer (machine limits)
  - [ ] Implement SelectionDrawer
  - [ ] Add lighting/shading for better depth perception
  - [ ] Implement anti-aliasing

- [ ] **Day 3-4**: Integration & Testing
  - [ ] Test with various G-Code files
  - [ ] Test performance with large files (1M+ line segments)
  - [ ] Test camera controls
  - [ ] Fix rendering bugs

- [ ] **Day 5**: Documentation
  - [ ] Document rendering API
  - [ ] Document shader code
  - [ ] Write usage examples
  - [ ] Document performance characteristics

### Deliverables
- ✅ Display loaded G-Code in 3D
- ✅ Interactive camera controls (60 FPS minimum)
- ✅ Real-time tool position updates
- ✅ Visual clarity and good aesthetics

### Success Criteria
- Render 1 million line segments at 60 FPS
- Smooth camera controls with no lag
- Visually accurate representation of toolpaths
- No graphical artifacts or glitches

---

## Phase 6: UI Framework (Weeks 11-13)

### Goals
Build complete user interface with Iced

### Tasks

#### Week 11
- [ ] **Day 1-2**: egui/eframe Application Setup
  - [ ] Set up eframe application structure
  - [ ] Implement basic main window with eframe::App trait
  - [ ] Set up immediate mode UI patterns
  - [ ] Implement basic layout (top panel, central, bottom)
  - [ ] Add menu bar with egui menus

- [ ] **Day 3-4**: Layout & Panels
  - [ ] Implement main content split (left/right panels)
  - [ ] Create collapsible panel framework
  - [ ] Add side panels for controls
  - [ ] Implement panel state persistence
  - [ ] Style panels with egui styling

- [ ] **Day 5**: File Operations
  - [ ] Integrate rfd for native file dialogs
  - [ ] Add Open/Save file functionality
  - [ ] Integrate with parser
  - [ ] Update program state on file load
  - [ ] Display file info in UI

#### Week 12
- [ ] **Day 1-2**: G-Code Editor Widget
  - [ ] Implement egui::TextEdit for code editing
  - [ ] Add line numbers display
  - [ ] Implement basic syntax highlighting (color keywords)
  - [ ] Add scrolling support with egui::ScrollArea
  - [ ] Implement current line highlighting
  - [ ] Add find/replace functionality

- [ ] **Day 3**: Console Widget
  - [ ] Implement console display with egui::ScrollArea
  - [ ] Add auto-scrolling
  - [ ] Implement command input field
  - [ ] Add log filtering (error, warning, info)
  - [ ] Implement command history with up/down arrows
  - [ ] Add timestamp display

- [ ] **Day 4**: 3D Viewport Integration
  - [ ] Create custom egui widget for WGPU surface
  - [ ] Integrate wgpu rendering with egui::CentralPanel
  - [ ] Handle viewport resizing in immediate mode
  - [ ] Forward mouse/keyboard input to camera
  - [ ] Update viewport on state changes
  - [ ] Add viewport controls overlay

- [ ] **Day 5**: Control Panels (Part 1)
  - [ ] Implement State Panel with egui::Grid
  - [ ] Implement Control Panel (connect/disconnect buttons)
  - [ ] Implement Coordinate System Panel with labels
  - [ ] Add real-time updates using egui's repaint system

#### Week 13
- [ ] **Day 1**: Control Panels (Part 2)
  - [ ] Implement Spindle Panel with egui::Slider
  - [ ] Implement Jog Panel (button grid with egui::Grid)
  - [ ] Implement Override Panel (sliders for feed/spindle)
  - [ ] Add tooltips using egui::on_hover_text

- [ ] **Day 2**: Settings Dialog
  - [ ] Implement settings window with egui::Window
  - [ ] Add tabbed interface using egui::TabBar (or manual tabs)
  - [ ] Implement form widgets for settings
  - [ ] Add validation feedback
  - [ ] Save/load settings integration

- [ ] **Day 3**: Program Control
  - [ ] Add Run/Pause/Stop buttons
  - [ ] Implement progress bar with egui::ProgressBar
  - [ ] Add time estimates display
  - [ ] Implement program execution flow with state updates
  - [ ] Add hotkeys for common actions

- [ ] **Day 4**: Integration & Testing
  - [ ] Test all UI interactions in immediate mode
  - [ ] Test on different screen sizes
  - [ ] Test keyboard shortcuts
  - [ ] Fix UI bugs and layout issues
  - [ ] Optimize UI performance (minimize redraws)

- [ ] **Day 5**: Theming & Polish
  - [ ] Configure egui theme (light/dark using egui::Visuals)
  - [ ] Add application icon
  - [ ] Polish layout and spacing with egui::Style
  - [ ] Improve responsiveness
  - [ ] Add loading indicators

### Deliverables
- ✅ Functional GUI application
- ✅ Load/save G-Code files
- ✅ Send commands to GRBL
- ✅ View machine status in real-time
- ✅ Manual jog controls functional
- ✅ Settings dialog operational

### Success Criteria
- UI is responsive (<16ms frame time)
- All controls work as expected
- Settings persist correctly
- No UI freezing during operations
- Intuitive and user-friendly

---

## Phase 7: Height Map (Weeks 14-15)

### Goals
Implement surface scanning and height compensation

### Tasks

#### Week 14
- [ ] **Day 1-2**: Height Map Data Structure
  - [ ] Implement Grid2D structure
  - [ ] Implement HeightMap structure
  - [ ] Add serialization/deserialization
  - [ ] Write data structure tests

- [ ] **Day 3-4**: Probing Sequence
  - [ ] Implement probe area definition UI
  - [ ] Generate probe points grid
  - [ ] Implement probing sequence (G38.2)
  - [ ] Store probe results
  - [ ] Handle probe errors
  - [ ] Write probing tests

- [ ] **Day 5**: Interpolation
  - [ ] Implement bilinear interpolation
  - [ ] Implement bicubic interpolation (optional)
  - [ ] Validate interpolation accuracy
  - [ ] Write interpolation tests

#### Week 15
- [ ] **Day 1-2**: G-Code Transformation
  - [ ] Implement height compensation algorithm
  - [ ] Apply height map to line segments
  - [ ] Apply height map to arc segments
  - [ ] Validate transformed G-Code
  - [ ] Write transformation tests

- [ ] **Day 3**: Visualization
  - [ ] Implement HeightMapDrawer (grid overlay)
  - [ ] Implement height color mapping
  - [ ] Visualize probe points
  - [ ] Add visualization controls

- [ ] **Day 4**: UI Integration
  - [ ] Add Height Map Panel
  - [ ] Add probe controls
  - [ ] Add height map visualization toggle
  - [ ] Add save/load height map

- [ ] **Day 5**: Testing & Documentation
  - [ ] Test probing with real machine
  - [ ] Test height compensation accuracy
  - [ ] Document height map workflow
  - [ ] Write user guide for height mapping

### Deliverables
- ✅ Create height maps through probing
- ✅ Apply height compensation to G-Code
- ✅ Visualize height maps in 3D viewport
- ✅ Save/load height maps

### Success Criteria
- Probing completes without errors
- Height compensation is accurate (within 0.1mm)
- Height map visualization is clear
- Workflow is intuitive

---

## Phase 8: Advanced Features (Weeks 16-17)

### Goals
Implement scripting, user commands, and additional features

### Tasks

#### Week 16
- [ ] **Day 1-2**: Scripting Engine
  - [ ] Integrate Rhai engine
  - [ ] Define script API
  - [ ] Implement API bindings
  - [ ] Add script editor
  - [ ] Test script execution

- [ ] **Day 3**: User Commands
  - [ ] Implement user command storage
  - [ ] Add user command panel
  - [ ] Implement custom buttons
  - [ ] Add command editor dialog
  - [ ] Test user commands

- [ ] **Day 4-5**: Additional Connection Types
  - [ ] Complete TelnetConnection implementation
  - [ ] Complete WebSocketConnection implementation
  - [ ] Add connection type selection in UI
  - [ ] Test alternative connections

#### Week 17
- [ ] **Day 1**: Override Controls
  - [ ] Implement feed rate override (0-200%)
  - [ ] Implement spindle speed override
  - [ ] Implement rapid override
  - [ ] Test override functionality

- [ ] **Day 2**: Advanced Visualization
  - [ ] Add measurement tools
  - [ ] Add selection tools
  - [ ] Implement view presets (top, front, side, iso)
  - [ ] Add screenshot/export functionality

- [ ] **Day 3**: Keyboard Shortcuts
  - [ ] Implement all keyboard shortcuts
  - [ ] Add keyboard shortcut customization
  - [ ] Display shortcuts in help

- [ ] **Day 4**: Additional Tools
  - [ ] Add G-Code statistics (time estimate, bounds)
  - [ ] Add work coordinate preset buttons
  - [ ] Add macro recorder (optional)

- [ ] **Day 5**: Integration & Testing
  - [ ] Test all advanced features
  - [ ] Fix bugs
  - [ ] Document features

### Deliverables
- ✅ Execute custom scripts
- ✅ User-defined command buttons
- ✅ Alternative connection methods functional
- ✅ Override controls working
- ✅ Enhanced visualization features

### Success Criteria
- Scripts execute correctly without crashes
- User commands save and load properly
- Alternative connections work reliably
- Override controls respond in real-time

---

## Phase 9: Polish & Testing (Weeks 18-20)

### Goals
Comprehensive testing, bug fixes, optimization, and documentation

### Tasks

#### Week 18
- [ ] **Day 1-3**: Comprehensive Testing
  - [ ] Integration testing with real GRBL
  - [ ] Test all features end-to-end
  - [ ] Test error scenarios
  - [ ] Test on all target platforms (Windows, Linux, macOS)
  - [ ] Create test matrix
  - [ ] Fix all critical bugs

- [ ] **Day 4-5**: Bug Fixing
  - [ ] Address all reported bugs
  - [ ] Fix edge cases
  - [ ] Improve error messages
  - [ ] Add more input validation

#### Week 19
- [ ] **Day 1-2**: Performance Optimization
  - [ ] Profile application performance
  - [ ] Optimize hot paths
  - [ ] Reduce memory allocations
  - [ ] Improve rendering performance
  - [ ] Optimize parser for large files

- [ ] **Day 3**: UI/UX Refinement
  - [ ] Improve visual design
  - [ ] Add animations/transitions
  - [ ] Improve error dialogs
  - [ ] Add tooltips
  - [ ] Improve accessibility

- [ ] **Day 4-5**: Cross-Platform Testing
  - [ ] Test on Windows 10/11
  - [ ] Test on Ubuntu/Fedora/Arch
  - [ ] Test on macOS
  - [ ] Fix platform-specific issues
  - [ ] Verify serial port permissions

#### Week 20
- [ ] **Day 1-2**: Documentation
  - [ ] Complete user manual
  - [ ] Write installation guide for each platform
  - [ ] Create quickstart guide
  - [ ] Write FAQ
  - [ ] Create troubleshooting guide
  - [ ] Complete API documentation
  - [ ] Write developer guide

- [ ] **Day 3**: Packaging
  - [ ] Create Linux AppImage
  - [ ] Create Windows installer (MSI)
  - [ ] Create macOS .app bundle
  - [ ] Test installation on clean systems
  - [ ] Write installation instructions

- [ ] **Day 4**: Release Preparation
  - [ ] Finalize version number
  - [ ] Write release notes
  - [ ] Create release on GitHub
  - [ ] Upload binaries
  - [ ] Update README with download links

- [ ] **Day 5**: Launch
  - [ ] Announce release
  - [ ] Monitor for issues
  - [ ] Provide user support
  - [ ] Celebrate! 🎉

### Deliverables
- ✅ Stable, tested application
- ✅ Complete user manual
- ✅ Installation packages for all platforms
- ✅ Developer documentation
- ✅ Release on GitHub

### Success Criteria
- All tests pass on all platforms
- No critical bugs remain
- Performance meets requirements
- Documentation is complete and accurate
- Installation packages work on clean systems

---

## Risk Mitigation

### High Priority Risks

1. **UI Framework Limitations**
   - **Mitigation**: Evaluate Iced early in Phase 6; have egui as backup
   - **Contingency**: If Iced proves inadequate, switch to egui (1-2 week delay)

2. **WGPU Rendering Issues**
   - **Mitigation**: Start with simple rendering in Phase 5; incremental complexity
   - **Contingency**: Fall back to glium/glow if WGPU is problematic (1 week delay)

3. **Serial Communication Reliability**
   - **Mitigation**: Extensive testing in Phase 3; robust error handling
   - **Contingency**: Implement alternative communication strategies

4. **Parser Edge Cases**
   - **Mitigation**: Build comprehensive test suite in Phase 2
   - **Contingency**: Allocate buffer time for parser bug fixes

5. **Scope Creep**
   - **Mitigation**: Strict adherence to specification; defer non-essential features
   - **Contingency**: Move low-priority features to post-1.0 roadmap

### Buffer Time
- 2 weeks of buffer distributed throughout the schedule
- Allocated to critical path items (Parser, Connection, UI, Testing)

---

## Post-Launch Roadmap (v1.1+)

### v1.1 (Month 6)
- Plugin system architecture
- Multi-language support (i18n)
- Advanced simulation features
- Performance improvements based on user feedback

### v1.2 (Month 7-8)
- Camera integration for machine monitoring
- Tool library management
- Cloud sync for settings and files
- Mobile companion app (view-only)

### v1.3 (Month 9-10)
- Network-based control (multiple clients)
- DRO (Digital Read Out) integration
- Collision detection simulation
- Advanced macros and automation

---

## Success Metrics

### Development Metrics
- Code coverage: >70% for core modules
- Build time: <2 minutes for release build
- Binary size: <50MB
- Startup time: <3 seconds

### Performance Metrics
- Parse 100k line file: <2 seconds
- Render 1M line segments: 60 FPS
- Command latency: <10ms
- Memory usage: <500MB typical

### User Metrics (Post-Launch)
- Installation success rate: >95%
- Crash-free sessions: >99%
- User satisfaction: >4.5/5
- GitHub stars: 100+ in first month

---

## Team Responsibilities

### Lead Developer
- Overall architecture
- Core modules (Connection, Parser, State)
- Code reviews
- Release management

### UI Developer (if separate)
- UI implementation with Iced
- UX design
- Theme development
- Accessibility

### QA/Testing (can be same person)
- Test plan development
- Integration testing
- Platform-specific testing
- Bug triage

---

## Communication & Reporting

### Weekly Updates
- Progress against roadmap
- Blockers and risks
- Decisions needed
- Next week priorities

### Milestone Reviews
- End of each phase
- Demo of completed features
- Retrospective
- Roadmap adjustments if needed

---

This roadmap provides a structured path to completing the rCandle migration while maintaining flexibility for adjustments based on progress and discoveries during development.
