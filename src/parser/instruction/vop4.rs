//! Original PTX specification:
//!
//! // SIMD instruction with secondary SIMD merge operation
//! vop4.dtype.atype.btype{.sat}  d{.mask}, a{.asel}, b{.bsel}, c;
//! // SIMD instruction with secondary accumulate operation
//! vop4.dtype.atype.btype.add  d{.mask}, a{.asel}, b{.bsel}, c;
//! vop4  = { vadd4, vsub4, vavrg4, vabsdiff4, vmin4, vmax4 };
//! .dtype = .atype = .btype = { .u32, .s32 };
//! .mask  = { .b0,
//! .b1, .b10
//! .b2, .b20, .b21, .b210,
//! .b3, .b30, .b31, .b310, .b32, .b320, .b321, .b3210 };
//! // defaults to .b3210
//! .asel = .bsel = { .b.n.n.n.n }
//! n = { 0, 1, 2, 3, 4, 5, 6, 7}
//! // .asel defaults to .b3210
//! // .bsel defaults to .b7654

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::vop4::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Btype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u32").is_ok() {
                    return Ok(Btype::U32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Btype::S32);
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

    impl PtxParser for Dtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u32").is_ok() {
                    return Ok(Dtype::U32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Dtype::S32);
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

    impl PtxParser for Asel {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try _0
            {
                let saved_pos = stream.position();
                if stream.expect_string("0").is_ok() {
                    return Ok(Asel::_0);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try _1
            {
                let saved_pos = stream.position();
                if stream.expect_string("1").is_ok() {
                    return Ok(Asel::_1);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _2
            {
                let saved_pos = stream.position();
                if stream.expect_string("2").is_ok() {
                    return Ok(Asel::_2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _3
            {
                let saved_pos = stream.position();
                if stream.expect_string("3").is_ok() {
                    return Ok(Asel::_3);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _4
            {
                let saved_pos = stream.position();
                if stream.expect_string("4").is_ok() {
                    return Ok(Asel::_4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _5
            {
                let saved_pos = stream.position();
                if stream.expect_string("5").is_ok() {
                    return Ok(Asel::_5);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _6
            {
                let saved_pos = stream.position();
                if stream.expect_string("6").is_ok() {
                    return Ok(Asel::_6);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _7
            {
                let saved_pos = stream.position();
                if stream.expect_string("7").is_ok() {
                    return Ok(Asel::_7);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &["0", "1", "2", "3", "4", "5", "6", "7"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Bsel {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try _0
            {
                let saved_pos = stream.position();
                if stream.expect_string("0").is_ok() {
                    return Ok(Bsel::_0);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try _1
            {
                let saved_pos = stream.position();
                if stream.expect_string("1").is_ok() {
                    return Ok(Bsel::_1);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _2
            {
                let saved_pos = stream.position();
                if stream.expect_string("2").is_ok() {
                    return Ok(Bsel::_2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _3
            {
                let saved_pos = stream.position();
                if stream.expect_string("3").is_ok() {
                    return Ok(Bsel::_3);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _4
            {
                let saved_pos = stream.position();
                if stream.expect_string("4").is_ok() {
                    return Ok(Bsel::_4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _5
            {
                let saved_pos = stream.position();
                if stream.expect_string("5").is_ok() {
                    return Ok(Bsel::_5);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _6
            {
                let saved_pos = stream.position();
                if stream.expect_string("6").is_ok() {
                    return Ok(Bsel::_6);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _7
            {
                let saved_pos = stream.position();
                if stream.expect_string("7").is_ok() {
                    return Ok(Bsel::_7);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &["0", "1", "2", "3", "4", "5", "6", "7"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u32").is_ok() {
                    return Ok(Atype::U32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Atype::S32);
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

    impl PtxParser for Mask {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B0
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b0").is_ok() {
                    return Ok(Mask::B0);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b1").is_ok() {
                    return Ok(Mask::B1);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B10B2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b10.b2").is_ok() {
                    return Ok(Mask::B10B2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B20
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b20").is_ok() {
                    return Ok(Mask::B20);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B21
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b21").is_ok() {
                    return Ok(Mask::B21);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B210
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b210").is_ok() {
                    return Ok(Mask::B210);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B3
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b3").is_ok() {
                    return Ok(Mask::B3);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B30
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b30").is_ok() {
                    return Ok(Mask::B30);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B31
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b31").is_ok() {
                    return Ok(Mask::B31);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B310
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b310").is_ok() {
                    return Ok(Mask::B310);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b32").is_ok() {
                    return Ok(Mask::B32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B320
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b320").is_ok() {
                    return Ok(Mask::B320);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B321
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b321").is_ok() {
                    return Ok(Mask::B321);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B3210
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b3210").is_ok() {
                    return Ok(Mask::B3210);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b0", ".b1", ".b10.b2", ".b20", ".b21", ".b210", ".b3", ".b30", ".b31", ".b310", ".b32", ".b320", ".b321", ".b3210"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Vop4DtypeAtypeBtypeSat {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vop4")?;
            let dtype = Dtype::parse(stream)?;
            let atype = Atype::parse(stream)?;
            let btype = Btype::parse(stream)?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            let d = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let mask = match Mask::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let asel = match Asel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let bsel = match Bsel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            Ok(Vop4DtypeAtypeBtypeSat {
                dtype,
                atype,
                btype,
                sat,
                d,
                mask,
                a,
                asel,
                b,
                bsel,
                c,
            })
        }
    }


    impl PtxParser for Vop4DtypeAtypeBtypeAdd {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vop4")?;
            let dtype = Dtype::parse(stream)?;
            let atype = Atype::parse(stream)?;
            let btype = Btype::parse(stream)?;
            stream.expect_string(".add")?;
            let add = ();
            let d = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let mask = match Mask::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let asel = match Asel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let bsel = match Bsel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            Ok(Vop4DtypeAtypeBtypeAdd {
                dtype,
                atype,
                btype,
                add,
                d,
                mask,
                a,
                asel,
                b,
                bsel,
                c,
            })
        }
    }


}

