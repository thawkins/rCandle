# State Management Module

## Overview

The state management module provides thread-safe state tracking for the rCandle application. It implements a comprehensive state system with change notifications using a publish-subscribe pattern.

## Architecture

### Components

1. **MachineState** - Tracks the CNC machine's current state
2. **ProgramState** - Tracks G-code program execution state
3. **AppState** - Top-level application state container
4. **StateEvent** - Event system for state change notifications
5. **StateEventBroadcaster** - Pub/sub system for distributing state events
6. **StateUpdater** - Processes GRBL responses and updates state

### Thread Safety

All state is wrapped in `SharedState<T>`, which uses `Arc<RwLock<T>>` internally. This provides:
- Thread-safe concurrent access
- Multiple readers or single writer semantics
- Clone-able state handles for passing between threads

## MachineState

Tracks the physical CNC machine state received from GRBL.

### Fields

```rust
pub struct MachineState {
    pub status: MachineStatus,              // Current machine status
    pub machine_position: Position,         // Machine coordinates
    pub work_position: Position,            // Work coordinates
    pub coordinate_system: CoordinateSystem, // Active WCS (G54-G59)
    pub work_offsets: [Position; 6],        // WCS offsets
    pub spindle_speed: f64,                 // RPM
    pub spindle_enabled: bool,              // Spindle state
    pub feed_rate: f64,                     // mm/min or in/min
    pub feed_override: f64,                 // Percentage (0-200)
    pub spindle_override: f64,              // Percentage (0-200)
    pub rapid_override: f64,                // Percentage (25-100)
    pub buffer_state: u32,                  // Planner buffer
    pub last_error: Option<String>,         // Last error message
}
```

### Machine Status Values

- `Idle` - Machine is ready for commands
- `Run` - Executing G-code program
- `Hold` - Program paused
- `Jog` - Manual jogging
- `Alarm` - Error state, requires reset
- `Door` - Safety door triggered
- `Check` - G-code check mode
- `Home` - Homing cycle active
- `Sleep` - Sleep mode
- `Unknown` - Unrecognized state

### Coordinate Systems

Supports GRBL work coordinate systems G54 through G59:
- Each system has its own offset from machine coordinates
- Active system determines work position calculation
- Work position = Machine position - Work offset

### Methods

```rust
// Get/set work offsets for coordinate systems
pub fn get_work_offset(&self, cs: CoordinateSystem) -> Position
pub fn set_work_offset(&mut self, cs: CoordinateSystem, offset: Position)

// Update machine position and recalculate work position
pub fn update_machine_position(&mut self, pos: Position)

// State queries
pub fn is_error_state(&self) -> bool
pub fn is_idle(&self) -> bool
pub fn is_running(&self) -> bool
```

## ProgramState

Tracks G-code program execution progress.

### Fields

```rust
pub struct ProgramState {
    pub state: ExecutionState,           // Current execution state
    pub file_path: Option<String>,       // Loaded file path
    pub total_lines: usize,              // Total lines in program
    pub current_line: usize,             // Currently executing line
    pub lines_sent: usize,               // Lines sent to controller
    pub lines_completed: usize,          // Lines completed
    pub start_time: Option<Instant>,     // Execution start time
    pub elapsed_time: Duration,          // Elapsed execution time
    pub estimated_remaining: Option<Duration>, // Estimated time remaining
    pub bounds_min: Option<[f64; 3]>,    // Program bounding box min
    pub bounds_max: Option<[f64; 3]>,    // Program bounding box max
}
```

### Execution States

- `NotLoaded` - No program loaded
- `Loaded` - Program loaded but not started
- `Running` - Program executing
- `Paused` - Program paused
- `Completed` - Program finished successfully
- `Error` - Program stopped due to error

### Lifecycle Methods

```rust
pub fn load(&mut self, file_path: String, total_lines: usize)
pub fn start(&mut self)
pub fn pause(&mut self)
pub fn stop(&mut self)
pub fn complete(&mut self)
pub fn error(&mut self)
pub fn reset(&mut self)
```

### Progress Tracking

```rust
// Get progress as 0.0 to 1.0
pub fn progress(&self) -> f64

// Increment completion counter
pub fn increment_completed(&mut self)

// Update elapsed and estimated remaining time
pub fn update_elapsed_time(&mut self)
```

## AppState

Top-level application state container.

```rust
pub struct AppState {
    pub machine: SharedState<MachineState>,
    pub program: SharedState<ProgramState>,
    pub connected: SharedState<bool>,
}
```

Access pattern:
```rust
let app_state = AppState::new();

// Read access
{
    let machine = app_state.machine.read();
    println!("Status: {}", machine.status);
}

// Write access
{
    let mut machine = app_state.machine.write();
    machine.spindle_speed = 1000.0;
}

// Update with closure
app_state.machine.update(|m| {
    m.spindle_speed = 1000.0;
    m.spindle_enabled = true;
});
```

## State Events

Events are emitted when state changes occur.

### Event Types

```rust
pub enum StateEvent {
    MachineStatusChanged { old, new },
    MachinePositionChanged { machine_pos, work_pos },
    SpindleStateChanged { enabled, speed },
    FeedRateChanged { feed_rate },
    OverridesChanged { feed, rapid, spindle },
    WorkOffsetChanged { system, offset },
    CoordinateSystemChanged { old, new },
    ProgramStateChanged { old, new },
    ProgramProgressChanged { current_line, total_lines, progress },
    ErrorOccurred { message },
    ConnectionChanged { connected },
}
```

