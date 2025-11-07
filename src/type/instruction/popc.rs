//! Original PTX specification:
//!
//! popc.type  d, a;
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
    pub struct PopcType {
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

}
