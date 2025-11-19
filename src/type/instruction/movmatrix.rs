//! Original PTX specification:
//!
//! movmatrix.sync.aligned.shape.trans.type d, a;
//! .shape  = {.m8n8};
//! .type   = {.b16};

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape {
        M8n8, // .m8n8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B16, // .b16
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct MovmatrixSyncAlignedShapeTransType {
        pub sync: (),          // .sync
        pub aligned: (),       // .aligned
        pub shape: Shape,      // .shape
        pub trans: (),         // .trans
        pub type_: Type,       // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::MovmatrixSyncAlignedShapeTransType;
pub use section_0::Shape as Shape0;
pub use section_0::Type as Type0;
