//! Original PTX specification:
//!
//! rsqrt.approx{.ftz}.f32  d, a;
//! rsqrt.approx.f64        d, a;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct RsqrtApproxFtzF32 {
        pub approx: (), // .approx
        pub ftz: bool, // {.ftz}
        pub f32: (), // .f32
        pub d: Operand, // d
        pub a: Operand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct RsqrtApproxF64 {
        pub approx: (), // .approx
        pub f64: (), // .f64
        pub d: Operand, // d
        pub a: Operand, // a
    }

}
