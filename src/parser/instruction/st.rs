//! Original PTX specification:
//!
//! st{.weak}{.ss}{.cop}{.level::cache_hint}{.vec}.type   [a], b{, cache-policy};
//! st{.weak}{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache-policy};
//! st.volatile{.ss}{.vec}.type                           [a], b;
//! st.relaxed.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache-policy};
//! st.release.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache-policy};
//! st.mmio.relaxed.sys{.global}.type         [a], b;
//! .ss =                       { .global, .local, .param, .param::func, .shared, .shared::cta, .shared::cluster};
//! .level1::eviction_priority = { .L1::evict_normal, .L1::evict_unchanged,
//! .L1::evict_first, .L1::evict_last, .L1::no_allocate };
//! .level2::eviction_priority = { .L2::evict_normal, .L2::evict_first, .L2::evict_last };
//! .level::cache_hint =        { .L2::cache_hint };
//! .cop =                      { .wb, .cg, .cs, .wt };
//! .sem =                      { .relaxed, .release };
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
    use crate::r#type::instruction::st::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Cop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".wb"), |_, _span| Cop::Wb),
                map(string_p(".cg"), |_, _span| Cop::Cg),
                map(string_p(".cs"), |_, _span| Cop::Cs),
                map(string_p(".wt"), |_, _span| Cop::Wt)
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
                map(string_p(".param::func"), |_, _span| Ss::ParamFunc),
                map(string_p(".shared::cta"), |_, _span| Ss::SharedCta),
                map(string_p(".global"), |_, _span| Ss::Global),
                map(string_p(".shared"), |_, _span| Ss::Shared),
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

    impl PtxParser for StWeakSsCopLevelCacheHintVecType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("st"),
                    map(optional(string_p(".weak")), |value, _| value.is_some()),
                    optional(Ss::parse()),
                    optional(Cop::parse()),
                    optional(LevelCacheHint::parse()),
                    optional(Vec::parse()),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, weak, ss, cop, level_cache_hint, vec, type_, a, _, b, cache_policy, _),
                 span| {
                    ok!(StWeakSsCopLevelCacheHintVecType {
                        weak = weak,
                        ss = ss,
                        cop = cop,
                        level_cache_hint = level_cache_hint,
                        vec = vec,
                        type_ = type_,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for StWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("st"),
                    map(optional(string_p(".weak")), |value, _| value.is_some()),
                    optional(Ss::parse()),
                    optional(Level1EvictionPriority::parse()),
                    optional(Level2EvictionPriority::parse()),
                    optional(LevelCacheHint::parse()),
                    optional(Vec::parse()),
                    Type::parse(),
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
                    weak,
                    ss,
                    level1_eviction_priority,
                    level2_eviction_priority,
                    level_cache_hint,
                    vec,
                    type_,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(StWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType {
                        weak = weak,
                        ss = ss,
                        level1_eviction_priority = level1_eviction_priority,
                        level2_eviction_priority = level2_eviction_priority,
                        level_cache_hint = level_cache_hint,
                        vec = vec,
                        type_ = type_,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for StVolatileSsVecType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("st"),
                    string_p(".volatile"),
                    optional(Ss::parse()),
                    optional(Vec::parse()),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, volatile, ss, vec, type_, a, _, b, _), span| {
                    ok!(StVolatileSsVecType {
                        volatile = volatile,
                        ss = ss,
                        vec = vec,
                        type_ = type_,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser
        for StRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType
    {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("st"),
                    string_p(".relaxed"),
                    Scope::parse(),
                    optional(Ss::parse()),
                    optional(Level1EvictionPriority::parse()),
                    optional(Level2EvictionPriority::parse()),
                    optional(LevelCacheHint::parse()),
                    optional(Vec::parse()),
                    Type::parse(),
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
                    relaxed,
                    scope,
                    ss,
                    level1_eviction_priority,
                    level2_eviction_priority,
                    level_cache_hint,
                    vec,
                    type_,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(StRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType {
                        relaxed = relaxed,
                        scope = scope,
                        ss = ss,
                        level1_eviction_priority = level1_eviction_priority,
                        level2_eviction_priority = level2_eviction_priority,
                        level_cache_hint = level_cache_hint,
                        vec = vec,
                        type_ = type_,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser
        for StReleaseScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType
    {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("st"),
                    string_p(".release"),
                    Scope::parse(),
                    optional(Ss::parse()),
                    optional(Level1EvictionPriority::parse()),
                    optional(Level2EvictionPriority::parse()),
                    optional(LevelCacheHint::parse()),
                    optional(Vec::parse()),
                    Type::parse(),
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
                    release,
                    scope,
                    ss,
                    level1_eviction_priority,
                    level2_eviction_priority,
                    level_cache_hint,
                    vec,
                    type_,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(StReleaseScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType {
                        release = release,
                        scope = scope,
                        ss = ss,
                        level1_eviction_priority = level1_eviction_priority,
                        level2_eviction_priority = level2_eviction_priority,
                        level_cache_hint = level_cache_hint,
                        vec = vec,
                        type_ = type_,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for StMmioRelaxedSysGlobalType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("st"),
                    string_p(".mmio"),
                    string_p(".relaxed"),
                    string_p(".sys"),
                    map(optional(string_p(".global")), |value, _| value.is_some()),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, mmio, relaxed, sys, global, type_, a, _, b, _), span| {
                    ok!(StMmioRelaxedSysGlobalType {
                        mmio = mmio,
                        relaxed = relaxed,
                        sys = sys,
                        global = global,
                        type_ = type_,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }
}
