//! Original PTX specification:
//!
//! prefetch{.space}.level                    [a];   // prefetch to data cache
//! prefetch.global.level::eviction_priority  [a];   // prefetch to data cache
//! prefetchu.L1  [a];             // prefetch to uniform cache
//! prefetch{.tensormap_space}.tensormap [a];  // prefetch the tensormap
//! .space =                    { .global, .local };
//! .level =                    { .L1, .L2 };
//! .level::eviction_priority = { .L2::evict_last, .L2::evict_normal };
//! .tensormap_space =          { .const, .param };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::prefetch::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Level {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try L1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L1").is_ok() {
                    return Ok(Level::L1);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try L2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2").is_ok() {
                    return Ok(Level::L2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L1", ".L2"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for TensormapSpace {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Const
            {
                let saved_pos = stream.position();
                if stream.expect_string(".const").is_ok() {
                    return Ok(TensormapSpace::Const);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Param
            {
                let saved_pos = stream.position();
                if stream.expect_string(".param").is_ok() {
                    return Ok(TensormapSpace::Param);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".const", ".param"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for LevelEvictionPriority {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try L2EvictLast
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::evict_last").is_ok() {
                    return Ok(LevelEvictionPriority::L2EvictLast);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try L2EvictNormal
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::evict_normal").is_ok() {
                    return Ok(LevelEvictionPriority::L2EvictNormal);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L2::evict_last", ".L2::evict_normal"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Space {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Global
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global").is_ok() {
                    return Ok(Space::Global);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Local
            {
                let saved_pos = stream.position();
                if stream.expect_string(".local").is_ok() {
                    return Ok(Space::Local);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".global", ".local"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for PrefetchSpaceLevel {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("prefetch")?;
            let saved_pos = stream.position();
            let space = match Space::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let level = Level::parse(stream)?;
            let a = AddressOperand::parse(stream)?;
            Ok(PrefetchSpaceLevel {
                space,
                level,
                a,
            })
        }
    }


    impl PtxParser for PrefetchGlobalLevelEvictionPriority {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("prefetch")?;
            stream.expect_string(".global")?;
            let global = ();
            let level_eviction_priority = LevelEvictionPriority::parse(stream)?;
            let a = AddressOperand::parse(stream)?;
            Ok(PrefetchGlobalLevelEvictionPriority {
                global,
                level_eviction_priority,
                a,
            })
        }
    }


    impl PtxParser for PrefetchuL1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("prefetchu")?;
            stream.expect_string(".L1")?;
            let l1 = ();
            let a = AddressOperand::parse(stream)?;
            Ok(PrefetchuL1 {
                l1,
                a,
            })
        }
    }


    impl PtxParser for PrefetchTensormapSpaceTensormap {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("prefetch")?;
            let saved_pos = stream.position();
            let tensormap_space = match TensormapSpace::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_string(".tensormap")?;
            let tensormap = ();
            let a = AddressOperand::parse(stream)?;
            Ok(PrefetchTensormapSpaceTensormap {
                tensormap_space,
                tensormap,
                a,
            })
        }
    }


}

