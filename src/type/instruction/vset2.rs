//! Original PTX specification:
//!
//! // SIMD instruction with secondary SIMD merge operation
//! vset2.atype.btype.cmp  d{.mask}, a{.asel}, b{.bsel}, c;
//! // SIMD instruction with secondary accumulate operation
//! vset2.atype.btype.cmp.add  d{.mask}, a{.asel}, b{.bsel}, c;
//! .atype = .btype = { .u32, .s32 };
//! .cmp   = { .eq, .ne, .lt, .le, .gt, .ge };
//! .mask  = { .h0, .h1, .h10 };  // defaults to .h10
//! .asel  = .bsel  = { .h00, .h01, .h02, .h03, .h10, .h11, .h12, .h13, .h20, .h21, .h22, .h23, .h30, .h31, .h32, .h33 }; // { .hxy, where x,y are from { 0, 1, 2, 3 } };
//! // .asel defaults to .h10
//! // .bsel defaults to .h32

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

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
    pub enum Cmp {
        Eq, // .eq
        Ne, // .ne
        Lt, // .lt
        Le, // .le
        Gt, // .gt
        Ge, // .ge
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Mask {
        H10, // .h10
        H0,  // .h0
        H1,  // .h1
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
    pub struct Vset2AtypeBtypeCmp {
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub cmp: Cmp,           // .cmp
        pub d: GeneralOperand,  // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Vset2AtypeBtypeCmpAdd {
        pub atype: Atype,       // .atype
        pub btype: Btype,       // .btype
        pub cmp: Cmp,           // .cmp
        pub add: (),            // .add
        pub d: GeneralOperand,  // d
        pub mask: Option<Mask>, // {.mask}
        pub a: GeneralOperand,  // a
        pub asel: Option<Asel>, // {.asel}
        pub b: GeneralOperand,  // b
        pub bsel: Option<Bsel>, // {.bsel}
        pub c: GeneralOperand,  // c
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Asel as Asel0;
pub use section_0::Atype as Atype0;
pub use section_0::Bsel as Bsel0;
pub use section_0::Btype as Btype0;
pub use section_0::Cmp as Cmp0;
pub use section_0::Mask as Mask0;
pub use section_0::Vset2AtypeBtypeCmp;
pub use section_0::Vset2AtypeBtypeCmpAdd;
