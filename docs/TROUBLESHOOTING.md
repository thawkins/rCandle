# rCandle Troubleshooting Guide

**Version**: 0.1.0-alpha  
**Last Updated**: January 2025

## Table of Contents

1. [Quick Diagnostics](#quick-diagnostics)
2. [Installation Issues](#installation-issues)
3. [Connection Problems](#connection-problems)
4. [G-Code Issues](#g-code-issues)
5. [Visualization Problems](#visualization-problems)
6. [Machine Control Issues](#machine-control-issues)
7. [Performance Problems](#performance-problems)
8. [Platform-Specific Issues](#platform-specific-issues)
9. [Error Messages](#error-messages)
10. [Getting More Help](#getting-more-help)

## Quick Diagnostics

Before diving into specific issues, try these quick fixes:

### First Steps
1. **Restart the application**
2. **Restart your computer**
3. **Check USB cable** is firmly connected
4. **Power cycle the GRBL controller**
5. **Update to latest version** of rCandle

### Check Logs
rCandle creates log files in:
- **Windows**: `%APPDATA%\rCandle\logs\`
- **Linux**: `~/.local/share/rCandle/logs/`
- **macOS**: `~/Library/Application Support/rCandle/logs/`

Run with debug logging: `RUST_LOG=debug ./rcandle`

## Installation Issues

### Application Won't Start

**Symptoms**: Double-clicking does nothing, or error appears immediately

**Solutions**:
1. Check system requirements are met
2. Install WGPU-compatible graphics drivers
3. Try running from terminal to see error messages:
   ```bash
   ./rcandle  # Linux/macOS
   rcandle.exe  # Windows
   ```
4. Check antivirus isn't blocking the application
5. Ensure you have correct permissions

### Missing Graphics Support

**Error**: `Failed to create graphics device` or similar

**Solutions**:
1. Update graphics drivers to latest version
2. For Intel integrated graphics, ensure drivers support Vulkan/DX12
3. Try software rendering (slower):
   ```bash
   WGPU_BACKEND=gl ./rcandle
   ```
4. Check system requirements - rCandle requires modern GPU

### Permission Denied (Linux)

**Error**: `Permission denied` when starting

**Solution**:
```bash
chmod +x rcandle
./rcandle
```

## Connection Problems

### Serial Port Not Detected

**Symptoms**: No ports appear in dropdown, or your device isn't listed

**Solutions**:

**Windows**:
1. Install FTDI/CH340 drivers if not automatic
2. Check Device Manager for COM ports
3. Disable "USB Selective Suspend" in Power Options

**Linux**:
1. Add user to dialout group:
   ```bash
   sudo usermod -a -G dialout $USER
   ```
2. Log out and log back in
3. Check device appears: `ls /dev/ttyUSB* /dev/ttyACM*`
4. Install udev rules if needed

**macOS**:
1. Check in System Information → USB
2. Install FTDI driver from official source
3. Allow driver in Security & Privacy settings

### Connection Fails

**Symptoms**: "Connect" button does nothing or times out

**Solutions**:
1. **Verify baud rate**: Most GRBL uses 115200
2. **Try different USB cable**: Some cables are power-only
3. **Close other serial programs**: Arduino IDE, PuTTY, etc.
4. **Power cycle controller**: Unplug and reconnect
5. **Check GRBL firmware**: Should respond to `$$` command
6. **Try manual connection**:
   ```bash
   screen /dev/ttyUSB0 115200  # Linux/macOS
   ```

### Connection Drops Randomly

**Symptoms**: Connected, then loses connection during operation

**Solutions**:
1. Replace USB cable with better quality
2. Use powered USB hub
3. Reduce baud rate to 57600
4. Check for electrical interference
5. Update USB drivers
6. Disable USB power management

### Can't Send Commands

**Symptoms**: Connected but commands don't execute

**Solutions**:
1. Check machine state - must be "Idle" or "Run"
2. If in "Alarm" state, send `$X` to unlock
3. If in "Hold" state, send `~` to resume
4. Verify GRBL is responding: send `?` for status
5. Check console for error messages

## G-Code Issues

### File Won't Load

**Symptoms**: Error when opening G-Code file

**Solutions**:
1. Check file encoding is UTF-8 or ASCII
2. Remove special characters from filename
3. Verify file isn't corrupted: open in text editor
4. Check file isn't too large (>10MB may be slow)
5. Ensure file extension is recognized (.nc, .gcode, .tap)

### Syntax Errors

**Symptoms**: Red highlights in editor, error in console

**Solutions**:
1. Check G-Code follows GRBL format
2. Common errors:
   - Missing space after command: `G0X10` → `G0 X10`
   - Invalid command for GRBL version
   - Out of range values
3. Validate in external tool (ncviewer.com, camotics)
4. Check GRBL compatibility: some features need v1.1+

### Toolpath Not Displaying

**Symptoms**: File loads but 3D view is empty

**Solutions**:
1. Press `F` to zoom to fit
2. Check file actually contains movement commands
3. Verify coordinates aren't extremely large/small
4. Check "Show Toolpath" is enabled
5. Try a known-good test file

### Incorrect Visualization

**Symptoms**: Toolpath looks wrong compared to CAM software

**Solutions**:
1. Check units match (mm vs inches)
2. Verify work coordinate system (G54-G59)
3. Check arc interpolation settings
4. Reload file after making changes
5. Compare with other G-Code viewers

## Visualization Problems

### Slow/Laggy 3D View

**Symptoms**: Choppy rendering, slow camera movement

**Solutions**:
1. Disable anti-aliasing in settings
2. Reduce console history limit
3. Close other applications
4. Update graphics drivers
5. Use simpler toolpaths for testing
6. For very large files (>10K lines), consider simplifying

### Black/Blank Screen

**Symptoms**: 3D view shows nothing, not even grid

**Solutions**:
1. Check graphics drivers are up to date
2. Try software rendering: `WGPU_BACKEND=gl`
3. Verify GPU supports required features
4. Check application has GPU access
5. Restart application

### Colors Look Wrong

**Symptoms**: Lines hard to see, poor contrast

**Solutions**:
1. Switch between light/dark theme
2. Adjust colors in Visualization settings
3. Check monitor calibration
4. Increase contrast in settings
5. Customize toolpath colors

## Machine Control Issues

### Homing Fails

**Symptoms**: Machine doesn't move or moves incorrectly during homing

**Solutions**:
1. Verify limit switches are wired correctly
2. Check GRBL homing is enabled: `$22=1`
3. Verify homing direction: `$23=0` (default)
4. Test limit switches manually
5. Check GRBL alarm messages in console
6. Ensure machine has clearance to move

### Jogging Doesn't Work

**Symptoms**: Jog buttons don't move machine

**Solutions**:
1. Check machine is in "Idle" state
2. If in "Alarm", unlock with `$X`
3. Verify connection is active
4. Check jog distance isn't zero
5. Test with console command: `$J=G91 G21 X10 F100`
6. Enable soft limits: `$20=1` (if configured)

### Wrong Movement Distance

**Symptoms**: Machine moves more/less than expected

**Solutions**:
1. Check units match (mm vs inches)
2. Verify steps/mm calibration in GRBL: `$$` → `$100-$102`
3. Check jog distance setting in UI
4. Ensure machine is homed for accurate positioning
5. Verify no coordinate system offsets active

### Machine Position Wrong

**Symptoms**: Display shows incorrect position

**Solutions**:
1. Home the machine to reset machine coordinates
2. Check GRBL status with `?` command
3. Zero work coordinates if needed
4. Verify not confusing WPos vs MPos
5. Reset work coordinate system: `G10 L20 P1 X0 Y0 Z0`

## Performance Problems

### High CPU Usage

**Symptoms**: Application uses excessive CPU

**Solutions**:
1. Reduce console history limit
2. Disable real-time visualizations
3. Close unused panels
4. Update to latest version
5. Check for background processes

### High Memory Usage

**Symptoms**: Application uses lots of RAM

**Solutions**:
1. Close large G-Code files when not needed
2. Reduce console history
3. Restart application periodically
4. Split very large files into smaller parts
5. Clear undo history

### Application Freezes

**Symptoms**: UI stops responding

**Solutions**:
1. Wait - may be processing large file
2. Check logs for errors
3. Kill and restart application
4. Avoid loading extremely large files (>50K lines)
5. Report bug with details

## Platform-Specific Issues

### Windows

**Serial Port Issues**:
- Install driver from manufacturer
- Check COM port in Device Manager
- Disable Driver Signature Enforcement if needed

**Graphics Issues**:
- Update DirectX 12
- Update GPU drivers from manufacturer
- Try running as administrator

### Linux

**Permission Issues**:
```bash
# Add to dialout group
sudo usermod -a -G dialout $USER
# Log out and back in

# Or use sudo (temporary)
sudo ./rcandle
```

**Graphics Issues**:
- Install Vulkan libraries: `sudo apt install vulkan-tools`
- Update Mesa drivers
- Check Wayland vs X11 compatibility

**AppImage Issues**:
- Make executable: `chmod +x rCandle.AppImage`
- Install FUSE: `sudo apt install fuse libfuse2`

### macOS

**Security Issues**:
- Right-click → Open (first time only)
- Allow in System Preferences → Security & Privacy
- May need to allow kernel extensions

**Serial Port Issues**:
- Install official FTDI driver
- Check /dev/tty.* devices
- Disable "About This Mac" if interfering

**Graphics Issues**:
- Ensure macOS 11+ (for Metal support)
- Update macOS to latest version

## Error Messages

### "Failed to parse G-Code"
**Cause**: Invalid G-Code syntax  
**Fix**: Check file format, validate syntax, check for special characters

### "Connection timeout"
**Cause**: No response from GRBL controller  
**Fix**: Check connection, baud rate, power to controller

### "GRBL Alarm"
**Cause**: GRBL detected problem (limit hit, homing failed, etc.)  
**Fix**: Check alarm code in console, resolve issue, send `$X` to clear

### "Buffer overflow"
**Cause**: Commands sent faster than GRBL can process  
**Fix**: Slow down command sending, check GRBL buffer size

### "Out of bounds"
**Cause**: Move would exceed soft limits  
**Fix**: Adjust toolpath, disable soft limits (unsafe), or increase travel

### "Failed to create graphics device"
**Cause**: GPU or driver doesn't support required features  
**Fix**: Update drivers, try software rendering, check system requirements

## Getting More Help

### Before Asking for Help

Gather this information:
1. **rCandle version**: Help → About
2. **Operating system** and version
3. **GRBL version**: Send `$I` command
4. **Error messages**: Copy from console
5. **Log files**: From application data directory
6. **Steps to reproduce**: What you were doing when problem occurred

### Where to Get Help

1. **Documentation**:
   - User Guide
   - FAQ (if available)
   - This troubleshooting guide

2. **Community**:
   - GitHub Discussions
   - GitHub Issues (for bugs)
   - GRBL forums

3. **Debug Mode**:
   ```bash
   RUST_LOG=debug ./rcandle 2>&1 | tee rcandle-debug.log
   ```

### Reporting Bugs

When reporting bugs on GitHub:
1. Search existing issues first
2. Use issue template
3. Include all diagnostic information
4. Attach logs if relevant
5. Describe expected vs actual behavior
6. Include screenshots if helpful

### Feature Requests

For feature requests:
1. Check roadmap first
2. Search existing requests
3. Explain use case
4. Provide examples if applicable

## Common Misconceptions

### "rCandle Crashed My Machine"
- rCandle only sends commands
- GRBL controls the machine
- Check G-Code, machine setup, and GRBL config

### "Position is Wrong After Homing"
- Homing sets machine coordinates (MPos)
- Work coordinates (WPos) are separate
- Zero work coordinates after homing

### "Jogging is Too Slow/Fast"
- Jog speed is separate from program feed rate
- Adjust in settings or control panel
- GRBL has maximum speeds in config

### "File Won't Run"
- Check machine state (must be Idle)
- Clear any alarms first
- Verify file loaded successfully
- Check connection is active

## Prevention Tips

### Regular Maintenance
- Update rCandle to latest version
- Update GRBL firmware periodically
- Keep machine mechanically sound
- Test with simple files regularly

### Best Practices
- Always home before running programs
- Test new programs at reduced speed first
- Save work coordinate zeros
- Keep backups of known-good G-Code
- Document machine configuration

### Safety
- Never leave running machine unattended
- Keep emergency stop accessible
- Maintain clear workspace
- Use appropriate feeds and speeds
- Test air cuts first

---

## Still Having Problems?

If this guide didn't help:

1. Check the [User Guide](USER_GUIDE.md) for usage instructions
2. Review [Keyboard Shortcuts](KEYBOARD_SHORTCUTS.md) for controls
3. Visit GitHub for community support
4. File a detailed bug report

**Remember**: Most issues are configuration-related, not software bugs. Double-check settings before assuming malfunction.

---

*This troubleshooting guide is continuously updated. Suggestions for improvements welcome!*
