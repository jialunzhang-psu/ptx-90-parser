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

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::ld::section_0::*;

    impl PtxUnparser for LdWeakSsCopLevelCacheHintLevelPrefetchSizeVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "ld");
                    if self.weak {
                            push_directive(tokens, "weak");
                    }
                    if let Some(ss_0) = self.ss.as_ref() {
                            match ss_0 {
                                    Ss::Const => {
                                            push_directive(tokens, "const");
                                    }
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Local => {
                                            push_directive(tokens, "local");
                                    }
                                    Ss::ParamEntry => {
                                            push_directive(tokens, "param::entry");
                                    }
                                    Ss::ParamFunc => {
                                            push_directive(tokens, "param::func");
                                    }
                                    Ss::Param => {
                                            push_directive(tokens, "param");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                            }
                    }
                    if let Some(cop_1) = self.cop.as_ref() {
                            match cop_1 {
                                    Cop::Ca => {
                                            push_directive(tokens, "ca");
                                    }
                                    Cop::Cg => {
                                            push_directive(tokens, "cg");
                                    }
                                    Cop::Cs => {
                                            push_directive(tokens, "cs");
                                    }
                                    Cop::Lu => {
                                            push_directive(tokens, "lu");
                                    }
                                    Cop::Cv => {
                                            push_directive(tokens, "cv");
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
                    if let Some(level_prefetch_size_3) = self.level_prefetch_size.as_ref() {
                            match level_prefetch_size_3 {
                                    LevelPrefetchSize::L264b => {
                                            push_directive(tokens, "L2::64B");
                                    }
                                    LevelPrefetchSize::L2128b => {
                                            push_directive(tokens, "L2::128B");
                                    }
                                    LevelPrefetchSize::L2256b => {
                                            push_directive(tokens, "L2::256B");
                                    }
                            }
                    }
                    if let Some(vec_4) = self.vec.as_ref() {
                            match vec_4 {
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
                            Type::B8 => {
                                    push_directive(tokens, "b8");
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
                            Type::B128 => {
                                    push_directive(tokens, "b128");
                            }
                            Type::U8 => {
                                    push_directive(tokens, "u8");
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
                            Type::S8 => {
                                    push_directive(tokens, "s8");
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
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
                    if self.unified {
                            push_directive(tokens, "unified");
                    }
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_5) = self.cache_policy.as_ref() {
                        opt_5.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for LdWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "ld");
                    if self.weak {
                            push_directive(tokens, "weak");
                    }
                    if let Some(ss_6) = self.ss.as_ref() {
                            match ss_6 {
                                    Ss::Const => {
                                            push_directive(tokens, "const");
                                    }
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Local => {
                                            push_directive(tokens, "local");
                                    }
                                    Ss::ParamEntry => {
                                            push_directive(tokens, "param::entry");
                                    }
                                    Ss::ParamFunc => {
                                            push_directive(tokens, "param::func");
                                    }
                                    Ss::Param => {
                                            push_directive(tokens, "param");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                            }
                    }
                    if let Some(level1_eviction_priority_7) = self.level1_eviction_priority.as_ref() {
                            match level1_eviction_priority_7 {
                                    Level1EvictionPriority::L1EvictNormal => {
                                            push_directive(tokens, "L1::evict_normal");
                                    }
                                    Level1EvictionPriority::L1EvictUnchanged => {
                                            push_directive(tokens, "L1::evict_unchanged");
                                    }
                                    Level1EvictionPriority::L1EvictFirst => {
                                            push_directive(tokens, "L1::evict_first");
                                    }
                                    Level1EvictionPriority::L1EvictLast => {
                                            push_directive(tokens, "L1::evict_last");
                                    }
                                    Level1EvictionPriority::L1NoAllocate => {
                                            push_directive(tokens, "L1::no_allocate");
                                    }
                            }
                    }
                    if let Some(level2_eviction_priority_8) = self.level2_eviction_priority.as_ref() {
                            match level2_eviction_priority_8 {
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
                    if let Some(level_cache_hint_9) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_9 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    if let Some(level_prefetch_size_10) = self.level_prefetch_size.as_ref() {
                            match level_prefetch_size_10 {
                                    LevelPrefetchSize::L264b => {
                                            push_directive(tokens, "L2::64B");
                                    }
                                    LevelPrefetchSize::L2128b => {
                                            push_directive(tokens, "L2::128B");
                                    }
                                    LevelPrefetchSize::L2256b => {
                                            push_directive(tokens, "L2::256B");
                                    }
                            }
                    }
                    if let Some(vec_11) = self.vec.as_ref() {
                            match vec_11 {
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
                            Type::B8 => {
                                    push_directive(tokens, "b8");
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
                            Type::B128 => {
                                    push_directive(tokens, "b128");
                            }
                            Type::U8 => {
                                    push_directive(tokens, "u8");
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
                            Type::S8 => {
                                    push_directive(tokens, "s8");
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
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
                    if self.unified {
                            push_directive(tokens, "unified");
                    }
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_12) = self.cache_policy.as_ref() {
                        opt_12.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for LdVolatileSsLevelPrefetchSizeVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "ld");
                    push_directive(tokens, "volatile");
                    if let Some(ss_13) = self.ss.as_ref() {
                            match ss_13 {
                                    Ss::Const => {
                                            push_directive(tokens, "const");
                                    }
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Local => {
                                            push_directive(tokens, "local");
                                    }
                                    Ss::ParamEntry => {
                                            push_directive(tokens, "param::entry");
                                    }
                                    Ss::ParamFunc => {
                                            push_directive(tokens, "param::func");
                                    }
                                    Ss::Param => {
                                            push_directive(tokens, "param");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                            }
                    }
                    if let Some(level_prefetch_size_14) = self.level_prefetch_size.as_ref() {
                            match level_prefetch_size_14 {
                                    LevelPrefetchSize::L264b => {
                                            push_directive(tokens, "L2::64B");
                                    }
                                    LevelPrefetchSize::L2128b => {
                                            push_directive(tokens, "L2::128B");
                                    }
                                    LevelPrefetchSize::L2256b => {
                                            push_directive(tokens, "L2::256B");
                                    }
                            }
                    }
                    if let Some(vec_15) = self.vec.as_ref() {
                            match vec_15 {
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
                            Type::B8 => {
                                    push_directive(tokens, "b8");
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
                            Type::B128 => {
                                    push_directive(tokens, "b128");
                            }
                            Type::U8 => {
                                    push_directive(tokens, "u8");
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
                            Type::S8 => {
                                    push_directive(tokens, "s8");
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
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for LdRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "ld");
                    push_directive(tokens, "relaxed");
                    match &self.scope {
                            Scope::Cta => {
                                    push_directive(tokens, "cta");
                            }
                            Scope::Cluster => {
                                    push_directive(tokens, "cluster");
                            }
                            Scope::Gpu => {
                                    push_directive(tokens, "gpu");
                            }
                            Scope::Sys => {
                                    push_directive(tokens, "sys");
                            }
                    }
                    if let Some(ss_16) = self.ss.as_ref() {
                            match ss_16 {
                                    Ss::Const => {
                                            push_directive(tokens, "const");
                                    }
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Local => {
                                            push_directive(tokens, "local");
                                    }
                                    Ss::ParamEntry => {
                                            push_directive(tokens, "param::entry");
                                    }
                                    Ss::ParamFunc => {
                                            push_directive(tokens, "param::func");
                                    }
                                    Ss::Param => {
                                            push_directive(tokens, "param");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                            }
                    }
                    if let Some(level1_eviction_priority_17) = self.level1_eviction_priority.as_ref() {
                            match level1_eviction_priority_17 {
                                    Level1EvictionPriority::L1EvictNormal => {
                                            push_directive(tokens, "L1::evict_normal");
                                    }
                                    Level1EvictionPriority::L1EvictUnchanged => {
                                            push_directive(tokens, "L1::evict_unchanged");
                                    }
                                    Level1EvictionPriority::L1EvictFirst => {
                                            push_directive(tokens, "L1::evict_first");
                                    }
                                    Level1EvictionPriority::L1EvictLast => {
                                            push_directive(tokens, "L1::evict_last");
                                    }
                                    Level1EvictionPriority::L1NoAllocate => {
                                            push_directive(tokens, "L1::no_allocate");
                                    }
                            }
                    }
                    if let Some(level2_eviction_priority_18) = self.level2_eviction_priority.as_ref() {
                            match level2_eviction_priority_18 {
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
                    if let Some(level_cache_hint_19) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_19 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    if let Some(level_prefetch_size_20) = self.level_prefetch_size.as_ref() {
                            match level_prefetch_size_20 {
                                    LevelPrefetchSize::L264b => {
                                            push_directive(tokens, "L2::64B");
                                    }
                                    LevelPrefetchSize::L2128b => {
                                            push_directive(tokens, "L2::128B");
                                    }
                                    LevelPrefetchSize::L2256b => {
                                            push_directive(tokens, "L2::256B");
                                    }
                            }
                    }
                    if let Some(vec_21) = self.vec.as_ref() {
                            match vec_21 {
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
                            Type::B8 => {
                                    push_directive(tokens, "b8");
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
                            Type::B128 => {
                                    push_directive(tokens, "b128");
                            }
                            Type::U8 => {
                                    push_directive(tokens, "u8");
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
                            Type::S8 => {
                                    push_directive(tokens, "s8");
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
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_22) = self.cache_policy.as_ref() {
                        opt_22.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for LdAcquireScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "ld");
                    push_directive(tokens, "acquire");
                    match &self.scope {
                            Scope::Cta => {
                                    push_directive(tokens, "cta");
                            }
                            Scope::Cluster => {
                                    push_directive(tokens, "cluster");
                            }
                            Scope::Gpu => {
                                    push_directive(tokens, "gpu");
                            }
                            Scope::Sys => {
                                    push_directive(tokens, "sys");
                            }
                    }
                    if let Some(ss_23) = self.ss.as_ref() {
                            match ss_23 {
                                    Ss::Const => {
                                            push_directive(tokens, "const");
                                    }
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Local => {
                                            push_directive(tokens, "local");
                                    }
                                    Ss::ParamEntry => {
                                            push_directive(tokens, "param::entry");
                                    }
                                    Ss::ParamFunc => {
                                            push_directive(tokens, "param::func");
                                    }
                                    Ss::Param => {
                                            push_directive(tokens, "param");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                            }
                    }
                    if let Some(level1_eviction_priority_24) = self.level1_eviction_priority.as_ref() {
                            match level1_eviction_priority_24 {
                                    Level1EvictionPriority::L1EvictNormal => {
                                            push_directive(tokens, "L1::evict_normal");
                                    }
                                    Level1EvictionPriority::L1EvictUnchanged => {
                                            push_directive(tokens, "L1::evict_unchanged");
                                    }
                                    Level1EvictionPriority::L1EvictFirst => {
                                            push_directive(tokens, "L1::evict_first");
                                    }
                                    Level1EvictionPriority::L1EvictLast => {
                                            push_directive(tokens, "L1::evict_last");
                                    }
                                    Level1EvictionPriority::L1NoAllocate => {
                                            push_directive(tokens, "L1::no_allocate");
                                    }
                            }
                    }
                    if let Some(level2_eviction_priority_25) = self.level2_eviction_priority.as_ref() {
                            match level2_eviction_priority_25 {
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
                    if let Some(level_cache_hint_26) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_26 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    if let Some(level_prefetch_size_27) = self.level_prefetch_size.as_ref() {
                            match level_prefetch_size_27 {
                                    LevelPrefetchSize::L264b => {
                                            push_directive(tokens, "L2::64B");
                                    }
                                    LevelPrefetchSize::L2128b => {
                                            push_directive(tokens, "L2::128B");
                                    }
                                    LevelPrefetchSize::L2256b => {
                                            push_directive(tokens, "L2::256B");
                                    }
                            }
                    }
                    if let Some(vec_28) = self.vec.as_ref() {
                            match vec_28 {
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
                            Type::B8 => {
                                    push_directive(tokens, "b8");
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
                            Type::B128 => {
                                    push_directive(tokens, "b128");
                            }
                            Type::U8 => {
                                    push_directive(tokens, "u8");
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
                            Type::S8 => {
                                    push_directive(tokens, "s8");
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
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_29) = self.cache_policy.as_ref() {
                        opt_29.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for LdMmioRelaxedSysGlobalType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "ld");
                    push_directive(tokens, "mmio");
                    push_directive(tokens, "relaxed");
                    push_directive(tokens, "sys");
                    if self.global {
                            push_directive(tokens, "global");
                    }
                    match &self.type_ {
                            Type::B8 => {
                                    push_directive(tokens, "b8");
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
                            Type::B128 => {
                                    push_directive(tokens, "b128");
                            }
                            Type::U8 => {
                                    push_directive(tokens, "u8");
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
                            Type::S8 => {
                                    push_directive(tokens, "s8");
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
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

