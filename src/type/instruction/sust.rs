//! Original PTX specification:
//!
//! sust.b.dim{.cop}.vec.ctype{.mode} [a, b], c;  // unformatted
//! sust.p.dim.vec.b32{.mode}       [a, b], c;  // formatted
//! sust.b.adim{.cop}.vec.ctype{.mode}   [a, b], c;  // unformatted
//! .cop   = { .wb, .cg, .cs, .wt };                     // cache operation
//! .vec   = { none, .v2, .v4 };
//! .ctype = { .b8 , .b16, .b32, .b64 };
//! .mode  = { .trap, .clamp, .zero };
//! .dim   = { .1d, .2d, .3d };
//! .adim  = { .a1d, .a2d };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dim {
        _1d, // .1d
        _2d, // .2d
        _3d, // .3d
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Cop {
        Wb, // .wb
        Cg, // .cg
        Cs, // .cs
        Wt, // .wt
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Vec {
        None, // none
        V2,   // .v2
        V4,   // .v4
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ctype {
        B16, // .b16
        B32, // .b32
        B64, // .b64
        B8,  // .b8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Mode {
        Clamp, // .clamp
        Trap,  // .trap
        Zero,  // .zero
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Adim {
        A1d, // .a1d
        A2d, // .a2d
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct SustBDimCopVecCtypeMode {
        pub b: (),              // .b
        pub dim: Dim,           // .dim
        pub cop: Option<Cop>,   // {.cop}
        pub vec: Vec,           // .vec
        pub ctype: Ctype,       // .ctype
        pub mode: Option<Mode>, // {.mode}
        pub a: TexHandler2,     // [a, b]
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct SustPDimVecB32Mode {
        pub p: (),              // .p
        pub dim: Dim,           // .dim
        pub vec: Vec,           // .vec
        pub b32: (),            // .b32
        pub mode: Option<Mode>, // {.mode}
        pub a: TexHandler2,     // [a, b]
        pub c: GeneralOperand,  // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct SustBAdimCopVecCtypeMode {
        pub b: (),              // .b
        pub adim: Adim,         // .adim
        pub cop: Option<Cop>,   // {.cop}
        pub vec: Vec,           // .vec
        pub ctype: Ctype,       // .ctype
        pub mode: Option<Mode>, // {.mode}
        pub a: TexHandler2,     // [a, b]
        pub c: GeneralOperand,  // c
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Adim as Adim0;
pub use section_0::Cop as Cop0;
pub use section_0::Ctype as Ctype0;
pub use section_0::Dim as Dim0;
pub use section_0::Mode as Mode0;
pub use section_0::SustBAdimCopVecCtypeMode;
pub use section_0::SustBDimCopVecCtypeMode;
pub use section_0::SustPDimVecB32Mode;
pub use section_0::Vec as Vec0;
