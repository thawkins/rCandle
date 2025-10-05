# Next Steps - rCandle Development

**Created:** January 2025  
**Current Status:** UI fix in progress, build ongoing  
**Priority:** Complete UI interaction testing

---

## Immediate Actions (Next 30 minutes)

### 1. Complete Build ⏳
**Status:** In progress (344/577 crates = 59%)

**Action:** Wait for build completion
```bash
# Build is running async in background
# Check status with:
ps aux | grep cargo
```

**Expected Result:** Successful compilation of minimal_ui_test example

### 2. Test Minimal UI Example 🎯
**Priority:** HIGH - This determines next steps

**Action:**
```bash
cargo run --example minimal_ui_test
```

**Test Checklist:**
- [ ] Window appears
- [ ] Button is visible
- [ ] Button responds to click
- [ ] Counter increments
- [ ] Text field accepts input
- [ ] Reset button works

**Success Criteria:**
- All checklist items pass
- Console shows "Button clicked!" messages
- UI is fully interactive

**If Successful:** Proceed to step 3
**If Failed:** See "Troubleshooting" section below

### 3. Build Full Application
**Condition:** Only if minimal test succeeds

**Action:**
```bash
cargo build --release 2>&1 | tee build.log
```

**Expected:** Clean build without errors
**Duration:** 15-20 minutes (first time)

---

## Short-term Actions (Next 1-2 days)

### 4. Test Full Application UI
**Condition:** After successful build

**Action:**
```bash
cargo run --release
```

**Systematic Testing:**

#### A. Window Startup
- [ ] Application window opens
- [ ] All panels visible
- [ ] Menu bar rendered
- [ ] Status bar shows "Ready"

#### B. Menu Bar
- [ ] File → Open responds
- [ ] File → Save responds
- [ ] File → Exit works
- [ ] Connection menu items work
- [ ] Edit menu items work
- [ ] View menu items work

#### C. File Operations
- [ ] Open G-Code file dialog appears
- [ ] File loads into editor
- [ ] Editor displays content
- [ ] Syntax highlighting works
- [ ] Find/Replace opens (Ctrl+F)

#### D. Connection Panel
- [ ] Port dropdown shows available ports
- [ ] Baud rate can be changed
- [ ] Connect button responds
- [ ] Status updates shown

#### E. Control Panel
- [ ] Jog buttons respond
- [ ] Step size can be changed
- [ ] Homing button works
- [ ] Zero position buttons respond
- [ ] Override sliders move
- [ ] User command buttons visible

#### F. 3D Visualization
- [ ] Viewport renders
- [ ] Mouse drag rotates camera
- [ ] Scroll wheel zooms
- [ ] View preset buttons work
- [ ] Toolpath displays when file loaded

#### G. Console Panel
- [ ] Console displays messages
- [ ] Command history works
- [ ] Color coding applied
- [ ] Scroll works

#### H. Program Execution
- [ ] Run/Pause/Stop buttons enabled when appropriate
- [ ] Progress bar updates
- [ ] Line counter updates
- [ ] Timer shows elapsed time
- [ ] Step mode checkbox works

#### I. Settings Dialog
- [ ] Settings opens (via menu or Ctrl+,)
- [ ] Tabs switch correctly
- [ ] Values can be changed
- [ ] Save/Cancel work
- [ ] Settings persist

### 5. Connection Testing
**Prerequisites:** 
- GRBL device or simulator
- USB serial connection available

**Setup:**
```bash
# Option 1: Real hardware
# Connect GRBL device via USB

# Option 2: Simulator (if available)
# Start GRBL simulator on virtual serial port
```

**Tests:**
- [ ] Port appears in dropdown
- [ ] Connection succeeds
- [ ] Status queries work
- [ ] Machine state displays
- [ ] Position updates
- [ ] Console shows communication

### 6. G-Code Testing
**Prerequisites:** Sample G-Code file

**Tests:**
- [ ] Load sample file (from examples/)
- [ ] Parse succeeds
- [ ] Toolpath renders in 3D
- [ ] Editor shows content
- [ ] Line count correct

**Sample Files Available:**
```
examples/
├── simple_square.gcode
├── circle_test.gcode
└── (other test files)
```

---

## Medium-term Actions (Next 1-2 weeks)

### 7. GRBL Protocol Testing
**Prerequisites:** Working connection

**Tests:**
- [ ] Send jog commands
- [ ] Verify machine moves
- [ ] Test homing cycle
- [ ] Test zero position setting
- [ ] Send G-code program
- [ ] Verify execution
- [ ] Test pause/resume
- [ ] Test stop/reset
- [ ] Test feed override
- [ ] Test spindle override
- [ ] Test rapid override

### 8. Advanced Features Testing
**Tests:**
- [ ] Create user script
- [ ] Execute script
- [ ] Test user commands
- [ ] Test keyboard shortcuts
- [ ] Test all view presets
- [ ] Test theme switching
- [ ] Test settings persistence

### 9. Error Handling
**Tests:**
- [ ] Disconnect during operation
- [ ] Invalid G-code file
- [ ] GRBL error responses
- [ ] File not found
- [ ] Permission denied
- [ ] Buffer overflow
- [ ] Communication timeout

### 10. Performance Testing
**Tests:**
- [ ] Large G-code file (10,000+ lines)
- [ ] Complex toolpath rendering
- [ ] Rapid status updates
- [ ] Memory usage over time
- [ ] CPU usage during rendering
- [ ] Response time to inputs

---

## Troubleshooting

### If Minimal Test Fails

