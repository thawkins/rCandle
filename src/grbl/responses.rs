//! GRBL response parsing
//!
//! Parses responses from GRBL controllers.

use crate::{Error, Result};
use std::str::FromStr;

/// GRBL response types
#[derive(Debug, Clone, PartialEq)]
pub enum GrblResponse {
    /// OK response
    Ok,

    /// Error response with error code
    Error(u8),

    /// Alarm response with alarm code
    Alarm(u8),

    /// Status report
    Status(GrblStatus),

    /// Welcome message (on startup/reset)
    Welcome {
        /// GRBL version
        version: String,
    },

    /// Settings line ($x=val)
    Setting {
        /// Setting number
        number: u32,
        /// Setting value
        value: String,
    },

    /// Feedback message in brackets (e.g., [MSG:Reset to continue])
    Feedback(String),

    /// Other message (informational)
    Message(String),
}

impl GrblResponse {
    /// Parse a line from GRBL into a response
    pub fn parse(line: &str) -> Result<Self> {
        let line = line.trim();

        if line.is_empty() {
            return Err(Error::Parse("Empty line".to_string()));
        }

        // OK response
        if line.eq_ignore_ascii_case("ok") {
            return Ok(GrblResponse::Ok);
        }

        // Error response: error:X
        if line.to_lowercase().starts_with("error:") {
            let code = line[6..]
                .trim()
                .parse()
                .map_err(|_| Error::Parse(format!("Invalid error code: {}", line)))?;
            return Ok(GrblResponse::Error(code));
        }

        // Alarm response: ALARM:X
        if line.to_uppercase().starts_with("ALARM:") {
            let code = line[6..]
                .trim()
                .parse()
                .map_err(|_| Error::Parse(format!("Invalid alarm code: {}", line)))?;
            return Ok(GrblResponse::Alarm(code));
        }

        // Status report: <Idle|MPos:0.000,0.000,0.000|...>
        if line.starts_with('<') && line.ends_with('>') {
            let status = GrblStatus::parse(&line[1..line.len() - 1])?;
            return Ok(GrblResponse::Status(status));
        }

        // Welcome message: Grbl X.Xx ['$' for help]
        if line.starts_with("Grbl ") {
            let version = line[5..]
                .split('[')
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            return Ok(GrblResponse::Welcome { version });
        }

        // Setting: $x=val
        if line.starts_with('$') && line.contains('=') {
            let parts: Vec<&str> = line[1..].split('=').collect();
            if parts.len() == 2 {
                if let Ok(number) = parts[0].parse() {
                    return Ok(GrblResponse::Setting {
                        number,
                        value: parts[1].to_string(),
                    });
                }
            }
        }

        // Feedback message: [MSG:...]
        if line.starts_with('[') && line.ends_with(']') {
            let content = &line[1..line.len() - 1];
            return Ok(GrblResponse::Feedback(content.to_string()));
        }

        // Default: treat as message
        Ok(GrblResponse::Message(line.to_string()))
    }

    /// Check if response indicates success
    pub fn is_ok(&self) -> bool {
        matches!(self, GrblResponse::Ok)
    }

    /// Check if response indicates an error
    pub fn is_error(&self) -> bool {
        matches!(self, GrblResponse::Error(_))
    }

    /// Check if response indicates an alarm
    pub fn is_alarm(&self) -> bool {
        matches!(self, GrblResponse::Alarm(_))
    }

    /// Get error message for error/alarm codes
    pub fn error_message(&self) -> Option<&'static str> {
        match self {
            GrblResponse::Error(code) => Some(get_error_message(*code)),
            GrblResponse::Alarm(code) => Some(get_alarm_message(*code)),
            _ => None,
        }
    }
}

/// GRBL status report
#[derive(Debug, Clone, PartialEq)]
pub struct GrblStatus {
    /// Machine state
    pub state: MachineState,

    /// Machine position (absolute coordinates)
    pub mpos: Option<Position>,

    /// Work position (relative to work offset)
    pub wpos: Option<Position>,

    /// Work coordinate offset
    pub wco: Option<Position>,

    /// Buffer state: planner blocks available, RX bytes available
    pub buffer: Option<(u8, u16)>,

    /// Feed rate in mm/min or inches/min
    pub feed_rate: Option<f64>,

    /// Spindle speed in RPM
    pub spindle_speed: Option<f64>,

    /// Feed override percentage
    pub feed_override: Option<u8>,

    /// Rapid override percentage
    pub rapid_override: Option<u8>,

    /// Spindle override percentage
    pub spindle_override: Option<u8>,

    /// Pin state (as bitmask string)
    pub pins: Option<String>,

    /// Accessories state (Spindle/Flood/Mist)
    pub accessories: Option<String>,
}

