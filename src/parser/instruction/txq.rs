//! Original PTX specification:
//!
//! txq.tquery.b32         d, [a];       // texture attributes
//! txq.level.tlquery.b32  d, [a], lod;  // texture attributes
//! txq.squery.b32         d, [a];       // sampler attributes
//! .tquery  = { .width, .height, .depth,
//! .channel_data_type, .channel_order,
//! .normalized_coords, .array_size,
//! .num_mipmap_levels, .num_samples};
//! .tlquery = { .width, .height, .depth };
//! .squery  = { .force_unnormalized_coords, .filter_mode,
//! .addr_mode_0, addr_mode_1, addr_mode_2 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::txq::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Squery {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try ForceUnnormalizedCoords
            {
                let saved_pos = stream.position();
                if stream.expect_string(".force_unnormalized_coords").is_ok() {
                    return Ok(Squery::ForceUnnormalizedCoords);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try FilterMode
            {
                let saved_pos = stream.position();
                if stream.expect_string(".filter_mode").is_ok() {
                    return Ok(Squery::FilterMode);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try AddrMode0
            {
                let saved_pos = stream.position();
                if stream.expect_string(".addr_mode_0").is_ok() {
                    return Ok(Squery::AddrMode0);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try AddrMode1
            {
                let saved_pos = stream.position();
                if stream.expect_string("addr_mode_1").is_ok() {
                    return Ok(Squery::AddrMode1);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try AddrMode2
            {
                let saved_pos = stream.position();
                if stream.expect_string("addr_mode_2").is_ok() {
                    return Ok(Squery::AddrMode2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[
                ".force_unnormalized_coords",
                ".filter_mode",
                ".addr_mode_0",
                "addr_mode_1",
                "addr_mode_2",
            ];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Tlquery {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Height
            {
                let saved_pos = stream.position();
                if stream.expect_string(".height").is_ok() {
                    return Ok(Tlquery::Height);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Width
            {
                let saved_pos = stream.position();
                if stream.expect_string(".width").is_ok() {
                    return Ok(Tlquery::Width);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Depth
            {
                let saved_pos = stream.position();
                if stream.expect_string(".depth").is_ok() {
                    return Ok(Tlquery::Depth);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".height", ".width", ".depth"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Tquery {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try ChannelDataType
            {
                let saved_pos = stream.position();
                if stream.expect_string(".channel_data_type").is_ok() {
                    return Ok(Tquery::ChannelDataType);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try NormalizedCoords
            {
                let saved_pos = stream.position();
                if stream.expect_string(".normalized_coords").is_ok() {
                    return Ok(Tquery::NormalizedCoords);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try NumMipmapLevels
            {
                let saved_pos = stream.position();
                if stream.expect_string(".num_mipmap_levels").is_ok() {
                    return Ok(Tquery::NumMipmapLevels);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try ChannelOrder
            {
                let saved_pos = stream.position();
                if stream.expect_string(".channel_order").is_ok() {
                    return Ok(Tquery::ChannelOrder);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try NumSamples
            {
                let saved_pos = stream.position();
                if stream.expect_string(".num_samples").is_ok() {
                    return Ok(Tquery::NumSamples);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try ArraySize
            {
                let saved_pos = stream.position();
                if stream.expect_string(".array_size").is_ok() {
                    return Ok(Tquery::ArraySize);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Height
            {
                let saved_pos = stream.position();
                if stream.expect_string(".height").is_ok() {
                    return Ok(Tquery::Height);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Width
            {
                let saved_pos = stream.position();
                if stream.expect_string(".width").is_ok() {
                    return Ok(Tquery::Width);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Depth
            {
                let saved_pos = stream.position();
                if stream.expect_string(".depth").is_ok() {
                    return Ok(Tquery::Depth);
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
                ".normalized_coords",
                ".num_mipmap_levels",
                ".channel_order",
                ".num_samples",
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

    impl PtxParser for TxqTqueryB32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("txq")?;
            let tquery = Tquery::parse(stream)?;
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
            Ok(TxqTqueryB32 { tquery, b32, d, a })
        }
    }

    impl PtxParser for TxqLevelTlqueryB32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("txq")?;
            stream.expect_string(".level")?;
            let level = ();
            stream.expect_complete()?;
            let tlquery = Tlquery::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".b32")?;
            let b32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let lod = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(TxqLevelTlqueryB32 {
                level,
                tlquery,
                b32,
                d,
                a,
                lod,
            })
        }
    }

    impl PtxParser for TxqSqueryB32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("txq")?;
            let squery = Squery::parse(stream)?;
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
            Ok(TxqSqueryB32 { squery, b32, d, a })
        }
    }
}
