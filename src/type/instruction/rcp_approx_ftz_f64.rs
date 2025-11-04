//! Original PTX specification:
//!
//! rcp.approx.ftz.f64  d, a;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct RcpApproxFtzF64 {
        pub approx: (), // .approx
        pub ftz: (), // .ftz
        pub f64: (), // .f64
        pub d: Operand, // d
        pub a: Operand, // a
    }

}
