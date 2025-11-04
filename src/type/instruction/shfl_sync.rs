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
        Up, // .up
        Down, // .down
        Bfly, // .bfly
        Idx, // .idx
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ShflSyncModeB32 {
        pub sync: (), // .sync
        pub mode: Mode, // .mode
        pub b32: (), // .b32
        pub d: Operand, // d{|p}
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
        pub membermask: Operand, // membermask
    }

}
