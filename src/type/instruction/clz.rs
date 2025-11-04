//! Original PTX specification:
//!
//! clz.type  d, a;
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
    pub struct ClzType {
        pub type_: Type, // .type
        pub d: Operand, // d
        pub a: Operand, // a
    }

}
