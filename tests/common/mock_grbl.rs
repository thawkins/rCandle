//! Mock GRBL simulator for testing
//!
//! This module provides a mock GRBL controller that can be used for testing
//! connection and protocol handling without requiring physical hardware.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::sleep;

/// Mock GRBL state
#[derive(Debug, Clone)]
pub struct MockGrblState {
    /// Current machine position (MPos)
    pub machine_pos: (f64, f64, f64),
    /// Current work position (WPos)
    pub work_pos: (f64, f64, f64),
    /// Work coordinate offset (WCO)
    pub wco: (f64, f64, f64),
    /// Current machine state
    pub state: String,
    /// Feed rate
    pub feed_rate: f64,
    /// Spindle speed
    pub spindle_speed: f64,
    /// Buffer state (planner blocks, RX bytes)
    pub buffer: (u8, u16),
}

impl Default for MockGrblState {
    fn default() -> Self {
        Self {
            machine_pos: (0.0, 0.0, 0.0),
            work_pos: (0.0, 0.0, 0.0),
            wco: (0.0, 0.0, 0.0),
            state: "Idle".to_string(),
            feed_rate: 0.0,
            spindle_speed: 0.0,
            buffer: (15, 128),
        }
    }
}

/// Mock GRBL simulator
pub struct MockGrbl {
    state: Arc<Mutex<MockGrblState>>,
    commands: Arc<Mutex<VecDeque<String>>>,
}

