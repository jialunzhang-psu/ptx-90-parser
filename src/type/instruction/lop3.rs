//! Original PTX specification:
//!
//! lop3.b32 d, a, b, c, immLut;
//! lop3.BoolOp.b32 d|p, a, b, c, immLut, q;
//! .BoolOp   = { .or , .and };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Boolop {
        And, // .and
        Or, // .or
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Lop3B32 {
        pub b32: (), // .b32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub immlut: GeneralOperand, // immLut
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Lop3BoolopB32 {
        pub boolop: Boolop, // .BoolOp
        pub b32: (), // .b32
        pub d: GeneralOperand, // first operand of d|p
        pub p: GeneralOperand, // second operand of d|p
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub immlut: GeneralOperand, // immLut
        pub q: GeneralOperand, // q
    }

}
