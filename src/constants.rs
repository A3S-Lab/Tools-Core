//! Constants for tool operations

/// Maximum output size in bytes before truncation
pub const MAX_OUTPUT_SIZE: usize = 100 * 1024; // 100KB

/// Maximum lines to read from a file
pub const MAX_READ_LINES: usize = 2000;

/// Maximum line length before truncation
pub const MAX_LINE_LENGTH: usize = 2000;

/// Default command timeout in milliseconds
pub const DEFAULT_TIMEOUT_MS: u64 = 120_000; // 2 minutes

/// Maximum command timeout in milliseconds
pub const MAX_TIMEOUT_MS: u64 = 600_000; // 10 minutes
