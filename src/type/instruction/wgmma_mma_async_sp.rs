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
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Shape {
        M64n104k32, // .m64n104k32
        M64n112k32, // .m64n112k32
        M64n120k32, // .m64n120k32
        M64n128k32, // .m64n128k32
        M64n136k32, // .m64n136k32
        M64n144k32, // .m64n144k32
        M64n152k32, // .m64n152k32
        M64n160k32, // .m64n160k32
        M64n168k32, // .m64n168k32
        M64n176k32, // .m64n176k32
        M64n184k32, // .m64n184k32
        M64n192k32, // .m64n192k32
        M64n200k32, // .m64n200k32
        M64n208k32, // .m64n208k32
        M64n216k32, // .m64n216k32
        M64n224k32, // .m64n224k32
        M64n232k32, // .m64n232k32
        M64n240k32, // .m64n240k32
        M64n248k32, // .m64n248k32
        M64n256k32, // .m64n256k32
        M64n16k32,  // .m64n16k32
        M64n24k32,  // .m64n24k32
        M64n32k32,  // .m64n32k32
        M64n40k32,  // .m64n40k32
        M64n48k32,  // .m64n48k32
        M64n56k32,  // .m64n56k32
        M64n64k32,  // .m64n64k32
        M64n72k32,  // .m64n72k32
        M64n80k32,  // .m64n80k32
        M64n88k32,  // .m64n88k32
        M64n96k32,  // .m64n96k32
        M64n8k32,   // .m64n8k32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Dtype {
        F16, // .f16
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F16 {
        pub mma_async: (),               // .mma_async
        pub sp: (),                      // .sp
        pub sync: (),                    // .sync
        pub aligned: (),                 // .aligned
        pub shape: Shape,                // .shape
        pub dtype: Dtype,                // .dtype
        pub f16: (),                     // .f16
        pub f162: (),                    // .f16
        pub d: GeneralOperand,           // d
        pub a_desc: GeneralOperand,      // a-desc
        pub b_desc: GeneralOperand,      // b-desc
        pub sp_meta: GeneralOperand,     // sp-meta
        pub sp_sel: GeneralOperand,      // sp-sel
        pub scale_d: GeneralOperand,     // scale-d
        pub imm_scale_a: GeneralOperand, // imm-scale-a
        pub imm_scale_b: GeneralOperand, // imm-scale-b
        pub imm_trans_a: GeneralOperand, // imm-trans-a
        pub imm_trans_b: GeneralOperand, // imm-trans-b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F161 {
        pub mma_async: (),               // .mma_async
        pub sp: (),                      // .sp
        pub sync: (),                    // .sync
        pub aligned: (),                 // .aligned
        pub shape: Shape,                // .shape
        pub dtype: Dtype,                // .dtype
        pub f16: (),                     // .f16
        pub f162: (),                    // .f16
        pub d: GeneralOperand,           // d
        pub a: GeneralOperand,           // a
        pub b_desc: GeneralOperand,      // b-desc
        pub sp_meta: GeneralOperand,     // sp-meta
        pub sp_sel: GeneralOperand,      // sp-sel
        pub scale_d: GeneralOperand,     // scale-d
        pub imm_scale_a: GeneralOperand, // imm-scale-a
        pub imm_scale_b: GeneralOperand, // imm-scale-b
        pub imm_trans_b: GeneralOperand, // imm-trans-b
        pub span: Span,
    }
}

