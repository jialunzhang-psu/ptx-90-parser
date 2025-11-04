//! Original PTX specification:
//!
//! // Floating point format .f16 loads:
//! wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k16, .m8n32k16, .m32n8k16};
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.f16, .s8, .u8};
//! .btype  = {.f16, .s8, .u8};
//! .ctype  = {.f16, .f32, .s32};
//! ----------------------------------------------------------------
//! // Alternate floating point format .bf16 loads:
//! wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k16, .m8n32k16, .m32n8k16};
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.bf16 };
//! .btype  = {.bf16 };
//! .ctype  = {.f32 };
//! ----------------------------------------------------------------
//! // Alternate floating point format .tf32 loads:
//! wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k8 };
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.tf32 };
//! .btype  = {.tf32 };
//! .ctype  = {.f32 };
//! ----------------------------------------------------------------
//! // Double precision Floating point .f64 loads:
//! wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k4 };
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.f64 };
//! .btype  = {.f64 };
//! .ctype  = {.f64 };
//! ----------------------------------------------------------------
//! // Sub-byte loads:
//! wmma.load.a.sync.aligned.row.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.col.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k32};
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.s4, .u4};
//! .btype  = {.s4, .u4};
//! .ctype  = {.s32};
//! ----------------------------------------------------------------
//! // Single-bit loads:
//! wmma.load.a.sync.aligned.row.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.col.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k128};
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.b1};
//! .btype  = {.b1};
//! .ctype  = {.s32};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::wmma_load::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Ctype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try F16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16").is_ok() {
                    return Ok(Ctype::F16);
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
            let saved_pos = stream.position();
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Ctype::S32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f16", ".f32", ".s32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Ss {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Global
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global").is_ok() {
                    return Ok(Ss::Global);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Shared
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared").is_ok() {
                    return Ok(Ss::Shared);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
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
            let expected = &[".global", ".shared", ".shared::cta"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Btype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try F16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16").is_ok() {
                    return Ok(Btype::F16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try S8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s8").is_ok() {
                    return Ok(Btype::S8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u8").is_ok() {
                    return Ok(Btype::U8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f16", ".s8", ".u8"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Layout {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Row
            {
                let saved_pos = stream.position();
                if stream.expect_string(".row").is_ok() {
                    return Ok(Layout::Row);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Col
            {
                let saved_pos = stream.position();
                if stream.expect_string(".col").is_ok() {
                    return Ok(Layout::Col);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".row", ".col"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try F16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16").is_ok() {
                    return Ok(Atype::F16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try S8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s8").is_ok() {
                    return Ok(Atype::S8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u8").is_ok() {
                    return Ok(Atype::U8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f16", ".s8", ".u8"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M16n16k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m16n16k16").is_ok() {
                    return Ok(Shape::M16n16k16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try M8n32k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m8n32k16").is_ok() {
                    return Ok(Shape::M8n32k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M32n8k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m32n8k16").is_ok() {
                    return Ok(Shape::M32n8k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m16n16k16", ".m8n32k16", ".m32n8k16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for WmmaLoadASyncAlignedLayoutShapeSsAtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".a")?;
            let a = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let layout = Layout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let atype = Atype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadASyncAlignedLayoutShapeSsAtype {
                load,
                a,
                sync,
                aligned,
                layout,
                shape,
                ss,
                atype,
                r,
                p,
                stride,
            })
        }
    }


    impl PtxParser for WmmaLoadBSyncAlignedLayoutShapeSsBtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".b")?;
            let b = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let layout = Layout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let btype = Btype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadBSyncAlignedLayoutShapeSsBtype {
                load,
                b,
                sync,
                aligned,
                layout,
                shape,
                ss,
                btype,
                r,
                p,
                stride,
            })
        }
    }


    impl PtxParser for WmmaLoadCSyncAlignedLayoutShapeSsCtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".c")?;
            let c = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let layout = Layout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let ctype = Ctype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadCSyncAlignedLayoutShapeSsCtype {
                load,
                c,
                sync,
                aligned,
                layout,
                shape,
                ss,
                ctype,
                r,
                p,
                stride,
            })
        }
    }


}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::wmma_load::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Ctype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try F32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f32").is_ok() {
                    return Ok(Ctype::F32);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Ss {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Global
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global").is_ok() {
                    return Ok(Ss::Global);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Shared
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared").is_ok() {
                    return Ok(Ss::Shared);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
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
            let expected = &[".global", ".shared", ".shared::cta"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Btype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Bf16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".bf16").is_ok() {
                    return Ok(Btype::Bf16);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".bf16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Layout {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Row
            {
                let saved_pos = stream.position();
                if stream.expect_string(".row").is_ok() {
                    return Ok(Layout::Row);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Col
            {
                let saved_pos = stream.position();
                if stream.expect_string(".col").is_ok() {
                    return Ok(Layout::Col);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".row", ".col"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Bf16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".bf16").is_ok() {
                    return Ok(Atype::Bf16);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".bf16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M16n16k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m16n16k16").is_ok() {
                    return Ok(Shape::M16n16k16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try M8n32k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m8n32k16").is_ok() {
                    return Ok(Shape::M8n32k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M32n8k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m32n8k16").is_ok() {
                    return Ok(Shape::M32n8k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m16n16k16", ".m8n32k16", ".m32n8k16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for WmmaLoadASyncAlignedLayoutShapeSsAtype1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".a")?;
            let a = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let layout = Layout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let atype = Atype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadASyncAlignedLayoutShapeSsAtype1 {
                load,
                a,
                sync,
                aligned,
                layout,
                shape,
                ss,
                atype,
                r,
                p,
                stride,
            })
        }
    }


    impl PtxParser for WmmaLoadBSyncAlignedLayoutShapeSsBtype1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".b")?;
            let b = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let layout = Layout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let btype = Btype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadBSyncAlignedLayoutShapeSsBtype1 {
                load,
                b,
                sync,
                aligned,
                layout,
                shape,
                ss,
                btype,
                r,
                p,
                stride,
            })
        }
    }


    impl PtxParser for WmmaLoadCSyncAlignedLayoutShapeSsCtype1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".c")?;
            let c = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let layout = Layout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let ctype = Ctype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadCSyncAlignedLayoutShapeSsCtype1 {
                load,
                c,
                sync,
                aligned,
                layout,
                shape,
                ss,
                ctype,
                r,
                p,
                stride,
            })
        }
    }


}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::wmma_load::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Ctype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try F32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f32").is_ok() {
                    return Ok(Ctype::F32);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Ss {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Global
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global").is_ok() {
                    return Ok(Ss::Global);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Shared
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared").is_ok() {
                    return Ok(Ss::Shared);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
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
            let expected = &[".global", ".shared", ".shared::cta"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Btype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Tf32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".tf32").is_ok() {
                    return Ok(Btype::Tf32);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".tf32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Layout {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Row
            {
                let saved_pos = stream.position();
                if stream.expect_string(".row").is_ok() {
                    return Ok(Layout::Row);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Col
            {
                let saved_pos = stream.position();
                if stream.expect_string(".col").is_ok() {
                    return Ok(Layout::Col);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".row", ".col"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Tf32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".tf32").is_ok() {
                    return Ok(Atype::Tf32);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".tf32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M16n16k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m16n16k8").is_ok() {
                    return Ok(Shape::M16n16k8);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m16n16k8"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for WmmaLoadASyncAlignedLayoutShapeSsAtype2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".a")?;
            let a = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let layout = Layout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let atype = Atype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadASyncAlignedLayoutShapeSsAtype2 {
                load,
                a,
                sync,
                aligned,
                layout,
                shape,
                ss,
                atype,
                r,
                p,
                stride,
            })
        }
    }


    impl PtxParser for WmmaLoadBSyncAlignedLayoutShapeSsBtype2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".b")?;
            let b = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let layout = Layout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let btype = Btype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadBSyncAlignedLayoutShapeSsBtype2 {
                load,
                b,
                sync,
                aligned,
                layout,
                shape,
                ss,
                btype,
                r,
                p,
                stride,
            })
        }
    }


    impl PtxParser for WmmaLoadCSyncAlignedLayoutShapeSsCtype2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".c")?;
            let c = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let layout = Layout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let ctype = Ctype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadCSyncAlignedLayoutShapeSsCtype2 {
                load,
                c,
                sync,
                aligned,
                layout,
                shape,
                ss,
                ctype,
                r,
                p,
                stride,
            })
        }
    }


}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::wmma_load::section_3::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Ctype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try F64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f64").is_ok() {
                    return Ok(Ctype::F64);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f64"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Ss {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Global
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global").is_ok() {
                    return Ok(Ss::Global);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Shared
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared").is_ok() {
                    return Ok(Ss::Shared);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
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
            let expected = &[".global", ".shared", ".shared::cta"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Btype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try F64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f64").is_ok() {
                    return Ok(Btype::F64);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f64"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Layout {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Row
            {
                let saved_pos = stream.position();
                if stream.expect_string(".row").is_ok() {
                    return Ok(Layout::Row);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Col
            {
                let saved_pos = stream.position();
                if stream.expect_string(".col").is_ok() {
                    return Ok(Layout::Col);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".row", ".col"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try F64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f64").is_ok() {
                    return Ok(Atype::F64);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f64"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M8n8k4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m8n8k4").is_ok() {
                    return Ok(Shape::M8n8k4);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m8n8k4"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for WmmaLoadASyncAlignedLayoutShapeSsAtype3 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".a")?;
            let a = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let layout = Layout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let atype = Atype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadASyncAlignedLayoutShapeSsAtype3 {
                load,
                a,
                sync,
                aligned,
                layout,
                shape,
                ss,
                atype,
                r,
                p,
                stride,
            })
        }
    }


    impl PtxParser for WmmaLoadBSyncAlignedLayoutShapeSsBtype3 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".b")?;
            let b = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let layout = Layout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let btype = Btype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadBSyncAlignedLayoutShapeSsBtype3 {
                load,
                b,
                sync,
                aligned,
                layout,
                shape,
                ss,
                btype,
                r,
                p,
                stride,
            })
        }
    }


    impl PtxParser for WmmaLoadCSyncAlignedLayoutShapeSsCtype3 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".c")?;
            let c = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let layout = Layout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let ctype = Ctype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadCSyncAlignedLayoutShapeSsCtype3 {
                load,
                c,
                sync,
                aligned,
                layout,
                shape,
                ss,
                ctype,
                r,
                p,
                stride,
            })
        }
    }


}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::wmma_load::section_4::*;

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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".s32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Ss {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Global
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global").is_ok() {
                    return Ok(Ss::Global);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Shared
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared").is_ok() {
                    return Ok(Ss::Shared);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
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
            let expected = &[".global", ".shared", ".shared::cta"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Btype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try S4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s4").is_ok() {
                    return Ok(Btype::S4);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try U4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u4").is_ok() {
                    return Ok(Btype::U4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".s4", ".u4"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Layout {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Row
            {
                let saved_pos = stream.position();
                if stream.expect_string(".row").is_ok() {
                    return Ok(Layout::Row);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Col
            {
                let saved_pos = stream.position();
                if stream.expect_string(".col").is_ok() {
                    return Ok(Layout::Col);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".row", ".col"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try S4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s4").is_ok() {
                    return Ok(Atype::S4);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try U4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u4").is_ok() {
                    return Ok(Atype::U4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".s4", ".u4"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M8n8k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m8n8k32").is_ok() {
                    return Ok(Shape::M8n8k32);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m8n8k32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for WmmaLoadASyncAlignedRowShapeSsAtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".a")?;
            let a = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_string(".row")?;
            let row = ();
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let atype = Atype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadASyncAlignedRowShapeSsAtype {
                load,
                a,
                sync,
                aligned,
                row,
                shape,
                ss,
                atype,
                r,
                p,
                stride,
            })
        }
    }


    impl PtxParser for WmmaLoadBSyncAlignedColShapeSsBtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".b")?;
            let b = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_string(".col")?;
            let col = ();
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let btype = Btype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadBSyncAlignedColShapeSsBtype {
                load,
                b,
                sync,
                aligned,
                col,
                shape,
                ss,
                btype,
                r,
                p,
                stride,
            })
        }
    }


    impl PtxParser for WmmaLoadCSyncAlignedLayoutShapeSsCtype4 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".c")?;
            let c = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let layout = Layout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let ctype = Ctype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadCSyncAlignedLayoutShapeSsCtype4 {
                load,
                c,
                sync,
                aligned,
                layout,
                shape,
                ss,
                ctype,
                r,
                p,
                stride,
            })
        }
    }


}

