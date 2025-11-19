//! Original PTX specification:
//!
//! tanh.approx.type d, a;
//! .type = {.f16, .f32, .f16x2, .bf16, .bf16x2};

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
    use crate::r#type::instruction::tanh::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".bf16x2"), |_, _span| Type::Bf16x2),
                map(string_p(".f16x2"), |_, _span| Type::F16x2),
                map(string_p(".bf16"), |_, _span| Type::Bf16),
                map(string_p(".f16"), |_, _span| Type::F16),
                map(string_p(".f32"), |_, _span| Type::F32)
            )
        }
    }

    impl PtxParser for TanhApproxType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tanh"),
                    string_p(".approx"),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, approx, type_, d, _, a, _), span| {
                    ok!(TanhApproxType {
                        approx = approx,
                        type_ = type_,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }
}
