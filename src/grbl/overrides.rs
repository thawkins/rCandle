//! GRBL override controls
//!
//! Implements feed rate, spindle speed, and rapid override controls.

use std::fmt;

/// Override type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverrideType {
    /// Feed rate override (10-200%)
    FeedRate,
    /// Spindle speed override (10-200%)
    SpindleSpeed,
    /// Rapid override (25%, 50%, 100%)
    Rapid,
}

/// Override command for real-time control
#[derive(Debug, Clone, Copy)]
pub enum OverrideCommand {
    /// Feed rate override commands
    FeedRate(FeedRateOverride),
    /// Spindle speed override commands
    SpindleSpeed(SpindleOverride),
    /// Rapid override commands
    Rapid(RapidOverride),
}

/// Feed rate override commands
#[derive(Debug, Clone, Copy)]
pub enum FeedRateOverride {
    /// Reset to 100%
    Reset,
    /// Increase by 10%
    CoarseUp,
    /// Decrease by 10%
    CoarseDown,
    /// Increase by 1%
    FineUp,
    /// Decrease by 1%
    FineDown,
}

/// Spindle speed override commands
#[derive(Debug, Clone, Copy)]
pub enum SpindleOverride {
    /// Reset to 100%
    Reset,
    /// Increase by 10%
    CoarseUp,
    /// Decrease by 10%
    CoarseDown,
    /// Increase by 1%
    FineUp,
    /// Decrease by 1%
    FineDown,
    /// Toggle spindle stop
    Stop,
}

/// Rapid override commands
#[derive(Debug, Clone, Copy)]
pub enum RapidOverride {
    /// Reset to 100%
    Reset,
    /// Set to 50%
    Medium,
    /// Set to 25%
    Low,
}

impl OverrideCommand {
    /// Convert to GRBL real-time command byte
    pub fn to_byte(&self) -> u8 {
        match self {
            OverrideCommand::FeedRate(cmd) => match cmd {
                FeedRateOverride::Reset => 0x90,
                FeedRateOverride::CoarseUp => 0x91,
                FeedRateOverride::CoarseDown => 0x92,
                FeedRateOverride::FineUp => 0x93,
                FeedRateOverride::FineDown => 0x94,
            },
            OverrideCommand::SpindleSpeed(cmd) => match cmd {
                SpindleOverride::Reset => 0x99,
                SpindleOverride::CoarseUp => 0x9A,
                SpindleOverride::CoarseDown => 0x9B,
                SpindleOverride::FineUp => 0x9C,
                SpindleOverride::FineDown => 0x9D,
                SpindleOverride::Stop => 0x9E,
            },
            OverrideCommand::Rapid(cmd) => match cmd {
                RapidOverride::Reset => 0x95,
                RapidOverride::Medium => 0x96,
                RapidOverride::Low => 0x97,
            },
        }
    }
}

impl fmt::Display for OverrideCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OverrideCommand::FeedRate(cmd) => write!(f, "Feed Rate {}", match cmd {
                FeedRateOverride::Reset => "Reset",
                FeedRateOverride::CoarseUp => "+10%",
                FeedRateOverride::CoarseDown => "-10%",
                FeedRateOverride::FineUp => "+1%",
                FeedRateOverride::FineDown => "-1%",
            }),
            OverrideCommand::SpindleSpeed(cmd) => write!(f, "Spindle Speed {}", match cmd {
                SpindleOverride::Reset => "Reset",
                SpindleOverride::CoarseUp => "+10%",
                SpindleOverride::CoarseDown => "-10%",
                SpindleOverride::FineUp => "+1%",
                SpindleOverride::FineDown => "-1%",
                SpindleOverride::Stop => "Stop",
            }),
            OverrideCommand::Rapid(cmd) => write!(f, "Rapid Override {}", match cmd {
                RapidOverride::Reset => "100%",
                RapidOverride::Medium => "50%",
                RapidOverride::Low => "25%",
            }),
        }
    }
}

/// Override state tracking
#[derive(Debug, Clone, Copy)]
pub struct OverrideState {
    /// Feed rate override percentage (10-200)
    pub feed_rate: u8,
    /// Spindle speed override percentage (10-200)
    pub spindle_speed: u8,
    /// Rapid override percentage (25, 50, or 100)
    pub rapid: u8,
}

