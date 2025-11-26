use super::TreeFormatter;
/// Trait for displaying AST nodes in a tree structure.
///
/// Types implementing this trait can be displayed as a tree with proper
/// indentation and box-drawing characters, showing both their structure
/// and the original source text.
use std::fmt;

pub trait TreeDisplay {
    /// Display this node in tree format.
    ///
    /// # Arguments
    /// * `f` - The formatter to write to
    /// * `source` - The original source code string for extracting raw text from spans
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> fmt::Result;
}
