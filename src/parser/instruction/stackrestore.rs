//! Original PTX specification:
//!
//! stackrestore.type  a;
//! .type = { .u32, .u64 };

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
    use crate::r#type::instruction::stackrestore::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".u64"), |_, _span| Type::U64)
            )
        }
    }

    impl PtxParser for StackrestoreType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("stackrestore"),
                    Type::parse(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, type_, a, _), span| {
                    ok!(StackrestoreType {
                        type_ = type_,
                        a = a,

                    })
                },
            )
        }
    }
}
