/// Command queue for managing GRBL command flow
///
/// This module implements a command queue that:
/// - Maintains a bounded queue of commands
/// - Handles command acknowledgments from GRBL
/// - Implements flow control (wait for "ok" before sending next command)
/// - Tracks command timeouts
/// - Supports priority for real-time commands

use crate::grbl::commands::GrblCommand;
use crate::grbl::responses::GrblResponse;
use crate::utils::error::{Error, Result};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, Mutex};

/// Default queue capacity
const DEFAULT_QUEUE_CAPACITY: usize = 128;

/// Default command timeout
const DEFAULT_COMMAND_TIMEOUT: Duration = Duration::from_secs(30);

/// Command queue entry
#[derive(Debug, Clone)]
struct QueuedCommand {
    /// The command to send
    command: GrblCommand,
    /// When the command was queued
    #[allow(dead_code)] // Used for metrics and debugging
    queued_at: Instant,
    /// When the command was sent (if sent)
    sent_at: Option<Instant>,
    /// Unique command ID for tracking
    #[allow(dead_code)] // Used for command tracking
    id: u64,
}

/// Command queue state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueState {
    /// Queue is idle and ready to accept commands
    Idle,
    /// Queue is actively processing commands
    Active,
    /// Queue is paused (no commands will be sent)
    Paused,
    /// Queue is waiting for acknowledgment
    WaitingForAck,
}

/// Command queue statistics
#[derive(Debug, Clone, Default)]
pub struct QueueStats {
    /// Total commands queued
    pub total_queued: u64,
    /// Total commands sent
    pub total_sent: u64,
    /// Total commands completed (acknowledged)
    pub total_completed: u64,
    /// Total commands timed out
    pub total_timeouts: u64,
    /// Total commands failed
    pub total_failed: u64,
    /// Current queue length
    pub current_length: usize,
    /// Average command execution time (milliseconds)
    pub avg_execution_time_ms: f64,
}

/// Command queue for managing GRBL commands
pub struct CommandQueue {
    /// Queue of pending commands
    queue: Arc<Mutex<VecDeque<QueuedCommand>>>,
    /// Current command being executed
    current_command: Arc<Mutex<Option<QueuedCommand>>>,
    /// Queue state
    state: Arc<Mutex<QueueState>>,
    /// Maximum queue capacity
    capacity: usize,
    /// Command timeout duration
    timeout: Duration,
    /// Next command ID
    next_id: Arc<Mutex<u64>>,
    /// Queue statistics
    stats: Arc<Mutex<QueueStats>>,
    /// Channel for sending commands to connection
    command_tx: Option<mpsc::UnboundedSender<GrblCommand>>,
}

