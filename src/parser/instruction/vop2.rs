//! Original PTX specification:
//!
//! // SIMD instruction with secondary SIMD merge operation
//! vop2.dtype.atype.btype{.sat}  d{.mask}, a{.asel}, b{.bsel}, c;
//! // SIMD instruction with secondary accumulate operation
//! vop2.dtype.atype.btype.add  d{.mask}, a{.asel}, b{.bsel}, c;
//! vop2  = { vadd2, vsub2, vavrg2, vabsdiff2, vmin2, vmax2 };
//! .dtype = .atype = .btype = { .u32, .s32 };
//! .mask  = { .h0, .h1, .h10 };  // defaults to .h10
//! .asel  = .bsel  = { .h00, .h01, .h02, .h03, .h10, .h11, .h12, .h13, .h20, .h21, .h22, .h23, .h30, .h31, .h32, .h33 }; 
//! // .asel defaults to .h10
//! // .bsel defaults to .h32

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::vop2::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Mask {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try H0
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h0").is_ok() {
                    return Ok(Mask::H0);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try H1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h1").is_ok() {
                    return Ok(Mask::H1);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H10
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h10").is_ok() {
                    return Ok(Mask::H10);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".h0", ".h1", ".h10"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Asel {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try H00
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h00").is_ok() {
                    return Ok(Asel::H00);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try H01
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h01").is_ok() {
                    return Ok(Asel::H01);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H02
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h02").is_ok() {
                    return Ok(Asel::H02);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H03
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h03").is_ok() {
                    return Ok(Asel::H03);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H10
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h10").is_ok() {
                    return Ok(Asel::H10);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H11
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h11").is_ok() {
                    return Ok(Asel::H11);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H12
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h12").is_ok() {
                    return Ok(Asel::H12);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H13
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h13").is_ok() {
                    return Ok(Asel::H13);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H20
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h20").is_ok() {
                    return Ok(Asel::H20);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H21
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h21").is_ok() {
                    return Ok(Asel::H21);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H22
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h22").is_ok() {
                    return Ok(Asel::H22);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H23
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h23").is_ok() {
                    return Ok(Asel::H23);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H30
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h30").is_ok() {
                    return Ok(Asel::H30);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H31
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h31").is_ok() {
                    return Ok(Asel::H31);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h32").is_ok() {
                    return Ok(Asel::H32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H33
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h33").is_ok() {
                    return Ok(Asel::H33);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".h00", ".h01", ".h02", ".h03", ".h10", ".h11", ".h12", ".h13", ".h20", ".h21", ".h22", ".h23", ".h30", ".h31", ".h32", ".h33"];
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

    impl PtxParser for Bsel {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try H00
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h00").is_ok() {
                    return Ok(Bsel::H00);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try H01
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h01").is_ok() {
                    return Ok(Bsel::H01);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H02
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h02").is_ok() {
                    return Ok(Bsel::H02);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H03
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h03").is_ok() {
                    return Ok(Bsel::H03);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H10
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h10").is_ok() {
                    return Ok(Bsel::H10);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H11
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h11").is_ok() {
                    return Ok(Bsel::H11);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H12
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h12").is_ok() {
                    return Ok(Bsel::H12);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H13
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h13").is_ok() {
                    return Ok(Bsel::H13);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H20
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h20").is_ok() {
                    return Ok(Bsel::H20);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H21
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h21").is_ok() {
                    return Ok(Bsel::H21);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H22
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h22").is_ok() {
                    return Ok(Bsel::H22);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H23
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h23").is_ok() {
                    return Ok(Bsel::H23);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H30
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h30").is_ok() {
                    return Ok(Bsel::H30);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H31
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h31").is_ok() {
                    return Ok(Bsel::H31);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h32").is_ok() {
                    return Ok(Bsel::H32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H33
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h33").is_ok() {
                    return Ok(Bsel::H33);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".h00", ".h01", ".h02", ".h03", ".h10", ".h11", ".h12", ".h13", ".h20", ".h21", ".h22", ".h23", ".h30", ".h31", ".h32", ".h33"];
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

    impl PtxParser for Vop2DtypeAtypeBtypeSat {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vop2")?;
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
            Ok(Vop2DtypeAtypeBtypeSat {
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


    impl PtxParser for Vop2DtypeAtypeBtypeAdd {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vop2")?;
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
            Ok(Vop2DtypeAtypeBtypeAdd {
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

