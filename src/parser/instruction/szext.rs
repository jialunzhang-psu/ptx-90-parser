//! Original PTX specification:
//!
//! szext.mode.type  d, a, b;
//! .mode = { .clamp, .wrap };
//! .type = { .u32, .s32 };

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
    use crate::r#type::instruction::szext::section_0::*;

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

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".s32"), |_, _span| Type::S32)
            )
        }
    }

    impl PtxParser for SzextModeType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("szext"),
                    Mode::parse(),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, mode, type_, d, _, a, _, b, _), span| {
                    ok!(SzextModeType {
                        mode = mode,
                        type_ = type_,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }
}
