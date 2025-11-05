//! Original PTX specification:
//!
//! // Base load instruction:
//! tcgen05.ld.sync.aligned.shape1.num{.pack}.b32    r, [taddr];
//! tcgen05.ld.sync.aligned.shape2.num{.pack}.b32    r, [taddr], immHalfSplitoff;
//! .shape1 = { .16x64b, .16x128b, .16x256b, .32x32b };
//! .shape2 = { .16x32bx2 };
//! .num    = { .x1, .x2, .x4, .x8, .x16, .x32, .x64, .x128 };
//! .pack   = { .pack::16b };
//! // Floating point type load along with reduction :
//! tcgen05.ld.red.sync.aligned.shape3.num.redOp{.abs}{.NaN}.f32 r, redval, [taddr];
//! tcgen05.ld.red.sync.aligned.shape4.num.redOp{.abs}{.NaN}.f32 r, redval, [taddr], immHalfSplitoff;
//! // Integer type load along with reduction :
//! tcgen05.ld.red.sync.aligned.shape3.num.redOp.type r, redval, [taddr];
//! tcgen05.ld.red.sync.aligned.shape4.num.redOp.type r, redval, [taddr], immHalfSplitoff;
//! .shape3 = { .32x32b   };
//! .shape4 = { .16x32bx2 };
//! .redOp  = { .min, .max };
//! .type   = { .u32, .s32 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_ld::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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

    impl PtxParser for Pack {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Pack16b
            {
                let saved_pos = stream.position();
                if stream.expect_string(".pack::16b").is_ok() {
                    return Ok(Pack::Pack16b);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".pack::16b"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape4 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try _16x32bx2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".16x32bx2").is_ok() {
                    return Ok(Shape4::_16x32bx2);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".16x32bx2"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape3 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try _32x32b
            {
                let saved_pos = stream.position();
                if stream.expect_string(".32x32b").is_ok() {
                    return Ok(Shape3::_32x32b);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".32x32b"];
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

    impl PtxParser for Type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u32").is_ok() {
                    return Ok(Type::U32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Type::S32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u32", ".s32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Redop {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Min
            {
                let saved_pos = stream.position();
                if stream.expect_string(".min").is_ok() {
                    return Ok(Redop::Min);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Max
            {
                let saved_pos = stream.position();
                if stream.expect_string(".max").is_ok() {
                    return Ok(Redop::Max);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".min", ".max"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Tcgen05LdSyncAlignedShape1NumPackB32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".ld")?;
            let ld = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let shape1 = Shape1::parse(stream)?;
            let num = Num::parse(stream)?;
            let saved_pos = stream.position();
            let pack = match Pack::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_string(".b32")?;
            let b32 = ();
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let taddr = AddressOperand::parse(stream)?;
            Ok(Tcgen05LdSyncAlignedShape1NumPackB32 {
                ld,
                sync,
                aligned,
                shape1,
                num,
                pack,
                b32,
                r,
                taddr,
            })
        }
    }


    impl PtxParser for Tcgen05LdSyncAlignedShape2NumPackB32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".ld")?;
            let ld = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let shape2 = Shape2::parse(stream)?;
            let num = Num::parse(stream)?;
            let saved_pos = stream.position();
            let pack = match Pack::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_string(".b32")?;
            let b32 = ();
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let taddr = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let immhalfsplitoff = Operand::parse(stream)?;
            Ok(Tcgen05LdSyncAlignedShape2NumPackB32 {
                ld,
                sync,
                aligned,
                shape2,
                num,
                pack,
                b32,
                r,
                taddr,
                immhalfsplitoff,
            })
        }
    }


    impl PtxParser for Tcgen05LdRedSyncAlignedShape3NumRedopAbsNanF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".ld")?;
            let ld = ();
            stream.expect_string(".red")?;
            let red = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let shape3 = Shape3::parse(stream)?;
            let num = Num::parse(stream)?;
            let redop = Redop::parse(stream)?;
            let saved_pos = stream.position();
            let abs = stream.expect_string(".abs").is_ok();
            if !abs {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let nan = stream.expect_string(".NaN").is_ok();
            if !nan {
                stream.set_position(saved_pos);
            }
            stream.expect_string(".f32")?;
            let f32 = ();
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let redval = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let taddr = AddressOperand::parse(stream)?;
            Ok(Tcgen05LdRedSyncAlignedShape3NumRedopAbsNanF32 {
                ld,
                red,
                sync,
                aligned,
                shape3,
                num,
                redop,
                abs,
                nan,
                f32,
                r,
                redval,
                taddr,
            })
        }
    }


    impl PtxParser for Tcgen05LdRedSyncAlignedShape4NumRedopAbsNanF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".ld")?;
            let ld = ();
            stream.expect_string(".red")?;
            let red = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let shape4 = Shape4::parse(stream)?;
            let num = Num::parse(stream)?;
            let redop = Redop::parse(stream)?;
            let saved_pos = stream.position();
            let abs = stream.expect_string(".abs").is_ok();
            if !abs {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let nan = stream.expect_string(".NaN").is_ok();
            if !nan {
                stream.set_position(saved_pos);
            }
            stream.expect_string(".f32")?;
            let f32 = ();
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let redval = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let taddr = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let immhalfsplitoff = Operand::parse(stream)?;
            Ok(Tcgen05LdRedSyncAlignedShape4NumRedopAbsNanF32 {
                ld,
                red,
                sync,
                aligned,
                shape4,
                num,
                redop,
                abs,
                nan,
                f32,
                r,
                redval,
                taddr,
                immhalfsplitoff,
            })
        }
    }


    impl PtxParser for Tcgen05LdRedSyncAlignedShape3NumRedopType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".ld")?;
            let ld = ();
            stream.expect_string(".red")?;
            let red = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let shape3 = Shape3::parse(stream)?;
            let num = Num::parse(stream)?;
            let redop = Redop::parse(stream)?;
            let type_ = Type::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let redval = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let taddr = AddressOperand::parse(stream)?;
            Ok(Tcgen05LdRedSyncAlignedShape3NumRedopType {
                ld,
                red,
                sync,
                aligned,
                shape3,
                num,
                redop,
                type_,
                r,
                redval,
                taddr,
            })
        }
    }


    impl PtxParser for Tcgen05LdRedSyncAlignedShape4NumRedopType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".ld")?;
            let ld = ();
            stream.expect_string(".red")?;
            let red = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let shape4 = Shape4::parse(stream)?;
            let num = Num::parse(stream)?;
            let redop = Redop::parse(stream)?;
            let type_ = Type::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let redval = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let taddr = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let immhalfsplitoff = Operand::parse(stream)?;
            Ok(Tcgen05LdRedSyncAlignedShape4NumRedopType {
                ld,
                red,
                sync,
                aligned,
                shape4,
                num,
                redop,
                type_,
                r,
                redval,
                taddr,
                immhalfsplitoff,
            })
        }
    }


}

