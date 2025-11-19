//! Original PTX specification:
//!
//! istypep.type   p, a;  // result is .pred
//! .type = { .texref, .samplerref, .surfref };

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
    use crate::r#type::instruction::istypep::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".samplerref"), |_, _span| Type::Samplerref),
                map(string_p(".surfref"), |_, _span| Type::Surfref),
                map(string_p(".texref"), |_, _span| Type::Texref)
            )
        }
    }

    impl PtxParser for IstypepType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("istypep"),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, type_, p, _, a, _), span| {
                    ok!(IstypepType {
                        type_ = type_,
                        p = p,
                        a = a,

                    })
                },
            )
        }
    }
}