pub mod section_5 {
    use super::*;
    use crate::r#type::instruction::wmma_load::section_5::*;

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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".s32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Ss {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Global
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global").is_ok() {
                    return Ok(Ss::Global);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Shared
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared").is_ok() {
                    return Ok(Ss::Shared);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
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
            let expected = &[".global", ".shared", ".shared::cta"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Btype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b1").is_ok() {
                    return Ok(Btype::B1);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b1"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Layout {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Row
            {
                let saved_pos = stream.position();
                if stream.expect_string(".row").is_ok() {
                    return Ok(Layout::Row);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Col
            {
                let saved_pos = stream.position();
                if stream.expect_string(".col").is_ok() {
                    return Ok(Layout::Col);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".row", ".col"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b1").is_ok() {
                    return Ok(Atype::B1);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b1"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M8n8k128
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m8n8k128").is_ok() {
                    return Ok(Shape::M8n8k128);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m8n8k128"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for WmmaLoadASyncAlignedRowShapeSsAtype1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".a")?;
            let a = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_string(".row")?;
            let row = ();
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let atype = Atype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadASyncAlignedRowShapeSsAtype1 {
                load,
                a,
                sync,
                aligned,
                row,
                shape,
                ss,
                atype,
                r,
                p,
                stride,
            })
        }
    }


    impl PtxParser for WmmaLoadBSyncAlignedColShapeSsBtype1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".b")?;
            let b = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_string(".col")?;
            let col = ();
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let btype = Btype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadBSyncAlignedColShapeSsBtype1 {
                load,
                b,
                sync,
                aligned,
                col,
                shape,
                ss,
                btype,
                r,
                p,
                stride,
            })
        }
    }


    impl PtxParser for WmmaLoadCSyncAlignedLayoutShapeSsCtype5 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".load")?;
            let load = ();
            stream.expect_string(".c")?;
            let c = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let layout = Layout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let ss = match Ss::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let ctype = Ctype::parse(stream)?;
            let r = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let p = AddressOperand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let stride = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(WmmaLoadCSyncAlignedLayoutShapeSsCtype5 {
                load,
                c,
                sync,
                aligned,
                layout,
                shape,
                ss,
                ctype,
                r,
                p,
                stride,
            })
        }
    }


}

