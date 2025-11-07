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
        Shr7, // .shr7
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

    #[derive(Debug, Clone, PartialEq)]
    pub struct VmadDtypeAtypeBtypeSatScale {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub scale: Option<Scale>, // {.scale}
        pub d: GeneralOperand, // d
        pub a_op: bool, // {-} operator
        pub a: GeneralOperand, // {-}a
        pub asel: Option<Asel>, // {.asel}
        pub b_op: bool, // {-} operator
        pub b: GeneralOperand, // {-}b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c_op: bool, // {-} operator
        pub c: GeneralOperand, // {-}c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VmadDtypeAtypeBtypePoSatScale {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub po: (), // .po
        pub sat: bool, // {.sat}
        pub scale: Option<Scale>, // {.scale}
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

}
