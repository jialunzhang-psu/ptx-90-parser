//! Original PTX specification:
//!
//! // Half precision floating point type:
//! wgmma.mma_async.sp.sync.aligned.shape.dtype.f16.f16  d, a-desc, b-desc, sp-meta, sp-sel, scale-d, imm-scale-a, imm-scale-b, imm-trans-a, imm-trans-b;
//! wgmma.mma_async.sp.sync.aligned.shape.dtype.f16.f16  d, a, b-desc, sp-meta, sp-sel, scale-d, imm-scale-a, imm-scale-b, imm-trans-b;
//! .shape   = {.m64n8k32, .m64n16k32, .m64n24k32, .m64n32k32,
//! .m64n40k32, .m64n48k32, .m64n56k32, .m64n64k32,
//! .m64n72k32, .m64n80k32, .m64n88k32, .m64n96k32,
//! .m64n104k32, .m64n112k32, .m64n120k32, .m64n128k32,
//! .m64n136k32, .m64n144k32, .m64n152k32, .m64n160k32,
//! .m64n168k32, .m64n176k32, .m64n184k32, .m64n192k32,
//! .m64n200k32, .m64n208k32, .m64n216k32, .m64n224k32,
//! .m64n232k32, .m64n240k32, .m64n248k32, .m64n256k32};
//! .dtype   = {.f16, .f32};
//! ------------------------------------------------------------------
//! // Alternate floating point type :
//! // .bf16 floating point type:
//! wgmma.mma_async.sp.sync.aligned.shape.dtype.bf16.bf16  d, a-desc, b-desc, sp-meta, sp-sel, scale-d, imm-scale-a, imm-scale-b, imm-trans-a, imm-trans-b;
//! wgmma.mma_async.sp.sync.aligned.shape.dtype.bf16.bf16  d, a, b-desc, sp-meta, sp-sel, scale-d, imm-scale-a, imm-scale-b, imm-trans-b;
//! .shape   = {.m64n8k32, .m64n16k32, .m64n24k32, .m64n32k32,
//! .m64n40k32, .m64n48k32, .m64n56k32, .m64n64k32,
//! .m64n72k32, .m64n80k32, .m64n88k32, .m64n96k32,
//! .m64n104k32, .m64n112k32, .m64n120k32, .m64n128k32,
//! .m64n136k32, .m64n144k32, .m64n152k32, .m64n160k32,
//! .m64n168k32, .m64n176k32, .m64n184k32, .m64n192k32,
//! .m64n200k32, .m64n208k32, .m64n216k32, .m64n224k32,
//! .m64n232k32, .m64n240k32, .m64n248k32, .m64n256k32};
//! .dtype  = {.f32};
//! ------------------------------------------------------------------
//! // .tf32 floating point type:
//! wgmma.mma_async.sp.sync.aligned.shape.dtype.tf32.tf32  d, a-desc, b-desc, sp-meta, sp-sel, scale-d, imm-scale-a, imm-scale-b;
//! wgmma.mma_async.sp.sync.aligned.shape.dtype.tf32.tf32  d, a, b-desc, sp-meta, sp-sel, scale-d, imm-scale-a, imm-scale-b;
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
//! // FP8 floating point type
//! wgmma.mma_async.sp.sync.aligned.shape.dtype.atype.btype  d, a-desc, b-desc, sp-meta, sp-sel, scale-d, imm-scale-a, imm-scale-b;
//! wgmma.mma_async.sp.sync.aligned.shape.dtype.atype.btype  d, a, b-desc, sp-meta, sp-sel, scale-d, imm-scale-a, imm-scale-b;
//! .shape   = {.m64n8k64, .m64n16k64, .m64n24k64, .m64n32k64,
//! .m64n40k64, .m64n48k64, .m64n56k64, .m64n64k64,
//! .m64n72k64, .m64n80k64, .m64n88k64, .m64n96k64,
//! .m64n104k64, .m64n112k64, .m64n120k64, .m64n128k64,
//! .m64n136k64, .m64n144k64, .m64n152k64, .m64n160k64,
//! .m64n168k64, .m64n176k64, .m64n184k64, .m64n192k64,
//! .m64n200k64, .m64n208k64, .m64n216k64, .m64n224k64,
//! .m64n232k64, .m64n240k64, .m64n248k64, .m64n256k64};
//! .atype  = {.e4m3, .e5m2};
//! .btype  = {.e4m3, .e5m2};
//! .dtype  = {.f16, .f32};
//! ------------------------------------------------------------------
//! // Integer type:
//! wgmma.mma_async.sp.sync.aligned.shape{.satfinite}.s32.atype.btype  d, a-desc, b-desc, sp-meta, sp-sel, scale-d;
//! wgmma.mma_async.sp.sync.aligned.shape{.satfinite}.s32.atype.btype  d, a, b-desc, sp-meta, sp-sel, scale-d;
//! .shape   = {.m64n8k64, .m64n16k64, .m64n24k64, .m64n32k64,
//! .m64n48k64, .m64n64k64, .m64n80k64, .m64n96k64,
//! .m64n112k64, .m64n128k64, .m64n144k64, .m64n160k64,
//! .m64n176k64, .m64n192k64, .m64n208k64, .m64n224k64,
//! .m64n240k64, .m64n256k64};
//! .atype  = {.s8, .u8};
//! .btype  = {.s8, .u8};

