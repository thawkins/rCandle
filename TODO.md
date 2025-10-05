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
- [ ] Implement serial port connection to GRBL
- [ ] Add port selection UI
- [ ] Handle connection errors gracefully
- [ ] Implement device discovery

### File Operations
- [ ] Complete file save functionality
- [ ] Add recent files list
- [ ] Implement file validation before loading

### GRBL Communication
- [ ] Send G-Code commands to GRBL
- [ ] Parse GRBL responses
- [ ] Handle error responses
- [ ] Implement command queue

### Machine Control
- [ ] Wire up jog controls to GRBL commands
- [ ] Implement homing functionality
- [ ] Add zero position controls
- [ ] Handle machine state transitions

### Execution Control
- [ ] Implement play/pause/stop for program execution
- [ ] Add step mode execution
- [ ] Handle feed/rapid/spindle overrides
- [ ] Monitor execution progress

### Status Monitoring
- [ ] Parse and display machine position (WPos, MPos)
- [ ] Show machine state (Idle, Run, Hold, etc.)
- [ ] Display feed rate and spindle speed
- [ ] Show buffer state

### Advanced Features
- [ ] Tool change support
- [ ] Probe operations
- [ ] Work coordinate systems (G54-G59)
- [ ] Custom macro support

## Documentation
- [ ] Add user guide
- [ ] Document keyboard shortcuts
- [ ] Add developer documentation
- [ ] Create build instructions

## Testing
- [ ] Add unit tests for parser
- [ ] Add tests for serial communication
- [ ] Test on Windows
- [ ] Test on macOS
- [ ] Test on Linux

## Known Issues
- UI interaction not working (see Critical Issues above)
- Documentation incomplete
- Serial port not implemented yet
