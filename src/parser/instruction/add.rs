//! Original PTX specification:
//!
//! add.type       d, a, b;
//! add{.sat}.s32  d, a, b;     // .sat applies only to .s32
//! .type = { .u16, .u32, .u64,
//! .s16, .s32, .s64,
//! .u16x2, .s16x2 };
//! -------------------------------------------
//! add{.rnd}{.ftz}{.sat}.f32  d, a, b;
//! add{.rnd}{.ftz}.f32x2      d, a, b;
//! add{.rnd}.f64              d, a, b;
//! .rnd = { .rn, .rz, .rm, .rp };
//! --------------------------------------------
//! add{.rnd}{.ftz}{.sat}.f16   d, a, b;
//! add{.rnd}{.ftz}{.sat}.f16x2 d, a, b;
//! add{.rnd}.bf16   d, a, b;
//! add{.rnd}.bf16x2 d, a, b;
//! .rnd = { .rn };
//! --------------------------------------------
//! add{.rnd}{.sat}.f32.atype  d, a, c;
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
    use crate::r#type::instruction::add::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u16x2"), |_, _span| Type::U16x2),
                map(string_p(".s16x2"), |_, _span| Type::S16x2),
                map(string_p(".u16"), |_, _span| Type::U16),
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".u64"), |_, _span| Type::U64),
                map(string_p(".s16"), |_, _span| Type::S16),
                map(string_p(".s32"), |_, _span| Type::S32),
                map(string_p(".s64"), |_, _span| Type::S64)
            )
        }
    }

    impl PtxParser for AddType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("add"),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, type_, d, _, a, _, b, _), span| {
                    ok!(AddType {
                        type_ = type_,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for AddSatS32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("add"),
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
                    ok!(AddSatS32 {
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
    use crate::r#type::instruction::add::section_1::*;

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

    impl PtxParser for AddRndFtzSatF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("add"),
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
                    ok!(AddRndFtzSatF32 {
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

    impl PtxParser for AddRndFtzF32x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("add"),
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
                    ok!(AddRndFtzF32x2 {
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

    impl PtxParser for AddRndF64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("add"),
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
                    ok!(AddRndF64 {
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
    use crate::r#type::instruction::add::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Rnd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".rn"), |_, _span| Rnd::Rn))
        }
    }

    impl PtxParser for AddRndFtzSatF16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("add"),
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
                    ok!(AddRndFtzSatF16 {
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

    impl PtxParser for AddRndFtzSatF16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("add"),
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
                    ok!(AddRndFtzSatF16x2 {
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

    impl PtxParser for AddRndBf16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("add"),
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
                    ok!(AddRndBf16 {
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

    impl PtxParser for AddRndBf16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("add"),
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
                    ok!(AddRndBf16x2 {
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
    use crate::r#type::instruction::add::section_3::*;

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

    impl PtxParser for AddRndSatF32Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("add"),
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
                    ok!(AddRndSatF32Atype {
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
