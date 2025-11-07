//! Original PTX specification:
//!
//! prmt.b32{.mode}  d, a, b, c;
//! .mode = { .f4e, .b4e, .rc8, .ecl, .ecr, .rc16 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Mode {
        Rc16, // .rc16
        F4e, // .f4e
        B4e, // .b4e
        Rc8, // .rc8
        Ecl, // .ecl
        Ecr, // .ecr
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct PrmtB32Mode {
        pub b32: (), // .b32
        pub mode: Option<Mode>, // {.mode}
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
    }

}
