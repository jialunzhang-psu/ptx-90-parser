//! Original PTX specification:
//!
//! istypep.type   p, a;  // result is .pred
//! .type = { .texref, .samplerref, .surfref };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        Samplerref, // .samplerref
        Surfref, // .surfref
        Texref, // .texref
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct IstypepType {
        pub type_: Type, // .type
        pub p: GeneralOperand, // p
        pub a: GeneralOperand, // a
    }

}
