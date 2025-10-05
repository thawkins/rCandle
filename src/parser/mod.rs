//! G-Code Parser module
//!
//! This module provides functionality for parsing G-Code files into an internal
//! representation that can be used for visualization and execution.
//!
//! The parser is divided into several stages:
//! - **Tokenizer/Lexer**: Breaks G-Code text into tokens
//! - **Parser**: Converts tokens into structured commands
//! - **Segment Generator**: Converts commands into motion segments
//! - **Preprocessor**: Optimizes and transforms segments

mod tokenizer;
mod parser;
mod segment;
mod preprocessor;
mod types;

pub use tokenizer::{Token, Tokenizer};
pub use parser::{Parser, ParsedCommand};
pub use segment::{ArcDirection, Point3D, Segment, SegmentType};
pub use preprocessor::Preprocessor;
pub use types::*;
