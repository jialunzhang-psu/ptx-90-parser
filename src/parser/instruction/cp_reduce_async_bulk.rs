//! Original PTX specification:
//!
//! cp.reduce.async.bulk.dst.src.completion_mechanism.redOp.type [dstMem], [srcMem], size, [mbar];
//! .dst =                  { .shared::cluster };
//! .src =                  { .shared::cta };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .redOp=                 { .and, .or, .xor, .add, .inc, .dec, .min, .max };
//! .type =                 { .b32, .u32, .s32, .b64, .u64 };
//! ----------------------------------------------------------------
//! cp.reduce.async.bulk.dst.src.completion_mechanism{.level::cache_hint}.redOp.type [dstMem], [srcMem], size{, cache-policy};
//! .dst =                  { .global      };
//! .src =                  { .shared::cta };
//! ----------------------------------------------------------------
//! .completion_mechanism = { .bulk_group };
//! .level::cache_hint    = { .L2::cache_hint };
//! .redOp=                 { .and, .or, .xor, .add, .inc, .dec, .min, .max };
//! .type =                 { .f16, .bf16, .b32, .u32, .s32, .b64, .u64, .s64, .f32, .f64 };
//! ----------------------------------------------------------------
//! cp.reduce.async.bulk.dst.src.completion_mechanism{.level::cache_hint}.add.noftz.type [dstMem], [srcMem], size{, cache-policy};
//! .dst  =                 { .global };
//! .src  =                 { .shared::cta };
//! .completion_mechanism = { .bulk_group };
//! .type =                 { .f16, .bf16 };

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
    use crate::r#type::instruction::cp_reduce_async_bulk::section_0::*;

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

    impl PtxParser for Dst {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".shared::cluster"), |_, _span| {
                Dst::SharedCluster
            }))
        }
    }

    impl PtxParser for Redop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".and"), |_, _span| Redop::And),
                map(string_p(".xor"), |_, _span| Redop::Xor),
                map(string_p(".add"), |_, _span| Redop::Add),
                map(string_p(".inc"), |_, _span| Redop::Inc),
                map(string_p(".dec"), |_, _span| Redop::Dec),
                map(string_p(".min"), |_, _span| Redop::Min),
                map(string_p(".max"), |_, _span| Redop::Max),
                map(string_p(".or"), |_, _span| Redop::Or)
            )
        }
    }

    impl PtxParser for Src {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".shared::cta"), |_, _span| Src::SharedCta))
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b32"), |_, _span| Type::B32),
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".s32"), |_, _span| Type::S32),
                map(string_p(".b64"), |_, _span| Type::B64),
                map(string_p(".u64"), |_, _span| Type::U64)
            )
        }
    }

    impl PtxParser for CpReduceAsyncBulkDstSrcCompletionMechanismRedopType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".reduce"),
                    string_p(".async"),
                    string_p(".bulk"),
                    Dst::parse(),
                    Src::parse(),
                    CompletionMechanism::parse(),
                    Redop::parse(),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    reduce,
                    async_,
                    bulk,
                    dst,
                    src,
                    completion_mechanism,
                    redop,
                    type_,
                    dstmem,
                    _,
                    srcmem,
                    _,
                    size,
                    _,
                    mbar,
                    _,
                ),
                 span| {
                    ok!(CpReduceAsyncBulkDstSrcCompletionMechanismRedopType {
                        reduce = reduce,
                        async_ = async_,
                        bulk = bulk,
                        dst = dst,
                        src = src,
                        completion_mechanism = completion_mechanism,
                        redop = redop,
                        type_ = type_,
                        dstmem = dstmem,
                        srcmem = srcmem,
                        size = size,
                        mbar = mbar,

                    })
                },
            )
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::cp_reduce_async_bulk::section_1::*;

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

    impl PtxParser for Dst {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".global"), |_, _span| Dst::Global))
        }
    }

    impl PtxParser for Redop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".and"), |_, _span| Redop::And),
                map(string_p(".xor"), |_, _span| Redop::Xor),
                map(string_p(".add"), |_, _span| Redop::Add),
                map(string_p(".inc"), |_, _span| Redop::Inc),
                map(string_p(".dec"), |_, _span| Redop::Dec),
                map(string_p(".min"), |_, _span| Redop::Min),
                map(string_p(".max"), |_, _span| Redop::Max),
                map(string_p(".or"), |_, _span| Redop::Or)
            )
        }
    }

    impl PtxParser for Src {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".shared::cta"), |_, _span| Src::SharedCta))
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b32"), |_, _span| Type::B32),
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".s32"), |_, _span| Type::S32),
                map(string_p(".b64"), |_, _span| Type::B64),
                map(string_p(".u64"), |_, _span| Type::U64)
            )
        }
    }

    impl PtxParser for CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintRedopType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".reduce"),
                    string_p(".async"),
                    string_p(".bulk"),
                    Dst::parse(),
                    Src::parse(),
                    CompletionMechanism::parse(),
                    map(optional(string_p(".level::cache_hint")), |value, _| value
                        .is_some()),
                    Redop::parse(),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(
                    _,
                    reduce,
                    async_,
                    bulk,
                    dst,
                    src,
                    completion_mechanism,
                    level_cache_hint,
                    redop,
                    type_,
                    dstmem,
                    _,
                    srcmem,
                    _,
                    size,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintRedopType {
                        reduce = reduce,
                        async_ = async_,
                        bulk = bulk,
                        dst = dst,
                        src = src,
                        completion_mechanism = completion_mechanism,
                        level_cache_hint = level_cache_hint,
                        redop = redop,
                        type_ = type_,
                        dstmem = dstmem,
                        srcmem = srcmem,
                        size = size,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }
}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::cp_reduce_async_bulk::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CompletionMechanism {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".bulk_group"), |_, _span| {
                CompletionMechanism::BulkGroup
            }))
        }
    }

    impl PtxParser for Dst {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".global"), |_, _span| Dst::Global))
        }
    }

    impl PtxParser for LevelCacheHint {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".L2::cache_hint"), |_, _span| {
                LevelCacheHint::L2CacheHint
            }))
        }
    }

    impl PtxParser for Src {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".shared::cta"), |_, _span| Src::SharedCta))
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".bf16"), |_, _span| Type::Bf16),
                map(string_p(".f16"), |_, _span| Type::F16)
            )
        }
    }

    impl PtxParser for CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintAddNoftzType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".reduce"),
                    string_p(".async"),
                    string_p(".bulk"),
                    Dst::parse(),
                    Src::parse(),
                    CompletionMechanism::parse(),
                    optional(LevelCacheHint::parse()),
                    string_p(".add"),
                    string_p(".noftz"),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(
                    _,
                    reduce,
                    async_,
                    bulk,
                    dst,
                    src,
                    completion_mechanism,
                    level_cache_hint,
                    add,
                    noftz,
                    type_,
                    dstmem,
                    _,
                    srcmem,
                    _,
                    size,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintAddNoftzType {
                        reduce = reduce,
                        async_ = async_,
                        bulk = bulk,
                        dst = dst,
                        src = src,
                        completion_mechanism = completion_mechanism,
                        level_cache_hint = level_cache_hint,
                        add = add,
                        noftz = noftz,
                        type_ = type_,
                        dstmem = dstmem,
                        srcmem = srcmem,
                        size = size,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }
}
