//! Original PTX specification:
//!
//! ex2.approx{.ftz}.f32  d, a;
//!
//! ex2.approx.atype     d, a;
//! ex2.approx.ftz.btype d, a;
//! .atype = { .f16,  .f16x2};
//! .btype = { .bf16, .bf16x2};

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
    use crate::r#type::instruction::ex2::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".f16x2"), |_, _span| Atype::F16x2),
                map(string_p(".f16"), |_, _span| Atype::F16)
            )
        }
    }

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".bf16x2"), |_, _span| Btype::Bf16x2),
                map(string_p(".bf16"), |_, _span| Btype::Bf16)
            )
        }
    }

    impl PtxParser for Ex2ApproxFtzF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ex2"),
                    string_p(".approx"),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, approx, ftz, f32, d, _, a, _), span| {
                    ok!(Ex2ApproxFtzF32 {
                        approx = approx,
                        ftz = ftz,
                        f32 = f32,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for Ex2ApproxAtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ex2"),
                    string_p(".approx"),
                    Atype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, approx, atype, d, _, a, _), span| {
                    ok!(Ex2ApproxAtype {
                        approx = approx,
                        atype = atype,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for Ex2ApproxFtzBtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ex2"),
                    string_p(".approx"),
                    string_p(".ftz"),
                    Btype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, approx, ftz, btype, d, _, a, _), span| {
                    ok!(Ex2ApproxFtzBtype {
                        approx = approx,
                        ftz = ftz,
                        btype = btype,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }
}
