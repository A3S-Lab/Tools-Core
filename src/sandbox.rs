//! Sandbox path resolution and validation
//!
//! This module ensures all file operations stay within the workspace boundary.
//! It provides two main functions for path resolution with different semantics:
//!
//! - [`resolve_path`] - For reading existing files (requires file to exist)
//! - [`resolve_path_for_write`] - For writing files (allows non-existent files)
//!
//! # Security
//!
//! Both functions enforce workspace boundaries by:
//! - Canonicalizing paths to resolve symlinks and `..` components
//! - Verifying the resolved path is within the workspace
//! - Rejecting paths that would escape the workspace
//!
//! # Examples
//!
//! ```rust
//! use a3s_tools_core::{resolve_path, resolve_path_for_write};
//! use std::path::Path;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let workspace = Path::new("/workspace");
//!
//! // For reading - file must exist
//! // let path = resolve_path(workspace, "existing_file.txt")?;
//!
//! // For writing - file can be new
//! let path = resolve_path_for_write(workspace, "new_file.txt")?;
//! # Ok(())
//! # }
//! ```

use crate::ToolError;
use std::path::{Path, PathBuf};

/// Resolve a path relative to workspace, ensuring it stays within sandbox
///
/// This function is used for read operations where the file must exist.
/// It canonicalizes the path to handle symlinks and `..` components,
/// then verifies the result is within the workspace boundary.
///
/// # Arguments
///
/// * `workspace` - The workspace root directory (sandbox boundary)
/// * `path` - The path to resolve (can be relative or absolute)
///
/// # Returns
///
/// * `Ok(PathBuf)` - The resolved canonical path within workspace
/// * `Err(ToolError::PathNotFound)` - If the path doesn't exist
/// * `Err(ToolError::PathOutsideWorkspace)` - If the path is outside workspace
///
/// # Security
///
/// This function canonicalizes paths to handle:
/// - Symlinks (e.g., `/var` → `/private/var` on macOS)
/// - Relative paths with `..` components
/// - Absolute paths
///
/// After canonicalization, it verifies the path is within the workspace.
///
/// # Examples
///
/// ```rust,no_run
/// use a3s_tools_core::resolve_path;
/// use std::path::Path;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let workspace = Path::new("/workspace");
///
/// // Resolve relative path
/// let path = resolve_path(workspace, "src/main.rs")?;
///
/// // Resolve absolute path (must be within workspace)
/// let path = resolve_path(workspace, "/workspace/config.json")?;
/// # Ok(())
/// # }
/// ```
pub fn resolve_path(workspace: &Path, path: &str) -> Result<PathBuf, ToolError> {
    let path = Path::new(path);

    let resolved = if path.is_absolute() {
        path.to_path_buf()
    } else {
        workspace.join(path)
    };

    // Canonicalize to resolve .. and symlinks
    let canonical = resolved
        .canonicalize()
        .map_err(|_| ToolError::PathNotFound(path.display().to_string()))?;

    // Canonicalize workspace for comparison (handles symlinks like /var -> /private/var on macOS)
    let canonical_workspace = workspace
        .canonicalize()
        .unwrap_or_else(|_| workspace.to_path_buf());

    // Security check: ensure path is within workspace
    if !canonical.starts_with(&canonical_workspace) {
        return Err(ToolError::PathOutsideWorkspace(path.display().to_string()));
    }

    Ok(canonical)
}

