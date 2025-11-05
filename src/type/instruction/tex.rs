//! Original PTX specification:
//!
//! tex.geom.v4.dtype.ctype  d, [a, c] {, e} {, f};
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

    #[derive(Debug, Clone, PartialEq)]
    pub enum Geom {
        _1d, // .1d
        _2d, // .2d
        _3d, // .3d
        A1d, // .a1d
        A2d, // .a2d
        Cube, // .cube
        Acube, // .acube
        _2dms, // .2dms
        A2dms, // .a2dms
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dtype {
        U32, // .u32
        S32, // .s32
        F16, // .f16
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ctype {
        S32, // .s32
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TexGeomV4DtypeCtype {
        pub geom: Geom, // .geom
        pub v4: (), // .v4
        pub dtype: Dtype, // .dtype
        pub ctype: Ctype, // .ctype
        pub d: Operand, // d
        pub a: (Operand, Operand), // [a, c]
        pub e: Option<Operand>, // {, e}
        pub f: Option<Operand>, // {, f}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TexGeomV4DtypeCtype1 {
        pub geom: Geom, // .geom
        pub v4: (), // .v4
        pub dtype: Dtype, // .dtype
        pub ctype: Ctype, // .ctype
        pub d: Operand, // first operand of d{|p}
        pub p: Option<Operand>, // optional second operand of d{|p}
        pub a: (Operand, Operand, Operand), // [a, b, c]
        pub e: Option<Operand>, // {, e}
        pub f: Option<Operand>, // {, f}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TexGeomV2F16x2Ctype {
        pub geom: Geom, // .geom
        pub v2: (), // .v2
        pub f16x2: (), // .f16x2
        pub ctype: Ctype, // .ctype
        pub d: Operand, // first operand of d{|p}
        pub p: Option<Operand>, // optional second operand of d{|p}
        pub a: (Operand, Operand), // [a, c]
        pub e: Option<Operand>, // {, e}
        pub f: Option<Operand>, // {, f}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TexGeomV2F16x2Ctype1 {
        pub geom: Geom, // .geom
        pub v2: (), // .v2
        pub f16x2: (), // .f16x2
        pub ctype: Ctype, // .ctype
        pub d: Operand, // first operand of d{|p}
        pub p: Option<Operand>, // optional second operand of d{|p}
        pub a: (Operand, Operand, Operand), // [a, b, c]
        pub e: Option<Operand>, // {, e}
        pub f: Option<Operand>, // {, f}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TexBaseGeomV4DtypeCtype {
        pub base: (), // .base
        pub geom: Geom, // .geom
        pub v4: (), // .v4
        pub dtype: Dtype, // .dtype
        pub ctype: Ctype, // .ctype
        pub d: Operand, // first operand of d{|p}
        pub p: Option<Operand>, // optional second operand of d{|p}
        pub a: (Operand, Operand, Operand), // [a, b, c]
        pub e: Option<Operand>, // {, e}
        pub f: Option<Operand>, // {, f}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TexLevelGeomV4DtypeCtype {
        pub level: (), // .level
        pub geom: Geom, // .geom
        pub v4: (), // .v4
        pub dtype: Dtype, // .dtype
        pub ctype: Ctype, // .ctype
        pub d: Operand, // first operand of d{|p}
        pub p: Option<Operand>, // optional second operand of d{|p}
        pub a: (Operand, Operand, Operand), // [a, b, c]
        pub lod: Operand, // lod
        pub e: Option<Operand>, // {, e}
        pub f: Option<Operand>, // {, f}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TexGradGeomV4DtypeCtype {
        pub grad: (), // .grad
        pub geom: Geom, // .geom
        pub v4: (), // .v4
        pub dtype: Dtype, // .dtype
        pub ctype: Ctype, // .ctype
        pub d: Operand, // first operand of d{|p}
        pub p: Option<Operand>, // optional second operand of d{|p}
        pub a: (Operand, Operand, Operand), // [a, b, c]
        pub dpdx: Operand, // dPdx
        pub dpdy: Operand, // dPdy
        pub e: Option<Operand>, // {, e}
        pub f: Option<Operand>, // {, f}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TexBaseGeomV2F16x2Ctype {
        pub base: (), // .base
        pub geom: Geom, // .geom
        pub v2: (), // .v2
        pub f16x2: (), // .f16x2
        pub ctype: Ctype, // .ctype
        pub d: Operand, // first operand of d{|p}
        pub p: Option<Operand>, // optional second operand of d{|p}
        pub a: (Operand, Operand, Operand), // [a, b, c]
        pub e: Option<Operand>, // {, e}
        pub f: Option<Operand>, // {, f}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TexLevelGeomV2F16x2Ctype {
        pub level: (), // .level
        pub geom: Geom, // .geom
        pub v2: (), // .v2
        pub f16x2: (), // .f16x2
        pub ctype: Ctype, // .ctype
        pub d: Operand, // first operand of d{|p}
        pub p: Option<Operand>, // optional second operand of d{|p}
        pub a: (Operand, Operand, Operand), // [a, b, c]
        pub lod: Operand, // lod
        pub e: Option<Operand>, // {, e}
        pub f: Option<Operand>, // {, f}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TexGradGeomV2F16x2Ctype {
        pub grad: (), // .grad
        pub geom: Geom, // .geom
        pub v2: (), // .v2
        pub f16x2: (), // .f16x2
        pub ctype: Ctype, // .ctype
        pub d: Operand, // first operand of d{|p}
        pub p: Option<Operand>, // optional second operand of d{|p}
        pub a: (Operand, Operand, Operand), // [a, b, c]
        pub dpdx: Operand, // dPdx
        pub dpdy: Operand, // dPdy
        pub e: Option<Operand>, // {, e}
        pub f: Option<Operand>, // {, f}
    }

}
