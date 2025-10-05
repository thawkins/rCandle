//! Connection module
//!
//! This module provides abstract interfaces for communicating with GRBL controllers
//! via different connection types (serial, telnet, websocket).

mod serial;
mod traits;

pub use serial::SerialConnection;
pub use traits::{Connection, ConnectionStatus, ConnectionEvent};
