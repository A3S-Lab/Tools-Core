//! Error types for tools
//!
//! This module defines the error types used throughout the A3S tools ecosystem.

use thiserror::Error;

/// Tool execution errors
///
/// This enum represents all possible errors that can occur during tool execution.
/// It uses [`thiserror`] for automatic error trait implementations.
///
/// # Examples
///
/// ```rust
/// use a3s_tools_core::ToolError;
///
/// fn validate_path(path: &str) -> Result<(), ToolError> {
///     if path.starts_with("..") {
///         return Err(ToolError::PathOutsideWorkspace(path.to_string()));
///     }
///     Ok(())
/// }
/// ```
#[derive(Debug, Error)]
pub enum ToolError {
    /// Path not found
    ///
    /// Returned when attempting to resolve a path that doesn't exist.
    #[error("Path not found: {0}")]
    PathNotFound(String),

    /// Path is outside workspace sandbox
    ///
    /// Returned when a path resolution would escape the workspace boundary.
    /// This is a security feature to prevent unauthorized file access.
    #[error("Path '{0}' is outside workspace")]
    PathOutsideWorkspace(String),

    /// Invalid argument
    ///
    /// Returned when a tool receives an argument with an invalid value.
    #[error("Invalid argument '{name}': {reason}")]
    InvalidArgument {
        /// The name of the invalid argument
        name: String,
        /// The reason why the argument is invalid
        reason: String,
    },

    /// Missing required argument
    ///
    /// Returned when a required argument is not provided.
    #[error("Missing required argument: {0}")]
    MissingArgument(String),

    /// I/O error
    ///
    /// Wraps standard I/O errors from file operations.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Command execution failed
    ///
    /// Returned when a shell command or external process fails to execute.
    #[error("Command failed: {0}")]
    CommandFailed(String),

    /// Timeout
    ///
    /// Returned when an operation exceeds its timeout limit.
    #[error("Operation timed out after {0}ms")]
    Timeout(u64),

    /// Other error
    ///
    /// A catch-all for errors that don't fit other categories.
    #[error("{0}")]
    Other(String),
}

impl ToolError {
    /// Create an invalid argument error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use a3s_tools_core::ToolError;
    ///
    /// let err = ToolError::invalid_arg("file_path", "cannot be empty");
    /// assert_eq!(err.to_string(), "Invalid argument 'file_path': cannot be empty");
    /// ```
    pub fn invalid_arg(name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidArgument {
            name: name.into(),
            reason: reason.into(),
        }
    }

    /// Create a missing argument error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use a3s_tools_core::ToolError;
    ///
    /// let err = ToolError::missing_arg("content");
    /// assert_eq!(err.to_string(), "Missing required argument: content");
    /// ```
    pub fn missing_arg(name: impl Into<String>) -> Self {
        Self::MissingArgument(name.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ToolError::PathNotFound("/foo/bar".to_string());
        assert_eq!(err.to_string(), "Path not found: /foo/bar");

        let err = ToolError::PathOutsideWorkspace("../etc/passwd".to_string());
        assert_eq!(
            err.to_string(),
            "Path '../etc/passwd' is outside workspace"
        );

        let err = ToolError::invalid_arg("file_path", "cannot be empty");
        assert_eq!(
            err.to_string(),
            "Invalid argument 'file_path': cannot be empty"
        );

        let err = ToolError::missing_arg("content");
        assert_eq!(err.to_string(), "Missing required argument: content");
    }
}
