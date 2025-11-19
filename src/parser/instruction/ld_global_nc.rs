//! Original PTX specification:
//!
//! ld.global{.cop}.nc{.level::cache_hint}{.level::prefetch_size}.type                 d, [a]{, cache-policy};
//! ld.global{.cop}.nc{.level::cache_hint}{.level::prefetch_size}.vec.type             d, [a]{, cache-policy};
//! ld.global.nc{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.level::prefetch_size}.type      d, [a]{, cache-policy};
//! ld.global.nc{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.level::prefetch_size}.vec.type  d, [a]{, cache-policy};
//! .cop  =                     { .ca, .cg, .cs };     // cache operation
//! .level1::eviction_priority = { .L1::evict_normal, .L1::evict_unchanged, .L1::evict_first, .L1::evict_last, .L1::no_allocate};
//! .level2::eviction_priority = {.L2::evict_normal, .L2::evict_first, .L2::evict_last};
//! .level::cache_hint =        { .L2::cache_hint };
//! .level::prefetch_size =     { .L2::64B, .L2::128B, .L2::256B };
//! .vec  =                     { .v2, .v4, .v8 };
//! .type =                     { .b8, .b16, .b32, .b64, .b128,
//! .u8, .u16, .u32, .u64,
//! .s8, .s16, .s32, .s64,
//! .f32, .f64 };

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
    use crate::r#type::instruction::ld_global_nc::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Cop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".ca"), |_, _span| Cop::Ca),
                map(string_p(".cg"), |_, _span| Cop::Cg),
                map(string_p(".cs"), |_, _span| Cop::Cs)
            )
        }
    }

    impl PtxParser for Level1EvictionPriority {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".L1::evict_unchanged"), |_, _span| {
                    Level1EvictionPriority::L1EvictUnchanged
                }),
                map(string_p(".L1::evict_normal"), |_, _span| {
                    Level1EvictionPriority::L1EvictNormal
                }),
                map(string_p(".L1::evict_first"), |_, _span| {
                    Level1EvictionPriority::L1EvictFirst
                }),
                map(string_p(".L1::no_allocate"), |_, _span| {
                    Level1EvictionPriority::L1NoAllocate
                }),
                map(string_p(".L1::evict_last"), |_, _span| {
                    Level1EvictionPriority::L1EvictLast
                })
            )
        }
    }

    impl PtxParser for Level2EvictionPriority {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".L2::evict_normal"), |_, _span| {
                    Level2EvictionPriority::L2EvictNormal
                }),
                map(string_p(".L2::evict_first"), |_, _span| {
                    Level2EvictionPriority::L2EvictFirst
                }),
                map(string_p(".L2::evict_last"), |_, _span| {
                    Level2EvictionPriority::L2EvictLast
                })
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

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b128"), |_, _span| Type::B128),
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
                map(string_p(".f64"), |_, _span| Type::F64),
                map(string_p(".b8"), |_, _span| Type::B8),
                map(string_p(".u8"), |_, _span| Type::U8),
                map(string_p(".s8"), |_, _span| Type::S8)
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

    impl PtxParser for LdGlobalCopNcLevelCacheHintLevelPrefetchSizeType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ld"),
                    string_p(".global"),
                    optional(Cop::parse()),
                    string_p(".nc"),
                    optional(LevelCacheHint::parse()),
                    optional(LevelPrefetchSize::parse()),
                    Type::parse(),
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
                    global,
                    cop,
                    nc,
                    level_cache_hint,
                    level_prefetch_size,
                    type_,
                    d,
                    _,
                    a,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(LdGlobalCopNcLevelCacheHintLevelPrefetchSizeType {
                        global = global,
                        cop = cop,
                        nc = nc,
                        level_cache_hint = level_cache_hint,
                        level_prefetch_size = level_prefetch_size,
                        type_ = type_,
                        d = d,
                        a = a,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for LdGlobalCopNcLevelCacheHintLevelPrefetchSizeVecType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ld"),
                    string_p(".global"),
                    optional(Cop::parse()),
                    string_p(".nc"),
                    optional(LevelCacheHint::parse()),
                    optional(LevelPrefetchSize::parse()),
                    Vec::parse(),
                    Type::parse(),
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
                    global,
                    cop,
                    nc,
                    level_cache_hint,
                    level_prefetch_size,
                    vec,
                    type_,
                    d,
                    _,
                    a,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(LdGlobalCopNcLevelCacheHintLevelPrefetchSizeVecType {
                        global = global,
                        cop = cop,
                        nc = nc,
                        level_cache_hint = level_cache_hint,
                        level_prefetch_size = level_prefetch_size,
                        vec = vec,
                        type_ = type_,
                        d = d,
                        a = a,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ld"),
                    string_p(".global"),
                    string_p(".nc"),
                    optional(Level1EvictionPriority::parse()),
                    optional(Level2EvictionPriority::parse()),
                    optional(LevelCacheHint::parse()),
                    optional(LevelPrefetchSize::parse()),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(optional(seq_n!(comma_p(), GeneralOperand::parse())), |value, _| value.map(|(_, operand)| operand)),
                    semicolon_p()
                ),
                |(_, global, nc, level1_eviction_priority, level2_eviction_priority, level_cache_hint, level_prefetch_size, type_, d, _, a, cache_policy, _), span| {
                    ok!(LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeType {
                        global = global,
                        nc = nc,
                        level1_eviction_priority = level1_eviction_priority,
                        level2_eviction_priority = level2_eviction_priority,
                        level_cache_hint = level_cache_hint,
                        level_prefetch_size = level_prefetch_size,
                        type_ = type_,
                        d = d,
                        a = a,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ld"),
                    string_p(".global"),
                    string_p(".nc"),
                    optional(Level1EvictionPriority::parse()),
                    optional(Level2EvictionPriority::parse()),
                    optional(LevelCacheHint::parse()),
                    optional(LevelPrefetchSize::parse()),
                    Vec::parse(),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(optional(seq_n!(comma_p(), GeneralOperand::parse())), |value, _| value.map(|(_, operand)| operand)),
                    semicolon_p()
                ),
                |(_, global, nc, level1_eviction_priority, level2_eviction_priority, level_cache_hint, level_prefetch_size, vec, type_, d, _, a, cache_policy, _), span| {
                    ok!(LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
                        global = global,
                        nc = nc,
                        level1_eviction_priority = level1_eviction_priority,
                        level2_eviction_priority = level2_eviction_priority,
                        level_cache_hint = level_cache_hint,
                        level_prefetch_size = level_prefetch_size,
                        vec = vec,
                        type_ = type_,
                        d = d,
                        a = a,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }
}
