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
        pub global: bool,         // {.global}
        pub level: Level,         // .level
        pub a: AddressOperand,    // [a]
        pub size: GeneralOperand, // size
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::DiscardGlobalLevel;
pub use section_0::Level as Level0;
