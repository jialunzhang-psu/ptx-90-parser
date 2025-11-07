//! Original PTX specification:
//!
//! activemask.b32 d;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct ActivemaskB32 {
        pub b32: (), // .b32
        pub d: GeneralOperand, // d
    }

}
