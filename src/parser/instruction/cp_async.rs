//! Original PTX specification:
//!
//! cp.async.ca.state.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], cp-size{, src-size}{, cache-policy};
//! cp.async.cg.state.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], 16{, src-size}{, cache-policy};
//! cp.async.ca.state.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], cp-size{, ignore-src}{, cache-policy} ;
//! cp.async.cg.state.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], 16{, ignore-src}{, cache-policy} ;
//! .level::cache_hint =     { .L2::cache_hint };
//! .level::prefetch_size =  { .L2::64B, .L2::128B, .L2::256B };
//! cp-size = { 4, 8, 16 };
//! .state = { .shared, .shared::cta}

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_async::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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

    impl PtxParser for CpSize {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            let start_pos = stream.position();
            if stream.expect_string("4").is_ok() {
                return Ok(CpSize::_4);
            }
            stream.set_position(start_pos);
            if stream.expect_string("8").is_ok() {
                return Ok(CpSize::_8);
            }
            stream.set_position(start_pos);
            if stream.expect_string("16").is_ok() {
                return Ok(CpSize::_16);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(0..0);
            let expected = &["4", "8", "16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for State {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Shared
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared").is_ok() {
                    return Ok(State::Shared);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try SharedCta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared::cta").is_ok() {
                    return Ok(State::SharedCta);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".shared", ".shared::cta"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cp")?;
            stream.expect_string(".async")?;
            let async_ = ();
            stream.expect_string(".ca")?;
            let ca = ();
            let state = State::parse(stream)?;
            stream.expect_string(".global")?;
            let global = ();
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
            let dst = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let src = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let cp_size = CpSize::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let src_size = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
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
            Ok(CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize {
                async_,
                ca,
                state,
                global,
                level_cache_hint,
                level_prefetch_size,
                dst,
                src,
                cp_size,
                src_size,
                cache_policy,
            })
        }
    }


    impl PtxParser for CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cp")?;
            stream.expect_string(".async")?;
            let async_ = ();
            stream.expect_string(".cg")?;
            let cg = ();
            let state = State::parse(stream)?;
            stream.expect_string(".global")?;
            let global = ();
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
            let dst = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let src = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            stream.expect_string("16")?;
            let imm_16 = ();
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let src_size = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
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
            Ok(CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize {
                async_,
                cg,
                state,
                global,
                level_cache_hint,
                level_prefetch_size,
                dst,
                src,
                imm_16,
                src_size,
                cache_policy,
            })
        }
    }


    impl PtxParser for CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cp")?;
            stream.expect_string(".async")?;
            let async_ = ();
            stream.expect_string(".ca")?;
            let ca = ();
            let state = State::parse(stream)?;
            stream.expect_string(".global")?;
            let global = ();
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
            let dst = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let src = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let cp_size = CpSize::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let ignore_src = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
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
            Ok(CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize1 {
                async_,
                ca,
                state,
                global,
                level_cache_hint,
                level_prefetch_size,
                dst,
                src,
                cp_size,
                ignore_src,
                cache_policy,
            })
        }
    }


    impl PtxParser for CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cp")?;
            stream.expect_string(".async")?;
            let async_ = ();
            stream.expect_string(".cg")?;
            let cg = ();
            let state = State::parse(stream)?;
            stream.expect_string(".global")?;
            let global = ();
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
            let dst = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let src = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            stream.expect_string("16")?;
            let imm_16 = ();
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let ignore_src = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
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
            Ok(CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize1 {
                async_,
                cg,
                state,
                global,
                level_cache_hint,
                level_prefetch_size,
                dst,
                src,
                imm_16,
                ignore_src,
                cache_policy,
            })
        }
    }


}

