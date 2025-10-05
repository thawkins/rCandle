# rCandle TODO List

## Critical Issues

### UI Interaction Not Working
**Priority: HIGH**
**Status: INVESTIGATING**

None of the UI controls (buttons, text fields, etc.) are responding to mouse or keyboard events.

**Symptoms:**
- Buttons don't respond to clicks
- Text fields don't accept input
- Menu items don't work
- All UI elements visible but non-interactive

**Investigation Notes:**
- Code structure looks correct - using `.clicked()` properly per egui immediate mode patterns
- Update method is implemented correctly per eframe::App trait
- Panels are set up in standard way
- May be related to event loop or window setup
- Forum reference checked: https://users.rust-lang.org/t/how-to-use-the-button-pressed-and-released-events-in-egui/104106

**Possible Causes to Investigate:**
1. Window/viewport configuration issue in main.rs
2. Event loop not running properly
3. Input being consumed somewhere before reaching widgets
4. egui/eframe version compatibility issue
5. Platform-specific event handling problem

**Next Steps:**
- Run application with debug logging to verify update() is being called
- Check if mouse/keyboard events are reaching the application at all
- Try minimal egui example to verify egui itself works
- Check eframe version and compatibility

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
- [ ] Implement play/pause/stop for program execution
- [ ] Add step mode execution
- [ ] Handle feed/rapid/spindle overrides
- [ ] Monitor execution progress
- [ ] Display current line being executed
- [ ] Show estimated time remaining

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

## Future Enhancements
- WebSocket and Telnet connection support (infrastructure exists)
- Plugin system for custom tools
- G-Code optimizer
- Simulation mode
- Multi-language support
- Touch screen optimization
- Mobile companion app

