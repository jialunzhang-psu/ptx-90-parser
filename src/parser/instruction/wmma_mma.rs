//! Original PTX specification:
//!
//! // Floating point (.f16 multiplicands) wmma.mma
//! wmma.mma.sync.aligned.alayout.blayout.shape.dtype.ctype d, a, b, c;
//! ----------------------------------------------------------------
//! // Integer (.u8/.s8 multiplicands) wmma.mma
//! wmma.mma.sync.aligned.alayout.blayout.shape.s32.atype.btype.s32{.satfinite} d, a, b, c;
//! .alayout = {.row, .col};
//! .blayout = {.row, .col};
//! .shape  =  {.m16n16k16, .m8n32k16, .m32n8k16};
//! .dtype   = {.f16, .f32};
//! .atype   = {.s8, .u8};
//! .btype   = {.s8, .u8};
//! .ctype   = {.f16, .f32};
//! ----------------------------------------------------------------
//! // Floating point format .bf16 wmma.mma:
//! wmma.mma.sync.aligned.alayout.blayout.shape.f32.atype.btype.f32 d, a, b, c;
//! .alayout = {.row, .col};
//! .blayout = {.row, .col};
//! .shape   = {.m16n16k16, .m8n32k16, .m32n8k16};
//! .atype   = {.bf16 };
//! .btype   = {.bf16};
//! ----------------------------------------------------------------
//! // Floating point format .tf32 wmma.mma:
//! wmma.mma.sync.aligned.alayout.blayout.shape.f32.atype.btype.f32 d, a, b, c;
//! .alayout = {.row, .col};
//! .blayout = {.row, .col};
//! .shape   = {.m16n16k8 };
//! .atype   = {.tf32 };
//! .btype   = {.tf32};
//! ----------------------------------------------------------------
//! // Floating point Double precision wmma.mma:
//! wmma.mma.sync.aligned.alayout.blayout.shape{.rnd}.f64.f64.f64.f64 d, a, b, c;
//! .alayout = {.row, .col};
//! .blayout = {.row, .col};
//! .shape   = {.m8n8k4 };
//! .rnd = { .rn, .rz, .rm, .rp };
//! ----------------------------------------------------------------
//! // Sub-byte (.u4/.s4 multiplicands) wmma.mma:
//! wmma.mma.sync.aligned.row.col.shape.s32.atype.btype.s32{.satfinite} d, a, b, c;
//! .shape  = {.m8n8k32};
//! .atype  = {.s4, .u4};
//! .btype  = {.s4, .u4};
//! ----------------------------------------------------------------
//! // Single-bit (.b1 multiplicands) wmma.mma:
//! wmma.mma.op.popc.sync.aligned.row.col.shape.s32.atype.btype.s32 d, a, b, c;
//! .shape  = {.m8n8k128};
//! .atype  = {.b1};
//! .btype  = {.b1};
//! .op     = {.xor, .and};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::wmma_mma::section_0::*;

    impl PtxParser for WmmaMmaSyncAlignedAlayoutBlayoutShapeDtypeCtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".mma")?;
            let mma = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_string(".alayout")?;
            let alayout = ();
            stream.expect_string(".blayout")?;
            let blayout = ();
            stream.expect_string(".shape")?;
            let shape = ();
            stream.expect_string(".dtype")?;
            let dtype = ();
            stream.expect_string(".ctype")?;
            let ctype = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            Ok(WmmaMmaSyncAlignedAlayoutBlayoutShapeDtypeCtype {
                mma,
                sync,
                aligned,
                alayout,
                blayout,
                shape,
                dtype,
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
    use crate::r#type::instruction::wmma_mma::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try S8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s8").is_ok() {
                    return Ok(Atype::S8);
                }
                stream.set_position(saved_pos);
            }
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
            let expected = &[".s8", ".u8"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".row", ".col"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Btype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try S8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s8").is_ok() {
                    return Ok(Btype::S8);
                }
                stream.set_position(saved_pos);
            }
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
            let expected = &[".s8", ".u8"];
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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".row", ".col"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for WmmaMmaSyncAlignedAlayoutBlayoutShapeS32AtypeBtypeS32Satfinite {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".mma")?;
            let mma = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let alayout = Alayout::parse(stream)?;
            let blayout = Blayout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            stream.expect_string(".s32")?;
            let s32 = ();
            let atype = Atype::parse(stream)?;
            let btype = Btype::parse(stream)?;
            stream.expect_string(".s32")?;
            let s322 = ();
            let saved_pos = stream.position();
            let satfinite = stream.expect_string(".satfinite").is_ok();
            if !satfinite {
                stream.set_position(saved_pos);
            }
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            Ok(WmmaMmaSyncAlignedAlayoutBlayoutShapeS32AtypeBtypeS32Satfinite {
                mma,
                sync,
                aligned,
                alayout,
                blayout,
                shape,
                s32,
                atype,
                btype,
                s322,
                satfinite,
                d,
                a,
                b,
                c,
            })
        }
    }


}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::wmma_mma::section_2::*;

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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".bf16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".row", ".col"];
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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".row", ".col"];
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

    impl PtxParser for WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".mma")?;
            let mma = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let alayout = Alayout::parse(stream)?;
            let blayout = Blayout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            stream.expect_string(".f32")?;
            let f32 = ();
            let atype = Atype::parse(stream)?;
            let btype = Btype::parse(stream)?;
            stream.expect_string(".f32")?;
            let f322 = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            Ok(WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF32 {
                mma,
                sync,
                aligned,
                alayout,
                blayout,
                shape,
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


}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::wmma_mma::section_3::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".row", ".col"];
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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".row", ".col"];
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

    impl PtxParser for WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF321 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".mma")?;
            let mma = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let alayout = Alayout::parse(stream)?;
            let blayout = Blayout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            stream.expect_string(".f32")?;
            let f32 = ();
            let atype = Atype::parse(stream)?;
            let btype = Btype::parse(stream)?;
            stream.expect_string(".f32")?;
            let f322 = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            Ok(WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF321 {
                mma,
                sync,
                aligned,
                alayout,
                blayout,
                shape,
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


}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::wmma_mma::section_4::*;

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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".row", ".col"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Rnd {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Rn
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rn").is_ok() {
                    return Ok(Rnd::Rn);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Rz
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rz").is_ok() {
                    return Ok(Rnd::Rz);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Rm
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rm").is_ok() {
                    return Ok(Rnd::Rm);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Rp
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rp").is_ok() {
                    return Ok(Rnd::Rp);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".rn", ".rz", ".rm", ".rp"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".row", ".col"];
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

    impl PtxParser for WmmaMmaSyncAlignedAlayoutBlayoutShapeRndF64F64F64F64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".mma")?;
            let mma = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let alayout = Alayout::parse(stream)?;
            let blayout = Blayout::parse(stream)?;
            let shape = Shape::parse(stream)?;
            let saved_pos = stream.position();
            let rnd = match Rnd::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_string(".f64")?;
            let f64 = ();
            stream.expect_string(".f64")?;
            let f642 = ();
            stream.expect_string(".f64")?;
            let f644 = ();
            stream.expect_string(".f64")?;
            let f646 = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            Ok(WmmaMmaSyncAlignedAlayoutBlayoutShapeRndF64F64F64F64 {
                mma,
                sync,
                aligned,
                alayout,
                blayout,
                shape,
                rnd,
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

pub mod section_5 {
    use super::*;
    use crate::r#type::instruction::wmma_mma::section_5::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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

    impl PtxParser for WmmaMmaSyncAlignedRowColShapeS32AtypeBtypeS32Satfinite {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".mma")?;
            let mma = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_string(".row")?;
            let row = ();
            stream.expect_string(".col")?;
            let col = ();
            let shape = Shape::parse(stream)?;
            stream.expect_string(".s32")?;
            let s32 = ();
            let atype = Atype::parse(stream)?;
            let btype = Btype::parse(stream)?;
            stream.expect_string(".s32")?;
            let s322 = ();
            let saved_pos = stream.position();
            let satfinite = stream.expect_string(".satfinite").is_ok();
            if !satfinite {
                stream.set_position(saved_pos);
            }
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            Ok(WmmaMmaSyncAlignedRowColShapeS32AtypeBtypeS32Satfinite {
                mma,
                sync,
                aligned,
                row,
                col,
                shape,
                s32,
                atype,
                btype,
                s322,
                satfinite,
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
    use crate::r#type::instruction::wmma_mma::section_6::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Op {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Xor
            {
                let saved_pos = stream.position();
                if stream.expect_string(".xor").is_ok() {
                    return Ok(Op::Xor);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try And
            {
                let saved_pos = stream.position();
                if stream.expect_string(".and").is_ok() {
                    return Ok(Op::And);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".xor", ".and"];
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

    impl PtxParser for WmmaMmaOpPopcSyncAlignedRowColShapeS32AtypeBtypeS32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wmma")?;
            stream.expect_string(".mma")?;
            let mma = ();
            let op = Op::parse(stream)?;
            stream.expect_string(".popc")?;
            let popc = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_string(".row")?;
            let row = ();
            stream.expect_string(".col")?;
            let col = ();
            let shape = Shape::parse(stream)?;
            stream.expect_string(".s32")?;
            let s32 = ();
            let atype = Atype::parse(stream)?;
            let btype = Btype::parse(stream)?;
            stream.expect_string(".s32")?;
            let s322 = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            Ok(WmmaMmaOpPopcSyncAlignedRowColShapeS32AtypeBtypeS32 {
                mma,
                op,
                popc,
                sync,
                aligned,
                row,
                col,
                shape,
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