#![allow(unused)]

use crate::parser::{
    PtxParseError, PtxParser, PtxTokenStream, Span,
    util::{
        between, comma_p, directive_p, exclamation_p, lbracket_p, lparen_p, map, minus_p, optional,
        pipe_p, rbracket_p, rparen_p, semicolon_p, sep_by, string_p, try_map,
    },
};
use crate::r#type::common::*;
use crate::{alt, ok, seq_n};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::wgmma_mma_async_sp::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".f16"), |_, _span| Dtype::F16),
                map(string_p(".f32"), |_, _span| Dtype::F32)
            )
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".m64n104k32"), |_, _span| Shape::M64n104k32),
                map(string_p(".m64n112k32"), |_, _span| Shape::M64n112k32),
                map(string_p(".m64n120k32"), |_, _span| Shape::M64n120k32),
                map(string_p(".m64n128k32"), |_, _span| Shape::M64n128k32),
                map(string_p(".m64n136k32"), |_, _span| Shape::M64n136k32),
                map(string_p(".m64n144k32"), |_, _span| Shape::M64n144k32),
                map(string_p(".m64n152k32"), |_, _span| Shape::M64n152k32),
                map(string_p(".m64n160k32"), |_, _span| Shape::M64n160k32),
                map(string_p(".m64n168k32"), |_, _span| Shape::M64n168k32),
                map(string_p(".m64n176k32"), |_, _span| Shape::M64n176k32),
                map(string_p(".m64n184k32"), |_, _span| Shape::M64n184k32),
                map(string_p(".m64n192k32"), |_, _span| Shape::M64n192k32),
                map(string_p(".m64n200k32"), |_, _span| Shape::M64n200k32),
                map(string_p(".m64n208k32"), |_, _span| Shape::M64n208k32),
                map(string_p(".m64n216k32"), |_, _span| Shape::M64n216k32),
                map(string_p(".m64n224k32"), |_, _span| Shape::M64n224k32),
                map(string_p(".m64n232k32"), |_, _span| Shape::M64n232k32),
                map(string_p(".m64n240k32"), |_, _span| Shape::M64n240k32),
                map(string_p(".m64n248k32"), |_, _span| Shape::M64n248k32),
                map(string_p(".m64n256k32"), |_, _span| Shape::M64n256k32),
                map(string_p(".m64n16k32"), |_, _span| Shape::M64n16k32),
                map(string_p(".m64n24k32"), |_, _span| Shape::M64n24k32),
                map(string_p(".m64n32k32"), |_, _span| Shape::M64n32k32),
                map(string_p(".m64n40k32"), |_, _span| Shape::M64n40k32),
                map(string_p(".m64n48k32"), |_, _span| Shape::M64n48k32),
                map(string_p(".m64n56k32"), |_, _span| Shape::M64n56k32),
                map(string_p(".m64n64k32"), |_, _span| Shape::M64n64k32),
                map(string_p(".m64n72k32"), |_, _span| Shape::M64n72k32),
                map(string_p(".m64n80k32"), |_, _span| Shape::M64n80k32),
                map(string_p(".m64n88k32"), |_, _span| Shape::M64n88k32),
                map(string_p(".m64n96k32"), |_, _span| Shape::M64n96k32),
                map(string_p(".m64n8k32"), |_, _span| Shape::M64n8k32)
            )
        }
    }

    impl PtxParser for WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wgmma"),
                    string_p(".mma_async"),
                    string_p(".sp"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape::parse(),
                    Dtype::parse(),
                    string_p(".f16"),
                    string_p(".f16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma_async,
                    sp,
                    sync,
                    aligned,
                    shape,
                    dtype,
                    f16,
                    f162,
                    d,
                    _,
                    a_desc,
                    _,
                    b_desc,
                    _,
                    sp_meta,
                    _,
                    sp_sel,
                    _,
                    scale_d,
                    _,
                    imm_scale_a,
                    _,
                    imm_scale_b,
                    _,
                    imm_trans_a,
                    _,
                    imm_trans_b,
                    _,
                ),
                 span| {
                    ok!(WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F16 {
                        mma_async = mma_async,
                        sp = sp,
                        sync = sync,
                        aligned = aligned,
                        shape = shape,
                        dtype = dtype,
                        f16 = f16,
                        f162 = f162,
                        d = d,
                        a_desc = a_desc,
                        b_desc = b_desc,
                        sp_meta = sp_meta,
                        sp_sel = sp_sel,
                        scale_d = scale_d,
                        imm_scale_a = imm_scale_a,
                        imm_scale_b = imm_scale_b,
                        imm_trans_a = imm_trans_a,
                        imm_trans_b = imm_trans_b,

                    })
                },
            )
        }
    }

    impl PtxParser for WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F161 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wgmma"),
                    string_p(".mma_async"),
                    string_p(".sp"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape::parse(),
                    Dtype::parse(),
                    string_p(".f16"),
                    string_p(".f16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma_async,
                    sp,
                    sync,
                    aligned,
                    shape,
                    dtype,
                    f16,
                    f162,
                    d,
                    _,
                    a,
                    _,
                    b_desc,
                    _,
                    sp_meta,
                    _,
                    sp_sel,
                    _,
                    scale_d,
                    _,
                    imm_scale_a,
                    _,
                    imm_scale_b,
                    _,
                    imm_trans_b,
                    _,
                ),
                 span| {
                    ok!(WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F161 {
                        mma_async = mma_async,
                        sp = sp,
                        sync = sync,
                        aligned = aligned,
                        shape = shape,
                        dtype = dtype,
                        f16 = f16,
                        f162 = f162,
                        d = d,
                        a = a,
                        b_desc = b_desc,
                        sp_meta = sp_meta,
                        sp_sel = sp_sel,
                        scale_d = scale_d,
                        imm_scale_a = imm_scale_a,
                        imm_scale_b = imm_scale_b,
                        imm_trans_b = imm_trans_b,

                    })
                },
            )
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::wgmma_mma_async_sp::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".f32"), |_, _span| Dtype::F32))
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".m64n104k32"), |_, _span| Shape::M64n104k32),
                map(string_p(".m64n112k32"), |_, _span| Shape::M64n112k32),
                map(string_p(".m64n120k32"), |_, _span| Shape::M64n120k32),
                map(string_p(".m64n128k32"), |_, _span| Shape::M64n128k32),
                map(string_p(".m64n136k32"), |_, _span| Shape::M64n136k32),
                map(string_p(".m64n144k32"), |_, _span| Shape::M64n144k32),
                map(string_p(".m64n152k32"), |_, _span| Shape::M64n152k32),
                map(string_p(".m64n160k32"), |_, _span| Shape::M64n160k32),
                map(string_p(".m64n168k32"), |_, _span| Shape::M64n168k32),
                map(string_p(".m64n176k32"), |_, _span| Shape::M64n176k32),
                map(string_p(".m64n184k32"), |_, _span| Shape::M64n184k32),
                map(string_p(".m64n192k32"), |_, _span| Shape::M64n192k32),
                map(string_p(".m64n200k32"), |_, _span| Shape::M64n200k32),
                map(string_p(".m64n208k32"), |_, _span| Shape::M64n208k32),
                map(string_p(".m64n216k32"), |_, _span| Shape::M64n216k32),
                map(string_p(".m64n224k32"), |_, _span| Shape::M64n224k32),
                map(string_p(".m64n232k32"), |_, _span| Shape::M64n232k32),
                map(string_p(".m64n240k32"), |_, _span| Shape::M64n240k32),
                map(string_p(".m64n248k32"), |_, _span| Shape::M64n248k32),
                map(string_p(".m64n256k32"), |_, _span| Shape::M64n256k32),
                map(string_p(".m64n16k32"), |_, _span| Shape::M64n16k32),
                map(string_p(".m64n24k32"), |_, _span| Shape::M64n24k32),
                map(string_p(".m64n32k32"), |_, _span| Shape::M64n32k32),
                map(string_p(".m64n40k32"), |_, _span| Shape::M64n40k32),
                map(string_p(".m64n48k32"), |_, _span| Shape::M64n48k32),
                map(string_p(".m64n56k32"), |_, _span| Shape::M64n56k32),
                map(string_p(".m64n64k32"), |_, _span| Shape::M64n64k32),
                map(string_p(".m64n72k32"), |_, _span| Shape::M64n72k32),
                map(string_p(".m64n80k32"), |_, _span| Shape::M64n80k32),
                map(string_p(".m64n88k32"), |_, _span| Shape::M64n88k32),
                map(string_p(".m64n96k32"), |_, _span| Shape::M64n96k32),
                map(string_p(".m64n8k32"), |_, _span| Shape::M64n8k32)
            )
        }
    }

    impl PtxParser for WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wgmma"),
                    string_p(".mma_async"),
                    string_p(".sp"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape::parse(),
                    Dtype::parse(),
                    string_p(".bf16"),
                    string_p(".bf16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma_async,
                    sp,
                    sync,
                    aligned,
                    shape,
                    dtype,
                    bf16,
                    bf162,
                    d,
                    _,
                    a_desc,
                    _,
                    b_desc,
                    _,
                    sp_meta,
                    _,
                    sp_sel,
                    _,
                    scale_d,
                    _,
                    imm_scale_a,
                    _,
                    imm_scale_b,
                    _,
                    imm_trans_a,
                    _,
                    imm_trans_b,
                    _,
                ),
                 span| {
                    ok!(WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf16 {
                        mma_async = mma_async,
                        sp = sp,
                        sync = sync,
                        aligned = aligned,
                        shape = shape,
                        dtype = dtype,
                        bf16 = bf16,
                        bf162 = bf162,
                        d = d,
                        a_desc = a_desc,
                        b_desc = b_desc,
                        sp_meta = sp_meta,
                        sp_sel = sp_sel,
                        scale_d = scale_d,
                        imm_scale_a = imm_scale_a,
                        imm_scale_b = imm_scale_b,
                        imm_trans_a = imm_trans_a,
                        imm_trans_b = imm_trans_b,

                    })
                },
            )
        }
    }

    impl PtxParser for WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf161 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wgmma"),
                    string_p(".mma_async"),
                    string_p(".sp"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape::parse(),
                    Dtype::parse(),
                    string_p(".bf16"),
                    string_p(".bf16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma_async,
                    sp,
                    sync,
                    aligned,
                    shape,
                    dtype,
                    bf16,
                    bf162,
                    d,
                    _,
                    a,
                    _,
                    b_desc,
                    _,
                    sp_meta,
                    _,
                    sp_sel,
                    _,
                    scale_d,
                    _,
                    imm_scale_a,
                    _,
                    imm_scale_b,
                    _,
                    imm_trans_b,
                    _,
                ),
                 span| {
                    ok!(WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf161 {
                        mma_async = mma_async,
                        sp = sp,
                        sync = sync,
                        aligned = aligned,
                        shape = shape,
                        dtype = dtype,
                        bf16 = bf16,
                        bf162 = bf162,
                        d = d,
                        a = a,
                        b_desc = b_desc,
                        sp_meta = sp_meta,
                        sp_sel = sp_sel,
                        scale_d = scale_d,
                        imm_scale_a = imm_scale_a,
                        imm_scale_b = imm_scale_b,
                        imm_trans_b = imm_trans_b,

                    })
                },
            )
        }
    }
}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::wgmma_mma_async_sp::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".f32"), |_, _span| Dtype::F32))
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".m64n104k16"), |_, _span| Shape::M64n104k16),
                map(string_p(".m64n112k16"), |_, _span| Shape::M64n112k16),
                map(string_p(".m64n120k16"), |_, _span| Shape::M64n120k16),
                map(string_p(".m64n128k16"), |_, _span| Shape::M64n128k16),
                map(string_p(".m64n136k16"), |_, _span| Shape::M64n136k16),
                map(string_p(".m64n144k16"), |_, _span| Shape::M64n144k16),
                map(string_p(".m64n152k16"), |_, _span| Shape::M64n152k16),
                map(string_p(".m64n160k16"), |_, _span| Shape::M64n160k16),
                map(string_p(".m64n168k16"), |_, _span| Shape::M64n168k16),
                map(string_p(".m64n176k16"), |_, _span| Shape::M64n176k16),
                map(string_p(".m64n184k16"), |_, _span| Shape::M64n184k16),
                map(string_p(".m64n192k16"), |_, _span| Shape::M64n192k16),
                map(string_p(".m64n200k16"), |_, _span| Shape::M64n200k16),
                map(string_p(".m64n208k16"), |_, _span| Shape::M64n208k16),
                map(string_p(".m64n216k16"), |_, _span| Shape::M64n216k16),
                map(string_p(".m64n224k16"), |_, _span| Shape::M64n224k16),
                map(string_p(".m64n232k16"), |_, _span| Shape::M64n232k16),
                map(string_p(".m64n240k16"), |_, _span| Shape::M64n240k16),
                map(string_p(".m64n248k16"), |_, _span| Shape::M64n248k16),
                map(string_p(".m64n256k16"), |_, _span| Shape::M64n256k16),
                map(string_p(".m64n16k16"), |_, _span| Shape::M64n16k16),
                map(string_p(".m64n24k16"), |_, _span| Shape::M64n24k16),
                map(string_p(".m64n32k16"), |_, _span| Shape::M64n32k16),
                map(string_p(".m64n40k16"), |_, _span| Shape::M64n40k16),
                map(string_p(".m64n48k16"), |_, _span| Shape::M64n48k16),
                map(string_p(".m64n56k16"), |_, _span| Shape::M64n56k16),
                map(string_p(".m64n64k16"), |_, _span| Shape::M64n64k16),
                map(string_p(".m64n72k16"), |_, _span| Shape::M64n72k16),
                map(string_p(".m64n80k16"), |_, _span| Shape::M64n80k16),
                map(string_p(".m64n88k16"), |_, _span| Shape::M64n88k16),
                map(string_p(".m64n96k16"), |_, _span| Shape::M64n96k16),
                map(string_p(".m64n8k16"), |_, _span| Shape::M64n8k16)
            )
        }
    }

    impl PtxParser for WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wgmma"),
                    string_p(".mma_async"),
                    string_p(".sp"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape::parse(),
                    Dtype::parse(),
                    string_p(".tf32"),
                    string_p(".tf32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma_async,
                    sp,
                    sync,
                    aligned,
                    shape,
                    dtype,
                    tf32,
                    tf322,
                    d,
                    _,
                    a_desc,
                    _,
                    b_desc,
                    _,
                    sp_meta,
                    _,
                    sp_sel,
                    _,
                    scale_d,
                    _,
                    imm_scale_a,
                    _,
                    imm_scale_b,
                    _,
                ),
                 span| {
                    ok!(WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf32 {
                        mma_async = mma_async,
                        sp = sp,
                        sync = sync,
                        aligned = aligned,
                        shape = shape,
                        dtype = dtype,
                        tf32 = tf32,
                        tf322 = tf322,
                        d = d,
                        a_desc = a_desc,
                        b_desc = b_desc,
                        sp_meta = sp_meta,
                        sp_sel = sp_sel,
                        scale_d = scale_d,
                        imm_scale_a = imm_scale_a,
                        imm_scale_b = imm_scale_b,

                    })
                },
            )
        }
    }

    impl PtxParser for WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf321 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wgmma"),
                    string_p(".mma_async"),
                    string_p(".sp"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape::parse(),
                    Dtype::parse(),
                    string_p(".tf32"),
                    string_p(".tf32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma_async,
                    sp,
                    sync,
                    aligned,
                    shape,
                    dtype,
                    tf32,
                    tf322,
                    d,
                    _,
                    a,
                    _,
                    b_desc,
                    _,
                    sp_meta,
                    _,
                    sp_sel,
                    _,
                    scale_d,
                    _,
                    imm_scale_a,
                    _,
                    imm_scale_b,
                    _,
                ),
                 span| {
                    ok!(WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf321 {
                        mma_async = mma_async,
                        sp = sp,
                        sync = sync,
                        aligned = aligned,
                        shape = shape,
                        dtype = dtype,
                        tf32 = tf32,
                        tf322 = tf322,
                        d = d,
                        a = a,
                        b_desc = b_desc,
                        sp_meta = sp_meta,
                        sp_sel = sp_sel,
                        scale_d = scale_d,
                        imm_scale_a = imm_scale_a,
                        imm_scale_b = imm_scale_b,

                    })
                },
            )
        }
    }
}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::wgmma_mma_async_sp::section_3::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".e4m3"), |_, _span| Atype::E4m3),
                map(string_p(".e5m2"), |_, _span| Atype::E5m2)
            )
        }
    }

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".e4m3"), |_, _span| Btype::E4m3),
                map(string_p(".e5m2"), |_, _span| Btype::E5m2)
            )
        }
    }

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".f16"), |_, _span| Dtype::F16),
                map(string_p(".f32"), |_, _span| Dtype::F32)
            )
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".m64n104k64"), |_, _span| Shape::M64n104k64),
                map(string_p(".m64n112k64"), |_, _span| Shape::M64n112k64),
                map(string_p(".m64n120k64"), |_, _span| Shape::M64n120k64),
                map(string_p(".m64n128k64"), |_, _span| Shape::M64n128k64),
                map(string_p(".m64n136k64"), |_, _span| Shape::M64n136k64),
                map(string_p(".m64n144k64"), |_, _span| Shape::M64n144k64),
                map(string_p(".m64n152k64"), |_, _span| Shape::M64n152k64),
                map(string_p(".m64n160k64"), |_, _span| Shape::M64n160k64),
                map(string_p(".m64n168k64"), |_, _span| Shape::M64n168k64),
                map(string_p(".m64n176k64"), |_, _span| Shape::M64n176k64),
                map(string_p(".m64n184k64"), |_, _span| Shape::M64n184k64),
                map(string_p(".m64n192k64"), |_, _span| Shape::M64n192k64),
                map(string_p(".m64n200k64"), |_, _span| Shape::M64n200k64),
                map(string_p(".m64n208k64"), |_, _span| Shape::M64n208k64),
                map(string_p(".m64n216k64"), |_, _span| Shape::M64n216k64),
                map(string_p(".m64n224k64"), |_, _span| Shape::M64n224k64),
                map(string_p(".m64n232k64"), |_, _span| Shape::M64n232k64),
                map(string_p(".m64n240k64"), |_, _span| Shape::M64n240k64),
                map(string_p(".m64n248k64"), |_, _span| Shape::M64n248k64),
                map(string_p(".m64n256k64"), |_, _span| Shape::M64n256k64),
                map(string_p(".m64n16k64"), |_, _span| Shape::M64n16k64),
                map(string_p(".m64n24k64"), |_, _span| Shape::M64n24k64),
                map(string_p(".m64n32k64"), |_, _span| Shape::M64n32k64),
                map(string_p(".m64n40k64"), |_, _span| Shape::M64n40k64),
                map(string_p(".m64n48k64"), |_, _span| Shape::M64n48k64),
                map(string_p(".m64n56k64"), |_, _span| Shape::M64n56k64),
                map(string_p(".m64n64k64"), |_, _span| Shape::M64n64k64),
                map(string_p(".m64n72k64"), |_, _span| Shape::M64n72k64),
                map(string_p(".m64n80k64"), |_, _span| Shape::M64n80k64),
                map(string_p(".m64n88k64"), |_, _span| Shape::M64n88k64),
                map(string_p(".m64n96k64"), |_, _span| Shape::M64n96k64),
                map(string_p(".m64n8k64"), |_, _span| Shape::M64n8k64)
            )
        }
    }

    impl PtxParser for WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wgmma"),
                    string_p(".mma_async"),
                    string_p(".sp"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape::parse(),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma_async,
                    sp,
                    sync,
                    aligned,
                    shape,
                    dtype,
                    atype,
                    btype,
                    d,
                    _,
                    a_desc,
                    _,
                    b_desc,
                    _,
                    sp_meta,
                    _,
                    sp_sel,
                    _,
                    scale_d,
                    _,
                    imm_scale_a,
                    _,
                    imm_scale_b,
                    _,
                ),
                 span| {
                    ok!(WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype {
                        mma_async = mma_async,
                        sp = sp,
                        sync = sync,
                        aligned = aligned,
                        shape = shape,
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        d = d,
                        a_desc = a_desc,
                        b_desc = b_desc,
                        sp_meta = sp_meta,
                        sp_sel = sp_sel,
                        scale_d = scale_d,
                        imm_scale_a = imm_scale_a,
                        imm_scale_b = imm_scale_b,

                    })
                },
            )
        }
    }

    impl PtxParser for WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wgmma"),
                    string_p(".mma_async"),
                    string_p(".sp"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape::parse(),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma_async,
                    sp,
                    sync,
                    aligned,
                    shape,
                    dtype,
                    atype,
                    btype,
                    d,
                    _,
                    a,
                    _,
                    b_desc,
                    _,
                    sp_meta,
                    _,
                    sp_sel,
                    _,
                    scale_d,
                    _,
                    imm_scale_a,
                    _,
                    imm_scale_b,
                    _,
                ),
                 span| {
                    ok!(WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype1 {
                        mma_async = mma_async,
                        sp = sp,
                        sync = sync,
                        aligned = aligned,
                        shape = shape,
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        d = d,
                        a = a,
                        b_desc = b_desc,
                        sp_meta = sp_meta,
                        sp_sel = sp_sel,
                        scale_d = scale_d,
                        imm_scale_a = imm_scale_a,
                        imm_scale_b = imm_scale_b,

                    })
                },
            )
        }
    }
}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::wgmma_mma_async_sp::section_4::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".s8"), |_, _span| Atype::S8),
                map(string_p(".u8"), |_, _span| Atype::U8)
            )
        }
    }

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".s8"), |_, _span| Btype::S8),
                map(string_p(".u8"), |_, _span| Btype::U8)
            )
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".m64n112k64"), |_, _span| Shape::M64n112k64),
                map(string_p(".m64n128k64"), |_, _span| Shape::M64n128k64),
                map(string_p(".m64n144k64"), |_, _span| Shape::M64n144k64),
                map(string_p(".m64n160k64"), |_, _span| Shape::M64n160k64),
                map(string_p(".m64n176k64"), |_, _span| Shape::M64n176k64),
                map(string_p(".m64n192k64"), |_, _span| Shape::M64n192k64),
                map(string_p(".m64n208k64"), |_, _span| Shape::M64n208k64),
                map(string_p(".m64n224k64"), |_, _span| Shape::M64n224k64),
                map(string_p(".m64n240k64"), |_, _span| Shape::M64n240k64),
                map(string_p(".m64n256k64"), |_, _span| Shape::M64n256k64),
                map(string_p(".m64n16k64"), |_, _span| Shape::M64n16k64),
                map(string_p(".m64n24k64"), |_, _span| Shape::M64n24k64),
                map(string_p(".m64n32k64"), |_, _span| Shape::M64n32k64),
                map(string_p(".m64n48k64"), |_, _span| Shape::M64n48k64),
                map(string_p(".m64n64k64"), |_, _span| Shape::M64n64k64),
                map(string_p(".m64n80k64"), |_, _span| Shape::M64n80k64),
                map(string_p(".m64n96k64"), |_, _span| Shape::M64n96k64),
                map(string_p(".m64n8k64"), |_, _span| Shape::M64n8k64)
            )
        }
    }

    impl PtxParser for WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wgmma"),
                    string_p(".mma_async"),
                    string_p(".sp"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape::parse(),
                    map(optional(string_p(".satfinite")), |value, _| value.is_some()),
                    string_p(".s32"),
                    Atype::parse(),
                    Btype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma_async,
                    sp,
                    sync,
                    aligned,
                    shape,
                    satfinite,
                    s32,
                    atype,
                    btype,
                    d,
                    _,
                    a_desc,
                    _,
                    b_desc,
                    _,
                    sp_meta,
                    _,
                    sp_sel,
                    _,
                    scale_d,
                    _,
                ),
                 span| {
                    ok!(WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype {
                        mma_async = mma_async,
                        sp = sp,
                        sync = sync,
                        aligned = aligned,
                        shape = shape,
                        satfinite = satfinite,
                        s32 = s32,
                        atype = atype,
                        btype = btype,
                        d = d,
                        a_desc = a_desc,
                        b_desc = b_desc,
                        sp_meta = sp_meta,
                        sp_sel = sp_sel,
                        scale_d = scale_d,

                    })
                },
            )
        }
    }

    impl PtxParser for WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wgmma"),
                    string_p(".mma_async"),
                    string_p(".sp"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape::parse(),
                    map(optional(string_p(".satfinite")), |value, _| value.is_some()),
                    string_p(".s32"),
                    Atype::parse(),
                    Btype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma_async,
                    sp,
                    sync,
                    aligned,
                    shape,
                    satfinite,
                    s32,
                    atype,
                    btype,
                    d,
                    _,
                    a,
                    _,
                    b_desc,
                    _,
                    sp_meta,
                    _,
                    sp_sel,
                    _,
                    scale_d,
                    _,
                ),
                 span| {
                    ok!(WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype1 {
                        mma_async = mma_async,
                        sp = sp,
                        sync = sync,
                        aligned = aligned,
                        shape = shape,
                        satfinite = satfinite,
                        s32 = s32,
                        atype = atype,
                        btype = btype,
                        d = d,
                        a = a,
                        b_desc = b_desc,
                        sp_meta = sp_meta,
                        sp_sel = sp_sel,
                        scale_d = scale_d,

                    })
                },
            )
        }
    }
}
