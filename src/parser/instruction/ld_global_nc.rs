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
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::ld_global_nc::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for LevelPrefetchSize {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try L264b
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::64B").is_ok() {
                    return Ok(LevelPrefetchSize::L264b);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try L2128b
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::128B").is_ok() {
                    return Ok(LevelPrefetchSize::L2128b);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L2::64B", ".L2::128B", ".L2::256B"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".ca", ".cg", ".cs"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".v2", ".v4", ".v8"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Level1EvictionPriority {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try L1EvictNormal
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L1::evict_normal").is_ok() {
                    return Ok(Level1EvictionPriority::L1EvictNormal);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try L1EvictUnchanged
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L1::evict_unchanged").is_ok() {
                    return Ok(Level1EvictionPriority::L1EvictUnchanged);
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
            // Try L1EvictLast
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L1::evict_last").is_ok() {
                    return Ok(Level1EvictionPriority::L1EvictLast);
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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L1::evict_normal", ".L1::evict_unchanged", ".L1::evict_first", ".L1::evict_last", ".L1::no_allocate"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L2::evict_normal", ".L2::evict_first", ".L2::evict_last"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b8").is_ok() {
                    return Ok(Type::B8);
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
            // Try B128
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b128").is_ok() {
                    return Ok(Type::B128);
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
            // Try S8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s8").is_ok() {
                    return Ok(Type::S8);
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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b8", ".b16", ".b32", ".b64", ".b128", ".u8", ".u16", ".u32", ".u64", ".s8", ".s16", ".s32", ".s64", ".f32", ".f64"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L2::cache_hint"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for LdGlobalCopNcLevelCacheHintLevelPrefetchSizeType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("ld")?;
            stream.expect_string(".global")?;
            let global = ();
            let saved_pos = stream.position();
            let cop = match Cop::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_string(".nc")?;
            let nc = ();
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let saved_pos = stream.position();
            let level_prefetch_size = match LevelPrefetchSize::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let type_ = Type::parse(stream)?;
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(LdGlobalCopNcLevelCacheHintLevelPrefetchSizeType {
                global,
                cop,
                nc,
                level_cache_hint,
                level_prefetch_size,
                type_,
                d,
                a,
                cache_policy,
            })
        }
    }


    impl PtxParser for LdGlobalCopNcLevelCacheHintLevelPrefetchSizeVecType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("ld")?;
            stream.expect_string(".global")?;
            let global = ();
            let saved_pos = stream.position();
            let cop = match Cop::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_string(".nc")?;
            let nc = ();
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let saved_pos = stream.position();
            let level_prefetch_size = match LevelPrefetchSize::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let vec = Vec::parse(stream)?;
            let type_ = Type::parse(stream)?;
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(LdGlobalCopNcLevelCacheHintLevelPrefetchSizeVecType {
                global,
                cop,
                nc,
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


    impl PtxParser for LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("ld")?;
            stream.expect_string(".global")?;
            let global = ();
            stream.expect_string(".nc")?;
            let nc = ();
            let saved_pos = stream.position();
            let level1_eviction_priority = match Level1EvictionPriority::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let saved_pos = stream.position();
            let level2_eviction_priority = match Level2EvictionPriority::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let saved_pos = stream.position();
            let level_prefetch_size = match LevelPrefetchSize::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let type_ = Type::parse(stream)?;
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeType {
                global,
                nc,
                level1_eviction_priority,
                level2_eviction_priority,
                level_cache_hint,
                level_prefetch_size,
                type_,
                d,
                a,
                cache_policy,
            })
        }
    }


    impl PtxParser for LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("ld")?;
            stream.expect_string(".global")?;
            let global = ();
            stream.expect_string(".nc")?;
            let nc = ();
            let saved_pos = stream.position();
            let level1_eviction_priority = match Level1EvictionPriority::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let saved_pos = stream.position();
            let level2_eviction_priority = match Level2EvictionPriority::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let saved_pos = stream.position();
            let level_prefetch_size = match LevelPrefetchSize::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let vec = Vec::parse(stream)?;
            let type_ = Type::parse(stream)?;
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
                global,
                nc,
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


}