pub mod section_1 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Shape {
        M64n104k32, // .m64n104k32
        M64n112k32, // .m64n112k32
        M64n120k32, // .m64n120k32
        M64n128k32, // .m64n128k32
        M64n136k32, // .m64n136k32
        M64n144k32, // .m64n144k32
        M64n152k32, // .m64n152k32
        M64n160k32, // .m64n160k32
        M64n168k32, // .m64n168k32
        M64n176k32, // .m64n176k32
        M64n184k32, // .m64n184k32
        M64n192k32, // .m64n192k32
        M64n200k32, // .m64n200k32
        M64n208k32, // .m64n208k32
        M64n216k32, // .m64n216k32
        M64n224k32, // .m64n224k32
        M64n232k32, // .m64n232k32
        M64n240k32, // .m64n240k32
        M64n248k32, // .m64n248k32
        M64n256k32, // .m64n256k32
        M64n16k32,  // .m64n16k32
        M64n24k32,  // .m64n24k32
        M64n32k32,  // .m64n32k32
        M64n40k32,  // .m64n40k32
        M64n48k32,  // .m64n48k32
        M64n56k32,  // .m64n56k32
        M64n64k32,  // .m64n64k32
        M64n72k32,  // .m64n72k32
        M64n80k32,  // .m64n80k32
        M64n88k32,  // .m64n88k32
        M64n96k32,  // .m64n96k32
        M64n8k32,   // .m64n8k32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Dtype {
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf16 {
        pub mma_async: (),               // .mma_async
        pub sp: (),                      // .sp
        pub sync: (),                    // .sync
        pub aligned: (),                 // .aligned
        pub shape: Shape,                // .shape
        pub dtype: Dtype,                // .dtype
        pub bf16: (),                    // .bf16
        pub bf162: (),                   // .bf16
        pub d: GeneralOperand,           // d
        pub a_desc: GeneralOperand,      // a-desc
        pub b_desc: GeneralOperand,      // b-desc
        pub sp_meta: GeneralOperand,     // sp-meta
        pub sp_sel: GeneralOperand,      // sp-sel
        pub scale_d: GeneralOperand,     // scale-d
        pub imm_scale_a: GeneralOperand, // imm-scale-a
        pub imm_scale_b: GeneralOperand, // imm-scale-b
        pub imm_trans_a: GeneralOperand, // imm-trans-a
        pub imm_trans_b: GeneralOperand, // imm-trans-b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf161 {
        pub mma_async: (),               // .mma_async
        pub sp: (),                      // .sp
        pub sync: (),                    // .sync
        pub aligned: (),                 // .aligned
        pub shape: Shape,                // .shape
        pub dtype: Dtype,                // .dtype
        pub bf16: (),                    // .bf16
        pub bf162: (),                   // .bf16
        pub d: GeneralOperand,           // d
        pub a: GeneralOperand,           // a
        pub b_desc: GeneralOperand,      // b-desc
        pub sp_meta: GeneralOperand,     // sp-meta
        pub sp_sel: GeneralOperand,      // sp-sel
        pub scale_d: GeneralOperand,     // scale-d
        pub imm_scale_a: GeneralOperand, // imm-scale-a
        pub imm_scale_b: GeneralOperand, // imm-scale-b
        pub imm_trans_b: GeneralOperand, // imm-trans-b
        pub span: Span,
    }
}

