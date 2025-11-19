//! Original PTX specification:
//!
//! cp.async.bulk.prefetch.L2.src{.level::cache_hint}   [srcMem], size {, cache-policy};
//! .src =                { .global };
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
    use crate::r#type::instruction::cp_async_bulk_prefetch::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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

    impl PtxParser for CpAsyncBulkPrefetchL2SrcLevelCacheHint {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".bulk"),
                    string_p(".prefetch"),
                    string_p(".L2"),
                    Src::parse(),
                    optional(LevelCacheHint::parse()),
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
                    async_,
                    bulk,
                    prefetch,
                    l2,
                    src,
                    level_cache_hint,
                    srcmem,
                    _,
                    size,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(CpAsyncBulkPrefetchL2SrcLevelCacheHint {
                        async_ = async_,
                        bulk = bulk,
                        prefetch = prefetch,
                        l2 = l2,
                        src = src,
                        level_cache_hint = level_cache_hint,
                        srcmem = srcmem,
                        size = size,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }
}
