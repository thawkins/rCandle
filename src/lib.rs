//! rCandle core library
//!
//! This library provides the core functionality for the rCandle GRBL controller.

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod connection;
pub mod grbl;
pub mod heightmap;
pub mod parser;
pub mod renderer;
pub mod script;
pub mod state;
pub mod ui;
pub mod utils;

/// Application version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application name
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
