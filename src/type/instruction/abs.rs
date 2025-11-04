//! Original PTX specification:
//!
//! abs.type  d, a;
//! .type = { .s16, .s32, .s64 };
//! 
//! abs{.ftz}.f32  d, a;
//! abs.f64        d, a;
//! 
//! abs{.ftz}.f16    d, a;
//! abs{.ftz}.f16x2  d, a;
//! abs.bf16         d, a;
//! abs.bf16x2       d, a;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        S16, // .s16
        S32, // .s32
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AbsType {
        pub type_: Type, // .type
        pub d: Operand, // d
        pub a: Operand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AbsFtzF32 {
        pub ftz: bool, // {.ftz}
        pub f32: (), // .f32
        pub d: Operand, // d
        pub a: Operand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AbsF64 {
        pub f64: (), // .f64
        pub d: Operand, // d
        pub a: Operand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AbsFtzF16 {
        pub ftz: bool, // {.ftz}
        pub f16: (), // .f16
        pub d: Operand, // d
        pub a: Operand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AbsFtzF16x2 {
        pub ftz: bool, // {.ftz}
        pub f16x2: (), // .f16x2
        pub d: Operand, // d
        pub a: Operand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AbsBf16 {
        pub bf16: (), // .bf16
        pub d: Operand, // d
        pub a: Operand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AbsBf16x2 {
        pub bf16x2: (), // .bf16x2
        pub d: Operand, // d
        pub a: Operand, // a
    }

}
