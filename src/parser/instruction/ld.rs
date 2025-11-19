//! Original PTX specification:
//!
//! ld{.weak}{.ss}{.cop}{.level::cache_hint}{.level::prefetch_size}{.vec}.type  d, [a]{.unified}{, cache-policy};
//! ld{.weak}{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type  d, [a]{.unified}{, cache-policy};
//! ld.volatile{.ss}{.level::prefetch_size}{.vec}.type  d, [a];
//! ld.relaxed.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type  d, [a]{, cache-policy};
//! ld.acquire.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type  d, [a]{, cache-policy};
//! ld.mmio.relaxed.sys{.global}.type  d, [a];
//! .ss =                       { .const, .global, .local, .param::entry, .param::func, .param, .shared, .shared::cta, .shared::cluster};
//! .cop =                      { .ca, .cg, .cs, .lu, .cv };
//! .level1::eviction_priority = { .L1::evict_normal, .L1::evict_unchanged, .L1::evict_first, .L1::evict_last, .L1::no_allocate };
//! .level2::eviction_priority = {.L2::evict_normal, .L2::evict_first, .L2::evict_last};
//! .level::cache_hint =        { .L2::cache_hint };
//! .level::prefetch_size =     { .L2::64B, .L2::128B, .L2::256B };
//! .scope =                    { .cta, .cluster, .gpu, .sys };
//! .vec =                      { .v2, .v4, .v8 };
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
    use crate::r#type::instruction::ld::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Cop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".ca"), |_, _span| Cop::Ca),
                map(string_p(".cg"), |_, _span| Cop::Cg),
                map(string_p(".cs"), |_, _span| Cop::Cs),
                map(string_p(".lu"), |_, _span| Cop::Lu),
                map(string_p(".cv"), |_, _span| Cop::Cv)
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
            alt!(
                map(string_p(".shared::cluster"), |_, _span| Ss::SharedCluster),
                map(string_p(".param::entry"), |_, _span| Ss::ParamEntry),
                map(string_p(".param::func"), |_, _span| Ss::ParamFunc),
                map(string_p(".shared::cta"), |_, _span| Ss::SharedCta),
                map(string_p(".global"), |_, _span| Ss::Global),
                map(string_p(".shared"), |_, _span| Ss::Shared),
                map(string_p(".const"), |_, _span| Ss::Const),
                map(string_p(".local"), |_, _span| Ss::Local),
                map(string_p(".param"), |_, _span| Ss::Param)
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

    impl PtxParser for LdWeakSsCopLevelCacheHintLevelPrefetchSizeVecType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ld"),
                    map(optional(string_p(".weak")), |value, _| value.is_some()),
                    optional(Ss::parse()),
                    optional(Cop::parse()),
                    optional(LevelCacheHint::parse()),
                    optional(LevelPrefetchSize::parse()),
                    optional(Vec::parse()),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(optional(string_p(".unified")), |value, _| value.is_some()),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(
                    _,
                    weak,
                    ss,
                    cop,
                    level_cache_hint,
                    level_prefetch_size,
                    vec,
                    type_,
                    d,
                    _,
                    a,
                    unified,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(LdWeakSsCopLevelCacheHintLevelPrefetchSizeVecType {
                        weak = weak,
                        ss = ss,
                        cop = cop,
                        level_cache_hint = level_cache_hint,
                        level_prefetch_size = level_prefetch_size,
                        vec = vec,
                        type_ = type_,
                        d = d,
                        a = a,
                        unified = unified,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for LdWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ld"),
                    map(optional(string_p(".weak")), |value, _| value.is_some()),
                    optional(Ss::parse()),
                    optional(Level1EvictionPriority::parse()),
                    optional(Level2EvictionPriority::parse()),
                    optional(LevelCacheHint::parse()),
                    optional(LevelPrefetchSize::parse()),
                    optional(Vec::parse()),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(optional(string_p(".unified")), |value, _| value.is_some()),
                    map(optional(seq_n!(comma_p(), GeneralOperand::parse())), |value, _| value.map(|(_, operand)| operand)),
                    semicolon_p()
                ),
                |(_, weak, ss, level1_eviction_priority, level2_eviction_priority, level_cache_hint, level_prefetch_size, vec, type_, d, _, a, unified, cache_policy, _), span| {
                    ok!(LdWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
                        weak = weak,
                        ss = ss,
                        level1_eviction_priority = level1_eviction_priority,
                        level2_eviction_priority = level2_eviction_priority,
                        level_cache_hint = level_cache_hint,
                        level_prefetch_size = level_prefetch_size,
                        vec = vec,
                        type_ = type_,
                        d = d,
                        a = a,
                        unified = unified,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for LdVolatileSsLevelPrefetchSizeVecType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ld"),
                    string_p(".volatile"),
                    optional(Ss::parse()),
                    optional(LevelPrefetchSize::parse()),
                    optional(Vec::parse()),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, volatile, ss, level_prefetch_size, vec, type_, d, _, a, _), span| {
                    ok!(LdVolatileSsLevelPrefetchSizeVecType {
                        volatile = volatile,
                        ss = ss,
                        level_prefetch_size = level_prefetch_size,
                        vec = vec,
                        type_ = type_,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for LdRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ld"),
                    string_p(".relaxed"),
                    Scope::parse(),
                    optional(Ss::parse()),
                    optional(Level1EvictionPriority::parse()),
                    optional(Level2EvictionPriority::parse()),
                    optional(LevelCacheHint::parse()),
                    optional(LevelPrefetchSize::parse()),
                    optional(Vec::parse()),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(optional(seq_n!(comma_p(), GeneralOperand::parse())), |value, _| value.map(|(_, operand)| operand)),
                    semicolon_p()
                ),
                |(_, relaxed, scope, ss, level1_eviction_priority, level2_eviction_priority, level_cache_hint, level_prefetch_size, vec, type_, d, _, a, cache_policy, _), span| {
                    ok!(LdRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
                        relaxed = relaxed,
                        scope = scope,
                        ss = ss,
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

    impl PtxParser for LdAcquireScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ld"),
                    string_p(".acquire"),
                    Scope::parse(),
                    optional(Ss::parse()),
                    optional(Level1EvictionPriority::parse()),
                    optional(Level2EvictionPriority::parse()),
                    optional(LevelCacheHint::parse()),
                    optional(LevelPrefetchSize::parse()),
                    optional(Vec::parse()),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(optional(seq_n!(comma_p(), GeneralOperand::parse())), |value, _| value.map(|(_, operand)| operand)),
                    semicolon_p()
                ),
                |(_, acquire, scope, ss, level1_eviction_priority, level2_eviction_priority, level_cache_hint, level_prefetch_size, vec, type_, d, _, a, cache_policy, _), span| {
                    ok!(LdAcquireScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
                        acquire = acquire,
                        scope = scope,
                        ss = ss,
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

    impl PtxParser for LdMmioRelaxedSysGlobalType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ld"),
                    string_p(".mmio"),
                    string_p(".relaxed"),
                    string_p(".sys"),
                    map(optional(string_p(".global")), |value, _| value.is_some()),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, mmio, relaxed, sys, global, type_, d, _, a, _), span| {
                    ok!(LdMmioRelaxedSysGlobalType {
                        mmio = mmio,
                        relaxed = relaxed,
                        sys = sys,
                        global = global,
                        type_ = type_,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }
}
