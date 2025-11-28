//! Original PTX specification:
//!
//! // 32-bit scalar operation, with optional secondary operation
//! vset.atype.btype.cmp       d, a{.asel}, b{.bsel};
//! vset.atype.btype.cmp.op2   d, a{.asel}, b{.bsel}, c;
//! // 32-bit scalar operation, with optional data merge
//! vset.atype.btype.cmp  d.dsel, a{.asel}, b{.bsel}, c;
//! .atype = .btype = { .u32, .s32 };
//! .cmp   = { .eq, .ne, .lt, .le, .gt, .ge };
//! .dsel  = .asel  = .bsel  = { .b0, .b1, .b2, .b3, .h0, .h1 };
//! .op2   = { .add, .min, .max };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Atype {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Btype {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Cmp {
        Eq, // .eq
        Ne, // .ne
        Lt, // .lt
        Le, // .le
        Gt, // .gt
        Ge, // .ge
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Asel {
        B0, // .b0
        B1, // .b1
        B2, // .b2
        B3, // .b3
        H0, // .h0
        H1, // .h1
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Bsel {
        B0, // .b0
        B1, // .b1
        B2, // .b2
        B3, // .b3
        H0, // .h0
        H1, // .h1
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Op2 {
        Add, // .add
        Min, // .min
        Max, // .max
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Dsel {
        B0, // .b0
        B1, // .b1
        B2, // .b2
        B3, // .b3
        H0, // .h0
        H1, // .h1
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VsetAtypeBtypeCmp {
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub cmp: Cmp, // .cmp
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VsetAtypeBtypeCmpOp2 {
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub cmp: Cmp, // .cmp
        pub op2: Op2, // .op2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VsetAtypeBtypeCmp1 {
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub cmp: Cmp, // .cmp
        pub d: GeneralOperand, // d
        pub dsel: Dsel, // .dsel
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::VsetAtypeBtypeCmp;
pub use section_0::VsetAtypeBtypeCmpOp2;
pub use section_0::VsetAtypeBtypeCmp1;
pub use section_0::Atype as Atype0;
pub use section_0::Btype as Btype0;
pub use section_0::Cmp as Cmp0;
pub use section_0::Asel as Asel0;
pub use section_0::Bsel as Bsel0;
pub use section_0::Op2 as Op20;
pub use section_0::Dsel as Dsel0;
