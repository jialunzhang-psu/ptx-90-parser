//! Original PTX specification:
//!
//! alloca.type  ptr, size{, immAlign};
//! .type = { .u32, .u64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U32, // .u32
        U64, // .u64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AllocaType {
        pub type_: Type, // .type
        pub ptr: GeneralOperand, // ptr
        pub size: GeneralOperand, // size
        pub immalign: Option<GeneralOperand>, // {, immAlign}
    }

}
