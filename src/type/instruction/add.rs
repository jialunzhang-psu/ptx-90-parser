//! Original PTX specification:
//!
//! add.type       d, a, b;
//! add{.sat}.s32  d, a, b;     // .sat applies only to .s32
//! .type = { .u16, .u32, .u64,
//! .s16, .s32, .s64,
//! .u16x2, .s16x2 };
//! -------------------------------------------
//! add{.rnd}{.ftz}{.sat}.f32  d, a, b;
//! add{.rnd}{.ftz}.f32x2      d, a, b;
//! add{.rnd}.f64              d, a, b;
//! .rnd = { .rn, .rz, .rm, .rp };
//! --------------------------------------------
//! add{.rnd}{.ftz}{.sat}.f16   d, a, b;
//! add{.rnd}{.ftz}{.sat}.f16x2 d, a, b;
//! add{.rnd}.bf16   d, a, b;
//! add{.rnd}.bf16x2 d, a, b;
//! .rnd = { .rn };
//! --------------------------------------------
//! add{.rnd}{.sat}.f32.atype  d, a, c;
//! .atype = { .f16, .bf16};
//! .rnd   = { .rn, .rz, .rm, .rp };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U16x2, // .u16x2
        S16x2, // .s16x2
        U16,   // .u16
        U32,   // .u32
        U64,   // .u64
        S16,   // .s16
        S32,   // .s32
        S64,   // .s64
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct AddType {
        pub type_: Type,       // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct AddSatS32 {
        pub sat: bool,         // {.sat}
        pub s32: (),           // .s32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }
}

pub mod section_1 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Rnd {
        Rn, // .rn
        Rz, // .rz
        Rm, // .rm
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct AddRndFtzSatF32 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub ftz: bool,         // {.ftz}
        pub sat: bool,         // {.sat}
        pub f32: (),           // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct AddRndFtzF32x2 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub ftz: bool,         // {.ftz}
        pub f32x2: (),         // .f32x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct AddRndF64 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub f64: (),           // .f64
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }
}

pub mod section_2 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Rnd {
        Rn, // .rn
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct AddRndFtzSatF16 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub ftz: bool,         // {.ftz}
        pub sat: bool,         // {.sat}
        pub f16: (),           // .f16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct AddRndFtzSatF16x2 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub ftz: bool,         // {.ftz}
        pub sat: bool,         // {.sat}
        pub f16x2: (),         // .f16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct AddRndBf16 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub bf16: (),          // .bf16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct AddRndBf16x2 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub bf16x2: (),        // .bf16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }
}

pub mod section_3 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Rnd {
        Rn, // .rn
        Rz, // .rz
        Rm, // .rm
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Atype {
        Bf16, // .bf16
        F16,  // .f16
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct AddRndSatF32Atype {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub sat: bool,         // {.sat}
        pub f32: (),           // .f32
        pub atype: Atype,      // .atype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub c: GeneralOperand, // c
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::AddSatS32;
pub use section_0::AddType;
pub use section_0::Type as Type0;
pub use section_1::AddRndF64;
pub use section_1::AddRndFtzF32x2;
pub use section_1::AddRndFtzSatF32;
pub use section_1::Rnd as Rnd1;
pub use section_2::AddRndBf16;
pub use section_2::AddRndBf16x2;
pub use section_2::AddRndFtzSatF16;
pub use section_2::AddRndFtzSatF16x2;
pub use section_2::Rnd as Rnd2;
pub use section_3::AddRndSatF32Atype;
pub use section_3::Atype as Atype3;
pub use section_3::Rnd as Rnd3;
