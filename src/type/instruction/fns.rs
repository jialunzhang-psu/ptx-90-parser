//! Original PTX specification:
//!
//! fns.b32 d, mask, base, offset;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct FnsB32 {
        pub b32: (), // .b32
        pub d: Operand, // d
        pub mask: Operand, // mask
        pub base: Operand, // base
        pub offset: Operand, // offset
    }

}
