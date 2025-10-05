//! Connection Manager
//!
//! Coordinates connection lifecycle, command sending, response receiving,
//! and status broadcasting.

use crate::connection::{Connection, ConnectionEvent, ConnectionStatus};
use crate::grbl::{CommandQueue, GrblCommand, GrblResponse, GrblStatus, QueueState};
use crate::utils::error::{Error, Result};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, RwLock};
use tokio::time::{interval, sleep};

/// Default status query interval (milliseconds)
const DEFAULT_STATUS_INTERVAL_MS: u64 = 250;

/// Default response timeout
const DEFAULT_RESPONSE_TIMEOUT: Duration = Duration::from_secs(1);

/// Default reconnection attempts
const DEFAULT_RECONNECT_ATTEMPTS: u32 = 3;

/// Default reconnection delay
const DEFAULT_RECONNECT_DELAY: Duration = Duration::from_secs(2);

/// Connection manager configuration
#[derive(Debug, Clone)]
pub struct ConnectionManagerConfig {
    /// Interval for automatic status queries (milliseconds)
    pub status_interval_ms: u64,
    /// Response timeout duration
    pub response_timeout: Duration,
    /// Maximum reconnection attempts on disconnect
    pub reconnect_attempts: u32,
    /// Delay between reconnection attempts
    pub reconnect_delay: Duration,
    /// Enable automatic status queries
    pub auto_status_query: bool,
}

impl Default for ConnectionManagerConfig {
    fn default() -> Self {
        Self {
            status_interval_ms: DEFAULT_STATUS_INTERVAL_MS,
            response_timeout: DEFAULT_RESPONSE_TIMEOUT,
            reconnect_attempts: DEFAULT_RECONNECT_ATTEMPTS,
            reconnect_delay: DEFAULT_RECONNECT_DELAY,
            auto_status_query: true,
        }
    }
}

/// Connection Manager
///
/// Manages the connection lifecycle, coordinates command sending/receiving,
/// and broadcasts status updates.
pub struct ConnectionManager {
    /// The underlying connection
    connection: Arc<RwLock<Box<dyn Connection>>>,
    
    /// Command queue
    queue: Arc<RwLock<CommandQueue>>,
    
    /// Configuration
    config: ConnectionManagerConfig,
    
    /// Status broadcast channel
    status_tx: broadcast::Sender<GrblStatus>,
    
    /// Event broadcast channel
    event_tx: broadcast::Sender<ConnectionEvent>,
    
    /// Response broadcast channel
    response_tx: broadcast::Sender<GrblResponse>,
    
    /// Shutdown signal
    shutdown_tx: Option<broadcast::Sender<()>>,
    
    /// Current connection status
    status: Arc<RwLock<ConnectionStatus>>,
}

impl ConnectionManager {
    /// Create a new connection manager
    ///
    /// # Arguments
    /// * `connection` - The connection to manage
    ///
    /// # Returns
    /// A new ConnectionManager instance
    pub fn new(connection: Box<dyn Connection>) -> Self {
        Self::with_config(connection, ConnectionManagerConfig::default())
    }
    
    /// Create a new connection manager with custom configuration
    ///
    /// # Arguments
    /// * `connection` - The connection to manage
    /// * `config` - Configuration options
    ///
    /// # Returns
    /// A new ConnectionManager instance
    pub fn with_config(connection: Box<dyn Connection>, config: ConnectionManagerConfig) -> Self {
        let (status_tx, _) = broadcast::channel(100);
        let (event_tx, _) = broadcast::channel(100);
        let (response_tx, _) = broadcast::channel(100);
        
        Self {
            connection: Arc::new(RwLock::new(connection)),
            queue: Arc::new(RwLock::new(CommandQueue::new())),
            config,
            status_tx,
            event_tx,
            response_tx,
            shutdown_tx: None,
            status: Arc::new(RwLock::new(ConnectionStatus::Disconnected)),
        }
    }
    
    /// Connect to the controller
    ///
    /// # Arguments
    /// * `timeout` - Connection timeout
    ///
    /// # Returns
    /// * `Ok(())` if connection successful
    /// * `Err(Error)` if connection failed
    pub async fn connect(&mut self, timeout: Duration) -> Result<()> {
        // Update status
        *self.status.write().await = ConnectionStatus::Connecting;
        
        // Attempt connection
        let mut conn = self.connection.write().await;
        match conn.connect(timeout).await {
            Ok(()) => {
                drop(conn); // Release lock before starting background tasks
                *self.status.write().await = ConnectionStatus::Connected;
                
                // Broadcast connection event
                let _ = self.event_tx.send(ConnectionEvent::Connected);
                
                // Start background tasks
                self.start_background_tasks().await?;
                
                Ok(())
            }
            Err(e) => {
                *self.status.write().await = ConnectionStatus::Error;
                let _ = self.event_tx.send(ConnectionEvent::Error(e.to_string()));
                Err(e)
            }
        }
    }
    
