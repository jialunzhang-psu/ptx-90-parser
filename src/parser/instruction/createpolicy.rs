//! Original PTX specification:
//!
//! // Range-based policy
//! createpolicy.range{.global}.level::primary_priority{.level::secondary_priority}.b64
//! cache-policy, [a], primary-size, total-size;
//! // Fraction-based policy
//! createpolicy.fractional.level::primary_priority{.level::secondary_priority}.b64
//! cache-policy{, fraction};
//! // Converting the access property from CUDA APIs
//! createpolicy.cvt.L2.b64            cache-policy, access-property;
//! .level::primary_priority =   { .L2::evict_last, .L2::evict_normal,
//! .L2::evict_first, .L2::evict_unchanged };
//! .level::secondary_priority = { .L2::evict_first, .L2::evict_unchanged };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::createpolicy::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for LevelPrimaryPriority {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try L2EvictUnchanged
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::evict_unchanged").is_ok() {
                    return Ok(LevelPrimaryPriority::L2EvictUnchanged);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try L2EvictNormal
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::evict_normal").is_ok() {
                    return Ok(LevelPrimaryPriority::L2EvictNormal);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try L2EvictFirst
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::evict_first").is_ok() {
                    return Ok(LevelPrimaryPriority::L2EvictFirst);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try L2EvictLast
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::evict_last").is_ok() {
                    return Ok(LevelPrimaryPriority::L2EvictLast);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L2::evict_unchanged", ".L2::evict_normal", ".L2::evict_first", ".L2::evict_last"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for LevelSecondaryPriority {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try L2EvictUnchanged
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::evict_unchanged").is_ok() {
                    return Ok(LevelSecondaryPriority::L2EvictUnchanged);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try L2EvictFirst
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::evict_first").is_ok() {
                    return Ok(LevelSecondaryPriority::L2EvictFirst);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L2::evict_unchanged", ".L2::evict_first"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CreatepolicyRangeGlobalLevelPrimaryPriorityLevelSecondaryPriorityB64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("createpolicy")?;
            stream.expect_string(".range")?;
            let range = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let global = stream.expect_string(".global").is_ok();
            if !global {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let level_primary_priority = LevelPrimaryPriority::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_secondary_priority = match LevelSecondaryPriority::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".b64")?;
            let b64 = ();
            stream.expect_complete()?;
            let cache_policy = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let primary_size = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let total_size = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CreatepolicyRangeGlobalLevelPrimaryPriorityLevelSecondaryPriorityB64 {
                range,
                global,
                level_primary_priority,
                level_secondary_priority,
                b64,
                cache_policy,
                a,
                primary_size,
                total_size,
            })
        }
    }


    impl PtxParser for CreatepolicyFractionalLevelPrimaryPriorityLevelSecondaryPriorityB64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("createpolicy")?;
            stream.expect_string(".fractional")?;
            let fractional = ();
            stream.expect_complete()?;
            let level_primary_priority = LevelPrimaryPriority::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_secondary_priority = match LevelSecondaryPriority::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".b64")?;
            let b64 = ();
            stream.expect_complete()?;
            let cache_policy = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let fraction = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CreatepolicyFractionalLevelPrimaryPriorityLevelSecondaryPriorityB64 {
                fractional,
                level_primary_priority,
                level_secondary_priority,
                b64,
                cache_policy,
                fraction,
            })
        }
    }


    impl PtxParser for CreatepolicyCvtL2B64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("createpolicy")?;
            stream.expect_string(".cvt")?;
            let cvt = ();
            stream.expect_complete()?;
            stream.expect_string(".L2")?;
            let l2 = ();
            stream.expect_complete()?;
            stream.expect_string(".b64")?;
            let b64 = ();
            stream.expect_complete()?;
            let cache_policy = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let access_property = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CreatepolicyCvtL2B64 {
                cvt,
                l2,
                b64,
                cache_policy,
                access_property,
            })
        }
    }


}