impl Default for OverrideState {
    fn default() -> Self {
        Self {
            feed_rate: 100,
            spindle_speed: 100,
            rapid: 100,
        }
    }
}

impl OverrideState {
    /// Create a new override state with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Apply a feed rate override command
    pub fn apply_feed_rate(&mut self, cmd: FeedRateOverride) {
        self.feed_rate = match cmd {
            FeedRateOverride::Reset => 100,
            FeedRateOverride::CoarseUp => (self.feed_rate + 10).min(200),
            FeedRateOverride::CoarseDown => (self.feed_rate.saturating_sub(10)).max(10),
            FeedRateOverride::FineUp => (self.feed_rate + 1).min(200),
            FeedRateOverride::FineDown => (self.feed_rate.saturating_sub(1)).max(10),
        };
    }
    
    /// Apply a spindle speed override command
    pub fn apply_spindle_speed(&mut self, cmd: SpindleOverride) {
        self.spindle_speed = match cmd {
            SpindleOverride::Reset => 100,
            SpindleOverride::CoarseUp => (self.spindle_speed + 10).min(200),
            SpindleOverride::CoarseDown => (self.spindle_speed.saturating_sub(10)).max(10),
            SpindleOverride::FineUp => (self.spindle_speed + 1).min(200),
            SpindleOverride::FineDown => (self.spindle_speed.saturating_sub(1)).max(10),
            SpindleOverride::Stop => self.spindle_speed, // Stop is a toggle, doesn't change percentage
        };
    }
    
    /// Apply a rapid override command
    pub fn apply_rapid(&mut self, cmd: RapidOverride) {
        self.rapid = match cmd {
            RapidOverride::Reset => 100,
            RapidOverride::Medium => 50,
            RapidOverride::Low => 25,
        };
    }
    
    /// Apply an override command
    pub fn apply(&mut self, cmd: OverrideCommand) {
        match cmd {
            OverrideCommand::FeedRate(cmd) => self.apply_feed_rate(cmd),
            OverrideCommand::SpindleSpeed(cmd) => self.apply_spindle_speed(cmd),
            OverrideCommand::Rapid(cmd) => self.apply_rapid(cmd),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_feed_rate_override() {
        let mut state = OverrideState::new();
        assert_eq!(state.feed_rate, 100);
        
        state.apply_feed_rate(FeedRateOverride::CoarseUp);
        assert_eq!(state.feed_rate, 110);
        
        state.apply_feed_rate(FeedRateOverride::CoarseDown);
        assert_eq!(state.feed_rate, 100);
        
        state.apply_feed_rate(FeedRateOverride::FineUp);
        assert_eq!(state.feed_rate, 101);
        
        state.apply_feed_rate(FeedRateOverride::FineDown);
        assert_eq!(state.feed_rate, 100);
    }
    
    #[test]
    fn test_rapid_override() {
        let mut state = OverrideState::new();
        assert_eq!(state.rapid, 100);
        
        state.apply_rapid(RapidOverride::Medium);
        assert_eq!(state.rapid, 50);
        
        state.apply_rapid(RapidOverride::Low);
        assert_eq!(state.rapid, 25);
        
        state.apply_rapid(RapidOverride::Reset);
        assert_eq!(state.rapid, 100);
    }
    
    #[test]
    fn test_override_bounds() {
        let mut state = OverrideState::new();
        
        // Test upper bound
        state.feed_rate = 195;
        state.apply_feed_rate(FeedRateOverride::CoarseUp);
        assert_eq!(state.feed_rate, 200);
        state.apply_feed_rate(FeedRateOverride::CoarseUp);
        assert_eq!(state.feed_rate, 200); // Should not exceed 200
        
        // Test lower bound
        state.feed_rate = 15;
        state.apply_feed_rate(FeedRateOverride::CoarseDown);
        assert_eq!(state.feed_rate, 10);
        state.apply_feed_rate(FeedRateOverride::CoarseDown);
        assert_eq!(state.feed_rate, 10); // Should not go below 10
    }
}
