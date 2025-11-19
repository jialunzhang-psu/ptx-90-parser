//! Original PTX specification:
//!
//! bmsk.mode.b32  d, a, b;
//! .mode = { .clamp, .wrap };

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
    use crate::r#type::instruction::bmsk::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Mode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".clamp"), |_, _span| Mode::Clamp),
                map(string_p(".wrap"), |_, _span| Mode::Wrap)
            )
        }
    }

    impl PtxParser for BmskModeB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("bmsk"),
                    Mode::parse(),
                    string_p(".b32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, mode, b32, d, _, a, _, b, _), span| {
                    ok!(BmskModeB32 {
                        mode = mode,
                        b32 = b32,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }
}