impl CommandQueue {
    /// Create a new command queue with default capacity
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_QUEUE_CAPACITY)
    }

    /// Create a new command queue with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::with_capacity(capacity))),
            current_command: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(QueueState::Idle)),
            capacity,
            timeout: DEFAULT_COMMAND_TIMEOUT,
            next_id: Arc::new(Mutex::new(0)),
            stats: Arc::new(Mutex::new(QueueStats::default())),
            command_tx: None,
        }
    }

    /// Set the command timeout duration
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    /// Set the command sender channel
    pub fn set_command_sender(&mut self, tx: mpsc::UnboundedSender<GrblCommand>) {
        self.command_tx = Some(tx);
    }

    /// Add a command to the queue
    pub async fn enqueue(&self, command: GrblCommand) -> Result<u64> {
        tracing::info!("Queue: enqueue called with command: {:?}", command);
        let mut queue = self.queue.lock().await;
        
        // Check if queue is full
        if queue.len() >= self.capacity {
            tracing::error!("Queue: queue is full (capacity: {})", self.capacity);
            return Err(Error::Queue("Command queue is full".to_string()));
        }

        // Get next command ID
        let mut next_id = self.next_id.lock().await;
        let id = *next_id;
        *next_id += 1;
        drop(next_id);

        // Create queued command
        let queued_cmd = QueuedCommand {
            command,
            queued_at: Instant::now(),
            sent_at: None,
            id,
        };

        queue.push_back(queued_cmd);
        tracing::info!("Queue: command {} added, queue length now: {}", id, queue.len());

        // Update statistics
        let mut stats = self.stats.lock().await;
        stats.total_queued += 1;
        stats.current_length = queue.len();
        drop(stats);

        // Try to send next command if we're idle
        drop(queue);
        tracing::info!("Queue: attempting to send next command");
        self.try_send_next().await?;

        Ok(id)
    }

    /// Handle a response from GRBL
    pub async fn handle_response(&self, response: &GrblResponse) -> Result<()> {
        match response {
            GrblResponse::Ok => {
                self.handle_ok().await?;
            }
            GrblResponse::Error(code) => {
                self.handle_error(*code).await?;
            }
            GrblResponse::Alarm(code) => {
                self.handle_alarm(*code).await?;
            }
            _ => {
                // Other responses don't affect command flow
            }
        }
        Ok(())
    }

    /// Handle OK response (command completed successfully)
    async fn handle_ok(&self) -> Result<()> {
        let mut current = self.current_command.lock().await;
        
        if let Some(cmd) = current.take() {
            // Calculate execution time
            if let Some(sent_at) = cmd.sent_at {
                let execution_time = sent_at.elapsed();
                
                // Update statistics
                let mut stats = self.stats.lock().await;
                stats.total_completed += 1;
                
                // Update average execution time
                let total = stats.total_completed as f64;
                let old_avg = stats.avg_execution_time_ms;
                let new_time = execution_time.as_secs_f64() * 1000.0;
                stats.avg_execution_time_ms = (old_avg * (total - 1.0) + new_time) / total;
            }
        }

        // Set state back to idle
        let mut state = self.state.lock().await;
        *state = QueueState::Idle;
        drop(state);

        // Try to send next command
        self.try_send_next().await?;

        Ok(())
    }

    /// Handle error response
    async fn handle_error(&self, _code: u8) -> Result<()> {
        let mut current = self.current_command.lock().await;
        current.take(); // Remove failed command

        // Update statistics
        let mut stats = self.stats.lock().await;
        stats.total_failed += 1;
        drop(stats);

        // Set state back to idle
        let mut state = self.state.lock().await;
        *state = QueueState::Idle;
        drop(state);

        // Try to send next command
        self.try_send_next().await?;

        Ok(())
    }

    /// Handle alarm response
    async fn handle_alarm(&self, _code: u8) -> Result<()> {
        // On alarm, pause the queue
        let mut state = self.state.lock().await;
        *state = QueueState::Paused;
        drop(state);

        // Clear current command
        let mut current = self.current_command.lock().await;
        current.take();

        Ok(())
    }

    /// Try to send the next command in the queue
    async fn try_send_next(&self) -> Result<()> {
        // Check if we can send
        let state = self.state.lock().await;
        if *state != QueueState::Idle {
            return Ok(());
        }
        drop(state);

        // Check if there's a command sender
        let command_tx = match &self.command_tx {
            Some(tx) => tx.clone(),
            None => return Ok(()), // No sender yet
        };

        // Get next command
        let mut queue = self.queue.lock().await;
        let mut cmd = match queue.pop_front() {
            Some(cmd) => cmd,
            None => return Ok(()), // Queue is empty
        };

        // Update statistics
        let mut stats = self.stats.lock().await;
        stats.current_length = queue.len();
        stats.total_sent += 1;
        drop(stats);
        drop(queue);

        // Mark as sent
        cmd.sent_at = Some(Instant::now());

        // Send the command
        command_tx
            .send(cmd.command.clone())
            .map_err(|e| Error::Connection(format!("Failed to send command: {}", e)))?;

        // Set current command
        let mut current = self.current_command.lock().await;
        *current = Some(cmd);

        // Update state
        let mut state = self.state.lock().await;
        *state = QueueState::WaitingForAck;

        Ok(())
    }

    /// Check for timed-out commands
    pub async fn check_timeouts(&self) -> Result<()> {
        let mut current = self.current_command.lock().await;
        
        if let Some(cmd) = current.as_ref() {
            if let Some(sent_at) = cmd.sent_at {
                if sent_at.elapsed() > self.timeout {
                    // Command timed out
                    current.take();
                    
                    // Update statistics
                    let mut stats = self.stats.lock().await;
                    stats.total_timeouts += 1;
                    drop(stats);

                    // Set state back to idle
                    let mut state = self.state.lock().await;
                    *state = QueueState::Idle;
                    drop(state);
                    drop(current);

                    // Try to send next command
                    self.try_send_next().await?;
                    
                    return Err(Error::Timeout("Command execution timed out".to_string()));
                }
            }
        }

        Ok(())
    }

    /// Pause the queue (stop sending new commands)
    pub async fn pause(&self) {
        let mut state = self.state.lock().await;
        *state = QueueState::Paused;
    }

    /// Resume the queue (start sending commands again)
    pub async fn resume(&self) -> Result<()> {
        let mut state = self.state.lock().await;
        *state = QueueState::Idle;
        drop(state);
        
        // Try to send next command
        self.try_send_next().await?;
        
        Ok(())
    }

    /// Clear all queued commands
    pub async fn clear(&self) {
        let mut queue = self.queue.lock().await;
        queue.clear();
        
        // Update statistics
        let mut stats = self.stats.lock().await;
        stats.current_length = 0;
    }

    /// Get the current queue state
    pub async fn get_state(&self) -> QueueState {
        *self.state.lock().await
    }
    
    /// Alias for get_state for consistency with other APIs
    pub async fn state(&self) -> QueueState {
        self.get_state().await
    }
    
    /// Get the next command to send (if ready)
    ///
    /// Returns None if queue is empty, paused, or waiting for acknowledgment
    pub async fn next_command(&self) -> Result<Option<GrblCommand>> {
        // Check if we can send
        let state = self.state.lock().await;
        tracing::debug!("Queue: next_command called, current state: {:?}", *state);
        if *state != QueueState::Idle {
            tracing::debug!("Queue: not in Idle state, returning None");
            return Ok(None);
        }
        drop(state);
        
        // Get next command
        let queue = self.queue.lock().await;
        let cmd = queue.front().map(|cmd| cmd.command.clone());
        if let Some(ref c) = cmd {
            tracing::info!("Queue: next_command returning: {:?}", c);
        } else {
            tracing::debug!("Queue: queue is empty");
        }
        Ok(cmd)
    }
    
    /// Mark the current command as sent
    ///
    /// Should be called after successfully sending the command returned by next_command
    pub async fn mark_sent(&self) -> Result<()> {
        // Get next command
        let mut queue = self.queue.lock().await;
        let mut cmd = match queue.pop_front() {
            Some(cmd) => cmd,
            None => return Err(Error::Queue("No command to mark as sent".to_string())),
        };
        
        // Update statistics
        let mut stats = self.stats.lock().await;
        stats.current_length = queue.len();
        stats.total_sent += 1;
        drop(stats);
        drop(queue);
        
        // Mark as sent
        cmd.sent_at = Some(Instant::now());
        
        // Set current command
        let mut current = self.current_command.lock().await;
        *current = Some(cmd);
        
        // Update state
        let mut state = self.state.lock().await;
        *state = QueueState::WaitingForAck;
        
        Ok(())
    }

    /// Get the current queue length
    pub async fn len(&self) -> usize {
        self.queue.lock().await.len()
    }

    /// Check if the queue is empty
    pub async fn is_empty(&self) -> bool {
        self.queue.lock().await.is_empty()
    }

    /// Get queue statistics
    pub async fn get_stats(&self) -> QueueStats {
        self.stats.lock().await.clone()
    }

    /// Reset queue statistics
    pub async fn reset_stats(&self) {
        let mut stats = self.stats.lock().await;
        *stats = QueueStats::default();
        stats.current_length = self.queue.lock().await.len();
    }
}

