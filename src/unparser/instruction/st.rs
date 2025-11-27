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

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::st::section_0::*;

    impl PtxUnparser for StWeakSsCopLevelCacheHintVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "st");
                    if self.weak {
                            push_directive(tokens, "weak");
                    }
                    if let Some(ss_0) = self.ss.as_ref() {
                            match ss_0 {
                                    Ss::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Ss::ParamFunc => {
                                            push_directive(tokens, "param::func");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::Local => {
                                            push_directive(tokens, "local");
                                    }
                                    Ss::Param => {
                                            push_directive(tokens, "param");
                                    }
                            }
                    }
                    if let Some(cop_1) = self.cop.as_ref() {
                            match cop_1 {
                                    Cop::Wb => {
                                            push_directive(tokens, "wb");
                                    }
                                    Cop::Cg => {
                                            push_directive(tokens, "cg");
                                    }
                                    Cop::Cs => {
                                            push_directive(tokens, "cs");
                                    }
                                    Cop::Wt => {
                                            push_directive(tokens, "wt");
                                    }
                            }
                    }
                    if let Some(level_cache_hint_2) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_2 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    if let Some(vec_3) = self.vec.as_ref() {
                            match vec_3 {
                                    Vec::V2 => {
                                            push_directive(tokens, "v2");
                                    }
                                    Vec::V4 => {
                                            push_directive(tokens, "v4");
                                    }
                                    Vec::V8 => {
                                            push_directive(tokens, "v8");
                                    }
                            }
                    }
                    match &self.type_ {
                            Type::B128 => {
                                    push_directive(tokens, "b128");
                            }
                            Type::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Type::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Type::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Type::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Type::S16 => {
                                    push_directive(tokens, "s16");
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
                            Type::B8 => {
                                    push_directive(tokens, "b8");
                            }
                            Type::U8 => {
                                    push_directive(tokens, "u8");
                            }
                            Type::S8 => {
                                    push_directive(tokens, "s8");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_4) = self.cache_policy.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_4.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for StWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "st");
                    if self.weak {
                            push_directive(tokens, "weak");
                    }
                    if let Some(ss_5) = self.ss.as_ref() {
                            match ss_5 {
                                    Ss::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Ss::ParamFunc => {
                                            push_directive(tokens, "param::func");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::Local => {
                                            push_directive(tokens, "local");
                                    }
                                    Ss::Param => {
                                            push_directive(tokens, "param");
                                    }
                            }
                    }
                    if let Some(level1_eviction_priority_6) = self.level1_eviction_priority.as_ref() {
                            match level1_eviction_priority_6 {
                                    Level1EvictionPriority::L1EvictUnchanged => {
                                            push_directive(tokens, "L1::evict_unchanged");
                                    }
                                    Level1EvictionPriority::L1EvictNormal => {
                                            push_directive(tokens, "L1::evict_normal");
                                    }
                                    Level1EvictionPriority::L1EvictFirst => {
                                            push_directive(tokens, "L1::evict_first");
                                    }
                                    Level1EvictionPriority::L1NoAllocate => {
                                            push_directive(tokens, "L1::no_allocate");
                                    }
                                    Level1EvictionPriority::L1EvictLast => {
                                            push_directive(tokens, "L1::evict_last");
                                    }
                            }
                    }
                    if let Some(level2_eviction_priority_7) = self.level2_eviction_priority.as_ref() {
                            match level2_eviction_priority_7 {
                                    Level2EvictionPriority::L2EvictNormal => {
                                            push_directive(tokens, "L2::evict_normal");
                                    }
                                    Level2EvictionPriority::L2EvictFirst => {
                                            push_directive(tokens, "L2::evict_first");
                                    }
                                    Level2EvictionPriority::L2EvictLast => {
                                            push_directive(tokens, "L2::evict_last");
                                    }
                            }
                    }
                    if let Some(level_cache_hint_8) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_8 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    if let Some(vec_9) = self.vec.as_ref() {
                            match vec_9 {
                                    Vec::V2 => {
                                            push_directive(tokens, "v2");
                                    }
                                    Vec::V4 => {
                                            push_directive(tokens, "v4");
                                    }
                                    Vec::V8 => {
                                            push_directive(tokens, "v8");
                                    }
                            }
                    }
                    match &self.type_ {
                            Type::B128 => {
                                    push_directive(tokens, "b128");
                            }
                            Type::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Type::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Type::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Type::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Type::S16 => {
                                    push_directive(tokens, "s16");
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
                            Type::B8 => {
                                    push_directive(tokens, "b8");
                            }
                            Type::U8 => {
                                    push_directive(tokens, "u8");
                            }
                            Type::S8 => {
                                    push_directive(tokens, "s8");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_10) = self.cache_policy.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_10.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for StVolatileSsVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "st");
                    push_directive(tokens, "volatile");
                    if let Some(ss_11) = self.ss.as_ref() {
                            match ss_11 {
                                    Ss::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Ss::ParamFunc => {
                                            push_directive(tokens, "param::func");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::Local => {
                                            push_directive(tokens, "local");
                                    }
                                    Ss::Param => {
                                            push_directive(tokens, "param");
                                    }
                            }
                    }
                    if let Some(vec_12) = self.vec.as_ref() {
                            match vec_12 {
                                    Vec::V2 => {
                                            push_directive(tokens, "v2");
                                    }
                                    Vec::V4 => {
                                            push_directive(tokens, "v4");
                                    }
                                    Vec::V8 => {
                                            push_directive(tokens, "v8");
                                    }
                            }
                    }
                    match &self.type_ {
                            Type::B128 => {
                                    push_directive(tokens, "b128");
                            }
                            Type::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Type::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Type::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Type::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Type::S16 => {
                                    push_directive(tokens, "s16");
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
                            Type::B8 => {
                                    push_directive(tokens, "b8");
                            }
                            Type::U8 => {
                                    push_directive(tokens, "u8");
                            }
                            Type::S8 => {
                                    push_directive(tokens, "s8");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for StRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "st");
                    push_directive(tokens, "relaxed");
                    match &self.scope {
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
                    if let Some(ss_13) = self.ss.as_ref() {
                            match ss_13 {
                                    Ss::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Ss::ParamFunc => {
                                            push_directive(tokens, "param::func");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::Local => {
                                            push_directive(tokens, "local");
                                    }
                                    Ss::Param => {
                                            push_directive(tokens, "param");
                                    }
                            }
                    }
                    if let Some(level1_eviction_priority_14) = self.level1_eviction_priority.as_ref() {
                            match level1_eviction_priority_14 {
                                    Level1EvictionPriority::L1EvictUnchanged => {
                                            push_directive(tokens, "L1::evict_unchanged");
                                    }
                                    Level1EvictionPriority::L1EvictNormal => {
                                            push_directive(tokens, "L1::evict_normal");
                                    }
                                    Level1EvictionPriority::L1EvictFirst => {
                                            push_directive(tokens, "L1::evict_first");
                                    }
                                    Level1EvictionPriority::L1NoAllocate => {
                                            push_directive(tokens, "L1::no_allocate");
                                    }
                                    Level1EvictionPriority::L1EvictLast => {
                                            push_directive(tokens, "L1::evict_last");
                                    }
                            }
                    }
                    if let Some(level2_eviction_priority_15) = self.level2_eviction_priority.as_ref() {
                            match level2_eviction_priority_15 {
                                    Level2EvictionPriority::L2EvictNormal => {
                                            push_directive(tokens, "L2::evict_normal");
                                    }
                                    Level2EvictionPriority::L2EvictFirst => {
                                            push_directive(tokens, "L2::evict_first");
                                    }
                                    Level2EvictionPriority::L2EvictLast => {
                                            push_directive(tokens, "L2::evict_last");
                                    }
                            }
                    }
                    if let Some(level_cache_hint_16) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_16 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    if let Some(vec_17) = self.vec.as_ref() {
                            match vec_17 {
                                    Vec::V2 => {
                                            push_directive(tokens, "v2");
                                    }
                                    Vec::V4 => {
                                            push_directive(tokens, "v4");
                                    }
                                    Vec::V8 => {
                                            push_directive(tokens, "v8");
                                    }
                            }
                    }
                    match &self.type_ {
                            Type::B128 => {
                                    push_directive(tokens, "b128");
                            }
                            Type::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Type::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Type::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Type::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Type::S16 => {
                                    push_directive(tokens, "s16");
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
                            Type::B8 => {
                                    push_directive(tokens, "b8");
                            }
                            Type::U8 => {
                                    push_directive(tokens, "u8");
                            }
                            Type::S8 => {
                                    push_directive(tokens, "s8");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_18) = self.cache_policy.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_18.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for StReleaseScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "st");
                    push_directive(tokens, "release");
                    match &self.scope {
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
                    if let Some(ss_19) = self.ss.as_ref() {
                            match ss_19 {
                                    Ss::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Ss::ParamFunc => {
                                            push_directive(tokens, "param::func");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::Local => {
                                            push_directive(tokens, "local");
                                    }
                                    Ss::Param => {
                                            push_directive(tokens, "param");
                                    }
                            }
                    }
                    if let Some(level1_eviction_priority_20) = self.level1_eviction_priority.as_ref() {
                            match level1_eviction_priority_20 {
                                    Level1EvictionPriority::L1EvictUnchanged => {
                                            push_directive(tokens, "L1::evict_unchanged");
                                    }
                                    Level1EvictionPriority::L1EvictNormal => {
                                            push_directive(tokens, "L1::evict_normal");
                                    }
                                    Level1EvictionPriority::L1EvictFirst => {
                                            push_directive(tokens, "L1::evict_first");
                                    }
                                    Level1EvictionPriority::L1NoAllocate => {
                                            push_directive(tokens, "L1::no_allocate");
                                    }
                                    Level1EvictionPriority::L1EvictLast => {
                                            push_directive(tokens, "L1::evict_last");
                                    }
                            }
                    }
                    if let Some(level2_eviction_priority_21) = self.level2_eviction_priority.as_ref() {
                            match level2_eviction_priority_21 {
                                    Level2EvictionPriority::L2EvictNormal => {
                                            push_directive(tokens, "L2::evict_normal");
                                    }
                                    Level2EvictionPriority::L2EvictFirst => {
                                            push_directive(tokens, "L2::evict_first");
                                    }
                                    Level2EvictionPriority::L2EvictLast => {
                                            push_directive(tokens, "L2::evict_last");
                                    }
                            }
                    }
                    if let Some(level_cache_hint_22) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_22 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    if let Some(vec_23) = self.vec.as_ref() {
                            match vec_23 {
                                    Vec::V2 => {
                                            push_directive(tokens, "v2");
                                    }
                                    Vec::V4 => {
                                            push_directive(tokens, "v4");
                                    }
                                    Vec::V8 => {
                                            push_directive(tokens, "v8");
                                    }
                            }
                    }
                    match &self.type_ {
                            Type::B128 => {
                                    push_directive(tokens, "b128");
                            }
                            Type::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Type::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Type::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Type::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Type::S16 => {
                                    push_directive(tokens, "s16");
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
                            Type::B8 => {
                                    push_directive(tokens, "b8");
                            }
                            Type::U8 => {
                                    push_directive(tokens, "u8");
                            }
                            Type::S8 => {
                                    push_directive(tokens, "s8");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_24) = self.cache_policy.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_24.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for StMmioRelaxedSysGlobalType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "st");
                    push_directive(tokens, "mmio");
                    push_directive(tokens, "relaxed");
                    push_directive(tokens, "sys");
                    if self.global {
                            push_directive(tokens, "global");
                    }
                    match &self.type_ {
                            Type::B128 => {
                                    push_directive(tokens, "b128");
                            }
                            Type::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Type::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Type::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Type::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Type::S16 => {
                                    push_directive(tokens, "s16");
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
                            Type::B8 => {
                                    push_directive(tokens, "b8");
                            }
                            Type::U8 => {
                                    push_directive(tokens, "u8");
                            }
                            Type::S8 => {
                                    push_directive(tokens, "s8");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

