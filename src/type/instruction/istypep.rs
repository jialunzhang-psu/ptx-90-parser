//! Original PTX specification:
//!
//! istypep.type   p, a;  // result is .pred
//! .type = { .texref, .samplerref, .surfref };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        Samplerref, // .samplerref
        Surfref,    // .surfref
        Texref,     // .texref
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct IstypepType {
        pub type_: Type,       // .type
        pub p: GeneralOperand, // p
        pub a: GeneralOperand, // a
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::IstypepType;
pub use section_0::Type as Type0;
