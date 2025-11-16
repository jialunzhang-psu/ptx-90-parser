//! PTX (Parallel Thread Execution) parser for NVIDIA GPU assembly language.
//!
//! This library provides a complete parser for PTX assembly code, including:
//! - Lexical analysis (tokenization)
//! - Syntactic parsing into structured types
//! - Unparsing back to PTX source code
//!
//! # Quick Start
//!
//! ```no_run
//! use ptx_parser::{parse_ptx};
//! use ptx_parser::r#type::{Module, ModuleDirective, Instruction};
//!
//! let source = r#"
//!     .version 8.5
//!     .target sm_90
//!     .address_size 64
//!     
//!     .entry kernel() {
//!         add.s32 %r1, %r2, %r3;
//!         ret;
//!     }
//! "#;
//!
//! let module: Module = parse_ptx(source).expect("Failed to parse PTX");
//! println!("Parsed {} directives", module.directives.len());
//! ```
//!
//! # Type Organization
//!
//! All types are re-exported at `ptx_parser::r#type::*` for easy access:
//!
//! ```rust
//! use ptx_parser::r#type::{
//!     Module,              // Root AST node
//!     Instruction,         // Instruction with label/predicate
//!     Predicate,           // Predicate guard
//!     Operand,             // Operand types
//!     EntryFunctionDirective,
//!     FuncFunctionDirective,
//!     // ... all other types
//! };
//! ```
//!
//! Instruction variants are under `instruction::`:
//!
//! ```rust
//! use ptx_parser::r#type::instruction::{Inst, add, mov};
//! ```

// Internal modules - not part of public API
mod lexer;
mod parser;
pub mod span;
mod unlexer;
mod unparser;

// Type definitions - AST nodes (public)
pub mod r#type;

// Re-export derive macro for the `Spanned` trait so downstream crates can use it.
pub use span_derive::Spanned;

// Re-export commonly used items for convenience

// Lexer exports
pub use lexer::{LexError, PtxToken, Span, tokenize};

// Parser exports
pub use parser::{
    ParseErrorKind, PtxParseError, PtxParser, PtxTokenStream, StreamPosition, parse_ptx,
};

// Unlexer exports
pub use unlexer::PtxUnlexer;

// Unparser exports
pub use unparser::PtxUnparser;