impl GrblStatus {
    /// Parse status report content (without < >)
    pub fn parse(content: &str) -> Result<Self> {
        let parts: Vec<&str> = content.split('|').collect();

        if parts.is_empty() {
            return Err(Error::Parse("Empty status report".to_string()));
        }

        let state = MachineState::from_str(parts[0])?;

        let mut status = GrblStatus {
            state,
            mpos: None,
            wpos: None,
            wco: None,
            buffer: None,
            feed_rate: None,
            spindle_speed: None,
            feed_override: None,
            rapid_override: None,
            spindle_override: None,
            pins: None,
            accessories: None,
        };

        // Parse remaining parts
        for part in &parts[1..] {
            if let Some(pos) = part.strip_prefix("MPos:") {
                status.mpos = Some(Position::parse(pos)?);
            } else if let Some(pos) = part.strip_prefix("WPos:") {
                status.wpos = Some(Position::parse(pos)?);
            } else if let Some(offset) = part.strip_prefix("WCO:") {
                status.wco = Some(Position::parse(offset)?);
            } else if let Some(buf) = part.strip_prefix("Bf:") {
                let buf_parts: Vec<&str> = buf.split(',').collect();
                if buf_parts.len() == 2 {
                    if let (Ok(planner), Ok(rx)) = (buf_parts[0].parse(), buf_parts[1].parse()) {
                        status.buffer = Some((planner, rx));
                    }
                }
            } else if let Some(rate) = part.strip_prefix("F:") {
                status.feed_rate = rate.parse().ok();
            } else if let Some(speed) = part.strip_prefix("S:") {
                status.spindle_speed = speed.parse().ok();
            } else if let Some(ov) = part.strip_prefix("Ov:") {
                let ov_parts: Vec<&str> = ov.split(',').collect();
                if ov_parts.len() == 3 {
                    status.feed_override = ov_parts[0].parse().ok();
                    status.rapid_override = ov_parts[1].parse().ok();
                    status.spindle_override = ov_parts[2].parse().ok();
                }
            } else if let Some(pins) = part.strip_prefix("Pn:") {
                status.pins = Some(pins.to_string());
            } else if part.starts_with('A') {
                status.accessories = Some(part.to_string());
            }
        }

        Ok(status)
    }
}

/// Machine state from GRBL
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineState {
    /// Machine is idle
    Idle,
    /// Machine is running a job
    Run,
    /// Machine is in hold (paused)
    Hold,
    /// Machine is jogging
    Jog,
    /// Machine is in alarm state
    Alarm,
    /// Machine is in door safety mode
    Door,
    /// Machine is performing check mode
    Check,
    /// Machine is homing
    Home,
    /// Machine is sleeping
    Sleep,
}

impl FromStr for MachineState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "idle" => Ok(MachineState::Idle),
            "run" => Ok(MachineState::Run),
            "hold" | "hold:0" | "hold:1" => Ok(MachineState::Hold),
            "jog" => Ok(MachineState::Jog),
            "alarm" => Ok(MachineState::Alarm),
            "door" | "door:0" | "door:1" | "door:2" | "door:3" => Ok(MachineState::Door),
            "check" => Ok(MachineState::Check),
            "home" => Ok(MachineState::Home),
            "sleep" => Ok(MachineState::Sleep),
            _ => Err(Error::Parse(format!("Unknown machine state: {}", s))),
        }
    }
}

/// 3D position
#[derive(Debug, Clone, Copy, PartialEq)]
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
        Self { x, y, z }
    }

    /// Parse position from string "x,y,z"
    pub fn parse(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err(Error::Parse(format!(
                "Invalid position format: {}",
                s
            )));
        }

        let x = parts[0]
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid X coordinate: {}", parts[0])))?;
        let y = parts[1]
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid Y coordinate: {}", parts[1])))?;
        let z = parts[2]
            .parse()
            .map_err(|_| Error::Parse(format!("Invalid Z coordinate: {}", parts[2])))?;

        Ok(Position { x, y, z })
    }
}

