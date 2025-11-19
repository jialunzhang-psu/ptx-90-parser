//! Original PTX specification:
//!
//! // Atomic operation with scalar type:
//! atom{.sem}{.scope}{.space}.op{.level::cache_hint}.type d, [a], b{, cache-policy};
//! atom{.sem}{.scope}{.space}.op.type d, [a], b, c;
//! atom{.sem}{.scope}{.space}.cas.b16 d, [a], b, c;
//! atom{.sem}{.scope}{.space}.cas.b128 d, [a], b, c;
//! atom{.sem}{.scope}{.space}.exch{.level::cache_hint}.b128 d, [a], b {, cache-policy};
//! atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16     d, [a], b{, cache-policy};
//! atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16x2   d, [a], b{, cache-policy};
//! atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16    d, [a], b{, cache-policy};
//! atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16x2  d, [a], b{, cache-policy};
//! .space =              { .global, .shared, .shared::cta, .shared::cluster};
//! .sem =                { .relaxed, .acquire, .release, .acq_rel };
//! .scope =              { .cta, .cluster, .gpu, .sys };
//! .op =                 { .and, .or, .xor, .cas, .exch, .add, .inc, .dec, .min, .max };
//! .level::cache_hint =  { .L2::cache_hint };
//! .type =               { .b32, .b64, .u32, .u64, .s32, .s64, .f32, .f64 };
//! -------------------------------------------------------------
//! // Atomic operation with vector type:
//! atom{.sem}{.scope}{.global}.add{.level::cache_hint}.vec_32_bit.f32                  d, [a], b{, cache-policy};
//! atom{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_16_bit.half_word_type  d, [a], b{, cache-policy};
//! atom{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_32_bit.packed_type     d, [a], b{, cache-policy};
//! .sem =               { .relaxed, .acquire, .release, .acq_rel };
//! .scope =             { .cta, .cluster, .gpu, .sys };
//! .op =                { .add, .min, .max };
//! .half_word_type =    { .f16, .bf16 };
//! .packed_type =       { .f16x2, .bf16x2 };
//! .vec_16_bit =        { .v2, .v4, .v8 };
//! .vec_32_bit =        { .v2, .v4 };
//! .level::cache_hint = { .L2::cache_hint };

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
    use crate::r#type::instruction::atom::section_0::*;

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
                map(string_p(".exch"), |_, _span| Op::Exch),
                map(string_p(".and"), |_, _span| Op::And),
                map(string_p(".xor"), |_, _span| Op::Xor),
                map(string_p(".cas"), |_, _span| Op::Cas),
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
                map(string_p(".acquire"), |_, _span| Sem::Acquire),
                map(string_p(".release"), |_, _span| Sem::Release),
                map(string_p(".acq_rel"), |_, _span| Sem::AcqRel)
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

    impl PtxParser for AtomSemScopeSpaceOpLevelCacheHintType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("atom"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    optional(Space::parse()),
                    Op::parse(),
                    optional(LevelCacheHint::parse()),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
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
                    sem,
                    scope,
                    space,
                    op,
                    level_cache_hint,
                    type_,
                    d,
                    _,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(AtomSemScopeSpaceOpLevelCacheHintType {
                        sem = sem,
                        scope = scope,
                        space = space,
                        op = op,
                        level_cache_hint = level_cache_hint,
                        type_ = type_,
                        d = d,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for AtomSemScopeSpaceOpType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("atom"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    optional(Space::parse()),
                    Op::parse(),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, sem, scope, space, op, type_, d, _, a, _, b, _, c, _), span| {
                    ok!(AtomSemScopeSpaceOpType {
                        sem = sem,
                        scope = scope,
                        space = space,
                        op = op,
                        type_ = type_,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for AtomSemScopeSpaceCasB16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("atom"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    optional(Space::parse()),
                    string_p(".cas"),
                    string_p(".b16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, sem, scope, space, cas, b16, d, _, a, _, b, _, c, _), span| {
                    ok!(AtomSemScopeSpaceCasB16 {
                        sem = sem,
                        scope = scope,
                        space = space,
                        cas = cas,
                        b16 = b16,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for AtomSemScopeSpaceCasB128 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("atom"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    optional(Space::parse()),
                    string_p(".cas"),
                    string_p(".b128"),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, sem, scope, space, cas, b128, d, _, a, _, b, _, c, _), span| {
                    ok!(AtomSemScopeSpaceCasB128 {
                        sem = sem,
                        scope = scope,
                        space = space,
                        cas = cas,
                        b128 = b128,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for AtomSemScopeSpaceExchLevelCacheHintB128 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("atom"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    optional(Space::parse()),
                    string_p(".exch"),
                    optional(LevelCacheHint::parse()),
                    string_p(".b128"),
                    GeneralOperand::parse(),
                    comma_p(),
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
                    sem,
                    scope,
                    space,
                    exch,
                    level_cache_hint,
                    b128,
                    d,
                    _,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(AtomSemScopeSpaceExchLevelCacheHintB128 {
                        sem = sem,
                        scope = scope,
                        space = space,
                        exch = exch,
                        level_cache_hint = level_cache_hint,
                        b128 = b128,
                        d = d,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for AtomSemScopeSpaceAddNoftzLevelCacheHintF16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("atom"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    optional(Space::parse()),
                    string_p(".add"),
                    string_p(".noftz"),
                    optional(LevelCacheHint::parse()),
                    string_p(".f16"),
                    GeneralOperand::parse(),
                    comma_p(),
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
                    sem,
                    scope,
                    space,
                    add,
                    noftz,
                    level_cache_hint,
                    f16,
                    d,
                    _,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(AtomSemScopeSpaceAddNoftzLevelCacheHintF16 {
                        sem = sem,
                        scope = scope,
                        space = space,
                        add = add,
                        noftz = noftz,
                        level_cache_hint = level_cache_hint,
                        f16 = f16,
                        d = d,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for AtomSemScopeSpaceAddNoftzLevelCacheHintF16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("atom"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    optional(Space::parse()),
                    string_p(".add"),
                    string_p(".noftz"),
                    optional(LevelCacheHint::parse()),
                    string_p(".f16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
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
                    sem,
                    scope,
                    space,
                    add,
                    noftz,
                    level_cache_hint,
                    f16x2,
                    d,
                    _,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(AtomSemScopeSpaceAddNoftzLevelCacheHintF16x2 {
                        sem = sem,
                        scope = scope,
                        space = space,
                        add = add,
                        noftz = noftz,
                        level_cache_hint = level_cache_hint,
                        f16x2 = f16x2,
                        d = d,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for AtomSemScopeSpaceAddNoftzLevelCacheHintBf16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("atom"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    optional(Space::parse()),
                    string_p(".add"),
                    string_p(".noftz"),
                    optional(LevelCacheHint::parse()),
                    string_p(".bf16"),
                    GeneralOperand::parse(),
                    comma_p(),
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
                    sem,
                    scope,
                    space,
                    add,
                    noftz,
                    level_cache_hint,
                    bf16,
                    d,
                    _,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(AtomSemScopeSpaceAddNoftzLevelCacheHintBf16 {
                        sem = sem,
                        scope = scope,
                        space = space,
                        add = add,
                        noftz = noftz,
                        level_cache_hint = level_cache_hint,
                        bf16 = bf16,
                        d = d,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for AtomSemScopeSpaceAddNoftzLevelCacheHintBf16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("atom"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    optional(Space::parse()),
                    string_p(".add"),
                    string_p(".noftz"),
                    optional(LevelCacheHint::parse()),
                    string_p(".bf16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
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
                    sem,
                    scope,
                    space,
                    add,
                    noftz,
                    level_cache_hint,
                    bf16x2,
                    d,
                    _,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(AtomSemScopeSpaceAddNoftzLevelCacheHintBf16x2 {
                        sem = sem,
                        scope = scope,
                        space = space,
                        add = add,
                        noftz = noftz,
                        level_cache_hint = level_cache_hint,
                        bf16x2 = bf16x2,
                        d = d,
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
    use crate::r#type::instruction::atom::section_1::*;

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
                map(string_p(".acquire"), |_, _span| Sem::Acquire),
                map(string_p(".release"), |_, _span| Sem::Release),
                map(string_p(".acq_rel"), |_, _span| Sem::AcqRel)
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

    impl PtxParser for AtomSemScopeGlobalAddLevelCacheHintVec32BitF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("atom"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    map(optional(string_p(".global")), |value, _| value.is_some()),
                    string_p(".add"),
                    optional(LevelCacheHint::parse()),
                    Vec32Bit::parse(),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
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
                    sem,
                    scope,
                    global,
                    add,
                    level_cache_hint,
                    vec_32_bit,
                    f32,
                    d,
                    _,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(AtomSemScopeGlobalAddLevelCacheHintVec32BitF32 {
                        sem = sem,
                        scope = scope,
                        global = global,
                        add = add,
                        level_cache_hint = level_cache_hint,
                        vec_32_bit = vec_32_bit,
                        f32 = f32,
                        d = d,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for AtomSemScopeGlobalOpNoftzLevelCacheHintVec16BitHalfWordType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("atom"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    map(optional(string_p(".global")), |value, _| value.is_some()),
                    Op::parse(),
                    string_p(".noftz"),
                    optional(LevelCacheHint::parse()),
                    Vec16Bit::parse(),
                    HalfWordType::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
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
                    sem,
                    scope,
                    global,
                    op,
                    noftz,
                    level_cache_hint,
                    vec_16_bit,
                    half_word_type,
                    d,
                    _,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(AtomSemScopeGlobalOpNoftzLevelCacheHintVec16BitHalfWordType {
                        sem = sem,
                        scope = scope,
                        global = global,
                        op = op,
                        noftz = noftz,
                        level_cache_hint = level_cache_hint,
                        vec_16_bit = vec_16_bit,
                        half_word_type = half_word_type,
                        d = d,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }

    impl PtxParser for AtomSemScopeGlobalOpNoftzLevelCacheHintVec32BitPackedType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("atom"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    map(optional(string_p(".global")), |value, _| value.is_some()),
                    Op::parse(),
                    string_p(".noftz"),
                    optional(LevelCacheHint::parse()),
                    Vec32Bit::parse(),
                    PackedType::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
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
                    sem,
                    scope,
                    global,
                    op,
                    noftz,
                    level_cache_hint,
                    vec_32_bit,
                    packed_type,
                    d,
                    _,
                    a,
                    _,
                    b,
                    cache_policy,
                    _,
                ),
                 span| {
                    ok!(AtomSemScopeGlobalOpNoftzLevelCacheHintVec32BitPackedType {
                        sem = sem,
                        scope = scope,
                        global = global,
                        op = op,
                        noftz = noftz,
                        level_cache_hint = level_cache_hint,
                        vec_32_bit = vec_32_bit,
                        packed_type = packed_type,
                        d = d,
                        a = a,
                        b = b,
                        cache_policy = cache_policy,

                    })
                },
            )
        }
    }
}
