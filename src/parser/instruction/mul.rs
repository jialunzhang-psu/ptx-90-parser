//! Original PTX specification:
//!
//! mul.mode.type  d, a, b;
//! .mode = { .hi, .lo, .wide };
//! .type = { .u16, .u32, .u64,
//! .s16, .s32, .s64 };
//! --------------------------------------------
//! mul{.rnd}{.ftz}{.sat}.f32  d, a, b;
//! mul{.rnd}{.ftz}.f32x2      d, a, b;
//! mul{.rnd}.f64              d, a, b;
//! .rnd = { .rn, .rz, .rm, .rp };
//! --------------------------------------------
//! mul{.rnd}{.ftz}{.sat}.f16   d, a, b;
//! mul{.rnd}{.ftz}{.sat}.f16x2 d, a, b;
//! mul{.rnd}.bf16   d, a, b;
//! mul{.rnd}.bf16x2 d, a, b;
//! .rnd = { .rn };

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
    use crate::r#type::instruction::mul::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Mode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".wide"), |_, _span| Mode::Wide),
                map(string_p(".hi"), |_, _span| Mode::Hi),
                map(string_p(".lo"), |_, _span| Mode::Lo)
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

    impl PtxParser for MulModeType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mul"),
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
                    ok!(MulModeType {
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

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::mul::section_1::*;

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

    impl PtxParser for MulRndFtzSatF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mul"),
                    optional(Rnd::parse()),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, ftz, sat, f32, d, _, a, _, b, _), span| {
                    ok!(MulRndFtzSatF32 {
                        rnd = rnd,
                        ftz = ftz,
                        sat = sat,
                        f32 = f32,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for MulRndFtzF32x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mul"),
                    optional(Rnd::parse()),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f32x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, ftz, f32x2, d, _, a, _, b, _), span| {
                    ok!(MulRndFtzF32x2 {
                        rnd = rnd,
                        ftz = ftz,
                        f32x2 = f32x2,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for MulRndF64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mul"),
                    optional(Rnd::parse()),
                    string_p(".f64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, f64, d, _, a, _, b, _), span| {
                    ok!(MulRndF64 {
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

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::mul::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Rnd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".rn"), |_, _span| Rnd::Rn))
        }
    }

    impl PtxParser for MulRndFtzSatF16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mul"),
                    optional(Rnd::parse()),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    string_p(".f16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, ftz, sat, f16, d, _, a, _, b, _), span| {
                    ok!(MulRndFtzSatF16 {
                        rnd = rnd,
                        ftz = ftz,
                        sat = sat,
                        f16 = f16,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for MulRndFtzSatF16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mul"),
                    optional(Rnd::parse()),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    string_p(".f16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, ftz, sat, f16x2, d, _, a, _, b, _), span| {
                    ok!(MulRndFtzSatF16x2 {
                        rnd = rnd,
                        ftz = ftz,
                        sat = sat,
                        f16x2 = f16x2,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for MulRndBf16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mul"),
                    optional(Rnd::parse()),
                    string_p(".bf16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, bf16, d, _, a, _, b, _), span| {
                    ok!(MulRndBf16 {
                        rnd = rnd,
                        bf16 = bf16,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for MulRndBf16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mul"),
                    optional(Rnd::parse()),
                    string_p(".bf16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, bf16x2, d, _, a, _, b, _), span| {
                    ok!(MulRndBf16x2 {
                        rnd = rnd,
                        bf16x2 = bf16x2,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }
}
