//! Original PTX specification:
//!
//! discard{.global}.level  [a], size;
//! .level = { .L2 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Level {
        L2, // .L2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct DiscardGlobalLevel {
        pub global: bool, // {.global}
        pub level: Level, // .level
        pub a: AddressOperand, // [a]
        pub size: Operand, // size
    }

}
