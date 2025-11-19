//! Original PTX specification:
//!
//! abs.type  d, a;
//! .type = { .s16, .s32, .s64 };
//!
//! abs{.ftz}.f32  d, a;
//! abs.f64        d, a;
//!
//! abs{.ftz}.f16    d, a;
//! abs{.ftz}.f16x2  d, a;
//! abs.bf16         d, a;
//! abs.bf16x2       d, a;

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
    use crate::r#type::instruction::abs::section_0::*;

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

    impl PtxParser for AbsType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("abs"),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, type_, d, _, a, _), span| {
                    ok!(AbsType {
                        type_ = type_,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for AbsFtzF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("abs"),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, ftz, f32, d, _, a, _), span| {
                    ok!(AbsFtzF32 {
                        ftz = ftz,
                        f32 = f32,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for AbsF64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("abs"),
                    string_p(".f64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, f64, d, _, a, _), span| {
                    ok!(AbsF64 {
                        f64 = f64,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for AbsFtzF16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("abs"),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, ftz, f16, d, _, a, _), span| {
                    ok!(AbsFtzF16 {
                        ftz = ftz,
                        f16 = f16,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for AbsFtzF16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("abs"),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, ftz, f16x2, d, _, a, _), span| {
                    ok!(AbsFtzF16x2 {
                        ftz = ftz,
                        f16x2 = f16x2,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for AbsBf16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("abs"),
                    string_p(".bf16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, bf16, d, _, a, _), span| {
                    ok!(AbsBf16 {
                        bf16 = bf16,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for AbsBf16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("abs"),
                    string_p(".bf16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, bf16x2, d, _, a, _), span| {
                    ok!(AbsBf16x2 {
                        bf16x2 = bf16x2,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }
}