    /// Disconnect from the controller
    ///
    /// # Returns
    /// * `Ok(())` if disconnection successful
    /// * `Err(Error)` if error occurred
    pub async fn disconnect(&mut self) -> Result<()> {
        // Stop background tasks
        self.stop_background_tasks().await;
        
        // Disconnect
        let mut conn = self.connection.write().await;
        match conn.disconnect().await {
            Ok(()) => {
                drop(conn);
                *self.status.write().await = ConnectionStatus::Disconnected;
                let _ = self.event_tx.send(ConnectionEvent::Disconnected);
                Ok(())
            }
            Err(e) => {
                *self.status.write().await = ConnectionStatus::Error;
                let _ = self.event_tx.send(ConnectionEvent::Error(e.to_string()));
                Err(e)
            }
        }
    }
    
    /// Check if currently connected
    pub async fn is_connected(&self) -> bool {
        let conn = self.connection.read().await;
        conn.is_connected()
    }
    
    /// Get current connection status
    pub async fn status(&self) -> ConnectionStatus {
        *self.status.read().await
    }
    
    /// Send a command to the controller
    ///
    /// Commands are queued and sent with flow control.
    ///
    /// # Arguments
    /// * `command` - The command to send
    ///
    /// # Returns
    /// * `Ok(())` if command queued successfully
    /// * `Err(Error)` if queueing failed
    pub async fn send_command(&self, command: GrblCommand) -> Result<()> {
        tracing::info!("send_command called with: {:?}", command);
        
        if !self.is_connected().await {
            tracing::error!("send_command: not connected");
            return Err(Error::Connection("Not connected".to_string()));
        }
        
        tracing::info!("send_command: connection OK, enqueueing command");
        let queue = self.queue.write().await;
        let result = queue.enqueue(command).await;
        tracing::info!("send_command: enqueue result: {:?}", result);
        result.map(|_| ())
    }
    
    /// Send a real-time command (immediate, bypasses queue)
    ///
    /// # Arguments
    /// * `byte` - The real-time command byte
    ///
    /// # Returns
    /// * `Ok(())` if command sent successfully
    /// * `Err(Error)` if send failed
    pub async fn send_realtime(&self, byte: u8) -> Result<()> {
        if !self.is_connected().await {
            return Err(Error::Connection("Not connected".to_string()));
        }
        
        let mut conn = self.connection.write().await;
        conn.send_bytes(&[byte]).await
    }
    
    /// Subscribe to status updates
    ///
    /// # Returns
    /// A receiver for status updates
    pub fn subscribe_status(&self) -> broadcast::Receiver<GrblStatus> {
        self.status_tx.subscribe()
    }
    
    /// Subscribe to connection events
    ///
    /// # Returns
    /// A receiver for connection events
    pub fn subscribe_events(&self) -> broadcast::Receiver<ConnectionEvent> {
        self.event_tx.subscribe()
    }
    
    /// Subscribe to responses
    ///
    /// # Returns
    /// A receiver for all GRBL responses
    pub fn subscribe_responses(&self) -> broadcast::Receiver<GrblResponse> {
        self.response_tx.subscribe()
    }
    
    /// Pause the command queue
    pub async fn pause(&self) -> Result<()> {
        let queue = self.queue.write().await;
        queue.pause().await;
        Ok(())
    }
    
    /// Resume the command queue
    pub async fn resume(&self) -> Result<()> {
        let queue = self.queue.write().await;
        queue.resume().await
    }
    
    /// Clear the command queue
    pub async fn clear_queue(&self) -> Result<()> {
        let queue = self.queue.write().await;
        queue.clear().await;
        Ok(())
    }
    
    /// Get queue state
    pub async fn queue_state(&self) -> QueueState {
        let queue = self.queue.read().await;
        queue.state().await
    }
    
    /// Get connection description
    pub async fn description(&self) -> String {
        let conn = self.connection.read().await;
        conn.description()
    }
    
