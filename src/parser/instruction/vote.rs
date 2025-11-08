//! Original PTX specification:
//!
//! vote.mode.pred  d, {!}a;
//! vote.ballot.b32 d, {!}a;  // 'ballot' form, returns bitmask
//! .mode = { .all, .any, .uni };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::vote::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Mode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try All
            {
                let saved_pos = stream.position();
                if stream.expect_string(".all").is_ok() {
                    return Ok(Mode::All);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Any
            {
                let saved_pos = stream.position();
                if stream.expect_string(".any").is_ok() {
                    return Ok(Mode::Any);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Uni
            {
                let saved_pos = stream.position();
                if stream.expect_string(".uni").is_ok() {
                    return Ok(Mode::Uni);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".all", ".any", ".uni"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for VoteModePred {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vote")?;
            let mode = Mode::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".pred")?;
            let pred = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a_op = stream
                .consume_if(|t| matches!(t, PtxToken::Exclaim))
                .is_some();
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(VoteModePred {
                mode,
                pred,
                d,
                a_op,
                a,
            })
        }
    }

    impl PtxParser for VoteBallotB32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vote")?;
            stream.expect_string(".ballot")?;
            let ballot = ();
            stream.expect_complete()?;
            stream.expect_string(".b32")?;
            let b32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a_op = stream
                .consume_if(|t| matches!(t, PtxToken::Exclaim))
                .is_some();
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(VoteBallotB32 {
                ballot,
                b32,
                d,
                a_op,
                a,
            })
        }
    }
}
