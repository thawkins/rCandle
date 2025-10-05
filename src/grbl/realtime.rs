//! GRBL real-time commands
//!
//! Real-time commands are single-byte characters that can be sent at any time
//! and are handled immediately by GRBL without requiring a buffer.

/// GRBL real-time commands
///
/// These commands are executed immediately and do not require a newline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RealtimeCommand {
    /// Status report query ('?')
    /// Requests current machine status
    StatusQuery,

    /// Cycle start/resume ('~')
    /// Resumes a paused program
    CycleStartResume,

    /// Feed hold ('!')
    /// Pauses the current operation
    FeedHold,

    /// Reset (Ctrl-X, 0x18)
    /// Soft-reset GRBL
    Reset,

    /// Safety door
    SafetyDoor,

    /// Jog cancel (0x85)
    /// Cancels jog motion
    JogCancel,

    /// Feed override: increase 10%
    FeedOverrideIncrease10,

    /// Feed override: decrease 10%
    FeedOverrideDecrease10,

    /// Feed override: increase 1%
    FeedOverrideIncrease1,

    /// Feed override: decrease 1%
    FeedOverrideDecrease1,

    /// Feed override: set to 100%
    FeedOverrideReset,

    /// Feed override: set to maximum (200%)
    FeedOverrideMax,

    /// Feed override: set to minimum (10%)
    FeedOverrideMin,

    /// Rapid override: set to 100%
    RapidOverrideReset,

    /// Rapid override: set to 50%
    RapidOverride50,

    /// Rapid override: set to 25%
    RapidOverride25,

    /// Spindle override: increase 10%
    SpindleOverrideIncrease10,

    /// Spindle override: decrease 10%
    SpindleOverrideDecrease10,

    /// Spindle override: increase 1%
    SpindleOverrideIncrease1,

    /// Spindle override: decrease 1%
    SpindleOverrideDecrease1,

    /// Toggle spindle stop
    SpindleToggleStop,

    /// Toggle flood coolant
    ToggleFloodCoolant,

    /// Toggle mist coolant
    ToggleMistCoolant,
}

impl RealtimeCommand {
    /// Convert command to its byte representation
    pub fn as_byte(&self) -> u8 {
        match self {
            RealtimeCommand::StatusQuery => b'?',
            RealtimeCommand::CycleStartResume => b'~',
            RealtimeCommand::FeedHold => b'!',
            RealtimeCommand::Reset => 0x18,
            RealtimeCommand::SafetyDoor => 0x84,
            RealtimeCommand::JogCancel => 0x85,
            RealtimeCommand::FeedOverrideReset => 0x90,
            RealtimeCommand::FeedOverrideIncrease10 => 0x91,
            RealtimeCommand::FeedOverrideDecrease10 => 0x92,
            RealtimeCommand::FeedOverrideIncrease1 => 0x93,
            RealtimeCommand::FeedOverrideDecrease1 => 0x94,
            RealtimeCommand::RapidOverrideReset => 0x95,
            RealtimeCommand::RapidOverride50 => 0x96,
            RealtimeCommand::RapidOverride25 => 0x97,
            RealtimeCommand::SpindleOverrideIncrease10 => 0x9A,
            RealtimeCommand::SpindleOverrideDecrease10 => 0x9B,
            RealtimeCommand::SpindleOverrideIncrease1 => 0x9C,
            RealtimeCommand::SpindleOverrideDecrease1 => 0x9D,
            RealtimeCommand::SpindleToggleStop => 0x9E,
            RealtimeCommand::ToggleFloodCoolant => 0xA0,
            RealtimeCommand::ToggleMistCoolant => 0xA1,
            RealtimeCommand::FeedOverrideMax => 0x91, // Same as increase 10%, send multiple times
            RealtimeCommand::FeedOverrideMin => 0x92, // Same as decrease 10%, send multiple times
        }
    }

    /// Get command description
    pub fn description(&self) -> &'static str {
        match self {
            RealtimeCommand::StatusQuery => "Query status",
            RealtimeCommand::CycleStartResume => "Cycle start/resume",
            RealtimeCommand::FeedHold => "Feed hold",
            RealtimeCommand::Reset => "Soft reset",
            RealtimeCommand::SafetyDoor => "Safety door",
            RealtimeCommand::JogCancel => "Jog cancel",
            RealtimeCommand::FeedOverrideIncrease10 => "Feed override +10%",
            RealtimeCommand::FeedOverrideDecrease10 => "Feed override -10%",
            RealtimeCommand::FeedOverrideIncrease1 => "Feed override +1%",
            RealtimeCommand::FeedOverrideDecrease1 => "Feed override -1%",
            RealtimeCommand::FeedOverrideReset => "Feed override 100%",
            RealtimeCommand::FeedOverrideMax => "Feed override 200%",
            RealtimeCommand::FeedOverrideMin => "Feed override 10%",
            RealtimeCommand::RapidOverrideReset => "Rapid override 100%",
            RealtimeCommand::RapidOverride50 => "Rapid override 50%",
            RealtimeCommand::RapidOverride25 => "Rapid override 25%",
            RealtimeCommand::SpindleOverrideIncrease10 => "Spindle override +10%",
            RealtimeCommand::SpindleOverrideDecrease10 => "Spindle override -10%",
            RealtimeCommand::SpindleOverrideIncrease1 => "Spindle override +1%",
            RealtimeCommand::SpindleOverrideDecrease1 => "Spindle override -1%",
            RealtimeCommand::SpindleToggleStop => "Toggle spindle stop",
            RealtimeCommand::ToggleFloodCoolant => "Toggle flood coolant",
            RealtimeCommand::ToggleMistCoolant => "Toggle mist coolant",
        }
    }
}

impl From<RealtimeCommand> for u8 {
    fn from(cmd: RealtimeCommand) -> Self {
        cmd.as_byte()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_realtime_command_bytes() {
        assert_eq!(RealtimeCommand::StatusQuery.as_byte(), b'?');
        assert_eq!(RealtimeCommand::CycleStartResume.as_byte(), b'~');
        assert_eq!(RealtimeCommand::FeedHold.as_byte(), b'!');
        assert_eq!(RealtimeCommand::Reset.as_byte(), 0x18);
    }

    #[test]
    fn test_realtime_command_description() {
        assert_eq!(
            RealtimeCommand::StatusQuery.description(),
            "Query status"
        );
        assert_eq!(RealtimeCommand::FeedHold.description(), "Feed hold");
    }

    #[test]
    fn test_realtime_command_into_u8() {
        let cmd = RealtimeCommand::StatusQuery;
        let byte: u8 = cmd.into();
        assert_eq!(byte, b'?');
    }
}
