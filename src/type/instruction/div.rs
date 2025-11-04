//! Original PTX specification:
//!
//! div.type  d, a, b;
//! .type = { .u16, .u32, .u64,
//! .s16, .s32, .s64 };
//! 
//! div.approx{.ftz}.f32  d, a, b;  // fast, approximate divide
//! div.full{.ftz}.f32    d, a, b;  // full-range approximate divide
//! div.rnd{.ftz}.f32     d, a, b;  // IEEE 754 compliant rounding
//! div.rnd.f64           d, a, b;  // IEEE 754 compliant rounding
//! .rnd = { .rn, .rz, .rm, .rp };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U16, // .u16
        U32, // .u32
        U64, // .u64
        S16, // .s16
        S32, // .s32
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Rnd {
        Rn, // .rn
        Rz, // .rz
        Rm, // .rm
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct DivType {
        pub type_: Type, // .type
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct DivApproxFtzF32 {
        pub approx: (), // .approx
        pub ftz: bool, // {.ftz}
        pub f32: (), // .f32
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct DivFullFtzF32 {
        pub full: (), // .full
        pub ftz: bool, // {.ftz}
        pub f32: (), // .f32
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct DivRndFtzF32 {
        pub rnd: Rnd, // .rnd
        pub ftz: bool, // {.ftz}
        pub f32: (), // .f32
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct DivRndF64 {
        pub rnd: Rnd, // .rnd
        pub f64: (), // .f64
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

}
