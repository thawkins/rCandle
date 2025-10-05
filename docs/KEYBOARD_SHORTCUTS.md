# rCandle Keyboard Shortcuts

**Version**: 0.1.0-alpha  
**Last Updated**: January 2025

## Quick Reference

### File Operations
| Shortcut | Action |
|----------|--------|
| `Ctrl+O` | Open G-Code file |
| `Ctrl+S` | Save current file |
| `Ctrl+Shift+S` | Save as... |
| `Ctrl+W` | Close file |
| `Ctrl+Q` | Quit application |

### Editing
| Shortcut | Action |
|----------|--------|
| `Ctrl+Z` | Undo |
| `Ctrl+Shift+Z` or `Ctrl+Y` | Redo |
| `Ctrl+X` | Cut |
| `Ctrl+C` | Copy |
| `Ctrl+V` | Paste |
| `Ctrl+A` | Select all |
| `Ctrl+F` | Find |
| `Ctrl+H` | Find and replace |
| `Ctrl+G` | Go to line |

### Machine Control
| Shortcut | Action |
|----------|--------|
| `H` | Home all axes |
| `Shift+H` | Home current axis |
| `Z` | Zero all axes |
| `Shift+Z` | Zero current axis |
| `Esc` | Emergency stop / Reset |

### Jogging (when enabled in settings)
| Shortcut | Action |
|----------|--------|
| `←` | Jog X- (left) |
| `→` | Jog X+ (right) |
| `↑` | Jog Y+ (away) |
| `↓` | Jog Y- (toward) |
| `PgUp` | Jog Z+ (up) |
| `PgDn` | Jog Z- (down) |
| `Shift` + arrows | Jog at 10x distance |
| `Ctrl` + arrows | Jog at 0.1x distance |

### Program Execution
| Shortcut | Action |
|----------|--------|
| `Space` | Run / Resume program |
| `P` | Pause execution |
| `S` | Stop execution |
| `R` | Reset to beginning |
| `N` | Next step (step mode) |
| `T` | Toggle step mode |

### View and Navigation
| Shortcut | Action |
|----------|--------|
| `F` | Zoom to fit |
| `1` | Isometric view |
| `2` | Top view |
| `3` | Front view |
| `4` | Right view |
| `5` | Left view |
| `6` | Back view |
| `7` | Bottom view |
| `Ctrl++` | Zoom in |
| `Ctrl+-` | Zoom out |
| `Ctrl+0` | Reset zoom |

### Windows and Panels
| Shortcut | Action |
|----------|--------|
| `F1` | Show help |
| `F11` | Toggle fullscreen |
| `Ctrl+,` | Open settings |
| `Ctrl+L` | Focus console |
| `Ctrl+E` | Focus editor |
| `Ctrl+Tab` | Switch between editor/console |

### Console Commands
| Shortcut | Action (in console) |
|----------|--------|
| `↑` | Previous command in history |
| `↓` | Next command in history |
| `Tab` | Auto-complete command |
| `Enter` | Send command |
| `Ctrl+L` | Clear console |
| `Ctrl+C` | Cancel command entry |

### Override Controls
| Shortcut | Action |
|----------|--------|
| `[` | Decrease feed override |
| `]` | Increase feed override |
| `{` (Shift+[) | Decrease spindle override |
| `}` (Shift+]) | Increase spindle override |
| `\` | Reset feed override to 100% |
| `|` (Shift+\) | Reset spindle override to 100% |

### Tools and Features
| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+S` | Open script editor |
| `Ctrl+Shift+C` | Open user commands |
| `Ctrl+R` | Refresh serial ports |
| `Ctrl+D` | Toggle dark/light theme |

## Platform-Specific Shortcuts

### macOS
Replace `Ctrl` with `Cmd` (⌘) for most shortcuts:
- `Cmd+O` - Open file
- `Cmd+S` - Save file
- `Cmd+C` / `Cmd+V` - Copy / Paste
- etc.

### Linux
Standard `Ctrl` shortcuts apply. Some window managers may intercept certain shortcuts.

