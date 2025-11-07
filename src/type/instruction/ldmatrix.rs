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

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape {
        M16n16, // .m16n16
        M8n8, // .m8n8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Num {
        X1, // .x1
        X2, // .x2
        X4, // .x4
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        SharedCta, // .shared::cta
        Shared, // .shared
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B16, // .b16
        B8, // .b8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum DstFmt {
        B8x16, // .b8x16
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum SrcFmt {
        B6x16P32, // .b6x16_p32
        B4x16P64, // .b4x16_p64
    }

    #[derive(Debug, Clone, PartialEq)]
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
    }

    #[derive(Debug, Clone, PartialEq)]
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
    }

    #[derive(Debug, Clone, PartialEq)]
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
    }

}
