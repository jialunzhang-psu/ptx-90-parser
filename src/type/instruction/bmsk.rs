//! Original PTX specification:
//!
//! bmsk.mode.b32  d, a, b;
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
    pub struct BmskModeB32 {
        pub mode: Mode, // .mode
        pub b32: (), // .b32
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

}
