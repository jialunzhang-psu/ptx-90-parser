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
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::ld::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Cop {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Ca
            {
                let saved_pos = stream.position();
                if stream.expect_string(".ca").is_ok() {
                    return Ok(Cop::Ca);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Cg
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cg").is_ok() {
                    return Ok(Cop::Cg);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Cs
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cs").is_ok() {
                    return Ok(Cop::Cs);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Lu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".lu").is_ok() {
                    return Ok(Cop::Lu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Cv
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cv").is_ok() {
                    return Ok(Cop::Cv);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".ca", ".cg", ".cs", ".lu", ".cv"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Level1EvictionPriority {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try L1EvictUnchanged
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L1::evict_unchanged").is_ok() {
                    return Ok(Level1EvictionPriority::L1EvictUnchanged);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try L1EvictNormal
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L1::evict_normal").is_ok() {
                    return Ok(Level1EvictionPriority::L1EvictNormal);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try L1EvictFirst
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L1::evict_first").is_ok() {
                    return Ok(Level1EvictionPriority::L1EvictFirst);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try L1NoAllocate
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L1::no_allocate").is_ok() {
                    return Ok(Level1EvictionPriority::L1NoAllocate);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try L1EvictLast
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L1::evict_last").is_ok() {
                    return Ok(Level1EvictionPriority::L1EvictLast);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[
                ".L1::evict_unchanged",
                ".L1::evict_normal",
                ".L1::evict_first",
                ".L1::no_allocate",
                ".L1::evict_last",
            ];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Level2EvictionPriority {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try L2EvictNormal
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::evict_normal").is_ok() {
                    return Ok(Level2EvictionPriority::L2EvictNormal);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try L2EvictFirst
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::evict_first").is_ok() {
                    return Ok(Level2EvictionPriority::L2EvictFirst);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try L2EvictLast
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::evict_last").is_ok() {
                    return Ok(Level2EvictionPriority::L2EvictLast);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L2::evict_normal", ".L2::evict_first", ".L2::evict_last"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for LevelCacheHint {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try L2CacheHint
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::cache_hint").is_ok() {
                    return Ok(LevelCacheHint::L2CacheHint);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L2::cache_hint"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for LevelPrefetchSize {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try L2128b
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::128B").is_ok() {
                    return Ok(LevelPrefetchSize::L2128b);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try L2256b
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::256B").is_ok() {
                    return Ok(LevelPrefetchSize::L2256b);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try L264b
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::64B").is_ok() {
                    return Ok(LevelPrefetchSize::L264b);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L2::128B", ".L2::256B", ".L2::64B"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Scope {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Cluster
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cluster").is_ok() {
                    return Ok(Scope::Cluster);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Cta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta").is_ok() {
                    return Ok(Scope::Cta);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Gpu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".gpu").is_ok() {
                    return Ok(Scope::Gpu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Sys
            {
                let saved_pos = stream.position();
                if stream.expect_string(".sys").is_ok() {
                    return Ok(Scope::Sys);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cluster", ".cta", ".gpu", ".sys"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Ss {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try SharedCluster
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared::cluster").is_ok() {
                    return Ok(Ss::SharedCluster);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try ParamEntry
            {
                let saved_pos = stream.position();
                if stream.expect_string(".param::entry").is_ok() {
                    return Ok(Ss::ParamEntry);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try ParamFunc
            {
                let saved_pos = stream.position();
                if stream.expect_string(".param::func").is_ok() {
                    return Ok(Ss::ParamFunc);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try SharedCta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared::cta").is_ok() {
                    return Ok(Ss::SharedCta);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Global
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global").is_ok() {
                    return Ok(Ss::Global);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Shared
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared").is_ok() {
                    return Ok(Ss::Shared);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Const
            {
                let saved_pos = stream.position();
                if stream.expect_string(".const").is_ok() {
                    return Ok(Ss::Const);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Local
            {
                let saved_pos = stream.position();
                if stream.expect_string(".local").is_ok() {
                    return Ok(Ss::Local);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Param
            {
                let saved_pos = stream.position();
                if stream.expect_string(".param").is_ok() {
                    return Ok(Ss::Param);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[
                ".shared::cluster",
                ".param::entry",
                ".param::func",
                ".shared::cta",
                ".global",
                ".shared",
                ".const",
                ".local",
                ".param",
            ];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B128
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b128").is_ok() {
                    return Ok(Type::B128);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b16").is_ok() {
                    return Ok(Type::B16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b32").is_ok() {
                    return Ok(Type::B32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b64").is_ok() {
                    return Ok(Type::B64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u16").is_ok() {
                    return Ok(Type::U16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u32").is_ok() {
                    return Ok(Type::U32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u64").is_ok() {
                    return Ok(Type::U64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s16").is_ok() {
                    return Ok(Type::S16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Type::S32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s64").is_ok() {
                    return Ok(Type::S64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try F32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f32").is_ok() {
                    return Ok(Type::F32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try F64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f64").is_ok() {
                    return Ok(Type::F64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b8").is_ok() {
                    return Ok(Type::B8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u8").is_ok() {
                    return Ok(Type::U8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s8").is_ok() {
                    return Ok(Type::S8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[
                ".b128", ".b16", ".b32", ".b64", ".u16", ".u32", ".u64", ".s16", ".s32", ".s64",
                ".f32", ".f64", ".b8", ".u8", ".s8",
            ];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Vec {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try V2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".v2").is_ok() {
                    return Ok(Vec::V2);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try V4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".v4").is_ok() {
                    return Ok(Vec::V4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try V8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".v8").is_ok() {
                    return Ok(Vec::V8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".v2", ".v4", ".v8"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for LdWeakSsCopLevelCacheHintLevelPrefetchSizeVecType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("ld")?;
            let saved_pos = stream.position();
            let weak = stream.expect_string(".weak").is_ok();
            if !weak {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let cop = match Cop::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_prefetch_size = match LevelPrefetchSize::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let vec = match Vec::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let unified = stream.expect_string(".unified").is_ok();
            if !unified {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(LdWeakSsCopLevelCacheHintLevelPrefetchSizeVecType {
                weak,
                ss,
                cop,
                level_cache_hint,
                level_prefetch_size,
                vec,
                type_,
                d,
                a,
                unified,
                cache_policy,
            })
        }
    }

    impl PtxParser for LdWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("ld")?;
            let saved_pos = stream.position();
            let weak = stream.expect_string(".weak").is_ok();
            if !weak {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level1_eviction_priority = match Level1EvictionPriority::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level2_eviction_priority = match Level2EvictionPriority::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_prefetch_size = match LevelPrefetchSize::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let vec = match Vec::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let unified = stream.expect_string(".unified").is_ok();
            if !unified {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(LdWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
                weak,
                ss,
                level1_eviction_priority,
                level2_eviction_priority,
                level_cache_hint,
                level_prefetch_size,
                vec,
                type_,
                d,
                a,
                unified,
                cache_policy,
            })
        }
    }

    impl PtxParser for LdVolatileSsLevelPrefetchSizeVecType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("ld")?;
            stream.expect_string(".volatile")?;
            let volatile = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_prefetch_size = match LevelPrefetchSize::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let vec = match Vec::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(LdVolatileSsLevelPrefetchSizeVecType {
                volatile,
                ss,
                level_prefetch_size,
                vec,
                type_,
                d,
                a,
            })
        }
    }

    impl PtxParser for LdRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("ld")?;
            stream.expect_string(".relaxed")?;
            let relaxed = ();
            stream.expect_complete()?;
            let scope = Scope::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level1_eviction_priority = match Level1EvictionPriority::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level2_eviction_priority = match Level2EvictionPriority::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_prefetch_size = match LevelPrefetchSize::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let vec = match Vec::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(LdRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
                relaxed,
                scope,
                ss,
                level1_eviction_priority,
                level2_eviction_priority,
                level_cache_hint,
                level_prefetch_size,
                vec,
                type_,
                d,
                a,
                cache_policy,
            })
        }
    }

    impl PtxParser for LdAcquireScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("ld")?;
            stream.expect_string(".acquire")?;
            let acquire = ();
            stream.expect_complete()?;
            let scope = Scope::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level1_eviction_priority = match Level1EvictionPriority::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level2_eviction_priority = match Level2EvictionPriority::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_prefetch_size = match LevelPrefetchSize::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let vec = match Vec::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(LdAcquireScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
                acquire,
                scope,
                ss,
                level1_eviction_priority,
                level2_eviction_priority,
                level_cache_hint,
                level_prefetch_size,
                vec,
                type_,
                d,
                a,
                cache_policy,
            })
        }
    }

    impl PtxParser for LdMmioRelaxedSysGlobalType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("ld")?;
            stream.expect_string(".mmio")?;
            let mmio = ();
            stream.expect_complete()?;
            stream.expect_string(".relaxed")?;
            let relaxed = ();
            stream.expect_complete()?;
            stream.expect_string(".sys")?;
            let sys = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let global = stream.expect_string(".global").is_ok();
            if !global {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(LdMmioRelaxedSysGlobalType {
                mmio,
                relaxed,
                sys,
                global,
                type_,
                d,
                a,
            })
        }
    }
}
