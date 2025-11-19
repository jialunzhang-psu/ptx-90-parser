//! Original PTX specification:
//!
//! // shared::cta -> global
//! cp.reduce.async.bulk.tensor.dim.dst.src.redOp{.load_mode}.completion_mechanism{.level::cache_hint} [tensorMap, tensorCoords], [srcMem] {,cache-policy};
//! .dst =                  { .global };
//! .src =                  { .shared::cta };
//! .dim =                  { .1d, .2d, .3d, .4d, .5d };
//! .completion_mechanism = { .bulk_group };
//! .load_mode =            { .tile, .im2col_no_offs };
//! .redOp =                { .add, .min, .max, .inc, .dec, .and, .or, .xor};

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
    use crate::r#type::instruction::cp_reduce_async_bulk_tensor::section_0::*;

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

    impl PtxParser for LoadMode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".im2col_no_offs"), |_, _span| {
                    LoadMode::Im2colNoOffs
                }),
                map(string_p(".tile"), |_, _span| LoadMode::Tile)
            )
        }
    }

    impl PtxParser for Redop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".add"), |_, _span| Redop::Add),
                map(string_p(".min"), |_, _span| Redop::Min),
                map(string_p(".max"), |_, _span| Redop::Max),
                map(string_p(".inc"), |_, _span| Redop::Inc),
                map(string_p(".dec"), |_, _span| Redop::Dec),
                map(string_p(".and"), |_, _span| Redop::And),
                map(string_p(".xor"), |_, _span| Redop::Xor),
                map(string_p(".or"), |_, _span| Redop::Or)
            )
        }
    }

    impl PtxParser for Src {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".shared::cta"), |_, _span| Src::SharedCta))
        }
    }

    impl PtxParser for CpReduceAsyncBulkTensorDimDstSrcRedopLoadModeCompletionMechanismLevelCacheHint {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".reduce"),
                    string_p(".async"),
                    string_p(".bulk"),
                    string_p(".tensor"),
                    Dim::parse(),
                    Dst::parse(),
                    Src::parse(),
                    Redop::parse(),
                    optional(LoadMode::parse()),
                    CompletionMechanism::parse(),
                    map(optional(string_p(".level::cache_hint")), |value, _| value
                        .is_some()),
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
                    reduce,
                    async_,
                    bulk,
                    tensor,
                    dim,
                    dst,
                    src,
                    redop,
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
                    ok!(CpReduceAsyncBulkTensorDimDstSrcRedopLoadModeCompletionMechanismLevelCacheHint {
                        reduce = reduce,
                        async_ = async_,
                        bulk = bulk,
                        tensor = tensor,
                        dim = dim,
                        dst = dst,
                        src = src,
                        redop = redop,
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
