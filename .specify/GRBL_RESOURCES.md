# GRBL Resources and Documentation

## Overview

This document provides comprehensive references for GRBL firmware documentation and resources that are essential for implementing rCandle's GRBL controller interface.

## Primary GRBL Documentation

### GRBL 1.1f Customized for Laser

**Repository**: https://github.com/craftweeks/grbl-1.1f.customized-for-laser

**Documentation Location**: https://github.com/craftweeks/grbl-1.1f.customized-for-laser/tree/master/doc/markdown

This is the primary reference for GRBL protocol implementation in rCandle.

### Available Documentation Files

#### 1. commands.md
**Purpose**: Complete GRBL command reference

**Contents**:
- G-code commands (motion, coordinate system, etc.)
- M-code commands (spindle, coolant, etc.)
- $ system commands (settings, queries, help)
- Real-time commands (status query, feed hold, cycle start, reset)
- Supported and unsupported commands

**Key for rCandle**:
- Parser module - understanding G-code syntax
- Connection module - command formatting and validation
- UI module - implementing command buttons and shortcuts

#### 2. interface.md
**Purpose**: Serial interface protocol specifications

**Contents**:
- Communication protocol (115200 baud, 8N1)
- Command/response format
- Status report format (`<...>`)
- Real-time command protocol (single byte commands)
- Streaming protocol
- Error handling and messages

**Key for rCandle**:
- Connection module - serial communication implementation
- Parser module - response parsing
- State module - status report parsing and state updates

#### 3. settings.md
**Purpose**: GRBL configuration settings ($$ commands)

**Contents**:
- Complete list of $ settings (0-132)
- Setting descriptions and valid ranges
- Setting categories (step calibration, max rates, acceleration, etc.)
- How to query and modify settings
- Default values and recommendations

**Key for rCandle**:
- Settings UI - implementing settings dialog
- State module - storing and managing settings
- Connection module - querying and updating settings

#### 4. jogging.md
**Purpose**: Real-time jogging protocol

**Contents**:
- Jogging command format (`$J=...`)
- Jogging modes (continuous, incremental)
- Jogging cancellation
- Safety features and limitations
- Examples of jogging commands

**Key for rCandle**:
- UI module - jog control panel implementation
- Connection module - real-time command handling
- State module - tracking jogging state

#### 5. laser_mode.md
**Purpose**: Laser-specific features and M-codes

**Contents**:
- Laser mode setting ($32)
- M3/M4/M5 commands (spindle/laser control)
- Dynamic laser power control
- S-value (spindle speed) interpretation for lasers
- Safety features for laser operation

**Key for rCandle**:
- Parser module - laser-specific G-code handling
- UI module - laser mode controls and display
- State module - laser power tracking

#### 6. change_summary.md
**Purpose**: Version changes and update history

**Contents**:
- Changes from v0.9 to v1.1
- New features and improvements
- Breaking changes
- Migration notes

**Key for rCandle**:
- Understanding version differences
- Supporting multiple GRBL versions
- Feature detection and compatibility

## Additional GRBL Resources

### GRBL v1.1 Official Wiki
**URL**: https://github.com/gnea/grbl/wiki

**Key Pages**:
- Grbl v1.1 Configuration
- Grbl v1.1 Commands
- Grbl v1.1 Interface
- Connecting Grbl
- Flashing Grbl to an Arduino

### GRBL v0.9 Wiki (Legacy)
**URL**: https://github.com/grbl/grbl/wiki

**Purpose**: Reference for older GRBL versions
- Some users may still have v0.9 controllers
- Understanding legacy behavior

## Implementation Guide for rCandle

### Phase 3: Connection Module (Weeks 5-6)

**Day 3-4: GRBL Protocol Handling**
- Study `interface.md` for protocol specifications
- Study `commands.md` for command reference
- Implement response parsing based on interface spec
- Implement status report parsing
- Handle real-time commands

**Reference Files**:
- `interface.md` - Primary protocol reference
- `commands.md` - Command validation

### Phase 6: UI Framework (Weeks 11-13)

**Settings Dialog Implementation**
- Reference `settings.md` for all settings
- Implement setting validation using documented ranges
- Group settings by category (as in documentation)

