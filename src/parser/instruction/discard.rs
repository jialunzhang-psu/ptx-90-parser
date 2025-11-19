//! Original PTX specification:
//!
//! discard{.global}.level  [a], size;
//! .level = { .L2 };

#![allow(unused)]

use crate::parser::{
    PtxParseError, PtxParser, PtxTokenStream, Span,
    util::{
        between, comma_p, directive_p, exclamation_p, lbracket_p, lparen_p, map, minus_p, optional,
        pipe_p, rbracket_p, rparen_p, semicolon_p, sep_by, string_p, try_map,
    },
};
use crate::r#type::common::*;
use crate::{alt, ok, seq_n};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::discard::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Level {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".L2"), |_, _span| Level::L2))
        }
    }

    impl PtxParser for DiscardGlobalLevel {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("discard"),
                    map(optional(string_p(".global")), |value, _| value.is_some()),
                    Level::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, global, level, a, _, size, _), span| {
                    ok!(DiscardGlobalLevel {
                        global = global,
                        level = level,
                        a = a,
                        size = size,

                    })
                },
            )
        }
    }
}
