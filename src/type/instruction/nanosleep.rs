//! Original PTX specification:
//!
//! nanosleep.u32 t;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct NanosleepU32 {
        pub u32: (), // .u32
        pub t: GeneralOperand, // t
    }

}
