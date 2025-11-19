//! Original PTX specification:
//!
//! sub.type       d, a, b;
//! sub{.sat}.s32  d, a, b;     // .sat applies only to .s32
//! .type = { .u16, .u32, .u64,
//! .s16, .s32, .s64 };
//! --------------------------------------------
//! sub{.rnd}{.ftz}{.sat}.f32  d, a, b;
//! sub{.rnd}{.ftz}.f32x2      d, a, b;
//! sub{.rnd}.f64              d, a, b;
//! .rnd = { .rn, .rz, .rm, .rp };
//! --------------------------------------------
//! sub{.rnd}{.ftz}{.sat}.f16   d, a, b;
//! sub{.rnd}{.ftz}{.sat}.f16x2 d, a, b;
//! sub{.rnd}.bf16   d, a, b;
//! sub{.rnd}.bf16x2 d, a, b;
//! .rnd = { .rn };
//! --------------------------------------------
//! sub{.rnd}{.sat}.f32.atype  d, a, c;
//! .atype = { .f16, .bf16};
//! .rnd   = { .rn, .rz, .rm, .rp };

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
    use crate::r#type::instruction::sub::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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

    impl PtxParser for SubType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sub"),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, type_, d, _, a, _, b, _), span| {
                    ok!(SubType {
                        type_ = type_,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for SubSatS32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sub"),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    string_p(".s32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, sat, s32, d, _, a, _, b, _), span| {
                    ok!(SubSatS32 {
                        sat = sat,
                        s32 = s32,
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
    use crate::r#type::instruction::sub::section_1::*;

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

    impl PtxParser for SubRndFtzSatF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sub"),
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
                    ok!(SubRndFtzSatF32 {
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

    impl PtxParser for SubRndFtzF32x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sub"),
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
                    ok!(SubRndFtzF32x2 {
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

    impl PtxParser for SubRndF64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sub"),
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
                    ok!(SubRndF64 {
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
    use crate::r#type::instruction::sub::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Rnd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".rn"), |_, _span| Rnd::Rn))
        }
    }

    impl PtxParser for SubRndFtzSatF16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sub"),
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
                    ok!(SubRndFtzSatF16 {
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

    impl PtxParser for SubRndFtzSatF16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sub"),
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
                    ok!(SubRndFtzSatF16x2 {
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

    impl PtxParser for SubRndBf16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sub"),
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
                    ok!(SubRndBf16 {
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

    impl PtxParser for SubRndBf16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sub"),
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
                    ok!(SubRndBf16x2 {
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

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::sub::section_3::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".bf16"), |_, _span| Atype::Bf16),
                map(string_p(".f16"), |_, _span| Atype::F16)
            )
        }
    }

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

    impl PtxParser for SubRndSatF32Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sub"),
                    optional(Rnd::parse()),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    string_p(".f32"),
                    Atype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, sat, f32, atype, d, _, a, _, c, _), span| {
                    ok!(SubRndSatF32Atype {
                        rnd = rnd,
                        sat = sat,
                        f32 = f32,
                        atype = atype,
                        d = d,
                        a = a,
                        c = c,

                    })
                },
            )
        }
    }
}
