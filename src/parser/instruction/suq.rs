//! Original PTX specification:
//!
//! suq.query.b32   d, [a];
//! .query = { .width, .height, .depth,
//! .channel_data_type, .channel_order,
//! .array_size, .memory_layout };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::suq::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Query {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try ChannelDataType
            {
                let saved_pos = stream.position();
                if stream.expect_string(".channel_data_type").is_ok() {
                    return Ok(Query::ChannelDataType);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try ChannelOrder
            {
                let saved_pos = stream.position();
                if stream.expect_string(".channel_order").is_ok() {
                    return Ok(Query::ChannelOrder);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try MemoryLayout
            {
                let saved_pos = stream.position();
                if stream.expect_string(".memory_layout").is_ok() {
                    return Ok(Query::MemoryLayout);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try ArraySize
            {
                let saved_pos = stream.position();
                if stream.expect_string(".array_size").is_ok() {
                    return Ok(Query::ArraySize);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Height
            {
                let saved_pos = stream.position();
                if stream.expect_string(".height").is_ok() {
                    return Ok(Query::Height);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Width
            {
                let saved_pos = stream.position();
                if stream.expect_string(".width").is_ok() {
                    return Ok(Query::Width);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Depth
            {
                let saved_pos = stream.position();
                if stream.expect_string(".depth").is_ok() {
                    return Ok(Query::Depth);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[
                ".channel_data_type",
                ".channel_order",
                ".memory_layout",
                ".array_size",
                ".height",
                ".width",
                ".depth",
            ];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for SuqQueryB32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("suq")?;
            let query = Query::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".b32")?;
            let b32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SuqQueryB32 { query, b32, d, a })
        }
    }
}
