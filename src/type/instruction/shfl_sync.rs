//! Original PTX specification:
//!
//! shfl.sync.mode.b32  d{|p}, a, b, c, membermask;
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
    pub struct ShflSyncModeB32 {
        pub sync: (), // .sync
        pub mode: Mode, // .mode
        pub b32: (), // .b32
        pub d: GeneralOperand, // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub membermask: GeneralOperand, // membermask
    }

}
