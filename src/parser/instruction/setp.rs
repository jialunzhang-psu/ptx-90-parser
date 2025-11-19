//! Original PTX specification:
//!
//! setp.CmpOp{.ftz}.type         p{|q}, a, b;
//! setp.CmpOp.BoolOp{.ftz}.type  p{|q}, a, b, {!}c;
//! .CmpOp  = { .eq, .ne, .lt, .le, .gt, .ge, .lo, .ls, .hi, .hs, .equ, .neu, .ltu, .leu, .gtu, .geu, .num, .nan };
//! .BoolOp = { .and, .or, .xor };
//! .type   = { .b16, .b32, .b64, .u16, .u32, .u64, .s16, .s32, .s64, .f32, .f64 };
//! --------------------------------------------------------------
//! setp.CmpOp{.ftz}.f16           p, a, b;
//! setp.CmpOp.BoolOp{.ftz}.f16    p, a, b, {!}c;
//! setp.CmpOp{.ftz}.f16x2         p|q, a, b;
//! setp.CmpOp.BoolOp{.ftz}.f16x2  p|q, a, b, {!}c;
//! setp.CmpOp.bf16                p, a, b;
//! setp.CmpOp.BoolOp.bf16         p, a, b, {!}c;
//! setp.CmpOp.bf16x2              p|q, a, b;
//! setp.CmpOp.BoolOp.bf16x2       p|q, a, b, {!}c;
//! .CmpOp  = { .eq, .ne, .lt, .le, .gt, .ge, .equ, .neu, .ltu, .leu, .gtu, .geu, .num, .nan };
//! .BoolOp = { .and, .or, .xor };

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
    use crate::r#type::instruction::setp::section_0::*;

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

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b16"), |_, _span| Type::B16),
                map(string_p(".b32"), |_, _span| Type::B32),
                map(string_p(".b64"), |_, _span| Type::B64),
                map(string_p(".u16"), |_, _span| Type::U16),
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".u64"), |_, _span| Type::U64),
                map(string_p(".s16"), |_, _span| Type::S16),
                map(string_p(".s32"), |_, _span| Type::S32),
                map(string_p(".s64"), |_, _span| Type::S64),
                map(string_p(".f32"), |_, _span| Type::F32),
                map(string_p(".f64"), |_, _span| Type::F64)
            )
        }
    }

    impl PtxParser for SetpCmpopFtzType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("setp"),
                    Cmpop::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    Type::parse(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(pipe_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, ftz, type_, p, q, _, a, _, b, _), span| {
                    ok!(SetpCmpopFtzType {
                        cmpop = cmpop,
                        ftz = ftz,
                        type_ = type_,
                        p = p,
                        q = q,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for SetpCmpopBoolopFtzType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("setp"),
                    Cmpop::parse(),
                    Boolop::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    Type::parse(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(pipe_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    map(optional(exclamation_p()), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, boolop, ftz, type_, p, q, _, a, _, b, _, c_op, c, _), span| {
                    ok!(SetpCmpopBoolopFtzType {
                        cmpop = cmpop,
                        boolop = boolop,
                        ftz = ftz,
                        type_ = type_,
                        p = p,
                        q = q,
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
    use crate::r#type::instruction::setp::section_1::*;

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

    impl PtxParser for SetpCmpopFtzF16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("setp"),
                    Cmpop::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, ftz, f16, p, _, a, _, b, _), span| {
                    ok!(SetpCmpopFtzF16 {
                        cmpop = cmpop,
                        ftz = ftz,
                        f16 = f16,
                        p = p,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for SetpCmpopBoolopFtzF16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("setp"),
                    Cmpop::parse(),
                    Boolop::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
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
                |(_, cmpop, boolop, ftz, f16, p, _, a, _, b, _, c_op, c, _), span| {
                    ok!(SetpCmpopBoolopFtzF16 {
                        cmpop = cmpop,
                        boolop = boolop,
                        ftz = ftz,
                        f16 = f16,
                        p = p,
                        a = a,
                        b = b,
                        c_op = c_op,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for SetpCmpopFtzF16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("setp"),
                    Cmpop::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f16x2"),
                    GeneralOperand::parse(),
                    pipe_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, ftz, f16x2, p, _, q, _, a, _, b, _), span| {
                    ok!(SetpCmpopFtzF16x2 {
                        cmpop = cmpop,
                        ftz = ftz,
                        f16x2 = f16x2,
                        p = p,
                        q = q,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for SetpCmpopBoolopFtzF16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("setp"),
                    Cmpop::parse(),
                    Boolop::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f16x2"),
                    GeneralOperand::parse(),
                    pipe_p(),
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
                |(_, cmpop, boolop, ftz, f16x2, p, _, q, _, a, _, b, _, c_op, c, _), span| {
                    ok!(SetpCmpopBoolopFtzF16x2 {
                        cmpop = cmpop,
                        boolop = boolop,
                        ftz = ftz,
                        f16x2 = f16x2,
                        p = p,
                        q = q,
                        a = a,
                        b = b,
                        c_op = c_op,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for SetpCmpopBf16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("setp"),
                    Cmpop::parse(),
                    string_p(".bf16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, bf16, p, _, a, _, b, _), span| {
                    ok!(SetpCmpopBf16 {
                        cmpop = cmpop,
                        bf16 = bf16,
                        p = p,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for SetpCmpopBoolopBf16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("setp"),
                    Cmpop::parse(),
                    Boolop::parse(),
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
                |(_, cmpop, boolop, bf16, p, _, a, _, b, _, c_op, c, _), span| {
                    ok!(SetpCmpopBoolopBf16 {
                        cmpop = cmpop,
                        boolop = boolop,
                        bf16 = bf16,
                        p = p,
                        a = a,
                        b = b,
                        c_op = c_op,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for SetpCmpopBf16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("setp"),
                    Cmpop::parse(),
                    string_p(".bf16x2"),
                    GeneralOperand::parse(),
                    pipe_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cmpop, bf16x2, p, _, q, _, a, _, b, _), span| {
                    ok!(SetpCmpopBf16x2 {
                        cmpop = cmpop,
                        bf16x2 = bf16x2,
                        p = p,
                        q = q,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for SetpCmpopBoolopBf16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("setp"),
                    Cmpop::parse(),
                    Boolop::parse(),
                    string_p(".bf16x2"),
                    GeneralOperand::parse(),
                    pipe_p(),
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
                |(_, cmpop, boolop, bf16x2, p, _, q, _, a, _, b, _, c_op, c, _), span| {
                    ok!(SetpCmpopBoolopBf16x2 {
                        cmpop = cmpop,
                        boolop = boolop,
                        bf16x2 = bf16x2,
                        p = p,
                        q = q,
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
