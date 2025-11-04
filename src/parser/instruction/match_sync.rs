//! Original PTX specification:
//!
//! match.any.sync.type  d, a, membermask;
//! match.all.sync.type  d{|p}, a, membermask;
//! .type = { .b32, .b64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::match_sync::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b32").is_ok() {
                    return Ok(Type::B32);
                }
                stream.set_position(saved_pos);
            }
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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b32", ".b64"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for MatchAnySyncType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("match")?;
            stream.expect_string(".any")?;
            let any = ();
            stream.expect_string(".sync")?;
            let sync = ();
            let type_ = Type::parse(stream)?;
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let membermask = Operand::parse(stream)?;
            Ok(MatchAnySyncType {
                any,
                sync,
                type_,
                d,
                a,
                membermask,
            })
        }
    }


    impl PtxParser for MatchAllSyncType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("match")?;
            stream.expect_string(".all")?;
            let all = ();
            stream.expect_string(".sync")?;
            let sync = ();
            let type_ = Type::parse(stream)?;
            let d = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let p = if stream.consume_if(|t| matches!(t, PtxToken::Pipe)).is_some() {
                Some(Operand::parse(stream)?)
            } else {
                stream.set_position(saved_pos);
                None
            };
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let membermask = Operand::parse(stream)?;
            Ok(MatchAllSyncType {
                all,
                sync,
                type_,
                d,
                a,
                membermask,
            })
        }
    }


}

