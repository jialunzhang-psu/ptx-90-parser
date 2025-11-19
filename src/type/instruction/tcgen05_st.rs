//! Original PTX specification:
//!
//! tcgen05.st.sync.aligned.shape1.num{.unpack}.b32    [taddr], r;
//! tcgen05.st.sync.aligned.shape2.num{.unpack}.b32    [taddr], immHalfSplitoff, r;
//! .shape1 = { .16x64b, .16x128b, .16x256b, .32x32b };
//! .shape2 = { .16x32bx2 };
//! .num    = { .x1, .x2, .x4, .x8, .x16, .x32, .x64, .x128 };
//! .unpack = { .unpack::16b };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape1 {
        _16x128b, // .16x128b
        _16x256b, // .16x256b
        _16x64b,  // .16x64b
        _32x32b,  // .32x32b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Num {
        X128, // .x128
        X16,  // .x16
        X32,  // .x32
        X64,  // .x64
        X1,   // .x1
        X2,   // .x2
        X4,   // .x4
        X8,   // .x8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Unpack {
        Unpack16b, // .unpack::16b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape2 {
        _16x32bx2, // .16x32bx2
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05StSyncAlignedShape1NumUnpackB32 {
        pub st: (),                 // .st
        pub sync: (),               // .sync
        pub aligned: (),            // .aligned
        pub shape1: Shape1,         // .shape1
        pub num: Num,               // .num
        pub unpack: Option<Unpack>, // {.unpack}
        pub b32: (),                // .b32
        pub taddr: AddressOperand,  // [taddr]
        pub r: GeneralOperand,      // r
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05StSyncAlignedShape2NumUnpackB32 {
        pub st: (),                          // .st
        pub sync: (),                        // .sync
        pub aligned: (),                     // .aligned
        pub shape2: Shape2,                  // .shape2
        pub num: Num,                        // .num
        pub unpack: Option<Unpack>,          // {.unpack}
        pub b32: (),                         // .b32
        pub taddr: AddressOperand,           // [taddr]
        pub immhalfsplitoff: GeneralOperand, // immHalfSplitoff
        pub r: GeneralOperand,               // r
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Num as Num0;
pub use section_0::Shape1 as Shape10;
pub use section_0::Shape2 as Shape20;
pub use section_0::Tcgen05StSyncAlignedShape1NumUnpackB32;
pub use section_0::Tcgen05StSyncAlignedShape2NumUnpackB32;
pub use section_0::Unpack as Unpack0;
