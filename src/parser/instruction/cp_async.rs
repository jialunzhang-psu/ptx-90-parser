//! Original PTX specification:
//!
//! cp.async.ca.state.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], cp-size{, src-size}{, cache-policy};
//! cp.async.cg.state.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], 16{, src-size}{, cache-policy};
//! cp.async.ca.state.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], cp-size{, ignore-src}{, cache-policy} ;
//! cp.async.cg.state.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], 16{, ignore-src}{, cache-policy} ;
//! .level::cache_hint =     { .L2::cache_hint };
//! .level::prefetch_size =  { .L2::64B, .L2::128B, .L2::256B };
//! cp-size = { 4, 8, 16 };
//! .state = { .shared, .shared::cta}

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
    use crate::r#type::instruction::cp_async::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CpSize {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p("16"), |_, _span| CpSize::_16),
                map(string_p("4"), |_, _span| CpSize::_4),
                map(string_p("8"), |_, _span| CpSize::_8)
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

    impl PtxParser for LevelPrefetchSize {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".L2::128B"), |_, _span| LevelPrefetchSize::L2128b),
                map(string_p(".L2::256B"), |_, _span| LevelPrefetchSize::L2256b),
                map(string_p(".L2::64B"), |_, _span| LevelPrefetchSize::L264b)
            )
        }
    }

    impl PtxParser for State {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".shared::cta"), |_, _span| State::SharedCta),
                map(string_p(".shared"), |_, _span| State::Shared)
            )
        }
    }

    impl PtxParser for CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".ca"),
                    State::parse(),
                    string_p(".global"),
                    optional(LevelCacheHint::parse()),
                    optional(LevelPrefetchSize::parse()),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    CpSize::parse(),
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
                    ca,
                    state,
                    global,
                    level_cache_hint,
                    level_prefetch_size,
                    dst,
                    _,
                    src,
                    _,
                    cp_size,
                    src_size,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize {
                        async_ = async_,
                        ca = ca,
                        state = state,
                        global = global,
                        level_cache_hint = level_cache_hint,
                        level_prefetch_size = level_prefetch_size,
                        dst = dst,
                        src = src,
                        cp_size = cp_size,
                        src_size = src_size,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".cg"),
                    State::parse(),
                    string_p(".global"),
                    optional(LevelCacheHint::parse()),
                    optional(LevelPrefetchSize::parse()),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    map(string_p("16"), |_, _| ()),
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
                    cg,
                    state,
                    global,
                    level_cache_hint,
                    level_prefetch_size,
                    dst,
                    _,
                    src,
                    _,
                    imm_16,
                    src_size,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize {
                        async_ = async_,
                        cg = cg,
                        state = state,
                        global = global,
                        level_cache_hint = level_cache_hint,
                        level_prefetch_size = level_prefetch_size,
                        dst = dst,
                        src = src,
                        imm_16 = imm_16,
                        src_size = src_size,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".ca"),
                    State::parse(),
                    string_p(".global"),
                    optional(LevelCacheHint::parse()),
                    optional(LevelPrefetchSize::parse()),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    CpSize::parse(),
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
                    ca,
                    state,
                    global,
                    level_cache_hint,
                    level_prefetch_size,
                    dst,
                    _,
                    src,
                    _,
                    cp_size,
                    ignore_src,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize1 {
                        async_ = async_,
                        ca = ca,
                        state = state,
                        global = global,
                        level_cache_hint = level_cache_hint,
                        level_prefetch_size = level_prefetch_size,
                        dst = dst,
                        src = src,
                        cp_size = cp_size,
                        ignore_src = ignore_src,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".cg"),
                    State::parse(),
                    string_p(".global"),
                    optional(LevelCacheHint::parse()),
                    optional(LevelPrefetchSize::parse()),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    map(string_p("16"), |_, _| ()),
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
                    cg,
                    state,
                    global,
                    level_cache_hint,
                    level_prefetch_size,
                    dst,
                    _,
                    src,
                    _,
                    imm_16,
                    ignore_src,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize1 {
                        async_ = async_,
                        cg = cg,
                        state = state,
                        global = global,
                        level_cache_hint = level_cache_hint,
                        level_prefetch_size = level_prefetch_size,
                        dst = dst,
                        src = src,
                        imm_16 = imm_16,
                        ignore_src = ignore_src,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }
}
