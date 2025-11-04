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
        F4e, // .f4e
        B4e, // .b4e
        Rc8, // .rc8
        Ecl, // .ecl
        Ecr, // .ecr
        Rc16, // .rc16
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct PrmtB32Mode {
        pub b32: (), // .b32
        pub mode: Option<Mode>, // {.mode}
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

}
