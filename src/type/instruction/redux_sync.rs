//! Original PTX specification:
//!
//! redux.sync.op.type dst, src, membermask;
//! .op   = {.add, .min, .max};
//! .type = {.u32, .s32};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! redux.sync.op.b32 dst, src, membermask;
//! .op   = {.and, .or, .xor};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! redux.sync.op{.abs}{.NaN}.f32 dst, src, membermask;
//! .op   = { .min, .max };

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
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct ReduxSyncOpType {
        pub sync: (), // .sync
        pub op: Op, // .op
        pub type_: Type, // .type
        pub dst: GeneralOperand, // dst
        pub src: GeneralOperand, // src
        pub membermask: GeneralOperand, // membermask
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
        And, // .and
        Xor, // .xor
        Or, // .or
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct ReduxSyncOpB32 {
        pub sync: (), // .sync
        pub op: Op, // .op
        pub b32: (), // .b32
        pub dst: GeneralOperand, // dst
        pub src: GeneralOperand, // src
        pub membermask: GeneralOperand, // membermask
        pub span: Span,
    }

}

pub mod section_2 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Op {
        Min, // .min
        Max, // .max
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct ReduxSyncOpAbsNanF32 {
        pub sync: (), // .sync
        pub op: Op, // .op
        pub abs: bool, // {.abs}
        pub nan: bool, // {.NaN}
        pub f32: (), // .f32
        pub dst: GeneralOperand, // dst
        pub src: GeneralOperand, // src
        pub membermask: GeneralOperand, // membermask
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::ReduxSyncOpType;
pub use section_0::Op as Op0;
pub use section_0::Type as Type0;
pub use section_1::ReduxSyncOpB32;
pub use section_1::Op as Op1;
pub use section_2::ReduxSyncOpAbsNanF32;
pub use section_2::Op as Op2;
