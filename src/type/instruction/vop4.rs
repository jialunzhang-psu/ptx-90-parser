//! Original PTX specification:
//!
//! // SIMD instruction with secondary SIMD merge operation
//! vop4.dtype.atype.btype{.sat}  d{.mask}, a{.asel}, b{.bsel}, c;
//! // SIMD instruction with secondary accumulate operation
//! vop4.dtype.atype.btype.add  d{.mask}, a{.asel}, b{.bsel}, c;
//! vop4  = { vadd4, vsub4, vavrg4, vabsdiff4, vmin4, vmax4 };
//! .dtype = .atype = .btype = { .u32, .s32 };
//! .mask  = { .b0,
//! .b1, .b10
//! .b2, .b20, .b21, .b210,
//! .b3, .b30, .b31, .b310, .b32, .b320, .b321, .b3210 };
//! // defaults to .b3210
//! .asel = .bsel = { .b.n.n.n.n }
//! n = { 0, 1, 2, 3, 4, 5, 6, 7}
//! // .asel defaults to .b3210
//! // .bsel defaults to .b7654

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
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
        B0, // .b0
        B1, // .b1
        B10B2, // .b10.b2
        B20, // .b20
        B21, // .b21
        B210, // .b210
        B3, // .b3
        B30, // .b30
        B31, // .b31
        B310, // .b310
        B32, // .b32
        B320, // .b320
        B321, // .b321
        B3210, // .b3210
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Asel {
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
    pub enum Bsel {
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
    pub struct Vop4DtypeAtypeBtypeSat {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub d: Operand, // d
        pub mask: Option<Mask>, // {.mask}
        pub a: Operand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: Operand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: Operand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Vop4DtypeAtypeBtypeAdd {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub add: (), // .add
        pub d: Operand, // d
        pub mask: Option<Mask>, // {.mask}
        pub a: Operand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: Operand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: Operand, // c
    }

}
