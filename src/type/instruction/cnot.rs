//! Original PTX specification:
//!
//! cnot.type d, a;
//! .type = { .b16, .b32, .b64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B16, // .b16
        B32, // .b32
        B64, // .b64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CnotType {
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

}
