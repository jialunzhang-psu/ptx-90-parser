//! Original PTX specification:
//!
//! lop3.b32 d, a, b, c, immLut;
//! lop3.BoolOp.b32 d|p, a, b, c, immLut, q;
//! .BoolOp   = { .or , .and };

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
    use crate::r#type::instruction::lop3::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Boolop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".and"), |_, _span| Boolop::And),
                map(string_p(".or"), |_, _span| Boolop::Or)
            )
        }
    }

    impl PtxParser for Lop3B32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("lop3"),
                    string_p(".b32"),
                    GeneralOperand::parse(),
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
                |(_, b32, d, _, a, _, b, _, c, _, immlut, _), span| {
                    ok!(Lop3B32 {
                        b32 = b32,
                        d = d,
                        a = a,
                        b = b,
                        c = c,
                        immlut = immlut,

                    })
                },
            )
        }
    }

    impl PtxParser for Lop3BoolopB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("lop3"),
                    Boolop::parse(),
                    string_p(".b32"),
                    GeneralOperand::parse(),
                    pipe_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
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
                |(_, boolop, b32, d, _, p, _, a, _, b, _, c, _, immlut, _, q, _), span| {
                    ok!(Lop3BoolopB32 {
                        boolop = boolop,
                        b32 = b32,
                        d = d,
                        p = p,
                        a = a,
                        b = b,
                        c = c,
                        immlut = immlut,
                        q = q,

                    })
                },
            )
        }
    }
}
