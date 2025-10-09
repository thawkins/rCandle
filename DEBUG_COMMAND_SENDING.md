# Debug: Command Sending Investigation

**Issue**: Commands not reaching GRBL device  
**Date**: January 5, 2025  
**Status**: 🔍 Debugging

## Problem Description

After successfully connecting to `/dev/ttyACM0` (laser engraver), the application shows "Connected" status but commands (Home, Jog, etc.) are not being sent to the device.

## Debug Logging Added

I've added extensive logging to track the command flow from UI to hardware:

### 1. Connection Manager - `send_command()` (manager.rs:202)
```rust
tracing::info!("send_command called with: {:?}", command);
tracing::info!("send_command: connection OK, enqueueing command");
tracing::info!("send_command: enqueue result: {:?}", result);
```

### 2. Command Queue - `enqueue()` (queue.rs:122)
```rust
tracing::info!("Queue: enqueue called with command: {:?}", command);
tracing::info!("Queue: command {} added, queue length now: {}", id, queue.len());
tracing::info!("Queue: attempting to send next command");
```

### 3. Command Queue - `next_command()` (queue.rs:366)
```rust
tracing::debug!("Queue: next_command called, current state: {:?}", *state);
tracing::info!("Queue: next_command returning: {:?}", c);
```

### 4. Connection Manager - `process_queue()` (manager.rs:437)
```rust
tracing::info!("Sending command to GRBL: {}", command_str);
tracing::info!("Command sent successfully: {}", command_str);
```

## How to Test

### Run with Debug Logging

```bash
cd /home/thawkins/projects/rCandle
RUST_LOG=info cargo run 2>&1 | tee debug_output.log
```

### Test Sequence

1. **Launch Application**
   - Watch for: "rCandle v... starting..."
   
2. **Connect to Device**
   - Select `/dev/ttyACM0` from dropdown
   - Click "Connect" button
   - Watch for:
     - "Attempting to connect to /dev/ttyACM0"
     - "Successfully connected to /dev/ttyACM0"
     - "Connection manager stored successfully"

3. **Send Home Command**
   - Click "🏠" button
   - Watch for:
     - "send_command called with: HomingCycle"
     - "send_command: connection OK, enqueueing command"
     - "Queue: enqueue called with command: HomingCycle"
     - "Queue: command 0 added, queue length now: 1"
     - "Sending command to GRBL: $H"
     - "Command sent successfully: $H"

4. **Send Jog Command**
   - Click X+ jog button
   - Watch for similar logging sequence
   - Command should be: `Jog { x: Some(...), y: None, z: None, feed_rate: ... }`

## Expected Log Flow

### Successful Command Sending

```
INFO send_command called with: HomingCycle
INFO send_command: connection OK, enqueueing command
INFO Queue: enqueue called with command: HomingCycle
INFO Queue: command 0 added, queue length now: 1
INFO Queue: attempting to send next command
DEBUG Queue: next_command called, current state: Idle
INFO Queue: next_command returning: HomingCycle
INFO Sending command to GRBL: $H
INFO Command sent successfully: $H
```

### Possible Failure Points

1. **Connection Not Established**
   ```
   ERROR send_command: not connected
   ```
   → Connection manager not properly stored

2. **Queue State Not Idle**
   ```
   DEBUG Queue: next_command called, current state: WaitingForAck
   DEBUG Queue: not in Idle state, returning None
   ```
   → Waiting for response from previous command

3. **Serial Port Write Failure**
   ```
   ERROR Error sending command: ...
   ```
   → Hardware communication problem

4. **No Commands in Log**
   → Commands not reaching `send_command()` method

## Diagnosis Guide

### Scenario 1: No "send_command called" Log

**Problem**: Commands not reaching the connection manager  
**Cause**: UI not properly invoking send methods  
**Check**:
- Is `connection_manager` field populated?
- Is button click handler being called?

### Scenario 2: "send_command: not connected" Error

