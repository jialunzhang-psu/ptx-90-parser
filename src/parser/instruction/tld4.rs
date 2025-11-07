//! Original PTX specification:
//!
//! tld4.comp.2d.v4.dtype.f32    d{|p}, [a, c] {, e} {, f};
//! tld4.comp.geom.v4.dtype.f32  d{|p}, [a, b, c] {, e} {, f};  // explicit sampler
//! .comp  = { .r, .g, .b, .a };
//! .geom  = { .2d, .a2d, .cube, .acube };
//! .dtype = { .u32, .s32, .f32 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tld4::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Comp {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try R
            {
                let saved_pos = stream.position();
                if stream.expect_string(".r").is_ok() {
                    return Ok(Comp::R);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try G
            {
                let saved_pos = stream.position();
                if stream.expect_string(".g").is_ok() {
                    return Ok(Comp::G);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b").is_ok() {
                    return Ok(Comp::B);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try A
            {
                let saved_pos = stream.position();
                if stream.expect_string(".a").is_ok() {
                    return Ok(Comp::A);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".r", ".g", ".b", ".a"];
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
            let saved_pos = stream.position();
            // Try F32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f32").is_ok() {
                    return Ok(Dtype::F32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u32", ".s32", ".f32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Geom {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Acube
            {
                let saved_pos = stream.position();
                if stream.expect_string(".acube").is_ok() {
                    return Ok(Geom::Acube);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Cube
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cube").is_ok() {
                    return Ok(Geom::Cube);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try A2d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".a2d").is_ok() {
                    return Ok(Geom::A2d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _2d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".2d").is_ok() {
                    return Ok(Geom::_2d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".acube", ".cube", ".a2d", ".2d"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Tld4Comp2dV4DtypeF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tld4")?;
            let comp = Comp::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".2d")?;
            let _2d = ();
            stream.expect_complete()?;
            stream.expect_string(".v4")?;
            let v4 = ();
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let p = if stream.consume_if(|t| matches!(t, PtxToken::Pipe)).is_some() {
                Some(GeneralOperand::parse(stream)?)
            } else {
                stream.set_position(saved_pos);
                None
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = TexHandler2::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let e = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let f = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Tld4Comp2dV4DtypeF32 {
                comp,
                _2d,
                v4,
                dtype,
                f32,
                d,
                p,
                a,
                e,
                f,
            })
        }
    }


    impl PtxParser for Tld4CompGeomV4DtypeF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tld4")?;
            let comp = Comp::parse(stream)?;
            stream.expect_complete()?;
            let geom = Geom::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".v4")?;
            let v4 = ();
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let p = if stream.consume_if(|t| matches!(t, PtxToken::Pipe)).is_some() {
                Some(GeneralOperand::parse(stream)?)
            } else {
                stream.set_position(saved_pos);
                None
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = TexHandler3::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let e = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let f = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Tld4CompGeomV4DtypeF32 {
                comp,
                geom,
                v4,
                dtype,
                f32,
                d,
                p,
                a,
                e,
                f,
            })
        }
    }


}

