//! Original PTX specification:
//!
//! bra{.uni}  tgt;           // tgt is a label
//! bra{.uni}  tgt;           // unconditional branch

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct BraUni {
        pub uni: bool, // {.uni}
        pub tgt: GeneralOperand, // tgt
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BraUni1 {
        pub uni: bool, // {.uni}
        pub tgt: GeneralOperand, // tgt
    }

}
