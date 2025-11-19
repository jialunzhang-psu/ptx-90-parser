//! Original PTX specification:
//!
//! div.type  d, a, b;
//! .type = { .u16, .u32, .u64,
//! .s16, .s32, .s64 };
//!
//! div.approx{.ftz}.f32  d, a, b;  // fast, approximate divide
//! div.full{.ftz}.f32    d, a, b;  // full-range approximate divide
//! div.rnd{.ftz}.f32     d, a, b;  // IEEE 754 compliant rounding
//! div.rnd.f64           d, a, b;  // IEEE 754 compliant rounding
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
    use crate::r#type::instruction::div::section_0::*;

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

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u16"), |_, _span| Type::U16),
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".u64"), |_, _span| Type::U64),
                map(string_p(".s16"), |_, _span| Type::S16),
                map(string_p(".s32"), |_, _span| Type::S32),
                map(string_p(".s64"), |_, _span| Type::S64)
            )
        }
    }

    impl PtxParser for DivType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("div"),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, type_, d, _, a, _, b, _), span| {
                    ok!(DivType {
                        type_ = type_,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for DivApproxFtzF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("div"),
                    string_p(".approx"),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, approx, ftz, f32, d, _, a, _, b, _), span| {
                    ok!(DivApproxFtzF32 {
                        approx = approx,
                        ftz = ftz,
                        f32 = f32,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for DivFullFtzF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("div"),
                    string_p(".full"),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, full, ftz, f32, d, _, a, _, b, _), span| {
                    ok!(DivFullFtzF32 {
                        full = full,
                        ftz = ftz,
                        f32 = f32,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for DivRndFtzF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("div"),
                    Rnd::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, ftz, f32, d, _, a, _, b, _), span| {
                    ok!(DivRndFtzF32 {
                        rnd = rnd,
                        ftz = ftz,
                        f32 = f32,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for DivRndF64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("div"),
                    Rnd::parse(),
                    string_p(".f64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, f64, d, _, a, _, b, _), span| {
                    ok!(DivRndF64 {
                        rnd = rnd,
                        f64 = f64,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }
}
