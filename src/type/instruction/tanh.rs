//! Original PTX specification:
//!
//! tanh.approx.type d, a;
//! .type = {.f16, .f32, .f16x2, .bf16, .bf16x2};

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        F16, // .f16
        F32, // .f32
        F16x2, // .f16x2
        Bf16, // .bf16
        Bf16x2, // .bf16x2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TanhApproxType {
        pub approx: (), // .approx
        pub type_: Type, // .type
        pub d: Operand, // d
        pub a: Operand, // a
    }

}
