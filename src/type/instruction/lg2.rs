//! Original PTX specification:
//!
//! lg2.approx{.ftz}.f32  d, a;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Lg2ApproxFtzF32 {
        pub approx: (), // .approx
        pub ftz: bool, // {.ftz}
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

}
