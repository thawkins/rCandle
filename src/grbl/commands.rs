//! GRBL command formatting
//!
//! Provides structures and formatting for GRBL commands.

use std::fmt;

/// GRBL command types
#[derive(Debug, Clone, PartialEq)]
pub enum GrblCommand {
    /// G-Code command
    GCode(String),

    /// Get GRBL version and settings ($)
    GetSettings,

    /// Get GRBL version ($$)
    GetVersion,

    /// Get G-Code parameter data ($#)
    GetParameters,

    /// Get parser state ($G)
    GetParserState,

    /// Get build info ($I)
    GetBuildInfo,

    /// Get startup blocks ($N)
    GetStartupBlocks,

    /// Check G-Code mode ($C)
    CheckMode(bool),

    /// Kill alarm lock ($X)
    KillAlarmLock,

    /// Run homing cycle ($H)
    HomingCycle,

    /// Run jogging command
    Jog {
        /// X axis distance/position
        x: Option<f64>,
        /// Y axis distance/position
        y: Option<f64>,
        /// Z axis distance/position
        z: Option<f64>,
        /// Feed rate in mm/min or inches/min
        feed_rate: f64,
    },

    /// Set GRBL setting ($x=val)
    SetSetting {
        /// Setting number
        setting: u32,
        /// Setting value
        value: f64,
    },

    /// Clear GRBL setting ($RST=$)
    ResetSettings,

    /// Clear G-Code parameters ($RST=#)
    ResetParameters,

    /// Clear G54-G59 offsets ($RST=*)
    ResetOffsets,

    /// Sleep mode ($SLP)
    Sleep,
}

impl GrblCommand {
    /// Format command as string for sending to GRBL
    pub fn format(&self) -> String {
        match self {
            GrblCommand::GCode(code) => {
                // Ensure proper line ending
                let code = code.trim();
                if code.ends_with('\n') {
                    code.to_string()
                } else {
                    format!("{}\n", code)
                }
            }
            GrblCommand::GetSettings => "$$\n".to_string(),
            GrblCommand::GetVersion => "$I\n".to_string(),
            GrblCommand::GetParameters => "$#\n".to_string(),
            GrblCommand::GetParserState => "$G\n".to_string(),
            GrblCommand::GetBuildInfo => "$I\n".to_string(),
            GrblCommand::GetStartupBlocks => "$N\n".to_string(),
            GrblCommand::CheckMode(enable) => {
                if *enable {
                    "$C\n".to_string()
                } else {
                    "$C\n".to_string() // Toggle
                }
            }
            GrblCommand::KillAlarmLock => "$X\n".to_string(),
            GrblCommand::HomingCycle => "$H\n".to_string(),
            GrblCommand::Jog {
                x,
                y,
                z,
                feed_rate,
            } => {
                let mut cmd = String::from("$J=G91");
                if let Some(x_val) = x {
                    cmd.push_str(&format!(" X{:.3}", x_val));
                }
                if let Some(y_val) = y {
                    cmd.push_str(&format!(" Y{:.3}", y_val));
                }
                if let Some(z_val) = z {
                    cmd.push_str(&format!(" Z{:.3}", z_val));
                }
                cmd.push_str(&format!(" F{:.0}\n", feed_rate));
                cmd
            }
            GrblCommand::SetSetting { setting, value } => {
                format!("${}={}\n", setting, value)
            }
            GrblCommand::ResetSettings => "$RST=$\n".to_string(),
            GrblCommand::ResetParameters => "$RST=#\n".to_string(),
            GrblCommand::ResetOffsets => "$RST=*\n".to_string(),
            GrblCommand::Sleep => "$SLP\n".to_string(),
        }
    }
}

impl fmt::Display for GrblCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format().trim())
    }
}

