//! Original PTX specification:
//!
//! slct.dtype.s32        d, a, b, c;
//! slct{.ftz}.dtype.f32  d, a, b, c;
//! .dtype = { .b16, .b32, .b64,
//! .u16, .u32, .u64,
//! .s16, .s32, .s64,
//! .f32, .f64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dtype {
        B16, // .b16
        B32, // .b32
        B64, // .b64
        U16, // .u16
        U32, // .u32
        U64, // .u64
        S16, // .s16
        S32, // .s32
        S64, // .s64
        F32, // .f32
        F64, // .f64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SlctDtypeS32 {
        pub dtype: Dtype, // .dtype
        pub s32: (), // .s32
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SlctFtzDtypeF32 {
        pub ftz: bool, // {.ftz}
        pub dtype: Dtype, // .dtype
        pub f32: (), // .f32
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

}
