//! Original PTX specification:
//!
//! cvt{.irnd}{.ftz}{.sat}.dtype.atype         d, a;  // integer rounding
//! cvt{.frnd}{.ftz}{.sat}.dtype.atype         d, a;  // fp rounding
//! cvt.frnd2{.relu}{.satfinite}.f16.f32       d, a;
//! cvt.frnd2{.relu}{.satfinite}.f16x2.f32     d, a, b;
//! cvt.rs{.relu}{.satfinite}.f16x2.f32        d, a, b, rbits;
//! cvt.frnd2{.relu}{.satfinite}.bf16.f32      d, a;
//! cvt.frnd2{.relu}{.satfinite}.bf16x2.f32    d, a, b;
//! cvt.rs{.relu}{.satfinite}.bf16x2.f32       d, a, b, rbits;
//! cvt.rna{.satfinite}.tf32.f32               d, a;
//! cvt.frnd2{.satfinite}{.relu}.tf32.f32      d, a;
//! cvt.rn.satfinite{.relu}.f8x2type.f32       d, a, b;
//! cvt.rn.satfinite{.relu}.f8x2type.f16x2     d, a;
//! cvt.rn{.relu}.f16x2.f8x2type              d, a;
//! cvt.rs{.relu}.satfinite.f8x4type.f32       d, {a, b, e, f}, rbits;
//! cvt.rn.satfinite{.relu}.f4x2type.f32       d, a, b;
//! cvt.rn{.relu}.f16x2.f4x2type               d, a;
//! cvt.rs{.relu}.satfinite.f4x4type.f32       d, {a, b, e, f}, rbits;
//! cvt.rn.satfinite{.relu}.f6x2type.f32       d, a, b;
//! cvt.rn{.relu}.f16x2.f6x2type               d, a;
//! cvt.rs{.relu}.satfinite.f6x4type.f32       d, {a, b, e, f}, rbits;
//! cvt.frnd3{.satfinite}.ue8m0x2.f32          d, a, b;
//! cvt.frnd3{.satfinite}.ue8m0x2.bf16x2       d, a;
//! cvt.rn.bf16x2.ue8m0x2                      d, a;
//! .irnd   = { .rni, .rzi, .rmi, .rpi };
//! .frnd   = { .rn,  .rz,  .rm,  .rp  };
//! .frnd2  = { .rn,  .rz };
//! .frnd3  = { .rz,  .rp };
//! .dtype = .atype = { .u8,   .u16, .u32, .u64,
//! .s8,   .s16, .s32, .s64,
//! .bf16, .f16, .f32, .f64 };
//! .f8x2type = { .e4m3x2, .e5m2x2 };
//! .f4x2type = { .e2m1x2 };
//! .f6x2type = { .e2m3x2, .e3m2x2 };
//! .f4x4type = { .e2m1x4 };
//! .f8x4type = { .e4m3x4, .e5m2x4 };
//! .f6x4type = { .e2m3x4, .e3m2x4 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Irnd {
        Rni, // .rni
        Rzi, // .rzi
        Rmi, // .rmi
        Rpi, // .rpi
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dtype {
        Bf16, // .bf16
        U16, // .u16
        U32, // .u32
        U64, // .u64
        S16, // .s16
        S32, // .s32
        S64, // .s64
        F16, // .f16
        F32, // .f32
        F64, // .f64
        U8, // .u8
        S8, // .s8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Atype {
        Bf16, // .bf16
        U16, // .u16
        U32, // .u32
        U64, // .u64
        S16, // .s16
        S32, // .s32
        S64, // .s64
        F16, // .f16
        F32, // .f32
        F64, // .f64
        U8, // .u8
        S8, // .s8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Frnd {
        Rn, // .rn
        Rz, // .rz
        Rm, // .rm
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Frnd2 {
        Rn, // .rn
        Rz, // .rz
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum F8x2type {
        E4m3x2, // .e4m3x2
        E5m2x2, // .e5m2x2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum F8x4type {
        E4m3x4, // .e4m3x4
        E5m2x4, // .e5m2x4
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum F4x2type {
        E2m1x2, // .e2m1x2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum F4x4type {
        E2m1x4, // .e2m1x4
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum F6x2type {
        E2m3x2, // .e2m3x2
        E3m2x2, // .e3m2x2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum F6x4type {
        E2m3x4, // .e2m3x4
        E3m2x4, // .e3m2x4
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Frnd3 {
        Rz, // .rz
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtIrndFtzSatDtypeAtype {
        pub irnd: Option<Irnd>, // {.irnd}
        pub ftz: bool, // {.ftz}
        pub sat: bool, // {.sat}
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtFrndFtzSatDtypeAtype {
        pub frnd: Option<Frnd>, // {.frnd}
        pub ftz: bool, // {.ftz}
        pub sat: bool, // {.sat}
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtFrnd2ReluSatfiniteF16F32 {
        pub frnd2: Frnd2, // .frnd2
        pub relu: bool, // {.relu}
        pub satfinite: bool, // {.satfinite}
        pub f16: (), // .f16
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtFrnd2ReluSatfiniteF16x2F32 {
        pub frnd2: Frnd2, // .frnd2
        pub relu: bool, // {.relu}
        pub satfinite: bool, // {.satfinite}
        pub f16x2: (), // .f16x2
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtRsReluSatfiniteF16x2F32 {
        pub rs: (), // .rs
        pub relu: bool, // {.relu}
        pub satfinite: bool, // {.satfinite}
        pub f16x2: (), // .f16x2
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub rbits: GeneralOperand, // rbits
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtFrnd2ReluSatfiniteBf16F32 {
        pub frnd2: Frnd2, // .frnd2
        pub relu: bool, // {.relu}
        pub satfinite: bool, // {.satfinite}
        pub bf16: (), // .bf16
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtFrnd2ReluSatfiniteBf16x2F32 {
        pub frnd2: Frnd2, // .frnd2
        pub relu: bool, // {.relu}
        pub satfinite: bool, // {.satfinite}
        pub bf16x2: (), // .bf16x2
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtRsReluSatfiniteBf16x2F32 {
        pub rs: (), // .rs
        pub relu: bool, // {.relu}
        pub satfinite: bool, // {.satfinite}
        pub bf16x2: (), // .bf16x2
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub rbits: GeneralOperand, // rbits
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtRnaSatfiniteTf32F32 {
        pub rna: (), // .rna
        pub satfinite: bool, // {.satfinite}
        pub tf32: (), // .tf32
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtFrnd2SatfiniteReluTf32F32 {
        pub frnd2: Frnd2, // .frnd2
        pub satfinite: bool, // {.satfinite}
        pub relu: bool, // {.relu}
        pub tf32: (), // .tf32
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtRnSatfiniteReluF8x2typeF32 {
        pub rn: (), // .rn
        pub satfinite: (), // .satfinite
        pub relu: bool, // {.relu}
        pub f8x2type: F8x2type, // .f8x2type
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtRnSatfiniteReluF8x2typeF16x2 {
        pub rn: (), // .rn
        pub satfinite: (), // .satfinite
        pub relu: bool, // {.relu}
        pub f8x2type: F8x2type, // .f8x2type
        pub f16x2: (), // .f16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtRnReluF16x2F8x2type {
        pub rn: (), // .rn
        pub relu: bool, // {.relu}
        pub f16x2: (), // .f16x2
        pub f8x2type: F8x2type, // .f8x2type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtRsReluSatfiniteF8x4typeF32 {
        pub rs: (), // .rs
        pub relu: bool, // {.relu}
        pub satfinite: (), // .satfinite
        pub f8x4type: F8x4type, // .f8x4type
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: VectorOperand, // {a, b, e, f}
        pub rbits: GeneralOperand, // rbits
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtRnSatfiniteReluF4x2typeF32 {
        pub rn: (), // .rn
        pub satfinite: (), // .satfinite
        pub relu: bool, // {.relu}
        pub f4x2type: F4x2type, // .f4x2type
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtRnReluF16x2F4x2type {
        pub rn: (), // .rn
        pub relu: bool, // {.relu}
        pub f16x2: (), // .f16x2
        pub f4x2type: F4x2type, // .f4x2type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtRsReluSatfiniteF4x4typeF32 {
        pub rs: (), // .rs
        pub relu: bool, // {.relu}
        pub satfinite: (), // .satfinite
        pub f4x4type: F4x4type, // .f4x4type
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: VectorOperand, // {a, b, e, f}
        pub rbits: GeneralOperand, // rbits
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtRnSatfiniteReluF6x2typeF32 {
        pub rn: (), // .rn
        pub satfinite: (), // .satfinite
        pub relu: bool, // {.relu}
        pub f6x2type: F6x2type, // .f6x2type
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtRnReluF16x2F6x2type {
        pub rn: (), // .rn
        pub relu: bool, // {.relu}
        pub f16x2: (), // .f16x2
        pub f6x2type: F6x2type, // .f6x2type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtRsReluSatfiniteF6x4typeF32 {
        pub rs: (), // .rs
        pub relu: bool, // {.relu}
        pub satfinite: (), // .satfinite
        pub f6x4type: F6x4type, // .f6x4type
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: VectorOperand, // {a, b, e, f}
        pub rbits: GeneralOperand, // rbits
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtFrnd3SatfiniteUe8m0x2F32 {
        pub frnd3: Frnd3, // .frnd3
        pub satfinite: bool, // {.satfinite}
        pub ue8m0x2: (), // .ue8m0x2
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtFrnd3SatfiniteUe8m0x2Bf16x2 {
        pub frnd3: Frnd3, // .frnd3
        pub satfinite: bool, // {.satfinite}
        pub ue8m0x2: (), // .ue8m0x2
        pub bf16x2: (), // .bf16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtRnBf16x2Ue8m0x2 {
        pub rn: (), // .rn
        pub bf16x2: (), // .bf16x2
        pub ue8m0x2: (), // .ue8m0x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

}
