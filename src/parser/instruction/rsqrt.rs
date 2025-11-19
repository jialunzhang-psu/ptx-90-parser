//! Original PTX specification:
//!
//! rsqrt.approx{.ftz}.f32  d, a;
//! rsqrt.approx.f64        d, a;

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
    use crate::r#type::instruction::rsqrt::section_0::*;

    impl PtxParser for RsqrtApproxFtzF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("rsqrt"),
                    string_p(".approx"),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, approx, ftz, f32, d, _, a, _), span| {
                    ok!(RsqrtApproxFtzF32 {
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

    impl PtxParser for RsqrtApproxF64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("rsqrt"),
                    string_p(".approx"),
                    string_p(".f64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, approx, f64, d, _, a, _), span| {
                    ok!(RsqrtApproxF64 {
                        approx = approx,
                        f64 = f64,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }
}