### StateEventBroadcaster

Implements publish-subscribe pattern using `tokio::sync::broadcast`.

```rust
// Create broadcaster (typically once at startup)
let broadcaster = StateEventBroadcaster::new(100); // buffer 100 events

// Subscribe to events
let mut receiver = broadcaster.subscribe();

// Send events
broadcaster.send(StateEvent::MachineStatusChanged {
    old: MachineStatus::Idle,
    new: MachineStatus::Run,
});

// Receive events (async)
while let Ok(event) = receiver.recv().await {
    match event {
        StateEvent::MachineStatusChanged { old, new } => {
            println!("Status changed: {:?} -> {:?}", old, new);
        }
        _ => {}
    }
}
```

### Multiple Subscribers

Multiple components can subscribe to state events:
- UI can update displays
- Logger can record state changes
- Persistence layer can save state
- External integrations can react to changes

## StateUpdater

Processes GRBL responses and updates state accordingly.

### Usage

```rust
let app_state = AppState::new();
let broadcaster = StateEventBroadcaster::new(100);
let updater = StateUpdater::new(app_state.clone(), broadcaster.clone());

// Process GRBL responses
updater.process_response(&grbl_response);

// Manual state updates
updater.start_program();
updater.pause_program();
updater.set_coordinate_system(CoordinateSystem::G55);
updater.set_work_offset(CoordinateSystem::G54, Position::new(10.0, 20.0, 5.0));
```

### GRBL Response Processing

The `StateUpdater` automatically processes GRBL responses:

- **Status Reports** (`<Idle|MPos:...>`) - Updates machine position, status, overrides
- **OK** - Increments program progress
- **Error/Alarm** - Sets error state, emits error event
- **Welcome** - Sets connected state
- **Settings** - Logged for debugging
- **Feedback/Messages** - Logged

### State-Response Mapping

| GRBL Response | State Update | Event Emitted |
|---------------|--------------|---------------|
| `<Idle\|...>` | Machine status | MachineStatusChanged |
| `<...MPos:...>` | Machine position | MachinePositionChanged |
| `<...F:500>` | Feed rate | FeedRateChanged |
| `<...S:1000>` | Spindle speed | SpindleStateChanged |
| `<...Ov:100,50,120>` | Overrides | OverridesChanged |
| `ok` | Program progress | ProgramProgressChanged |
| `error:X` | Error state | ErrorOccurred, ProgramStateChanged |

## Integration Example

Complete integration with connection and UI:

```rust
use rcandle::state::{AppState, StateEventBroadcaster, StateUpdater};
use rcandle::connection::ConnectionManager;

#[tokio::main]
async fn main() {
    // Create state infrastructure
    let app_state = AppState::new();
    let broadcaster = StateEventBroadcaster::new(100);
    let updater = StateUpdater::new(app_state.clone(), broadcaster.clone());
    
    // Subscribe to events for UI updates
    let mut event_rx = broadcaster.subscribe();
    tokio::spawn(async move {
        while let Ok(event) = event_rx.recv().await {
            // Update UI based on event
            println!("State event: {:?}", event);
        }
    });
    
    // Create connection and process responses
    let connection_manager = ConnectionManager::new();
    
    // Connect and get response stream
    connection_manager.connect_serial("/dev/ttyUSB0", 115200).await?;
    let mut response_stream = connection_manager.get_response_stream();
    
    // Process responses and update state
    while let Some(response) = response_stream.recv().await {
        updater.process_response(&response);
    }
}
```

## Testing

The state module includes comprehensive tests:

- **Machine State Tests** (3 tests)
  - Position handling
  - Work offset management
  - Position calculations
  
- **Program State Tests** (3 tests)
  - Program lifecycle
  - Progress tracking
  - State transitions
  
- **Event System Tests** (5 tests)
  - Broadcaster creation
  - Subscribe/unsubscribe
  - Event delivery
  - Multiple subscribers
  - No-receiver handling
  
- **State Updater Tests** (4 tests)
  - GRBL state conversion
  - Error handling
  - Coordinate system updates
  - Program lifecycle

Run tests:
```bash
cargo test state::
```

## Thread Safety Guarantees

### Read-Write Lock Semantics

- Multiple concurrent readers allowed
- Only one writer at a time
- Readers wait for writers to finish
- Writers wait for all readers to finish

### Deadlock Prevention

To prevent deadlocks:
1. Always acquire locks in the same order
2. Keep lock scope as small as possible
3. Don't call external code while holding locks
4. Use `update()` method for simple modifications

### Clone Safety

`AppState` is `Clone`, but cloning creates a new handle to the same underlying state:
```rust
let state1 = AppState::new();
let state2 = state1.clone();  // Same state, different handle

// Both see the same data
state1.machine.write().spindle_speed = 1000.0;
assert_eq!(state2.machine.read().spindle_speed, 1000.0);
```

## Performance Considerations

- **Event Buffer Size**: Default 100 events. Increase if events are lost.
- **Lock Contention**: Minimize time spent holding locks
- **Event Frequency**: Status reports typically arrive 10-100 Hz
- **Memory**: State is lightweight (~1KB total)

## Future Enhancements

Potential future additions:
- State persistence (save/restore)
- State history/undo
- State snapshots
- State diff/patch
- Event replay
- State validation rules
- Custom event filters
- Event rate limiting

## See Also

- [Connection Module](CONNECTION_MODULE.md) - GRBL communication
- [GRBL Protocol](../src/grbl/mod.rs) - Response parsing
- [UI Module](../src/ui/mod.rs) - State display
