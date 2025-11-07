//! Original PTX specification:
//!
//! elect.sync d|p, membermask;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct ElectSync {
        pub sync: (), // .sync
        pub d: GeneralOperand, // first operand of d|p
        pub p: GeneralOperand, // second operand of d|p
        pub membermask: GeneralOperand, // membermask
    }

}
