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
        Texref, // .texref
        Samplerref, // .samplerref
        Surfref, // .surfref
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct IstypepType {
        pub type_: Type, // .type
        pub p: Operand, // p
        pub a: Operand, // a
    }

}
