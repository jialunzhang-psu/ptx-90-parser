//! Original PTX specification:
//!
//! lop3.b32 d, a, b, c, immLut;
//! lop3.BoolOp.b32 d|p, a, b, c, immLut, q;
//! .BoolOp   = { .or , .and };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Boolop {
        And, // .and
        Or,  // .or
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Lop3B32 {
        pub b32: (),                // .b32
        pub d: GeneralOperand,      // d
        pub a: GeneralOperand,      // a
        pub b: GeneralOperand,      // b
        pub c: GeneralOperand,      // c
        pub immlut: GeneralOperand, // immLut
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Lop3BoolopB32 {
        pub boolop: Boolop,         // .BoolOp
        pub b32: (),                // .b32
        pub d: GeneralOperand,      // first operand of d|p
        pub p: GeneralOperand,      // second operand of d|p
        pub a: GeneralOperand,      // a
        pub b: GeneralOperand,      // b
        pub c: GeneralOperand,      // c
        pub immlut: GeneralOperand, // immLut
        pub q: GeneralOperand,      // q
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Boolop as Boolop0;
pub use section_0::Lop3B32;
pub use section_0::Lop3BoolopB32;
