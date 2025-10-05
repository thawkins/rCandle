//! Connection module
//!
//! This module provides abstract interfaces for communicating with GRBL controllers
//! via different connection types (serial, telnet, websocket).

mod manager;
mod serial;
mod traits;

pub use manager::{ConnectionManager, ConnectionManagerConfig};
pub use serial::SerialConnection;
pub use traits::{Connection, ConnectionStatus, ConnectionEvent};
