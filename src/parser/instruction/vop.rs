//! Original PTX specification:
//!
//! // 32-bit scalar operation, with optional secondary operation
//! vop.dtype.atype.btype{.sat}       d, a{.asel}, b{.bsel};
//! vop.dtype.atype.btype{.sat}.op2   d, a{.asel}, b{.bsel}, c;
//! // 32-bit scalar operation, with optional data merge
//! vop.dtype.atype.btype{.sat}  d.dsel, a{.asel}, b{.bsel}, c;
//! vop   = { vadd, vsub, vabsdiff, vmin, vmax };
//! .dtype = .atype = .btype = { .u32, .s32 };
//! .dsel  = .asel  = .bsel  = { .b0, .b1, .b2, .b3, .h0, .h1 };
//! .op2   = { .add, .min, .max };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::vop::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Dsel {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            let start_pos = stream.position();
            if stream.expect_string(".b0").is_ok() {
                return Ok(Dsel::B0);
            }
            stream.set_position(start_pos);
            if stream.expect_string(".b1").is_ok() {
                return Ok(Dsel::B1);
            }
            stream.set_position(start_pos);
            if stream.expect_string(".b2").is_ok() {
                return Ok(Dsel::B2);
            }
            stream.set_position(start_pos);
            if stream.expect_string(".b3").is_ok() {
                return Ok(Dsel::B3);
            }
            stream.set_position(start_pos);
            if stream.expect_string(".h0").is_ok() {
                return Ok(Dsel::H0);
            }
            stream.set_position(start_pos);
            if stream.expect_string(".h1").is_ok() {
                return Ok(Dsel::H1);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(0..0);
            let expected = &[".b0", ".b1", ".b2", ".b3", ".h0", ".h1"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

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

    impl PtxParser for Asel {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B0
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b0").is_ok() {
                    return Ok(Asel::B0);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b1").is_ok() {
                    return Ok(Asel::B1);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b2").is_ok() {
                    return Ok(Asel::B2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B3
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b3").is_ok() {
                    return Ok(Asel::B3);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H0
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h0").is_ok() {
                    return Ok(Asel::H0);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h1").is_ok() {
                    return Ok(Asel::H1);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b0", ".b1", ".b2", ".b3", ".h0", ".h1"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Bsel {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B0
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b0").is_ok() {
                    return Ok(Bsel::B0);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b1").is_ok() {
                    return Ok(Bsel::B1);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b2").is_ok() {
                    return Ok(Bsel::B2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B3
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b3").is_ok() {
                    return Ok(Bsel::B3);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H0
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h0").is_ok() {
                    return Ok(Bsel::H0);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h1").is_ok() {
                    return Ok(Bsel::H1);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b0", ".b1", ".b2", ".b3", ".h0", ".h1"];
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

    impl PtxParser for Op2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Add
            {
                let saved_pos = stream.position();
                if stream.expect_string(".add").is_ok() {
                    return Ok(Op2::Add);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Min
            {
                let saved_pos = stream.position();
                if stream.expect_string(".min").is_ok() {
                    return Ok(Op2::Min);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Max
            {
                let saved_pos = stream.position();
                if stream.expect_string(".max").is_ok() {
                    return Ok(Op2::Max);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".add", ".min", ".max"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for VopDtypeAtypeBtypeSat {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vop")?;
            let dtype = Dtype::parse(stream)?;
            let atype = Atype::parse(stream)?;
            let btype = Btype::parse(stream)?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            let d = Operand::parse(stream)?;
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
            Ok(VopDtypeAtypeBtypeSat {
                dtype,
                atype,
                btype,
                sat,
                d,
                a,
                asel,
                b,
                bsel,
            })
        }
    }


    impl PtxParser for VopDtypeAtypeBtypeSatOp2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vop")?;
            let dtype = Dtype::parse(stream)?;
            let atype = Atype::parse(stream)?;
            let btype = Btype::parse(stream)?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            let op2 = Op2::parse(stream)?;
            let d = Operand::parse(stream)?;
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
            Ok(VopDtypeAtypeBtypeSatOp2 {
                dtype,
                atype,
                btype,
                sat,
                op2,
                d,
                a,
                asel,
                b,
                bsel,
                c,
            })
        }
    }


    impl PtxParser for VopDtypeAtypeBtypeSat1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vop")?;
            let dtype = Dtype::parse(stream)?;
            let atype = Atype::parse(stream)?;
            let btype = Btype::parse(stream)?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let dsel = Dsel::parse(stream)?;
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
            Ok(VopDtypeAtypeBtypeSat1 {
                dtype,
                atype,
                btype,
                sat,
                d,
                dsel,
                a,
                asel,
                b,
                bsel,
                c,
            })
        }
    }


}