impl Default for CommandQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_queue_creation() {
        let queue = CommandQueue::new();
        assert_eq!(queue.get_state().await, QueueState::Idle);
        assert_eq!(queue.len().await, 0);
        assert!(queue.is_empty().await);
    }

    #[tokio::test]
    async fn test_enqueue_command() {
        let queue = CommandQueue::new();
        let cmd = GrblCommand::GCode("G0 X10".to_string());
        
        let result = queue.enqueue(cmd).await;
        assert!(result.is_ok());
        
        let stats = queue.get_stats().await;
        assert_eq!(stats.total_queued, 1);
    }

    #[tokio::test]
    async fn test_queue_capacity() {
        let queue = CommandQueue::with_capacity(2);
        let cmd = GrblCommand::GCode("G0 X10".to_string());
        
        // Should succeed for first two
        assert!(queue.enqueue(cmd.clone()).await.is_ok());
        assert!(queue.enqueue(cmd.clone()).await.is_ok());
        
        // Third should fail (queue full)
        let result = queue.enqueue(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_handle_ok_response() {
        let (tx, _rx) = mpsc::unbounded_channel();
        let mut queue = CommandQueue::new();
        queue.set_command_sender(tx);
        
        let cmd = GrblCommand::GCode("G0 X10".to_string());
        queue.enqueue(cmd).await.unwrap();
        
        // Simulate sending by manually setting current command
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        // Handle OK response
        let response = GrblResponse::Ok;
        let result = queue.handle_response(&response).await;
        assert!(result.is_ok());
        
        let stats = queue.get_stats().await;
        assert_eq!(stats.total_completed, 1);
    }

    #[tokio::test]
    async fn test_pause_resume() {
        let queue = CommandQueue::new();
        
        queue.pause().await;
        assert_eq!(queue.get_state().await, QueueState::Paused);
        
        queue.resume().await.unwrap();
        assert_eq!(queue.get_state().await, QueueState::Idle);
    }

    #[tokio::test]
    async fn test_clear_queue() {
        let queue = CommandQueue::new();
        let cmd = GrblCommand::GCode("G0 X10".to_string());
        
        queue.enqueue(cmd.clone()).await.unwrap();
        queue.enqueue(cmd).await.unwrap();
        
        assert_eq!(queue.len().await, 2);
        
        queue.clear().await;
        assert_eq!(queue.len().await, 0);
    }

    #[tokio::test]
    async fn test_queue_stats() {
        let queue = CommandQueue::new();
        let stats = queue.get_stats().await;
        
        assert_eq!(stats.total_queued, 0);
        assert_eq!(stats.total_sent, 0);
        assert_eq!(stats.total_completed, 0);
        assert_eq!(stats.current_length, 0);
    }

    #[tokio::test]
    async fn test_reset_stats() {
        let (tx, _rx) = mpsc::unbounded_channel();
        let mut queue = CommandQueue::new();
        queue.set_command_sender(tx);
        
        let cmd = GrblCommand::GCode("G0 X10".to_string());
        queue.enqueue(cmd).await.unwrap();
        
        let stats = queue.get_stats().await;
        assert_eq!(stats.total_queued, 1);
        
        queue.reset_stats().await;
        let stats = queue.get_stats().await;
        assert_eq!(stats.total_queued, 0);
    }

    #[tokio::test]
    async fn test_handle_error_response() {
        let (tx, _rx) = mpsc::unbounded_channel();
        let mut queue = CommandQueue::new();
        queue.set_command_sender(tx);
        
        let cmd = GrblCommand::GCode("G0 X10".to_string());
        queue.enqueue(cmd).await.unwrap();
        
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        // Handle error response
        let response = GrblResponse::Error(1);
        queue.handle_response(&response).await.unwrap();
        
        let stats = queue.get_stats().await;
        assert_eq!(stats.total_failed, 1);
    }

    #[tokio::test]
    async fn test_handle_alarm_response() {
        let (tx, _rx) = mpsc::unbounded_channel();
        let mut queue = CommandQueue::new();
        queue.set_command_sender(tx);
        
        let cmd = GrblCommand::GCode("G0 X10".to_string());
        queue.enqueue(cmd).await.unwrap();
        
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        // Handle alarm response - should pause queue
        let response = GrblResponse::Alarm(1);
        queue.handle_response(&response).await.unwrap();
        
        assert_eq!(queue.get_state().await, QueueState::Paused);
    }
}
