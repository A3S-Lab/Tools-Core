//! Sandbox path resolution and validation
//!
//! Ensures all file operations stay within the workspace boundary.

use crate::ToolError;
use std::path::{Path, PathBuf};

/// Resolve a path relative to workspace, ensuring it stays within sandbox
///
/// # Arguments
/// * `workspace` - The workspace root directory (sandbox boundary)
/// * `path` - The path to resolve (can be relative or absolute)
///
/// # Returns
/// * `Ok(PathBuf)` - The resolved path within workspace
/// * `Err(ToolError)` - If path is outside workspace or doesn't exist
///
/// # Security
/// This function canonicalizes paths to handle symlinks and `..` components,
/// then verifies the result is within the workspace boundary.
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
/// # Arguments
/// * `workspace` - The workspace root directory (sandbox boundary)
/// * `path` - The path to resolve (can be relative or absolute)
///
/// # Returns
/// * `Ok(PathBuf)` - The resolved path within workspace
/// * `Err(ToolError)` - If path would be outside workspace
///
/// # Security
/// For write operations, we can't canonicalize non-existent paths.
/// Instead, we verify the parent directory is within workspace.
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
