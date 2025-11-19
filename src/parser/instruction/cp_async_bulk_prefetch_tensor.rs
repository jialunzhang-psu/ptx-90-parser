//! Original PTX specification:
//!
//! // global -> shared::cluster:
//! cp.async.bulk.prefetch.tensor.dim.L2.src{.load_mode}{.level::cache_hint} [tensorMap, tensorCoords] {, im2colInfo } {, cache-policy};
//! .src =                { .global };
//! .dim =                { .1d, .2d, .3d, .4d, .5d };
//! .load_mode =          { .tile, .tile::gather4, .im2col, .im2col::w, .im2col::w::128 };
//! .level::cache_hint =  { .L2::cache_hint };

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
    use crate::r#type::instruction::cp_async_bulk_prefetch_tensor::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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

    impl PtxParser for CpAsyncBulkPrefetchTensorDimL2SrcLoadModeLevelCacheHint {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".bulk"),
                    string_p(".prefetch"),
                    string_p(".tensor"),
                    Dim::parse(),
                    string_p(".L2"),
                    Src::parse(),
                    optional(LoadMode::parse()),
                    optional(LevelCacheHint::parse()),
                    TexHandler2::parse(),
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
                    prefetch,
                    tensor,
                    dim,
                    l2,
                    src,
                    load_mode,
                    level_cache_hint,
                    tensormap,
                    im2colinfo,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(CpAsyncBulkPrefetchTensorDimL2SrcLoadModeLevelCacheHint {
                        async_ = async_,
                        bulk = bulk,
                        prefetch = prefetch,
                        tensor = tensor,
                        dim = dim,
                        l2 = l2,
                        src = src,
                        load_mode = load_mode,
                        level_cache_hint = level_cache_hint,
                        tensormap = tensormap,
                        im2colinfo = im2colinfo,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }
}
