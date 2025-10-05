//! Telnet connection implementation for GRBL controllers
//!
//! This module provides a basic Telnet connection implementation for connecting
//! to GRBL controllers over TCP/IP networks. This is useful for networked CNC
//! machines or virtual machines running GRBL.

use async_trait::async_trait;
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::timeout;

use crate::connection::traits::{Connection, ConnectionEvent, ConnectionStatus};
use crate::utils::error::{Error, Result};

/// Configuration for a Telnet connection
#[derive(Debug, Clone)]
pub struct TelnetConfig {
    /// Remote host address (IP or hostname)
    pub host: String,
    /// Remote port number (default: 23 for telnet)
    pub port: u16,
    /// Connection timeout in milliseconds
    pub connect_timeout_ms: u64,
    /// Read timeout in milliseconds
    pub read_timeout_ms: u64,
    /// Enable TCP keepalive
    pub keepalive: bool,
    /// TCP keepalive interval in seconds (if enabled)
    pub keepalive_interval_secs: u64,
}

impl Default for TelnetConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 23,
            connect_timeout_ms: 5000,
            read_timeout_ms: 1000,
            keepalive: true,
            keepalive_interval_secs: 60,
        }
    }
}

/// Telnet connection to a GRBL controller
pub struct TelnetConnection {
    config: TelnetConfig,
    stream: Arc<Mutex<Option<TcpStream>>>,
    status: Arc<Mutex<ConnectionStatus>>,
    receive_buffer: Arc<Mutex<VecDeque<String>>>,
}

