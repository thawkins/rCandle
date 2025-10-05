# UI Interaction Fix Attempt

## Date: January 2025

## Problem Summary
The rCandle application window opens and renders all UI elements correctly, but mouse and keyboard interactions are not functioning. This affects buttons, text fields, menu selections, and all user interactions.

## Root Cause Analysis

### Suspected Issues
1. **Outdated egui/eframe versions** (0.27.x)
   - The project was using egui 0.27.2 and eframe 0.27.2
   - Latest versions are 0.32.3
   - Version 0.27 to 0.28 had significant API changes

2. **API Changes in eframe 0.28+**
   - The `run_native` function signature changed
   - Creator closure now returns `Result<Box<dyn App>>` instead of `Box<dyn App>`
   - Viewport configuration may have additional requirements

3. **Viewport Configuration**
   - Missing `.with_focused(true)` flag
   - Potentially missing event handling configuration

## Changes Made

### 1. Updated Dependencies (Cargo.toml)

**Before:**
```toml
egui = "0.27"
eframe = { version = "0.27", features = ["wgpu"] }
wgpu = "0.19"
```

**After:**
```toml
egui = "0.28"
eframe = { version = "0.28", features = ["wgpu"] }
wgpu = "0.20"
```

**Rationale:** Update to 0.28.x (not jumping all the way to 0.32 to minimize breaking changes)

### 2. Updated main.rs API Call

**Before:**
```rust
let native_options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
        .with_title("rCandle - GRBL Controller")
        .with_inner_size([1280.0, 800.0])
        .with_min_inner_size([800.0, 600.0])
        .with_active(true)
        .with_visible(true),
    ..Default::default()
};

eframe::run_native(
    "rCandle",
    native_options,
    Box::new(|cc| Box::new(RCandleApp::new(cc))),
)
```

**After:**
```rust
let native_options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
        .with_title("rCandle - GRBL Controller")
        .with_inner_size([1280.0, 800.0])
        .with_min_inner_size([800.0, 600.0])
        .with_active(true)
        .with_visible(true)
        .with_focused(true)          // NEW: Explicitly request focus
        .with_decorations(true)       // NEW: Ensure window decorations
        .with_resizable(true),        // NEW: Explicitly enable resize
    ..Default::default()
};

eframe::run_native(
    "rCandle",
    native_options,
    Box::new(|cc| Ok(Box::new(RCandleApp::new(cc)))),  // NEW: Wrap in Ok()
)
```

**Key Changes:**
1. Added `.with_focused(true)` - ensures window gets input focus on startup
2. Added `.with_decorations(true)` and `.with_resizable(true)` - explicitly set expected defaults
3. Changed creator closure to return `Ok(Box::new(...))` instead of just `Box::new(...)`

### 3. Created Minimal Test Example

Created `examples/minimal_ui_test.rs` - a minimal egui application to test if basic UI interaction works with the updated dependencies.

**Purpose:**
- Verify egui library itself works
- Test button clicks
- Test text input
- Isolate whether issue is in egui or in rCandle's implementation

## Testing Strategy

### Step 1: Build Minimal Test
```bash
cargo build --example minimal_ui_test
cargo run --example minimal_ui_test
```

**Expected Result:** A simple window with a button and text field that responds to clicks and input.

**If this works:** The issue was likely the API mismatch or viewport configuration.
**If this fails:** There may be a deeper platform or dependency issue.

### Step 2: Build Full Application
```bash
cargo build --release
cargo run --release
```

**Expected Result:** Full rCandle UI with working interactions.

### Step 3: Test Specific Interactions
If the application runs, test in this order:
1. ✓ Menu bar items (File → Open, etc.)
2. ✓ Buttons in control panel (Connect, Jog controls)
3. ✓ Text input in console
4. ✓ G-Code editor typing and selection
5. ✓ Settings dialog
6. ✓ 3D viewport camera controls (mouse drag)

## Additional Considerations

### If Issue Persists

1. **Check for Platform-Specific Issues**
   ```bash
   RUST_LOG=debug cargo run 2>&1 | grep -i "event\|input\|focus"
   ```

2. **Verify Event Loop**
   - Check if `update()` method is being called (already has debug logging)
   - Verify frame counter increments

3. **Try Different Backend**
   - egui supports both `glow` (OpenGL) and `wgpu` backends
   - Current config uses `wgpu`
   - Could try switching to `glow` if needed

4. **Update to Latest egui (0.32.3)**
   - If 0.28 doesn't fix it, try latest version
   - Note: May require more code changes

5. **Check Window Manager**
   - On Linux: Some window managers may interfere
   - Try running on different desktop environment

6. **Input Debugging**
   Add to `update()` method:
   ```rust
   ctx.input(|i| {
       if i.pointer.any_click() {
           tracing::info!("Mouse click detected at {:?}", i.pointer.interact_pos());
       }
       if !i.events.is_empty() {
           tracing::info!("Events: {}", i.events.len());
       }
   });
   ```

## References

- [egui 0.28 CHANGELOG](https://github.com/emilk/egui/blob/master/CHANGELOG.md)
- [eframe run_native documentation](https://docs.rs/eframe/0.28.1/eframe/fn.run_native.html)
- [egui ViewportBuilder](https://docs.rs/egui/0.28.1/egui/viewport/struct.ViewportBuilder.html)

## Next Steps

1. Complete compilation of minimal test example
2. Run minimal test to verify basic egui functionality
3. If successful, compile and test full application
4. Document results and any additional fixes needed
5. Update TODO.md with progress

## Success Criteria

- [ ] Minimal test application shows window with working buttons
- [ ] Buttons in minimal test respond to clicks (counter increments)
- [ ] Text field in minimal test accepts keyboard input
- [ ] Full rCandle application compiles without errors
- [ ] Menu bar items in rCandle respond to clicks
- [ ] Connect button in rCandle works
- [ ] G-Code editor accepts text input
- [ ] All control panel buttons respond

## Notes

- This is an incremental update (0.27 → 0.28) to minimize breaking changes
- WGPU also updated from 0.19 → 0.20 as required by egui 0.28
- If this doesn't resolve the issue, may need to update to egui 0.32.3
- The update method implementation in `src/ui/app.rs` looks correct and doesn't need changes
