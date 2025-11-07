//! Original PTX specification:
//!
//! // 32-bit scalar operation, with optional secondary operation
//! vop.dtype.atype.u32{.sat}.mode       d, a{.asel}, b{.bsel};
//! vop.dtype.atype.u32{.sat}.mode.op2   d, a{.asel}, b{.bsel}, c;
//! // 32-bit scalar operation, with optional data merge
//! vop.dtype.atype.u32{.sat}.mode  d.dsel, a{.asel}, b{.bsel}, c;
//! vop   = { vshl, vshr };
//! .dtype = .atype = { .u32, .s32 };
//! .mode  = { .clamp, .wrap };
//! .dsel  = .asel  = .bsel  = { .b0, .b1, .b2, .b3, .h0, .h1 };
//! .op2   = { .add, .min, .max };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::vsh::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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

    impl PtxParser for Dsel {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B0
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b0").is_ok() {
                    return Ok(Dsel::B0);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b1").is_ok() {
                    return Ok(Dsel::B1);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b2").is_ok() {
                    return Ok(Dsel::B2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B3
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b3").is_ok() {
                    return Ok(Dsel::B3);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H0
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h0").is_ok() {
                    return Ok(Dsel::H0);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try H1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".h1").is_ok() {
                    return Ok(Dsel::H1);
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

    impl PtxParser for Mode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Clamp
            {
                let saved_pos = stream.position();
                if stream.expect_string(".clamp").is_ok() {
                    return Ok(Mode::Clamp);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Wrap
            {
                let saved_pos = stream.position();
                if stream.expect_string(".wrap").is_ok() {
                    return Ok(Mode::Wrap);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".clamp", ".wrap"];
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

    impl PtxParser for VshlDtypeAtypeU32SatMode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vshl")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".u32")?;
            let u32 = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let mode = Mode::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
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
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(VshlDtypeAtypeU32SatMode {
                dtype,
                atype,
                u32,
                sat,
                mode,
                d,
                a,
                asel,
                b,
                bsel,
            })
        }
    }


    impl PtxParser for VshrDtypeAtypeU32SatMode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vshr")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".u32")?;
            let u32 = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let mode = Mode::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
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
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(VshrDtypeAtypeU32SatMode {
                dtype,
                atype,
                u32,
                sat,
                mode,
                d,
                a,
                asel,
                b,
                bsel,
            })
        }
    }


    impl PtxParser for VshlDtypeAtypeU32SatModeOp2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vshl")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".u32")?;
            let u32 = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let mode = Mode::parse(stream)?;
            stream.expect_complete()?;
            let op2 = Op2::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
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
            Ok(VshlDtypeAtypeU32SatModeOp2 {
                dtype,
                atype,
                u32,
                sat,
                mode,
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


    impl PtxParser for VshrDtypeAtypeU32SatModeOp2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vshr")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".u32")?;
            let u32 = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let mode = Mode::parse(stream)?;
            stream.expect_complete()?;
            let op2 = Op2::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
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
            Ok(VshrDtypeAtypeU32SatModeOp2 {
                dtype,
                atype,
                u32,
                sat,
                mode,
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


    impl PtxParser for VshlDtypeAtypeU32SatMode1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vshl")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".u32")?;
            let u32 = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let mode = Mode::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            let dsel = Dsel::parse(stream)?;
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
            Ok(VshlDtypeAtypeU32SatMode1 {
                dtype,
                atype,
                u32,
                sat,
                mode,
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


    impl PtxParser for VshrDtypeAtypeU32SatMode1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("vshr")?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".u32")?;
            let u32 = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let mode = Mode::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            let dsel = Dsel::parse(stream)?;
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
            Ok(VshrDtypeAtypeU32SatMode1 {
                dtype,
                atype,
                u32,
                sat,
                mode,
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

