//! Original PTX specification:
//!
//! tcgen05.st.sync.aligned.shape1.num{.unpack}.b32    [taddr], r;
//! tcgen05.st.sync.aligned.shape2.num{.unpack}.b32    [taddr], immHalfSplitoff, r;
//! .shape1 = { .16x64b, .16x128b, .16x256b, .32x32b };
//! .shape2 = { .16x32bx2 };
//! .num    = { .x1, .x2, .x4, .x8, .x16, .x32, .x64, .x128 };
//! .unpack = { .unpack::16b };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_st::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Shape2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try _16x32bx2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".16x32bx2").is_ok() {
                    return Ok(Shape2::_16x32bx2);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".16x32bx2"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try _16x64b
            {
                let saved_pos = stream.position();
                if stream.expect_string(".16x64b").is_ok() {
                    return Ok(Shape1::_16x64b);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try _16x128b
            {
                let saved_pos = stream.position();
                if stream.expect_string(".16x128b").is_ok() {
                    return Ok(Shape1::_16x128b);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _16x256b
            {
                let saved_pos = stream.position();
                if stream.expect_string(".16x256b").is_ok() {
                    return Ok(Shape1::_16x256b);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _32x32b
            {
                let saved_pos = stream.position();
                if stream.expect_string(".32x32b").is_ok() {
                    return Ok(Shape1::_32x32b);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".16x64b", ".16x128b", ".16x256b", ".32x32b"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Unpack {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Unpack16b
            {
                let saved_pos = stream.position();
                if stream.expect_string(".unpack::16b").is_ok() {
                    return Ok(Unpack::Unpack16b);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".unpack::16b"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Num {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try X1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".x1").is_ok() {
                    return Ok(Num::X1);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try X2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".x2").is_ok() {
                    return Ok(Num::X2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try X4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".x4").is_ok() {
                    return Ok(Num::X4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try X8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".x8").is_ok() {
                    return Ok(Num::X8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try X16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".x16").is_ok() {
                    return Ok(Num::X16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try X32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".x32").is_ok() {
                    return Ok(Num::X32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try X64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".x64").is_ok() {
                    return Ok(Num::X64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try X128
            {
                let saved_pos = stream.position();
                if stream.expect_string(".x128").is_ok() {
                    return Ok(Num::X128);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".x1", ".x2", ".x4", ".x8", ".x16", ".x32", ".x64", ".x128"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Tcgen05StSyncAlignedShape1NumUnpackB32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".st")?;
            let st = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let shape1 = Shape1::parse(stream)?;
            let num = Num::parse(stream)?;
            let saved_pos = stream.position();
            let unpack = match Unpack::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_string(".b32")?;
            let b32 = ();
            let taddr = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let r = Operand::parse(stream)?;
            Ok(Tcgen05StSyncAlignedShape1NumUnpackB32 {
                st,
                sync,
                aligned,
                shape1,
                num,
                unpack,
                b32,
                taddr,
                r,
            })
        }
    }


    impl PtxParser for Tcgen05StSyncAlignedShape2NumUnpackB32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".st")?;
            let st = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let shape2 = Shape2::parse(stream)?;
            let num = Num::parse(stream)?;
            let saved_pos = stream.position();
            let unpack = match Unpack::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_string(".b32")?;
            let b32 = ();
            let taddr = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let immhalfsplitoff = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let r = Operand::parse(stream)?;
            Ok(Tcgen05StSyncAlignedShape2NumUnpackB32 {
                st,
                sync,
                aligned,
                shape2,
                num,
                unpack,
                b32,
                taddr,
                immhalfsplitoff,
                r,
            })
        }
    }


}

