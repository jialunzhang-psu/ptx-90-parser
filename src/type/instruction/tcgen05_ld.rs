//! Original PTX specification:
//!
//! // Base load instruction:
//! tcgen05.ld.sync.aligned.shape1.num{.pack}.b32    r, [taddr];
//! tcgen05.ld.sync.aligned.shape2.num{.pack}.b32    r, [taddr], immHalfSplitoff;
//! .shape1 = { .16x64b, .16x128b, .16x256b, .32x32b };
//! .shape2 = { .16x32bx2 };
//! .num    = { .x1, .x2, .x4, .x8, .x16, .x32, .x64, .x128 };
//! .pack   = { .pack::16b };
//! // Floating point type load along with reduction :
//! tcgen05.ld.red.sync.aligned.shape3.num.redOp{.abs}{.NaN}.f32 r, redval, [taddr];
//! tcgen05.ld.red.sync.aligned.shape4.num.redOp{.abs}{.NaN}.f32 r, redval, [taddr], immHalfSplitoff;
//! // Integer type load along with reduction :
//! tcgen05.ld.red.sync.aligned.shape3.num.redOp.type r, redval, [taddr];
//! tcgen05.ld.red.sync.aligned.shape4.num.redOp.type r, redval, [taddr], immHalfSplitoff;
//! .shape3 = { .32x32b   };
//! .shape4 = { .16x32bx2 };
//! .redOp  = { .min, .max };
//! .type   = { .u32, .s32 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape1 {
        _16x64b, // .16x64b
        _16x128b, // .16x128b
        _16x256b, // .16x256b
        _32x32b, // .32x32b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Num {
        X1, // .x1
        X2, // .x2
        X4, // .x4
        X8, // .x8
        X16, // .x16
        X32, // .x32
        X64, // .x64
        X128, // .x128
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Pack {
        Pack16b, // .pack::16b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape2 {
        _16x32bx2, // .16x32bx2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape3 {
        _32x32b, // .32x32b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Redop {
        Min, // .min
        Max, // .max
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape4 {
        _16x32bx2, // .16x32bx2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05LdSyncAlignedShape1NumPackB32 {
        pub ld: (), // .ld
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub shape1: Shape1, // .shape1
        pub num: Num, // .num
        pub pack: Option<Pack>, // {.pack}
        pub b32: (), // .b32
        pub r: Operand, // r
        pub taddr: AddressOperand, // [taddr]
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05LdSyncAlignedShape2NumPackB32 {
        pub ld: (), // .ld
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub shape2: Shape2, // .shape2
        pub num: Num, // .num
        pub pack: Option<Pack>, // {.pack}
        pub b32: (), // .b32
        pub r: Operand, // r
        pub taddr: AddressOperand, // [taddr]
        pub immhalfsplitoff: Operand, // immHalfSplitoff
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05LdRedSyncAlignedShape3NumRedopAbsNanF32 {
        pub ld: (), // .ld
        pub red: (), // .red
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub shape3: Shape3, // .shape3
        pub num: Num, // .num
        pub redop: Redop, // .redOp
        pub abs: bool, // {.abs}
        pub nan: bool, // {.NaN}
        pub f32: (), // .f32
        pub r: Operand, // r
        pub redval: Operand, // redval
        pub taddr: AddressOperand, // [taddr]
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05LdRedSyncAlignedShape4NumRedopAbsNanF32 {
        pub ld: (), // .ld
        pub red: (), // .red
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub shape4: Shape4, // .shape4
        pub num: Num, // .num
        pub redop: Redop, // .redOp
        pub abs: bool, // {.abs}
        pub nan: bool, // {.NaN}
        pub f32: (), // .f32
        pub r: Operand, // r
        pub redval: Operand, // redval
        pub taddr: AddressOperand, // [taddr]
        pub immhalfsplitoff: Operand, // immHalfSplitoff
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05LdRedSyncAlignedShape3NumRedopType {
        pub ld: (), // .ld
        pub red: (), // .red
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub shape3: Shape3, // .shape3
        pub num: Num, // .num
        pub redop: Redop, // .redOp
        pub type_: Type, // .type
        pub r: Operand, // r
        pub redval: Operand, // redval
        pub taddr: AddressOperand, // [taddr]
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05LdRedSyncAlignedShape4NumRedopType {
        pub ld: (), // .ld
        pub red: (), // .red
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub shape4: Shape4, // .shape4
        pub num: Num, // .num
        pub redop: Redop, // .redOp
        pub type_: Type, // .type
        pub r: Operand, // r
        pub redval: Operand, // redval
        pub taddr: AddressOperand, // [taddr]
        pub immhalfsplitoff: Operand, // immHalfSplitoff
    }

}
