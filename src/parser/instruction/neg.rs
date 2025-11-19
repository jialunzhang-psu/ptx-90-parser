//! Original PTX specification:
//!
//! neg.type  d, a;
//! .type = { .s16, .s32, .s64 };
//!
//! neg{.ftz}.f32  d, a;
//! neg.f64        d, a;
//!
//! neg{.ftz}.f16    d, a;
//! neg{.ftz}.f16x2  d, a;
//! neg.bf16         d, a;
//! neg.bf16x2       d, a;

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
    use crate::r#type::instruction::neg::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".s16"), |_, _span| Type::S16),
                map(string_p(".s32"), |_, _span| Type::S32),
                map(string_p(".s64"), |_, _span| Type::S64)
            )
        }
    }

    impl PtxParser for NegType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("neg"),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, type_, d, _, a, _), span| {
                    ok!(NegType {
                        type_ = type_,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for NegFtzF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("neg"),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, ftz, f32, d, _, a, _), span| {
                    ok!(NegFtzF32 {
                        ftz = ftz,
                        f32 = f32,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for NegF64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("neg"),
                    string_p(".f64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, f64, d, _, a, _), span| {
                    ok!(NegF64 {
                        f64 = f64,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for NegFtzF16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("neg"),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, ftz, f16, d, _, a, _), span| {
                    ok!(NegFtzF16 {
                        ftz = ftz,
                        f16 = f16,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for NegFtzF16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("neg"),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, ftz, f16x2, d, _, a, _), span| {
                    ok!(NegFtzF16x2 {
                        ftz = ftz,
                        f16x2 = f16x2,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for NegBf16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("neg"),
                    string_p(".bf16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, bf16, d, _, a, _), span| {
                    ok!(NegBf16 {
                        bf16 = bf16,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for NegBf16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("neg"),
                    string_p(".bf16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, bf16x2, d, _, a, _), span| {
                    ok!(NegBf16x2 {
                        bf16x2 = bf16x2,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }
}
