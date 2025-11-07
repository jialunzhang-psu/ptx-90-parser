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

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::red::section_0::*;

    impl PtxUnparser for RedOpSpaceSemScopeLevelCacheHintType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "red");
                    match &self.op {
                            Op::And => {
                                    push_directive(tokens, "and");
                            }
                            Op::Xor => {
                                    push_directive(tokens, "xor");
                            }
                            Op::Add => {
                                    push_directive(tokens, "add");
                            }
                            Op::Inc => {
                                    push_directive(tokens, "inc");
                            }
                            Op::Dec => {
                                    push_directive(tokens, "dec");
                            }
                            Op::Min => {
                                    push_directive(tokens, "min");
                            }
                            Op::Max => {
                                    push_directive(tokens, "max");
                            }
                            Op::Or => {
                                    push_directive(tokens, "or");
                            }
                    }
                    if let Some(space_0) = self.space.as_ref() {
                            match space_0 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Space::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Space::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Space::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    if let Some(sem_1) = self.sem.as_ref() {
                            match sem_1 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                            }
                    }
                    if let Some(scope_2) = self.scope.as_ref() {
                            match scope_2 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    if let Some(level_cache_hint_3) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_3 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    match &self.type_ {
                            Type::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Type::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Type::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Type::S64 => {
                                    push_directive(tokens, "s64");
                            }
                            Type::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Type::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_4) = self.cache_policy.as_ref() {
                        opt_4.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for RedAddSpaceSemScopeNoftzLevelCacheHintF16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "red");
                    push_directive(tokens, "add");
                    if let Some(space_5) = self.space.as_ref() {
                            match space_5 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Space::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Space::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Space::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    if let Some(sem_6) = self.sem.as_ref() {
                            match sem_6 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                            }
                    }
                    if let Some(scope_7) = self.scope.as_ref() {
                            match scope_7 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    push_directive(tokens, "noftz");
                    if let Some(level_cache_hint_8) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_8 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    push_directive(tokens, "f16");
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_9) = self.cache_policy.as_ref() {
                        opt_9.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for RedAddSpaceSemScopeNoftzLevelCacheHintF16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "red");
                    push_directive(tokens, "add");
                    if let Some(space_10) = self.space.as_ref() {
                            match space_10 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Space::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Space::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Space::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    if let Some(sem_11) = self.sem.as_ref() {
                            match sem_11 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                            }
                    }
                    if let Some(scope_12) = self.scope.as_ref() {
                            match scope_12 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    push_directive(tokens, "noftz");
                    if let Some(level_cache_hint_13) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_13 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    push_directive(tokens, "f16x2");
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_14) = self.cache_policy.as_ref() {
                        opt_14.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for RedAddSpaceSemScopeNoftzLevelCacheHintBf16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "red");
                    push_directive(tokens, "add");
                    if let Some(space_15) = self.space.as_ref() {
                            match space_15 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Space::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Space::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Space::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    if let Some(sem_16) = self.sem.as_ref() {
                            match sem_16 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                            }
                    }
                    if let Some(scope_17) = self.scope.as_ref() {
                            match scope_17 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    push_directive(tokens, "noftz");
                    if let Some(level_cache_hint_18) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_18 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    push_directive(tokens, "bf16");
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_19) = self.cache_policy.as_ref() {
                        opt_19.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for RedAddSpaceSemScopeNoftzLevelCacheHintBf16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "red");
                    push_directive(tokens, "add");
                    if let Some(space_20) = self.space.as_ref() {
                            match space_20 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Space::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Space::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Space::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    if let Some(sem_21) = self.sem.as_ref() {
                            match sem_21 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                            }
                    }
                    if let Some(scope_22) = self.scope.as_ref() {
                            match scope_22 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    push_directive(tokens, "noftz");
                    if let Some(level_cache_hint_23) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_23 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    push_directive(tokens, "bf16x2");
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_24) = self.cache_policy.as_ref() {
                        opt_24.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::red::section_1::*;

    impl PtxUnparser for RedAddSpaceSemScopeLevelCacheHintVec32BitF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "red");
                    push_directive(tokens, "add");
                    if let Some(space_25) = self.space.as_ref() {
                            match space_25 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Space::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Space::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Space::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    if let Some(sem_26) = self.sem.as_ref() {
                            match sem_26 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                            }
                    }
                    if let Some(scope_27) = self.scope.as_ref() {
                            match scope_27 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    if let Some(level_cache_hint_28) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_28 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    match &self.vec_32_bit {
                            Vec32Bit::V2 => {
                                    push_directive(tokens, "v2");
                            }
                            Vec32Bit::V4 => {
                                    push_directive(tokens, "v4");
                            }
                    }
                    push_directive(tokens, "f32");
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_29) = self.cache_policy.as_ref() {
                        opt_29.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for RedOpSpaceSemScopeNoftzLevelCacheHintVec16BitHalfWordType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "red");
                    match &self.op {
                            Op::Add => {
                                    push_directive(tokens, "add");
                            }
                            Op::Min => {
                                    push_directive(tokens, "min");
                            }
                            Op::Max => {
                                    push_directive(tokens, "max");
                            }
                    }
                    if let Some(space_30) = self.space.as_ref() {
                            match space_30 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Space::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Space::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Space::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    if let Some(sem_31) = self.sem.as_ref() {
                            match sem_31 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                            }
                    }
                    if let Some(scope_32) = self.scope.as_ref() {
                            match scope_32 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    push_directive(tokens, "noftz");
                    if let Some(level_cache_hint_33) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_33 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    match &self.vec_16_bit {
                            Vec16Bit::V2 => {
                                    push_directive(tokens, "v2");
                            }
                            Vec16Bit::V4 => {
                                    push_directive(tokens, "v4");
                            }
                            Vec16Bit::V8 => {
                                    push_directive(tokens, "v8");
                            }
                    }
                    match &self.half_word_type {
                            HalfWordType::Bf16 => {
                                    push_directive(tokens, "bf16");
                            }
                            HalfWordType::F16 => {
                                    push_directive(tokens, "f16");
                            }
                    }
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_34) = self.cache_policy.as_ref() {
                        opt_34.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for RedOpSpaceSemScopeNoftzLevelCacheHintVec32BitPackedType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "red");
                    match &self.op {
                            Op::Add => {
                                    push_directive(tokens, "add");
                            }
                            Op::Min => {
                                    push_directive(tokens, "min");
                            }
                            Op::Max => {
                                    push_directive(tokens, "max");
                            }
                    }
                    if let Some(space_35) = self.space.as_ref() {
                            match space_35 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Space::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Space::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Space::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    if let Some(sem_36) = self.sem.as_ref() {
                            match sem_36 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                            }
                    }
                    if let Some(scope_37) = self.scope.as_ref() {
                            match scope_37 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    push_directive(tokens, "noftz");
                    if let Some(level_cache_hint_38) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_38 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    match &self.vec_32_bit {
                            Vec32Bit::V2 => {
                                    push_directive(tokens, "v2");
                            }
                            Vec32Bit::V4 => {
                                    push_directive(tokens, "v4");
                            }
                    }
                    match &self.packed_type {
                            PackedType::Bf16x2 => {
                                    push_directive(tokens, "bf16x2");
                            }
                            PackedType::F16x2 => {
                                    push_directive(tokens, "f16x2");
                            }
                    }
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_39) = self.cache_policy.as_ref() {
                        opt_39.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

