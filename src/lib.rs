//! Shared library for A3S tools
//!
//! This crate provides common functionality for tool implementations in the A3S ecosystem:
//! - **Sandbox path resolution and validation** - Ensures all file operations stay within workspace boundaries
//! - **Constants for output limits** - Predefined limits for output size, line length, and timeouts
//! - **Error types** - Comprehensive error handling with [`ToolError`]
//! - **Output formatting utilities** - Line numbering and output truncation helpers
//!
//! # Examples
//!
//! ## Path Resolution
//!
//! ```rust
//! use a3s_tools_core::{resolve_path, resolve_path_for_write};
//! use std::path::Path;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let workspace = Path::new("/workspace");
//!
//! // Resolve existing file (requires file to exist)
//! // let path = resolve_path(workspace, "src/main.rs")?;
//!
//! // Resolve for write (allows non-existent files)
//! let path = resolve_path_for_write(workspace, "output/new_file.txt")?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Output Formatting
//!
//! ```rust
//! use a3s_tools_core::{format_line_numbered, truncate_output};
//!
//! let content = "line1\nline2\nline3";
//! let formatted = format_line_numbered(content, 0);
//! // Output:
//! // 1    line1
//! // 2    line2
//! // 3    line3
//!
//! let large_output = "x".repeat(200_000);
//! let truncated = truncate_output(&large_output);
//! // Truncates to MAX_OUTPUT_SIZE (100KB) with message
//! ```
//!
//! # Security
//!
//! All path operations enforce workspace boundaries:
//! - Canonicalizes paths to handle symlinks (e.g., `/var` → `/private/var` on macOS)
//! - Rejects absolute paths outside workspace
//! - Rejects relative paths that escape workspace (e.g., `../../etc/passwd`)

mod constants;
mod error;
mod output;
mod sandbox;

pub use constants::*;
pub use error::ToolError;
pub use output::{format_line_numbered, truncate_output};
pub use sandbox::{resolve_path, resolve_path_for_write};
