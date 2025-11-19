//! Original PTX specification:
//!
//! // SIMD instruction with secondary SIMD merge operation
//! vop4.dtype.atype.btype{.sat}  d{.mask}, a{.asel}, b{.bsel}, c;
//! // SIMD instruction with secondary accumulate operation
//! vop4.dtype.atype.btype.add  d{.mask}, a{.asel}, b{.bsel}, c;
//! vop4  = { vadd4, vsub4, vavrg4, vabsdiff4, vmin4, vmax4 };
//! .dtype = .atype = .btype = { .u32, .s32 };
//! .mask  = { .b0,
//! .b1, .b10,
//! .b2, .b20, .b21, .b210,
//! .b3, .b30, .b31, .b310, .b32, .b320, .b321, .b3210 };
//! // defaults to .b3210
//! .asel = .bsel = { .b.n.n.n.n };
//! .n = { 0, 1, 2, 3, 4, 5, 6, 7};
//! // .asel defaults to .b3210
//! // .bsel defaults to .b7654

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
    pub enum Mask {
        B3210, // .b3210
        B210,  // .b210
        B310,  // .b310
        B320,  // .b320
        B321,  // .b321
        B10,   // .b10
        B20,   // .b20
        B21,   // .b21
        B30,   // .b30
        B31,   // .b31
        B32,   // .b32
        B0,    // .b0
        B1,    // .b1
        B2,    // .b2
        B3,    // .b3
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum N {
        _0, // 0
        _1, // 1
        _2, // 2
        _3, // 3
        _4, // 4
        _5, // 5
        _6, // 6
        _7, // 7
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Asel {
        BNNNN((), N, N, N, N), // .b.n.n.n.n
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Bsel {
        BNNNN((), N, N, N, N), // .b.n.n.n.n
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Vadd4DtypeAtypeBtypeSat {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub d: GeneralOperand,  // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Vsub4DtypeAtypeBtypeSat {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub d: GeneralOperand,  // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Vavrg4DtypeAtypeBtypeSat {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub d: GeneralOperand,  // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Vabsdiff4DtypeAtypeBtypeSat {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub d: GeneralOperand,  // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Vmin4DtypeAtypeBtypeSat {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub d: GeneralOperand,  // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Vmax4DtypeAtypeBtypeSat {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub sat: bool,          // {.sat}
        pub d: GeneralOperand,  // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Vadd4DtypeAtypeBtypeAdd {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub add: (),            // .add
        pub d: GeneralOperand,  // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Vsub4DtypeAtypeBtypeAdd {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub add: (),            // .add
        pub d: GeneralOperand,  // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Vavrg4DtypeAtypeBtypeAdd {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub add: (),            // .add
        pub d: GeneralOperand,  // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Vabsdiff4DtypeAtypeBtypeAdd {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub add: (),            // .add
        pub d: GeneralOperand,  // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Vmin4DtypeAtypeBtypeAdd {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub add: (),            // .add
        pub d: GeneralOperand,  // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Vmax4DtypeAtypeBtypeAdd {
        pub dtype: Dtype,       // .dtype
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub add: (),            // .add
        pub d: GeneralOperand,  // d
        pub mask: Option<Mask>, // {.mask}
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
pub use section_0::Dtype as Dtype0;
pub use section_0::Mask as Mask0;
pub use section_0::N as N0;
pub use section_0::Vabsdiff4DtypeAtypeBtypeAdd;
pub use section_0::Vabsdiff4DtypeAtypeBtypeSat;
pub use section_0::Vadd4DtypeAtypeBtypeAdd;
pub use section_0::Vadd4DtypeAtypeBtypeSat;
pub use section_0::Vavrg4DtypeAtypeBtypeAdd;
pub use section_0::Vavrg4DtypeAtypeBtypeSat;
pub use section_0::Vmax4DtypeAtypeBtypeAdd;
pub use section_0::Vmax4DtypeAtypeBtypeSat;
pub use section_0::Vmin4DtypeAtypeBtypeAdd;
pub use section_0::Vmin4DtypeAtypeBtypeSat;
pub use section_0::Vsub4DtypeAtypeBtypeAdd;
pub use section_0::Vsub4DtypeAtypeBtypeSat;
