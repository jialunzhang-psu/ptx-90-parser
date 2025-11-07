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
    pub enum Op2 {
        Add, // .add
        Min, // .min
        Max, // .max
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dsel {
        B0, // .b0
        B1, // .b1
        B2, // .b2
        B3, // .b3
        H0, // .h0
        H1, // .h1
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VaddDtypeAtypeBtypeSat {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VsubDtypeAtypeBtypeSat {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VabsdiffDtypeAtypeBtypeSat {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VminDtypeAtypeBtypeSat {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VmaxDtypeAtypeBtypeSat {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VaddDtypeAtypeBtypeSatOp2 {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub op2: Op2, // .op2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VsubDtypeAtypeBtypeSatOp2 {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub op2: Op2, // .op2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VabsdiffDtypeAtypeBtypeSatOp2 {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub op2: Op2, // .op2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VminDtypeAtypeBtypeSatOp2 {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub op2: Op2, // .op2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VmaxDtypeAtypeBtypeSatOp2 {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub op2: Op2, // .op2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VaddDtypeAtypeBtypeSat1 {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub d: GeneralOperand, // d
        pub dsel: Dsel, // .dsel
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VsubDtypeAtypeBtypeSat1 {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub d: GeneralOperand, // d
        pub dsel: Dsel, // .dsel
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VabsdiffDtypeAtypeBtypeSat1 {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub d: GeneralOperand, // d
        pub dsel: Dsel, // .dsel
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VminDtypeAtypeBtypeSat1 {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub d: GeneralOperand, // d
        pub dsel: Dsel, // .dsel
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VmaxDtypeAtypeBtypeSat1 {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub d: GeneralOperand, // d
        pub dsel: Dsel, // .dsel
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

}
