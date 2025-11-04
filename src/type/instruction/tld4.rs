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
        _2d, // .2d
        A2d, // .a2d
        Cube, // .cube
        Acube, // .acube
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tld4Comp2dV4DtypeF32 {
        pub comp: Comp, // .comp
        pub _2d: (), // .2d
        pub v4: (), // .v4
        pub dtype: Dtype, // .dtype
        pub f32: (), // .f32
        pub d: Operand, // d{|p}
        pub a: (Operand, Operand), // [a, c]
        pub e: Option<Operand>, // {, e}
        pub f: Option<Operand>, // {, f}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tld4CompGeomV4DtypeF32 {
        pub comp: Comp, // .comp
        pub geom: Geom, // .geom
        pub v4: (), // .v4
        pub dtype: Dtype, // .dtype
        pub f32: (), // .f32
        pub d: Operand, // d{|p}
        pub a: (Operand, Operand, Operand), // [a, b, c]
        pub e: Option<Operand>, // {, e}
        pub f: Option<Operand>, // {, f}
    }

}
