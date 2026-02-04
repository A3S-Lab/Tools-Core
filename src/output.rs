//! Output formatting utilities

use crate::constants::{MAX_LINE_LENGTH, MAX_OUTPUT_SIZE};

/// Format content with line numbers
///
/// # Arguments
/// * `content` - The content to format
/// * `offset` - Starting line number (0-indexed)
///
/// # Returns
/// Content with line numbers prefixed to each line
pub fn format_line_numbered(content: &str, offset: usize) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = offset + lines.len();
    let width = total_lines.to_string().len().max(1);

    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let line_num = offset + i + 1;
            let truncated = if line.len() > MAX_LINE_LENGTH {
                format!("{}...", &line[..MAX_LINE_LENGTH - 3])
            } else {
                line.to_string()
            };
            format!("{:>width$}\t{}", line_num, truncated, width = width)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Truncate output if it exceeds maximum size
///
/// # Arguments
/// * `output` - The output to potentially truncate
///
/// # Returns
/// The output, truncated with a message if it exceeded MAX_OUTPUT_SIZE
pub fn truncate_output(output: &str) -> String {
    if output.len() > MAX_OUTPUT_SIZE {
        let truncated = &output[..MAX_OUTPUT_SIZE];
        format!(
            "{}\n\n[Output truncated: {} bytes total, showing first {} bytes]",
            truncated,
            output.len(),
            MAX_OUTPUT_SIZE
        )
    } else {
        output.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_line_numbered() {
        let content = "line1\nline2\nline3";
        let result = format_line_numbered(content, 0);

        assert!(result.contains("1\tline1"));
        assert!(result.contains("2\tline2"));
        assert!(result.contains("3\tline3"));
    }

    #[test]
    fn test_format_line_numbered_with_offset() {
        let content = "line1\nline2";
        let result = format_line_numbered(content, 10);

        assert!(result.contains("11\tline1"));
        assert!(result.contains("12\tline2"));
    }

    #[test]
    fn test_format_line_numbered_long_line() {
        let long_line = "x".repeat(3000);
        let result = format_line_numbered(&long_line, 0);

        // Should be truncated to MAX_LINE_LENGTH
        assert!(result.len() < 3000);
        assert!(result.contains("..."));
    }

    #[test]
    fn test_truncate_output_small() {
        let small = "hello world";
        let result = truncate_output(small);
        assert_eq!(result, small);
    }

    #[test]
    fn test_truncate_output_large() {
        let large = "x".repeat(MAX_OUTPUT_SIZE + 1000);
        let result = truncate_output(&large);

        assert!(result.len() < large.len());
        assert!(result.contains("[Output truncated:"));
    }
}
