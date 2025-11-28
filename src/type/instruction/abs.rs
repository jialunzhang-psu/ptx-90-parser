//! Original PTX specification:
//!
//! abs.type  d, a;
//! .type = { .s16, .s32, .s64 };
//! 
//! abs{.ftz}.f32  d, a;
//! abs.f64        d, a;
//! 
//! abs{.ftz}.f16    d, a;
//! abs{.ftz}.f16x2  d, a;
//! abs.bf16         d, a;
//! abs.bf16x2       d, a;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        S16, // .s16
        S32, // .s32
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct AbsType {
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct AbsFtzF32 {
        pub ftz: bool, // {.ftz}
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct AbsF64 {
        pub f64: (), // .f64
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct AbsFtzF16 {
        pub ftz: bool, // {.ftz}
        pub f16: (), // .f16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct AbsFtzF16x2 {
        pub ftz: bool, // {.ftz}
        pub f16x2: (), // .f16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct AbsBf16 {
        pub bf16: (), // .bf16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct AbsBf16x2 {
        pub bf16x2: (), // .bf16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::AbsType;
pub use section_0::AbsFtzF32;
pub use section_0::AbsF64;
pub use section_0::AbsFtzF16;
pub use section_0::AbsFtzF16x2;
pub use section_0::AbsBf16;
pub use section_0::AbsBf16x2;
pub use section_0::Type as Type0;
