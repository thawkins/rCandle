//! Utilities module
//!
//! Provides error types, logging setup, and common utilities.

pub mod error;
pub mod logging;

pub use error::{Error, Result};
pub use logging::init_logging;
