//! Original PTX specification:
//!
//! // Half precision floating point type:
//! mma.sync.aligned.m8n8k4.alayout.blayout.dtype.f16.f16.ctype  d, a, b, c;
//! mma.sync.aligned.m16n8k8.row.col.dtype.f16.f16.ctype  d, a, b, c;
//! mma.sync.aligned.m16n8k16.row.col.dtype.f16.f16.ctype d, a, b, c;
//! .alayout = {.row, .col};
//! .blayout = {.row, .col};
//! .ctype   = {.f16, .f32};
//! .dtype   = {.f16, .f32};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! // Alternate floating point type:
//! mma.sync.aligned.m16n8k4.row.col.f32.tf32.tf32.f32        d, a, b, c;
//! mma.sync.aligned.m16n8k8.row.col.f32.atype.btype.f32      d, a, b, c;
//! mma.sync.aligned.m16n8k16.row.col.f32.bf16.bf16.f32       d, a, b, c;
//! mma.sync.aligned.shape.row.col.dtype.f8type.f8type.ctype  d, a, b, c;
//! mma.sync.aligned.m16n8k32.row.col.kind.dtype.f8f6f4type.f8f6f4type.ctype d, a, b, c;
//! .atype      = {.bf16, .tf32};
//! .btype      = {.bf16, .tf32};
//! .f8type     = {.e4m3, .e5m2};
//! .f8f6f4type = {.e4m3, .e5m2, .e3m2, .e2m3, .e2m1};
//! .ctype      = {.f16, .f32};
//! .dtype      = {.f16, .f32};
//! .shape      = {.m16n8k16, .m16n8k32};
//! .kind       = {.kind::f8f6f4};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! // Alternate floating point type with block scaling:
//! mma.sync.aligned.m16n8k64.row.col.kind.block_scale{.scale_vec_size}.f32.e2m1.e2m1.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};
//! .kind           = {.kind::mxf4};
//! .scale_vec_size = {.scale_vec::2X};
//! .stype          = {.ue8m0};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! mma.sync.aligned.m16n8k64.row.col.kind.block_scale.scale_vec_size.f32.e2m1.e2m1.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};
//! .kind           = {.kind::mxf4nvf4};
//! .scale_vec_size = {.scale_vec::2X, .scale_vec::4X};
//! .stype          = {.ue8m0, .ue4m3};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! mma.sync.aligned.m16n8k32.row.col.kind.block_scale{.scale_vec_size}.f32.f8f6f4type.f8f6f4type.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};
//! .kind           = {.kind::mxf8f6f4};
//! .scale_vec_size = {.scale_vec::1X};
//! .f8f6f4type     = {.e4m3, .e5m2, .e3m2, .e2m3, .e2m1};
//! .stype          = {.ue8m0};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! // Double precision floating point type:
//! mma.sync.aligned.shape.row.col.f64.f64.f64.f64 d, a, b, c;
//! .shape   = {.m8n84, .m16n8k4, .m16n8k8, .m16n8k16};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! // Integer type:
//! mma.sync.aligned.shape.row.col{.satfinite}.s32.atype.btype.s32 d, a, b, c;
//! .shape   = {.m8n8k16, .m16n8k16, .m16n8k32};
//! .atype   = {.u8, .s8};
//! .btype   = {.u8, .s8};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! mma.sync.aligned.shape.row.col{.satfinite}.s32.atype.btype.s32 d, a, b, c;
//! .shape   = {.m8n8k32, .m16n8k32, .m16n8k64};
//! .atype   = {.u4, .s4};
//! .btype   = {.u4, .s4};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! // Single bit:
//! mma.sync.aligned.shape.row.col.s32.b1.b1.s32.bitOp.popc d, a, b, c;
//! .bitOp = {.xor, .and};
//! .shape = {.m8n8k128, .m16n8k128, .m16n8k256};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mma::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Alayout {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Row
            {
                let saved_pos = stream.position();
                if stream.expect_string(".row").is_ok() {
                    return Ok(Alayout::Row);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Col
            {
                let saved_pos = stream.position();
                if stream.expect_string(".col").is_ok() {
                    return Ok(Alayout::Col);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".row", ".col"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Blayout {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Row
            {
                let saved_pos = stream.position();
                if stream.expect_string(".row").is_ok() {
                    return Ok(Blayout::Row);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Col
            {
                let saved_pos = stream.position();
                if stream.expect_string(".col").is_ok() {
                    return Ok(Blayout::Col);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".row", ".col"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f16", ".f32"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Dtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try F16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16").is_ok() {
                    return Ok(Dtype::F16);
                }
                stream.set_position(saved_pos);
            }
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
            let expected = &[".f16", ".f32"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for MmaSyncAlignedM8n8k4AlayoutBlayoutDtypeF16F16Ctype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mma")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            stream.expect_string(".m8n8k4")?;
            let m8n8k4 = ();
            stream.expect_complete()?;
            let alayout = Alayout::parse(stream)?;
            stream.expect_complete()?;
            let blayout = Blayout::parse(stream)?;
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".f16")?;
            let f16 = ();
            stream.expect_complete()?;
            stream.expect_string(".f16")?;
            let f162 = ();
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MmaSyncAlignedM8n8k4AlayoutBlayoutDtypeF16F16Ctype {
                sync,
                aligned,
                m8n8k4,
                alayout,
                blayout,
                dtype,
                f16,
                f162,
                ctype,
                d,
                a,
                b,
                c,
            })
        }
    }

    impl PtxParser for MmaSyncAlignedM16n8k8RowColDtypeF16F16Ctype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mma")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            stream.expect_string(".m16n8k8")?;
            let m16n8k8 = ();
            stream.expect_complete()?;
            stream.expect_string(".row")?;
            let row = ();
            stream.expect_complete()?;
            stream.expect_string(".col")?;
            let col = ();
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".f16")?;
            let f16 = ();
            stream.expect_complete()?;
            stream.expect_string(".f16")?;
            let f162 = ();
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MmaSyncAlignedM16n8k8RowColDtypeF16F16Ctype {
                sync,
                aligned,
                m16n8k8,
                row,
                col,
                dtype,
                f16,
                f162,
                ctype,
                d,
                a,
                b,
                c,
            })
        }
    }

    impl PtxParser for MmaSyncAlignedM16n8k16RowColDtypeF16F16Ctype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mma")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            stream.expect_string(".m16n8k16")?;
            let m16n8k16 = ();
            stream.expect_complete()?;
            stream.expect_string(".row")?;
            let row = ();
            stream.expect_complete()?;
            stream.expect_string(".col")?;
            let col = ();
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".f16")?;
            let f16 = ();
            stream.expect_complete()?;
            stream.expect_string(".f16")?;
            let f162 = ();
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MmaSyncAlignedM16n8k16RowColDtypeF16F16Ctype {
                sync,
                aligned,
                m16n8k16,
                row,
                col,
                dtype,
                f16,
                f162,
                ctype,
                d,
                a,
                b,
                c,
            })
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::mma::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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
            let saved_pos = stream.position();
            // Try Tf32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".tf32").is_ok() {
                    return Ok(Atype::Tf32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".bf16", ".tf32"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
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
            let saved_pos = stream.position();
            // Try Tf32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".tf32").is_ok() {
                    return Ok(Btype::Tf32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".bf16", ".tf32"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f16", ".f32"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Dtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try F16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16").is_ok() {
                    return Ok(Dtype::F16);
                }
                stream.set_position(saved_pos);
            }
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
            let expected = &[".f16", ".f32"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for F8f6f4type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try E4m3
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e4m3").is_ok() {
                    return Ok(F8f6f4type::E4m3);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try E5m2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e5m2").is_ok() {
                    return Ok(F8f6f4type::E5m2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try E3m2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e3m2").is_ok() {
                    return Ok(F8f6f4type::E3m2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try E2m3
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e2m3").is_ok() {
                    return Ok(F8f6f4type::E2m3);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try E2m1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e2m1").is_ok() {
                    return Ok(F8f6f4type::E2m1);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".e4m3", ".e5m2", ".e3m2", ".e2m3", ".e2m1"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for F8type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try E4m3
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e4m3").is_ok() {
                    return Ok(F8type::E4m3);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try E5m2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e5m2").is_ok() {
                    return Ok(F8type::E5m2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".e4m3", ".e5m2"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Kind {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try KindF8f6f4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".kind::f8f6f4").is_ok() {
                    return Ok(Kind::KindF8f6f4);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".kind::f8f6f4"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M16n8k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m16n8k16").is_ok() {
                    return Ok(Shape::M16n8k16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try M16n8k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m16n8k32").is_ok() {
                    return Ok(Shape::M16n8k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m16n8k16", ".m16n8k32"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for MmaSyncAlignedM16n8k4RowColF32Tf32Tf32F32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mma")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            stream.expect_string(".m16n8k4")?;
            let m16n8k4 = ();
            stream.expect_complete()?;
            stream.expect_string(".row")?;
            let row = ();
            stream.expect_complete()?;
            stream.expect_string(".col")?;
            let col = ();
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
            stream.expect_complete()?;
            stream.expect_string(".tf32")?;
            let tf32 = ();
            stream.expect_complete()?;
            stream.expect_string(".tf32")?;
            let tf322 = ();
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f322 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MmaSyncAlignedM16n8k4RowColF32Tf32Tf32F32 {
                sync,
                aligned,
                m16n8k4,
                row,
                col,
                f32,
                tf32,
                tf322,
                f322,
                d,
                a,
                b,
                c,
            })
        }
    }

    impl PtxParser for MmaSyncAlignedM16n8k8RowColF32AtypeBtypeF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mma")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            stream.expect_string(".m16n8k8")?;
            let m16n8k8 = ();
            stream.expect_complete()?;
            stream.expect_string(".row")?;
            let row = ();
            stream.expect_complete()?;
            stream.expect_string(".col")?;
            let col = ();
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f322 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MmaSyncAlignedM16n8k8RowColF32AtypeBtypeF32 {
                sync,
                aligned,
                m16n8k8,
                row,
                col,
                f32,
                atype,
                btype,
                f322,
                d,
                a,
                b,
                c,
            })
        }
    }

    impl PtxParser for MmaSyncAlignedM16n8k16RowColF32Bf16Bf16F32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mma")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            stream.expect_string(".m16n8k16")?;
            let m16n8k16 = ();
            stream.expect_complete()?;
            stream.expect_string(".row")?;
            let row = ();
            stream.expect_complete()?;
            stream.expect_string(".col")?;
            let col = ();
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
            stream.expect_complete()?;
            stream.expect_string(".bf16")?;
            let bf16 = ();
            stream.expect_complete()?;
            stream.expect_string(".bf16")?;
            let bf162 = ();
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f322 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MmaSyncAlignedM16n8k16RowColF32Bf16Bf16F32 {
                sync,
                aligned,
                m16n8k16,
                row,
                col,
                f32,
                bf16,
                bf162,
                f322,
                d,
                a,
                b,
                c,
            })
        }
    }

    impl PtxParser for MmaSyncAlignedShapeRowColDtypeF8typeF8typeCtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mma")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".row")?;
            let row = ();
            stream.expect_complete()?;
            stream.expect_string(".col")?;
            let col = ();
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let f8type = F8type::parse(stream)?;
            stream.expect_complete()?;
            let f8type1 = F8type::parse(stream)?;
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MmaSyncAlignedShapeRowColDtypeF8typeF8typeCtype {
                sync,
                aligned,
                shape,
                row,
                col,
                dtype,
                f8type,
                f8type1,
                ctype,
                d,
                a,
                b,
                c,
            })
        }
    }

    impl PtxParser for MmaSyncAlignedM16n8k32RowColKindDtypeF8f6f4typeF8f6f4typeCtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mma")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            stream.expect_string(".m16n8k32")?;
            let m16n8k32 = ();
            stream.expect_complete()?;
            stream.expect_string(".row")?;
            let row = ();
            stream.expect_complete()?;
            stream.expect_string(".col")?;
            let col = ();
            stream.expect_complete()?;
            let kind = Kind::parse(stream)?;
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let f8f6f4type = F8f6f4type::parse(stream)?;
            stream.expect_complete()?;
            let f8f6f4type1 = F8f6f4type::parse(stream)?;
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(
                MmaSyncAlignedM16n8k32RowColKindDtypeF8f6f4typeF8f6f4typeCtype {
                    sync,
                    aligned,
                    m16n8k32,
                    row,
                    col,
                    kind,
                    dtype,
                    f8f6f4type,
                    f8f6f4type1,
                    ctype,
                    d,
                    a,
                    b,
                    c,
                },
            )
        }
    }
}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::mma::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Kind {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try KindMxf4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".kind::mxf4").is_ok() {
                    return Ok(Kind::KindMxf4);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".kind::mxf4"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for ScaleVecSize {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try ScaleVec2x
            {
                let saved_pos = stream.position();
                if stream.expect_string(".scale_vec::2X").is_ok() {
                    return Ok(ScaleVecSize::ScaleVec2x);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".scale_vec::2X"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Stype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Ue8m0
            {
                let saved_pos = stream.position();
                if stream.expect_string(".ue8m0").is_ok() {
                    return Ok(Stype::Ue8m0);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".ue8m0"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mma")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            stream.expect_string(".m16n8k64")?;
            let m16n8k64 = ();
            stream.expect_complete()?;
            stream.expect_string(".row")?;
            let row = ();
            stream.expect_complete()?;
            stream.expect_string(".col")?;
            let col = ();
            stream.expect_complete()?;
            let kind = Kind::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".block_scale")?;
            let block_scale = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scale_vec_size = match ScaleVecSize::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
            stream.expect_complete()?;
            stream.expect_string(".e2m1")?;
            let e2m1 = ();
            stream.expect_complete()?;
            stream.expect_string(".e2m1")?;
            let e2m12 = ();
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f322 = ();
            stream.expect_complete()?;
            let stype = Stype::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_a_data = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let byte_id_a = VectorOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_b_data = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let byte_id_b = VectorOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(
                MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype {
                    sync,
                    aligned,
                    m16n8k64,
                    row,
                    col,
                    kind,
                    block_scale,
                    scale_vec_size,
                    f32,
                    e2m1,
                    e2m12,
                    f322,
                    stype,
                    d,
                    a,
                    b,
                    c,
                    scale_a_data,
                    byte_id_a,
                    scale_b_data,
                    byte_id_b,
                },
            )
        }
    }
}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::mma::section_3::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Kind {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try KindMxf4nvf4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".kind::mxf4nvf4").is_ok() {
                    return Ok(Kind::KindMxf4nvf4);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".kind::mxf4nvf4"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for ScaleVecSize {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try ScaleVec2x
            {
                let saved_pos = stream.position();
                if stream.expect_string(".scale_vec::2X").is_ok() {
                    return Ok(ScaleVecSize::ScaleVec2x);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try ScaleVec4x
            {
                let saved_pos = stream.position();
                if stream.expect_string(".scale_vec::4X").is_ok() {
                    return Ok(ScaleVecSize::ScaleVec4x);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".scale_vec::2X", ".scale_vec::4X"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Stype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Ue8m0
            {
                let saved_pos = stream.position();
                if stream.expect_string(".ue8m0").is_ok() {
                    return Ok(Stype::Ue8m0);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Ue4m3
            {
                let saved_pos = stream.position();
                if stream.expect_string(".ue4m3").is_ok() {
                    return Ok(Stype::Ue4m3);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".ue8m0", ".ue4m3"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mma")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            stream.expect_string(".m16n8k64")?;
            let m16n8k64 = ();
            stream.expect_complete()?;
            stream.expect_string(".row")?;
            let row = ();
            stream.expect_complete()?;
            stream.expect_string(".col")?;
            let col = ();
            stream.expect_complete()?;
            let kind = Kind::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".block_scale")?;
            let block_scale = ();
            stream.expect_complete()?;
            let scale_vec_size = ScaleVecSize::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
            stream.expect_complete()?;
            stream.expect_string(".e2m1")?;
            let e2m1 = ();
            stream.expect_complete()?;
            stream.expect_string(".e2m1")?;
            let e2m12 = ();
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f322 = ();
            stream.expect_complete()?;
            let stype = Stype::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_a_data = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let byte_id_a = VectorOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_b_data = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let byte_id_b = VectorOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(
                MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1 {
                    sync,
                    aligned,
                    m16n8k64,
                    row,
                    col,
                    kind,
                    block_scale,
                    scale_vec_size,
                    f32,
                    e2m1,
                    e2m12,
                    f322,
                    stype,
                    d,
                    a,
                    b,
                    c,
                    scale_a_data,
                    byte_id_a,
                    scale_b_data,
                    byte_id_b,
                },
            )
        }
    }
}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::mma::section_4::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for F8f6f4type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try E4m3
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e4m3").is_ok() {
                    return Ok(F8f6f4type::E4m3);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try E5m2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e5m2").is_ok() {
                    return Ok(F8f6f4type::E5m2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try E3m2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e3m2").is_ok() {
                    return Ok(F8f6f4type::E3m2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try E2m3
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e2m3").is_ok() {
                    return Ok(F8f6f4type::E2m3);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try E2m1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e2m1").is_ok() {
                    return Ok(F8f6f4type::E2m1);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".e4m3", ".e5m2", ".e3m2", ".e2m3", ".e2m1"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Kind {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try KindMxf8f6f4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".kind::mxf8f6f4").is_ok() {
                    return Ok(Kind::KindMxf8f6f4);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".kind::mxf8f6f4"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for ScaleVecSize {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try ScaleVec1x
            {
                let saved_pos = stream.position();
                if stream.expect_string(".scale_vec::1X").is_ok() {
                    return Ok(ScaleVecSize::ScaleVec1x);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".scale_vec::1X"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Stype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Ue8m0
            {
                let saved_pos = stream.position();
                if stream.expect_string(".ue8m0").is_ok() {
                    return Ok(Stype::Ue8m0);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".ue8m0"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser
        for MmaSyncAlignedM16n8k32RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype
    {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mma")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            stream.expect_string(".m16n8k32")?;
            let m16n8k32 = ();
            stream.expect_complete()?;
            stream.expect_string(".row")?;
            let row = ();
            stream.expect_complete()?;
            stream.expect_string(".col")?;
            let col = ();
            stream.expect_complete()?;
            let kind = Kind::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".block_scale")?;
            let block_scale = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scale_vec_size = match ScaleVecSize::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
            stream.expect_complete()?;
            let f8f6f4type = F8f6f4type::parse(stream)?;
            stream.expect_complete()?;
            let f8f6f4type1 = F8f6f4type::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f322 = ();
            stream.expect_complete()?;
            let stype = Stype::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_a_data = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let byte_id_a = VectorOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_b_data = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let byte_id_b = VectorOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MmaSyncAlignedM16n8k32RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype {
                sync,
                aligned,
                m16n8k32,
                row,
                col,
                kind,
                block_scale,
                scale_vec_size,
                f32,
                f8f6f4type,
                f8f6f4type1,
                f322,
                stype,
                d,
                a,
                b,
                c,
                scale_a_data,
                byte_id_a,
                scale_b_data,
                byte_id_b,
            })
        }
    }
}

pub mod section_5 {
    use super::*;
    use crate::r#type::instruction::mma::section_5::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M16n8k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m16n8k16").is_ok() {
                    return Ok(Shape::M16n8k16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try M16n8k4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m16n8k4").is_ok() {
                    return Ok(Shape::M16n8k4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M16n8k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m16n8k8").is_ok() {
                    return Ok(Shape::M16n8k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M8n84
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m8n84").is_ok() {
                    return Ok(Shape::M8n84);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m16n8k16", ".m16n8k4", ".m16n8k8", ".m8n84"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for MmaSyncAlignedShapeRowColF64F64F64F64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mma")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".row")?;
            let row = ();
            stream.expect_complete()?;
            stream.expect_string(".col")?;
            let col = ();
            stream.expect_complete()?;
            stream.expect_string(".f64")?;
            let f64 = ();
            stream.expect_complete()?;
            stream.expect_string(".f64")?;
            let f642 = ();
            stream.expect_complete()?;
            stream.expect_string(".f64")?;
            let f644 = ();
            stream.expect_complete()?;
            stream.expect_string(".f64")?;
            let f646 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MmaSyncAlignedShapeRowColF64F64F64F64 {
                sync,
                aligned,
                shape,
                row,
                col,
                f64,
                f642,
                f644,
                f646,
                d,
                a,
                b,
                c,
            })
        }
    }
}

pub mod section_6 {
    use super::*;
    use crate::r#type::instruction::mma::section_6::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u8").is_ok() {
                    return Ok(Atype::U8);
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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u8", ".s8"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Btype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u8").is_ok() {
                    return Ok(Btype::U8);
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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u8", ".s8"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M16n8k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m16n8k16").is_ok() {
                    return Ok(Shape::M16n8k16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try M16n8k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m16n8k32").is_ok() {
                    return Ok(Shape::M16n8k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M8n8k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m8n8k16").is_ok() {
                    return Ok(Shape::M8n8k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m16n8k16", ".m16n8k32", ".m8n8k16"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mma")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".row")?;
            let row = ();
            stream.expect_complete()?;
            stream.expect_string(".col")?;
            let col = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let satfinite = stream.expect_string(".satfinite").is_ok();
            if !satfinite {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".s32")?;
            let s32 = ();
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".s32")?;
            let s322 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32 {
                sync,
                aligned,
                shape,
                row,
                col,
                satfinite,
                s32,
                atype,
                btype,
                s322,
                d,
                a,
                b,
                c,
            })
        }
    }
}

pub mod section_7 {
    use super::*;
    use crate::r#type::instruction::mma::section_7::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u4").is_ok() {
                    return Ok(Atype::U4);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try S4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s4").is_ok() {
                    return Ok(Atype::S4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u4", ".s4"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Btype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u4").is_ok() {
                    return Ok(Btype::U4);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try S4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s4").is_ok() {
                    return Ok(Btype::S4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u4", ".s4"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M16n8k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m16n8k32").is_ok() {
                    return Ok(Shape::M16n8k32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try M16n8k64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m16n8k64").is_ok() {
                    return Ok(Shape::M16n8k64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M8n8k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m8n8k32").is_ok() {
                    return Ok(Shape::M8n8k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m16n8k32", ".m16n8k64", ".m8n8k32"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mma")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".row")?;
            let row = ();
            stream.expect_complete()?;
            stream.expect_string(".col")?;
            let col = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let satfinite = stream.expect_string(".satfinite").is_ok();
            if !satfinite {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".s32")?;
            let s32 = ();
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".s32")?;
            let s322 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321 {
                sync,
                aligned,
                shape,
                row,
                col,
                satfinite,
                s32,
                atype,
                btype,
                s322,
                d,
                a,
                b,
                c,
            })
        }
    }
}

pub mod section_8 {
    use super::*;
    use crate::r#type::instruction::mma::section_8::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Bitop {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Xor
            {
                let saved_pos = stream.position();
                if stream.expect_string(".xor").is_ok() {
                    return Ok(Bitop::Xor);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try And
            {
                let saved_pos = stream.position();
                if stream.expect_string(".and").is_ok() {
                    return Ok(Bitop::And);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".xor", ".and"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M16n8k128
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m16n8k128").is_ok() {
                    return Ok(Shape::M16n8k128);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try M16n8k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m16n8k256").is_ok() {
                    return Ok(Shape::M16n8k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M8n8k128
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m8n8k128").is_ok() {
                    return Ok(Shape::M8n8k128);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m16n8k128", ".m16n8k256", ".m8n8k128"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for MmaSyncAlignedShapeRowColS32B1B1S32BitopPopc {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mma")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".row")?;
            let row = ();
            stream.expect_complete()?;
            stream.expect_string(".col")?;
            let col = ();
            stream.expect_complete()?;
            stream.expect_string(".s32")?;
            let s32 = ();
            stream.expect_complete()?;
            stream.expect_string(".b1")?;
            let b1 = ();
            stream.expect_complete()?;
            stream.expect_string(".b1")?;
            let b12 = ();
            stream.expect_complete()?;
            stream.expect_string(".s32")?;
            let s322 = ();
            stream.expect_complete()?;
            let bitop = Bitop::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".popc")?;
            let popc = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(MmaSyncAlignedShapeRowColS32B1B1S32BitopPopc {
                sync,
                aligned,
                shape,
                row,
                col,
                s32,
                b1,
                b12,
                s322,
                bitop,
                popc,
                d,
                a,
                b,
                c,
            })
        }
    }
}
