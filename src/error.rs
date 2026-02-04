//! Error types for tools

use thiserror::Error;

/// Tool execution errors
#[derive(Debug, Error)]
pub enum ToolError {
    /// Path not found
    #[error("Path not found: {0}")]
    PathNotFound(String),

    /// Path is outside workspace sandbox
    #[error("Path '{0}' is outside workspace")]
    PathOutsideWorkspace(String),

    /// Invalid argument
    #[error("Invalid argument '{name}': {reason}")]
    InvalidArgument { name: String, reason: String },

    /// Missing required argument
    #[error("Missing required argument: {0}")]
    MissingArgument(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Command execution failed
    #[error("Command failed: {0}")]
    CommandFailed(String),

    /// Timeout
    #[error("Operation timed out after {0}ms")]
    Timeout(u64),

    /// Other error
    #[error("{0}")]
    Other(String),
}

impl ToolError {
    /// Create an invalid argument error
    pub fn invalid_arg(name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidArgument {
            name: name.into(),
            reason: reason.into(),
        }
    }

    /// Create a missing argument error
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
