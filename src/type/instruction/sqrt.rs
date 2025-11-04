//! Original PTX specification:
//!
//! sqrt.approx{.ftz}.f32  d, a; // fast, approximate square root
//! sqrt.rnd{.ftz}.f32     d, a; // IEEE 754 compliant rounding
//! sqrt.rnd.f64           d, a; // IEEE 754 compliant rounding
//! .rnd = { .rn, .rz, .rm, .rp };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Rnd {
        Rn, // .rn
        Rz, // .rz
        Rm, // .rm
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SqrtApproxFtzF32 {
        pub approx: (), // .approx
        pub ftz: bool, // {.ftz}
        pub f32: (), // .f32
        pub d: Operand, // d
        pub a: Operand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SqrtRndFtzF32 {
        pub rnd: Rnd, // .rnd
        pub ftz: bool, // {.ftz}
        pub f32: (), // .f32
        pub d: Operand, // d
        pub a: Operand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SqrtRndF64 {
        pub rnd: Rnd, // .rnd
        pub f64: (), // .f64
        pub d: Operand, // d
        pub a: Operand, // a
    }

}
