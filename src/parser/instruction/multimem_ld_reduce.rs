//! Original PTX specification:
//!
//! // Integer type:
//! multimem.ld_reduce{.ldsem}{.scope}{.ss}.op.type      d, [a];
//! multimem.ld_reduce.weak{.ss}.op.type                 d, [a];
//! multimem.st{.stsem}{.scope}{.ss}.type                [a], b;
//! multimem.st.weak{.ss}.type                           [a], b;
//! multimem.red{.redsem}{.scope}{.ss}.op.type           [a], b;
//! .ss =       { .global };
//! .ldsem =    { .relaxed, .acquire };
//! .stsem =    { .relaxed, .release };
//! .redsem =   { .relaxed, .release };
//! .scope =    { .cta, .cluster, .gpu, .sys };
//! .op  =      { .min, .max, .add, .and, .or, .xor };
//! .type =     { .b32, .b64,  .u32, .u64, .s32, .s64 };
//! ------------------------------------------------------------------
//! // Floating point type:
//! multimem.ld_reduce{.ldsem}{.scope}{.ss}.op{.acc_prec}{.vec}.type    d, [a];
//! multimem.ld_reduce.weak{.ss}.op{.acc_prec}{.vec}.type               d, [a];
//! multimem.st{.stsem}{.scope}{.ss}{.vec}.type                         [a], b;
//! multimem.st.weak{.ss}{.vec}.type                                    [a], b;
//! multimem.red{.redsem}{.scope}{.ss}.redop{.vec}.redtype              [a], b;
//! .ss =       { .global };
//! .ldsem =    { .relaxed, .acquire };
//! .stsem =    { .relaxed, .release };
//! .redsem =   { .relaxed, .release };
//! .scope =    { .cta, .cluster, .gpu, .sys };
//! .op  =      { .min, .max, .add };
//! .redop  =   { .add };
//! .acc_prec = { .acc::f32, .acc::f16 };
//! .vec =      { .v2, .v4, .v8 };
//! .type=      { .f16, .f16x2, .bf16, .bf16x2, .f32, .f64, .e5m2, .e5m2x2, .e5m2x4, .e4m3, .e4m3x2, .e4m3x4 };
//! .redtype =  { .f16, .f16x2, .bf16, .bf16x2, .f32, .f64 };

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
    use crate::r#type::instruction::multimem_ld_reduce::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Ldsem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".relaxed"), |_, _span| Ldsem::Relaxed),
                map(string_p(".acquire"), |_, _span| Ldsem::Acquire)
            )
        }
    }

    impl PtxParser for Op {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".min"), |_, _span| Op::Min),
                map(string_p(".max"), |_, _span| Op::Max),
                map(string_p(".add"), |_, _span| Op::Add),
                map(string_p(".and"), |_, _span| Op::And),
                map(string_p(".xor"), |_, _span| Op::Xor),
                map(string_p(".or"), |_, _span| Op::Or)
            )
        }
    }

    impl PtxParser for Redsem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".relaxed"), |_, _span| Redsem::Relaxed),
                map(string_p(".release"), |_, _span| Redsem::Release)
            )
        }
    }

    impl PtxParser for Scope {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cluster"), |_, _span| Scope::Cluster),
                map(string_p(".cta"), |_, _span| Scope::Cta),
                map(string_p(".gpu"), |_, _span| Scope::Gpu),
                map(string_p(".sys"), |_, _span| Scope::Sys)
            )
        }
    }

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".global"), |_, _span| Ss::Global))
        }
    }

    impl PtxParser for Stsem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".relaxed"), |_, _span| Stsem::Relaxed),
                map(string_p(".release"), |_, _span| Stsem::Release)
            )
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b32"), |_, _span| Type::B32),
                map(string_p(".b64"), |_, _span| Type::B64),
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".u64"), |_, _span| Type::U64),
                map(string_p(".s32"), |_, _span| Type::S32),
                map(string_p(".s64"), |_, _span| Type::S64)
            )
        }
    }

    impl PtxParser for MultimemLdReduceLdsemScopeSsOpType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("multimem"),
                    string_p(".ld_reduce"),
                    optional(Ldsem::parse()),
                    optional(Scope::parse()),
                    optional(Ss::parse()),
                    Op::parse(),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, ld_reduce, ldsem, scope, ss, op, type_, d, _, a, _), span| {
                    ok!(MultimemLdReduceLdsemScopeSsOpType {
                        ld_reduce = ld_reduce,
                        ldsem = ldsem,
                        scope = scope,
                        ss = ss,
                        op = op,
                        type_ = type_,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for MultimemLdReduceWeakSsOpType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("multimem"),
                    string_p(".ld_reduce"),
                    string_p(".weak"),
                    optional(Ss::parse()),
                    Op::parse(),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, ld_reduce, weak, ss, op, type_, d, _, a, _), span| {
                    ok!(MultimemLdReduceWeakSsOpType {
                        ld_reduce = ld_reduce,
                        weak = weak,
                        ss = ss,
                        op = op,
                        type_ = type_,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for MultimemStStsemScopeSsType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("multimem"),
                    string_p(".st"),
                    optional(Stsem::parse()),
                    optional(Scope::parse()),
                    optional(Ss::parse()),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, st, stsem, scope, ss, type_, a, _, b, _), span| {
                    ok!(MultimemStStsemScopeSsType {
                        st = st,
                        stsem = stsem,
                        scope = scope,
                        ss = ss,
                        type_ = type_,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for MultimemStWeakSsType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("multimem"),
                    string_p(".st"),
                    string_p(".weak"),
                    optional(Ss::parse()),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, st, weak, ss, type_, a, _, b, _), span| {
                    ok!(MultimemStWeakSsType {
                        st = st,
                        weak = weak,
                        ss = ss,
                        type_ = type_,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for MultimemRedRedsemScopeSsOpType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("multimem"),
                    string_p(".red"),
                    optional(Redsem::parse()),
                    optional(Scope::parse()),
                    optional(Ss::parse()),
                    Op::parse(),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, red, redsem, scope, ss, op, type_, a, _, b, _), span| {
                    ok!(MultimemRedRedsemScopeSsOpType {
                        red = red,
                        redsem = redsem,
                        scope = scope,
                        ss = ss,
                        op = op,
                        type_ = type_,
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
    use crate::r#type::instruction::multimem_ld_reduce::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for AccPrec {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".acc::f32"), |_, _span| AccPrec::AccF32),
                map(string_p(".acc::f16"), |_, _span| AccPrec::AccF16)
            )
        }
    }

    impl PtxParser for Ldsem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".relaxed"), |_, _span| Ldsem::Relaxed),
                map(string_p(".acquire"), |_, _span| Ldsem::Acquire)
            )
        }
    }

    impl PtxParser for Op {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".min"), |_, _span| Op::Min),
                map(string_p(".max"), |_, _span| Op::Max),
                map(string_p(".add"), |_, _span| Op::Add)
            )
        }
    }

    impl PtxParser for Redop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".add"), |_, _span| Redop::Add))
        }
    }

    impl PtxParser for Redsem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".relaxed"), |_, _span| Redsem::Relaxed),
                map(string_p(".release"), |_, _span| Redsem::Release)
            )
        }
    }

    impl PtxParser for Redtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".bf16x2"), |_, _span| Redtype::Bf16x2),
                map(string_p(".f16x2"), |_, _span| Redtype::F16x2),
                map(string_p(".bf16"), |_, _span| Redtype::Bf16),
                map(string_p(".f16"), |_, _span| Redtype::F16),
                map(string_p(".f32"), |_, _span| Redtype::F32),
                map(string_p(".f64"), |_, _span| Redtype::F64)
            )
        }
    }

    impl PtxParser for Scope {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cluster"), |_, _span| Scope::Cluster),
                map(string_p(".cta"), |_, _span| Scope::Cta),
                map(string_p(".gpu"), |_, _span| Scope::Gpu),
                map(string_p(".sys"), |_, _span| Scope::Sys)
            )
        }
    }

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".global"), |_, _span| Ss::Global))
        }
    }

    impl PtxParser for Stsem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".relaxed"), |_, _span| Stsem::Relaxed),
                map(string_p(".release"), |_, _span| Stsem::Release)
            )
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".bf16x2"), |_, _span| Type::Bf16x2),
                map(string_p(".e5m2x2"), |_, _span| Type::E5m2x2),
                map(string_p(".e5m2x4"), |_, _span| Type::E5m2x4),
                map(string_p(".e4m3x2"), |_, _span| Type::E4m3x2),
                map(string_p(".e4m3x4"), |_, _span| Type::E4m3x4),
                map(string_p(".f16x2"), |_, _span| Type::F16x2),
                map(string_p(".bf16"), |_, _span| Type::Bf16),
                map(string_p(".e5m2"), |_, _span| Type::E5m2),
                map(string_p(".e4m3"), |_, _span| Type::E4m3),
                map(string_p(".f16"), |_, _span| Type::F16),
                map(string_p(".f32"), |_, _span| Type::F32),
                map(string_p(".f64"), |_, _span| Type::F64)
            )
        }
    }

    impl PtxParser for Vec {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".v2"), |_, _span| Vec::V2),
                map(string_p(".v4"), |_, _span| Vec::V4),
                map(string_p(".v8"), |_, _span| Vec::V8)
            )
        }
    }

    impl PtxParser for MultimemLdReduceLdsemScopeSsOpAccPrecVecType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("multimem"),
                    string_p(".ld_reduce"),
                    optional(Ldsem::parse()),
                    optional(Scope::parse()),
                    optional(Ss::parse()),
                    Op::parse(),
                    optional(AccPrec::parse()),
                    optional(Vec::parse()),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, ld_reduce, ldsem, scope, ss, op, acc_prec, vec, type_, d, _, a, _), span| {
                    ok!(MultimemLdReduceLdsemScopeSsOpAccPrecVecType {
                        ld_reduce = ld_reduce,
                        ldsem = ldsem,
                        scope = scope,
                        ss = ss,
                        op = op,
                        acc_prec = acc_prec,
                        vec = vec,
                        type_ = type_,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for MultimemLdReduceWeakSsOpAccPrecVecType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("multimem"),
                    string_p(".ld_reduce"),
                    string_p(".weak"),
                    optional(Ss::parse()),
                    Op::parse(),
                    optional(AccPrec::parse()),
                    optional(Vec::parse()),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, ld_reduce, weak, ss, op, acc_prec, vec, type_, d, _, a, _), span| {
                    ok!(MultimemLdReduceWeakSsOpAccPrecVecType {
                        ld_reduce = ld_reduce,
                        weak = weak,
                        ss = ss,
                        op = op,
                        acc_prec = acc_prec,
                        vec = vec,
                        type_ = type_,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for MultimemStStsemScopeSsVecType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("multimem"),
                    string_p(".st"),
                    optional(Stsem::parse()),
                    optional(Scope::parse()),
                    optional(Ss::parse()),
                    optional(Vec::parse()),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, st, stsem, scope, ss, vec, type_, a, _, b, _), span| {
                    ok!(MultimemStStsemScopeSsVecType {
                        st = st,
                        stsem = stsem,
                        scope = scope,
                        ss = ss,
                        vec = vec,
                        type_ = type_,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for MultimemStWeakSsVecType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("multimem"),
                    string_p(".st"),
                    string_p(".weak"),
                    optional(Ss::parse()),
                    optional(Vec::parse()),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, st, weak, ss, vec, type_, a, _, b, _), span| {
                    ok!(MultimemStWeakSsVecType {
                        st = st,
                        weak = weak,
                        ss = ss,
                        vec = vec,
                        type_ = type_,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for MultimemRedRedsemScopeSsRedopVecRedtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("multimem"),
                    string_p(".red"),
                    optional(Redsem::parse()),
                    optional(Scope::parse()),
                    optional(Ss::parse()),
                    Redop::parse(),
                    optional(Vec::parse()),
                    Redtype::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, red, redsem, scope, ss, redop, vec, redtype, a, _, b, _), span| {
                    ok!(MultimemRedRedsemScopeSsRedopVecRedtype {
                        red = red,
                        redsem = redsem,
                        scope = scope,
                        ss = ss,
                        redop = redop,
                        vec = vec,
                        redtype = redtype,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }
}
