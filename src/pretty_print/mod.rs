mod primitives;
/// Pretty-print module for displaying PTX AST nodes in a tree structure.
///
/// This module provides the `TreeDisplay` trait and supporting infrastructure
/// for displaying PTX AST nodes in a hierarchical tree format with box-drawing
/// characters. Each node displays its type, fields, and the raw source text
/// from its span.
///
/// # Module Organization
///
/// - `tree_display.rs` - TreeDisplay trait definition
/// - `tree_formatter.rs` - TreeFormatter helper for formatting output
/// - `primitives.rs` - TreeDisplay implementations for primitive types
/// - `impls_*.rs` - TreeDisplay implementations for PTX AST types
///
/// All TreeDisplay implementations are located in this module to keep
/// the type definitions clean and separate formatting concerns.
mod tree_display;
mod tree_formatter;

// TreeDisplay implementations for PTX types
mod common;
mod compact;
mod function;
mod instruction;
mod module;
mod variable;

pub use compact::print_compact_module;
pub use tree_display::TreeDisplay;
pub use tree_formatter::TreeFormatter;

#[cfg(test)]
mod tests;
