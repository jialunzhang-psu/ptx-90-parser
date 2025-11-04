//! Original PTX specification:
//!
//! dp2a.mode.atype.btype  d, a, b, c;
//! .atype = .btype = { .u32, .s32 };
//! .mode = { .lo, .hi };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Mode {
        Lo, // .lo
        Hi, // .hi
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Atype {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Btype {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Dp2aModeAtypeBtype {
        pub mode: Mode, // .mode
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

}
