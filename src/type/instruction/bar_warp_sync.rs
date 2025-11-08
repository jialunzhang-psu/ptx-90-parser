//! Original PTX specification:
//!
//! bar.warp.sync      membermask;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct BarWarpSync {
        pub warp: (),                   // .warp
        pub sync: (),                   // .sync
        pub membermask: GeneralOperand, // membermask
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::BarWarpSync;
