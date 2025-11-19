//! Original PTX specification:
//!
//! set.CmpOp{.ftz}.dtype.stype         d, a, b;
//! set.CmpOp.BoolOp{.ftz}.dtype.stype  d, a, b, {!}c;
//! .CmpOp  = { .eq, .ne, .lt, .le, .gt, .ge, .lo, .ls, .hi, .hs,
//! .equ, .neu, .ltu, .leu, .gtu, .geu, .num, .nan };
//! .BoolOp = { .and, .or, .xor };
//! .dtype  = { .u32, .s32, .f32 };
//! .stype  = { .b16, .b32, .b64,
//! .u16, .u32, .u64,
//! .s16, .s32, .s64,
//! .f32, .f64 };
//! -------------------------------------------------------------
//! set.CmpOp{.ftz}.f16.stype            d, a, b;
//! set.CmpOp.BoolOp{.ftz}.f16.stype     d, a, b, {!}c;
//! set.CmpOp.bf16.stype                 d, a, b;
//! set.CmpOp.BoolOp.bf16.stype          d, a, b, {!}c;
//! set.CmpOp{.ftz}.dtype.f16            d, a, b;
//! set.CmpOp.BoolOp{.ftz}.dtype.f16     d, a, b, {!}c;
//! .dtype  = { .u16, .s16, .u32, .s32};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! set.CmpOp.dtype.bf16                 d, a, b;
//! set.CmpOp.BoolOp.dtype.bf16          d, a, b, {!}c;
//! .dtype  = { .u16, .s16, .u32, .s32};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! set.CmpOp{.ftz}.dtype.f16x2          d, a, b;
//! set.CmpOp.BoolOp{.ftz}.dtype.f16x2   d, a, b, {!}c;
//! .dtype  = { .f16x2, .u32, .s32};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! set.CmpOp.dtype.bf16x2               d, a, b;
//! set.CmpOp.BoolOp.dtype.bf16x2        d, a, b, {!}c;
//! .dtype  = { .bf16x2, .u32, .s32};
//! .CmpOp  = { .eq, .ne, .lt, .le, .gt, .ge,
//! .equ, .neu, .ltu, .leu, .gtu, .geu, .num, .nan };
//! .BoolOp = { .and, .or, .xor };
//! .stype  = { .b16, .b32, .b64,
//! .u16, .u32, .u64,
//! .s16, .s32, .s64,
//! .f16, .f32, .f64};

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
    use crate::r#type::instruction::set::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Boolop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".and"), |_, _span| Boolop::And),
                map(string_p(".xor"), |_, _span| Boolop::Xor),
                map(string_p(".or"), |_, _span| Boolop::Or)
            )
        }
    }

    impl PtxParser for Cmpop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".equ"), |_, _span| Cmpop::Equ),
                map(string_p(".neu"), |_, _span| Cmpop::Neu),
                map(string_p(".ltu"), |_, _span| Cmpop::Ltu),
                map(string_p(".leu"), |_, _span| Cmpop::Leu),
                map(string_p(".gtu"), |_, _span| Cmpop::Gtu),
                map(string_p(".geu"), |_, _span| Cmpop::Geu),
                map(string_p(".num"), |_, _span| Cmpop::Num),
                map(string_p(".nan"), |_, _span| Cmpop::Nan),
                map(string_p(".eq"), |_, _span| Cmpop::Eq),
                map(string_p(".ne"), |_, _span| Cmpop::Ne),
                map(string_p(".lt"), |_, _span| Cmpop::Lt),
                map(string_p(".le"), |_, _span| Cmpop::Le),
                map(string_p(".gt"), |_, _span| Cmpop::Gt),
                map(string_p(".ge"), |_, _span| Cmpop::Ge),
                map(string_p(".lo"), |_, _span| Cmpop::Lo),
                map(string_p(".ls"), |_, _span| Cmpop::Ls),
                map(string_p(".hi"), |_, _span| Cmpop::Hi),
                map(string_p(".hs"), |_, _span| Cmpop::Hs)
            )
        }
    }

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Dtype::U32),
                map(string_p(".s32"), |_, _span| Dtype::S32),
                map(string_p(".f32"), |_, _span| Dtype::F32)
            )
        }
    }

    impl PtxParser for Stype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b16"), |_, _span| Stype::B16),
                map(string_p(".b32"), |_, _span| Stype::B32),
                map(string_p(".b64"), |_, _span| Stype::B64),
                map(string_p(".u16"), |_, _span| Stype::U16),
                map(string_p(".u32"), |_, _span| Stype::U32),
                map(string_p(".u64"), |_, _span| Stype::U64),
                map(string_p(".s16"), |_, _span| Stype::S16),
                map(string_p(".s32"), |_, _span| Stype::S32),
                map(string_p(".s64"), |_, _span| Stype::S64),
                map(string_p(".f32"), |_, _span| Stype::F32),
                map(string_p(".f64"), |_, _span| Stype::F64)
            )
        }
    }

    impl PtxParser for SetCmpopFtzDtypeStype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("set"),
                    Cmpop::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    Dtype::parse(),
                    Stype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, ftz, dtype, stype, d, _, a, _, b, _), span| {
                    ok!(SetCmpopFtzDtypeStype {
                        cmpop = cmpop,
                        ftz = ftz,
                        dtype = dtype,
                        stype = stype,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for SetCmpopBoolopFtzDtypeStype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("set"),
                    Cmpop::parse(),
                    Boolop::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    Dtype::parse(),
                    Stype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    map(optional(exclamation_p()), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, boolop, ftz, dtype, stype, d, _, a, _, b, _, c_op, c, _), span| {
                    ok!(SetCmpopBoolopFtzDtypeStype {
                        cmpop = cmpop,
                        boolop = boolop,
                        ftz = ftz,
                        dtype = dtype,
                        stype = stype,
                        d = d,
                        a = a,
                        b = b,
                        c_op = c_op,
                        c = c,

                    })
                },
            )
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::set::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Boolop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".and"), |_, _span| Boolop::And),
                map(string_p(".xor"), |_, _span| Boolop::Xor),
                map(string_p(".or"), |_, _span| Boolop::Or)
            )
        }
    }

    impl PtxParser for Cmpop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".equ"), |_, _span| Cmpop::Equ),
                map(string_p(".neu"), |_, _span| Cmpop::Neu),
                map(string_p(".ltu"), |_, _span| Cmpop::Ltu),
                map(string_p(".leu"), |_, _span| Cmpop::Leu),
                map(string_p(".gtu"), |_, _span| Cmpop::Gtu),
                map(string_p(".geu"), |_, _span| Cmpop::Geu),
                map(string_p(".num"), |_, _span| Cmpop::Num),
                map(string_p(".nan"), |_, _span| Cmpop::Nan),
                map(string_p(".eq"), |_, _span| Cmpop::Eq),
                map(string_p(".ne"), |_, _span| Cmpop::Ne),
                map(string_p(".lt"), |_, _span| Cmpop::Lt),
                map(string_p(".le"), |_, _span| Cmpop::Le),
                map(string_p(".gt"), |_, _span| Cmpop::Gt),
                map(string_p(".ge"), |_, _span| Cmpop::Ge),
                map(string_p(".lo"), |_, _span| Cmpop::Lo),
                map(string_p(".ls"), |_, _span| Cmpop::Ls),
                map(string_p(".hi"), |_, _span| Cmpop::Hi),
                map(string_p(".hs"), |_, _span| Cmpop::Hs)
            )
        }
    }

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u16"), |_, _span| Dtype::U16),
                map(string_p(".s16"), |_, _span| Dtype::S16),
                map(string_p(".u32"), |_, _span| Dtype::U32),
                map(string_p(".s32"), |_, _span| Dtype::S32)
            )
        }
    }

    impl PtxParser for Stype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b16"), |_, _span| Stype::B16),
                map(string_p(".b32"), |_, _span| Stype::B32),
                map(string_p(".b64"), |_, _span| Stype::B64),
                map(string_p(".u16"), |_, _span| Stype::U16),
                map(string_p(".u32"), |_, _span| Stype::U32),
                map(string_p(".u64"), |_, _span| Stype::U64),
                map(string_p(".s16"), |_, _span| Stype::S16),
                map(string_p(".s32"), |_, _span| Stype::S32),
                map(string_p(".s64"), |_, _span| Stype::S64),
                map(string_p(".f32"), |_, _span| Stype::F32),
                map(string_p(".f64"), |_, _span| Stype::F64)
            )
        }
    }

    impl PtxParser for SetCmpopFtzF16Stype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("set"),
                    Cmpop::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f16"),
                    Stype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, ftz, f16, stype, d, _, a, _, b, _), span| {
                    ok!(SetCmpopFtzF16Stype {
                        cmpop = cmpop,
                        ftz = ftz,
                        f16 = f16,
                        stype = stype,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for SetCmpopBoolopFtzF16Stype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("set"),
                    Cmpop::parse(),
                    Boolop::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f16"),
                    Stype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    map(optional(exclamation_p()), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, boolop, ftz, f16, stype, d, _, a, _, b, _, c_op, c, _), span| {
                    ok!(SetCmpopBoolopFtzF16Stype {
                        cmpop = cmpop,
                        boolop = boolop,
                        ftz = ftz,
                        f16 = f16,
                        stype = stype,
                        d = d,
                        a = a,
                        b = b,
                        c_op = c_op,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for SetCmpopBf16Stype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("set"),
                    Cmpop::parse(),
                    string_p(".bf16"),
                    Stype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, bf16, stype, d, _, a, _, b, _), span| {
                    ok!(SetCmpopBf16Stype {
                        cmpop = cmpop,
                        bf16 = bf16,
                        stype = stype,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for SetCmpopBoolopBf16Stype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("set"),
                    Cmpop::parse(),
                    Boolop::parse(),
                    string_p(".bf16"),
                    Stype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    map(optional(exclamation_p()), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, boolop, bf16, stype, d, _, a, _, b, _, c_op, c, _), span| {
                    ok!(SetCmpopBoolopBf16Stype {
                        cmpop = cmpop,
                        boolop = boolop,
                        bf16 = bf16,
                        stype = stype,
                        d = d,
                        a = a,
                        b = b,
                        c_op = c_op,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for SetCmpopFtzDtypeF16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("set"),
                    Cmpop::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    Dtype::parse(),
                    string_p(".f16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, ftz, dtype, f16, d, _, a, _, b, _), span| {
                    ok!(SetCmpopFtzDtypeF16 {
                        cmpop = cmpop,
                        ftz = ftz,
                        dtype = dtype,
                        f16 = f16,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for SetCmpopBoolopFtzDtypeF16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("set"),
                    Cmpop::parse(),
                    Boolop::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    Dtype::parse(),
                    string_p(".f16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    map(optional(exclamation_p()), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, boolop, ftz, dtype, f16, d, _, a, _, b, _, c_op, c, _), span| {
                    ok!(SetCmpopBoolopFtzDtypeF16 {
                        cmpop = cmpop,
                        boolop = boolop,
                        ftz = ftz,
                        dtype = dtype,
                        f16 = f16,
                        d = d,
                        a = a,
                        b = b,
                        c_op = c_op,
                        c = c,

                    })
                },
            )
        }
    }
}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::set::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Boolop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".and"), |_, _span| Boolop::And),
                map(string_p(".xor"), |_, _span| Boolop::Xor),
                map(string_p(".or"), |_, _span| Boolop::Or)
            )
        }
    }

    impl PtxParser for Cmpop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".equ"), |_, _span| Cmpop::Equ),
                map(string_p(".neu"), |_, _span| Cmpop::Neu),
                map(string_p(".ltu"), |_, _span| Cmpop::Ltu),
                map(string_p(".leu"), |_, _span| Cmpop::Leu),
                map(string_p(".gtu"), |_, _span| Cmpop::Gtu),
                map(string_p(".geu"), |_, _span| Cmpop::Geu),
                map(string_p(".num"), |_, _span| Cmpop::Num),
                map(string_p(".nan"), |_, _span| Cmpop::Nan),
                map(string_p(".eq"), |_, _span| Cmpop::Eq),
                map(string_p(".ne"), |_, _span| Cmpop::Ne),
                map(string_p(".lt"), |_, _span| Cmpop::Lt),
                map(string_p(".le"), |_, _span| Cmpop::Le),
                map(string_p(".gt"), |_, _span| Cmpop::Gt),
                map(string_p(".ge"), |_, _span| Cmpop::Ge),
                map(string_p(".lo"), |_, _span| Cmpop::Lo),
                map(string_p(".ls"), |_, _span| Cmpop::Ls),
                map(string_p(".hi"), |_, _span| Cmpop::Hi),
                map(string_p(".hs"), |_, _span| Cmpop::Hs)
            )
        }
    }

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u16"), |_, _span| Dtype::U16),
                map(string_p(".s16"), |_, _span| Dtype::S16),
                map(string_p(".u32"), |_, _span| Dtype::U32),
                map(string_p(".s32"), |_, _span| Dtype::S32)
            )
        }
    }

    impl PtxParser for SetCmpopDtypeBf16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("set"),
                    Cmpop::parse(),
                    Dtype::parse(),
                    string_p(".bf16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, dtype, bf16, d, _, a, _, b, _), span| {
                    ok!(SetCmpopDtypeBf16 {
                        cmpop = cmpop,
                        dtype = dtype,
                        bf16 = bf16,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for SetCmpopBoolopDtypeBf16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("set"),
                    Cmpop::parse(),
                    Boolop::parse(),
                    Dtype::parse(),
                    string_p(".bf16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    map(optional(exclamation_p()), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, boolop, dtype, bf16, d, _, a, _, b, _, c_op, c, _), span| {
                    ok!(SetCmpopBoolopDtypeBf16 {
                        cmpop = cmpop,
                        boolop = boolop,
                        dtype = dtype,
                        bf16 = bf16,
                        d = d,
                        a = a,
                        b = b,
                        c_op = c_op,
                        c = c,

                    })
                },
            )
        }
    }
}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::set::section_3::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Boolop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".and"), |_, _span| Boolop::And),
                map(string_p(".xor"), |_, _span| Boolop::Xor),
                map(string_p(".or"), |_, _span| Boolop::Or)
            )
        }
    }

    impl PtxParser for Cmpop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".equ"), |_, _span| Cmpop::Equ),
                map(string_p(".neu"), |_, _span| Cmpop::Neu),
                map(string_p(".ltu"), |_, _span| Cmpop::Ltu),
                map(string_p(".leu"), |_, _span| Cmpop::Leu),
                map(string_p(".gtu"), |_, _span| Cmpop::Gtu),
                map(string_p(".geu"), |_, _span| Cmpop::Geu),
                map(string_p(".num"), |_, _span| Cmpop::Num),
                map(string_p(".nan"), |_, _span| Cmpop::Nan),
                map(string_p(".eq"), |_, _span| Cmpop::Eq),
                map(string_p(".ne"), |_, _span| Cmpop::Ne),
                map(string_p(".lt"), |_, _span| Cmpop::Lt),
                map(string_p(".le"), |_, _span| Cmpop::Le),
                map(string_p(".gt"), |_, _span| Cmpop::Gt),
                map(string_p(".ge"), |_, _span| Cmpop::Ge),
                map(string_p(".lo"), |_, _span| Cmpop::Lo),
                map(string_p(".ls"), |_, _span| Cmpop::Ls),
                map(string_p(".hi"), |_, _span| Cmpop::Hi),
                map(string_p(".hs"), |_, _span| Cmpop::Hs)
            )
        }
    }

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".f16x2"), |_, _span| Dtype::F16x2),
                map(string_p(".u32"), |_, _span| Dtype::U32),
                map(string_p(".s32"), |_, _span| Dtype::S32)
            )
        }
    }

    impl PtxParser for SetCmpopFtzDtypeF16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("set"),
                    Cmpop::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    Dtype::parse(),
                    string_p(".f16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, ftz, dtype, f16x2, d, _, a, _, b, _), span| {
                    ok!(SetCmpopFtzDtypeF16x2 {
                        cmpop = cmpop,
                        ftz = ftz,
                        dtype = dtype,
                        f16x2 = f16x2,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for SetCmpopBoolopFtzDtypeF16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("set"),
                    Cmpop::parse(),
                    Boolop::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    Dtype::parse(),
                    string_p(".f16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    map(optional(exclamation_p()), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, boolop, ftz, dtype, f16x2, d, _, a, _, b, _, c_op, c, _), span| {
                    ok!(SetCmpopBoolopFtzDtypeF16x2 {
                        cmpop = cmpop,
                        boolop = boolop,
                        ftz = ftz,
                        dtype = dtype,
                        f16x2 = f16x2,
                        d = d,
                        a = a,
                        b = b,
                        c_op = c_op,
                        c = c,

                    })
                },
            )
        }
    }
}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::set::section_4::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Boolop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".and"), |_, _span| Boolop::And),
                map(string_p(".xor"), |_, _span| Boolop::Xor),
                map(string_p(".or"), |_, _span| Boolop::Or)
            )
        }
    }

    impl PtxParser for Cmpop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".equ"), |_, _span| Cmpop::Equ),
                map(string_p(".neu"), |_, _span| Cmpop::Neu),
                map(string_p(".ltu"), |_, _span| Cmpop::Ltu),
                map(string_p(".leu"), |_, _span| Cmpop::Leu),
                map(string_p(".gtu"), |_, _span| Cmpop::Gtu),
                map(string_p(".geu"), |_, _span| Cmpop::Geu),
                map(string_p(".num"), |_, _span| Cmpop::Num),
                map(string_p(".nan"), |_, _span| Cmpop::Nan),
                map(string_p(".eq"), |_, _span| Cmpop::Eq),
                map(string_p(".ne"), |_, _span| Cmpop::Ne),
                map(string_p(".lt"), |_, _span| Cmpop::Lt),
                map(string_p(".le"), |_, _span| Cmpop::Le),
                map(string_p(".gt"), |_, _span| Cmpop::Gt),
                map(string_p(".ge"), |_, _span| Cmpop::Ge)
            )
        }
    }

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".bf16x2"), |_, _span| Dtype::Bf16x2),
                map(string_p(".u32"), |_, _span| Dtype::U32),
                map(string_p(".s32"), |_, _span| Dtype::S32)
            )
        }
    }

    impl PtxParser for SetCmpopDtypeBf16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("set"),
                    Cmpop::parse(),
                    Dtype::parse(),
                    string_p(".bf16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, dtype, bf16x2, d, _, a, _, b, _), span| {
                    ok!(SetCmpopDtypeBf16x2 {
                        cmpop = cmpop,
                        dtype = dtype,
                        bf16x2 = bf16x2,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for SetCmpopBoolopDtypeBf16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("set"),
                    Cmpop::parse(),
                    Boolop::parse(),
                    Dtype::parse(),
                    string_p(".bf16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    map(optional(exclamation_p()), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, boolop, dtype, bf16x2, d, _, a, _, b, _, c_op, c, _), span| {
                    ok!(SetCmpopBoolopDtypeBf16x2 {
                        cmpop = cmpop,
                        boolop = boolop,
                        dtype = dtype,
                        bf16x2 = bf16x2,
                        d = d,
                        a = a,
                        b = b,
                        c_op = c_op,
                        c = c,

                    })
                },
            )
        }
    }
}
