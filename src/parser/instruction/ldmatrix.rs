//! Original PTX specification:
//!
//! ldmatrix.sync.aligned.shape.num{.trans}{.ss}.type r, [p];
//! ldmatrix.sync.aligned.m8n16.num{.ss}.dst_fmt.src_fmt        r, [p];
//! ldmatrix.sync.aligned.m16n16.num.trans{.ss}.dst_fmt.src_fmt r, [p];
//! .shape   = {.m8n8, .m16n16};
//! .num     = {.x1, .x2, .x4};
//! .ss      = {.shared, .shared::cta};
//! .type    = {.b16, .b8};
//! .dst_fmt = { .b8x16 };
//! .src_fmt = { .b6x16_p32, .b4x16_p64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::ldmatrix::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M8n8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m8n8").is_ok() {
                    return Ok(Shape::M8n8);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try M16n16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m16n16").is_ok() {
                    return Ok(Shape::M16n16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m8n8", ".m16n16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b16").is_ok() {
                    return Ok(Type::B16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b8").is_ok() {
                    return Ok(Type::B8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b16", ".b8"];
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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".x1", ".x2", ".x4"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Ss {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Shared
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared").is_ok() {
                    return Ok(Ss::Shared);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try SharedCta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared::cta").is_ok() {
                    return Ok(Ss::SharedCta);
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

    impl PtxParser for DstFmt {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B8x16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b8x16").is_ok() {
                    return Ok(DstFmt::B8x16);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b8x16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for SrcFmt {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B6x16P32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b6x16_p32").is_ok() {
                    return Ok(SrcFmt::B6x16P32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B4x16P64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b4x16_p64").is_ok() {
                    return Ok(SrcFmt::B4x16P64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b6x16_p32", ".b4x16_p64"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for LdmatrixSyncAlignedShapeNumTransSsType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("ldmatrix")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let shape = Shape::parse(stream)?;
            let num = Num::parse(stream)?;
            let saved_pos = stream.position();
            let trans = stream.expect_string(".trans").is_ok();
            if !trans {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let type_ = Type::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            Ok(LdmatrixSyncAlignedShapeNumTransSsType {
                sync,
                aligned,
                shape,
                num,
                trans,
                ss,
                type_,
                r,
                p,
            })
        }
    }


    impl PtxParser for LdmatrixSyncAlignedM8n16NumSsDstFmtSrcFmt {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("ldmatrix")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_string(".m8n16")?;
            let m8n16 = ();
            let num = Num::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let dst_fmt = DstFmt::parse(stream)?;
            let src_fmt = SrcFmt::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            Ok(LdmatrixSyncAlignedM8n16NumSsDstFmtSrcFmt {
                sync,
                aligned,
                m8n16,
                num,
                ss,
                dst_fmt,
                src_fmt,
                r,
                p,
            })
        }
    }


    impl PtxParser for LdmatrixSyncAlignedM16n16NumTransSsDstFmtSrcFmt {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("ldmatrix")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_string(".m16n16")?;
            let m16n16 = ();
            let num = Num::parse(stream)?;
            stream.expect_string(".trans")?;
            let trans = ();
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let dst_fmt = DstFmt::parse(stream)?;
            let src_fmt = SrcFmt::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            Ok(LdmatrixSyncAlignedM16n16NumTransSsDstFmtSrcFmt {
                sync,
                aligned,
                m16n16,
                num,
                trans,
                ss,
                dst_fmt,
                src_fmt,
                r,
                p,
            })
        }
    }


}

