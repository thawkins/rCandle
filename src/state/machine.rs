//! Machine state tracking

use serde::{Deserialize, Serialize};
use glam::Vec3;

/// Machine status from GRBL
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MachineStatus {
    /// Machine is idle
    Idle,
    /// Machine is running a program
    Run,
    /// Machine is in hold state
    Hold,
    /// Machine is jogging
    Jog,
    /// Machine is in alarm state
    Alarm,
    /// Machine is in door state (safety door open)
    Door,
    /// Machine is checking G-code
    Check,
    /// Machine is homing
    Home,
    /// Machine is in sleep mode
    Sleep,
    /// Unknown status
    Unknown,
}

impl Default for MachineStatus {
    fn default() -> Self {
        MachineStatus::Unknown
    }
}

impl std::fmt::Display for MachineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MachineStatus::Idle => write!(f, "Idle"),
            MachineStatus::Run => write!(f, "Run"),
            MachineStatus::Hold => write!(f, "Hold"),
            MachineStatus::Jog => write!(f, "Jog"),
            MachineStatus::Alarm => write!(f, "Alarm"),
            MachineStatus::Door => write!(f, "Door"),
            MachineStatus::Check => write!(f, "Check"),
            MachineStatus::Home => write!(f, "Home"),
            MachineStatus::Sleep => write!(f, "Sleep"),
            MachineStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Position in 3D space
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
}

impl Position {
    /// Create a new position
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Position { x, y, z }
    }

    /// Zero position
    pub fn zero() -> Self {
        Position { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Convert to Vec3
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    /// Create from Vec3
    pub fn from_vec3(v: Vec3) -> Self {
        Position {
            x: v.x as f64,
            y: v.y as f64,
            z: v.z as f64,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::zero()
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "X:{:.3} Y:{:.3} Z:{:.3}", self.x, self.y, self.z)
    }
}

/// Work coordinate system offset (G54-G59)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoordinateSystem {
    /// G54
    G54,
    /// G55
    G55,
    /// G56
    G56,
    /// G57
    G57,
    /// G58
    G58,
    /// G59
    G59,
}

impl Default for CoordinateSystem {
    fn default() -> Self {
        CoordinateSystem::G54
    }
}

impl std::fmt::Display for CoordinateSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoordinateSystem::G54 => write!(f, "G54"),
            CoordinateSystem::G55 => write!(f, "G55"),
            CoordinateSystem::G56 => write!(f, "G56"),
            CoordinateSystem::G57 => write!(f, "G57"),
            CoordinateSystem::G58 => write!(f, "G58"),
            CoordinateSystem::G59 => write!(f, "G59"),
        }
    }
}

/// Complete machine state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineState {
    /// Current machine status
    pub status: MachineStatus,
    
    /// Machine position (in machine coordinates)
    pub machine_position: Position,
    
    /// Work position (in work coordinates)
    pub work_position: Position,
    
    /// Active coordinate system
    pub coordinate_system: CoordinateSystem,
    
    /// Work coordinate offsets for each system
    pub work_offsets: [Position; 6],
    
    /// Spindle speed (RPM)
    pub spindle_speed: f64,
    
    /// Spindle enabled
    pub spindle_enabled: bool,
    
    /// Feed rate (mm/min or in/min)
    pub feed_rate: f64,
    
    /// Feed rate override (percentage, 0-200)
    pub feed_override: f64,
    
    /// Spindle speed override (percentage, 0-200)
    pub spindle_override: f64,
    
    /// Rapid override (percentage, 25, 50, or 100)
    pub rapid_override: f64,
    
    /// Buffer state (number of blocks in planner buffer)
    pub buffer_state: u32,
    
    /// Last error message
    pub last_error: Option<String>,
}

impl Default for MachineState {
    fn default() -> Self {
        MachineState {
            status: MachineStatus::default(),
            machine_position: Position::default(),
            work_position: Position::default(),
            coordinate_system: CoordinateSystem::default(),
            work_offsets: [Position::default(); 6],
            spindle_speed: 0.0,
            spindle_enabled: false,
            feed_rate: 0.0,
            feed_override: 100.0,
            spindle_override: 100.0,
            rapid_override: 100.0,
            buffer_state: 0,
            last_error: None,
        }
    }
}

impl MachineState {
    /// Create a new machine state
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the work offset for a specific coordinate system
    pub fn get_work_offset(&self, cs: CoordinateSystem) -> Position {
        let index = match cs {
            CoordinateSystem::G54 => 0,
            CoordinateSystem::G55 => 1,
            CoordinateSystem::G56 => 2,
            CoordinateSystem::G57 => 3,
            CoordinateSystem::G58 => 4,
            CoordinateSystem::G59 => 5,
        };
        self.work_offsets[index]
    }

    /// Set the work offset for a specific coordinate system
    pub fn set_work_offset(&mut self, cs: CoordinateSystem, offset: Position) {
        let index = match cs {
            CoordinateSystem::G54 => 0,
            CoordinateSystem::G55 => 1,
            CoordinateSystem::G56 => 2,
            CoordinateSystem::G57 => 3,
            CoordinateSystem::G58 => 4,
            CoordinateSystem::G59 => 5,
        };
        self.work_offsets[index] = offset;
    }