**Jog Control Panel**
- Reference `jogging.md` for command format
- Implement incremental and continuous jogging
- Handle jogging cancellation

**Laser Controls (if applicable)**
- Reference `laser_mode.md`
- Implement laser-specific UI elements
- Display laser power (S-value)

### Parser Module (Weeks 3-4)

**G-Code Command Parsing**
- Reference `commands.md` for supported commands
- Validate commands against GRBL capabilities
- Handle modal groups correctly

## Testing Strategy

### Protocol Compliance Testing

1. **Command Format Testing**
   - Verify all commands match `commands.md` specifications
   - Test command validation

2. **Response Parsing Testing**
   - Parse all response formats from `interface.md`
   - Handle error conditions correctly
   - Parse status reports accurately

3. **Settings Management Testing**
   - Query all settings from `settings.md`
   - Validate setting ranges
   - Test setting persistence

4. **Jogging Protocol Testing**
   - Test jogging commands from `jogging.md`
   - Verify cancellation behavior
   - Test safety limits

### Integration Testing with Real Hardware

- Test with Arduino running GRBL 1.1f
- Verify protocol compliance
- Test all documented features
- Validate error handling

## Status Report Format Reference

From `interface.md`:

```
<Idle|MPos:0.000,0.000,0.000|FS:0,0|WCO:0.000,0.000,0.000>
```

**Fields**:
- Machine state (Idle, Run, Hold, Jog, Alarm, Door, Check, Home, Sleep)
- MPos: Machine position (X, Y, Z)
- WPos: Work position (alternative to MPos)
- FS: Feed rate and spindle speed
- WCO: Work coordinate offset
- Ov: Override values (feed, rapid, spindle)
- A: Accessory state (spindle, flood, mist)

## Command Examples

From `commands.md`:

### Motion Commands
```gcode
G0 X10 Y20      ; Rapid positioning
G1 X10 Y20 F500 ; Linear interpolation
G2 X10 Y20 I5 J5; Clockwise arc
G3 X10 Y20 I5 J5; Counter-clockwise arc
```

### System Commands
```
$$              ; View all settings
$10=255         ; Set setting #10 to 255
$H              ; Homing cycle
$X              ; Kill alarm lock
$C              ; Check gcode mode
```

### Real-Time Commands
```
?               ; Status report query
!               ; Feed hold
~               ; Cycle start/resume
0x18 (Ctrl-X)   ; Soft reset
```

### Jogging Commands
From `jogging.md`:
```
$J=G91 X10 F100 ; Jog 10mm in X at 100mm/min
$J=G90 X0 Y0    ; Jog to absolute position
```

## Error Codes

From `interface.md`:

Common error codes that rCandle must handle:
- `error:1` - Expected command letter
- `error:2` - Bad number format
- `error:3` - Invalid $ statement
- `error:9` - G-code locked out during alarm
- `error:20` - Unsupported command
- And many more...

rCandle must parse these and display user-friendly messages.

## Implementation Checklist

### Connection Module
- [ ] Implement serial communication (115200 baud, 8N1)
- [ ] Parse status reports format
- [ ] Handle error codes
- [ ] Implement real-time commands
- [ ] Handle command/response flow

### Parser Module
- [ ] Validate G-codes against supported commands
- [ ] Parse modal groups correctly
- [ ] Handle laser mode specifics
- [ ] Validate parameter ranges

### State Module
- [ ] Track machine state from status reports
- [ ] Store and manage settings
- [ ] Track work coordinate offsets
- [ ] Monitor override values

### UI Module
- [ ] Display machine state
- [ ] Implement settings dialog with all settings
- [ ] Create jog control panel
- [ ] Add laser mode controls (if applicable)
- [ ] Show real-time status updates

## Conclusion

The GRBL 1.1f documentation at https://github.com/craftweeks/grbl-1.1f.customized-for-laser/tree/master/doc/markdown provides comprehensive specifications for implementing a GRBL controller. All rCandle modules should reference these documents during development to ensure protocol compliance and feature completeness.

---

**Last Updated**: 2024  
**GRBL Version**: 1.1f (with laser customizations)  
**Status**: Reference documentation for rCandle implementation
