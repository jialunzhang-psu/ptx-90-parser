//! Original PTX specification:
//!
//! stacksave.type  d;
//! .type = { .u32, .u64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U32, // .u32
        U64, // .u64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct StacksaveType {
        pub type_: Type, // .type
        pub d: Operand, // d
    }

}
