//! Original PTX specification:
//!
//! tex.geom.v4.dtype.ctype  d{|p}, [a, c] {, e} {, f};
//! tex.geom.v4.dtype.ctype  d{|p}, [a, b, c] {, e} {, f};  // explicit sampler
//! tex.geom.v2.f16x2.ctype  d{|p}, [a, c] {, e} {, f};
//! tex.geom.v2.f16x2.ctype  d{|p}, [a, b, c] {, e} {, f};  // explicit sampler
//! // mipmaps
//! tex.base.geom.v4.dtype.ctype   d{|p}, [a, {b,} c] {, e} {, f};
//! tex.level.geom.v4.dtype.ctype  d{|p}, [a, {b,} c], lod {, e} {, f};
//! tex.grad.geom.v4.dtype.ctype   d{|p}, [a, {b,} c], dPdx, dPdy {, e} {, f};
//! tex.base.geom.v2.f16x2.ctype   d{|p}, [a, {b,} c] {, e} {, f};
//! tex.level.geom.v2.f16x2.ctype  d{|p}, [a, {b,} c], lod {, e} {, f};
//! tex.grad.geom.v2.f16x2.ctype   d{|p}, [a, {b,} c], dPdx, dPdy {, e} {, f};
//! .geom  = { .1d, .2d, .3d, .a1d, .a2d, .cube, .acube, .2dms, .a2dms };
//! .dtype = { .u32, .s32, .f16,  .f32 };
//! .ctype = {       .s32, .f32 };          // .cube, .acube require .f32
//! // .2dms, .a2dms require .s32

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Geom {
        Acube, // .acube
        A2dms, // .a2dms
        Cube, // .cube
        _2dms, // .2dms
        A1d, // .a1d
        A2d, // .a2d
        _1d, // .1d
        _2d, // .2d
        _3d, // .3d
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Dtype {
        U32, // .u32
        S32, // .s32
        F16, // .f16
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Ctype {
        S32, // .s32
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct TexGeomV4DtypeCtype {
        pub geom: Geom, // .geom
        pub v4: (), // .v4
        pub dtype: Dtype, // .dtype
        pub ctype: Ctype, // .ctype
        pub d: GeneralOperand, // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: TexHandler2, // [a, c]
        pub e: Option<GeneralOperand>, // {, e}
        pub f: Option<GeneralOperand>, // {, f}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct TexGeomV4DtypeCtype1 {
        pub geom: Geom, // .geom
        pub v4: (), // .v4
        pub dtype: Dtype, // .dtype
        pub ctype: Ctype, // .ctype
        pub d: GeneralOperand, // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: TexHandler3, // [a, b, c]
        pub e: Option<GeneralOperand>, // {, e}
        pub f: Option<GeneralOperand>, // {, f}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct TexGeomV2F16x2Ctype {
        pub geom: Geom, // .geom
        pub v2: (), // .v2
        pub f16x2: (), // .f16x2
        pub ctype: Ctype, // .ctype
        pub d: GeneralOperand, // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: TexHandler2, // [a, c]
        pub e: Option<GeneralOperand>, // {, e}
        pub f: Option<GeneralOperand>, // {, f}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct TexGeomV2F16x2Ctype1 {
        pub geom: Geom, // .geom
        pub v2: (), // .v2
        pub f16x2: (), // .f16x2
        pub ctype: Ctype, // .ctype
        pub d: GeneralOperand, // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: TexHandler3, // [a, b, c]
        pub e: Option<GeneralOperand>, // {, e}
        pub f: Option<GeneralOperand>, // {, f}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct TexBaseGeomV4DtypeCtype {
        pub base: (), // .base
        pub geom: Geom, // .geom
        pub v4: (), // .v4
        pub dtype: Dtype, // .dtype
        pub ctype: Ctype, // .ctype
        pub d: GeneralOperand, // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: TexHandler3Optional, // [a, {b,}, c]
        pub e: Option<GeneralOperand>, // {, e}
        pub f: Option<GeneralOperand>, // {, f}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct TexLevelGeomV4DtypeCtype {
        pub level: (), // .level
        pub geom: Geom, // .geom
        pub v4: (), // .v4
        pub dtype: Dtype, // .dtype
        pub ctype: Ctype, // .ctype
        pub d: GeneralOperand, // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: TexHandler3Optional, // [a, {b,}, c]
        pub lod: GeneralOperand, // lod
        pub e: Option<GeneralOperand>, // {, e}
        pub f: Option<GeneralOperand>, // {, f}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct TexGradGeomV4DtypeCtype {
        pub grad: (), // .grad
        pub geom: Geom, // .geom
        pub v4: (), // .v4
        pub dtype: Dtype, // .dtype
        pub ctype: Ctype, // .ctype
        pub d: GeneralOperand, // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: TexHandler3Optional, // [a, {b,}, c]
        pub dpdx: GeneralOperand, // dPdx
        pub dpdy: GeneralOperand, // dPdy
        pub e: Option<GeneralOperand>, // {, e}
        pub f: Option<GeneralOperand>, // {, f}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct TexBaseGeomV2F16x2Ctype {
        pub base: (), // .base
        pub geom: Geom, // .geom
        pub v2: (), // .v2
        pub f16x2: (), // .f16x2
        pub ctype: Ctype, // .ctype
        pub d: GeneralOperand, // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: TexHandler3Optional, // [a, {b,}, c]
        pub e: Option<GeneralOperand>, // {, e}
        pub f: Option<GeneralOperand>, // {, f}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct TexLevelGeomV2F16x2Ctype {
        pub level: (), // .level
        pub geom: Geom, // .geom
        pub v2: (), // .v2
        pub f16x2: (), // .f16x2
        pub ctype: Ctype, // .ctype
        pub d: GeneralOperand, // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: TexHandler3Optional, // [a, {b,}, c]
        pub lod: GeneralOperand, // lod
        pub e: Option<GeneralOperand>, // {, e}
        pub f: Option<GeneralOperand>, // {, f}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct TexGradGeomV2F16x2Ctype {
        pub grad: (), // .grad
        pub geom: Geom, // .geom
        pub v2: (), // .v2
        pub f16x2: (), // .f16x2
        pub ctype: Ctype, // .ctype
        pub d: GeneralOperand, // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: TexHandler3Optional, // [a, {b,}, c]
        pub dpdx: GeneralOperand, // dPdx
        pub dpdy: GeneralOperand, // dPdy
        pub e: Option<GeneralOperand>, // {, e}
        pub f: Option<GeneralOperand>, // {, f}
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::TexGeomV4DtypeCtype;
pub use section_0::TexGeomV4DtypeCtype1;
pub use section_0::TexGeomV2F16x2Ctype;
pub use section_0::TexGeomV2F16x2Ctype1;
pub use section_0::TexBaseGeomV4DtypeCtype;
pub use section_0::TexLevelGeomV4DtypeCtype;
pub use section_0::TexGradGeomV4DtypeCtype;
pub use section_0::TexBaseGeomV2F16x2Ctype;
pub use section_0::TexLevelGeomV2F16x2Ctype;
pub use section_0::TexGradGeomV2F16x2Ctype;
pub use section_0::Geom as Geom0;
pub use section_0::Dtype as Dtype0;
pub use section_0::Ctype as Ctype0;
