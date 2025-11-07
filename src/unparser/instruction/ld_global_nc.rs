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

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::ld_global_nc::section_0::*;

    impl PtxUnparser for LdGlobalCopNcLevelCacheHintLevelPrefetchSizeType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "ld");
                    push_directive(tokens, "global");
                    if let Some(cop_0) = self.cop.as_ref() {
                            match cop_0 {
                                    Cop::Ca => {
                                            push_directive(tokens, "ca");
                                    }
                                    Cop::Cg => {
                                            push_directive(tokens, "cg");
                                    }
                                    Cop::Cs => {
                                            push_directive(tokens, "cs");
                                    }
                            }
                    }
                    push_directive(tokens, "nc");
                    if let Some(level_cache_hint_1) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_1 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    if let Some(level_prefetch_size_2) = self.level_prefetch_size.as_ref() {
                            match level_prefetch_size_2 {
                                    LevelPrefetchSize::L2128b => {
                                            push_directive(tokens, "L2::128B");
                                    }
                                    LevelPrefetchSize::L2256b => {
                                            push_directive(tokens, "L2::256B");
                                    }
                                    LevelPrefetchSize::L264b => {
                                            push_directive(tokens, "L2::64B");
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
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_3) = self.cache_policy.as_ref() {
                        opt_3.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for LdGlobalCopNcLevelCacheHintLevelPrefetchSizeVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "ld");
                    push_directive(tokens, "global");
                    if let Some(cop_4) = self.cop.as_ref() {
                            match cop_4 {
                                    Cop::Ca => {
                                            push_directive(tokens, "ca");
                                    }
                                    Cop::Cg => {
                                            push_directive(tokens, "cg");
                                    }
                                    Cop::Cs => {
                                            push_directive(tokens, "cs");
                                    }
                            }
                    }
                    push_directive(tokens, "nc");
                    if let Some(level_cache_hint_5) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_5 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    if let Some(level_prefetch_size_6) = self.level_prefetch_size.as_ref() {
                            match level_prefetch_size_6 {
                                    LevelPrefetchSize::L2128b => {
                                            push_directive(tokens, "L2::128B");
                                    }
                                    LevelPrefetchSize::L2256b => {
                                            push_directive(tokens, "L2::256B");
                                    }
                                    LevelPrefetchSize::L264b => {
                                            push_directive(tokens, "L2::64B");
                                    }
                            }
                    }
                    match &self.vec {
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
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_7) = self.cache_policy.as_ref() {
                        opt_7.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "ld");
                    push_directive(tokens, "global");
                    push_directive(tokens, "nc");
                    if let Some(level1_eviction_priority_8) = self.level1_eviction_priority.as_ref() {
                            match level1_eviction_priority_8 {
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
                    if let Some(level2_eviction_priority_9) = self.level2_eviction_priority.as_ref() {
                            match level2_eviction_priority_9 {
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
                    if let Some(level_cache_hint_10) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_10 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    if let Some(level_prefetch_size_11) = self.level_prefetch_size.as_ref() {
                            match level_prefetch_size_11 {
                                    LevelPrefetchSize::L2128b => {
                                            push_directive(tokens, "L2::128B");
                                    }
                                    LevelPrefetchSize::L2256b => {
                                            push_directive(tokens, "L2::256B");
                                    }
                                    LevelPrefetchSize::L264b => {
                                            push_directive(tokens, "L2::64B");
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
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_12) = self.cache_policy.as_ref() {
                        opt_12.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "ld");
                    push_directive(tokens, "global");
                    push_directive(tokens, "nc");
                    if let Some(level1_eviction_priority_13) = self.level1_eviction_priority.as_ref() {
                            match level1_eviction_priority_13 {
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
                    if let Some(level2_eviction_priority_14) = self.level2_eviction_priority.as_ref() {
                            match level2_eviction_priority_14 {
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
                    if let Some(level_cache_hint_15) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_15 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    if let Some(level_prefetch_size_16) = self.level_prefetch_size.as_ref() {
                            match level_prefetch_size_16 {
                                    LevelPrefetchSize::L2128b => {
                                            push_directive(tokens, "L2::128B");
                                    }
                                    LevelPrefetchSize::L2256b => {
                                            push_directive(tokens, "L2::256B");
                                    }
                                    LevelPrefetchSize::L264b => {
                                            push_directive(tokens, "L2::64B");
                                    }
                            }
                    }
                    match &self.vec {
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
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_17) = self.cache_policy.as_ref() {
                        opt_17.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

