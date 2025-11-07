//! Original PTX specification:
//!
//! // SIMD instruction with secondary SIMD merge operation
//! vop2.dtype.atype.btype{.sat}  d{.mask}, a{.asel}, b{.bsel}, c;
//! // SIMD instruction with secondary accumulate operation
//! vop2.dtype.atype.btype.add  d{.mask}, a{.asel}, b{.bsel}, c;
//! vop2  = { vadd2, vsub2, vavrg2, vabsdiff2, vmin2, vmax2 };
//! .dtype = .atype = .btype = { .u32, .s32 };
//! .mask  = { .h0, .h1, .h10 };  // defaults to .h10
//! .asel  = .bsel  = { .h00, .h01, .h02, .h03, .h10, .h11, .h12, .h13, .h20, .h21, .h22, .h23, .h30, .h31, .h32, .h33 }; 
//! // .asel defaults to .h10
//! // .bsel defaults to .h32

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
        H10, // .h10
        H0, // .h0
        H1, // .h1
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Asel {
        H00, // .h00
        H01, // .h01
        H02, // .h02
        H03, // .h03
        H10, // .h10
        H11, // .h11
        H12, // .h12
        H13, // .h13
        H20, // .h20
        H21, // .h21
        H22, // .h22
        H23, // .h23
        H30, // .h30
        H31, // .h31
        H32, // .h32
        H33, // .h33
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Bsel {
        H00, // .h00
        H01, // .h01
        H02, // .h02
        H03, // .h03
        H10, // .h10
        H11, // .h11
        H12, // .h12
        H13, // .h13
        H20, // .h20
        H21, // .h21
        H22, // .h22
        H23, // .h23
        H30, // .h30
        H31, // .h31
        H32, // .h32
        H33, // .h33
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Vadd2DtypeAtypeBtypeSat {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub d: GeneralOperand, // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Vsub2DtypeAtypeBtypeSat {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub d: GeneralOperand, // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Vavrg2DtypeAtypeBtypeSat {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub d: GeneralOperand, // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Vabsdiff2DtypeAtypeBtypeSat {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub d: GeneralOperand, // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Vmin2DtypeAtypeBtypeSat {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub d: GeneralOperand, // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Vmax2DtypeAtypeBtypeSat {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub sat: bool, // {.sat}
        pub d: GeneralOperand, // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Vadd2DtypeAtypeBtypeAdd {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub add: (), // .add
        pub d: GeneralOperand, // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Vsub2DtypeAtypeBtypeAdd {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub add: (), // .add
        pub d: GeneralOperand, // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Vavrg2DtypeAtypeBtypeAdd {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub add: (), // .add
        pub d: GeneralOperand, // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Vabsdiff2DtypeAtypeBtypeAdd {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub add: (), // .add
        pub d: GeneralOperand, // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Vmin2DtypeAtypeBtypeAdd {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub add: (), // .add
        pub d: GeneralOperand, // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Vmax2DtypeAtypeBtypeAdd {
        pub dtype: Dtype, // .dtype
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub add: (), // .add
        pub d: GeneralOperand, // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand, // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand, // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand, // c
    }

}
