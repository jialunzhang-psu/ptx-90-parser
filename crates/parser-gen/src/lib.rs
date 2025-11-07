pub mod analyzer;
mod formatting;
mod lexer;
pub mod naming;
pub mod parser_generator;
mod spec_parser;
pub mod r#type;
pub mod type_generator;
pub mod unparser_generator;

pub use lexer::{PtxSpecToken, Span, tokenize};
pub use spec_parser::{SpecParseError, parse_spec, parse_spec_with_name};

/// Parse a PTX specification and render the resulting AST using the
/// Tree-sitter style pretty printer helper on [`TopLevel`].
pub fn parse_spec_to_treesitter(source: &str) -> Result<String, SpecParseError> {
    let tops = parse_spec(source)?;
    Ok(tops
        .into_iter()
        .map(|top| top.to_treesitter_pretty())
        .collect::<Vec<_>>()
        .join("\n"))
}
