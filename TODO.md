# rCandle TODO List

## Critical Issues

### UI Interaction Not Working
**Priority: HIGH**
**Status: ✅ RESOLVED**

**Issue (Resolved):**
UI controls were not responding to mouse or keyboard events due to outdated egui/eframe versions and API compatibility issues.

**Root Cause:**
- Outdated egui/eframe versions (0.27.x)
- WGPU 0.19 compatibility issues
- API breaking changes in egui 0.28 and WGPU 0.20

**Resolution (January 2025):**
1. ✅ Updated egui from 0.27 to 0.28
2. ✅ Updated eframe from 0.27 to 0.28
3. ✅ Updated wgpu from 0.19 to 0.20
4. ✅ Fixed `run_native` API call to return `Ok(Box<...>)`
5. ✅ Removed `.with_focused(true)` (method doesn't exist in egui 0.28)
6. ✅ Added missing `compilation_options` field to VertexState and FragmentState
7. ✅ Fixed all deprecated `clamp_range` to use `range` instead
8. ✅ Removed unused `mut` qualifiers and unsafe code
9. ✅ Build succeeds without errors
10. ✅ All 133 tests passing
11. ✅ **UI interaction confirmed working**

**Verification:**
- ✅ Application launches successfully
- ✅ Mouse interactions working (buttons, menus, sliders)
- ✅ Keyboard input working (text fields, shortcuts)
- ✅ All UI elements responsive
- ✅ 3D visualization interactive (camera controls)

**Documentation:**
- See `COMPILATION_FIX_SUMMARY.md` for build fix details
- See `BUILD_FIX_SESSION.md` for complete resolution process
- See `TASK_COMPLETION_SUMMARY.md` for comprehensive overview

## Phase 8: Advanced Features - COMPLETE ✅

### Scripting System ✅
- [x] Integrate Rhai scripting engine
- [x] Define script API
- [x] Implement API bindings (machine control, status, program control)
- [x] Script executor with lifecycle management
- [x] Script library for managing user scripts
- [x] Add script editor UI
- [x] Wire up script command processing
- [ ] Test script execution (pending hardware connection)

### User Commands ✅
- [x] Implement user command storage structure
- [x] Create default command library
- [x] User command with multiple GRBL commands
- [x] Category organization
- [x] Confirmation dialogs support
- [x] Keyboard shortcuts support
- [x] Add user command panel UI
- [x] Implement custom buttons in UI
- [x] Add command editor dialog
- [ ] Test user commands with connection (pending hardware)

### Override Controls ✅
- [x] Implement feed rate override (10-200%)
- [x] Implement spindle speed override (10-200%)
- [x] Implement rapid override (25%, 50%, 100%)
- [x] Override state tracking
- [x] Real-time command byte generation
- [x] Unit tests for overrides
- [x] Add override control UI (sliders/buttons)
- [x] Wire up to connection manager
- [ ] Test override functionality with GRBL (pending hardware)

### Advanced Visualization ✅
- [x] Implement view presets (7 views)
- [x] Camera positioning for presets
- [x] Distance and center calculation
- [x] Unit tests for view presets
- [x] Add view preset buttons to UI
- [ ] Add measurement tools (future enhancement)
- [ ] Add selection tools (future enhancement)
- [ ] Add screenshot/export functionality (future enhancement)

### Additional Connection Types ⏸️
- [ ] Complete TelnetConnection implementation
- [ ] Complete WebSocketConnection implementation
- [ ] Add connection type selection in UI
- [ ] Test alternative connections
Note: Infrastructure exists, implementation deferred

## Phase 6: Complete Remaining Features

### Connection Management
- [x] Implement serial port connection infrastructure
- [x] Add port selection UI
- [x] Wire up connect/disconnect buttons
- [x] Add connection status indicator
- [x] Implement device discovery (list ports)
- [ ] Handle connection errors gracefully with user feedback
- [ ] Store and reuse ConnectionManager instance
- [ ] Implement automatic reconnection on disconnect

### GRBL Communication
- [x] Command queue infrastructure
- [x] Send G-Code commands to GRBL
- [x] Wire up jog controls to GRBL commands
- [x] Implement homing functionality
- [x] Add zero position controls
- [ ] Parse GRBL responses in UI
- [ ] Handle error responses with user feedback
- [ ] Implement response handling loop

### File Operations
- [x] Open G-Code file functionality
- [x] Display file content in editor
- [x] Parse and visualize G-Code
- [ ] Complete file save functionality
- [ ] Add recent files list
- [ ] Implement file validation before loading
- [ ] Add unsaved changes warning

### Machine Control
- [x] Jog controls UI and commands
- [x] Homing button and command
- [x] Zero position controls
- [ ] Handle machine state transitions
- [ ] Implement feed hold / resume
- [ ] Add soft reset functionality
- [ ] Implement safety interlocks

### Execution Control
- [x] Implement play/pause/stop for program execution
- [x] Add step mode execution
- [x] Handle feed/rapid/spindle overrides (UI implemented)
- [x] Monitor execution progress (UI implemented)
- [x] Display current line being executed
- [x] Show estimated time remaining
- [ ] Connect execution controls to GRBL connection manager
- [ ] Handle execution errors and recovery

### Status Monitoring
- [ ] Parse and display machine position (WPos, MPos)
- [ ] Show machine state (Idle, Run, Hold, etc.)
- [ ] Display feed rate and spindle speed
- [ ] Show buffer state
- [ ] Update status in real-time
- [ ] Add status query interval configuration

### Advanced Features
- [ ] Tool change support
- [ ] Probe operations
- [ ] Work coordinate systems (G54-G59)
- [ ] Custom macro support
- [ ] Keyboard shortcuts for common operations
- [ ] Customizable button layouts
- [x] Settings dialog with comprehensive configuration options

### UI Polish
- [x] Settings dialog implementation
- [x] Tabbed settings interface
- [x] Form validation through widget constraints
- [x] Settings persistence
- [ ] Theme switching (dark/light mode from settings)
- [ ] Custom fonts and sizing
- [ ] Keyboard shortcut configuration
- [ ] Layout customization

### 3D Visualization
- [x] Basic 3D renderer infrastructure
- [x] Display toolpath
- [x] Camera controls (rotate, pan, zoom)
- [x] Zoom to fit functionality
- [ ] Show current tool position in 3D view
- [ ] Highlight current segment being executed
- [ ] Add measurement tools
- [ ] Implement 2D/3D view switching

## Documentation
- [ ] Add user guide
- [ ] Document keyboard shortcuts
- [ ] Add developer documentation
- [ ] Create build instructions
- [ ] Add contribution guidelines
- [ ] Document GRBL protocol integration

## Testing
- [ ] Add unit tests for parser
- [ ] Add tests for serial communication
- [ ] Add integration tests for command queue
- [ ] Test on Windows
- [ ] Test on macOS
- [ ] Test on Linux
- [ ] Test with real GRBL hardware

## Known Issues
- **UI interaction not working** (see Critical Issues above)
- ConnectionManager not stored after connection (needs Arc<Mutex<>> handling)
- No response handling loop implemented yet
- Status updates not being parsed and displayed
- Console messages from async tasks not showing up

## Completed Tasks
- ✅ Project structure and basic scaffolding
- ✅ G-Code parser implementation
- ✅ G-Code preprocessor
- ✅ 3D renderer with WGPU
- ✅ Serial port connection infrastructure
- ✅ GRBL command formatting
- ✅ Command queue implementation
- ✅ Connection manager
- ✅ Basic UI layout with egui
- ✅ G-Code editor widget
- ✅ Console widget
- ✅ Connection UI components
- ✅ Jog controls
- ✅ File open/save dialogs
- ✅ Settings management
- ✅ Program execution controls (UI)
- ✅ Settings dialog with tabbed interface
- ✅ Scripting engine with Rhai
- ✅ User command system
- ✅ Override controls (feed, spindle, rapid)
- ✅ View presets for camera

## Future Enhancements
- WebSocket and Telnet connection support (infrastructure exists)
- Plugin system for custom tools
- G-Code optimizer
- Simulation mode
- Multi-language support
- Touch screen optimization
- Mobile companion app

