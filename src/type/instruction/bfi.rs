//! Original PTX specification:
//!
//! bfi.type  f, a, b, c, d;
//! .type = { .b32, .b64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B32, // .b32
        B64, // .b64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BfiType {
        pub type_: Type, // .type
        pub f: Operand, // f
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
        pub d: Operand, // d
    }

}
