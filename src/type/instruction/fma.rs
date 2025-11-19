//! Original PTX specification:
//!
//! fma.rnd{.ftz}{.sat}.f32  d, a, b, c;
//! fma.rnd{.ftz}.f32x2      d, a, b, c;
//! fma.rnd.f64              d, a, b, c;
//! .rnd = { .rn, .rz, .rm, .rp };
//! ---------------------------------------------
//! fma.rnd{.ftz}{.sat}.f16     d, a, b, c;
//! fma.rnd{.ftz}{.sat}.f16x2   d, a, b, c;
//! fma.rnd{.ftz}.relu.f16      d, a, b, c;
//! fma.rnd{.ftz}.relu.f16x2    d, a, b, c;
//! fma.rnd{.relu}.bf16         d, a, b, c;
//! fma.rnd{.relu}.bf16x2       d, a, b, c;
//! fma.rnd.oob{.relu}.type     d, a, b, c;
//! .rnd = { .rn };
//! ---------------------------------------------
//! fma.rnd{.sat}.f32.abtype  d, a, b, c;
//! .abtype = { .f16, .bf16};
//! .rnd    = { .rn, .rz, .rm, .rp };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
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
    pub struct FmaRndFtzSatF32 {
        pub rnd: Rnd,          // .rnd
        pub ftz: bool,         // {.ftz}
        pub sat: bool,         // {.sat}
        pub f32: (),           // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct FmaRndFtzF32x2 {
        pub rnd: Rnd,          // .rnd
        pub ftz: bool,         // {.ftz}
        pub f32x2: (),         // .f32x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct FmaRndF64 {
        pub rnd: Rnd,          // .rnd
        pub f64: (),           // .f64
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
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
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct FmaRndFtzSatF16 {
        pub rnd: Rnd,          // .rnd
        pub ftz: bool,         // {.ftz}
        pub sat: bool,         // {.sat}
        pub f16: (),           // .f16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct FmaRndFtzSatF16x2 {
        pub rnd: Rnd,          // .rnd
        pub ftz: bool,         // {.ftz}
        pub sat: bool,         // {.sat}
        pub f16x2: (),         // .f16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct FmaRndFtzReluF16 {
        pub rnd: Rnd,          // .rnd
        pub ftz: bool,         // {.ftz}
        pub relu: (),          // .relu
        pub f16: (),           // .f16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct FmaRndFtzReluF16x2 {
        pub rnd: Rnd,          // .rnd
        pub ftz: bool,         // {.ftz}
        pub relu: (),          // .relu
        pub f16x2: (),         // .f16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct FmaRndReluBf16 {
        pub rnd: Rnd,          // .rnd
        pub relu: bool,        // {.relu}
        pub bf16: (),          // .bf16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct FmaRndReluBf16x2 {
        pub rnd: Rnd,          // .rnd
        pub relu: bool,        // {.relu}
        pub bf16x2: (),        // .bf16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct FmaRndOobReluType {
        pub rnd: Rnd,          // .rnd
        pub oob: (),           // .oob
        pub relu: bool,        // {.relu}
        pub type_: (),         // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
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
        Rz, // .rz
        Rm, // .rm
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Abtype {
        Bf16, // .bf16
        F16,  // .f16
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct FmaRndSatF32Abtype {
        pub rnd: Rnd,          // .rnd
        pub sat: bool,         // {.sat}
        pub f32: (),           // .f32
        pub abtype: Abtype,    // .abtype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::FmaRndF64;
pub use section_0::FmaRndFtzF32x2;
pub use section_0::FmaRndFtzSatF32;
pub use section_0::Rnd as Rnd0;
pub use section_1::FmaRndFtzReluF16;
pub use section_1::FmaRndFtzReluF16x2;
pub use section_1::FmaRndFtzSatF16;
pub use section_1::FmaRndFtzSatF16x2;
pub use section_1::FmaRndOobReluType;
pub use section_1::FmaRndReluBf16;
pub use section_1::FmaRndReluBf16x2;
pub use section_1::Rnd as Rnd1;
pub use section_2::Abtype as Abtype2;
pub use section_2::FmaRndSatF32Abtype;
pub use section_2::Rnd as Rnd2;
