//! Original PTX specification:
//!
//! applypriority{.global}.level::eviction_priority  [a], size;
//! .level::eviction_priority = { .L2::evict_normal };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::applypriority::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for LevelEvictionPriority {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try L2EvictNormal
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::evict_normal").is_ok() {
                    return Ok(LevelEvictionPriority::L2EvictNormal);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L2::evict_normal"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for ApplypriorityGlobalLevelEvictionPriority {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("applypriority")?;
            let saved_pos = stream.position();
            let global = stream.expect_string(".global").is_ok();
            if !global {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let level_eviction_priority = LevelEvictionPriority::parse(stream)?;
            stream.expect_complete()?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let size = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(ApplypriorityGlobalLevelEvictionPriority {
                global,
                level_eviction_priority,
                a,
                size,
            })
        }
    }
}
