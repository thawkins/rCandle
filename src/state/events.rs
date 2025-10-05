//! State change events and notifications
//!
//! This module provides a pub/sub pattern for state changes.

use super::{MachineStatus, Position, CoordinateSystem, ExecutionState};
use tokio::sync::broadcast;

/// State change events
#[derive(Debug, Clone)]
pub enum StateEvent {
    /// Machine status changed
    MachineStatusChanged {
        /// Previous status
        old: MachineStatus,
        /// New status
        new: MachineStatus,
    },
    
    /// Machine position updated
    MachinePositionChanged {
        /// Machine position
        machine_pos: Position,
        /// Work position
        work_pos: Position,
    },
    
    /// Spindle state changed
    SpindleStateChanged {
        /// Spindle enabled
        enabled: bool,
        /// Spindle speed (RPM)
        speed: f64,
    },
    
    /// Feed rate changed
    FeedRateChanged {
        /// Feed rate (mm/min or in/min)
        feed_rate: f64,
    },
    
    /// Override values changed
    OverridesChanged {
        /// Feed override percentage (0-200)
        feed: f64,
        /// Rapid override percentage (25-100)
        rapid: f64,
        /// Spindle override percentage (0-200)
        spindle: f64,
    },
    
    /// Work offset changed
    WorkOffsetChanged {
        /// Coordinate system
        system: CoordinateSystem,
        /// Offset
        offset: Position,
    },
    
    /// Active coordinate system changed
    CoordinateSystemChanged {
        /// Old coordinate system
        old: CoordinateSystem,
        /// New coordinate system
        new: CoordinateSystem,
    },
    
    /// Program state changed
    ProgramStateChanged {
        /// Old state
        old: ExecutionState,
        /// New state
        new: ExecutionState,
    },
    
    /// Program progress updated
    ProgramProgressChanged {
        /// Current line
        current_line: usize,
        /// Total lines
        total_lines: usize,
        /// Progress percentage (0.0-1.0)
        progress: f64,
    },
    
    /// Error occurred
    ErrorOccurred {
        /// Error message
        message: String,
    },
    
    /// Connection status changed
    ConnectionChanged {
        /// Connected
        connected: bool,
    },
}

/// State event broadcaster
#[derive(Clone)]
pub struct StateEventBroadcaster {
    sender: broadcast::Sender<StateEvent>,
}

impl StateEventBroadcaster {
    /// Create a new state event broadcaster
    ///
    /// # Arguments
    /// * `capacity` - Maximum number of events to buffer (typically 100)
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        StateEventBroadcaster { sender }
    }
    
    /// Subscribe to state events
    pub fn subscribe(&self) -> broadcast::Receiver<StateEvent> {
        self.sender.subscribe()
    }
    
    /// Send a state event
    pub fn send(&self, event: StateEvent) {
        // Ignore send errors - it's OK if there are no receivers
        let _ = self.sender.send(event);
    }
    
    /// Get the number of active subscribers
    pub fn receiver_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for StateEventBroadcaster {
    fn default() -> Self {
        Self::new(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_broadcaster_creation() {
        let broadcaster = StateEventBroadcaster::new(10);
        assert_eq!(broadcaster.receiver_count(), 0);
    }
    
    #[tokio::test]
    async fn test_subscribe() {
        let broadcaster = StateEventBroadcaster::new(10);
        let _receiver = broadcaster.subscribe();
        assert_eq!(broadcaster.receiver_count(), 1);
    }
    
    #[tokio::test]
    async fn test_send_receive() {
        let broadcaster = StateEventBroadcaster::new(10);
        let mut receiver = broadcaster.subscribe();
        
        broadcaster.send(StateEvent::MachineStatusChanged {
            old: MachineStatus::Idle,
            new: MachineStatus::Run,
        });
        
        let event = receiver.recv().await.unwrap();
        match event {
            StateEvent::MachineStatusChanged { old, new } => {
                assert_eq!(old, MachineStatus::Idle);
                assert_eq!(new, MachineStatus::Run);
            }
            _ => panic!("Wrong event type"),
        }
    }
    
    #[tokio::test]
    async fn test_multiple_subscribers() {
        let broadcaster = StateEventBroadcaster::new(10);
        let mut receiver1 = broadcaster.subscribe();
        let mut receiver2 = broadcaster.subscribe();
        
        assert_eq!(broadcaster.receiver_count(), 2);
        
        broadcaster.send(StateEvent::ConnectionChanged { connected: true });
        
        let event1 = receiver1.recv().await.unwrap();
        let event2 = receiver2.recv().await.unwrap();
        
        match (event1, event2) {
            (
                StateEvent::ConnectionChanged { connected: c1 },
                StateEvent::ConnectionChanged { connected: c2 },
            ) => {
                assert!(c1);
                assert!(c2);
            }
            _ => panic!("Wrong event types"),
        }
    }
    
    #[tokio::test]
    async fn test_no_panic_without_receivers() {
        let broadcaster = StateEventBroadcaster::new(10);
        // Should not panic even without receivers
        broadcaster.send(StateEvent::ConnectionChanged { connected: true });
    }
}
