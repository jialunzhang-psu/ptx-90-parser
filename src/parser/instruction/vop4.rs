//! Original PTX specification:
//!
//! // SIMD instruction with secondary SIMD merge operation
//! vop4.dtype.atype.btype{.sat}  d{.mask}, a{.asel}, b{.bsel}, c;
//! // SIMD instruction with secondary accumulate operation
//! vop4.dtype.atype.btype.add  d{.mask}, a{.asel}, b{.bsel}, c;
//! vop4  = { vadd4, vsub4, vavrg4, vabsdiff4, vmin4, vmax4 };
//! .dtype = .atype = .btype = { .u32, .s32 };
//! .mask  = { .b0,
//! .b1, .b10,
//! .b2, .b20, .b21, .b210,
//! .b3, .b30, .b31, .b310, .b32, .b320, .b321, .b3210 };
//! // defaults to .b3210
//! .asel = .bsel = { .b.n.n.n.n };
//! .n = { 0, 1, 2, 3, 4, 5, 6, 7};
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

    impl PtxParser for Asel {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try BNNNN
            {
                let saved_seq_pos = stream.position();
                match (|| -> Result<_, PtxParseError> {
            stream.expect_string(".b")?;
            let b = ();
            let n = N::parse(stream)?;
            let n1 = N::parse(stream)?;
            let n2 = N::parse(stream)?;
            let n3 = N::parse(stream)?;
                    Ok((b, n, n1, n2, n3))
                })() {
                    Ok((b, n, n1, n2, n3)) => {
                        return Ok(Asel::BNNNN(b, n, n1, n2, n3));
                    }
                    Err(_) => {
                        stream.set_position(saved_seq_pos);
                    }
                }
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &["<complex>"];
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
            // Try BNNNN
            {
                let saved_seq_pos = stream.position();
                match (|| -> Result<_, PtxParseError> {
            stream.expect_string(".b")?;
            let b = ();
            let n = N::parse(stream)?;
            let n1 = N::parse(stream)?;
            let n2 = N::parse(stream)?;
            let n3 = N::parse(stream)?;
                    Ok((b, n, n1, n2, n3))
                })() {
                    Ok((b, n, n1, n2, n3)) => {
                        return Ok(Bsel::BNNNN(b, n, n1, n2, n3));
                    }
                    Err(_) => {
                        stream.set_position(saved_seq_pos);
                    }
                }
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &["<complex>"];
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

    impl PtxParser for Mask {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B3210
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b3210").is_ok() {
                    return Ok(Mask::B3210);
                }
                stream.set_position(saved_pos);
            }
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
            // Try B10
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b10").is_ok() {
                    return Ok(Mask::B10);
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
            // Try B0
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b0").is_ok() {
                    return Ok(Mask::B0);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
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
            // Try B2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b2").is_ok() {
                    return Ok(Mask::B2);
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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b3210", ".b210", ".b310", ".b320", ".b321", ".b10", ".b20", ".b21", ".b30", ".b31", ".b32", ".b0", ".b1", ".b2", ".b3"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for N {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try _0
            {
                let saved_pos = stream.position();
                if stream.expect_string("0").is_ok() {
                    return Ok(N::_0);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try _1
            {
                let saved_pos = stream.position();
                if stream.expect_string("1").is_ok() {
                    return Ok(N::_1);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _2
            {
                let saved_pos = stream.position();
                if stream.expect_string("2").is_ok() {
                    return Ok(N::_2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _3
            {
                let saved_pos = stream.position();
                if stream.expect_string("3").is_ok() {
                    return Ok(N::_3);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _4
            {
                let saved_pos = stream.position();
                if stream.expect_string("4").is_ok() {
                    return Ok(N::_4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _5
            {
                let saved_pos = stream.position();
                if stream.expect_string("5").is_ok() {
                    return Ok(N::_5);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _6
            {
                let saved_pos = stream.position();
                if stream.expect_string("6").is_ok() {
                    return Ok(N::_6);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _7
            {
                let saved_pos = stream.position();
                if stream.expect_string("7").is_ok() {
                    return Ok(N::_7);
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

    impl PtxParser for Vadd4DtypeAtypeBtypeSat {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vadd4")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let mask = match Mask::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let asel = match Asel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let bsel = match Bsel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Vadd4DtypeAtypeBtypeSat {
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


    impl PtxParser for Vsub4DtypeAtypeBtypeSat {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vsub4")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let mask = match Mask::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let asel = match Asel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let bsel = match Bsel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Vsub4DtypeAtypeBtypeSat {
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


    impl PtxParser for Vavrg4DtypeAtypeBtypeSat {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vavrg4")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let mask = match Mask::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let asel = match Asel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let bsel = match Bsel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Vavrg4DtypeAtypeBtypeSat {
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


    impl PtxParser for Vabsdiff4DtypeAtypeBtypeSat {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vabsdiff4")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let mask = match Mask::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let asel = match Asel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let bsel = match Bsel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Vabsdiff4DtypeAtypeBtypeSat {
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


    impl PtxParser for Vmin4DtypeAtypeBtypeSat {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vmin4")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let mask = match Mask::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let asel = match Asel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let bsel = match Bsel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Vmin4DtypeAtypeBtypeSat {
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


    impl PtxParser for Vmax4DtypeAtypeBtypeSat {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vmax4")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let mask = match Mask::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let asel = match Asel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let bsel = match Bsel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Vmax4DtypeAtypeBtypeSat {
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


    impl PtxParser for Vadd4DtypeAtypeBtypeAdd {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vadd4")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".add")?;
            let add = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let mask = match Mask::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let asel = match Asel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let bsel = match Bsel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Vadd4DtypeAtypeBtypeAdd {
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


    impl PtxParser for Vsub4DtypeAtypeBtypeAdd {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vsub4")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".add")?;
            let add = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let mask = match Mask::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let asel = match Asel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let bsel = match Bsel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Vsub4DtypeAtypeBtypeAdd {
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


    impl PtxParser for Vavrg4DtypeAtypeBtypeAdd {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vavrg4")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".add")?;
            let add = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let mask = match Mask::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let asel = match Asel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let bsel = match Bsel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Vavrg4DtypeAtypeBtypeAdd {
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


    impl PtxParser for Vabsdiff4DtypeAtypeBtypeAdd {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vabsdiff4")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".add")?;
            let add = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let mask = match Mask::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let asel = match Asel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let bsel = match Bsel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Vabsdiff4DtypeAtypeBtypeAdd {
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


    impl PtxParser for Vmin4DtypeAtypeBtypeAdd {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vmin4")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".add")?;
            let add = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let mask = match Mask::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let asel = match Asel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let bsel = match Bsel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Vmin4DtypeAtypeBtypeAdd {
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


    impl PtxParser for Vmax4DtypeAtypeBtypeAdd {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vmax4")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".add")?;
            let add = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let mask = match Mask::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let asel = match Asel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let bsel = match Bsel::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Vmax4DtypeAtypeBtypeAdd {
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

