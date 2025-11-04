//! Original PTX specification:
//!
//! bfind.type           d, a;
//! bfind.shiftamt.type  d, a;
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
    pub struct BfindType {
        pub type_: Type, // .type
        pub d: Operand, // d
        pub a: Operand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BfindShiftamtType {
        pub shiftamt: (), // .shiftamt
        pub type_: Type, // .type
        pub d: Operand, // d
        pub a: Operand, // a
    }

}
