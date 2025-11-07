//! Original PTX specification:
//!
//! shfl.mode.b32  d{|p}, a, b, c;
//! .mode = { .up, .down, .bfly, .idx };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Mode {
        Down, // .down
        Bfly, // .bfly
        Idx, // .idx
        Up, // .up
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ShflModeB32 {
        pub mode: Mode, // .mode
        pub b32: (), // .b32
        pub d: GeneralOperand, // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
    }

}
