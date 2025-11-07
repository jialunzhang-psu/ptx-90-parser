//! Original PTX specification:
//!
//! testp.op.type  p, a;  // result is .pred
//! .op   = { .finite, .infinite,
//! .number, .notanumber,
//! .normal, .subnormal };
//! .type = { .f32, .f64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        Notanumber, // .notanumber
        Subnormal, // .subnormal
        Infinite, // .infinite
        Finite, // .finite
        Number, // .number
        Normal, // .normal
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        F32, // .f32
        F64, // .f64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TestpOpType {
        pub op: Op, // .op
        pub type_: Type, // .type
        pub p: GeneralOperand, // p
        pub a: GeneralOperand, // a
    }

}
