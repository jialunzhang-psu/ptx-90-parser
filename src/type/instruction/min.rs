//! Original PTX specification:
//!
//! min.atype         d, a, b;
//! min{.relu}.btype  d, a, b;
//! .atype = { .u16, .u32, .u64, .u16x2, .s16, .s64 };
//! .btype = { .s16x2, .s32 };
//! 
//! min{.ftz}{.NaN}{.xorsign.abs}.f32  d, a, b;
//! min{.ftz}{.NaN}{.abs}.f32          d, a, b, c;
//! min.f64                            d, a, b;
//! 
//! min{.ftz}{.NaN}{.xorsign.abs}.f16      d, a, b;
//! min{.ftz}{.NaN}{.xorsign.abs}.f16x2    d, a, b;
//! min{.NaN}{.xorsign.abs}.bf16           d, a, b;
//! min{.NaN}{.xorsign.abs}.bf16x2         d, a, b;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Atype {
        U16, // .u16
        U32, // .u32
        U64, // .u64
        U16x2, // .u16x2
        S16, // .s16
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Btype {
        S16x2, // .s16x2
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MinAtype {
        pub atype: Atype, // .atype
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MinReluBtype {
        pub relu: bool, // {.relu}
        pub btype: Btype, // .btype
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MinFtzNanXorsignAbsF32 {
        pub ftz: bool, // {.ftz}
        pub nan: bool, // {.NaN}
        pub xorsign_abs: bool, // {.xorsign.abs}
        pub f32: (), // .f32
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MinFtzNanAbsF32 {
        pub ftz: bool, // {.ftz}
        pub nan: bool, // {.NaN}
        pub abs: bool, // {.abs}
        pub f32: (), // .f32
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MinF64 {
        pub f64: (), // .f64
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MinFtzNanXorsignAbsF16 {
        pub ftz: bool, // {.ftz}
        pub nan: bool, // {.NaN}
        pub xorsign_abs: bool, // {.xorsign.abs}
        pub f16: (), // .f16
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MinFtzNanXorsignAbsF16x2 {
        pub ftz: bool, // {.ftz}
        pub nan: bool, // {.NaN}
        pub xorsign_abs: bool, // {.xorsign.abs}
        pub f16x2: (), // .f16x2
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MinNanXorsignAbsBf16 {
        pub nan: bool, // {.NaN}
        pub xorsign_abs: bool, // {.xorsign.abs}
        pub bf16: (), // .bf16
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MinNanXorsignAbsBf16x2 {
        pub nan: bool, // {.NaN}
        pub xorsign_abs: bool, // {.xorsign.abs}
        pub bf16x2: (), // .bf16x2
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

}
