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
        Cube, // .cube
        A2d, // .a2d
        _2d, // .2d
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tld4Comp2dV4DtypeF32 {
        pub comp: Comp, // .comp
        pub _2d: (), // .2d
        pub v4: (), // .v4
        pub dtype: Dtype, // .dtype
        pub f32: (), // .f32
        pub d: GeneralOperand, // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: TexHandler2, // [a, c]
        pub e: Option<GeneralOperand>, // {, e}
        pub f: Option<GeneralOperand>, // {, f}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tld4CompGeomV4DtypeF32 {
        pub comp: Comp, // .comp
        pub geom: Geom, // .geom
        pub v4: (), // .v4
        pub dtype: Dtype, // .dtype
        pub f32: (), // .f32
        pub d: GeneralOperand, // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: TexHandler3, // [a, b, c]
        pub e: Option<GeneralOperand>, // {, e}
        pub f: Option<GeneralOperand>, // {, f}
    }

}
