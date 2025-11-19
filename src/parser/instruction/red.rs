//! Original PTX specification:
//!
//! // Reduction operation with scalar type:
//! red.op{.space}{.sem}{.scope}{.level::cache_hint}.type          [a], b{, cache-policy};
//! red.add{.space}{.sem}{.scope}.noftz{.level::cache_hint}.f16    [a], b{, cache-policy};
//! red.add{.space}{.sem}{.scope}.noftz{.level::cache_hint}.f16x2  [a], b{, cache-policy};
//! red.add{.space}{.sem}{.scope}.noftz{.level::cache_hint}.bf16   [a], b{, cache-policy};
//! red.add{.space}{.sem}{.scope}.noftz{.level::cache_hint}.bf16x2 [a], b{, cache-policy};
//! .space =              { .global, .shared, .shared::cta, .shared::cluster};
//! .sem =                {.relaxed, .release};
//! .scope =              {.cta, .cluster, .gpu, .sys};
//! .op =                 { .and, .or, .xor, .add, .inc, .dec, .min, .max };
//! .level::cache_hint =  { .L2::cache_hint };
//! .type =               { .b32, .b64, .u32, .u64, .s32, .s64, .f32, .f64 };
//! ------------------------------------------------------------------
//! // Reduction operation with vector type:
//! red.add{.space}{.sem}{.scope}{.level::cache_hint}.vec_32_bit.f32 [a], b{, cache-policy};
//! red.op{.space}{.sem}{.scope}.noftz{.level::cache_hint}. vec_16_bit.half_word_type [a], b{, cache-policy};
//! red.op{.space}{.sem}{.scope}.noftz{.level::cache_hint}.vec_32_bit.packed_type [a], b {, cache-policy};
//! .sem =                { .relaxed, .release };
//! .scope =              { .cta, .cluster, .gpu, .sys };
//! .op =                 { .add, .min, .max };
//! .half_word_type =     { .f16, .bf16 };
//! .packed_type =        { .f16x2,.bf16x2 };
//! .vec_16_bit =         { .v2, .v4, .v8 };
//! .vec_32_bit =         { .v2, .v4 };
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
    use crate::r#type::instruction::red::section_0::*;

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

    impl PtxParser for Op {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".and"), |_, _span| Op::And),
                map(string_p(".xor"), |_, _span| Op::Xor),
                map(string_p(".add"), |_, _span| Op::Add),
                map(string_p(".inc"), |_, _span| Op::Inc),
                map(string_p(".dec"), |_, _span| Op::Dec),
                map(string_p(".min"), |_, _span| Op::Min),
                map(string_p(".max"), |_, _span| Op::Max),
                map(string_p(".or"), |_, _span| Op::Or)
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

    impl PtxParser for Sem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".relaxed"), |_, _span| Sem::Relaxed),
                map(string_p(".release"), |_, _span| Sem::Release)
            )
        }
    }

    impl PtxParser for Space {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".shared::cluster"), |_, _span| {
                    Space::SharedCluster
                }),
                map(string_p(".shared::cta"), |_, _span| Space::SharedCta),
                map(string_p(".global"), |_, _span| Space::Global),
                map(string_p(".shared"), |_, _span| Space::Shared)
            )
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b32"), |_, _span| Type::B32),
                map(string_p(".b64"), |_, _span| Type::B64),
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".u64"), |_, _span| Type::U64),
                map(string_p(".s32"), |_, _span| Type::S32),
                map(string_p(".s64"), |_, _span| Type::S64),
                map(string_p(".f32"), |_, _span| Type::F32),
                map(string_p(".f64"), |_, _span| Type::F64)
            )
        }
    }

    impl PtxParser for RedOpSpaceSemScopeLevelCacheHintType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("red"),
                    Op::parse(),
                    optional(Space::parse()),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    optional(LevelCacheHint::parse()),
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
                |(_, op, space, sem, scope, level_cache_hint, type_, a, _, b, cache_policy, _),
                 span| {
                    ok!(RedOpSpaceSemScopeLevelCacheHintType {
                        op = op,
                        space = space,
                        sem = sem,
                        scope = scope,
                        level_cache_hint = level_cache_hint,
                        type_ = type_,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for RedAddSpaceSemScopeNoftzLevelCacheHintF16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("red"),
                    string_p(".add"),
                    optional(Space::parse()),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    string_p(".noftz"),
                    optional(LevelCacheHint::parse()),
                    string_p(".f16"),
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
                    add,
                    space,
                    sem,
                    scope,
                    noftz,
                    level_cache_hint,
                    f16,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(RedAddSpaceSemScopeNoftzLevelCacheHintF16 {
                        add = add,
                        space = space,
                        sem = sem,
                        scope = scope,
                        noftz = noftz,
                        level_cache_hint = level_cache_hint,
                        f16 = f16,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for RedAddSpaceSemScopeNoftzLevelCacheHintF16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("red"),
                    string_p(".add"),
                    optional(Space::parse()),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    string_p(".noftz"),
                    optional(LevelCacheHint::parse()),
                    string_p(".f16x2"),
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
                    add,
                    space,
                    sem,
                    scope,
                    noftz,
                    level_cache_hint,
                    f16x2,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(RedAddSpaceSemScopeNoftzLevelCacheHintF16x2 {
                        add = add,
                        space = space,
                        sem = sem,
                        scope = scope,
                        noftz = noftz,
                        level_cache_hint = level_cache_hint,
                        f16x2 = f16x2,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for RedAddSpaceSemScopeNoftzLevelCacheHintBf16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("red"),
                    string_p(".add"),
                    optional(Space::parse()),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    string_p(".noftz"),
                    optional(LevelCacheHint::parse()),
                    string_p(".bf16"),
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
                    add,
                    space,
                    sem,
                    scope,
                    noftz,
                    level_cache_hint,
                    bf16,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(RedAddSpaceSemScopeNoftzLevelCacheHintBf16 {
                        add = add,
                        space = space,
                        sem = sem,
                        scope = scope,
                        noftz = noftz,
                        level_cache_hint = level_cache_hint,
                        bf16 = bf16,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for RedAddSpaceSemScopeNoftzLevelCacheHintBf16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("red"),
                    string_p(".add"),
                    optional(Space::parse()),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    string_p(".noftz"),
                    optional(LevelCacheHint::parse()),
                    string_p(".bf16x2"),
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
                    add,
                    space,
                    sem,
                    scope,
                    noftz,
                    level_cache_hint,
                    bf16x2,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(RedAddSpaceSemScopeNoftzLevelCacheHintBf16x2 {
                        add = add,
                        space = space,
                        sem = sem,
                        scope = scope,
                        noftz = noftz,
                        level_cache_hint = level_cache_hint,
                        bf16x2 = bf16x2,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::red::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for HalfWordType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".bf16"), |_, _span| HalfWordType::Bf16),
                map(string_p(".f16"), |_, _span| HalfWordType::F16)
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

    impl PtxParser for Op {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".add"), |_, _span| Op::Add),
                map(string_p(".min"), |_, _span| Op::Min),
                map(string_p(".max"), |_, _span| Op::Max)
            )
        }
    }

    impl PtxParser for PackedType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".bf16x2"), |_, _span| PackedType::Bf16x2),
                map(string_p(".f16x2"), |_, _span| PackedType::F16x2)
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

    impl PtxParser for Sem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".relaxed"), |_, _span| Sem::Relaxed),
                map(string_p(".release"), |_, _span| Sem::Release)
            )
        }
    }

    impl PtxParser for Space {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".shared::cluster"), |_, _span| {
                    Space::SharedCluster
                }),
                map(string_p(".shared::cta"), |_, _span| Space::SharedCta),
                map(string_p(".global"), |_, _span| Space::Global),
                map(string_p(".shared"), |_, _span| Space::Shared)
            )
        }
    }

    impl PtxParser for Vec16Bit {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".v2"), |_, _span| Vec16Bit::V2),
                map(string_p(".v4"), |_, _span| Vec16Bit::V4),
                map(string_p(".v8"), |_, _span| Vec16Bit::V8)
            )
        }
    }

    impl PtxParser for Vec32Bit {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".v2"), |_, _span| Vec32Bit::V2),
                map(string_p(".v4"), |_, _span| Vec32Bit::V4)
            )
        }
    }

    impl PtxParser for RedAddSpaceSemScopeLevelCacheHintVec32BitF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("red"),
                    string_p(".add"),
                    optional(Space::parse()),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    optional(LevelCacheHint::parse()),
                    Vec32Bit::parse(),
                    string_p(".f32"),
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
                    add,
                    space,
                    sem,
                    scope,
                    level_cache_hint,
                    vec_32_bit,
                    f32,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(RedAddSpaceSemScopeLevelCacheHintVec32BitF32 {
                        add = add,
                        space = space,
                        sem = sem,
                        scope = scope,
                        level_cache_hint = level_cache_hint,
                        vec_32_bit = vec_32_bit,
                        f32 = f32,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for RedOpSpaceSemScopeNoftzLevelCacheHintVec16BitHalfWordType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("red"),
                    Op::parse(),
                    optional(Space::parse()),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    string_p(".noftz"),
                    optional(LevelCacheHint::parse()),
                    Vec16Bit::parse(),
                    HalfWordType::parse(),
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
                    op,
                    space,
                    sem,
                    scope,
                    noftz,
                    level_cache_hint,
                    vec_16_bit,
                    half_word_type,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(RedOpSpaceSemScopeNoftzLevelCacheHintVec16BitHalfWordType {
                        op = op,
                        space = space,
                        sem = sem,
                        scope = scope,
                        noftz = noftz,
                        level_cache_hint = level_cache_hint,
                        vec_16_bit = vec_16_bit,
                        half_word_type = half_word_type,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for RedOpSpaceSemScopeNoftzLevelCacheHintVec32BitPackedType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("red"),
                    Op::parse(),
                    optional(Space::parse()),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    string_p(".noftz"),
                    optional(LevelCacheHint::parse()),
                    Vec32Bit::parse(),
                    PackedType::parse(),
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
                    op,
                    space,
                    sem,
                    scope,
                    noftz,
                    level_cache_hint,
                    vec_32_bit,
                    packed_type,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(RedOpSpaceSemScopeNoftzLevelCacheHintVec32BitPackedType {
                        op = op,
                        space = space,
                        sem = sem,
                        scope = scope,
                        noftz = noftz,
                        level_cache_hint = level_cache_hint,
                        vec_32_bit = vec_32_bit,
                        packed_type = packed_type,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }
}
