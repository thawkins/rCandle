//! WebSocket connection implementation for GRBL controllers
//!
//! This module provides a WebSocket connection implementation for connecting
//! to GRBL controllers through WebSocket protocols. This is useful for
//! web-based interfaces and cloud-connected CNC machines.

use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};
use tokio_tungstenite::MaybeTlsStream;
use tokio::net::TcpStream;

use crate::connection::traits::{Connection, ConnectionEvent, ConnectionStatus};
use crate::utils::error::{Error, Result};

/// Configuration for a WebSocket connection
#[derive(Debug, Clone)]
pub struct WebSocketConfig {
    /// WebSocket URL (ws:// or wss://)
    pub url: String,
    /// Connection timeout in milliseconds
    pub connect_timeout_ms: u64,
    /// Read timeout in milliseconds
    pub read_timeout_ms: u64,
    /// Ping interval in seconds (0 to disable)
    pub ping_interval_secs: u64,
    /// Reconnect automatically on disconnect
    pub auto_reconnect: bool,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            url: "ws://localhost:8080".to_string(),
            connect_timeout_ms: 5000,
            read_timeout_ms: 1000,
            ping_interval_secs: 30,
            auto_reconnect: false,
        }
    }
}

/// WebSocket connection to a GRBL controller
pub struct WebSocketConnection {
    config: WebSocketConfig,
    stream: Arc<Mutex<Option<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    status: Arc<Mutex<ConnectionStatus>>,
    receive_buffer: Arc<Mutex<VecDeque<String>>>,
}

impl WebSocketConnection {
    /// Create a new WebSocket connection with the given configuration
    pub fn new(config: WebSocketConfig) -> Self {
        Self {
            config,
            stream: Arc::new(Mutex::new(None)),
            status: Arc::new(Mutex::new(ConnectionStatus::Disconnected)),
            receive_buffer: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Create a new WebSocket connection with a URL
    pub fn with_url(url: String) -> Self {
        let config = WebSocketConfig {
            url,
            ..Default::default()
        };
        Self::new(config)
    }

    /// Get the current configuration
    pub fn config(&self) -> &WebSocketConfig {
        &self.config
    }
}

#[async_trait]
impl Connection for WebSocketConnection {
    async fn connect(&mut self, connect_timeout: Duration) -> Result<()> {
        // Update status to connecting
        {
            let mut status = self.status.lock().await;
            *status = ConnectionStatus::Connecting;
        }

        // Attempt connection with timeout
        let url = self.config.url.clone();

        let (ws_stream, _) = timeout(connect_timeout, connect_async(&url))
            .await
            .map_err(|_| Error::Connection(format!("Connection timeout to {}", url)))?
            .map_err(|e| Error::Connection(format!("Failed to connect to {}: {}", url, e)))?;

        // Store the stream
        {
            let mut stream_lock = self.stream.lock().await;
            *stream_lock = Some(ws_stream);
        }

        // Update status to connected
        {
            let mut status = self.status.lock().await;
            *status = ConnectionStatus::Connected;
        }

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        // Close the WebSocket connection
        {
            let mut stream_lock = self.stream.lock().await;
            if let Some(mut stream) = stream_lock.take() {
                let _ = stream.close(None).await;
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

        // Send as text message with newline
        let line = format!("{}\n", data);
        stream
            .send(Message::Text(line))
            .await
            .map_err(|e| Error::Connection(format!("Failed to send message: {}", e)))?;

        Ok(())
    }

    async fn send_bytes(&mut self, data: &[u8]) -> Result<()> {
        let mut stream_lock = self.stream.lock().await;
        let stream = stream_lock
            .as_mut()
            .ok_or_else(|| Error::Connection("Not connected".to_string()))?;

        // Send as binary message
        stream
            .send(Message::Binary(data.to_vec()))
            .await
            .map_err(|e| Error::Connection(format!("Failed to send binary: {}", e)))?;

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

        match timeout(read_timeout, stream.next()).await {
            Ok(Some(Ok(msg))) => match msg {
                Message::Text(text) => {
                    // Split on newlines and add to buffer
                    let lines: Vec<String> = text
                        .lines()
                        .map(|s| s.trim_end().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();

                    if lines.is_empty() {
                        return Ok(None);
                    }

                    // Return first line, buffer the rest
                    if lines.len() > 1 {
                        let mut buffer = self.receive_buffer.lock().await;
                        for line in lines.iter().skip(1) {
                            buffer.push_back(line.clone());
                        }
                    }

                    Ok(Some(lines[0].clone()))
                }
                Message::Binary(_) => {
                    // For binary messages, we'll skip them for now
                    // as GRBL primarily uses text
                    Ok(None)
                }
                Message::Ping(_) => {
                    // Pong will be sent automatically by tungstenite
                    Ok(None)
                }
                Message::Pong(_) => Ok(None),
                Message::Close(_) => {
                    drop(stream_lock);
                    self.disconnect().await?;
                    Err(Error::Connection("Connection closed by remote host".to_string()))
                }
                Message::Frame(_) => Ok(None),
            },
            Ok(Some(Err(e))) => Err(Error::Connection(format!("WebSocket error: {}", e))),
            Ok(None) => {
                // Stream ended
                drop(stream_lock);
                self.disconnect().await?;
                Err(Error::Connection("Connection closed".to_string()))
            }
            Err(_) => {
                // Timeout - return None to indicate no data available
                Ok(None)
            }
        }
    }

    fn is_connected(&self) -> bool {
        // Use try_lock for sync method
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
        format!("WebSocket: {}", self.config.url)
    }

    async fn flush(&mut self) -> Result<()> {
        // WebSocket connections are automatically flushed on send
        // No explicit flush needed
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_websocket_config_default() {
        let config = WebSocketConfig::default();
        assert_eq!(config.url, "ws://localhost:8080");
        assert_eq!(config.connect_timeout_ms, 5000);
        assert_eq!(config.read_timeout_ms, 1000);
        assert_eq!(config.ping_interval_secs, 30);
        assert!(!config.auto_reconnect);
    }

    #[tokio::test]
    async fn test_websocket_creation() {
        let conn = WebSocketConnection::with_url("wss://example.com:9090/grbl".to_string());
        assert_eq!(conn.config().url, "wss://example.com:9090/grbl");
    }

    #[tokio::test]
    async fn test_websocket_status() {
        let conn = WebSocketConnection::new(WebSocketConfig::default());
        let status = conn.status();
        assert!(matches!(status, ConnectionStatus::Disconnected));
    }

    #[tokio::test]
    async fn test_websocket_description() {
        let conn = WebSocketConnection::with_url("ws://192.168.1.50:3000".to_string());
        let desc = conn.description();
        assert_eq!(desc, "WebSocket: ws://192.168.1.50:3000");
    }

    #[tokio::test]
    async fn test_websocket_send_when_disconnected() {
        let mut conn = WebSocketConnection::new(WebSocketConfig::default());
        let result = conn.send_line("G0 X10").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_websocket_receive_when_disconnected() {
        let mut conn = WebSocketConnection::new(WebSocketConfig::default());
        let result = conn.receive_line(Duration::from_millis(100)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_websocket_is_connected() {
        let conn = WebSocketConnection::new(WebSocketConfig::default());
        assert!(!conn.is_connected());
    }
}
