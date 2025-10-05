//! Connection trait definitions
//!
//! Defines the abstract interface for all connection types.

use crate::Result;
use async_trait::async_trait;
use std::time::Duration;

/// Connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    /// Not connected
    Disconnected,
    /// Attempting to connect
    Connecting,
    /// Connected and ready
    Connected,
    /// Connection error occurred
    Error,
}

/// Connection events that can be broadcast to listeners
#[derive(Debug, Clone)]
pub enum ConnectionEvent {
    /// Connection established
    Connected,
    /// Connection lost
    Disconnected,
    /// Data received from controller
    DataReceived(String),
    /// Error occurred
    Error(String),
}

/// Abstract connection trait for GRBL communication
///
/// This trait defines the interface that all connection types (serial, telnet, websocket)
/// must implement to communicate with GRBL controllers.
#[async_trait]
pub trait Connection: Send + Sync {
    /// Connect to the controller
    ///
    /// # Arguments
    /// * `timeout` - Maximum time to wait for connection
    ///
    /// # Returns
    /// * `Ok(())` if connection successful
    /// * `Err(Error)` if connection failed
    async fn connect(&mut self, timeout: Duration) -> Result<()>;

    /// Disconnect from the controller
    ///
    /// # Returns
    /// * `Ok(())` if disconnection successful
    /// * `Err(Error)` if error occurred during disconnection
    async fn disconnect(&mut self) -> Result<()>;

    /// Check if currently connected
    ///
    /// # Returns
    /// * `true` if connected
    /// * `false` if not connected
    fn is_connected(&self) -> bool;

    /// Get current connection status
    ///
    /// # Returns
    /// Current `ConnectionStatus`
    fn status(&self) -> ConnectionStatus;

    /// Send a line of data to the controller
    ///
    /// The line will be automatically terminated with a newline if not already present.
    ///
    /// # Arguments
    /// * `data` - Line to send
    ///
    /// # Returns
    /// * `Ok(())` if send successful
    /// * `Err(Error)` if send failed
    async fn send_line(&mut self, data: &str) -> Result<()>;

    /// Send raw bytes to the controller
    ///
    /// # Arguments
    /// * `data` - Bytes to send
    ///
    /// # Returns
    /// * `Ok(())` if send successful
    /// * `Err(Error)` if send failed
    async fn send_bytes(&mut self, data: &[u8]) -> Result<()>;

    /// Receive a line of data from the controller
    ///
    /// Blocks until a complete line is received or timeout occurs.
    ///
    /// # Arguments
    /// * `timeout` - Maximum time to wait for data
    ///
    /// # Returns
    /// * `Ok(Some(String))` if line received
    /// * `Ok(None)` if timeout occurred
    /// * `Err(Error)` if error occurred
    async fn receive_line(&mut self, timeout: Duration) -> Result<Option<String>>;

    /// Get connection description/address
    ///
    /// # Returns
    /// String describing the connection (e.g., "COM3", "192.168.1.100:23")
    fn description(&self) -> String;

    /// Flush any pending write data
    ///
    /// # Returns
    /// * `Ok(())` if flush successful
    /// * `Err(Error)` if flush failed
    async fn flush(&mut self) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_status_equality() {
        assert_eq!(ConnectionStatus::Connected, ConnectionStatus::Connected);
        assert_ne!(ConnectionStatus::Connected, ConnectionStatus::Disconnected);
    }

    #[test]
    fn test_connection_event_debug() {
        let event = ConnectionEvent::Connected;
        let debug_str = format!("{:?}", event);
        assert!(debug_str.contains("Connected"));
    }
}
