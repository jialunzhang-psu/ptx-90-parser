//! Original PTX specification:
//!
//! cvt{.irnd}{.ftz}{.sat}.dtype.atype         d, a;  // integer rounding
//! cvt{.frnd}{.ftz}{.sat}.dtype.atype         d, a;  // fp rounding
//! cvt.frnd2{.relu}{.satfinite}.f16.f32       d, a;
//! cvt.frnd2{.relu}{.satfinite}.f16x2.f32     d, a, b;
//! cvt.rs{.relu}{.satfinite}.f16x2.f32        d, a, b, rbits;
//! cvt.frnd2{.relu}{.satfinite}.bf16.f32      d, a;
//! cvt.frnd2{.relu}{.satfinite}.bf16x2.f32    d, a, b;
//! cvt.rs{.relu}{.satfinite}.bf16x2.f32       d, a, b, rbits;
//! cvt.rna{.satfinite}.tf32.f32               d, a;
//! cvt.frnd2{.satfinite}{.relu}.tf32.f32      d, a;
//! cvt.rn.satfinite{.relu}.f8x2type.f32       d, a, b;
//! cvt.rn.satfinite{.relu}.f8x2type.f16x2     d, a;
//! cvt.rn{.relu}.f16x2.f8x2type              d, a;
//! cvt.rs{.relu}.satfinite.f8x4type.f32       d, {a, b, e, f}, rbits;
//! cvt.rn.satfinite{.relu}.f4x2type.f32       d, a, b;
//! cvt.rn{.relu}.f16x2.f4x2type               d, a;
//! cvt.rs{.relu}.satfinite.f4x4type.f32       d, {a, b, e, f}, rbits;
//! cvt.rn.satfinite{.relu}.f6x2type.f32       d, a, b;
//! cvt.rn{.relu}.f16x2.f6x2type               d, a;
//! cvt.rs{.relu}.satfinite.f6x4type.f32       d, {a, b, e, f}, rbits;
//! cvt.frnd3{.satfinite}.ue8m0x2.f32          d, a, b;
//! cvt.frnd3{.satfinite}.ue8m0x2.bf16x2       d, a;
//! cvt.rn.bf16x2.ue8m0x2                      d, a;
//! .irnd   = { .rni, .rzi, .rmi, .rpi };
//! .frnd   = { .rn,  .rz,  .rm,  .rp  };
//! .frnd2  = { .rn,  .rz };
//! .frnd3  = { .rz,  .rp };
//! .dtype = .atype = { .u8,   .u16, .u32, .u64,
//! .s8,   .s16, .s32, .s64,
//! .bf16, .f16, .f32, .f64 };
//! .f8x2type = { .e4m3x2, .e5m2x2 };
//! .f4x2type = { .e2m1x2 };
//! .f6x2type = { .e2m3x2, .e3m2x2 };
//! .f4x4type = { .e2m1x4 };
//! .f8x4type = { .e4m3x4, .e5m2x4 };
//! .f6x4type = { .e2m3x4, .e3m2x4 };

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
    use crate::r#type::instruction::cvt::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".bf16"), |_, _span| Atype::Bf16),
                map(string_p(".u16"), |_, _span| Atype::U16),
                map(string_p(".u32"), |_, _span| Atype::U32),
                map(string_p(".u64"), |_, _span| Atype::U64),
                map(string_p(".s16"), |_, _span| Atype::S16),
                map(string_p(".s32"), |_, _span| Atype::S32),
                map(string_p(".s64"), |_, _span| Atype::S64),
                map(string_p(".f16"), |_, _span| Atype::F16),
                map(string_p(".f32"), |_, _span| Atype::F32),
                map(string_p(".f64"), |_, _span| Atype::F64),
                map(string_p(".u8"), |_, _span| Atype::U8),
                map(string_p(".s8"), |_, _span| Atype::S8)
            )
        }
    }

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".bf16"), |_, _span| Dtype::Bf16),
                map(string_p(".u16"), |_, _span| Dtype::U16),
                map(string_p(".u32"), |_, _span| Dtype::U32),
                map(string_p(".u64"), |_, _span| Dtype::U64),
                map(string_p(".s16"), |_, _span| Dtype::S16),
                map(string_p(".s32"), |_, _span| Dtype::S32),
                map(string_p(".s64"), |_, _span| Dtype::S64),
                map(string_p(".f16"), |_, _span| Dtype::F16),
                map(string_p(".f32"), |_, _span| Dtype::F32),
                map(string_p(".f64"), |_, _span| Dtype::F64),
                map(string_p(".u8"), |_, _span| Dtype::U8),
                map(string_p(".s8"), |_, _span| Dtype::S8)
            )
        }
    }

    impl PtxParser for F4x2type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".e2m1x2"), |_, _span| F4x2type::E2m1x2))
        }
    }

    impl PtxParser for F4x4type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".e2m1x4"), |_, _span| F4x4type::E2m1x4))
        }
    }

    impl PtxParser for F6x2type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".e2m3x2"), |_, _span| F6x2type::E2m3x2),
                map(string_p(".e3m2x2"), |_, _span| F6x2type::E3m2x2)
            )
        }
    }

    impl PtxParser for F6x4type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".e2m3x4"), |_, _span| F6x4type::E2m3x4),
                map(string_p(".e3m2x4"), |_, _span| F6x4type::E3m2x4)
            )
        }
    }

    impl PtxParser for F8x2type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".e4m3x2"), |_, _span| F8x2type::E4m3x2),
                map(string_p(".e5m2x2"), |_, _span| F8x2type::E5m2x2)
            )
        }
    }

    impl PtxParser for F8x4type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".e4m3x4"), |_, _span| F8x4type::E4m3x4),
                map(string_p(".e5m2x4"), |_, _span| F8x4type::E5m2x4)
            )
        }
    }

    impl PtxParser for Frnd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".rn"), |_, _span| Frnd::Rn),
                map(string_p(".rz"), |_, _span| Frnd::Rz),
                map(string_p(".rm"), |_, _span| Frnd::Rm),
                map(string_p(".rp"), |_, _span| Frnd::Rp)
            )
        }
    }

    impl PtxParser for Frnd2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".rn"), |_, _span| Frnd2::Rn),
                map(string_p(".rz"), |_, _span| Frnd2::Rz)
            )
        }
    }

    impl PtxParser for Frnd3 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".rz"), |_, _span| Frnd3::Rz),
                map(string_p(".rp"), |_, _span| Frnd3::Rp)
            )
        }
    }

    impl PtxParser for Irnd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".rni"), |_, _span| Irnd::Rni),
                map(string_p(".rzi"), |_, _span| Irnd::Rzi),
                map(string_p(".rmi"), |_, _span| Irnd::Rmi),
                map(string_p(".rpi"), |_, _span| Irnd::Rpi)
            )
        }
    }

    impl PtxParser for CvtIrndFtzSatDtypeAtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    optional(Irnd::parse()),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    Dtype::parse(),
                    Atype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, irnd, ftz, sat, dtype, atype, d, _, a, _), span| {
                    ok!(CvtIrndFtzSatDtypeAtype {
                        irnd = irnd,
                        ftz = ftz,
                        sat = sat,
                        dtype = dtype,
                        atype = atype,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtFrndFtzSatDtypeAtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    optional(Frnd::parse()),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    Dtype::parse(),
                    Atype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, frnd, ftz, sat, dtype, atype, d, _, a, _), span| {
                    ok!(CvtFrndFtzSatDtypeAtype {
                        frnd = frnd,
                        ftz = ftz,
                        sat = sat,
                        dtype = dtype,
                        atype = atype,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtFrnd2ReluSatfiniteF16F32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    Frnd2::parse(),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    map(optional(string_p(".satfinite")), |value, _| value.is_some()),
                    string_p(".f16"),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, frnd2, relu, satfinite, f16, f32, d, _, a, _), span| {
                    ok!(CvtFrnd2ReluSatfiniteF16F32 {
                        frnd2 = frnd2,
                        relu = relu,
                        satfinite = satfinite,
                        f16 = f16,
                        f32 = f32,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtFrnd2ReluSatfiniteF16x2F32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    Frnd2::parse(),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    map(optional(string_p(".satfinite")), |value, _| value.is_some()),
                    string_p(".f16x2"),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, frnd2, relu, satfinite, f16x2, f32, d, _, a, _, b, _), span| {
                    ok!(CvtFrnd2ReluSatfiniteF16x2F32 {
                        frnd2 = frnd2,
                        relu = relu,
                        satfinite = satfinite,
                        f16x2 = f16x2,
                        f32 = f32,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtRsReluSatfiniteF16x2F32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    string_p(".rs"),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    map(optional(string_p(".satfinite")), |value, _| value.is_some()),
                    string_p(".f16x2"),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rs, relu, satfinite, f16x2, f32, d, _, a, _, b, _, rbits, _), span| {
                    ok!(CvtRsReluSatfiniteF16x2F32 {
                        rs = rs,
                        relu = relu,
                        satfinite = satfinite,
                        f16x2 = f16x2,
                        f32 = f32,
                        d = d,
                        a = a,
                        b = b,
                        rbits = rbits,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtFrnd2ReluSatfiniteBf16F32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    Frnd2::parse(),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    map(optional(string_p(".satfinite")), |value, _| value.is_some()),
                    string_p(".bf16"),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, frnd2, relu, satfinite, bf16, f32, d, _, a, _), span| {
                    ok!(CvtFrnd2ReluSatfiniteBf16F32 {
                        frnd2 = frnd2,
                        relu = relu,
                        satfinite = satfinite,
                        bf16 = bf16,
                        f32 = f32,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtFrnd2ReluSatfiniteBf16x2F32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    Frnd2::parse(),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    map(optional(string_p(".satfinite")), |value, _| value.is_some()),
                    string_p(".bf16x2"),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, frnd2, relu, satfinite, bf16x2, f32, d, _, a, _, b, _), span| {
                    ok!(CvtFrnd2ReluSatfiniteBf16x2F32 {
                        frnd2 = frnd2,
                        relu = relu,
                        satfinite = satfinite,
                        bf16x2 = bf16x2,
                        f32 = f32,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtRsReluSatfiniteBf16x2F32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    string_p(".rs"),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    map(optional(string_p(".satfinite")), |value, _| value.is_some()),
                    string_p(".bf16x2"),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rs, relu, satfinite, bf16x2, f32, d, _, a, _, b, _, rbits, _), span| {
                    ok!(CvtRsReluSatfiniteBf16x2F32 {
                        rs = rs,
                        relu = relu,
                        satfinite = satfinite,
                        bf16x2 = bf16x2,
                        f32 = f32,
                        d = d,
                        a = a,
                        b = b,
                        rbits = rbits,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtRnaSatfiniteTf32F32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    string_p(".rna"),
                    map(optional(string_p(".satfinite")), |value, _| value.is_some()),
                    string_p(".tf32"),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rna, satfinite, tf32, f32, d, _, a, _), span| {
                    ok!(CvtRnaSatfiniteTf32F32 {
                        rna = rna,
                        satfinite = satfinite,
                        tf32 = tf32,
                        f32 = f32,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtFrnd2SatfiniteReluTf32F32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    Frnd2::parse(),
                    map(optional(string_p(".satfinite")), |value, _| value.is_some()),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    string_p(".tf32"),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, frnd2, satfinite, relu, tf32, f32, d, _, a, _), span| {
                    ok!(CvtFrnd2SatfiniteReluTf32F32 {
                        frnd2 = frnd2,
                        satfinite = satfinite,
                        relu = relu,
                        tf32 = tf32,
                        f32 = f32,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtRnSatfiniteReluF8x2typeF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    string_p(".rn"),
                    string_p(".satfinite"),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    F8x2type::parse(),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rn, satfinite, relu, f8x2type, f32, d, _, a, _, b, _), span| {
                    ok!(CvtRnSatfiniteReluF8x2typeF32 {
                        rn = rn,
                        satfinite = satfinite,
                        relu = relu,
                        f8x2type = f8x2type,
                        f32 = f32,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtRnSatfiniteReluF8x2typeF16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    string_p(".rn"),
                    string_p(".satfinite"),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    F8x2type::parse(),
                    string_p(".f16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rn, satfinite, relu, f8x2type, f16x2, d, _, a, _), span| {
                    ok!(CvtRnSatfiniteReluF8x2typeF16x2 {
                        rn = rn,
                        satfinite = satfinite,
                        relu = relu,
                        f8x2type = f8x2type,
                        f16x2 = f16x2,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtRnReluF16x2F8x2type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    string_p(".rn"),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    string_p(".f16x2"),
                    F8x2type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rn, relu, f16x2, f8x2type, d, _, a, _), span| {
                    ok!(CvtRnReluF16x2F8x2type {
                        rn = rn,
                        relu = relu,
                        f16x2 = f16x2,
                        f8x2type = f8x2type,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtRsReluSatfiniteF8x4typeF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    string_p(".rs"),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    string_p(".satfinite"),
                    F8x4type::parse(),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    VectorOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rs, relu, satfinite, f8x4type, f32, d, _, a, _, rbits, _), span| {
                    ok!(CvtRsReluSatfiniteF8x4typeF32 {
                        rs = rs,
                        relu = relu,
                        satfinite = satfinite,
                        f8x4type = f8x4type,
                        f32 = f32,
                        d = d,
                        a = a,
                        rbits = rbits,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtRnSatfiniteReluF4x2typeF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    string_p(".rn"),
                    string_p(".satfinite"),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    F4x2type::parse(),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rn, satfinite, relu, f4x2type, f32, d, _, a, _, b, _), span| {
                    ok!(CvtRnSatfiniteReluF4x2typeF32 {
                        rn = rn,
                        satfinite = satfinite,
                        relu = relu,
                        f4x2type = f4x2type,
                        f32 = f32,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtRnReluF16x2F4x2type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    string_p(".rn"),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    string_p(".f16x2"),
                    F4x2type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rn, relu, f16x2, f4x2type, d, _, a, _), span| {
                    ok!(CvtRnReluF16x2F4x2type {
                        rn = rn,
                        relu = relu,
                        f16x2 = f16x2,
                        f4x2type = f4x2type,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtRsReluSatfiniteF4x4typeF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    string_p(".rs"),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    string_p(".satfinite"),
                    F4x4type::parse(),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    VectorOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rs, relu, satfinite, f4x4type, f32, d, _, a, _, rbits, _), span| {
                    ok!(CvtRsReluSatfiniteF4x4typeF32 {
                        rs = rs,
                        relu = relu,
                        satfinite = satfinite,
                        f4x4type = f4x4type,
                        f32 = f32,
                        d = d,
                        a = a,
                        rbits = rbits,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtRnSatfiniteReluF6x2typeF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    string_p(".rn"),
                    string_p(".satfinite"),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    F6x2type::parse(),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rn, satfinite, relu, f6x2type, f32, d, _, a, _, b, _), span| {
                    ok!(CvtRnSatfiniteReluF6x2typeF32 {
                        rn = rn,
                        satfinite = satfinite,
                        relu = relu,
                        f6x2type = f6x2type,
                        f32 = f32,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtRnReluF16x2F6x2type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    string_p(".rn"),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    string_p(".f16x2"),
                    F6x2type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rn, relu, f16x2, f6x2type, d, _, a, _), span| {
                    ok!(CvtRnReluF16x2F6x2type {
                        rn = rn,
                        relu = relu,
                        f16x2 = f16x2,
                        f6x2type = f6x2type,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtRsReluSatfiniteF6x4typeF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    string_p(".rs"),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    string_p(".satfinite"),
                    F6x4type::parse(),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    VectorOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rs, relu, satfinite, f6x4type, f32, d, _, a, _, rbits, _), span| {
                    ok!(CvtRsReluSatfiniteF6x4typeF32 {
                        rs = rs,
                        relu = relu,
                        satfinite = satfinite,
                        f6x4type = f6x4type,
                        f32 = f32,
                        d = d,
                        a = a,
                        rbits = rbits,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtFrnd3SatfiniteUe8m0x2F32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    Frnd3::parse(),
                    map(optional(string_p(".satfinite")), |value, _| value.is_some()),
                    string_p(".ue8m0x2"),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, frnd3, satfinite, ue8m0x2, f32, d, _, a, _, b, _), span| {
                    ok!(CvtFrnd3SatfiniteUe8m0x2F32 {
                        frnd3 = frnd3,
                        satfinite = satfinite,
                        ue8m0x2 = ue8m0x2,
                        f32 = f32,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtFrnd3SatfiniteUe8m0x2Bf16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    Frnd3::parse(),
                    map(optional(string_p(".satfinite")), |value, _| value.is_some()),
                    string_p(".ue8m0x2"),
                    string_p(".bf16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, frnd3, satfinite, ue8m0x2, bf16x2, d, _, a, _), span| {
                    ok!(CvtFrnd3SatfiniteUe8m0x2Bf16x2 {
                        frnd3 = frnd3,
                        satfinite = satfinite,
                        ue8m0x2 = ue8m0x2,
                        bf16x2 = bf16x2,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtRnBf16x2Ue8m0x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    string_p(".rn"),
                    string_p(".bf16x2"),
                    string_p(".ue8m0x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rn, bf16x2, ue8m0x2, d, _, a, _), span| {
                    ok!(CvtRnBf16x2Ue8m0x2 {
                        rn = rn,
                        bf16x2 = bf16x2,
                        ue8m0x2 = ue8m0x2,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }
}
