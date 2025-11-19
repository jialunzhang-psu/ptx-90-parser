//! Original PTX specification:
//!
//! // global -> shared::cta
//! cp.async.bulk.dst.src.completion_mechanism{.level::cache_hint} [dstMem], [srcMem], size, [mbar] {, cache-policy};
//! .dst =                  { .shared::cta };
//! .src =                  { .global };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .level::cache_hint =    { .L2::cache_hint };
//! ----------------------------------------------------------------
//! // global -> shared::cluster;
//! cp.async.bulk.dst.src.completion_mechanism{.multicast}{.level::cache_hint} [dstMem], [srcMem], size, [mbar] {, ctaMask} {, cache-policy};
//! .dst =                  { .shared::cluster };
//! .src =                  { .global };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .level::cache_hint =    { .L2::cache_hint };
//! .multicast =            { .multicast::cluster  };
//! ----------------------------------------------------------------
//! // shared::cta -> shared::cluster
//! cp.async.bulk.dst.src.completion_mechanism [dstMem], [srcMem], size, [mbar];
//! .dst =                  { .shared::cluster };
//! .src =                  { .shared::cta };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ----------------------------------------------------------------
//! // shared::cta -> global
//! cp.async.bulk.dst.src.completion_mechanism{.level::cache_hint}{.cp_mask} [dstMem], [srcMem], size {, cache-policy} {, byteMask};
//! .dst =                  { .global };
//! .src =                  { .shared::cta };
//! .completion_mechanism = { .bulk_group };
//! .level::cache_hint =    { .L2::cache_hint };

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
    use crate::r#type::instruction::cp_async_bulk::section_0::*;

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
            alt!(map(string_p(".shared::cta"), |_, _span| Dst::SharedCta))
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
            alt!(map(string_p(".global"), |_, _span| Src::Global))
        }
    }

    impl PtxParser for CpAsyncBulkDstSrcCompletionMechanismLevelCacheHint {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".bulk"),
                    Dst::parse(),
                    Src::parse(),
                    CompletionMechanism::parse(),
                    optional(LevelCacheHint::parse()),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(
                    _,
                    async_,
                    bulk,
                    dst,
                    src,
                    completion_mechanism,
                    level_cache_hint,
                    dstmem,
                    _,
                    srcmem,
                    _,
                    size,
                    _,
                    mbar,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(CpAsyncBulkDstSrcCompletionMechanismLevelCacheHint {
                        async_ = async_,
                        bulk = bulk,
                        dst = dst,
                        src = src,
                        completion_mechanism = completion_mechanism,
                        level_cache_hint = level_cache_hint,
                        dstmem = dstmem,
                        srcmem = srcmem,
                        size = size,
                        mbar = mbar,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk::section_1::*;

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

    impl PtxParser for LevelCacheHint {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".L2::cache_hint"), |_, _span| {
                LevelCacheHint::L2CacheHint
            }))
        }
    }

    impl PtxParser for Multicast {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".multicast::cluster"), |_, _span| {
                Multicast::MulticastCluster
            }))
        }
    }

    impl PtxParser for Src {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".global"), |_, _span| Src::Global))
        }
    }

    impl PtxParser for CpAsyncBulkDstSrcCompletionMechanismMulticastLevelCacheHint {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".bulk"),
                    Dst::parse(),
                    Src::parse(),
                    CompletionMechanism::parse(),
                    optional(Multicast::parse()),
                    optional(LevelCacheHint::parse()),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(
                    _,
                    async_,
                    bulk,
                    dst,
                    src,
                    completion_mechanism,
                    multicast,
                    level_cache_hint,
                    dstmem,
                    _,
                    srcmem,
                    _,
                    size,
                    _,
                    mbar,
                    ctamask,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(CpAsyncBulkDstSrcCompletionMechanismMulticastLevelCacheHint {
                        async_ = async_,
                        bulk = bulk,
                        dst = dst,
                        src = src,
                        completion_mechanism = completion_mechanism,
                        multicast = multicast,
                        level_cache_hint = level_cache_hint,
                        dstmem = dstmem,
                        srcmem = srcmem,
                        size = size,
                        mbar = mbar,
                        ctamask = ctamask,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }
}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk::section_2::*;

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

    impl PtxParser for Src {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".shared::cta"), |_, _span| Src::SharedCta))
        }
    }

    impl PtxParser for CpAsyncBulkDstSrcCompletionMechanism {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".bulk"),
                    Dst::parse(),
                    Src::parse(),
                    CompletionMechanism::parse(),
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
                    async_,
                    bulk,
                    dst,
                    src,
                    completion_mechanism,
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
                    ok!(CpAsyncBulkDstSrcCompletionMechanism {
                        async_ = async_,
                        bulk = bulk,
                        dst = dst,
                        src = src,
                        completion_mechanism = completion_mechanism,
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

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk::section_3::*;

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

    impl PtxParser for CpAsyncBulkDstSrcCompletionMechanismLevelCacheHintCpMask {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".bulk"),
                    Dst::parse(),
                    Src::parse(),
                    CompletionMechanism::parse(),
                    optional(LevelCacheHint::parse()),
                    map(optional(string_p(".cp_mask")), |value, _| value.is_some()),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(
                    _,
                    async_,
                    bulk,
                    dst,
                    src,
                    completion_mechanism,
                    level_cache_hint,
                    cp_mask,
                    dstmem,
                    _,
                    srcmem,
                    _,
                    size,
                    cache_policy,
                    bytemask,
                    _,
                ),
                 span| {
                    ok!(CpAsyncBulkDstSrcCompletionMechanismLevelCacheHintCpMask {
                        async_ = async_,
                        bulk = bulk,
                        dst = dst,
                        src = src,
                        completion_mechanism = completion_mechanism,
                        level_cache_hint = level_cache_hint,
                        cp_mask = cp_mask,
                        dstmem = dstmem,
                        srcmem = srcmem,
                        size = size,
                        cache_policy = cache_policy,
                        bytemask = bytemask,

                    })
                },
            )
        }
    }
}
