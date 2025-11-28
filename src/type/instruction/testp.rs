//! Original PTX specification:
//!
//! testp.op.type  p, a;  // result is .pred
//! .op   = { .finite, .infinite,
//! .number, .notanumber,
//! .normal, .subnormal };
//! .type = { .f32, .f64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Op {
        Notanumber, // .notanumber
        Subnormal, // .subnormal
        Infinite, // .infinite
        Finite, // .finite
        Number, // .number
        Normal, // .normal
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        F32, // .f32
        F64, // .f64
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct TestpOpType {
        pub op: Op, // .op
        pub type_: Type, // .type
        pub p: GeneralOperand, // p
        pub a: GeneralOperand, // a
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::TestpOpType;
pub use section_0::Op as Op0;
pub use section_0::Type as Type0;
