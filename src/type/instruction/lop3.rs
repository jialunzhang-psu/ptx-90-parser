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
        Or, // .or
        And, // .and
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Lop3B32 {
        pub b32: (), // .b32
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
        pub immlut: Operand, // immLut
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Lop3BoolopB32 {
        pub boolop: Boolop, // .BoolOp
        pub b32: (), // .b32
        pub d: Operand, // first operand of d|p
        pub p: Operand, // second operand of d|p
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
        pub immlut: Operand, // immLut
        pub q: Operand, // q
    }

}
