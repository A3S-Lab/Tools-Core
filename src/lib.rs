//! Shared library for A3S tools
//!
//! Provides common functionality for tool implementations:
//! - Sandbox path resolution and validation
//! - Constants for output limits
//! - Error types
//! - Output formatting utilities

mod constants;
mod error;
mod output;
mod sandbox;

pub use constants::*;
pub use error::ToolError;
pub use output::{format_line_numbered, truncate_output};
pub use sandbox::{resolve_path, resolve_path_for_write};
