//! Original PTX specification:
//!
//! szext.mode.type  d, a, b;
//! .mode = { .clamp, .wrap };
//! .type = { .u32, .s32 };

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
    pub enum Type {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SzextModeType {
        pub mode: Mode, // .mode
        pub type_: Type, // .type
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

}
