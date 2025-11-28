//! Original PTX specification:
//!
//! // 32-bit scalar operation, with optional secondary operation
//! vop.dtype.atype.btype{.sat}       d, a{.asel}, b{.bsel};
//! vop.dtype.atype.btype{.sat}.op2   d, a{.asel}, b{.bsel}, c;
//! // 32-bit scalar operation, with optional data merge
//! vop.dtype.atype.btype{.sat}  d.dsel, a{.asel}, b{.bsel}, c;
//! vop   = { vadd, vsub, vabsdiff, vmin, vmax };
//! .dtype = .atype = .btype = { .u32, .s32 };
//! .dsel  = .asel  = .bsel  = { .b0, .b1, .b2, .b3, .h0, .h1 };
//! .op2   = { .add, .min, .max };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Dtype {
        U32, // .u32
        S32, // .s32
    }

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
    pub struct VaddDtypeAtypeBtypeSat {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub d: GeneralOperand,  // d
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VsubDtypeAtypeBtypeSat {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub d: GeneralOperand,  // d
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VabsdiffDtypeAtypeBtypeSat {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub d: GeneralOperand,  // d
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VminDtypeAtypeBtypeSat {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub d: GeneralOperand,  // d
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VmaxDtypeAtypeBtypeSat {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub d: GeneralOperand,  // d
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VaddDtypeAtypeBtypeSatOp2 {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub op2: Op2,           // .op2
        pub d: GeneralOperand,  // d
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VsubDtypeAtypeBtypeSatOp2 {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub op2: Op2,           // .op2
        pub d: GeneralOperand,  // d
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VabsdiffDtypeAtypeBtypeSatOp2 {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub op2: Op2,           // .op2
        pub d: GeneralOperand,  // d
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VminDtypeAtypeBtypeSatOp2 {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub op2: Op2,           // .op2
        pub d: GeneralOperand,  // d
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VmaxDtypeAtypeBtypeSatOp2 {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub op2: Op2,           // .op2
        pub d: GeneralOperand,  // d
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VaddDtypeAtypeBtypeSat1 {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub d: GeneralOperand,  // d
        pub dsel: Dsel,         // .dsel
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VsubDtypeAtypeBtypeSat1 {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub d: GeneralOperand,  // d
        pub dsel: Dsel,         // .dsel
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VabsdiffDtypeAtypeBtypeSat1 {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub d: GeneralOperand,  // d
        pub dsel: Dsel,         // .dsel
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VminDtypeAtypeBtypeSat1 {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub d: GeneralOperand,  // d
        pub dsel: Dsel,         // .dsel
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VmaxDtypeAtypeBtypeSat1 {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub d: GeneralOperand,  // d
        pub dsel: Dsel,         // .dsel
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Asel as Asel0;
pub use section_0::Atype as Atype0;
pub use section_0::Bsel as Bsel0;
pub use section_0::Btype as Btype0;
pub use section_0::Dsel as Dsel0;
pub use section_0::Dtype as Dtype0;
pub use section_0::Op2 as Op20;
pub use section_0::VabsdiffDtypeAtypeBtypeSat;
pub use section_0::VabsdiffDtypeAtypeBtypeSat1;
pub use section_0::VabsdiffDtypeAtypeBtypeSatOp2;
pub use section_0::VaddDtypeAtypeBtypeSat;
pub use section_0::VaddDtypeAtypeBtypeSat1;
pub use section_0::VaddDtypeAtypeBtypeSatOp2;
pub use section_0::VmaxDtypeAtypeBtypeSat;
pub use section_0::VmaxDtypeAtypeBtypeSat1;
pub use section_0::VmaxDtypeAtypeBtypeSatOp2;
pub use section_0::VminDtypeAtypeBtypeSat;
pub use section_0::VminDtypeAtypeBtypeSat1;
pub use section_0::VminDtypeAtypeBtypeSatOp2;
pub use section_0::VsubDtypeAtypeBtypeSat;
pub use section_0::VsubDtypeAtypeBtypeSat1;
pub use section_0::VsubDtypeAtypeBtypeSatOp2;
