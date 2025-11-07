//! Original PTX specification:
//!
//! neg.type  d, a;
//! .type = { .s16, .s32, .s64 };
//! 
//! neg{.ftz}.f32  d, a;
//! neg.f64        d, a;
//! 
//! neg{.ftz}.f16    d, a;
//! neg{.ftz}.f16x2  d, a;
//! neg.bf16         d, a;
//! neg.bf16x2       d, a;

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
    pub struct NegType {
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct NegFtzF32 {
        pub ftz: bool, // {.ftz}
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct NegF64 {
        pub f64: (), // .f64
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct NegFtzF16 {
        pub ftz: bool, // {.ftz}
        pub f16: (), // .f16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct NegFtzF16x2 {
        pub ftz: bool, // {.ftz}
        pub f16x2: (), // .f16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct NegBf16 {
        pub bf16: (), // .bf16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct NegBf16x2 {
        pub bf16x2: (), // .bf16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

}