    /// Update machine position and calculate work position
    pub fn update_machine_position(&mut self, pos: Position) {
        self.machine_position = pos;
        let offset = self.get_work_offset(self.coordinate_system);
        self.work_position = Position::new(
            pos.x - offset.x,
            pos.y - offset.y,
            pos.z - offset.z,
        );
    }

    /// Check if machine is in an error state
    pub fn is_error_state(&self) -> bool {
        matches!(self.status, MachineStatus::Alarm)
    }

    /// Check if machine is idle
    pub fn is_idle(&self) -> bool {
        matches!(self.status, MachineStatus::Idle)
    }

    /// Check if machine is running
    pub fn is_running(&self) -> bool {
        matches!(self.status, MachineStatus::Run | MachineStatus::Jog | MachineStatus::Home)
    }
    
    /// Update machine state from GRBL status report
    /// 
    /// This updates the machine state based on status reports received from GRBL
    /// (in response to `?` queries).
    pub fn update_from_grbl_status(&mut self, grbl_status: &crate::grbl::GrblStatus) {
        // Update machine status
        self.status = match grbl_status.state {
            crate::grbl::MachineState::Idle => MachineStatus::Idle,
            crate::grbl::MachineState::Run => MachineStatus::Run,
            crate::grbl::MachineState::Hold => MachineStatus::Hold,
            crate::grbl::MachineState::Jog => MachineStatus::Jog,
            crate::grbl::MachineState::Alarm => MachineStatus::Alarm,
            crate::grbl::MachineState::Door => MachineStatus::Door,
            crate::grbl::MachineState::Check => MachineStatus::Check,
            crate::grbl::MachineState::Home => MachineStatus::Home,
            crate::grbl::MachineState::Sleep => MachineStatus::Sleep,
        };
        
        // Update machine position if available
        if let Some(mpos) = grbl_status.mpos {
            self.machine_position = Position::new(mpos.x, mpos.y, mpos.z);
        }
        
        // Update work position if available
        if let Some(wpos) = grbl_status.wpos {
            self.work_position = Position::new(wpos.x, wpos.y, wpos.z);
        }
        
        // If we have work coordinate offset, we can calculate the other position
        if let Some(wco) = grbl_status.wco {
            let offset = Position::new(wco.x, wco.y, wco.z);
            
            // Update work offset for current coordinate system
            self.set_work_offset(self.coordinate_system, offset);
            
            // If we have machine position, calculate work position
            if grbl_status.mpos.is_some() {
                self.work_position = Position::new(
                    self.machine_position.x - offset.x,
                    self.machine_position.y - offset.y,
                    self.machine_position.z - offset.z,
                );
            }
            // If we have work position, calculate machine position
            else if grbl_status.wpos.is_some() {
                self.machine_position = Position::new(
                    self.work_position.x + offset.x,
                    self.work_position.y + offset.y,
                    self.work_position.z + offset.z,
                );
            }
        }
        
        // Update feed rate if available
        if let Some(feed_rate) = grbl_status.feed_rate {
            self.feed_rate = feed_rate;
        }
        
        // Update spindle speed if available
        if let Some(spindle_speed) = grbl_status.spindle_speed {
            self.spindle_speed = spindle_speed;
            self.spindle_enabled = spindle_speed > 0.0;
        }
        
        // Update override values if available
        if let Some(feed_ov) = grbl_status.feed_override {
            self.feed_override = feed_ov as f64;
        }
        if let Some(rapid_ov) = grbl_status.rapid_override {
            self.rapid_override = rapid_ov as f64;
        }
        if let Some(spindle_ov) = grbl_status.spindle_override {
            self.spindle_override = spindle_ov as f64;
        }
        
        // Update buffer state if available
        if let Some((planner, _rx)) = grbl_status.buffer {
            self.buffer_state = planner as u32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let pos = Position::new(1.0, 2.0, 3.0);
        assert_eq!(pos.x, 1.0);
        assert_eq!(pos.y, 2.0);
        assert_eq!(pos.z, 3.0);
    }

    #[test]
    fn test_machine_state_work_offset() {
        let mut state = MachineState::new();
        let offset = Position::new(10.0, 20.0, 5.0);
        state.set_work_offset(CoordinateSystem::G54, offset);
        
        let retrieved = state.get_work_offset(CoordinateSystem::G54);
        assert_eq!(retrieved.x, 10.0);
        assert_eq!(retrieved.y, 20.0);
        assert_eq!(retrieved.z, 5.0);
    }

    #[test]
    fn test_machine_state_position_update() {
        let mut state = MachineState::new();
        state.set_work_offset(CoordinateSystem::G54, Position::new(10.0, 20.0, 5.0));
        state.update_machine_position(Position::new(50.0, 60.0, 15.0));
        
        assert_eq!(state.work_position.x, 40.0);
        assert_eq!(state.work_position.y, 40.0);
        assert_eq!(state.work_position.z, 10.0);
    }
}
