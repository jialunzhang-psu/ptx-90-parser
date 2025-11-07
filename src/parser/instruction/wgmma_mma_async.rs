//! Original PTX specification:
//!
//! // Half precision floating point type:
//! wgmma.mma_async.sync.aligned.shape.dtype.f16.f16  d, a-desc, b-desc, scale-d, imm-scale-a, imm-scale-b, imm-trans-a, imm-trans-b;
//! wgmma.mma_async.sync.aligned.shape.dtype.f16.f16  d, a, b-desc, scale-d, imm-scale-a, imm-scale-b, imm-trans-b;
//! .shape   = {.m64n8k16, .m64n16k16, .m64n24k16, .m64n32k16,
//! .m64n40k16, .m64n48k16, .m64n56k16, .m64n64k16,
//! .m64n72k16, .m64n80k16, .m64n88k16, .m64n96k16,
//! .m64n104k16, .m64n112k16, .m64n120k16, .m64n128k16,
//! .m64n136k16, .m64n144k16, .m64n152k16, .m64n160k16,
//! .m64n168k16, .m64n176k16, .m64n184k16, .m64n192k16,
//! .m64n200k16, .m64n208k16, .m64n216k16, .m64n224k16,
//! .m64n232k16, .m64n240k16, .m64n248k16, .m64n256k16};
//! .dtype   = {.f16, .f32};
//! ------------------------------------------------------------------
//! // Alternate floating point type :
//! // .bf16 floating point type:
//! wgmma.mma_async.sync.aligned.shape.dtype.bf16.bf16  d, a-desc, b-desc, scale-d, imm-scale-a, imm-scale-b, imm-trans-a, imm-trans-b;
//! wgmma.mma_async.sync.aligned.shape.dtype.bf16.bf16  d, a, b-desc, scale-d, imm-scale-a, imm-scale-b, imm-trans-b;
//! .shape   = {.m64n8k16, .m64n16k16, .m64n24k16, .m64n32k16,
//! .m64n40k16, .m64n48k16, .m64n56k16, .m64n64k16,
//! .m64n72k16, .m64n80k16, .m64n88k16, .m64n96k16,
//! .m64n104k16, .m64n112k16, .m64n120k16, .m64n128k16,
//! .m64n136k16, .m64n144k16, .m64n152k16, .m64n160k16,
//! .m64n168k16, .m64n176k16, .m64n184k16, .m64n192k16,
//! .m64n200k16, .m64n208k16, .m64n216k16, .m64n224k16,
//! .m64n232k16, .m64n240k16, .m64n248k16, .m64n256k16};
//! .dtype  = {.f32};
//! ------------------------------------------------------------------
//! // .tf32 floating point type:
//! wgmma.mma_async.sync.aligned.shape.dtype.tf32.tf32  d, a-desc, b-desc, scale-d, imm-scale-a, imm-scale-b;
//! wgmma.mma_async.sync.aligned.shape.dtype.tf32.tf32  d, a, b-desc, scale-d, imm-scale-a, imm-scale-b;
//! .shape   = {.m64n8k8, .m64n16k8, .m64n24k8, .m64n32k8,
//! .m64n40k8, .m64n48k8, .m64n56k8, .m64n64k8,
//! .m64n72k8, .m64n80k8, .m64n88k8, .m64n96k8,
//! .m64n104k8, .m64n112k8, .m64n120k8, .m64n128k8,
//! .m64n136k8, .m64n144k8, .m64n152k8, .m64n160k8,
//! .m64n168k8, .m64n176k8, .m64n184k8, .m64n192k8,
//! .m64n200k8, .m64n208k8, .m64n216k8, .m64n224k8,
//! .m64n232k8, .m64n240k8, .m64n248k8, .m64n256k8};
//! .dtype  = {.f32};
//! ------------------------------------------------------------------
//! // FP8 floating point type
//! wgmma.mma_async.sync.aligned.shape.dtype.atype.btype  d, a-desc, b-desc, scale-d, imm-scale-a, imm-scale-b;
//! wgmma.mma_async.sync.aligned.shape.dtype.atype.btype  d, a, b-desc, scale-d, imm-scale-a, imm-scale-b;
//! .shape   = {.m64n8k32, .m64n16k32, .m64n24k32, .m64n32k32,
//! .m64n40k32, .m64n48k32, .m64n56k32, .m64n64k32,
//! .m64n72k32, .m64n80k32, .m64n88k32, .m64n96k32,
//! .m64n104k32, .m64n112k32, .m64n120k32, .m64n128k32,
//! .m64n136k32, .m64n144k32, .m64n152k32, .m64n160k32,
//! .m64n168k32, .m64n176k32, .m64n184k32, .m64n192k32,
//! .m64n200k32, .m64n208k32, .m64n216k32, .m64n224k32,
//! .m64n232k32, .m64n240k32, .m64n248k32, .m64n256k32};
//! .atype  = {.e4m3, .e5m2};
//! .btype  = {.e4m3, .e5m2};
//! .dtype  = {.f16, .f32};
//! ------------------------------------------------------------------
//! // Integer type:
//! wgmma.mma_async.sync.aligned.shape{.satfinite}.s32.atype.btype  d, a-desc, b-desc, scale-d;
//! wgmma.mma_async.sync.aligned.shape{.satfinite}.s32.atype.btype  d, a, b-desc, scale-d;
//! .shape   = {.m64n8k32, .m64n16k32, .m64n24k32, .m64n32k32,
//! .m64n48k32, .m64n64k32, .m64n80k32, .m64n96k32,
//! .m64n112k32, .m64n128k32, .m64n144k32, .m64n160k32,
//! .m64n176k32, .m64n192k32, .m64n208k32, .m64n224k32};
//! .atype  = {.s8, .u8};
//! .btype  = {.s8, .u8};
//! ------------------------------------------------------------------
//! // Single bit:
//! wgmma.mma_async.sync.aligned.shape.s32.b1.b1.op.popc  d, a-desc, b-desc, scale-d;
//! wgmma.mma_async.sync.aligned.shape.s32.b1.b1.op.popc  d, a, b-desc, scale-d;
//! .shape   = {.m64n8k256, .m64n16k256, .m64n24k256, .m64n32k256,
//! .m64n48k256, .m64n64k256, .m64n80k256, .m64n96k256,
//! .m64n112k256, .m64n128k256, .m64n144k256, .m64n160k256,
//! .m64n176k256, .m64n192k256, .m64n208k256, .m64n224k256,
//! .m64n240k256, .m64n256k256};
//! .op  = {.and};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::wgmma_mma_async::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f16", ".f32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M64n104k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n104k16").is_ok() {
                    return Ok(Shape::M64n104k16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try M64n112k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n112k16").is_ok() {
                    return Ok(Shape::M64n112k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n120k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n120k16").is_ok() {
                    return Ok(Shape::M64n120k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n128k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n128k16").is_ok() {
                    return Ok(Shape::M64n128k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n136k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n136k16").is_ok() {
                    return Ok(Shape::M64n136k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n144k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n144k16").is_ok() {
                    return Ok(Shape::M64n144k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n152k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n152k16").is_ok() {
                    return Ok(Shape::M64n152k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n160k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n160k16").is_ok() {
                    return Ok(Shape::M64n160k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n168k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n168k16").is_ok() {
                    return Ok(Shape::M64n168k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n176k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n176k16").is_ok() {
                    return Ok(Shape::M64n176k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n184k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n184k16").is_ok() {
                    return Ok(Shape::M64n184k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n192k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n192k16").is_ok() {
                    return Ok(Shape::M64n192k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n200k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n200k16").is_ok() {
                    return Ok(Shape::M64n200k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n208k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n208k16").is_ok() {
                    return Ok(Shape::M64n208k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n216k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n216k16").is_ok() {
                    return Ok(Shape::M64n216k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n224k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n224k16").is_ok() {
                    return Ok(Shape::M64n224k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n232k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n232k16").is_ok() {
                    return Ok(Shape::M64n232k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n240k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n240k16").is_ok() {
                    return Ok(Shape::M64n240k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n248k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n248k16").is_ok() {
                    return Ok(Shape::M64n248k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n256k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n256k16").is_ok() {
                    return Ok(Shape::M64n256k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n16k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n16k16").is_ok() {
                    return Ok(Shape::M64n16k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n24k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n24k16").is_ok() {
                    return Ok(Shape::M64n24k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n32k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n32k16").is_ok() {
                    return Ok(Shape::M64n32k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n40k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n40k16").is_ok() {
                    return Ok(Shape::M64n40k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n48k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n48k16").is_ok() {
                    return Ok(Shape::M64n48k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n56k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n56k16").is_ok() {
                    return Ok(Shape::M64n56k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n64k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n64k16").is_ok() {
                    return Ok(Shape::M64n64k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n72k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n72k16").is_ok() {
                    return Ok(Shape::M64n72k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n80k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n80k16").is_ok() {
                    return Ok(Shape::M64n80k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n88k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n88k16").is_ok() {
                    return Ok(Shape::M64n88k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n96k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n96k16").is_ok() {
                    return Ok(Shape::M64n96k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n8k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n8k16").is_ok() {
                    return Ok(Shape::M64n8k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m64n104k16", ".m64n112k16", ".m64n120k16", ".m64n128k16", ".m64n136k16", ".m64n144k16", ".m64n152k16", ".m64n160k16", ".m64n168k16", ".m64n176k16", ".m64n184k16", ".m64n192k16", ".m64n200k16", ".m64n208k16", ".m64n216k16", ".m64n224k16", ".m64n232k16", ".m64n240k16", ".m64n248k16", ".m64n256k16", ".m64n16k16", ".m64n24k16", ".m64n32k16", ".m64n40k16", ".m64n48k16", ".m64n56k16", ".m64n64k16", ".m64n72k16", ".m64n80k16", ".m64n88k16", ".m64n96k16", ".m64n8k16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for WgmmaMmaAsyncSyncAlignedShapeDtypeF16F16 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wgmma")?;
            stream.expect_string(".mma_async")?;
            let mma_async = ();
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".f16")?;
            let f16 = ();
            stream.expect_complete()?;
            stream.expect_string(".f16")?;
            let f162 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_scale_a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_scale_b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_trans_a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_trans_b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(WgmmaMmaAsyncSyncAlignedShapeDtypeF16F16 {
                mma_async,
                sync,
                aligned,
                shape,
                dtype,
                f16,
                f162,
                d,
                a_desc,
                b_desc,
                scale_d,
                imm_scale_a,
                imm_scale_b,
                imm_trans_a,
                imm_trans_b,
            })
        }
    }


    impl PtxParser for WgmmaMmaAsyncSyncAlignedShapeDtypeF16F161 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wgmma")?;
            stream.expect_string(".mma_async")?;
            let mma_async = ();
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".f16")?;
            let f16 = ();
            stream.expect_complete()?;
            stream.expect_string(".f16")?;
            let f162 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_scale_a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_scale_b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_trans_b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(WgmmaMmaAsyncSyncAlignedShapeDtypeF16F161 {
                mma_async,
                sync,
                aligned,
                shape,
                dtype,
                f16,
                f162,
                d,
                a,
                b_desc,
                scale_d,
                imm_scale_a,
                imm_scale_b,
                imm_trans_b,
            })
        }
    }


}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::wgmma_mma_async::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Dtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try F32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f32").is_ok() {
                    return Ok(Dtype::F32);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M64n104k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n104k16").is_ok() {
                    return Ok(Shape::M64n104k16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try M64n112k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n112k16").is_ok() {
                    return Ok(Shape::M64n112k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n120k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n120k16").is_ok() {
                    return Ok(Shape::M64n120k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n128k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n128k16").is_ok() {
                    return Ok(Shape::M64n128k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n136k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n136k16").is_ok() {
                    return Ok(Shape::M64n136k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n144k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n144k16").is_ok() {
                    return Ok(Shape::M64n144k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n152k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n152k16").is_ok() {
                    return Ok(Shape::M64n152k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n160k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n160k16").is_ok() {
                    return Ok(Shape::M64n160k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n168k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n168k16").is_ok() {
                    return Ok(Shape::M64n168k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n176k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n176k16").is_ok() {
                    return Ok(Shape::M64n176k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n184k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n184k16").is_ok() {
                    return Ok(Shape::M64n184k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n192k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n192k16").is_ok() {
                    return Ok(Shape::M64n192k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n200k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n200k16").is_ok() {
                    return Ok(Shape::M64n200k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n208k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n208k16").is_ok() {
                    return Ok(Shape::M64n208k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n216k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n216k16").is_ok() {
                    return Ok(Shape::M64n216k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n224k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n224k16").is_ok() {
                    return Ok(Shape::M64n224k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n232k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n232k16").is_ok() {
                    return Ok(Shape::M64n232k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n240k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n240k16").is_ok() {
                    return Ok(Shape::M64n240k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n248k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n248k16").is_ok() {
                    return Ok(Shape::M64n248k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n256k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n256k16").is_ok() {
                    return Ok(Shape::M64n256k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n16k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n16k16").is_ok() {
                    return Ok(Shape::M64n16k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n24k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n24k16").is_ok() {
                    return Ok(Shape::M64n24k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n32k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n32k16").is_ok() {
                    return Ok(Shape::M64n32k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n40k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n40k16").is_ok() {
                    return Ok(Shape::M64n40k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n48k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n48k16").is_ok() {
                    return Ok(Shape::M64n48k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n56k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n56k16").is_ok() {
                    return Ok(Shape::M64n56k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n64k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n64k16").is_ok() {
                    return Ok(Shape::M64n64k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n72k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n72k16").is_ok() {
                    return Ok(Shape::M64n72k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n80k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n80k16").is_ok() {
                    return Ok(Shape::M64n80k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n88k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n88k16").is_ok() {
                    return Ok(Shape::M64n88k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n96k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n96k16").is_ok() {
                    return Ok(Shape::M64n96k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n8k16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n8k16").is_ok() {
                    return Ok(Shape::M64n8k16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m64n104k16", ".m64n112k16", ".m64n120k16", ".m64n128k16", ".m64n136k16", ".m64n144k16", ".m64n152k16", ".m64n160k16", ".m64n168k16", ".m64n176k16", ".m64n184k16", ".m64n192k16", ".m64n200k16", ".m64n208k16", ".m64n216k16", ".m64n224k16", ".m64n232k16", ".m64n240k16", ".m64n248k16", ".m64n256k16", ".m64n16k16", ".m64n24k16", ".m64n32k16", ".m64n40k16", ".m64n48k16", ".m64n56k16", ".m64n64k16", ".m64n72k16", ".m64n80k16", ".m64n88k16", ".m64n96k16", ".m64n8k16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf16 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wgmma")?;
            stream.expect_string(".mma_async")?;
            let mma_async = ();
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".bf16")?;
            let bf16 = ();
            stream.expect_complete()?;
            stream.expect_string(".bf16")?;
            let bf162 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_scale_a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_scale_b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_trans_a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_trans_b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf16 {
                mma_async,
                sync,
                aligned,
                shape,
                dtype,
                bf16,
                bf162,
                d,
                a_desc,
                b_desc,
                scale_d,
                imm_scale_a,
                imm_scale_b,
                imm_trans_a,
                imm_trans_b,
            })
        }
    }


    impl PtxParser for WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf161 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wgmma")?;
            stream.expect_string(".mma_async")?;
            let mma_async = ();
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".bf16")?;
            let bf16 = ();
            stream.expect_complete()?;
            stream.expect_string(".bf16")?;
            let bf162 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_scale_a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_scale_b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_trans_b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf161 {
                mma_async,
                sync,
                aligned,
                shape,
                dtype,
                bf16,
                bf162,
                d,
                a,
                b_desc,
                scale_d,
                imm_scale_a,
                imm_scale_b,
                imm_trans_b,
            })
        }
    }


}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::wgmma_mma_async::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Dtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try F32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f32").is_ok() {
                    return Ok(Dtype::F32);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M64n104k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n104k8").is_ok() {
                    return Ok(Shape::M64n104k8);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try M64n112k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n112k8").is_ok() {
                    return Ok(Shape::M64n112k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n120k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n120k8").is_ok() {
                    return Ok(Shape::M64n120k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n128k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n128k8").is_ok() {
                    return Ok(Shape::M64n128k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n136k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n136k8").is_ok() {
                    return Ok(Shape::M64n136k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n144k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n144k8").is_ok() {
                    return Ok(Shape::M64n144k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n152k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n152k8").is_ok() {
                    return Ok(Shape::M64n152k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n160k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n160k8").is_ok() {
                    return Ok(Shape::M64n160k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n168k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n168k8").is_ok() {
                    return Ok(Shape::M64n168k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n176k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n176k8").is_ok() {
                    return Ok(Shape::M64n176k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n184k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n184k8").is_ok() {
                    return Ok(Shape::M64n184k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n192k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n192k8").is_ok() {
                    return Ok(Shape::M64n192k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n200k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n200k8").is_ok() {
                    return Ok(Shape::M64n200k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n208k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n208k8").is_ok() {
                    return Ok(Shape::M64n208k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n216k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n216k8").is_ok() {
                    return Ok(Shape::M64n216k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n224k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n224k8").is_ok() {
                    return Ok(Shape::M64n224k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n232k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n232k8").is_ok() {
                    return Ok(Shape::M64n232k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n240k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n240k8").is_ok() {
                    return Ok(Shape::M64n240k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n248k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n248k8").is_ok() {
                    return Ok(Shape::M64n248k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n256k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n256k8").is_ok() {
                    return Ok(Shape::M64n256k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n16k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n16k8").is_ok() {
                    return Ok(Shape::M64n16k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n24k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n24k8").is_ok() {
                    return Ok(Shape::M64n24k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n32k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n32k8").is_ok() {
                    return Ok(Shape::M64n32k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n40k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n40k8").is_ok() {
                    return Ok(Shape::M64n40k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n48k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n48k8").is_ok() {
                    return Ok(Shape::M64n48k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n56k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n56k8").is_ok() {
                    return Ok(Shape::M64n56k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n64k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n64k8").is_ok() {
                    return Ok(Shape::M64n64k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n72k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n72k8").is_ok() {
                    return Ok(Shape::M64n72k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n80k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n80k8").is_ok() {
                    return Ok(Shape::M64n80k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n88k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n88k8").is_ok() {
                    return Ok(Shape::M64n88k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n96k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n96k8").is_ok() {
                    return Ok(Shape::M64n96k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n8k8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n8k8").is_ok() {
                    return Ok(Shape::M64n8k8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m64n104k8", ".m64n112k8", ".m64n120k8", ".m64n128k8", ".m64n136k8", ".m64n144k8", ".m64n152k8", ".m64n160k8", ".m64n168k8", ".m64n176k8", ".m64n184k8", ".m64n192k8", ".m64n200k8", ".m64n208k8", ".m64n216k8", ".m64n224k8", ".m64n232k8", ".m64n240k8", ".m64n248k8", ".m64n256k8", ".m64n16k8", ".m64n24k8", ".m64n32k8", ".m64n40k8", ".m64n48k8", ".m64n56k8", ".m64n64k8", ".m64n72k8", ".m64n80k8", ".m64n88k8", ".m64n96k8", ".m64n8k8"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wgmma")?;
            stream.expect_string(".mma_async")?;
            let mma_async = ();
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".tf32")?;
            let tf32 = ();
            stream.expect_complete()?;
            stream.expect_string(".tf32")?;
            let tf322 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_scale_a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_scale_b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf32 {
                mma_async,
                sync,
                aligned,
                shape,
                dtype,
                tf32,
                tf322,
                d,
                a_desc,
                b_desc,
                scale_d,
                imm_scale_a,
                imm_scale_b,
            })
        }
    }


    impl PtxParser for WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf321 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wgmma")?;
            stream.expect_string(".mma_async")?;
            let mma_async = ();
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".tf32")?;
            let tf32 = ();
            stream.expect_complete()?;
            stream.expect_string(".tf32")?;
            let tf322 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_scale_a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_scale_b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf321 {
                mma_async,
                sync,
                aligned,
                shape,
                dtype,
                tf32,
                tf322,
                d,
                a,
                b_desc,
                scale_d,
                imm_scale_a,
                imm_scale_b,
            })
        }
    }


}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::wgmma_mma_async::section_3::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try E4m3
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e4m3").is_ok() {
                    return Ok(Atype::E4m3);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try E5m2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e5m2").is_ok() {
                    return Ok(Atype::E5m2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".e4m3", ".e5m2"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Btype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try E4m3
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e4m3").is_ok() {
                    return Ok(Btype::E4m3);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try E5m2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e5m2").is_ok() {
                    return Ok(Btype::E5m2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".e4m3", ".e5m2"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f16", ".f32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M64n104k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n104k32").is_ok() {
                    return Ok(Shape::M64n104k32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try M64n112k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n112k32").is_ok() {
                    return Ok(Shape::M64n112k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n120k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n120k32").is_ok() {
                    return Ok(Shape::M64n120k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n128k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n128k32").is_ok() {
                    return Ok(Shape::M64n128k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n136k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n136k32").is_ok() {
                    return Ok(Shape::M64n136k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n144k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n144k32").is_ok() {
                    return Ok(Shape::M64n144k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n152k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n152k32").is_ok() {
                    return Ok(Shape::M64n152k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n160k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n160k32").is_ok() {
                    return Ok(Shape::M64n160k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n168k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n168k32").is_ok() {
                    return Ok(Shape::M64n168k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n176k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n176k32").is_ok() {
                    return Ok(Shape::M64n176k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n184k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n184k32").is_ok() {
                    return Ok(Shape::M64n184k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n192k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n192k32").is_ok() {
                    return Ok(Shape::M64n192k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n200k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n200k32").is_ok() {
                    return Ok(Shape::M64n200k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n208k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n208k32").is_ok() {
                    return Ok(Shape::M64n208k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n216k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n216k32").is_ok() {
                    return Ok(Shape::M64n216k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n224k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n224k32").is_ok() {
                    return Ok(Shape::M64n224k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n232k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n232k32").is_ok() {
                    return Ok(Shape::M64n232k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n240k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n240k32").is_ok() {
                    return Ok(Shape::M64n240k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n248k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n248k32").is_ok() {
                    return Ok(Shape::M64n248k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n256k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n256k32").is_ok() {
                    return Ok(Shape::M64n256k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n16k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n16k32").is_ok() {
                    return Ok(Shape::M64n16k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n24k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n24k32").is_ok() {
                    return Ok(Shape::M64n24k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n32k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n32k32").is_ok() {
                    return Ok(Shape::M64n32k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n40k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n40k32").is_ok() {
                    return Ok(Shape::M64n40k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n48k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n48k32").is_ok() {
                    return Ok(Shape::M64n48k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n56k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n56k32").is_ok() {
                    return Ok(Shape::M64n56k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n64k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n64k32").is_ok() {
                    return Ok(Shape::M64n64k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n72k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n72k32").is_ok() {
                    return Ok(Shape::M64n72k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n80k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n80k32").is_ok() {
                    return Ok(Shape::M64n80k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n88k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n88k32").is_ok() {
                    return Ok(Shape::M64n88k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n96k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n96k32").is_ok() {
                    return Ok(Shape::M64n96k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n8k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n8k32").is_ok() {
                    return Ok(Shape::M64n8k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m64n104k32", ".m64n112k32", ".m64n120k32", ".m64n128k32", ".m64n136k32", ".m64n144k32", ".m64n152k32", ".m64n160k32", ".m64n168k32", ".m64n176k32", ".m64n184k32", ".m64n192k32", ".m64n200k32", ".m64n208k32", ".m64n216k32", ".m64n224k32", ".m64n232k32", ".m64n240k32", ".m64n248k32", ".m64n256k32", ".m64n16k32", ".m64n24k32", ".m64n32k32", ".m64n40k32", ".m64n48k32", ".m64n56k32", ".m64n64k32", ".m64n72k32", ".m64n80k32", ".m64n88k32", ".m64n96k32", ".m64n8k32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wgmma")?;
            stream.expect_string(".mma_async")?;
            let mma_async = ();
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_scale_a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_scale_b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype {
                mma_async,
                sync,
                aligned,
                shape,
                dtype,
                atype,
                btype,
                d,
                a_desc,
                b_desc,
                scale_d,
                imm_scale_a,
                imm_scale_b,
            })
        }
    }


    impl PtxParser for WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wgmma")?;
            stream.expect_string(".mma_async")?;
            let mma_async = ();
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_scale_a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let imm_scale_b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype1 {
                mma_async,
                sync,
                aligned,
                shape,
                dtype,
                atype,
                btype,
                d,
                a,
                b_desc,
                scale_d,
                imm_scale_a,
                imm_scale_b,
            })
        }
    }


}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::wgmma_mma_async::section_4::*;

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
            // Try M64n112k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n112k32").is_ok() {
                    return Ok(Shape::M64n112k32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try M64n128k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n128k32").is_ok() {
                    return Ok(Shape::M64n128k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n144k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n144k32").is_ok() {
                    return Ok(Shape::M64n144k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n160k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n160k32").is_ok() {
                    return Ok(Shape::M64n160k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n176k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n176k32").is_ok() {
                    return Ok(Shape::M64n176k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n192k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n192k32").is_ok() {
                    return Ok(Shape::M64n192k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n208k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n208k32").is_ok() {
                    return Ok(Shape::M64n208k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n224k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n224k32").is_ok() {
                    return Ok(Shape::M64n224k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n16k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n16k32").is_ok() {
                    return Ok(Shape::M64n16k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n24k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n24k32").is_ok() {
                    return Ok(Shape::M64n24k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n32k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n32k32").is_ok() {
                    return Ok(Shape::M64n32k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n48k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n48k32").is_ok() {
                    return Ok(Shape::M64n48k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n64k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n64k32").is_ok() {
                    return Ok(Shape::M64n64k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n80k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n80k32").is_ok() {
                    return Ok(Shape::M64n80k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n96k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n96k32").is_ok() {
                    return Ok(Shape::M64n96k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n8k32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n8k32").is_ok() {
                    return Ok(Shape::M64n8k32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m64n112k32", ".m64n128k32", ".m64n144k32", ".m64n160k32", ".m64n176k32", ".m64n192k32", ".m64n208k32", ".m64n224k32", ".m64n16k32", ".m64n24k32", ".m64n32k32", ".m64n48k32", ".m64n64k32", ".m64n80k32", ".m64n96k32", ".m64n8k32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wgmma")?;
            stream.expect_string(".mma_async")?;
            let mma_async = ();
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
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
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype {
                mma_async,
                sync,
                aligned,
                shape,
                satfinite,
                s32,
                atype,
                btype,
                d,
                a_desc,
                b_desc,
                scale_d,
            })
        }
    }


    impl PtxParser for WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wgmma")?;
            stream.expect_string(".mma_async")?;
            let mma_async = ();
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
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
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype1 {
                mma_async,
                sync,
                aligned,
                shape,
                satfinite,
                s32,
                atype,
                btype,
                d,
                a,
                b_desc,
                scale_d,
            })
        }
    }


}

pub mod section_5 {
    use super::*;
    use crate::r#type::instruction::wgmma_mma_async::section_5::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Op {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try And
            {
                let saved_pos = stream.position();
                if stream.expect_string(".and").is_ok() {
                    return Ok(Op::And);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".and"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Shape {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try M64n112k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n112k256").is_ok() {
                    return Ok(Shape::M64n112k256);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try M64n128k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n128k256").is_ok() {
                    return Ok(Shape::M64n128k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n144k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n144k256").is_ok() {
                    return Ok(Shape::M64n144k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n160k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n160k256").is_ok() {
                    return Ok(Shape::M64n160k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n176k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n176k256").is_ok() {
                    return Ok(Shape::M64n176k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n192k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n192k256").is_ok() {
                    return Ok(Shape::M64n192k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n208k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n208k256").is_ok() {
                    return Ok(Shape::M64n208k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n224k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n224k256").is_ok() {
                    return Ok(Shape::M64n224k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n240k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n240k256").is_ok() {
                    return Ok(Shape::M64n240k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n256k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n256k256").is_ok() {
                    return Ok(Shape::M64n256k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n16k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n16k256").is_ok() {
                    return Ok(Shape::M64n16k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n24k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n24k256").is_ok() {
                    return Ok(Shape::M64n24k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n32k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n32k256").is_ok() {
                    return Ok(Shape::M64n32k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n48k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n48k256").is_ok() {
                    return Ok(Shape::M64n48k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n64k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n64k256").is_ok() {
                    return Ok(Shape::M64n64k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n80k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n80k256").is_ok() {
                    return Ok(Shape::M64n80k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n96k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n96k256").is_ok() {
                    return Ok(Shape::M64n96k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try M64n8k256
            {
                let saved_pos = stream.position();
                if stream.expect_string(".m64n8k256").is_ok() {
                    return Ok(Shape::M64n8k256);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".m64n112k256", ".m64n128k256", ".m64n144k256", ".m64n160k256", ".m64n176k256", ".m64n192k256", ".m64n208k256", ".m64n224k256", ".m64n240k256", ".m64n256k256", ".m64n16k256", ".m64n24k256", ".m64n32k256", ".m64n48k256", ".m64n64k256", ".m64n80k256", ".m64n96k256", ".m64n8k256"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wgmma")?;
            stream.expect_string(".mma_async")?;
            let mma_async = ();
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
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
            let op = Op::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".popc")?;
            let popc = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc {
                mma_async,
                sync,
                aligned,
                shape,
                s32,
                b1,
                b12,
                op,
                popc,
                d,
                a_desc,
                b_desc,
                scale_d,
            })
        }
    }


    impl PtxParser for WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wgmma")?;
            stream.expect_string(".mma_async")?;
            let mma_async = ();
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let shape = Shape::parse(stream)?;
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
            let op = Op::parse(stream)?;
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
            let b_desc = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let scale_d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc1 {
                mma_async,
                sync,
                aligned,
                shape,
                s32,
                b1,
                b12,
                op,
                popc,
                d,
                a,
                b_desc,
                scale_d,
            })
        }
    }


}

