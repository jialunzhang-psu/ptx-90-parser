//! Original PTX specification:
//!
//! shf.l.mode.b32  d, a, b, c;  // left shift
//! shf.r.mode.b32  d, a, b, c;  // right shift
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
    use crate::r#type::instruction::shf::section_0::*;

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

    impl PtxParser for ShfLModeB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("shf"),
                    string_p(".l"),
                    Mode::parse(),
                    string_p(".b32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, l, mode, b32, d, _, a, _, b, _, c, _), span| {
                    ok!(ShfLModeB32 {
                        l = l,
                        mode = mode,
                        b32 = b32,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for ShfRModeB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("shf"),
                    string_p(".r"),
                    Mode::parse(),
                    string_p(".b32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, r, mode, b32, d, _, a, _, b, _, c, _), span| {
                    ok!(ShfRModeB32 {
                        r = r,
                        mode = mode,
                        b32 = b32,
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