### Windows
Standard `Ctrl` shortcuts apply. `Alt+F4` closes the application.

## Customization

**Note**: Keyboard shortcut customization will be available in a future version. Current shortcuts are fixed.

To request changes to default shortcuts, please file an issue on GitHub.

## Context-Specific Shortcuts

### In G-Code Editor
- Standard text editing shortcuts apply
- Line operations (duplicate, delete) coming in future version
- Block comment/uncomment coming in future version

### In 3D View
- Mouse controls take precedence
- Keyboard shortcuts for camera presets available
- Jogging shortcuts work when view is focused

### In Console
- Command history navigation with arrow keys
- Standard terminal-like behavior
- Ctrl+C does not copy, use Ctrl+Shift+C

## Tips

### Quick Workflow
1. `Ctrl+O` - Open file
2. `F` - Zoom to fit
3. `H` - Home machine
4. `Space` - Start program

### Emergency Situations
- `Esc` - Quick stop
- `P` - Pause for inspection
- Click physical e-stop if needed

### Efficient Editing
- `Ctrl+G` - Jump to line number
- `Ctrl+F` - Find specific G-code
- `Ctrl+H` - Replace values (e.g., feed rates)

### Navigation
- Number keys (1-7) for quick view changes
- Mouse wheel for precise zoom
- F to reset and frame toolpath

## Accessibility

### For Users with Limited Mobility
- Most functions accessible via mouse
- Toolbar buttons for common operations
- Large button mode in settings (future)

### For Screen Reader Users
- Keyboard navigation fully supported
- Status announcements in console
- Descriptive button labels

## Learning Tips

### Memorize These First
1. `Space` - Run/Pause (most important!)
2. `Esc` - Emergency stop
3. `H` - Home
4. `F` - Fit view
5. Arrow keys - Jogging

### Practice Safe
- Test on air (without work piece) first
- Use feed override to slow down
- Keep hand near Space for pause
- Know where Esc is

### Build Muscle Memory
- Use shortcuts consistently
- Print this guide for reference
- Practice common workflows
- Create your own macro sequences

## Conflicts and Troubleshooting

### Shortcut Not Working
- Check if console is focused (shortcuts won't work)
- Verify shortcut is enabled in settings
- Check for OS-level conflicts
- Restart application if persistent

### OS Conflicts
- Some Linux DEs use `Ctrl+Alt+arrows`
- macOS uses `Cmd+H` to hide windows
- Windows uses `Ctrl+W` in browsers
- Disable conflicting shortcuts in OS settings

### Special Keys
- Ensure NumLock is off for arrow keys
- Function keys may require `Fn` key
- Some keyboards have no Page Up/Down
- External numpad recommended for jogging

## Future Enhancements

Planned for future versions:
- [ ] Custom shortcut mapping
- [ ] Shortcut profiles (beginner, advanced)
- [ ] On-screen shortcut hints
- [ ] Shortcut conflicts detection
- [ ] Import/export shortcut configurations
- [ ] Context-sensitive help (press ? for shortcuts)

## Reference Card

Print this section for quick reference:

```
┌─────────────────────────────────────┐
│     rCandle Quick Reference         │
├─────────────────────────────────────┤
│ ESSENTIAL                           │
│  Space    Run/Pause                 │
│  Esc      Emergency Stop            │
│  H        Home All                  │
│  Z        Zero All                  │
│                                     │
│ FILE                                │
│  Ctrl+O   Open                      │
│  Ctrl+S   Save                      │
│                                     │
│ JOG                                 │
│  ← → ↑ ↓  X/Y Axis                 │
│  PgUp/Dn  Z Axis                    │
│                                     │
│ VIEW                                │
│  1-7      View Presets              │
│  F        Fit View                  │
│                                     │
│ OVERRIDE                            │
│  [ ]      Feed Rate +/-             │
│  { }      Spindle +/-               │
└─────────────────────────────────────┘
```

---

**Remember**: When in doubt, press `Esc` to stop!

For more information, see the [User Guide](USER_GUIDE.md).