pub mod section_2 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Shape {
        M64n104k16, // .m64n104k16
        M64n112k16, // .m64n112k16
        M64n120k16, // .m64n120k16
        M64n128k16, // .m64n128k16
        M64n136k16, // .m64n136k16
        M64n144k16, // .m64n144k16
        M64n152k16, // .m64n152k16
        M64n160k16, // .m64n160k16
        M64n168k16, // .m64n168k16
        M64n176k16, // .m64n176k16
        M64n184k16, // .m64n184k16
        M64n192k16, // .m64n192k16
        M64n200k16, // .m64n200k16
        M64n208k16, // .m64n208k16
        M64n216k16, // .m64n216k16
        M64n224k16, // .m64n224k16
        M64n232k16, // .m64n232k16
        M64n240k16, // .m64n240k16
        M64n248k16, // .m64n248k16
        M64n256k16, // .m64n256k16
        M64n16k16,  // .m64n16k16
        M64n24k16,  // .m64n24k16
        M64n32k16,  // .m64n32k16
        M64n40k16,  // .m64n40k16
        M64n48k16,  // .m64n48k16
        M64n56k16,  // .m64n56k16
        M64n64k16,  // .m64n64k16
        M64n72k16,  // .m64n72k16
        M64n80k16,  // .m64n80k16
        M64n88k16,  // .m64n88k16
        M64n96k16,  // .m64n96k16
        M64n8k16,   // .m64n8k16
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Dtype {
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf32 {
        pub mma_async: (),               // .mma_async
        pub sp: (),                      // .sp
        pub sync: (),                    // .sync
        pub aligned: (),                 // .aligned
        pub shape: Shape,                // .shape
        pub dtype: Dtype,                // .dtype
        pub tf32: (),                    // .tf32
        pub tf322: (),                   // .tf32
        pub d: GeneralOperand,           // d
        pub a_desc: GeneralOperand,      // a-desc
        pub b_desc: GeneralOperand,      // b-desc
        pub sp_meta: GeneralOperand,     // sp-meta
        pub sp_sel: GeneralOperand,      // sp-sel
        pub scale_d: GeneralOperand,     // scale-d
        pub imm_scale_a: GeneralOperand, // imm-scale-a
        pub imm_scale_b: GeneralOperand, // imm-scale-b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf321 {
        pub mma_async: (),               // .mma_async
        pub sp: (),                      // .sp
        pub sync: (),                    // .sync
        pub aligned: (),                 // .aligned
        pub shape: Shape,                // .shape
        pub dtype: Dtype,                // .dtype
        pub tf32: (),                    // .tf32
        pub tf322: (),                   // .tf32
        pub d: GeneralOperand,           // d
        pub a: GeneralOperand,           // a
        pub b_desc: GeneralOperand,      // b-desc
        pub sp_meta: GeneralOperand,     // sp-meta
        pub sp_sel: GeneralOperand,      // sp-sel
        pub scale_d: GeneralOperand,     // scale-d
        pub imm_scale_a: GeneralOperand, // imm-scale-a
        pub imm_scale_b: GeneralOperand, // imm-scale-b
        pub span: Span,
    }
}

pub mod section_3 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Shape {
        M64n104k64, // .m64n104k64
        M64n112k64, // .m64n112k64
        M64n120k64, // .m64n120k64
        M64n128k64, // .m64n128k64
        M64n136k64, // .m64n136k64
        M64n144k64, // .m64n144k64
        M64n152k64, // .m64n152k64
        M64n160k64, // .m64n160k64
        M64n168k64, // .m64n168k64
        M64n176k64, // .m64n176k64
        M64n184k64, // .m64n184k64
        M64n192k64, // .m64n192k64
        M64n200k64, // .m64n200k64
        M64n208k64, // .m64n208k64
        M64n216k64, // .m64n216k64
        M64n224k64, // .m64n224k64
        M64n232k64, // .m64n232k64
        M64n240k64, // .m64n240k64
        M64n248k64, // .m64n248k64
        M64n256k64, // .m64n256k64
        M64n16k64,  // .m64n16k64
        M64n24k64,  // .m64n24k64
        M64n32k64,  // .m64n32k64
        M64n40k64,  // .m64n40k64
        M64n48k64,  // .m64n48k64
        M64n56k64,  // .m64n56k64
        M64n64k64,  // .m64n64k64
        M64n72k64,  // .m64n72k64
        M64n80k64,  // .m64n80k64
        M64n88k64,  // .m64n88k64
        M64n96k64,  // .m64n96k64
        M64n8k64,   // .m64n8k64
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Dtype {
        F16, // .f16
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Atype {
        E4m3, // .e4m3
        E5m2, // .e5m2
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Btype {
        E4m3, // .e4m3
        E5m2, // .e5m2
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype {
        pub mma_async: (),               // .mma_async
        pub sp: (),                      // .sp
        pub sync: (),                    // .sync
        pub aligned: (),                 // .aligned
        pub shape: Shape,                // .shape
        pub dtype: Dtype,                // .dtype
        pub atype: Atype,                // .atype
        pub btype: Btype,                // .btype
        pub d: GeneralOperand,           // d
        pub a_desc: GeneralOperand,      // a-desc
        pub b_desc: GeneralOperand,      // b-desc
        pub sp_meta: GeneralOperand,     // sp-meta
        pub sp_sel: GeneralOperand,      // sp-sel
        pub scale_d: GeneralOperand,     // scale-d
        pub imm_scale_a: GeneralOperand, // imm-scale-a
        pub imm_scale_b: GeneralOperand, // imm-scale-b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype1 {
        pub mma_async: (),               // .mma_async
        pub sp: (),                      // .sp
        pub sync: (),                    // .sync
        pub aligned: (),                 // .aligned
        pub shape: Shape,                // .shape
        pub dtype: Dtype,                // .dtype
        pub atype: Atype,                // .atype
        pub btype: Btype,                // .btype
        pub d: GeneralOperand,           // d
        pub a: GeneralOperand,           // a
        pub b_desc: GeneralOperand,      // b-desc
        pub sp_meta: GeneralOperand,     // sp-meta
        pub sp_sel: GeneralOperand,      // sp-sel
        pub scale_d: GeneralOperand,     // scale-d
        pub imm_scale_a: GeneralOperand, // imm-scale-a
        pub imm_scale_b: GeneralOperand, // imm-scale-b
        pub span: Span,
    }
}

pub mod section_4 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Shape {
        M64n112k64, // .m64n112k64
        M64n128k64, // .m64n128k64
        M64n144k64, // .m64n144k64
        M64n160k64, // .m64n160k64
        M64n176k64, // .m64n176k64
        M64n192k64, // .m64n192k64
        M64n208k64, // .m64n208k64
        M64n224k64, // .m64n224k64
        M64n240k64, // .m64n240k64
        M64n256k64, // .m64n256k64
        M64n16k64,  // .m64n16k64
        M64n24k64,  // .m64n24k64
        M64n32k64,  // .m64n32k64
        M64n48k64,  // .m64n48k64
        M64n64k64,  // .m64n64k64
        M64n80k64,  // .m64n80k64
        M64n96k64,  // .m64n96k64
        M64n8k64,   // .m64n8k64
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Atype {
        S8, // .s8
        U8, // .u8
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Btype {
        S8, // .s8
        U8, // .u8
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype {
        pub mma_async: (),           // .mma_async
        pub sp: (),                  // .sp
        pub sync: (),                // .sync
        pub aligned: (),             // .aligned
        pub shape: Shape,            // .shape
        pub satfinite: bool,         // {.satfinite}
        pub s32: (),                 // .s32
        pub atype: Atype,            // .atype
        pub btype: Btype,            // .btype
        pub d: GeneralOperand,       // d
        pub a_desc: GeneralOperand,  // a-desc
        pub b_desc: GeneralOperand,  // b-desc
        pub sp_meta: GeneralOperand, // sp-meta
        pub sp_sel: GeneralOperand,  // sp-sel
        pub scale_d: GeneralOperand, // scale-d
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype1 {
        pub mma_async: (),           // .mma_async
        pub sp: (),                  // .sp
        pub sync: (),                // .sync
        pub aligned: (),             // .aligned
        pub shape: Shape,            // .shape
        pub satfinite: bool,         // {.satfinite}
        pub s32: (),                 // .s32
        pub atype: Atype,            // .atype
        pub btype: Btype,            // .btype
        pub d: GeneralOperand,       // d
        pub a: GeneralOperand,       // a
        pub b_desc: GeneralOperand,  // b-desc
        pub sp_meta: GeneralOperand, // sp-meta
        pub sp_sel: GeneralOperand,  // sp-sel
        pub scale_d: GeneralOperand, // scale-d
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Dtype as Dtype0;
pub use section_0::Shape as Shape0;
pub use section_0::WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F16;
pub use section_0::WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F161;
pub use section_1::Dtype as Dtype1;
pub use section_1::Shape as Shape1;
pub use section_1::WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf16;
pub use section_1::WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf161;
pub use section_2::Dtype as Dtype2;
pub use section_2::Shape as Shape2;
pub use section_2::WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf32;
pub use section_2::WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf321;
pub use section_3::Atype as Atype3;
pub use section_3::Btype as Btype3;
pub use section_3::Dtype as Dtype3;
pub use section_3::Shape as Shape3;
pub use section_3::WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype;
pub use section_3::WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype1;
pub use section_4::Atype as Atype4;
pub use section_4::Btype as Btype4;
pub use section_4::Shape as Shape4;
pub use section_4::WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype;
pub use section_4::WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype1;