    /// Start background tasks for receiving data and status queries
    async fn start_background_tasks(&mut self) -> Result<()> {
        let (shutdown_tx, _) = broadcast::channel(1);
        self.shutdown_tx = Some(shutdown_tx.clone());
        
        // Task 1: Receive and parse responses
        let connection_recv = Arc::clone(&self.connection);
        let response_tx = self.response_tx.clone();
        let status_tx = self.status_tx.clone();
        let event_tx = self.event_tx.clone();
        let queue_recv = Arc::clone(&self.queue);
        let status_recv = Arc::clone(&self.status);
        let mut shutdown_rx_recv = shutdown_tx.subscribe();
        
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = shutdown_rx_recv.recv() => {
                        break;
                    }
                    result = Self::receive_and_parse(
                        &connection_recv,
                        &response_tx,
                        &status_tx,
                        &event_tx,
                        &queue_recv,
                    ) => {
                        if let Err(e) = result {
                            tracing::error!("Error receiving data: {}", e);
                            *status_recv.write().await = ConnectionStatus::Error;
                            let _ = event_tx.send(ConnectionEvent::Error(e.to_string()));
                            break;
                        }
                    }
                }
            }
        });
        
        // Task 2: Send commands from queue
        let connection_send = Arc::clone(&self.connection);
        let queue_send = Arc::clone(&self.queue);
        let mut shutdown_rx_send = shutdown_tx.subscribe();
        
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = shutdown_rx_send.recv() => {
                        break;
                    }
                    _ = sleep(Duration::from_millis(10)) => {
                        if let Err(e) = Self::process_queue(&connection_send, &queue_send).await {
                            tracing::error!("Error processing queue: {}", e);
                        }
                    }
                }
            }
        });
        
        // Task 3: Periodic status queries (if enabled)
        if self.config.auto_status_query {
            let connection_status = Arc::clone(&self.connection);
            let interval_ms = self.config.status_interval_ms;
            let mut shutdown_rx_status = shutdown_tx.subscribe();
            
            tokio::spawn(async move {
                let mut timer = interval(Duration::from_millis(interval_ms));
                loop {
                    tokio::select! {
                        _ = shutdown_rx_status.recv() => {
                            break;
                        }
                        _ = timer.tick() => {
                            let mut conn = connection_status.write().await;
                            if conn.is_connected() {
                                // Send status query (? is 0x3F)
                                if let Err(e) = conn.send_bytes(&[b'?']).await {
                                    tracing::error!("Error sending status query: {}", e);
                                }
                            }
                        }
                    }
                }
            });
        }
        
        Ok(())
    }
    
    /// Stop background tasks
    async fn stop_background_tasks(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
    
    /// Receive and parse data from connection
    async fn receive_and_parse(
        connection: &Arc<RwLock<Box<dyn Connection>>>,
        response_tx: &broadcast::Sender<GrblResponse>,
        status_tx: &broadcast::Sender<GrblStatus>,
        event_tx: &broadcast::Sender<ConnectionEvent>,
        queue: &Arc<RwLock<CommandQueue>>,
    ) -> Result<()> {
        let mut conn = connection.write().await;
        
        match conn.receive_line(DEFAULT_RESPONSE_TIMEOUT).await? {
            Some(line) => {
                tracing::debug!("Received: {}", line);
                
                // Broadcast raw data event
                let _ = event_tx.send(ConnectionEvent::DataReceived(line.clone()));
                
                // Parse response
                match GrblResponse::parse(&line) {
                    Ok(response) => {
                        // Broadcast parsed response
                        let _ = response_tx.send(response.clone());
                        
                        // Handle specific response types
                        match &response {
                            GrblResponse::Ok => {
                                let q = queue.write().await;
                                q.handle_response(&response).await?;
                            }
                            GrblResponse::Error(_) => {
                                let q = queue.write().await;
                                q.handle_response(&response).await?;
                            }
                            GrblResponse::Alarm(_) => {
                                let q = queue.write().await;
                                q.handle_response(&response).await?;
                            }
                            GrblResponse::Status(status) => {
                                // Broadcast status update
                                let _ = status_tx.send(status.clone());
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to parse response: {} - Error: {}", line, e);
                    }
                }
            }
            None => {
                // Timeout, no data received - this is normal
            }
        }
        
        Ok(())
    }
    
    /// Process the command queue
    async fn process_queue(
        connection: &Arc<RwLock<Box<dyn Connection>>>,
        queue: &Arc<RwLock<CommandQueue>>,
    ) -> Result<()> {
        let q = queue.write().await;
        
        // Check if we can send the next command
        if let Some(command) = q.next_command().await? {
            let command_str = command.to_string();
            tracing::info!("Sending command to GRBL: {}", command_str);
            
            // Send the command
            let mut conn = connection.write().await;
            if !conn.is_connected() {
                tracing::error!("Attempted to send command but connection is not active");
                return Err(Error::Connection("Not connected".to_string()));
            }
            
            conn.send_line(&command_str).await?;
            tracing::info!("Command sent successfully: {}", command_str);
            
            // Mark as sent in queue
            q.mark_sent().await?;
        }
        
        Ok(())
    }
}

impl Drop for ConnectionManager {
    fn drop(&mut self) {
        // Best effort cleanup - spawn a task to shutdown gracefully
        // Note: In a real shutdown, disconnect() should be called explicitly
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::ConnectionStatus;
    use async_trait::async_trait;
    
    /// Mock connection for testing
    struct MockConnection {
        connected: bool,
        send_buffer: Vec<String>,
        receive_buffer: Vec<String>,
    }
    
    impl MockConnection {
        fn new() -> Self {
            Self {
                connected: false,
                send_buffer: Vec::new(),
                receive_buffer: vec!["ok".to_string()],
            }
        }
    }
    
    #[async_trait]
    impl Connection for MockConnection {
        async fn connect(&mut self, _timeout: Duration) -> Result<()> {
            self.connected = true;
            Ok(())
        }
        
        async fn disconnect(&mut self) -> Result<()> {
            self.connected = false;
            Ok(())
        }
        
        fn is_connected(&self) -> bool {
            self.connected
        }
        
        fn status(&self) -> ConnectionStatus {
            if self.connected {
                ConnectionStatus::Connected
            } else {
                ConnectionStatus::Disconnected
            }
        }
        
        async fn send_line(&mut self, data: &str) -> Result<()> {
            self.send_buffer.push(data.to_string());
            Ok(())
        }
        
        async fn send_bytes(&mut self, data: &[u8]) -> Result<()> {
            self.send_buffer.push(String::from_utf8_lossy(data).to_string());
            Ok(())
        }
        
        async fn receive_line(&mut self, _timeout: Duration) -> Result<Option<String>> {
            if !self.receive_buffer.is_empty() {
                Ok(Some(self.receive_buffer.remove(0)))
            } else {
                Ok(None)
            }
        }
        
        fn description(&self) -> String {
            "Mock Connection".to_string()
        }
        
        async fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    #[tokio::test]
    async fn test_manager_creation() {
        let conn = Box::new(MockConnection::new());
        let manager = ConnectionManager::new(conn);
        
        assert_eq!(manager.status().await, ConnectionStatus::Disconnected);
        assert!(!manager.is_connected().await);
    }
    
    #[tokio::test]
    async fn test_manager_connect() {
        let conn = Box::new(MockConnection::new());
        let mut manager = ConnectionManager::new(conn);
        
        manager.connect(Duration::from_secs(5)).await.unwrap();
        assert!(manager.is_connected().await);
        assert_eq!(manager.status().await, ConnectionStatus::Connected);
    }
    
    #[tokio::test]
    async fn test_manager_disconnect() {
        let conn = Box::new(MockConnection::new());
        let mut manager = ConnectionManager::new(conn);
        
        manager.connect(Duration::from_secs(5)).await.unwrap();
        manager.disconnect().await.unwrap();
        
        assert!(!manager.is_connected().await);
        assert_eq!(manager.status().await, ConnectionStatus::Disconnected);
    }
    
    #[tokio::test]
    async fn test_manager_description() {
        let conn = Box::new(MockConnection::new());
        let manager = ConnectionManager::new(conn);
        
        let desc = manager.description().await;
        assert_eq!(desc, "Mock Connection");
    }
    
    #[tokio::test]
    async fn test_manager_status_subscription() {
        let conn = Box::new(MockConnection::new());
        let manager = ConnectionManager::new(conn);
        
        let mut rx = manager.subscribe_status();
        
        // Should not block - just checking it compiles
        drop(rx);
    }
    
    #[tokio::test]
    async fn test_manager_event_subscription() {
        let conn = Box::new(MockConnection::new());
        let manager = ConnectionManager::new(conn);
        
        let mut rx = manager.subscribe_events();
        
        // Should not block - just checking it compiles
        drop(rx);
    }
    
    #[tokio::test]
    async fn test_manager_config() {
        let config = ConnectionManagerConfig {
            status_interval_ms: 100,
            response_timeout: Duration::from_secs(2),
            reconnect_attempts: 5,
            reconnect_delay: Duration::from_secs(3),
            auto_status_query: false,
        };
        
        let conn = Box::new(MockConnection::new());
        let manager = ConnectionManager::with_config(conn, config.clone());
        
        assert_eq!(manager.config.status_interval_ms, 100);
        assert_eq!(manager.config.reconnect_attempts, 5);
        assert_eq!(manager.config.auto_status_query, false);
    }
}
