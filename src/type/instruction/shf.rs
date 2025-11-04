//! Original PTX specification:
//!
//! shf.l.mode.b32  d, a, b, c;  // left shift
//! shf.r.mode.b32  d, a, b, c;  // right shift
//! .mode = { .clamp, .wrap };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Mode {
        Clamp, // .clamp
        Wrap, // .wrap
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ShfLModeB32 {
        pub l: (), // .l
        pub mode: Mode, // .mode
        pub b32: (), // .b32
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ShfRModeB32 {
        pub r: (), // .r
        pub mode: Mode, // .mode
        pub b32: (), // .b32
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

}
