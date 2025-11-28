//! Original PTX specification:
//!
//! mul.mode.type  d, a, b;
//! .mode = { .hi, .lo, .wide };
//! .type = { .u16, .u32, .u64,
//! .s16, .s32, .s64 };
//! --------------------------------------------
//! mul{.rnd}{.ftz}{.sat}.f32  d, a, b;
//! mul{.rnd}{.ftz}.f32x2      d, a, b;
//! mul{.rnd}.f64              d, a, b;
//! .rnd = { .rn, .rz, .rm, .rp };
//! --------------------------------------------
//! mul{.rnd}{.ftz}{.sat}.f16   d, a, b;
//! mul{.rnd}{.ftz}{.sat}.f16x2 d, a, b;
//! mul{.rnd}.bf16   d, a, b;
//! mul{.rnd}.bf16x2 d, a, b;
//! .rnd = { .rn };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Mode {
        Wide, // .wide
        Hi,   // .hi
        Lo,   // .lo
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        U16, // .u16
        U32, // .u32
        U64, // .u64
        S16, // .s16
        S32, // .s32
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MulModeType {
        pub mode: Mode,        // .mode
        pub type_: Type,       // .type
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

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Rnd {
        Rn, // .rn
        Rz, // .rz
        Rm, // .rm
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MulRndFtzSatF32 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub ftz: bool,         // {.ftz}
        pub sat: bool,         // {.sat}
        pub f32: (),           // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MulRndFtzF32x2 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub ftz: bool,         // {.ftz}
        pub f32x2: (),         // .f32x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MulRndF64 {
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

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Rnd {
        Rn, // .rn
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MulRndFtzSatF16 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub ftz: bool,         // {.ftz}
        pub sat: bool,         // {.sat}
        pub f16: (),           // .f16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MulRndFtzSatF16x2 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub ftz: bool,         // {.ftz}
        pub sat: bool,         // {.sat}
        pub f16x2: (),         // .f16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MulRndBf16 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub bf16: (),          // .bf16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MulRndBf16x2 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub bf16x2: (),        // .bf16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Mode as Mode0;
pub use section_0::MulModeType;
pub use section_0::Type as Type0;
pub use section_1::MulRndF64;
pub use section_1::MulRndFtzF32x2;
pub use section_1::MulRndFtzSatF32;
pub use section_1::Rnd as Rnd1;
pub use section_2::MulRndBf16;
pub use section_2::MulRndBf16x2;
pub use section_2::MulRndFtzSatF16;
pub use section_2::MulRndFtzSatF16x2;
pub use section_2::Rnd as Rnd2;
