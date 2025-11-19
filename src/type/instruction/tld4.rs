//! Original PTX specification:
//!
//! tld4.comp.2d.v4.dtype.f32    d{|p}, [a, c] {, e} {, f};
//! tld4.comp.geom.v4.dtype.f32  d{|p}, [a, b, c] {, e} {, f};  // explicit sampler
//! .comp  = { .r, .g, .b, .a };
//! .geom  = { .2d, .a2d, .cube, .acube };
//! .dtype = { .u32, .s32, .f32 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Comp {
        R, // .r
        G, // .g
        B, // .b
        A, // .a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dtype {
        U32, // .u32
        S32, // .s32
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Geom {
        Acube, // .acube
        Cube,  // .cube
        A2d,   // .a2d
        _2d,   // .2d
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tld4Comp2dV4DtypeF32 {
        pub comp: Comp,                // .comp
        pub _2d: (),                   // .2d
        pub v4: (),                    // .v4
        pub dtype: Dtype,              // .dtype
        pub f32: (),                   // .f32
        pub d: GeneralOperand,         // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: TexHandler2,            // [a, c]
        pub e: Option<GeneralOperand>, // {, e}
        pub f: Option<GeneralOperand>, // {, f}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tld4CompGeomV4DtypeF32 {
        pub comp: Comp,                // .comp
        pub geom: Geom,                // .geom
        pub v4: (),                    // .v4
        pub dtype: Dtype,              // .dtype
        pub f32: (),                   // .f32
        pub d: GeneralOperand,         // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: TexHandler3,            // [a, b, c]
        pub e: Option<GeneralOperand>, // {, e}
        pub f: Option<GeneralOperand>, // {, f}
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Comp as Comp0;
pub use section_0::Dtype as Dtype0;
pub use section_0::Geom as Geom0;
pub use section_0::Tld4Comp2dV4DtypeF32;
pub use section_0::Tld4CompGeomV4DtypeF32;
