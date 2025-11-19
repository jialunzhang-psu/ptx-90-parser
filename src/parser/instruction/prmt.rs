//! Original PTX specification:
//!
//! prmt.b32{.mode}  d, a, b, c;
//! .mode = { .f4e, .b4e, .rc8, .ecl, .ecr, .rc16 };

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
    use crate::r#type::instruction::prmt::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Mode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".rc16"), |_, _span| Mode::Rc16),
                map(string_p(".f4e"), |_, _span| Mode::F4e),
                map(string_p(".b4e"), |_, _span| Mode::B4e),
                map(string_p(".rc8"), |_, _span| Mode::Rc8),
                map(string_p(".ecl"), |_, _span| Mode::Ecl),
                map(string_p(".ecr"), |_, _span| Mode::Ecr)
            )
        }
    }

    impl PtxParser for PrmtB32Mode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("prmt"),
                    string_p(".b32"),
                    optional(Mode::parse()),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, b32, mode, d, _, a, _, b, _, c, _), span| {
                    ok!(PrmtB32Mode {
                        b32 = b32,
                        mode = mode,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }
}
