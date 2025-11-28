//! Original PTX specification:
//!
//! ldmatrix.sync.aligned.shape.num{.trans}{.ss}.type r, [p];
//! ldmatrix.sync.aligned.m8n16.num{.ss}.dst_fmt.src_fmt        r, [p];
//! ldmatrix.sync.aligned.m16n16.num.trans{.ss}.dst_fmt.src_fmt r, [p];
//! .shape   = {.m8n8, .m16n16};
//! .num     = {.x1, .x2, .x4};
//! .ss      = {.shared, .shared::cta};
//! .type    = {.b16, .b8};
//! .dst_fmt = { .b8x16 };
//! .src_fmt = { .b6x16_p32, .b4x16_p64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Shape {
        M16n16, // .m16n16
        M8n8, // .m8n8
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Num {
        X1, // .x1
        X2, // .x2
        X4, // .x4
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Ss {
        SharedCta, // .shared::cta
        Shared, // .shared
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        B16, // .b16
        B8, // .b8
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum DstFmt {
        B8x16, // .b8x16
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum SrcFmt {
        B6x16P32, // .b6x16_p32
        B4x16P64, // .b4x16_p64
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct LdmatrixSyncAlignedShapeNumTransSsType {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub shape: Shape, // .shape
        pub num: Num, // .num
        pub trans: bool, // {.trans}
        pub ss: Option<Ss>, // {.ss}
        pub type_: Type, // .type
        pub r: GeneralOperand, // r
        pub p: AddressOperand, // [p]
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct LdmatrixSyncAlignedM8n16NumSsDstFmtSrcFmt {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub m8n16: (), // .m8n16
        pub num: Num, // .num
        pub ss: Option<Ss>, // {.ss}
        pub dst_fmt: DstFmt, // .dst_fmt
        pub src_fmt: SrcFmt, // .src_fmt
        pub r: GeneralOperand, // r
        pub p: AddressOperand, // [p]
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct LdmatrixSyncAlignedM16n16NumTransSsDstFmtSrcFmt {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub m16n16: (), // .m16n16
        pub num: Num, // .num
        pub trans: (), // .trans
        pub ss: Option<Ss>, // {.ss}
        pub dst_fmt: DstFmt, // .dst_fmt
        pub src_fmt: SrcFmt, // .src_fmt
        pub r: GeneralOperand, // r
        pub p: AddressOperand, // [p]
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::LdmatrixSyncAlignedShapeNumTransSsType;
pub use section_0::LdmatrixSyncAlignedM8n16NumSsDstFmtSrcFmt;
pub use section_0::LdmatrixSyncAlignedM16n16NumTransSsDstFmtSrcFmt;
pub use section_0::Shape as Shape0;
pub use section_0::Num as Num0;
pub use section_0::Ss as Ss0;
pub use section_0::Type as Type0;
pub use section_0::DstFmt as DstFmt0;
pub use section_0::SrcFmt as SrcFmt0;
