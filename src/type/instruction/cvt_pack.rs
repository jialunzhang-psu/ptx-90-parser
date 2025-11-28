//! Original PTX specification:
//!
//! cvt.pack.sat.convertType.abType  d, a, b;
//! .convertType  = { .u16, .s16 };
//! .abType       = { .s32 };
//! ----------------------------------------------------------------
//! cvt.pack.sat.convertType.abType.cType  d, a, b, c;
//! .convertType  = { .u2, .s2, .u4, .s4, .u8, .s8 };
//! .abType       = { .s32 };
//! .cType        = { .b32 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Converttype {
        U16, // .u16
        S16, // .s16
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Abtype {
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct CvtPackSatConverttypeAbtype {
        pub pack: (), // .pack
        pub sat: (), // .sat
        pub converttype: Converttype, // .convertType
        pub abtype: Abtype, // .abType
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

}

pub mod section_1 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Converttype {
        U2, // .u2
        S2, // .s2
        U4, // .u4
        S4, // .s4
        U8, // .u8
        S8, // .s8
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Abtype {
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Ctype {
        B32, // .b32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct CvtPackSatConverttypeAbtypeCtype {
        pub pack: (), // .pack
        pub sat: (), // .sat
        pub converttype: Converttype, // .convertType
        pub abtype: Abtype, // .abType
        pub ctype: Ctype, // .cType
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::CvtPackSatConverttypeAbtype;
pub use section_0::Converttype as Converttype0;
pub use section_0::Abtype as Abtype0;
pub use section_1::CvtPackSatConverttypeAbtypeCtype;
pub use section_1::Converttype as Converttype1;
pub use section_1::Abtype as Abtype1;
pub use section_1::Ctype as Ctype1;
