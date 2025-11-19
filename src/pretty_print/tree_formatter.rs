/// Helper struct for formatting tree structures with box-drawing characters.
///
/// Manages indentation, prefixes, and provides utility methods for common
/// formatting patterns like displaying fields, vectors, and options.

use std::fmt::{self, Write};
use crate::Span;
use super::TreeDisplay;

pub struct TreeFormatter {
    buffer: String,
    indent_stack: Vec<bool>, // true = continue line (│), false = last item (space)
}

impl TreeFormatter {
    /// Create a new TreeFormatter.
    pub fn new() -> Self {
        TreeFormatter {
            buffer: String::new(),
            indent_stack: Vec::new(),
        }
    }

    /// Get the formatted output.
    pub fn finish(self) -> String {
        self.buffer
    }

    /// Write a line with proper indentation and prefix.
    fn write_line(&mut self, is_last: bool, content: &str) -> fmt::Result {
        // Write indentation
        for &continues in &self.indent_stack {
            if continues {
                write!(self.buffer, "│   ")?;
            } else {
                write!(self.buffer, "    ")?;
            }
        }

        // Write prefix
        if is_last {
            write!(self.buffer, "└── ")?;
        } else {
            write!(self.buffer, "├── ")?;
        }

        // Write content
        writeln!(self.buffer, "{}", content)?;
        Ok(())
    }

    /// Display a root node (no indentation prefix).
    /// Only use this for the very top-level node.
    pub fn root(&mut self, content: &str) -> fmt::Result {
        if self.indent_stack.is_empty() {
            // Top level - no tree prefix
            writeln!(self.buffer, "{}", content)?;
        } else {
            // Nested node - write with tree structure
            self.node(content)?;
        }
        Ok(())
    }

    /// Display a node (with tree structure based on current indent level).
    fn node(&mut self, content: &str) -> fmt::Result {
        // Write indentation
        for &continues in &self.indent_stack {
            if continues {
                write!(self.buffer, "│   ")?;
            } else {
                write!(self.buffer, "    ")?;
            }
        }

        // Write content without prefix (child nodes don't get ├── or └──)
        writeln!(self.buffer, "{}", content)?;
        Ok(())
    }

    /// Display a field with a name and value.
    pub fn field(&mut self, is_last: bool, name: &str, value: &str) -> fmt::Result {
        self.write_line(is_last, &format!("{}: {}", name, value))
    }

    /// Display a field with a child node that implements TreeDisplay.
    pub fn field_with_child<T: TreeDisplay>(
        &mut self,
        is_last: bool,
        name: &str,
        value: &T,
        source: &str,
    ) -> fmt::Result {
        self.write_line(is_last, name)?;
        self.indent_stack.push(!is_last);
        value.tree_display(self, source)?;
        self.indent_stack.pop();
        Ok(())
    }

    /// Display an optional field.
    pub fn field_option<T: TreeDisplay>(
        &mut self,
        is_last: bool,
        name: &str,
        value: &Option<T>,
        source: &str,
    ) -> fmt::Result {
        match value {
            Some(v) => {
                self.write_line(is_last, &format!("{}: Some", name))?;
                self.indent_stack.push(!is_last);
                v.tree_display(self, source)?;
                self.indent_stack.pop();
            }
            None => {
                self.write_line(is_last, &format!("{}: None", name))?;
            }
        }
        Ok(())
    }

    /// Display a vector of items.
    pub fn field_vec<T: TreeDisplay>(
        &mut self,
        is_last: bool,
        name: &str,
        items: &[T],
        source: &str,
    ) -> fmt::Result {
        self.write_line(is_last, &format!("{}: Vec ({} items)", name, items.len()))?;
        if !items.is_empty() {
            self.indent_stack.push(!is_last);
            for (i, item) in items.iter().enumerate() {
                let is_last_item = i == items.len() - 1;
                self.write_line(is_last_item, &format!("[{}]", i))?;
                self.indent_stack.push(!is_last_item);
                item.tree_display(self, source)?;
                self.indent_stack.pop();
            }
            self.indent_stack.pop();
        }
        Ok(())
    }

    /// Display an array of items.
    pub fn field_array<T: TreeDisplay, const N: usize>(
        &mut self,
        is_last: bool,
        name: &str,
        items: &[T; N],
        source: &str,
    ) -> fmt::Result {
        self.field_vec(is_last, name, items, source)
    }

    /// Display a tuple of 2 items.
    pub fn field_tuple2<T1: TreeDisplay, T2: TreeDisplay>(
        &mut self,
        is_last: bool,
        name: &str,
        value: &(T1, T2),
        source: &str,
    ) -> fmt::Result {
        self.write_line(is_last, &format!("{}: (2 items)", name))?;
        self.indent_stack.push(!is_last);

        self.write_line(false, "[0]")?;
        self.indent_stack.push(true);
        value.0.tree_display(self, source)?;
        self.indent_stack.pop();

        self.write_line(true, "[1]")?;
        self.indent_stack.push(false);
        value.1.tree_display(self, source)?;
        self.indent_stack.pop();

        self.indent_stack.pop();
        Ok(())
    }

    /// Extract and format raw source text from a span.
    ///
    /// If the text is longer than 40 characters, it will be truncated to show
    /// the first 20 and last 20 characters with "..." in the middle.
    /// Newlines and other whitespace are normalized to single spaces.
    pub fn format_raw(&self, span: Span, source: &str) -> String {
        let raw = extract_span_text(span, source);
        // Normalize whitespace: replace all whitespace sequences with a single space
        let normalized = raw.split_whitespace().collect::<Vec<_>>().join(" ");
        truncate_with_ellipsis(&normalized, 40)
    }
}

impl Default for TreeFormatter {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract the text corresponding to a span from the source.
fn extract_span_text(span: Span, source: &str) -> String {
    if span.start <= span.end && span.end <= source.len() {
        source[span.start..span.end].to_string()
    } else {
        format!("<invalid span: {}..{}>", span.start, span.end)
    }
}

/// Truncate a string to max_len, showing head and tail with "..." in middle.
///
/// If the string is longer than max_len, shows first max_len/2 characters,
/// "...", then last max_len/2 characters.
pub(crate) fn truncate_with_ellipsis(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        let half = max_len / 2;
        let start = &s[..half.min(s.len())];
        let end_start = s.len().saturating_sub(half);
        let end = &s[end_start..];
        format!("{}...{}", start, end)
    }
}