impl TelnetConnection {
    /// Create a new Telnet connection with the given configuration
    pub fn new(config: TelnetConfig) -> Self {
        Self {
            config,
            stream: Arc::new(Mutex::new(None)),
            status: Arc::new(Mutex::new(ConnectionStatus::Disconnected)),
            receive_buffer: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Create a new Telnet connection with default configuration
    pub fn with_address(host: String, port: u16) -> Self {
        let config = TelnetConfig {
            host,
            port,
            ..Default::default()
        };
        Self::new(config)
    }

    /// Get the current configuration
    pub fn config(&self) -> &TelnetConfig {
        &self.config
    }
}

#[async_trait]
impl Connection for TelnetConnection {
    async fn connect(&mut self, connect_timeout: Duration) -> Result<()> {
        // Update status to connecting
        {
            let mut status = self.status.lock().await;
            *status = ConnectionStatus::Connecting;
        }

        // Build the address
        let addr = format!("{}:{}", self.config.host, self.config.port);

        // Attempt connection with timeout
        let stream = timeout(connect_timeout, TcpStream::connect(&addr))
            .await
            .map_err(|_| Error::Connection(format!("Connection timeout to {}", addr)))?
            .map_err(|e| Error::Connection(format!("Failed to connect to {}: {}", addr, e)))?;

        // Configure keepalive if enabled
        if self.config.keepalive {
            let keepalive = socket2::TcpKeepalive::new()
                .with_time(Duration::from_secs(self.config.keepalive_interval_secs));
            let sock_ref = socket2::SockRef::from(&stream);
            sock_ref
                .set_tcp_keepalive(&keepalive)
                .map_err(|e| Error::Connection(format!("Failed to set keepalive: {}", e)))?;
        }

        // Store the stream
        {
            let mut stream_lock = self.stream.lock().await;
            *stream_lock = Some(stream);
        }

        // Update status to connected
        {
            let mut status = self.status.lock().await;
            *status = ConnectionStatus::Connected;
        }

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        // Close the stream
        {
            let mut stream_lock = self.stream.lock().await;
            if let Some(mut stream) = stream_lock.take() {
                let _ = stream.shutdown().await;
            }
        }

        // Clear receive buffer
        {
            let mut buffer = self.receive_buffer.lock().await;
            buffer.clear();
        }

        // Update status
        {
            let mut status = self.status.lock().await;
            *status = ConnectionStatus::Disconnected;
        }

        Ok(())
    }

    async fn send_line(&mut self, data: &str) -> Result<()> {
        let mut stream_lock = self.stream.lock().await;
        let stream = stream_lock
            .as_mut()
            .ok_or_else(|| Error::Connection("Not connected".to_string()))?;

        // Send the data with newline
        let line = format!("{}\n", data);
        stream
            .write_all(line.as_bytes())
            .await
            .map_err(|e| Error::Connection(format!("Failed to send data: {}", e)))?;

        stream
            .flush()
            .await
            .map_err(|e| Error::Connection(format!("Failed to flush data: {}", e)))?;

        Ok(())
    }

    async fn send_bytes(&mut self, data: &[u8]) -> Result<()> {
        let mut stream_lock = self.stream.lock().await;
        let stream = stream_lock
            .as_mut()
            .ok_or_else(|| Error::Connection("Not connected".to_string()))?;

        // Send raw bytes
        stream
            .write_all(data)
            .await
            .map_err(|e| Error::Connection(format!("Failed to send bytes: {}", e)))?;

        stream
            .flush()
            .await
            .map_err(|e| Error::Connection(format!("Failed to flush bytes: {}", e)))?;

        Ok(())
    }

    async fn receive_line(&mut self, read_timeout: Duration) -> Result<Option<String>> {
        // Check buffer first
        {
            let mut buffer = self.receive_buffer.lock().await;
            if let Some(line) = buffer.pop_front() {
                return Ok(Some(line));
            }
        }

        // Read from stream
        let mut stream_lock = self.stream.lock().await;
        let stream = stream_lock
            .as_mut()
            .ok_or_else(|| Error::Connection("Not connected".to_string()))?;

        // Create a buffered reader
        let mut reader = BufReader::new(stream);
        let mut line = String::new();

        match timeout(read_timeout, reader.read_line(&mut line)).await {
            Ok(Ok(0)) => {
                // Connection closed
                drop(reader);
                drop(stream_lock);
                self.disconnect().await?;
                return Err(Error::Connection("Connection closed by remote host".to_string()));
            }
            Ok(Ok(_)) => {
                // Successfully read a line
                let line = line.trim_end().to_string();
                Ok(Some(line))
            }
            Ok(Err(e)) => Err(Error::Connection(format!("Failed to read line: {}", e))),
            Err(_) => {
                // Timeout - return None to indicate no data available
                Ok(None)
            }
        }
    }

    fn is_connected(&self) -> bool {
        // We need to block on the async lock
        // For a sync trait method, we'll use try_lock instead
        if let Ok(status) = self.status.try_lock() {
            matches!(*status, ConnectionStatus::Connected)
        } else {
            false
        }
    }

    fn status(&self) -> ConnectionStatus {
        // Use try_lock for sync method
        if let Ok(status) = self.status.try_lock() {
            status.clone()
        } else {
            ConnectionStatus::Error
        }
    }

    fn description(&self) -> String {
        format!("Telnet: {}:{}", self.config.host, self.config.port)
    }

    async fn flush(&mut self) -> Result<()> {
        let mut stream_lock = self.stream.lock().await;
        let stream = stream_lock
            .as_mut()
            .ok_or_else(|| Error::Connection("Not connected".to_string()))?;

        stream
            .flush()
            .await
            .map_err(|e| Error::Connection(format!("Failed to flush: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_telnet_config_default() {
        let config = TelnetConfig::default();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 23);
        assert_eq!(config.connect_timeout_ms, 5000);
        assert_eq!(config.read_timeout_ms, 1000);
        assert!(config.keepalive);
    }

    #[tokio::test]
    async fn test_telnet_creation() {
        let conn = TelnetConnection::with_address("192.168.1.100".to_string(), 8080);
        assert_eq!(conn.config().host, "192.168.1.100");
        assert_eq!(conn.config().port, 8080);
    }

    #[tokio::test]
    async fn test_telnet_status() {
        let conn = TelnetConnection::new(TelnetConfig::default());
        let status = conn.status();
        assert!(matches!(status, ConnectionStatus::Disconnected));
    }

    #[tokio::test]
    async fn test_telnet_description() {
        let conn = TelnetConnection::with_address("example.com".to_string(), 2323);
        let desc = conn.description();
        assert_eq!(desc, "Telnet: example.com:2323");
    }

    #[tokio::test]
    async fn test_telnet_send_when_disconnected() {
        let mut conn = TelnetConnection::new(TelnetConfig::default());
        let result = conn.send_line("G0 X10").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_telnet_receive_when_disconnected() {
        let mut conn = TelnetConnection::new(TelnetConfig::default());
        let result = conn.receive_line(Duration::from_millis(100)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_telnet_is_connected() {
        let conn = TelnetConnection::new(TelnetConfig::default());
        assert!(!conn.is_connected());
    }
}