impl MockGrbl {
    /// Create a new mock GRBL simulator
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(MockGrblState::default())),
            commands: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Get the current state
    pub fn state(&self) -> MockGrblState {
        self.state.lock().unwrap().clone()
    }

    /// Set the machine state
    pub fn set_state(&self, state: &str) {
        self.state.lock().unwrap().state = state.to_string();
    }

    /// Get received commands
    pub fn get_commands(&self) -> Vec<String> {
        self.commands.lock().unwrap().iter().cloned().collect()
    }

    /// Clear received commands
    pub fn clear_commands(&self) {
        self.commands.lock().unwrap().clear();
    }

    /// Start a TCP server that simulates GRBL
    pub async fn start_tcp_server(self: Arc<Self>, port: u16) -> tokio::io::Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        
        loop {
            let (stream, _) = listener.accept().await?;
            let grbl = Arc::clone(&self);
            
            tokio::spawn(async move {
                if let Err(e) = grbl.handle_client(stream).await {
                    eprintln!("Error handling client: {}", e);
                }
            });
        }
    }

    /// Handle a client connection
    async fn handle_client(&self, stream: TcpStream) -> tokio::io::Result<()> {
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        
        // Send welcome message
        writer.write_all(b"Grbl 1.1f ['$' for help]\r\n").await?;
        
        let mut line = String::new();
        loop {
            line.clear();
            let n = reader.read_line(&mut line).await?;
            if n == 0 {
                break; // Connection closed
            }
            
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            
            // Store command
            self.commands.lock().unwrap().push_back(trimmed.to_string());
            
            // Process command and send response
            let response = self.process_command(trimmed).await;
            writer.write_all(response.as_bytes()).await?;
            writer.flush().await?;
        }
        
        Ok(())
    }

    /// Process a command and generate response
    async fn process_command(&self, command: &str) -> String {
        // Simulate processing delay
        sleep(Duration::from_millis(10)).await;
        
        match command.chars().next() {
            Some('?') => {
                // Status query
                let state = self.state.lock().unwrap();
                format!(
                    "<{}|MPos:{:.3},{:.3},{:.3}|FS:{:.0},{:.0}|Bf:{},{}|WCO:{:.3},{:.3},{:.3}>\r\n",
                    state.state,
                    state.machine_pos.0, state.machine_pos.1, state.machine_pos.2,
                    state.feed_rate, state.spindle_speed,
                    state.buffer.0, state.buffer.1,
                    state.wco.0, state.wco.1, state.wco.2
                )
            }
            Some('$') => {
                if command == "$$" {
                    // Settings query
                    "$0=10\r\n$1=25\r\n$2=0\r\n$3=0\r\n$4=0\r\n$5=0\r\n".to_string()
                } else if command == "$#" {
                    // Coordinate systems query
                    "[G54:0.000,0.000,0.000]\r\n[G55:0.000,0.000,0.000]\r\n".to_string()
                } else if command == "$G" {
                    // Parser state query
                    "[GC:G0 G54 G17 G21 G90 G94 M5 M9 T0 F0 S0]\r\nok\r\n".to_string()
                } else if command == "$I" {
                    // Build info query
                    "[VER:1.1f.20170801:Mock GRBL]\r\n[OPT:V,15,128]\r\nok\r\n".to_string()
                } else if command.starts_with("$H") {
                    // Homing
                    self.state.lock().unwrap().state = "Home".to_string();
                    sleep(Duration::from_millis(100)).await;
                    self.state.lock().unwrap().state = "Idle".to_string();
                    "ok\r\n".to_string()
                } else if command.starts_with("$X") {
                    // Unlock
                    self.state.lock().unwrap().state = "Idle".to_string();
                    "ok\r\n".to_string()
                } else {
                    "ok\r\n".to_string()
                }
            }
            Some('~') => {
                // Cycle start
                if self.state.lock().unwrap().state == "Hold" {
                    self.state.lock().unwrap().state = "Run".to_string();
                }
                String::new() // Real-time commands don't send responses
            }
            Some('!') => {
                // Feed hold
                if self.state.lock().unwrap().state == "Run" {
                    self.state.lock().unwrap().state = "Hold".to_string();
                }
                String::new() // Real-time commands don't send responses
            }
            Some('G') | Some('M') | Some('X') | Some('Y') | Some('Z') | Some('F') | Some('S') => {
                // G-code command
                // Simulate motion
                if command.starts_with("G0") || command.starts_with("G1") {
                    self.state.lock().unwrap().state = "Run".to_string();
                    sleep(Duration::from_millis(50)).await;
                    self.state.lock().unwrap().state = "Idle".to_string();
                }
                "ok\r\n".to_string()
            }
            _ => {
                // Unknown command
                "error:1\r\n".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_grbl_creation() {
        let grbl = MockGrbl::new();
        let state = grbl.state();
        assert_eq!(state.state, "Idle");
        assert_eq!(state.machine_pos, (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_mock_grbl_state_modification() {
        let grbl = MockGrbl::new();
        grbl.set_state("Run");
        let state = grbl.state();
        assert_eq!(state.state, "Run");
    }

    #[test]
    fn test_mock_grbl_commands() {
        let grbl = MockGrbl::new();
        {
            let mut cmds = grbl.commands.lock().unwrap();
            cmds.push_back("G0 X10".to_string());
            cmds.push_back("G1 Y20 F100".to_string());
        }
        
        let commands = grbl.get_commands();
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0], "G0 X10");
        assert_eq!(commands[1], "G1 Y20 F100");
        
        grbl.clear_commands();
        let commands = grbl.get_commands();
        assert_eq!(commands.len(), 0);
    }

    #[tokio::test]
    async fn test_process_status_query() {
        let grbl = MockGrbl::new();
        let response = grbl.process_command("?").await;
        assert!(response.starts_with("<Idle|"));
        assert!(response.contains("MPos:"));
        assert!(response.contains("FS:"));
    }

    #[tokio::test]
    async fn test_process_gcode() {
        let grbl = MockGrbl::new();
        let response = grbl.process_command("G0 X10").await;
        assert_eq!(response, "ok\r\n");
    }

    #[tokio::test]
    async fn test_process_settings_query() {
        let grbl = MockGrbl::new();
        let response = grbl.process_command("$$").await;
        assert!(response.contains("$0="));
        assert!(response.contains("$1="));
    }
}