/// Resolve a path for write operations (allows non-existent files)
///
/// This function is used for write operations where the file may not exist yet.
/// Unlike [`resolve_path`], it doesn't require the file to exist, but still
/// ensures the path would be within the workspace boundary.
///
/// # Arguments
///
/// * `workspace` - The workspace root directory (sandbox boundary)
/// * `path` - The path to resolve (can be relative or absolute)
///
/// # Returns
///
/// * `Ok(PathBuf)` - The resolved path within workspace
/// * `Err(ToolError::PathOutsideWorkspace)` - If the path would be outside workspace
///
/// # Security
///
/// For write operations, we can't canonicalize non-existent paths.
/// Instead, we verify the parent directory is within workspace.
/// This prevents creating files outside the workspace boundary.
///
/// # Examples
///
/// ```rust
/// use a3s_tools_core::resolve_path_for_write;
/// use std::path::Path;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let workspace = Path::new("/workspace");
///
/// // Create new file in workspace
/// let path = resolve_path_for_write(workspace, "output/new_file.txt")?;
///
/// // This would fail - escapes workspace
/// let result = resolve_path_for_write(workspace, "../outside.txt");
/// assert!(result.is_err());
/// # Ok(())
/// # }
/// ```
pub fn resolve_path_for_write(workspace: &Path, path: &str) -> Result<PathBuf, ToolError> {
    let path = Path::new(path);

    let resolved = if path.is_absolute() {
        path.to_path_buf()
    } else {
        workspace.join(path)
    };

    // Canonicalize workspace for comparison
    let canonical_workspace = workspace
        .canonicalize()
        .unwrap_or_else(|_| workspace.to_path_buf());

    // For write operations, check that the parent directory is within workspace
    if let Some(parent) = resolved.parent() {
        let canonical_parent = parent
            .canonicalize()
            .unwrap_or_else(|_| parent.to_path_buf());

        // Allow if parent is workspace or within workspace
        if canonical_parent != canonical_workspace
            && !canonical_parent.starts_with(&canonical_workspace)
        {
            return Err(ToolError::PathOutsideWorkspace(path.display().to_string()));
        }
    }

    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_resolve_relative_path() {
        let temp_dir = tempfile::tempdir().unwrap();
        let workspace = temp_dir.path();

        // Create a test file
        let test_file = workspace.join("test.txt");
        fs::write(&test_file, "hello").unwrap();

        let result = resolve_path(workspace, "test.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_file.canonicalize().unwrap());
    }

    #[test]
    fn test_resolve_absolute_path_within_workspace() {
        let temp_dir = tempfile::tempdir().unwrap();
        let workspace = temp_dir.path();

        // Create a test file
        let test_file = workspace.join("test.txt");
        fs::write(&test_file, "hello").unwrap();

        let result = resolve_path(workspace, test_file.to_str().unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn test_reject_path_outside_workspace() {
        let temp_dir = tempfile::tempdir().unwrap();
        let workspace = temp_dir.path();

        // Try to access /etc/passwd (outside workspace)
        let result = resolve_path(workspace, "/etc/passwd");
        assert!(matches!(result, Err(ToolError::PathOutsideWorkspace(_))));
    }

    #[test]
    fn test_reject_path_escape_with_dotdot() {
        let temp_dir = tempfile::tempdir().unwrap();
        let workspace = temp_dir.path();

        // Create a file in parent directory
        let parent = temp_dir.path().parent().unwrap();
        let outside_file = parent.join("outside.txt");
        if fs::write(&outside_file, "secret").is_ok() {
            // Try to escape with ..
            let result = resolve_path(workspace, "../outside.txt");
            assert!(matches!(result, Err(ToolError::PathOutsideWorkspace(_))));
            let _ = fs::remove_file(outside_file);
        }
    }

    #[test]
    fn test_resolve_path_not_found() {
        let temp_dir = tempfile::tempdir().unwrap();
        let workspace = temp_dir.path();

        let result = resolve_path(workspace, "nonexistent.txt");
        assert!(matches!(result, Err(ToolError::PathNotFound(_))));
    }

    #[test]
    fn test_resolve_path_for_write_new_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let workspace = temp_dir.path();

        // Should succeed for non-existent file in workspace
        let result = resolve_path_for_write(workspace, "new_file.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), workspace.join("new_file.txt"));
    }

    #[test]
    fn test_resolve_path_for_write_reject_outside() {
        let temp_dir = tempfile::tempdir().unwrap();
        let workspace = temp_dir.path();

        // Should reject path outside workspace
        let result = resolve_path_for_write(workspace, "/tmp/outside.txt");
        assert!(matches!(result, Err(ToolError::PathOutsideWorkspace(_))));
    }

    #[test]
    fn test_resolve_nested_path() {
        let temp_dir = tempfile::tempdir().unwrap();
        let workspace = temp_dir.path();

        // Create nested directory and file
        let nested_dir = workspace.join("subdir");
        fs::create_dir(&nested_dir).unwrap();
        let nested_file = nested_dir.join("file.txt");
        fs::write(&nested_file, "content").unwrap();

        let result = resolve_path(workspace, "subdir/file.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), nested_file.canonicalize().unwrap());
    }
}