**Problem**: Connection manager reports not connected  
**Cause**: `is_connected()` returning false  
**Check**:
- Connection status in manager
- Serial port state
- Background tasks still running

### Scenario 3: Commands Queued but Not Sent

**Logs Show**:
```
INFO Queue: command 0 added, queue length now: 1
DEBUG Queue: next_command called, current state: WaitingForAck
```

**Problem**: Queue stuck in WaitingForAck state  
**Cause**: Not receiving "ok" responses from GRBL  
**Solutions**:
- Check GRBL is responsive (send `?` status query manually)
- Check response parsing
- Queue may need to be reset to Idle state

### Scenario 4: Commands Sent but GRBL Not Responding

**Logs Show**:
```
INFO Sending command to GRBL: $H
INFO Command sent successfully: $H
```

**Problem**: Command sent to serial port but GRBL not executing  
**Possible Causes**:
- GRBL in alarm state (needs $X to unlock)
- Incorrect baud rate (should be 115200)
- Flow control issues
- Device not actually GRBL (wrong firmware)

**Troubleshooting**:
- Use `screen` or `minicom` to test device directly
- Verify GRBL responds to `$` command
- Check for GRBL startup message after connection

## Common GRBL States

### Alarm State
GRBL boots in alarm state and requires:
1. `$X` - Kill alarm lock (unlock)
2. `$H` - Home (if homing is enabled)

### Check State
If GRBL is in check mode, commands are simulated not executed.
- `$C` toggles check mode

### Hold State
If GRBL is in hold/pause:
- Send `~` (0x7E) to resume

## Testing Without Hardware

If you don't have access to real hardware, you can test with a GRBL simulator:

```bash
# Install socat for virtual serial ports
sudo apt-get install socat

# Create virtual serial port pair
socat -d -d pty,raw,echo=0 pty,raw,echo=0

# Note the two PTY devices (e.g., /dev/pts/2 and /dev/pts/3)
# Connect rCandle to one, and use screen to monitor the other:
screen /dev/pts/3 115200
```

## Next Steps Based on Results

1. **If logs show commands reaching hardware**: Issue is with GRBL state or hardware
2. **If logs show commands queued but not sent**: Issue with queue processing or response handling
3. **If logs show "not connected"**: Issue with connection manager storage (already fixed)
4. **If no command logs at all**: Issue with UI event handling

## Files Modified for Debugging

- `src/connection/manager.rs` - Added logging to send_command() and process_queue()
- `src/grbl/queue.rs` - Added logging to enqueue() and next_command()

These changes can be removed or reduced to debug level after issue is resolved.

## Background Tasks

The ConnectionManager starts 3 background tasks on connect:

1. **Response Receiver** - Reads and parses GRBL responses
2. **Queue Processor** - Sends commands from queue (every 10ms)
3. **Status Poller** - Sends `?` query (every 250ms by default)

If these tasks aren't running, commands won't be processed.

**Check**: Look for "start_background_tasks" in initialization logs.

## Manual Serial Port Test

To verify the device is actually GRBL:

```bash
# Connect with screen
screen /dev/ttyACM0 115200

# You should see GRBL startup message:
# Grbl 1.1f ['$' for help]

# Try commands:
$           # Should list settings
?           # Should return status
$H          # Should home (if configured)
```

If these don't work, the device may not be running GRBL or may be in a locked state.

## Results

**TODO**: Run the application with logging and document the results here.

### Connection Phase
- [ ] Connection initiated
- [ ] Serial port opened
- [ ] Background tasks started
- [ ] Connection manager stored
- [ ] Status: ___________

### Command Phase  
- [ ] 🏠 button clicked
- [ ] send_command() called
- [ ] Command enqueued
- [ ] Command sent to serial
- [ ] GRBL responded
- [ ] Status: ___________

### Observed Logs
```
[Paste relevant logs here]
```

### Conclusion
[To be filled after testing]
