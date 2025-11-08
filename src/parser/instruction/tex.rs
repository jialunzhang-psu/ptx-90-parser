//! Original PTX specification:
//!
//! tex.geom.v4.dtype.ctype  d{|p}, [a, c] {, e} {, f};
//! tex.geom.v4.dtype.ctype  d{|p}, [a, b, c] {, e} {, f};  // explicit sampler
//! tex.geom.v2.f16x2.ctype  d{|p}, [a, c] {, e} {, f};
//! tex.geom.v2.f16x2.ctype  d{|p}, [a, b, c] {, e} {, f};  // explicit sampler
//! // mipmaps
//! tex.base.geom.v4.dtype.ctype   d{|p}, [a, {b,} c] {, e} {, f};
//! tex.level.geom.v4.dtype.ctype  d{|p}, [a, {b,} c], lod {, e} {, f};
//! tex.grad.geom.v4.dtype.ctype   d{|p}, [a, {b,} c], dPdx, dPdy {, e} {, f};
//! tex.base.geom.v2.f16x2.ctype   d{|p}, [a, {b,} c] {, e} {, f};
//! tex.level.geom.v2.f16x2.ctype  d{|p}, [a, {b,} c], lod {, e} {, f};
//! tex.grad.geom.v2.f16x2.ctype   d{|p}, [a, {b,} c], dPdx, dPdy {, e} {, f};
//! .geom  = { .1d, .2d, .3d, .a1d, .a2d, .cube, .acube, .2dms, .a2dms };
//! .dtype = { .u32, .s32, .f16,  .f32 };
//! .ctype = {       .s32, .f32 };          // .cube, .acube require .f32
//! // .2dms, .a2dms require .s32

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tex::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Ctype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Ctype::S32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try F32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f32").is_ok() {
                    return Ok(Ctype::F32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".s32", ".f32"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
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
            // Try F16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16").is_ok() {
                    return Ok(Dtype::F16);
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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u32", ".s32", ".f16", ".f32"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
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
            // Try A2dms
            {
                let saved_pos = stream.position();
                if stream.expect_string(".a2dms").is_ok() {
                    return Ok(Geom::A2dms);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
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
            // Try _2dms
            {
                let saved_pos = stream.position();
                if stream.expect_string(".2dms").is_ok() {
                    return Ok(Geom::_2dms);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try A1d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".a1d").is_ok() {
                    return Ok(Geom::A1d);
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
            // Try _1d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".1d").is_ok() {
                    return Ok(Geom::_1d);
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
            let saved_pos = stream.position();
            // Try _3d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".3d").is_ok() {
                    return Ok(Geom::_3d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[
                ".acube", ".a2dms", ".cube", ".2dms", ".a1d", ".a2d", ".1d", ".2d", ".3d",
            ];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for TexGeomV4DtypeCtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tex")?;
            let geom = Geom::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".v4")?;
            let v4 = ();
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
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
            Ok(TexGeomV4DtypeCtype {
                geom,
                v4,
                dtype,
                ctype,
                d,
                p,
                a,
                e,
                f,
            })
        }
    }

    impl PtxParser for TexGeomV4DtypeCtype1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tex")?;
            let geom = Geom::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".v4")?;
            let v4 = ();
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
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
            Ok(TexGeomV4DtypeCtype1 {
                geom,
                v4,
                dtype,
                ctype,
                d,
                p,
                a,
                e,
                f,
            })
        }
    }

    impl PtxParser for TexGeomV2F16x2Ctype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tex")?;
            let geom = Geom::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".v2")?;
            let v2 = ();
            stream.expect_complete()?;
            stream.expect_string(".f16x2")?;
            let f16x2 = ();
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
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
            Ok(TexGeomV2F16x2Ctype {
                geom,
                v2,
                f16x2,
                ctype,
                d,
                p,
                a,
                e,
                f,
            })
        }
    }

    impl PtxParser for TexGeomV2F16x2Ctype1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tex")?;
            let geom = Geom::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".v2")?;
            let v2 = ();
            stream.expect_complete()?;
            stream.expect_string(".f16x2")?;
            let f16x2 = ();
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
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
            Ok(TexGeomV2F16x2Ctype1 {
                geom,
                v2,
                f16x2,
                ctype,
                d,
                p,
                a,
                e,
                f,
            })
        }
    }

    impl PtxParser for TexBaseGeomV4DtypeCtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tex")?;
            stream.expect_string(".base")?;
            let base = ();
            stream.expect_complete()?;
            let geom = Geom::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".v4")?;
            let v4 = ();
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
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
            let a = TexHandler3Optional::parse(stream)?;
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
            Ok(TexBaseGeomV4DtypeCtype {
                base,
                geom,
                v4,
                dtype,
                ctype,
                d,
                p,
                a,
                e,
                f,
            })
        }
    }

    impl PtxParser for TexLevelGeomV4DtypeCtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tex")?;
            stream.expect_string(".level")?;
            let level = ();
            stream.expect_complete()?;
            let geom = Geom::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".v4")?;
            let v4 = ();
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
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
            let a = TexHandler3Optional::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let lod = GeneralOperand::parse(stream)?;
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
            Ok(TexLevelGeomV4DtypeCtype {
                level,
                geom,
                v4,
                dtype,
                ctype,
                d,
                p,
                a,
                lod,
                e,
                f,
            })
        }
    }

    impl PtxParser for TexGradGeomV4DtypeCtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tex")?;
            stream.expect_string(".grad")?;
            let grad = ();
            stream.expect_complete()?;
            let geom = Geom::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".v4")?;
            let v4 = ();
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
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
            let a = TexHandler3Optional::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let dpdx = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let dpdy = GeneralOperand::parse(stream)?;
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
            Ok(TexGradGeomV4DtypeCtype {
                grad,
                geom,
                v4,
                dtype,
                ctype,
                d,
                p,
                a,
                dpdx,
                dpdy,
                e,
                f,
            })
        }
    }

    impl PtxParser for TexBaseGeomV2F16x2Ctype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tex")?;
            stream.expect_string(".base")?;
            let base = ();
            stream.expect_complete()?;
            let geom = Geom::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".v2")?;
            let v2 = ();
            stream.expect_complete()?;
            stream.expect_string(".f16x2")?;
            let f16x2 = ();
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
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
            let a = TexHandler3Optional::parse(stream)?;
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
            Ok(TexBaseGeomV2F16x2Ctype {
                base,
                geom,
                v2,
                f16x2,
                ctype,
                d,
                p,
                a,
                e,
                f,
            })
        }
    }

    impl PtxParser for TexLevelGeomV2F16x2Ctype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tex")?;
            stream.expect_string(".level")?;
            let level = ();
            stream.expect_complete()?;
            let geom = Geom::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".v2")?;
            let v2 = ();
            stream.expect_complete()?;
            stream.expect_string(".f16x2")?;
            let f16x2 = ();
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
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
            let a = TexHandler3Optional::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let lod = GeneralOperand::parse(stream)?;
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
            Ok(TexLevelGeomV2F16x2Ctype {
                level,
                geom,
                v2,
                f16x2,
                ctype,
                d,
                p,
                a,
                lod,
                e,
                f,
            })
        }
    }

    impl PtxParser for TexGradGeomV2F16x2Ctype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tex")?;
            stream.expect_string(".grad")?;
            let grad = ();
            stream.expect_complete()?;
            let geom = Geom::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".v2")?;
            let v2 = ();
            stream.expect_complete()?;
            stream.expect_string(".f16x2")?;
            let f16x2 = ();
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
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
            let a = TexHandler3Optional::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let dpdx = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let dpdy = GeneralOperand::parse(stream)?;
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
            Ok(TexGradGeomV2F16x2Ctype {
                grad,
                geom,
                v2,
                f16x2,
                ctype,
                d,
                p,
                a,
                dpdx,
                dpdy,
                e,
                f,
            })
        }
    }
}
