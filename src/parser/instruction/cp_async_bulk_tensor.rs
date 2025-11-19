//! Original PTX specification:
//!
//! // global -> shared::cta
//! cp.async.bulk.tensor.dim.dst.src{.load_mode}.completion_mechanism{.cta_group}{.level::cache_hint} [dstMem], [tensorMap, tensorCoords], [mbar]{, im2colInfo} {, cache-policy};
//! .dst =                  { .shared::cta };
//! .src =                  { .global };
//! .dim =                  { .1d, .2d, .3d, .4d, .5d };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .cta_group =            { .cta_group::1, .cta_group::2 };
//! .load_mode =            { .tile, .tile::gather4, .im2col, .im2col::w, .im2col::w::128 };
//! .level::cache_hint =    { .L2::cache_hint };
//! ----------------------------------------------------------------
//! // global -> shared::cluster
//! cp.async.bulk.tensor.dim.dst.src{.load_mode}.completion_mechanism{.multicast}{.cta_group}{.level::cache_hint} [dstMem], [tensorMap, tensorCoords], [mbar]{, im2colInfo} {, ctaMask} {, cache-policy};
//! .dst =                  { .shared::cluster };
//! .src =                  { .global };
//! .dim =                  { .1d, .2d, .3d, .4d, .5d };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .cta_group =            { .cta_group::1, .cta_group::2 };
//! .load_mode =            { .tile, .tile::gather4, .im2col, .im2col::w, .im2col::w::128 };
//! .level::cache_hint =    { .L2::cache_hint };
//! .multicast =            { .multicast::cluster  };
//! ----------------------------------------------------------------
//! // shared::cta -> global;
//! cp.async.bulk.tensor.dim.dst.src{.load_mode}.completion_mechanism{.level::cache_hint} [tensorMap, tensorCoords], [srcMem] {, cache-policy};
//! .dst =                  { .global };
//! .src =                  { .shared::cta };
//! .dim =                  { .1d, .2d, .3d, .4d, .5d };
//! .completion_mechanism = { .bulk_group };
//! .load_mode =            { .tile, .tile::scatter4, .im2col_no_offs };
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
    use crate::r#type::instruction::cp_async_bulk_tensor::section_0::*;

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

    impl PtxParser for CtaGroup {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cta_group::1"), |_, _span| CtaGroup::CtaGroup1),
                map(string_p(".cta_group::2"), |_, _span| CtaGroup::CtaGroup2)
            )
        }
    }

    impl PtxParser for Dim {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".1d"), |_, _span| Dim::_1d),
                map(string_p(".2d"), |_, _span| Dim::_2d),
                map(string_p(".3d"), |_, _span| Dim::_3d),
                map(string_p(".4d"), |_, _span| Dim::_4d),
                map(string_p(".5d"), |_, _span| Dim::_5d)
            )
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

    impl PtxParser for LoadMode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".im2col::w::128"), |_, _span| LoadMode::Im2colW128),
                map(string_p(".tile::gather4"), |_, _span| LoadMode::TileGather4),
                map(string_p(".im2col::w"), |_, _span| LoadMode::Im2colW),
                map(string_p(".im2col"), |_, _span| LoadMode::Im2col),
                map(string_p(".tile"), |_, _span| LoadMode::Tile)
            )
        }
    }

    impl PtxParser for Src {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".global"), |_, _span| Src::Global))
        }
    }

    impl PtxParser for CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismCtaGroupLevelCacheHint {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".bulk"),
                    string_p(".tensor"),
                    Dim::parse(),
                    Dst::parse(),
                    Src::parse(),
                    optional(LoadMode::parse()),
                    CompletionMechanism::parse(),
                    optional(CtaGroup::parse()),
                    optional(LevelCacheHint::parse()),
                    AddressOperand::parse(),
                    comma_p(),
                    TexHandler2::parse(),
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
                    tensor,
                    dim,
                    dst,
                    src,
                    load_mode,
                    completion_mechanism,
                    cta_group,
                    level_cache_hint,
                    dstmem,
                    _,
                    tensormap,
                    _,
                    mbar,
                    im2colinfo,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismCtaGroupLevelCacheHint {
                        async_ = async_,
                        bulk = bulk,
                        tensor = tensor,
                        dim = dim,
                        dst = dst,
                        src = src,
                        load_mode = load_mode,
                        completion_mechanism = completion_mechanism,
                        cta_group = cta_group,
                        level_cache_hint = level_cache_hint,
                        dstmem = dstmem,
                        tensormap = tensormap,
                        mbar = mbar,
                        im2colinfo = im2colinfo,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk_tensor::section_1::*;

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

    impl PtxParser for CtaGroup {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cta_group::1"), |_, _span| CtaGroup::CtaGroup1),
                map(string_p(".cta_group::2"), |_, _span| CtaGroup::CtaGroup2)
            )
        }
    }

    impl PtxParser for Dim {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".1d"), |_, _span| Dim::_1d),
                map(string_p(".2d"), |_, _span| Dim::_2d),
                map(string_p(".3d"), |_, _span| Dim::_3d),
                map(string_p(".4d"), |_, _span| Dim::_4d),
                map(string_p(".5d"), |_, _span| Dim::_5d)
            )
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

    impl PtxParser for LoadMode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".im2col::w::128"), |_, _span| LoadMode::Im2colW128),
                map(string_p(".tile::gather4"), |_, _span| LoadMode::TileGather4),
                map(string_p(".im2col::w"), |_, _span| LoadMode::Im2colW),
                map(string_p(".im2col"), |_, _span| LoadMode::Im2col),
                map(string_p(".tile"), |_, _span| LoadMode::Tile)
            )
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

    impl PtxParser
        for CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismMulticastCtaGroupLevelCacheHint
    {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".bulk"),
                    string_p(".tensor"),
                    Dim::parse(),
                    Dst::parse(),
                    Src::parse(),
                    optional(LoadMode::parse()),
                    CompletionMechanism::parse(),
                    optional(Multicast::parse()),
                    optional(CtaGroup::parse()),
                    optional(LevelCacheHint::parse()),
                    AddressOperand::parse(),
                    comma_p(),
                    TexHandler2::parse(),
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
                    tensor,
                    dim,
                    dst,
                    src,
                    load_mode,
                    completion_mechanism,
                    multicast,
                    cta_group,
                    level_cache_hint,
                    dstmem,
                    _,
                    tensormap,
                    _,
                    mbar,
                    im2colinfo,
                    ctamask,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismMulticastCtaGroupLevelCacheHint {
                        async_ = async_,
                        bulk = bulk,
                        tensor = tensor,
                        dim = dim,
                        dst = dst,
                        src = src,
                        load_mode = load_mode,
                        completion_mechanism = completion_mechanism,
                        multicast = multicast,
                        cta_group = cta_group,
                        level_cache_hint = level_cache_hint,
                        dstmem = dstmem,
                        tensormap = tensormap,
                        mbar = mbar,
                        im2colinfo = im2colinfo,
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
    use crate::r#type::instruction::cp_async_bulk_tensor::section_2::*;

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

    impl PtxParser for Dim {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".1d"), |_, _span| Dim::_1d),
                map(string_p(".2d"), |_, _span| Dim::_2d),
                map(string_p(".3d"), |_, _span| Dim::_3d),
                map(string_p(".4d"), |_, _span| Dim::_4d),
                map(string_p(".5d"), |_, _span| Dim::_5d)
            )
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

    impl PtxParser for LoadMode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".tile::scatter4"), |_, _span| {
                    LoadMode::TileScatter4
                }),
                map(string_p(".im2col_no_offs"), |_, _span| {
                    LoadMode::Im2colNoOffs
                }),
                map(string_p(".tile"), |_, _span| LoadMode::Tile)
            )
        }
    }

    impl PtxParser for Src {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".shared::cta"), |_, _span| Src::SharedCta))
        }
    }

    impl PtxParser for CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismLevelCacheHint {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".bulk"),
                    string_p(".tensor"),
                    Dim::parse(),
                    Dst::parse(),
                    Src::parse(),
                    optional(LoadMode::parse()),
                    CompletionMechanism::parse(),
                    optional(LevelCacheHint::parse()),
                    TexHandler2::parse(),
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
                    tensor,
                    dim,
                    dst,
                    src,
                    load_mode,
                    completion_mechanism,
                    level_cache_hint,
                    tensormap,
                    _,
                    srcmem,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismLevelCacheHint {
                        async_ = async_,
                        bulk = bulk,
                        tensor = tensor,
                        dim = dim,
                        dst = dst,
                        src = src,
                        load_mode = load_mode,
                        completion_mechanism = completion_mechanism,
                        level_cache_hint = level_cache_hint,
                        tensormap = tensormap,
                        srcmem = srcmem,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }
}
