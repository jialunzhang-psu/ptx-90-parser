//! Original PTX specification:
//!
//! sin.approx{.ftz}.f32  d, a;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct SinApproxFtzF32 {
        pub approx: (), // .approx
        pub ftz: bool, // {.ftz}
        pub f32: (), // .f32
        pub d: Operand, // d
        pub a: Operand, // a
    }

}
