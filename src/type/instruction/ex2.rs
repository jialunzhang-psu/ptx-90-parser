//! Original PTX specification:
//!
//! ex2.approx{.ftz}.f32  d, a;
//! 
//! ex2.approx.atype     d, a;
//! ex2.approx.ftz.btype d, a;
//! .atype = { .f16,  .f16x2};
//! .btype = { .bf16, .bf16x2};

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Atype {
        F16, // .f16
        F16x2, // .f16x2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Btype {
        Bf16, // .bf16
        Bf16x2, // .bf16x2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Ex2ApproxFtzF32 {
        pub approx: (), // .approx
        pub ftz: bool, // {.ftz}
        pub f32: (), // .f32
        pub d: Operand, // d
        pub a: Operand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Ex2ApproxAtype {
        pub approx: (), // .approx
        pub atype: Atype, // .atype
        pub d: Operand, // d
        pub a: Operand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Ex2ApproxFtzBtype {
        pub approx: (), // .approx
        pub ftz: (), // .ftz
        pub btype: Btype, // .btype
        pub d: Operand, // d
        pub a: Operand, // a
    }

}
