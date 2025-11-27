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
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::wgmma_mma_async::section_0::*;

    impl PtxUnparser for WgmmaMmaAsyncSyncAlignedShapeDtypeF16F16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wgmma");
                    push_directive(tokens, "mma_async");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.shape {
                            Shape::M64n104k16 => {
                                    push_directive(tokens, "m64n104k16");
                            }
                            Shape::M64n112k16 => {
                                    push_directive(tokens, "m64n112k16");
                            }
                            Shape::M64n120k16 => {
                                    push_directive(tokens, "m64n120k16");
                            }
                            Shape::M64n128k16 => {
                                    push_directive(tokens, "m64n128k16");
                            }
                            Shape::M64n136k16 => {
                                    push_directive(tokens, "m64n136k16");
                            }
                            Shape::M64n144k16 => {
                                    push_directive(tokens, "m64n144k16");
                            }
                            Shape::M64n152k16 => {
                                    push_directive(tokens, "m64n152k16");
                            }
                            Shape::M64n160k16 => {
                                    push_directive(tokens, "m64n160k16");
                            }
                            Shape::M64n168k16 => {
                                    push_directive(tokens, "m64n168k16");
                            }
                            Shape::M64n176k16 => {
                                    push_directive(tokens, "m64n176k16");
                            }
                            Shape::M64n184k16 => {
                                    push_directive(tokens, "m64n184k16");
                            }
                            Shape::M64n192k16 => {
                                    push_directive(tokens, "m64n192k16");
                            }
                            Shape::M64n200k16 => {
                                    push_directive(tokens, "m64n200k16");
                            }
                            Shape::M64n208k16 => {
                                    push_directive(tokens, "m64n208k16");
                            }
                            Shape::M64n216k16 => {
                                    push_directive(tokens, "m64n216k16");
                            }
                            Shape::M64n224k16 => {
                                    push_directive(tokens, "m64n224k16");
                            }
                            Shape::M64n232k16 => {
                                    push_directive(tokens, "m64n232k16");
                            }
                            Shape::M64n240k16 => {
                                    push_directive(tokens, "m64n240k16");
                            }
                            Shape::M64n248k16 => {
                                    push_directive(tokens, "m64n248k16");
                            }
                            Shape::M64n256k16 => {
                                    push_directive(tokens, "m64n256k16");
                            }
                            Shape::M64n16k16 => {
                                    push_directive(tokens, "m64n16k16");
                            }
                            Shape::M64n24k16 => {
                                    push_directive(tokens, "m64n24k16");
                            }
                            Shape::M64n32k16 => {
                                    push_directive(tokens, "m64n32k16");
                            }
                            Shape::M64n40k16 => {
                                    push_directive(tokens, "m64n40k16");
                            }
                            Shape::M64n48k16 => {
                                    push_directive(tokens, "m64n48k16");
                            }
                            Shape::M64n56k16 => {
                                    push_directive(tokens, "m64n56k16");
                            }
                            Shape::M64n64k16 => {
                                    push_directive(tokens, "m64n64k16");
                            }
                            Shape::M64n72k16 => {
                                    push_directive(tokens, "m64n72k16");
                            }
                            Shape::M64n80k16 => {
                                    push_directive(tokens, "m64n80k16");
                            }
                            Shape::M64n88k16 => {
                                    push_directive(tokens, "m64n88k16");
                            }
                            Shape::M64n96k16 => {
                                    push_directive(tokens, "m64n96k16");
                            }
                            Shape::M64n8k16 => {
                                    push_directive(tokens, "m64n8k16");
                            }
                    }
                    match &self.dtype {
                            Dtype::F16 => {
                                    push_directive(tokens, "f16");
                            }
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    push_directive(tokens, "f16");
                    push_directive(tokens, "f16");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.scale_d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_scale_a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_scale_b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_trans_a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_trans_b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for WgmmaMmaAsyncSyncAlignedShapeDtypeF16F161 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wgmma");
                    push_directive(tokens, "mma_async");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.shape {
                            Shape::M64n104k16 => {
                                    push_directive(tokens, "m64n104k16");
                            }
                            Shape::M64n112k16 => {
                                    push_directive(tokens, "m64n112k16");
                            }
                            Shape::M64n120k16 => {
                                    push_directive(tokens, "m64n120k16");
                            }
                            Shape::M64n128k16 => {
                                    push_directive(tokens, "m64n128k16");
                            }
                            Shape::M64n136k16 => {
                                    push_directive(tokens, "m64n136k16");
                            }
                            Shape::M64n144k16 => {
                                    push_directive(tokens, "m64n144k16");
                            }
                            Shape::M64n152k16 => {
                                    push_directive(tokens, "m64n152k16");
                            }
                            Shape::M64n160k16 => {
                                    push_directive(tokens, "m64n160k16");
                            }
                            Shape::M64n168k16 => {
                                    push_directive(tokens, "m64n168k16");
                            }
                            Shape::M64n176k16 => {
                                    push_directive(tokens, "m64n176k16");
                            }
                            Shape::M64n184k16 => {
                                    push_directive(tokens, "m64n184k16");
                            }
                            Shape::M64n192k16 => {
                                    push_directive(tokens, "m64n192k16");
                            }
                            Shape::M64n200k16 => {
                                    push_directive(tokens, "m64n200k16");
                            }
                            Shape::M64n208k16 => {
                                    push_directive(tokens, "m64n208k16");
                            }
                            Shape::M64n216k16 => {
                                    push_directive(tokens, "m64n216k16");
                            }
                            Shape::M64n224k16 => {
                                    push_directive(tokens, "m64n224k16");
                            }
                            Shape::M64n232k16 => {
                                    push_directive(tokens, "m64n232k16");
                            }
                            Shape::M64n240k16 => {
                                    push_directive(tokens, "m64n240k16");
                            }
                            Shape::M64n248k16 => {
                                    push_directive(tokens, "m64n248k16");
                            }
                            Shape::M64n256k16 => {
                                    push_directive(tokens, "m64n256k16");
                            }
                            Shape::M64n16k16 => {
                                    push_directive(tokens, "m64n16k16");
                            }
                            Shape::M64n24k16 => {
                                    push_directive(tokens, "m64n24k16");
                            }
                            Shape::M64n32k16 => {
                                    push_directive(tokens, "m64n32k16");
                            }
                            Shape::M64n40k16 => {
                                    push_directive(tokens, "m64n40k16");
                            }
                            Shape::M64n48k16 => {
                                    push_directive(tokens, "m64n48k16");
                            }
                            Shape::M64n56k16 => {
                                    push_directive(tokens, "m64n56k16");
                            }
                            Shape::M64n64k16 => {
                                    push_directive(tokens, "m64n64k16");
                            }
                            Shape::M64n72k16 => {
                                    push_directive(tokens, "m64n72k16");
                            }
                            Shape::M64n80k16 => {
                                    push_directive(tokens, "m64n80k16");
                            }
                            Shape::M64n88k16 => {
                                    push_directive(tokens, "m64n88k16");
                            }
                            Shape::M64n96k16 => {
                                    push_directive(tokens, "m64n96k16");
                            }
                            Shape::M64n8k16 => {
                                    push_directive(tokens, "m64n8k16");
                            }
                    }
                    match &self.dtype {
                            Dtype::F16 => {
                                    push_directive(tokens, "f16");
                            }
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    push_directive(tokens, "f16");
                    push_directive(tokens, "f16");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.scale_d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_scale_a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_scale_b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_trans_b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::wgmma_mma_async::section_1::*;

    impl PtxUnparser for WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wgmma");
                    push_directive(tokens, "mma_async");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.shape {
                            Shape::M64n104k16 => {
                                    push_directive(tokens, "m64n104k16");
                            }
                            Shape::M64n112k16 => {
                                    push_directive(tokens, "m64n112k16");
                            }
                            Shape::M64n120k16 => {
                                    push_directive(tokens, "m64n120k16");
                            }
                            Shape::M64n128k16 => {
                                    push_directive(tokens, "m64n128k16");
                            }
                            Shape::M64n136k16 => {
                                    push_directive(tokens, "m64n136k16");
                            }
                            Shape::M64n144k16 => {
                                    push_directive(tokens, "m64n144k16");
                            }
                            Shape::M64n152k16 => {
                                    push_directive(tokens, "m64n152k16");
                            }
                            Shape::M64n160k16 => {
                                    push_directive(tokens, "m64n160k16");
                            }
                            Shape::M64n168k16 => {
                                    push_directive(tokens, "m64n168k16");
                            }
                            Shape::M64n176k16 => {
                                    push_directive(tokens, "m64n176k16");
                            }
                            Shape::M64n184k16 => {
                                    push_directive(tokens, "m64n184k16");
                            }
                            Shape::M64n192k16 => {
                                    push_directive(tokens, "m64n192k16");
                            }
                            Shape::M64n200k16 => {
                                    push_directive(tokens, "m64n200k16");
                            }
                            Shape::M64n208k16 => {
                                    push_directive(tokens, "m64n208k16");
                            }
                            Shape::M64n216k16 => {
                                    push_directive(tokens, "m64n216k16");
                            }
                            Shape::M64n224k16 => {
                                    push_directive(tokens, "m64n224k16");
                            }
                            Shape::M64n232k16 => {
                                    push_directive(tokens, "m64n232k16");
                            }
                            Shape::M64n240k16 => {
                                    push_directive(tokens, "m64n240k16");
                            }
                            Shape::M64n248k16 => {
                                    push_directive(tokens, "m64n248k16");
                            }
                            Shape::M64n256k16 => {
                                    push_directive(tokens, "m64n256k16");
                            }
                            Shape::M64n16k16 => {
                                    push_directive(tokens, "m64n16k16");
                            }
                            Shape::M64n24k16 => {
                                    push_directive(tokens, "m64n24k16");
                            }
                            Shape::M64n32k16 => {
                                    push_directive(tokens, "m64n32k16");
                            }
                            Shape::M64n40k16 => {
                                    push_directive(tokens, "m64n40k16");
                            }
                            Shape::M64n48k16 => {
                                    push_directive(tokens, "m64n48k16");
                            }
                            Shape::M64n56k16 => {
                                    push_directive(tokens, "m64n56k16");
                            }
                            Shape::M64n64k16 => {
                                    push_directive(tokens, "m64n64k16");
                            }
                            Shape::M64n72k16 => {
                                    push_directive(tokens, "m64n72k16");
                            }
                            Shape::M64n80k16 => {
                                    push_directive(tokens, "m64n80k16");
                            }
                            Shape::M64n88k16 => {
                                    push_directive(tokens, "m64n88k16");
                            }
                            Shape::M64n96k16 => {
                                    push_directive(tokens, "m64n96k16");
                            }
                            Shape::M64n8k16 => {
                                    push_directive(tokens, "m64n8k16");
                            }
                    }
                    match &self.dtype {
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    push_directive(tokens, "bf16");
                    push_directive(tokens, "bf16");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.scale_d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_scale_a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_scale_b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_trans_a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_trans_b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf161 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wgmma");
                    push_directive(tokens, "mma_async");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.shape {
                            Shape::M64n104k16 => {
                                    push_directive(tokens, "m64n104k16");
                            }
                            Shape::M64n112k16 => {
                                    push_directive(tokens, "m64n112k16");
                            }
                            Shape::M64n120k16 => {
                                    push_directive(tokens, "m64n120k16");
                            }
                            Shape::M64n128k16 => {
                                    push_directive(tokens, "m64n128k16");
                            }
                            Shape::M64n136k16 => {
                                    push_directive(tokens, "m64n136k16");
                            }
                            Shape::M64n144k16 => {
                                    push_directive(tokens, "m64n144k16");
                            }
                            Shape::M64n152k16 => {
                                    push_directive(tokens, "m64n152k16");
                            }
                            Shape::M64n160k16 => {
                                    push_directive(tokens, "m64n160k16");
                            }
                            Shape::M64n168k16 => {
                                    push_directive(tokens, "m64n168k16");
                            }
                            Shape::M64n176k16 => {
                                    push_directive(tokens, "m64n176k16");
                            }
                            Shape::M64n184k16 => {
                                    push_directive(tokens, "m64n184k16");
                            }
                            Shape::M64n192k16 => {
                                    push_directive(tokens, "m64n192k16");
                            }
                            Shape::M64n200k16 => {
                                    push_directive(tokens, "m64n200k16");
                            }
                            Shape::M64n208k16 => {
                                    push_directive(tokens, "m64n208k16");
                            }
                            Shape::M64n216k16 => {
                                    push_directive(tokens, "m64n216k16");
                            }
                            Shape::M64n224k16 => {
                                    push_directive(tokens, "m64n224k16");
                            }
                            Shape::M64n232k16 => {
                                    push_directive(tokens, "m64n232k16");
                            }
                            Shape::M64n240k16 => {
                                    push_directive(tokens, "m64n240k16");
                            }
                            Shape::M64n248k16 => {
                                    push_directive(tokens, "m64n248k16");
                            }
                            Shape::M64n256k16 => {
                                    push_directive(tokens, "m64n256k16");
                            }
                            Shape::M64n16k16 => {
                                    push_directive(tokens, "m64n16k16");
                            }
                            Shape::M64n24k16 => {
                                    push_directive(tokens, "m64n24k16");
                            }
                            Shape::M64n32k16 => {
                                    push_directive(tokens, "m64n32k16");
                            }
                            Shape::M64n40k16 => {
                                    push_directive(tokens, "m64n40k16");
                            }
                            Shape::M64n48k16 => {
                                    push_directive(tokens, "m64n48k16");
                            }
                            Shape::M64n56k16 => {
                                    push_directive(tokens, "m64n56k16");
                            }
                            Shape::M64n64k16 => {
                                    push_directive(tokens, "m64n64k16");
                            }
                            Shape::M64n72k16 => {
                                    push_directive(tokens, "m64n72k16");
                            }
                            Shape::M64n80k16 => {
                                    push_directive(tokens, "m64n80k16");
                            }
                            Shape::M64n88k16 => {
                                    push_directive(tokens, "m64n88k16");
                            }
                            Shape::M64n96k16 => {
                                    push_directive(tokens, "m64n96k16");
                            }
                            Shape::M64n8k16 => {
                                    push_directive(tokens, "m64n8k16");
                            }
                    }
                    match &self.dtype {
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    push_directive(tokens, "bf16");
                    push_directive(tokens, "bf16");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.scale_d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_scale_a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_scale_b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_trans_b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::wgmma_mma_async::section_2::*;

    impl PtxUnparser for WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wgmma");
                    push_directive(tokens, "mma_async");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.shape {
                            Shape::M64n104k8 => {
                                    push_directive(tokens, "m64n104k8");
                            }
                            Shape::M64n112k8 => {
                                    push_directive(tokens, "m64n112k8");
                            }
                            Shape::M64n120k8 => {
                                    push_directive(tokens, "m64n120k8");
                            }
                            Shape::M64n128k8 => {
                                    push_directive(tokens, "m64n128k8");
                            }
                            Shape::M64n136k8 => {
                                    push_directive(tokens, "m64n136k8");
                            }
                            Shape::M64n144k8 => {
                                    push_directive(tokens, "m64n144k8");
                            }
                            Shape::M64n152k8 => {
                                    push_directive(tokens, "m64n152k8");
                            }
                            Shape::M64n160k8 => {
                                    push_directive(tokens, "m64n160k8");
                            }
                            Shape::M64n168k8 => {
                                    push_directive(tokens, "m64n168k8");
                            }
                            Shape::M64n176k8 => {
                                    push_directive(tokens, "m64n176k8");
                            }
                            Shape::M64n184k8 => {
                                    push_directive(tokens, "m64n184k8");
                            }
                            Shape::M64n192k8 => {
                                    push_directive(tokens, "m64n192k8");
                            }
                            Shape::M64n200k8 => {
                                    push_directive(tokens, "m64n200k8");
                            }
                            Shape::M64n208k8 => {
                                    push_directive(tokens, "m64n208k8");
                            }
                            Shape::M64n216k8 => {
                                    push_directive(tokens, "m64n216k8");
                            }
                            Shape::M64n224k8 => {
                                    push_directive(tokens, "m64n224k8");
                            }
                            Shape::M64n232k8 => {
                                    push_directive(tokens, "m64n232k8");
                            }
                            Shape::M64n240k8 => {
                                    push_directive(tokens, "m64n240k8");
                            }
                            Shape::M64n248k8 => {
                                    push_directive(tokens, "m64n248k8");
                            }
                            Shape::M64n256k8 => {
                                    push_directive(tokens, "m64n256k8");
                            }
                            Shape::M64n16k8 => {
                                    push_directive(tokens, "m64n16k8");
                            }
                            Shape::M64n24k8 => {
                                    push_directive(tokens, "m64n24k8");
                            }
                            Shape::M64n32k8 => {
                                    push_directive(tokens, "m64n32k8");
                            }
                            Shape::M64n40k8 => {
                                    push_directive(tokens, "m64n40k8");
                            }
                            Shape::M64n48k8 => {
                                    push_directive(tokens, "m64n48k8");
                            }
                            Shape::M64n56k8 => {
                                    push_directive(tokens, "m64n56k8");
                            }
                            Shape::M64n64k8 => {
                                    push_directive(tokens, "m64n64k8");
                            }
                            Shape::M64n72k8 => {
                                    push_directive(tokens, "m64n72k8");
                            }
                            Shape::M64n80k8 => {
                                    push_directive(tokens, "m64n80k8");
                            }
                            Shape::M64n88k8 => {
                                    push_directive(tokens, "m64n88k8");
                            }
                            Shape::M64n96k8 => {
                                    push_directive(tokens, "m64n96k8");
                            }
                            Shape::M64n8k8 => {
                                    push_directive(tokens, "m64n8k8");
                            }
                    }
                    match &self.dtype {
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    push_directive(tokens, "tf32");
                    push_directive(tokens, "tf32");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.scale_d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_scale_a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_scale_b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf321 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wgmma");
                    push_directive(tokens, "mma_async");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.shape {
                            Shape::M64n104k8 => {
                                    push_directive(tokens, "m64n104k8");
                            }
                            Shape::M64n112k8 => {
                                    push_directive(tokens, "m64n112k8");
                            }
                            Shape::M64n120k8 => {
                                    push_directive(tokens, "m64n120k8");
                            }
                            Shape::M64n128k8 => {
                                    push_directive(tokens, "m64n128k8");
                            }
                            Shape::M64n136k8 => {
                                    push_directive(tokens, "m64n136k8");
                            }
                            Shape::M64n144k8 => {
                                    push_directive(tokens, "m64n144k8");
                            }
                            Shape::M64n152k8 => {
                                    push_directive(tokens, "m64n152k8");
                            }
                            Shape::M64n160k8 => {
                                    push_directive(tokens, "m64n160k8");
                            }
                            Shape::M64n168k8 => {
                                    push_directive(tokens, "m64n168k8");
                            }
                            Shape::M64n176k8 => {
                                    push_directive(tokens, "m64n176k8");
                            }
                            Shape::M64n184k8 => {
                                    push_directive(tokens, "m64n184k8");
                            }
                            Shape::M64n192k8 => {
                                    push_directive(tokens, "m64n192k8");
                            }
                            Shape::M64n200k8 => {
                                    push_directive(tokens, "m64n200k8");
                            }
                            Shape::M64n208k8 => {
                                    push_directive(tokens, "m64n208k8");
                            }
                            Shape::M64n216k8 => {
                                    push_directive(tokens, "m64n216k8");
                            }
                            Shape::M64n224k8 => {
                                    push_directive(tokens, "m64n224k8");
                            }
                            Shape::M64n232k8 => {
                                    push_directive(tokens, "m64n232k8");
                            }
                            Shape::M64n240k8 => {
                                    push_directive(tokens, "m64n240k8");
                            }
                            Shape::M64n248k8 => {
                                    push_directive(tokens, "m64n248k8");
                            }
                            Shape::M64n256k8 => {
                                    push_directive(tokens, "m64n256k8");
                            }
                            Shape::M64n16k8 => {
                                    push_directive(tokens, "m64n16k8");
                            }
                            Shape::M64n24k8 => {
                                    push_directive(tokens, "m64n24k8");
                            }
                            Shape::M64n32k8 => {
                                    push_directive(tokens, "m64n32k8");
                            }
                            Shape::M64n40k8 => {
                                    push_directive(tokens, "m64n40k8");
                            }
                            Shape::M64n48k8 => {
                                    push_directive(tokens, "m64n48k8");
                            }
                            Shape::M64n56k8 => {
                                    push_directive(tokens, "m64n56k8");
                            }
                            Shape::M64n64k8 => {
                                    push_directive(tokens, "m64n64k8");
                            }
                            Shape::M64n72k8 => {
                                    push_directive(tokens, "m64n72k8");
                            }
                            Shape::M64n80k8 => {
                                    push_directive(tokens, "m64n80k8");
                            }
                            Shape::M64n88k8 => {
                                    push_directive(tokens, "m64n88k8");
                            }
                            Shape::M64n96k8 => {
                                    push_directive(tokens, "m64n96k8");
                            }
                            Shape::M64n8k8 => {
                                    push_directive(tokens, "m64n8k8");
                            }
                    }
                    match &self.dtype {
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    push_directive(tokens, "tf32");
                    push_directive(tokens, "tf32");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.scale_d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_scale_a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_scale_b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::wgmma_mma_async::section_3::*;

    impl PtxUnparser for WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wgmma");
                    push_directive(tokens, "mma_async");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.shape {
                            Shape::M64n104k32 => {
                                    push_directive(tokens, "m64n104k32");
                            }
                            Shape::M64n112k32 => {
                                    push_directive(tokens, "m64n112k32");
                            }
                            Shape::M64n120k32 => {
                                    push_directive(tokens, "m64n120k32");
                            }
                            Shape::M64n128k32 => {
                                    push_directive(tokens, "m64n128k32");
                            }
                            Shape::M64n136k32 => {
                                    push_directive(tokens, "m64n136k32");
                            }
                            Shape::M64n144k32 => {
                                    push_directive(tokens, "m64n144k32");
                            }
                            Shape::M64n152k32 => {
                                    push_directive(tokens, "m64n152k32");
                            }
                            Shape::M64n160k32 => {
                                    push_directive(tokens, "m64n160k32");
                            }
                            Shape::M64n168k32 => {
                                    push_directive(tokens, "m64n168k32");
                            }
                            Shape::M64n176k32 => {
                                    push_directive(tokens, "m64n176k32");
                            }
                            Shape::M64n184k32 => {
                                    push_directive(tokens, "m64n184k32");
                            }
                            Shape::M64n192k32 => {
                                    push_directive(tokens, "m64n192k32");
                            }
                            Shape::M64n200k32 => {
                                    push_directive(tokens, "m64n200k32");
                            }
                            Shape::M64n208k32 => {
                                    push_directive(tokens, "m64n208k32");
                            }
                            Shape::M64n216k32 => {
                                    push_directive(tokens, "m64n216k32");
                            }
                            Shape::M64n224k32 => {
                                    push_directive(tokens, "m64n224k32");
                            }
                            Shape::M64n232k32 => {
                                    push_directive(tokens, "m64n232k32");
                            }
                            Shape::M64n240k32 => {
                                    push_directive(tokens, "m64n240k32");
                            }
                            Shape::M64n248k32 => {
                                    push_directive(tokens, "m64n248k32");
                            }
                            Shape::M64n256k32 => {
                                    push_directive(tokens, "m64n256k32");
                            }
                            Shape::M64n16k32 => {
                                    push_directive(tokens, "m64n16k32");
                            }
                            Shape::M64n24k32 => {
                                    push_directive(tokens, "m64n24k32");
                            }
                            Shape::M64n32k32 => {
                                    push_directive(tokens, "m64n32k32");
                            }
                            Shape::M64n40k32 => {
                                    push_directive(tokens, "m64n40k32");
                            }
                            Shape::M64n48k32 => {
                                    push_directive(tokens, "m64n48k32");
                            }
                            Shape::M64n56k32 => {
                                    push_directive(tokens, "m64n56k32");
                            }
                            Shape::M64n64k32 => {
                                    push_directive(tokens, "m64n64k32");
                            }
                            Shape::M64n72k32 => {
                                    push_directive(tokens, "m64n72k32");
                            }
                            Shape::M64n80k32 => {
                                    push_directive(tokens, "m64n80k32");
                            }
                            Shape::M64n88k32 => {
                                    push_directive(tokens, "m64n88k32");
                            }
                            Shape::M64n96k32 => {
                                    push_directive(tokens, "m64n96k32");
                            }
                            Shape::M64n8k32 => {
                                    push_directive(tokens, "m64n8k32");
                            }
                    }
                    match &self.dtype {
                            Dtype::F16 => {
                                    push_directive(tokens, "f16");
                            }
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    match &self.atype {
                            Atype::E4m3 => {
                                    push_directive(tokens, "e4m3");
                            }
                            Atype::E5m2 => {
                                    push_directive(tokens, "e5m2");
                            }
                    }
                    match &self.btype {
                            Btype::E4m3 => {
                                    push_directive(tokens, "e4m3");
                            }
                            Btype::E5m2 => {
                                    push_directive(tokens, "e5m2");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.scale_d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_scale_a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_scale_b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wgmma");
                    push_directive(tokens, "mma_async");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.shape {
                            Shape::M64n104k32 => {
                                    push_directive(tokens, "m64n104k32");
                            }
                            Shape::M64n112k32 => {
                                    push_directive(tokens, "m64n112k32");
                            }
                            Shape::M64n120k32 => {
                                    push_directive(tokens, "m64n120k32");
                            }
                            Shape::M64n128k32 => {
                                    push_directive(tokens, "m64n128k32");
                            }
                            Shape::M64n136k32 => {
                                    push_directive(tokens, "m64n136k32");
                            }
                            Shape::M64n144k32 => {
                                    push_directive(tokens, "m64n144k32");
                            }
                            Shape::M64n152k32 => {
                                    push_directive(tokens, "m64n152k32");
                            }
                            Shape::M64n160k32 => {
                                    push_directive(tokens, "m64n160k32");
                            }
                            Shape::M64n168k32 => {
                                    push_directive(tokens, "m64n168k32");
                            }
                            Shape::M64n176k32 => {
                                    push_directive(tokens, "m64n176k32");
                            }
                            Shape::M64n184k32 => {
                                    push_directive(tokens, "m64n184k32");
                            }
                            Shape::M64n192k32 => {
                                    push_directive(tokens, "m64n192k32");
                            }
                            Shape::M64n200k32 => {
                                    push_directive(tokens, "m64n200k32");
                            }
                            Shape::M64n208k32 => {
                                    push_directive(tokens, "m64n208k32");
                            }
                            Shape::M64n216k32 => {
                                    push_directive(tokens, "m64n216k32");
                            }
                            Shape::M64n224k32 => {
                                    push_directive(tokens, "m64n224k32");
                            }
                            Shape::M64n232k32 => {
                                    push_directive(tokens, "m64n232k32");
                            }
                            Shape::M64n240k32 => {
                                    push_directive(tokens, "m64n240k32");
                            }
                            Shape::M64n248k32 => {
                                    push_directive(tokens, "m64n248k32");
                            }
                            Shape::M64n256k32 => {
                                    push_directive(tokens, "m64n256k32");
                            }
                            Shape::M64n16k32 => {
                                    push_directive(tokens, "m64n16k32");
                            }
                            Shape::M64n24k32 => {
                                    push_directive(tokens, "m64n24k32");
                            }
                            Shape::M64n32k32 => {
                                    push_directive(tokens, "m64n32k32");
                            }
                            Shape::M64n40k32 => {
                                    push_directive(tokens, "m64n40k32");
                            }
                            Shape::M64n48k32 => {
                                    push_directive(tokens, "m64n48k32");
                            }
                            Shape::M64n56k32 => {
                                    push_directive(tokens, "m64n56k32");
                            }
                            Shape::M64n64k32 => {
                                    push_directive(tokens, "m64n64k32");
                            }
                            Shape::M64n72k32 => {
                                    push_directive(tokens, "m64n72k32");
                            }
                            Shape::M64n80k32 => {
                                    push_directive(tokens, "m64n80k32");
                            }
                            Shape::M64n88k32 => {
                                    push_directive(tokens, "m64n88k32");
                            }
                            Shape::M64n96k32 => {
                                    push_directive(tokens, "m64n96k32");
                            }
                            Shape::M64n8k32 => {
                                    push_directive(tokens, "m64n8k32");
                            }
                    }
                    match &self.dtype {
                            Dtype::F16 => {
                                    push_directive(tokens, "f16");
                            }
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    match &self.atype {
                            Atype::E4m3 => {
                                    push_directive(tokens, "e4m3");
                            }
                            Atype::E5m2 => {
                                    push_directive(tokens, "e5m2");
                            }
                    }
                    match &self.btype {
                            Btype::E4m3 => {
                                    push_directive(tokens, "e4m3");
                            }
                            Btype::E5m2 => {
                                    push_directive(tokens, "e5m2");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.scale_d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_scale_a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.imm_scale_b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::wgmma_mma_async::section_4::*;

    impl PtxUnparser for WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wgmma");
                    push_directive(tokens, "mma_async");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.shape {
                            Shape::M64n112k32 => {
                                    push_directive(tokens, "m64n112k32");
                            }
                            Shape::M64n128k32 => {
                                    push_directive(tokens, "m64n128k32");
                            }
                            Shape::M64n144k32 => {
                                    push_directive(tokens, "m64n144k32");
                            }
                            Shape::M64n160k32 => {
                                    push_directive(tokens, "m64n160k32");
                            }
                            Shape::M64n176k32 => {
                                    push_directive(tokens, "m64n176k32");
                            }
                            Shape::M64n192k32 => {
                                    push_directive(tokens, "m64n192k32");
                            }
                            Shape::M64n208k32 => {
                                    push_directive(tokens, "m64n208k32");
                            }
                            Shape::M64n224k32 => {
                                    push_directive(tokens, "m64n224k32");
                            }
                            Shape::M64n16k32 => {
                                    push_directive(tokens, "m64n16k32");
                            }
                            Shape::M64n24k32 => {
                                    push_directive(tokens, "m64n24k32");
                            }
                            Shape::M64n32k32 => {
                                    push_directive(tokens, "m64n32k32");
                            }
                            Shape::M64n48k32 => {
                                    push_directive(tokens, "m64n48k32");
                            }
                            Shape::M64n64k32 => {
                                    push_directive(tokens, "m64n64k32");
                            }
                            Shape::M64n80k32 => {
                                    push_directive(tokens, "m64n80k32");
                            }
                            Shape::M64n96k32 => {
                                    push_directive(tokens, "m64n96k32");
                            }
                            Shape::M64n8k32 => {
                                    push_directive(tokens, "m64n8k32");
                            }
                    }
                    if self.satfinite {
                            push_directive(tokens, "satfinite");
                    }
                    push_directive(tokens, "s32");
                    match &self.atype {
                            Atype::S8 => {
                                    push_directive(tokens, "s8");
                            }
                            Atype::U8 => {
                                    push_directive(tokens, "u8");
                            }
                    }
                    match &self.btype {
                            Btype::S8 => {
                                    push_directive(tokens, "s8");
                            }
                            Btype::U8 => {
                                    push_directive(tokens, "u8");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.scale_d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wgmma");
                    push_directive(tokens, "mma_async");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.shape {
                            Shape::M64n112k32 => {
                                    push_directive(tokens, "m64n112k32");
                            }
                            Shape::M64n128k32 => {
                                    push_directive(tokens, "m64n128k32");
                            }
                            Shape::M64n144k32 => {
                                    push_directive(tokens, "m64n144k32");
                            }
                            Shape::M64n160k32 => {
                                    push_directive(tokens, "m64n160k32");
                            }
                            Shape::M64n176k32 => {
                                    push_directive(tokens, "m64n176k32");
                            }
                            Shape::M64n192k32 => {
                                    push_directive(tokens, "m64n192k32");
                            }
                            Shape::M64n208k32 => {
                                    push_directive(tokens, "m64n208k32");
                            }
                            Shape::M64n224k32 => {
                                    push_directive(tokens, "m64n224k32");
                            }
                            Shape::M64n16k32 => {
                                    push_directive(tokens, "m64n16k32");
                            }
                            Shape::M64n24k32 => {
                                    push_directive(tokens, "m64n24k32");
                            }
                            Shape::M64n32k32 => {
                                    push_directive(tokens, "m64n32k32");
                            }
                            Shape::M64n48k32 => {
                                    push_directive(tokens, "m64n48k32");
                            }
                            Shape::M64n64k32 => {
                                    push_directive(tokens, "m64n64k32");
                            }
                            Shape::M64n80k32 => {
                                    push_directive(tokens, "m64n80k32");
                            }
                            Shape::M64n96k32 => {
                                    push_directive(tokens, "m64n96k32");
                            }
                            Shape::M64n8k32 => {
                                    push_directive(tokens, "m64n8k32");
                            }
                    }
                    if self.satfinite {
                            push_directive(tokens, "satfinite");
                    }
                    push_directive(tokens, "s32");
                    match &self.atype {
                            Atype::S8 => {
                                    push_directive(tokens, "s8");
                            }
                            Atype::U8 => {
                                    push_directive(tokens, "u8");
                            }
                    }
                    match &self.btype {
                            Btype::S8 => {
                                    push_directive(tokens, "s8");
                            }
                            Btype::U8 => {
                                    push_directive(tokens, "u8");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.scale_d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

pub mod section_5 {
    use super::*;
    use crate::r#type::instruction::wgmma_mma_async::section_5::*;

    impl PtxUnparser for WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wgmma");
                    push_directive(tokens, "mma_async");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.shape {
                            Shape::M64n112k256 => {
                                    push_directive(tokens, "m64n112k256");
                            }
                            Shape::M64n128k256 => {
                                    push_directive(tokens, "m64n128k256");
                            }
                            Shape::M64n144k256 => {
                                    push_directive(tokens, "m64n144k256");
                            }
                            Shape::M64n160k256 => {
                                    push_directive(tokens, "m64n160k256");
                            }
                            Shape::M64n176k256 => {
                                    push_directive(tokens, "m64n176k256");
                            }
                            Shape::M64n192k256 => {
                                    push_directive(tokens, "m64n192k256");
                            }
                            Shape::M64n208k256 => {
                                    push_directive(tokens, "m64n208k256");
                            }
                            Shape::M64n224k256 => {
                                    push_directive(tokens, "m64n224k256");
                            }
                            Shape::M64n240k256 => {
                                    push_directive(tokens, "m64n240k256");
                            }
                            Shape::M64n256k256 => {
                                    push_directive(tokens, "m64n256k256");
                            }
                            Shape::M64n16k256 => {
                                    push_directive(tokens, "m64n16k256");
                            }
                            Shape::M64n24k256 => {
                                    push_directive(tokens, "m64n24k256");
                            }
                            Shape::M64n32k256 => {
                                    push_directive(tokens, "m64n32k256");
                            }
                            Shape::M64n48k256 => {
                                    push_directive(tokens, "m64n48k256");
                            }
                            Shape::M64n64k256 => {
                                    push_directive(tokens, "m64n64k256");
                            }
                            Shape::M64n80k256 => {
                                    push_directive(tokens, "m64n80k256");
                            }
                            Shape::M64n96k256 => {
                                    push_directive(tokens, "m64n96k256");
                            }
                            Shape::M64n8k256 => {
                                    push_directive(tokens, "m64n8k256");
                            }
                    }
                    push_directive(tokens, "s32");
                    push_directive(tokens, "b1");
                    push_directive(tokens, "b1");
                    match &self.op {
                            Op::And => {
                                    push_directive(tokens, "and");
                            }
                    }
                    push_directive(tokens, "popc");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.scale_d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wgmma");
                    push_directive(tokens, "mma_async");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.shape {
                            Shape::M64n112k256 => {
                                    push_directive(tokens, "m64n112k256");
                            }
                            Shape::M64n128k256 => {
                                    push_directive(tokens, "m64n128k256");
                            }
                            Shape::M64n144k256 => {
                                    push_directive(tokens, "m64n144k256");
                            }
                            Shape::M64n160k256 => {
                                    push_directive(tokens, "m64n160k256");
                            }
                            Shape::M64n176k256 => {
                                    push_directive(tokens, "m64n176k256");
                            }
                            Shape::M64n192k256 => {
                                    push_directive(tokens, "m64n192k256");
                            }
                            Shape::M64n208k256 => {
                                    push_directive(tokens, "m64n208k256");
                            }
                            Shape::M64n224k256 => {
                                    push_directive(tokens, "m64n224k256");
                            }
                            Shape::M64n240k256 => {
                                    push_directive(tokens, "m64n240k256");
                            }
                            Shape::M64n256k256 => {
                                    push_directive(tokens, "m64n256k256");
                            }
                            Shape::M64n16k256 => {
                                    push_directive(tokens, "m64n16k256");
                            }
                            Shape::M64n24k256 => {
                                    push_directive(tokens, "m64n24k256");
                            }
                            Shape::M64n32k256 => {
                                    push_directive(tokens, "m64n32k256");
                            }
                            Shape::M64n48k256 => {
                                    push_directive(tokens, "m64n48k256");
                            }
                            Shape::M64n64k256 => {
                                    push_directive(tokens, "m64n64k256");
                            }
                            Shape::M64n80k256 => {
                                    push_directive(tokens, "m64n80k256");
                            }
                            Shape::M64n96k256 => {
                                    push_directive(tokens, "m64n96k256");
                            }
                            Shape::M64n8k256 => {
                                    push_directive(tokens, "m64n8k256");
                            }
                    }
                    push_directive(tokens, "s32");
                    push_directive(tokens, "b1");
                    push_directive(tokens, "b1");
                    match &self.op {
                            Op::And => {
                                    push_directive(tokens, "and");
                            }
                    }
                    push_directive(tokens, "popc");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b_desc.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.scale_d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

