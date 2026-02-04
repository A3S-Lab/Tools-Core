//! Constants for tool operations
//!
//! This module defines limits and timeouts used across all A3S tools.

/// Maximum output size in bytes before truncation
///
/// When tool output exceeds this limit, it will be truncated with a message
/// indicating the total size and how much was shown.
///
/// # Value
/// 100KB (102,400 bytes)
pub const MAX_OUTPUT_SIZE: usize = 100 * 1024; // 100KB

/// Maximum lines to read from a file
///
/// Used by the read tool to limit the number of lines returned in a single operation.
/// Users can use offset/limit parameters to read additional lines.
///
/// # Value
/// 2,000 lines
pub const MAX_READ_LINES: usize = 2000;

/// Maximum line length before truncation
///
/// Lines longer than this will be truncated with "..." appended.
///
/// # Value
/// 2,000 characters
pub const MAX_LINE_LENGTH: usize = 2000;

/// Default command timeout in milliseconds
///
/// Used for bash commands and other operations that may hang.
///
/// # Value
/// 120,000ms (2 minutes)
pub const DEFAULT_TIMEOUT_MS: u64 = 120_000; // 2 minutes

/// Maximum command timeout in milliseconds
///
/// The upper limit for user-specified timeouts.
///
/// # Value
/// 600,000ms (10 minutes)
pub const MAX_TIMEOUT_MS: u64 = 600_000; // 10 minutes