/// GRBL settings structure
#[derive(Debug, Clone)]
pub struct GrblSettings {
    /// Step pulse time (microseconds)
    pub step_pulse: Option<f64>,
    /// Step idle delay (milliseconds)
    pub step_idle_delay: Option<f64>,
    /// Step port invert mask
    pub step_port_invert: Option<u32>,
    /// Direction port invert mask
    pub dir_port_invert: Option<u32>,
    /// Step enable invert
    pub step_enable_invert: Option<bool>,
    /// Limit pins invert
    pub limit_pins_invert: Option<bool>,
    /// Probe pin invert
    pub probe_pin_invert: Option<bool>,
    /// Status report options (mask)
    pub status_report: Option<u32>,
    /// Junction deviation (mm)
    pub junction_deviation: Option<f64>,
    /// Arc tolerance (mm)
    pub arc_tolerance: Option<f64>,
    /// Report in inches
    pub report_inches: Option<bool>,
    /// Soft limits enable
    pub soft_limits: Option<bool>,
    /// Hard limits enable
    pub hard_limits: Option<bool>,
    /// Homing cycle enable
    pub homing_enable: Option<bool>,
    /// Homing direction invert mask
    pub homing_dir_invert: Option<u32>,
    /// Homing feed rate (mm/min)
    pub homing_feed: Option<f64>,
    /// Homing seek rate (mm/min)
    pub homing_seek: Option<f64>,
    /// Homing debounce (ms)
    pub homing_debounce: Option<f64>,
    /// Homing pull-off distance (mm)
    pub homing_pull_off: Option<f64>,
    /// Max spindle speed (RPM)
    pub max_spindle_speed: Option<f64>,
    /// Min spindle speed (RPM)
    pub min_spindle_speed: Option<f64>,
    /// Laser mode enable
    pub laser_mode: Option<bool>,
    /// X-axis travel (mm)
    pub x_max_travel: Option<f64>,
    /// Y-axis travel (mm)
    pub y_max_travel: Option<f64>,
    /// Z-axis travel (mm)
    pub z_max_travel: Option<f64>,
    /// X-axis steps per mm
    pub x_steps_per_mm: Option<f64>,
    /// Y-axis steps per mm
    pub y_steps_per_mm: Option<f64>,
    /// Z-axis steps per mm
    pub z_steps_per_mm: Option<f64>,
    /// X-axis max rate (mm/min)
    pub x_max_rate: Option<f64>,
    /// Y-axis max rate (mm/min)
    pub y_max_rate: Option<f64>,
    /// Z-axis max rate (mm/min)
    pub z_max_rate: Option<f64>,
    /// X-axis acceleration (mm/sec^2)
    pub x_acceleration: Option<f64>,
    /// Y-axis acceleration (mm/sec^2)
    pub y_acceleration: Option<f64>,
    /// Z-axis acceleration (mm/sec^2)
    pub z_acceleration: Option<f64>,
}

impl Default for GrblSettings {
    fn default() -> Self {
        Self {
            step_pulse: None,
            step_idle_delay: None,
            step_port_invert: None,
            dir_port_invert: None,
            step_enable_invert: None,
            limit_pins_invert: None,
            probe_pin_invert: None,
            status_report: None,
            junction_deviation: None,
            arc_tolerance: None,
            report_inches: None,
            soft_limits: None,
            hard_limits: None,
            homing_enable: None,
            homing_dir_invert: None,
            homing_feed: None,
            homing_seek: None,
            homing_debounce: None,
            homing_pull_off: None,
            max_spindle_speed: None,
            min_spindle_speed: None,
            laser_mode: None,
            x_max_travel: None,
            y_max_travel: None,
            z_max_travel: None,
            x_steps_per_mm: None,
            y_steps_per_mm: None,
            z_steps_per_mm: None,
            x_max_rate: None,
            y_max_rate: None,
            z_max_rate: None,
            x_acceleration: None,
            y_acceleration: None,
            z_acceleration: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcode_command_format() {
        let cmd = GrblCommand::GCode("G0 X10".to_string());
        assert_eq!(cmd.format(), "G0 X10\n");
    }

    #[test]
    fn test_gcode_command_with_newline() {
        let cmd = GrblCommand::GCode("G0 X10\n".to_string());
        assert_eq!(cmd.format(), "G0 X10\n");
    }

    #[test]
    fn test_get_settings_format() {
        let cmd = GrblCommand::GetSettings;
        assert_eq!(cmd.format(), "$$\n");
    }

    #[test]
    fn test_homing_cycle_format() {
        let cmd = GrblCommand::HomingCycle;
        assert_eq!(cmd.format(), "$H\n");
    }

    #[test]
    fn test_jog_command_format() {
        let cmd = GrblCommand::Jog {
            x: Some(10.0),
            y: Some(-5.5),
            z: None,
            feed_rate: 1000.0,
        };
        let formatted = cmd.format();
        assert!(formatted.contains("$J=G91"));
        assert!(formatted.contains("X10.000"));
        assert!(formatted.contains("Y-5.500"));
        assert!(formatted.contains("F1000"));
    }

    #[test]
    fn test_set_setting_format() {
        let cmd = GrblCommand::SetSetting {
            setting: 110,
            value: 500.0,
        };
        assert_eq!(cmd.format(), "$110=500\n");
    }

    #[test]
    fn test_kill_alarm_lock_format() {
        let cmd = GrblCommand::KillAlarmLock;
        assert_eq!(cmd.format(), "$X\n");
    }

    #[test]
    fn test_command_display() {
        let cmd = GrblCommand::GetSettings;
        assert_eq!(format!("{}", cmd), "$$");
    }

    #[test]
    fn test_grbl_settings_default() {
        let settings = GrblSettings::default();
        assert!(settings.step_pulse.is_none());
        assert!(settings.homing_enable.is_none());
    }
}
