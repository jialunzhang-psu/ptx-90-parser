//! Original PTX specification:
//!
//! bar.warp.sync      membermask;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct BarWarpSync {
        pub warp: (), // .warp
        pub sync: (), // .sync
        pub membermask: GeneralOperand, // membermask
    }

}
