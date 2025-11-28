//! Original PTX specification:
//!
//! tensormap.replace.mode.field1{.ss}.b1024.type  [addr], new_val;
//! tensormap.replace.mode.field2{.ss}.b1024.type  [addr], ord, new_val;
//! tensormap.replace.mode.field3{.ss}.b1024.type  [addr], new_val;
//! .mode    = { .tile };
//! .field1  = { .global_address, .rank };
//! .field2  = { .box_dim, .global_dim, .global_stride, .element_stride  };
//! .field3  = { .elemtype,  .interleave_layout, .swizzle_mode, .swizzle_atomicity, .fill_mode };
//! .ss      = { .global, .shared::cta };
//! .type    = { .b32, .b64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Mode {
        Tile, // .tile
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Field1 {
        GlobalAddress, // .global_address
        Rank,          // .rank
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Ss {
        SharedCta, // .shared::cta
        Global,    // .global
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        B32, // .b32
        B64, // .b64
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Field2 {
        ElementStride, // .element_stride
        GlobalStride,  // .global_stride
        GlobalDim,     // .global_dim
        BoxDim,        // .box_dim
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Field3 {
        InterleaveLayout, // .interleave_layout
        SwizzleAtomicity, // .swizzle_atomicity
        SwizzleMode,      // .swizzle_mode
        FillMode,         // .fill_mode
        Elemtype,         // .elemtype
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct TensormapReplaceModeField1SsB1024Type {
        pub replace: (),             // .replace
        pub mode: Mode,              // .mode
        pub field1: Field1,          // .field1
        pub ss: Option<Ss>,          // {.ss}
        pub b1024: (),               // .b1024
        pub type_: Type,             // .type
        pub addr: AddressOperand,    // [addr]
        pub new_val: GeneralOperand, // new_val
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct TensormapReplaceModeField2SsB1024Type {
        pub replace: (),             // .replace
        pub mode: Mode,              // .mode
        pub field2: Field2,          // .field2
        pub ss: Option<Ss>,          // {.ss}
        pub b1024: (),               // .b1024
        pub type_: Type,             // .type
        pub addr: AddressOperand,    // [addr]
        pub ord: GeneralOperand,     // ord
        pub new_val: GeneralOperand, // new_val
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct TensormapReplaceModeField3SsB1024Type {
        pub replace: (),             // .replace
        pub mode: Mode,              // .mode
        pub field3: Field3,          // .field3
        pub ss: Option<Ss>,          // {.ss}
        pub b1024: (),               // .b1024
        pub type_: Type,             // .type
        pub addr: AddressOperand,    // [addr]
        pub new_val: GeneralOperand, // new_val
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Field1 as Field10;
pub use section_0::Field2 as Field20;
pub use section_0::Field3 as Field30;
pub use section_0::Mode as Mode0;
pub use section_0::Ss as Ss0;
pub use section_0::TensormapReplaceModeField1SsB1024Type;
pub use section_0::TensormapReplaceModeField2SsB1024Type;
pub use section_0::TensormapReplaceModeField3SsB1024Type;
pub use section_0::Type as Type0;
