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
        pub f: GeneralOperand, // f
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub d: GeneralOperand, // d
    }

}