#### Option A: Update to Latest egui (0.32)
```bash
# Update Cargo.toml
egui = "0.32"
eframe = { version = "0.32", features = ["wgpu"] }
wgpu = "0.22"  # Check latest compatible version

cargo update
cargo build --example minimal_ui_test
```

#### Option B: Try glow Backend
```bash
# Update Cargo.toml
eframe = { version = "0.28", features = ["glow"] }

# Update src/main.rs
# No WGPU-specific options needed for glow

cargo build --example minimal_ui_test
```

#### Option C: Add Debug Logging
```rust
// In examples/minimal_ui_test.rs update method:
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            if i.pointer.any_click() {
                println!("CLICK DETECTED at {:?}", i.pointer.interact_pos());
            }
            if !i.events.is_empty() {
                println!("EVENTS: {:?}", i.events);
            }
        });
        
        // ... rest of update method
    }
}
```

Run with:
```bash
RUST_LOG=debug cargo run --example minimal_ui_test 2>&1 | tee debug.log
```

#### Option D: Platform-Specific Issues

**Linux Wayland:**
```bash
# Try forcing X11
GDK_BACKEND=x11 cargo run --example minimal_ui_test
```

**Linux Window Manager:**
```bash
# Check window manager compatibility
echo $XDG_CURRENT_DESKTOP
# Some WMs may have focus issues
```

### If Build Fails

#### Check Compilation Errors
```bash
cargo build 2>&1 | grep "error\[E"
```

#### Clean and Rebuild
```bash
cargo clean
cargo build
```

#### Update All Dependencies
```bash
cargo update
cargo build
```

---

## Success Criteria

### Minimal Test Success
✅ **Definition:**
- Window opens and receives focus
- Button clicks work
- Text input works
- Counter increments correctly
- Console shows expected output

### Full Application Success  
✅ **Definition:**
- All UI tests pass (from section 4)
- Can connect to GRBL device
- Can load and visualize G-code
- Program execution controls work
- Settings can be saved
- No crashes or hangs

### Production Ready
✅ **Definition:**
- All tests pass consistently
- GRBL communication verified
- Documentation complete
- Performance acceptable
- Cross-platform tested
- User feedback incorporated

---

## Timeline Estimates

### Optimistic (Everything Works)
- **Today:** UI tests pass, full app works
- **Tomorrow:** GRBL testing complete
- **This Week:** All features verified
- **Next Week:** Polish and documentation
- **Week 3:** Release candidate

### Realistic (Some Issues)
- **This Week:** UI working, initial testing
- **Week 2:** GRBL integration, bug fixes
- **Week 3-4:** Complete testing, polish
- **Week 5:** Documentation, packaging
- **Week 6:** Release candidate

### Pessimistic (Major Issues)
- **Week 1-2:** Resolve UI, refactor if needed
- **Week 3-4:** Complete testing
- **Week 5-6:** Integration and fixes
- **Week 7-8:** Polish and documentation
- **Week 9-10:** Release candidate

---

## Resources

### Documentation
- **UI_FIX_ATTEMPT.md** - Technical details of fix
- **INCOMPLETE_TASKS_PROGRESS.md** - Task tracking
- **TODO.md** - Issue tracking
- **PROJECT_STATUS.md** - Current state
- **README.md** - User guide

### Testing Resources
- `examples/minimal_ui_test.rs` - UI test harness
- `examples/*.gcode` - Sample files
- GRBL simulator (if available)
- Test CNC machine (if available)

### Community Resources
- [egui Discord](https://discord.gg/vbuv9Xan65)
- [r/rust](https://reddit.com/r/rust)
- [GRBL Wiki](https://github.com/gnea/grbl/wiki)
- [Original Candle Issues](https://github.com/Denvi/Candle/issues)

---

## Decision Points

### Decision 1: UI Fix Success
**When:** After minimal test completes
**Options:**
- ✅ Success → Proceed to full app testing
- ❌ Failure → Try fallback options (update to 0.32, try glow)

### Decision 2: Full App Functionality  
**When:** After UI testing complete
**Options:**
- ✅ All working → Proceed to GRBL testing
- ⚠️ Some issues → Fix critical, defer nice-to-have
- ❌ Major issues → Investigate and refactor

### Decision 3: GRBL Communication
**When:** After connection testing
**Options:**
- ✅ Working → Full feature testing
- ❌ Not working → Debug protocol implementation

### Decision 4: Release Timeline
**When:** After all testing complete
**Options:**
- Fast track (1-2 weeks) if everything works
- Normal track (4-6 weeks) with some issues
- Extended track (8-10 weeks) for major work

---

## Communication

### Status Updates
Create status update after each major milestone:
- [ ] Minimal test results
- [ ] Full UI testing results
- [ ] GRBL connection results
- [ ] Feature testing results
- [ ] Performance testing results

### Issue Tracking
Document issues in:
- GitHub Issues (if public)
- TODO.md for internal tracking
- KNOWN_ISSUES.md for user-facing issues

### Documentation Updates
Keep these updated:
- README.md - Overall status
- PROJECT_STATUS.md - Technical status
- PROGRESS.md - Development log
- TODO.md - Task tracking

---

## Conclusion

**Current State:** 
- ✅ Fixes implemented
- 🔄 Build in progress (59%)
- ⏳ Testing pending

**Next Immediate Action:**
1. Wait for build to complete
2. Run minimal_ui_test
3. Verify UI interaction works

**Expected Outcome:** 
High probability of success based on fixes implemented. If minimal test works, full application should work with minimal additional changes.

**Risk Level:** MEDIUM (down from CRITICAL)

**Confidence:** HIGH

---

**Last Updated:** January 2025  
**Status:** Ready for testing phase  
**Build Progress:** 344/577 (59%)
