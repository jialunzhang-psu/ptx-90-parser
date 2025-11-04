//! Original PTX specification:
//!
//! mul24.mode.type  d, a, b;
//! .mode = { .hi, .lo };
//! .type = { .u32, .s32 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Mode {
        Hi, // .hi
        Lo, // .lo
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Mul24ModeType {
        pub mode: Mode, // .mode
        pub type_: Type, // .type
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

}
