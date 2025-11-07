//! Original PTX specification:
//!
//! bfe.type  d, a, b, c;
//! .type = { .u32, .u64, .s32, .s64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U32, // .u32
        U64, // .u64
        S32, // .s32
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BfeType {
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
    }

}
