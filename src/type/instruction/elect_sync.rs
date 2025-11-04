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
        pub d: Operand, // d|p
        pub membermask: Operand, // membermask
    }

}
