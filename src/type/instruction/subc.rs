//! Original PTX specification:
//!
//! subc{.cc}.type  d, a, b;
//! .type = { .u32, .s32, .u64, .s64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U32, // .u32
        S32, // .s32
        U64, // .u64
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubcCcType {
        pub cc: bool, // {.cc}
        pub type_: Type, // .type
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

}