/// Get error message for GRBL error code
fn get_error_message(code: u8) -> &'static str {
    match code {
        1 => "G-code words consist of a letter and a value. Letter was not found.",
        2 => "Numeric value format is not valid or missing an expected value.",
        3 => "Grbl '$' system command was not recognized or supported.",
        4 => "Negative value received for an expected positive value.",
        5 => "Homing cycle is not enabled via settings.",
        6 => "Minimum step pulse time must be greater than 3usec",
        7 => "EEPROM read failed. Reset and restored to default values.",
        8 => "Grbl '$' command cannot be used unless Grbl is IDLE.",
        9 => "G-code locked out during alarm or jog state",
        10 => "Soft limits cannot be enabled without homing also enabled.",
        11 => "Max characters per line exceeded. Line was not processed and executed.",
        12 => "Grbl '$' setting value exceeds the maximum step rate supported.",
        13 => "Safety door detected as opened and door state initiated.",
        14 => "Build info or startup line exceeded EEPROM line length limit.",
        15 => "Jog target exceeds machine travel. Command ignored.",
        16 => "Jog command with no '=' or contains prohibited g-code.",
        17 => "Laser mode requires PWM output.",
        20 => "Unsupported or invalid g-code command found in block.",
        21 => "More than one g-code command from same modal group found in block.",
        22 => "Feed rate has not yet been set or is undefined.",
        23 => "G-code command in block requires an integer value.",
        24 => "Two G-code commands that both require the use of the XYZ axis words were detected.",
        25 => "A G-code word was repeated in the block.",
        26 => "A G-code command implicitly or explicitly requires XYZ axis words but none were detected.",
        27 => "N line number value is not within the valid range of 1 - 9,999,999.",
        28 => "A G-code command was sent, but is missing required P or L value words.",
        29 => "Grbl supports six work coordinate systems G54-G59. G59.1, G59.2, and G59.3 are not supported.",
        30 => "The G53 G-code command requires either a G0 seek or G1 feed motion mode.",
        31 => "There are unused axis words in the block and G80 motion mode cancel is active.",
        32 => "A G2 or G3 arc was commanded but there are no XYZ axis words.",
        33 => "The motion command has an invalid target. G2, G3, and G38.2 generates this error.",
        34 => "Arc radius value is invalid.",
        35 => "G2 and G3 arcs require at least one in-plane axis word.",
        36 => "Multiple axis words found in the same command block.",
        37 => "Line number is missing. Coordinate offsets must be specified with a line number.",
        38 => "A G59.x work coordinate system is not supported.",
        _ => "Unknown error code",
    }
}

/// Get alarm message for GRBL alarm code
fn get_alarm_message(code: u8) -> &'static str {
    match code {
        1 => "Hard limit triggered. Machine position is likely lost.",
        2 => "G-code motion target exceeds machine travel.",
        3 => "Reset while in motion. Grbl cannot guarantee position.",
        4 => "Probe fail. The probe is not in the expected initial state before starting probe cycle.",
        5 => "Probe fail. Probe did not contact the workpiece within the programmed travel.",
        6 => "Homing fail. Reset during active homing cycle.",
        7 => "Homing fail. Safety door was opened during active homing cycle.",
        8 => "Homing fail. Cycle failed to clear limit switch. Try increasing pull-off setting or check wiring.",
        9 => "Homing fail. Could not find limit switch within search distance.",
        _ => "Unknown alarm code",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ok() {
        let response = GrblResponse::parse("ok").unwrap();
        assert_eq!(response, GrblResponse::Ok);
        assert!(response.is_ok());
    }

    #[test]
    fn test_parse_error() {
        let response = GrblResponse::parse("error:1").unwrap();
        assert!(matches!(response, GrblResponse::Error(1)));
        assert!(response.is_error());
        assert!(response.error_message().is_some());
    }

    #[test]
    fn test_parse_alarm() {
        let response = GrblResponse::parse("ALARM:1").unwrap();
        assert!(matches!(response, GrblResponse::Alarm(1)));
        assert!(response.is_alarm());
    }

    #[test]
    fn test_parse_welcome() {
        let response = GrblResponse::parse("Grbl 1.1f ['$' for help]").unwrap();
        assert!(matches!(response, GrblResponse::Welcome { .. }));
    }

    #[test]
    fn test_parse_setting() {
        let response = GrblResponse::parse("$110=500.000").unwrap();
        assert!(matches!(
            response,
            GrblResponse::Setting {
                number: 110,
                value: _
            }
        ));
    }

    #[test]
    fn test_parse_feedback() {
        let response = GrblResponse::parse("[MSG:Reset to continue]").unwrap();
        assert!(matches!(response, GrblResponse::Feedback(_)));
    }

    #[test]
    fn test_parse_status() {
        let response = GrblResponse::parse("<Idle|MPos:0.000,0.000,0.000|WPos:0.000,0.000,0.000>")
            .unwrap();
        if let GrblResponse::Status(status) = response {
            assert_eq!(status.state, MachineState::Idle);
            assert!(status.mpos.is_some());
            assert!(status.wpos.is_some());
        } else {
            panic!("Expected Status response");
        }
    }

    #[test]
    fn test_position_parse() {
        let pos = Position::parse("1.5,-2.3,10.0").unwrap();
        assert_eq!(pos.x, 1.5);
        assert_eq!(pos.y, -2.3);
        assert_eq!(pos.z, 10.0);
    }

    #[test]
    fn test_machine_state_from_str() {
        assert_eq!(MachineState::from_str("Idle").unwrap(), MachineState::Idle);
        assert_eq!(MachineState::from_str("Run").unwrap(), MachineState::Run);
        assert_eq!(MachineState::from_str("Hold").unwrap(), MachineState::Hold);
        assert_eq!(MachineState::from_str("Jog").unwrap(), MachineState::Jog);
    }

    #[test]
    fn test_error_messages() {
        assert!(get_error_message(1).len() > 0);
        assert!(get_error_message(20).len() > 0);
    }

    #[test]
    fn test_alarm_messages() {
        assert!(get_alarm_message(1).len() > 0);
        assert!(get_alarm_message(5).len() > 0);
    }
}
