//! Original PTX specification:
//!
//! sqrt.approx{.ftz}.f32  d, a; // fast, approximate square root
//! sqrt.rnd{.ftz}.f32     d, a; // IEEE 754 compliant rounding
//! sqrt.rnd.f64           d, a; // IEEE 754 compliant rounding
//! .rnd = { .rn, .rz, .rm, .rp };

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
    use crate::r#type::instruction::sqrt::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Rnd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".rn"), |_, _span| Rnd::Rn),
                map(string_p(".rz"), |_, _span| Rnd::Rz),
                map(string_p(".rm"), |_, _span| Rnd::Rm),
                map(string_p(".rp"), |_, _span| Rnd::Rp)
            )
        }
    }

    impl PtxParser for SqrtApproxFtzF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sqrt"),
                    string_p(".approx"),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, approx, ftz, f32, d, _, a, _), span| {
                    ok!(SqrtApproxFtzF32 {
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

    impl PtxParser for SqrtRndFtzF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sqrt"),
                    Rnd::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, ftz, f32, d, _, a, _), span| {
                    ok!(SqrtRndFtzF32 {
                        rnd = rnd,
                        ftz = ftz,
                        f32 = f32,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for SqrtRndF64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sqrt"),
                    Rnd::parse(),
                    string_p(".f64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, f64, d, _, a, _), span| {
                    ok!(SqrtRndF64 {
                        rnd = rnd,
                        f64 = f64,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }
}
