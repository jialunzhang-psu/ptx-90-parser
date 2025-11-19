//! Original PTX specification:
//!
//! // 32-bit scalar operation
//! vmad.dtype.atype.btype{.sat}{.scale}     d, {-}a{.asel}, {-}b{.bsel},
//! {-}c;
//! vmad.dtype.atype.btype.po{.sat}{.scale}  d, a{.asel}, b{.bsel}, c;
//! .dtype = .atype = .btype = { .u32, .s32 };
//! .asel  = .bsel  = { .b0, .b1, .b2, .b3, .h0, .h1 };
//! .scale = { .shr7, .shr15 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dtype {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Atype {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Btype {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scale {
        Shr15, // .shr15
        Shr7,  // .shr7
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Asel {
        B0, // .b0
        B1, // .b1
        B2, // .b2
        B3, // .b3
        H0, // .h0
        H1, // .h1
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Bsel {
        B0, // .b0
        B1, // .b1
        B2, // .b2
        B3, // .b3
        H0, // .h0
        H1, // .h1
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct VmadDtypeAtypeBtypeSatScale {
        pub dtype: Dtype,         // .dtype
        pub atype: Atype,         // .atype
        pub btype: Btype,         // .btype
        pub sat: bool,            // {.sat}
        pub scale: Option<Scale>, // {.scale}
        pub d: GeneralOperand,    // d
        pub a_op: bool,           // {-} operator
        pub a: GeneralOperand,    // {-}a
        pub asel: Option<Asel>,   // {.asel}
        pub b_op: bool,           // {-} operator
        pub b: GeneralOperand,    // {-}b
        pub bsel: Option<Bsel>,   // {.bsel}
        pub c_op: bool,           // {-} operator
        pub c: GeneralOperand,    // {-}c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct VmadDtypeAtypeBtypePoSatScale {
        pub dtype: Dtype,         // .dtype
        pub atype: Atype,         // .atype
        pub btype: Btype,         // .btype
        pub po: (),               // .po
        pub sat: bool,            // {.sat}
        pub scale: Option<Scale>, // {.scale}
        pub d: GeneralOperand,    // d
        pub a: GeneralOperand,    // a
        pub asel: Option<Asel>,   // {.asel}
        pub b: GeneralOperand,    // b
        pub bsel: Option<Bsel>,   // {.bsel}
        pub c: GeneralOperand,    // c
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Asel as Asel0;
pub use section_0::Atype as Atype0;
pub use section_0::Bsel as Bsel0;
pub use section_0::Btype as Btype0;
pub use section_0::Dtype as Dtype0;
pub use section_0::Scale as Scale0;
pub use section_0::VmadDtypeAtypeBtypePoSatScale;
pub use section_0::VmadDtypeAtypeBtypeSatScale;
