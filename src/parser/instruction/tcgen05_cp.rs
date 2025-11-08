//! Original PTX specification:
//!
//! tcgen05.cp.cta_group.shape{.multicast}{.dst_src_fmt} [taddr], s-desc;
//! .cta_group = { .cta_group::1, .cta_group::2 };
//! .dst_src_fmt   = { .b8x16.b6x16_p32, .b8x16.b4x16_p64 };
//! .shape     = { .128x256b, .4x256b, .128x128b, .64x128b**, .32x128b*** };
//! .multicast = { .warpx2::02_13** , .warpx2::01_23**, .warpx4*** };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_cp::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CtaGroup {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try CtaGroup1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::1").is_ok() {
                    return Ok(CtaGroup::CtaGroup1);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try CtaGroup2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::2").is_ok() {
                    return Ok(CtaGroup::CtaGroup2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cta_group::1", ".cta_group::2"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for DstSrcFmt {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B8x16B6x16P32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b8x16.b6x16_p32").is_ok() {
                    return Ok(DstSrcFmt::B8x16B6x16P32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B8x16B4x16P64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b8x16.b4x16_p64").is_ok() {
                    return Ok(DstSrcFmt::B8x16B4x16P64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b8x16.b6x16_p32", ".b8x16.b4x16_p64"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Multicast {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Warpx20213
            {
                let saved_pos = stream.position();
                if stream.expect_string(".warpx2::02_13**").is_ok() {
                    return Ok(Multicast::Warpx20213);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Warpx20123
            {
                let saved_pos = stream.position();
                if stream.expect_string(".warpx2::01_23**").is_ok() {
                    return Ok(Multicast::Warpx20123);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Warpx4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".warpx4***").is_ok() {
                    return Ok(Multicast::Warpx4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".warpx2::02_13**", ".warpx2::01_23**", ".warpx4***"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try _32x128b
            {
                let saved_pos = stream.position();
                if stream.expect_string(".32x128b***").is_ok() {
                    return Ok(Shape::_32x128b);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try _64x128b
            {
                let saved_pos = stream.position();
                if stream.expect_string(".64x128b**").is_ok() {
                    return Ok(Shape::_64x128b);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _128x256b
            {
                let saved_pos = stream.position();
                if stream.expect_string(".128x256b").is_ok() {
                    return Ok(Shape::_128x256b);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _128x128b
            {
                let saved_pos = stream.position();
                if stream.expect_string(".128x128b").is_ok() {
                    return Ok(Shape::_128x128b);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _4x256b
            {
                let saved_pos = stream.position();
                if stream.expect_string(".4x256b").is_ok() {
                    return Ok(Shape::_4x256b);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[
                ".32x128b***",
                ".64x128b**",
                ".128x256b",
                ".128x128b",
                ".4x256b",
            ];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Tcgen05CpCtaGroupShapeMulticastDstSrcFmt {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".cp")?;
            let cp = ();
            stream.expect_complete()?;
            let cta_group = CtaGroup::parse(stream)?;
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let multicast = match Multicast::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let dst_src_fmt = match DstSrcFmt::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let taddr = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let s_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Tcgen05CpCtaGroupShapeMulticastDstSrcFmt {
                cp,
                cta_group,
                shape,
                multicast,
                dst_src_fmt,
                taddr,
                s_desc,
            })
        }
    }
}
