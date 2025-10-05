//! Script API
//!
//! Provides the scripting interface to application functionality.

use crate::state::AppState;
use tokio::sync::mpsc;

/// Commands that can be sent from scripts
#[derive(Debug, Clone)]
pub enum ScriptCommand {
    /// Send a raw GRBL command
    SendCommand(String),
    /// Jog in a specific direction
    Jog { axis: String, distance: f64 },
    /// Home the machine
    Home,
    /// Zero an axis
    ZeroAxis(String),
    /// Start program execution
    StartProgram,
    /// Pause program execution
    PauseProgram,
    /// Stop program execution
    StopProgram,
    /// Log a message
    Log(String),
}

/// Script API for accessing application functionality
pub struct ScriptApi {
    state: AppState,
    command_tx: mpsc::UnboundedSender<ScriptCommand>,
}

impl ScriptApi {
    /// Create a new script API
    pub fn new(state: AppState, command_tx: mpsc::UnboundedSender<ScriptCommand>) -> Self {
        Self { state, command_tx }
    }
    
    /// Send a command to GRBL
    pub fn send_command(&self, command: String) -> bool {
        self.command_tx.send(ScriptCommand::SendCommand(command)).is_ok()
    }
    
    /// Jog the machine
    pub fn jog(&self, axis: String, distance: f64) -> bool {
        self.command_tx.send(ScriptCommand::Jog { axis, distance }).is_ok()
    }
    
    /// Home the machine
    pub fn home(&self) -> bool {
        self.command_tx.send(ScriptCommand::Home).is_ok()
    }
    
    /// Zero an axis
    pub fn zero_axis(&self, axis: String) -> bool {
        self.command_tx.send(ScriptCommand::ZeroAxis(axis)).is_ok()
    }
    
    /// Get current position for an axis
    pub fn get_position(&self, axis: String) -> f64 {
        let machine = self.state.machine.read();
        match axis.to_uppercase().as_str() {
            "X" => machine.work_position.x,
            "Y" => machine.work_position.y,
            "Z" => machine.work_position.z,
            _ => 0.0,
        }
    }
    
    /// Check if connected to machine
    pub fn is_connected(&self) -> bool {
        self.state.is_connected()
    }
    
    /// Get current machine state as a string
    pub fn get_state(&self) -> String {
        let machine = self.state.machine.read();
        format!("{:?}", machine.status)
    }
    
    /// Start program execution
    pub fn start_program(&self) -> bool {
        self.command_tx.send(ScriptCommand::StartProgram).is_ok()
    }
    
    /// Pause program execution
    pub fn pause_program(&self) -> bool {
        self.command_tx.send(ScriptCommand::PauseProgram).is_ok()
    }
    
    /// Stop program execution
    pub fn stop_program(&self) -> bool {
        self.command_tx.send(ScriptCommand::StopProgram).is_ok()
    }
    
    /// Log a message
    pub fn log(&self, message: String) -> bool {
        self.command_tx.send(ScriptCommand::Log(message)).is_ok()
    }
    
    /// Sleep for a duration (in milliseconds)
    pub fn sleep(&self, ms: u64) {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
}
