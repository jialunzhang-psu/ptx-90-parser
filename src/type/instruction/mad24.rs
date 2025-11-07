//! Original PTX specification:
//!
//! mad24.mode.type  d, a, b, c;
//! mad24.hi.sat.s32 d, a, b, c;
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
    pub struct Mad24ModeType {
        pub mode: Mode, // .mode
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Mad24HiSatS32 {
        pub hi: (), // .hi
        pub sat: (), // .sat
        pub s32: (), // .s32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
    }

}
