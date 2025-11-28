//! Original PTX specification:
//!
//! sured.b.op.geom.ctype.mode [a,b],c; // byte addressing
//! .op    = { .add, .min, .max, .and, .or };
//! .geom  = { .1d, .2d, .3d };
//! .ctype = { .u32, .u64, .s32, .b32, .s64 };  // for sured.b
//! .mode  = { .trap, .clamp, .zero };
//! ----------------------------------------------------
//! sured.p.op.geom.ctype.mode [a,b],c; // sample addressing
//! .op    = { .add, .min, .max, .and, .or };
//! .geom  = { .1d, .2d, .3d };
//! .ctype = { .b32, .b64 };                    // for sured.p
//! .mode  = { .trap, .clamp, .zero };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Op {
        Add, // .add
        Min, // .min
        Max, // .max
        And, // .and
        Or, // .or
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Geom {
        _1d, // .1d
        _2d, // .2d
        _3d, // .3d
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Ctype {
        U32, // .u32
        U64, // .u64
        S32, // .s32
        B32, // .b32
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Mode {
        Clamp, // .clamp
        Trap, // .trap
        Zero, // .zero
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SuredBOpGeomCtypeMode {
        pub b: (), // .b
        pub op: Op, // .op
        pub geom: Geom, // .geom
        pub ctype: Ctype, // .ctype
        pub mode: Mode, // .mode
        pub a: TexHandler2, // [a, b]
        pub c: GeneralOperand, // c
        pub span: Span,
    }

}

pub mod section_1 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Op {
        Add, // .add
        Min, // .min
        Max, // .max
        And, // .and
        Or, // .or
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Geom {
        _1d, // .1d
        _2d, // .2d
        _3d, // .3d
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Ctype {
        B32, // .b32
        B64, // .b64
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Mode {
        Clamp, // .clamp
        Trap, // .trap
        Zero, // .zero
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SuredPOpGeomCtypeMode {
        pub p: (), // .p
        pub op: Op, // .op
        pub geom: Geom, // .geom
        pub ctype: Ctype, // .ctype
        pub mode: Mode, // .mode
        pub a: TexHandler2, // [a, b]
        pub c: GeneralOperand, // c
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::SuredBOpGeomCtypeMode;
pub use section_0::Op as Op0;
pub use section_0::Geom as Geom0;
pub use section_0::Ctype as Ctype0;
pub use section_0::Mode as Mode0;
pub use section_1::SuredPOpGeomCtypeMode;
pub use section_1::Op as Op1;
pub use section_1::Geom as Geom1;
pub use section_1::Ctype as Ctype1;
pub use section_1::Mode as Mode1;
