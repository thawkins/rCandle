//! Error types for rCandle
//!
//! Provides a comprehensive error type using thiserror.

use std::io;

/// Result type alias for rCandle operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for rCandle
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// Serial port error
    #[error("Serial port error: {0}")]
    SerialPort(#[from] serialport::Error),

    /// Parse error
    #[error("Parse error: {0}")]
    Parse(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// GRBL error
    #[error("GRBL error: {0}")]
    Grbl(String),

    /// Rendering error
    #[error("Rendering error: {0}")]
    Render(String),

    /// Invalid state
    #[error("Invalid state: {0}")]
    InvalidState(String),

    /// Not connected
    #[error("Not connected to device")]
    NotConnected,

    /// Timeout
    #[error("Timeout: {0}")]
    Timeout(String),

    /// Queue error
    #[error("Queue error: {0}")]
    Queue(String),

    /// Script error
    #[error("Script error: {0}")]
    Script(String),

    /// Generic error
    #[error("{0}")]
    Generic(String),
}

impl Error {
    /// Create a parse error
    pub fn parse<S: Into<String>>(msg: S) -> Self {
        Error::Parse(msg.into())
    }

    /// Create a config error
    pub fn config<S: Into<String>>(msg: S) -> Self {
        Error::Config(msg.into())
    }

    /// Create a connection error
    pub fn connection<S: Into<String>>(msg: S) -> Self {
        Error::Connection(msg.into())
    }

    /// Create a GRBL error
    pub fn grbl<S: Into<String>>(msg: S) -> Self {
        Error::Grbl(msg.into())
    }

    /// Create a render error
    pub fn render<S: Into<String>>(msg: S) -> Self {
        Error::Render(msg.into())
    }

    /// Create an invalid state error
    pub fn invalid_state<S: Into<String>>(msg: S) -> Self {
        Error::InvalidState(msg.into())
    }

    /// Create a generic error
    pub fn generic<S: Into<String>>(msg: S) -> Self {
        Error::Generic(msg.into())
    }

    /// Create a queue error
    pub fn queue<S: Into<String>>(msg: S) -> Self {
        Error::Queue(msg.into())
    }

    /// Create a timeout error
    pub fn timeout<S: Into<String>>(msg: S) -> Self {
        Error::Timeout(msg.into())
    }

    /// Create a script error
    pub fn script<S: Into<String>>(msg: S) -> Self {
        Error::Script(msg.into())
    }
}
