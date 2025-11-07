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
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Sem {
        Relaxed, // .relaxed
        Acquire, // .acquire
        Release, // .release
        AcqRel, // .acq_rel
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scope {
        Cluster, // .cluster
        Cta, // .cta
        Gpu, // .gpu
        Sys, // .sys
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Space {
        SharedCluster, // .shared::cluster
        SharedCta, // .shared::cta
        Global, // .global
        Shared, // .shared
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        Exch, // .exch
        And, // .and
        Xor, // .xor
        Cas, // .cas
        Add, // .add
        Inc, // .inc
        Dec, // .dec
        Min, // .min
        Max, // .max
        Or, // .or
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LevelCacheHint {
        L2CacheHint, // .L2::cache_hint
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B32, // .b32
        B64, // .b64
        U32, // .u32
        U64, // .u64
        S32, // .s32
        S64, // .s64
        F32, // .f32
        F64, // .f64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AtomSemScopeSpaceOpLevelCacheHintType {
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub space: Option<Space>, // {.space}
        pub op: Op, // .op
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AtomSemScopeSpaceOpType {
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub space: Option<Space>, // {.space}
        pub op: Op, // .op
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AtomSemScopeSpaceCasB16 {
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub space: Option<Space>, // {.space}
        pub cas: (), // .cas
        pub b16: (), // .b16
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AtomSemScopeSpaceCasB128 {
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub space: Option<Space>, // {.space}
        pub cas: (), // .cas
        pub b128: (), // .b128
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AtomSemScopeSpaceExchLevelCacheHintB128 {
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub space: Option<Space>, // {.space}
        pub exch: (), // .exch
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub b128: (), // .b128
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AtomSemScopeSpaceAddNoftzLevelCacheHintF16 {
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub space: Option<Space>, // {.space}
        pub add: (), // .add
        pub noftz: (), // .noftz
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub f16: (), // .f16
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AtomSemScopeSpaceAddNoftzLevelCacheHintF16x2 {
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub space: Option<Space>, // {.space}
        pub add: (), // .add
        pub noftz: (), // .noftz
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub f16x2: (), // .f16x2
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AtomSemScopeSpaceAddNoftzLevelCacheHintBf16 {
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub space: Option<Space>, // {.space}
        pub add: (), // .add
        pub noftz: (), // .noftz
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub bf16: (), // .bf16
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AtomSemScopeSpaceAddNoftzLevelCacheHintBf16x2 {
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub space: Option<Space>, // {.space}
        pub add: (), // .add
        pub noftz: (), // .noftz
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub bf16x2: (), // .bf16x2
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Sem {
        Relaxed, // .relaxed
        Acquire, // .acquire
        Release, // .release
        AcqRel, // .acq_rel
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scope {
        Cluster, // .cluster
        Cta, // .cta
        Gpu, // .gpu
        Sys, // .sys
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LevelCacheHint {
        L2CacheHint, // .L2::cache_hint
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Vec32Bit {
        V2, // .v2
        V4, // .v4
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        Add, // .add
        Min, // .min
        Max, // .max
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Vec16Bit {
        V2, // .v2
        V4, // .v4
        V8, // .v8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum HalfWordType {
        Bf16, // .bf16
        F16, // .f16
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum PackedType {
        Bf16x2, // .bf16x2
        F16x2, // .f16x2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AtomSemScopeGlobalAddLevelCacheHintVec32BitF32 {
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub global: bool, // {.global}
        pub add: (), // .add
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub vec_32_bit: Vec32Bit, // .vec_32_bit
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AtomSemScopeGlobalOpNoftzLevelCacheHintVec16BitHalfWordType {
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub global: bool, // {.global}
        pub op: Op, // .op
        pub noftz: (), // .noftz
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub vec_16_bit: Vec16Bit, // .vec_16_bit
        pub half_word_type: HalfWordType, // .half_word_type
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AtomSemScopeGlobalOpNoftzLevelCacheHintVec32BitPackedType {
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub global: bool, // {.global}
        pub op: Op, // .op
        pub noftz: (), // .noftz
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub vec_32_bit: Vec32Bit, // .vec_32_bit
        pub packed_type: PackedType, // .packed_type
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

}
