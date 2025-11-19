//! Original PTX specification:
//!
//! // Increment and Decrement reductions
//! red.async.sem.scope{.ss}.completion_mechanism.op.type [a], b, [mbar];
//! .sem  =                 { .relaxed };
//! .scope =                { .cluster };
//! .ss   =                 { .shared::cluster };
//! .op   =                 { .inc, .dec };
//! .type =                 { .u32 };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ------------------------------------------------------------------
//! // MIN and MAX reductions
//! red.async.sem.scope{.ss}.completion_mechanism.op.type [a], b, [mbar];
//! .sem  = { .relaxed };
//! .scope = { .cluster };
//! .ss   = { .shared::cluster };
//! .op   = { .min, .max };
//! .type = { .u32, .s32 };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ------------------------------------------------------------------
//! // Bitwise AND, OR and XOR reductions
//! red.async.sem.scope{.ss}.completion_mechanism.op.type [a], b, [mbar];
//! .sem  = { .relaxed };
//! .scope = { .cluster };
//! .ss   = { .shared::cluster };
//! .op   = { .and, .or, .xor };
//! .type = { .b32 };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ------------------------------------------------------------------
//! // ADD reductions
//! red.async.sem.scope{.ss}.completion_mechanism.add.type [a], b, [mbar];
//! .sem  = { .relaxed };
//! .scope = { .cluster };
//! .ss   = { .shared::cluster };
//! .type = { .u32, .s32, .u64 };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ----------------------------------------------------
//! // Alternate floating point type:
//! red.async{.mmio}.sem.scope{.ss}.add.type [a], b;
//! .sem  = { .release };
//! .scope = { .gpu, .cluster };
//! .ss   = { .global };
//! .type = { .u32, .s32, .u64, .s64 };

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
    use crate::r#type::instruction::red_async::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CompletionMechanism {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(
                string_p(".mbarrier::complete_tx::bytes"),
                |_, _span| CompletionMechanism::MbarrierCompleteTxBytes
            ))
        }
    }

    impl PtxParser for Op {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".inc"), |_, _span| Op::Inc),
                map(string_p(".dec"), |_, _span| Op::Dec)
            )
        }
    }

    impl PtxParser for Scope {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".cluster"), |_, _span| Scope::Cluster))
        }
    }

    impl PtxParser for Sem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".relaxed"), |_, _span| Sem::Relaxed))
        }
    }

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".shared::cluster"), |_, _span| {
                Ss::SharedCluster
            }))
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".u32"), |_, _span| Type::U32))
        }
    }

    impl PtxParser for RedAsyncSemScopeSsCompletionMechanismOpType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("red"),
                    string_p(".async"),
                    Sem::parse(),
                    Scope::parse(),
                    optional(Ss::parse()),
                    CompletionMechanism::parse(),
                    Op::parse(),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    async_,
                    sem,
                    scope,
                    ss,
                    completion_mechanism,
                    op,
                    type_,
                    a,
                    _,
                    b,
                    _,
                    mbar,
                    _,
                ),
                 span| {
                    ok!(RedAsyncSemScopeSsCompletionMechanismOpType {
                        async_ = async_,
                        sem = sem,
                        scope = scope,
                        ss = ss,
                        completion_mechanism = completion_mechanism,
                        op = op,
                        type_ = type_,
                        a = a,
                        b = b,
                        mbar = mbar,

                    })
                },
            )
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::red_async::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CompletionMechanism {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(
                string_p(".mbarrier::complete_tx::bytes"),
                |_, _span| CompletionMechanism::MbarrierCompleteTxBytes
            ))
        }
    }

    impl PtxParser for Op {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".min"), |_, _span| Op::Min),
                map(string_p(".max"), |_, _span| Op::Max)
            )
        }
    }

    impl PtxParser for Scope {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".cluster"), |_, _span| Scope::Cluster))
        }
    }

    impl PtxParser for Sem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".relaxed"), |_, _span| Sem::Relaxed))
        }
    }

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".shared::cluster"), |_, _span| {
                Ss::SharedCluster
            }))
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".s32"), |_, _span| Type::S32)
            )
        }
    }

    impl PtxParser for RedAsyncSemScopeSsCompletionMechanismOpType1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("red"),
                    string_p(".async"),
                    Sem::parse(),
                    Scope::parse(),
                    optional(Ss::parse()),
                    CompletionMechanism::parse(),
                    Op::parse(),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    async_,
                    sem,
                    scope,
                    ss,
                    completion_mechanism,
                    op,
                    type_,
                    a,
                    _,
                    b,
                    _,
                    mbar,
                    _,
                ),
                 span| {
                    ok!(RedAsyncSemScopeSsCompletionMechanismOpType1 {
                        async_ = async_,
                        sem = sem,
                        scope = scope,
                        ss = ss,
                        completion_mechanism = completion_mechanism,
                        op = op,
                        type_ = type_,
                        a = a,
                        b = b,
                        mbar = mbar,

                    })
                },
            )
        }
    }
}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::red_async::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CompletionMechanism {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(
                string_p(".mbarrier::complete_tx::bytes"),
                |_, _span| CompletionMechanism::MbarrierCompleteTxBytes
            ))
        }
    }

    impl PtxParser for Op {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".and"), |_, _span| Op::And),
                map(string_p(".xor"), |_, _span| Op::Xor),
                map(string_p(".or"), |_, _span| Op::Or)
            )
        }
    }

    impl PtxParser for Scope {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".cluster"), |_, _span| Scope::Cluster))
        }
    }

    impl PtxParser for Sem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".relaxed"), |_, _span| Sem::Relaxed))
        }
    }

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".shared::cluster"), |_, _span| {
                Ss::SharedCluster
            }))
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".b32"), |_, _span| Type::B32))
        }
    }

    impl PtxParser for RedAsyncSemScopeSsCompletionMechanismOpType2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("red"),
                    string_p(".async"),
                    Sem::parse(),
                    Scope::parse(),
                    optional(Ss::parse()),
                    CompletionMechanism::parse(),
                    Op::parse(),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    async_,
                    sem,
                    scope,
                    ss,
                    completion_mechanism,
                    op,
                    type_,
                    a,
                    _,
                    b,
                    _,
                    mbar,
                    _,
                ),
                 span| {
                    ok!(RedAsyncSemScopeSsCompletionMechanismOpType2 {
                        async_ = async_,
                        sem = sem,
                        scope = scope,
                        ss = ss,
                        completion_mechanism = completion_mechanism,
                        op = op,
                        type_ = type_,
                        a = a,
                        b = b,
                        mbar = mbar,

                    })
                },
            )
        }
    }
}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::red_async::section_3::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CompletionMechanism {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(
                string_p(".mbarrier::complete_tx::bytes"),
                |_, _span| CompletionMechanism::MbarrierCompleteTxBytes
            ))
        }
    }

    impl PtxParser for Scope {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".cluster"), |_, _span| Scope::Cluster))
        }
    }

    impl PtxParser for Sem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".relaxed"), |_, _span| Sem::Relaxed))
        }
    }

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".shared::cluster"), |_, _span| {
                Ss::SharedCluster
            }))
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".s32"), |_, _span| Type::S32),
                map(string_p(".u64"), |_, _span| Type::U64)
            )
        }
    }

    impl PtxParser for RedAsyncSemScopeSsCompletionMechanismAddType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("red"),
                    string_p(".async"),
                    Sem::parse(),
                    Scope::parse(),
                    optional(Ss::parse()),
                    CompletionMechanism::parse(),
                    string_p(".add"),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    async_,
                    sem,
                    scope,
                    ss,
                    completion_mechanism,
                    add,
                    type_,
                    a,
                    _,
                    b,
                    _,
                    mbar,
                    _,
                ),
                 span| {
                    ok!(RedAsyncSemScopeSsCompletionMechanismAddType {
                        async_ = async_,
                        sem = sem,
                        scope = scope,
                        ss = ss,
                        completion_mechanism = completion_mechanism,
                        add = add,
                        type_ = type_,
                        a = a,
                        b = b,
                        mbar = mbar,

                    })
                },
            )
        }
    }
}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::red_async::section_4::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Scope {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cluster"), |_, _span| Scope::Cluster),
                map(string_p(".gpu"), |_, _span| Scope::Gpu)
            )
        }
    }

    impl PtxParser for Sem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".release"), |_, _span| Sem::Release))
        }
    }

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".global"), |_, _span| Ss::Global))
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".s32"), |_, _span| Type::S32),
                map(string_p(".u64"), |_, _span| Type::U64),
                map(string_p(".s64"), |_, _span| Type::S64)
            )
        }
    }

    impl PtxParser for RedAsyncMmioSemScopeSsAddType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("red"),
                    string_p(".async"),
                    map(optional(string_p(".mmio")), |value, _| value.is_some()),
                    Sem::parse(),
                    Scope::parse(),
                    optional(Ss::parse()),
                    string_p(".add"),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, async_, mmio, sem, scope, ss, add, type_, a, _, b, _), span| {
                    ok!(RedAsyncMmioSemScopeSsAddType {
                        async_ = async_,
                        mmio = mmio,
                        sem = sem,
                        scope = scope,
                        ss = ss,
                        add = add,
                        type_ = type_,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }
}
