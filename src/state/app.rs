//! Application state management

use super::{MachineState, ProgramState, SharedState};

/// Complete application state
#[derive(Clone)]
pub struct AppState {
    /// Machine state
    pub machine: SharedState<MachineState>,
    
    /// Program state
    pub program: SharedState<ProgramState>,
    
    /// Connection state
    pub connected: SharedState<bool>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            machine: SharedState::new(MachineState::default()),
            program: SharedState::new(ProgramState::default()),
            connected: SharedState::new(false),
        }
    }
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if connected to machine
    pub fn is_connected(&self) -> bool {
        *self.connected.read()
    }

    /// Set connection status
    pub fn set_connected(&self, connected: bool) {
        *self.connected.write() = connected;
    }
}
