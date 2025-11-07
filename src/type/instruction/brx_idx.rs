//! Original PTX specification:
//!
//! brx.idx{.uni} index, tlist;
//! brx.idx{.uni} index, tlist;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct BrxIdxUni {
        pub idx: (), // .idx
        pub uni: bool, // {.uni}
        pub index: GeneralOperand, // index
        pub tlist: GeneralOperand, // tlist
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BrxIdxUni1 {
        pub idx: (), // .idx
        pub uni: bool, // {.uni}
        pub index: GeneralOperand, // index
        pub tlist: GeneralOperand, // tlist
    }

}
