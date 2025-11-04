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
//! .asel = .bsel = { .b00, .b01, .b02, .b03, .b04, .b05, .b06, .b07,
//!                   .b10, .b11, .b12, .b13, .b14, .b15, .b16, .b17,
//!                   .b20, .b21, .b22, .b23, .b24, .b25, .b26, .b27,
//!                   .b30, .b31, .b32, .b33, .b34, .b35, .b36, .b37,
//!                   .b40, .b41, .b42, .b43, .b44, .b45, .b46, .b47,
//!                   .b50, .b51, .b52, .b53, .b54, .b55, .b56, .b57,
//!                   .b60, .b61, .b62, .b63, .b64, .b65, .b66, .b67,
//!                   .b70, .b71, .b72, .b73, .b74, .b75, .b76, .b77
//!                   } // where x,y,z,w are from { 0, ..., 7 };
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
        B00, // .b00
        B01, // .b01
        B02, // .b02
        B03, // .b03
        B04, // .b04
        B05, // .b05
        B06, // .b06
        B07, // .b07
        B10, // .b10
        B11, // .b11
        B12, // .b12
        B13, // .b13
        B14, // .b14
        B15, // .b15
        B16, // .b16
        B17, // .b17
        B20, // .b20
        B21, // .b21
        B22, // .b22
        B23, // .b23
        B24, // .b24
        B25, // .b25
        B26, // .b26
        B27, // .b27
        B30, // .b30
        B31, // .b31
        B32, // .b32
        B33, // .b33
        B34, // .b34
        B35, // .b35
        B36, // .b36
        B37, // .b37
        B40, // .b40
        B41, // .b41
        B42, // .b42
        B43, // .b43
        B44, // .b44
        B45, // .b45
        B46, // .b46
        B47, // .b47
        B50, // .b50
        B51, // .b51
        B52, // .b52
        B53, // .b53
        B54, // .b54
        B55, // .b55
        B56, // .b56
        B57, // .b57
        B60, // .b60
        B61, // .b61
        B62, // .b62
        B63, // .b63
        B64, // .b64
        B65, // .b65
        B66, // .b66
        B67, // .b67
        B70, // .b70
        B71, // .b71
        B72, // .b72
        B73, // .b73
        B74, // .b74
        B75, // .b75
        B76, // .b76
        B77, // .b77
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Bsel {
        B00, // .b00
        B01, // .b01
        B02, // .b02
        B03, // .b03
        B04, // .b04
        B05, // .b05
        B06, // .b06
        B07, // .b07
        B10, // .b10
        B11, // .b11
        B12, // .b12
        B13, // .b13
        B14, // .b14
        B15, // .b15
        B16, // .b16
        B17, // .b17
        B20, // .b20
        B21, // .b21
        B22, // .b22
        B23, // .b23
        B24, // .b24
        B25, // .b25
        B26, // .b26
        B27, // .b27
        B30, // .b30
        B31, // .b31
        B32, // .b32
        B33, // .b33
        B34, // .b34
        B35, // .b35
        B36, // .b36
        B37, // .b37
        B40, // .b40
        B41, // .b41
        B42, // .b42
        B43, // .b43
        B44, // .b44
        B45, // .b45
        B46, // .b46
        B47, // .b47
        B50, // .b50
        B51, // .b51
        B52, // .b52
        B53, // .b53
        B54, // .b54
        B55, // .b55
        B56, // .b56
        B57, // .b57
        B60, // .b60
        B61, // .b61
        B62, // .b62
        B63, // .b63
        B64, // .b64
        B65, // .b65
        B66, // .b66
        B67, // .b67
        B70, // .b70
        B71, // .b71
        B72, // .b72
        B73, // .b73
        B74, // .b74
        B75, // .b75
        B76, // .b76
        B77, // .b77
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
