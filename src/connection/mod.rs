//! Connection module
//!
//! This module provides abstract interfaces for communicating with GRBL controllers
//! via different connection types (serial, telnet, websocket).

mod manager;
mod serial;
mod telnet;
mod traits;
mod websocket;

pub use manager::{ConnectionManager, ConnectionManagerConfig};
pub use serial::{SerialConfig, SerialConnection};
pub use telnet::{TelnetConfig, TelnetConnection};
pub use traits::{Connection, ConnectionEvent, ConnectionStatus};
pub use websocket::{WebSocketConfig, WebSocketConnection};
