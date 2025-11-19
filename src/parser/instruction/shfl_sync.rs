//! Original PTX specification:
//!
//! shfl.sync.mode.b32  d{|p}, a, b, c, membermask;
//! .mode = { .up, .down, .bfly, .idx };

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
    use crate::r#type::instruction::shfl_sync::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Mode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".down"), |_, _span| Mode::Down),
                map(string_p(".bfly"), |_, _span| Mode::Bfly),
                map(string_p(".idx"), |_, _span| Mode::Idx),
                map(string_p(".up"), |_, _span| Mode::Up)
            )
        }
    }

    impl PtxParser for ShflSyncModeB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("shfl"),
                    string_p(".sync"),
                    Mode::parse(),
                    string_p(".b32"),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(pipe_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, sync, mode, b32, d, p, _, a, _, b, _, c, _, membermask, _), span| {
                    ok!(ShflSyncModeB32 {
                        sync = sync,
                        mode = mode,
                        b32 = b32,
                        d = d,
                        p = p,
                        a = a,
                        b = b,
                        c = c,
                        membermask = membermask,

                    })
                },
            )
        }
    }
}
