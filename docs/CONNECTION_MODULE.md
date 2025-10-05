# Connection Module Documentation

## Overview

The connection module provides a flexible, async-first interface for communicating with GRBL controllers through multiple protocols. It supports Serial, Telnet, and WebSocket connections with a unified API.

## Architecture

```
┌─────────────────────────────────┐
│    ConnectionManager            │
│  - Command Queue Management     │
│  - Status Broadcasting          │
│  - Background Task Orchestration│
└────────────┬────────────────────┘
             │ uses
             ↓
┌─────────────────────────────────┐
│      Connection Trait           │
│  - connect/disconnect           │
│  - send_line/send_bytes         │
│  - receive_line                 │
│  - status/description           │
└────────────┬────────────────────┘
             │ implemented by
        ┌────┴────┬────────┬──────┐
        ↓         ↓        ↓      ↓
  ┌──────────┬───────┬────────┬────────┐
  │ Serial   │Telnet │WebSocket│ ...    │
  └──────────┴───────┴────────┴────────┘
```

## Connection Types

### 1. SerialConnection

Direct serial port communication, the most common connection type for GRBL controllers.

**Features:**
- Standard serial communication (RS-232)
- Configurable baud rate, data bits, stop bits, parity
- Flow control support (RTS/CTS, Xon/Xoff)
- Built-in buffering

**Example:**
```rust
use rcandle::connection::{Connection, SerialConnection};
use std::time::Duration;

let mut conn = SerialConnection::new("/dev/ttyUSB0".to_string(), 115200);
conn.connect(Duration::from_secs(5)).await?;
conn.send_line("G0 X10 Y10").await?;
```

**Best for:**
- Direct USB connections
- Traditional CNC setups
- Reliable, low-latency communication

### 2. TelnetConnection

TCP/IP network connection using the Telnet protocol.

**Features:**
- Network-based communication
- TCP keepalive support
- Configurable timeouts
- Works over LAN/WAN

**Example:**
```rust
use rcandle::connection::{Connection, TelnetConnection, TelnetConfig};
use std::time::Duration;

let config = TelnetConfig {
    host: "192.168.1.100".to_string(),
    port: 23,
    keepalive: true,
    ..Default::default()
};

let mut conn = TelnetConnection::new(config);
conn.connect(Duration::from_secs(5)).await?;
```

**Best for:**
- Network-connected CNCs
- Remote machine control
- ESP32/ESP8266 GRBL interfaces

### 3. WebSocketConnection

WebSocket protocol connection for web-based interfaces.

**Features:**
- Full-duplex communication
- Support for ws:// and wss:// (TLS)
- Automatic ping/pong keepalive
- Binary and text message support

**Example:**
```rust
use rcandle::connection::{Connection, WebSocketConnection};
use std::time::Duration;

let mut conn = WebSocketConnection::with_url(
    "ws://192.168.1.100:8080/grbl".to_string()
);
conn.connect(Duration::from_secs(5)).await?;
```

**Best for:**
- Web-based control interfaces
- Cloud-connected machines
- Cross-platform browser compatibility

## ConnectionManager

The ConnectionManager provides high-level management of connections with advanced features:

**Features:**
1. **Command Queue Management**
   - Automatic command flow control
   - Wait for acknowledgments before sending next command
   - Configurable queue size

2. **Status Broadcasting**
   - Automatic periodic status queries
   - Broadcast status updates to multiple subscribers
   - Real-time machine state monitoring

3. **Event System**
   - Connection events (connected, disconnected, errors)
   - Response broadcasts to all subscribers
   - Decoupled event handling

4. **Background Tasks**
   - Response receiving loop
   - Command processing loop
   - Status query loop (if enabled)
   - Graceful shutdown coordination

**Example:**
```rust
use rcandle::connection::{ConnectionManager, ConnectionManagerConfig, SerialConnection};
use std::time::Duration;

let connection = Box::new(SerialConnection::new("/dev/ttyUSB0".to_string(), 115200));
let config = ConnectionManagerConfig {
    status_interval_ms: 250,
    auto_status_query: true,
    ..Default::default()
};

let mut manager = ConnectionManager::with_config(connection, config);

// Subscribe to updates
let mut status_rx = manager.subscribe_status();
let mut event_rx = manager.subscribe_events();

// Connect
manager.connect(Duration::from_secs(5)).await?;

// Send commands (automatically queued)
manager.send_command(GrblCommand::GCode("G0 X10".to_string())).await?;

// Handle status updates in a background task
tokio::spawn(async move {
    while let Ok(status) = status_rx.recv().await {
        println!("Machine state: {:?}", status.state);
    }
});
```

## Configuration

### Serial Configuration
```rust
let mut conn = SerialConnection::new(port, baud_rate);
// Default: 8N1, no flow control, 1000ms timeout
```

### Telnet Configuration
```rust
let config = TelnetConfig {
    host: "192.168.1.100".to_string(),
    port: 23,
    connect_timeout_ms: 5000,
    read_timeout_ms: 1000,
    keepalive: true,
    keepalive_interval_secs: 60,
};
```

### WebSocket Configuration
```rust
let config = WebSocketConfig {
    url: "ws://192.168.1.100:8080/grbl".to_string(),
    connect_timeout_ms: 5000,
    read_timeout_ms: 1000,
    ping_interval_secs: 30,
    auto_reconnect: false,
};
```

### ConnectionManager Configuration
```rust
let config = ConnectionManagerConfig {
    status_interval_ms: 250,           // Status query every 250ms
    response_timeout: Duration::from_millis(2000),
    reconnect_attempts: 3,
    reconnect_delay: Duration::from_secs(1),
    auto_status_query: true,           // Enable automatic status queries
};
```

## Error Handling

All connection operations return `Result<T, Error>` where `Error` is the crate's unified error type:

```rust
use rcandle::Result;

async fn example() -> Result<()> {
    let mut conn = SerialConnection::new("/dev/ttyUSB0".to_string(), 115200);
    
    // Explicit error handling
    match conn.connect(Duration::from_secs(5)).await {
        Ok(_) => println!("Connected!"),
        Err(e) => eprintln!("Connection failed: {}", e),
    }
    
    // Or use ? operator
    conn.send_line("G0 X10")? ;
    
    Ok(())
}
```

## Thread Safety

All connection types are designed to be used with async/await and are:
- `Send` + `Sync` for use across tasks
- Use `Arc<Mutex<>>` internally where needed
- Safe for concurrent access through ConnectionManager

## Examples

See the `examples/` directory for complete working examples:
- `serial_connection.rs` - Basic serial connection
- `telnet_connection.rs` - Network connection via Telnet
- `websocket_connection.rs` - WebSocket connection
- `connection_manager.rs` - Advanced usage with ConnectionManager

Run examples with:
```bash
cargo run --example serial_connection
cargo run --example telnet_connection
cargo run --example websocket_connection
cargo run --example connection_manager
```

## Testing

The connection module has comprehensive test coverage:
- 95 unit tests (100% pass rate)
- Tests for all connection types
- Tests for ConnectionManager
- Tests for GRBL protocol handling

Run tests with:
```bash
cargo test
```

## Future Enhancements

Planned improvements:
- [ ] UDP connection support
- [ ] Bluetooth connection support
- [ ] Mock connection for testing
- [ ] Connection pooling
- [ ] Automatic reconnection logic
- [ ] Metrics and monitoring
