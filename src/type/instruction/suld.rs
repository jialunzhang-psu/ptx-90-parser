//! Original PTX specification:
//!
//! suld.b.geom{.cop}.vec.dtype{.mode}  d, [a, b];  // unformatted
//! 
//! .geom  = { .1d, .2d, .3d, .a1d, .a2d };
//! .cop   = { .ca, .cg, .cs, .cv };               // cache operation
//! .vec   = { none, .v2, .v4 };
//! .dtype = { .b8 , .b16, .b32, .b64 };
//! .mode = { .trap, .clamp, .zero };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Geom {
        A1d, // .a1d
        A2d, // .a2d
        _1d, // .1d
        _2d, // .2d
        _3d, // .3d
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Cop {
        Ca, // .ca
        Cg, // .cg
        Cs, // .cs
        Cv, // .cv
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Vec {
        None, // none
        V2, // .v2
        V4, // .v4
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Dtype {
        B16, // .b16
        B32, // .b32
        B64, // .b64
        B8, // .b8
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Mode {
        Clamp, // .clamp
        Trap, // .trap
        Zero, // .zero
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SuldBGeomCopVecDtypeMode {
        pub b: (), // .b
        pub geom: Geom, // .geom
        pub cop: Option<Cop>, // {.cop}
        pub vec: Vec, // .vec
        pub dtype: Dtype, // .dtype
        pub mode: Option<Mode>, // {.mode}
        pub d: GeneralOperand, // d
        pub a: TexHandler2, // [a, b]
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::SuldBGeomCopVecDtypeMode;
pub use section_0::Geom as Geom0;
pub use section_0::Cop as Cop0;
pub use section_0::Vec as Vec0;
pub use section_0::Dtype as Dtype0;
pub use section_0::Mode as Mode0;
