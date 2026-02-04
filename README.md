# A3S Tools Core

[![Crates.io](https://img.shields.io/crates/v/a3s-tools-core.svg)](https://crates.io/crates/a3s-tools-core)
[![Documentation](https://docs.rs/a3s-tools-core/badge.svg)](https://docs.rs/a3s-tools-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Shared library for A3S tools providing sandbox path resolution, error types, constants, and utilities.

## Features

- **Sandbox Security**: Path resolution with workspace boundary enforcement
- **Error Handling**: Comprehensive error types with `thiserror`
- **Constants**: Predefined limits for output size, line length, and timeouts
- **Output Utilities**: Line numbering and output truncation helpers

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
a3s-tools-core = "0.1"
```

### Path Resolution

```rust
use a3s_tools_core::{resolve_path, resolve_path_for_write};
use std::path::Path;

let workspace = Path::new("/workspace");

// Resolve existing file (requires file to exist)
let path = resolve_path(workspace, "src/main.rs")?;

// Resolve for write (allows non-existent files)
let path = resolve_path_for_write(workspace, "output/new_file.txt")?;
```

### Error Handling

```rust
use a3s_tools_core::ToolError;

fn my_tool() -> Result<(), ToolError> {
    // Path outside workspace
    Err(ToolError::PathOutsideWorkspace("../etc/passwd".to_string()))
}
```

### Output Formatting

```rust
use a3s_tools_core::{format_line_numbered, truncate_output};

let content = "line1\nline2\nline3";
let formatted = format_line_numbered(content, 0);
// Output:
// 1	line1
// 2	line2
// 3	line3

let large_output = "x".repeat(200_000);
let truncated = truncate_output(&large_output);
// Truncates to MAX_OUTPUT_SIZE (100KB) with message
```

## Security

All path operations enforce workspace boundaries:

- Canonicalizes paths to handle symlinks (e.g., `/var` → `/private/var` on macOS)
- Rejects absolute paths outside workspace
- Rejects relative paths that escape workspace (e.g., `../../etc/passwd`)

## Constants

- `MAX_OUTPUT_SIZE`: 100KB (102,400 bytes)
- `MAX_READ_LINES`: 2,000 lines
- `MAX_LINE_LENGTH`: 2,000 characters
- `DEFAULT_TIMEOUT_MS`: 120,000ms (2 minutes)
- `MAX_TIMEOUT_MS`: 600,000ms (10 minutes)

## Testing

```bash
cargo test
```

All 14 tests passing ✅

## License

MIT License - see [LICENSE](LICENSE) for details.

## Part of A3S

This library is part of the [A3S](https://github.com/A3S-Lab/A3S) (AI Agent Sandbox System) project.
