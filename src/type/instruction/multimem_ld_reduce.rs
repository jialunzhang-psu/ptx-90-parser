//! Original PTX specification:
//!
//! // Integer type:
//! multimem.ld_reduce{.ldsem}{.scope}{.ss}.op.type      d, [a];
//! multimem.ld_reduce.weak{.ss}.op.type                 d, [a];
//! multimem.st{.stsem}{.scope}{.ss}.type                [a], b;
//! multimem.st.weak{.ss}.type                           [a], b;
//! multimem.red{.redsem}{.scope}{.ss}.op.type           [a], b;
//! .ss =       { .global };
//! .ldsem =    { .relaxed, .acquire };
//! .stsem =    { .relaxed, .release };
//! .redsem =   { .relaxed, .release };
//! .scope =    { .cta, .cluster, .gpu, .sys };
//! .op  =      { .min, .max, .add, .and, .or, .xor };
//! .type =     { .b32, .b64,  .u32, .u64, .s32, .s64 };
//! ------------------------------------------------------------------
//! // Floating point type:
//! multimem.ld_reduce{.ldsem}{.scope}{.ss}.op{.acc_prec}{.vec}.type    d, [a];
//! multimem.ld_reduce.weak{.ss}.op{.acc_prec}{.vec}.type               d, [a];
//! multimem.st{.stsem}{.scope}{.ss}{.vec}.type                         [a], b;
//! multimem.st.weak{.ss}{.vec}.type                                    [a], b;
//! multimem.red{.redsem}{.scope}{.ss}.redop{.vec}.redtype              [a], b;
//! .ss =       { .global };
//! .ldsem =    { .relaxed, .acquire };
//! .stsem =    { .relaxed, .release };
//! .redsem =   { .relaxed, .release };
//! .scope =    { .cta, .cluster, .gpu, .sys };
//! .op  =      { .min, .max, .add };
//! .redop  =   { .add };
//! .acc_prec = { .acc::f32, .acc::f16 };
//! .vec =      { .v2, .v4, .v8 };
//! .type=      { .f16, .f16x2, .bf16, .bf16x2, .f32, .f64, .e5m2, .e5m2x2, .e5m2x4, .e4m3, .e4m3x2, .e4m3x4 };
//! .redtype =  { .f16, .f16x2, .bf16, .bf16x2, .f32, .f64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Ldsem {
        Relaxed, // .relaxed
        Acquire, // .acquire
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Scope {
        Cluster, // .cluster
        Cta,     // .cta
        Gpu,     // .gpu
        Sys,     // .sys
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Ss {
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Op {
        Min, // .min
        Max, // .max
        Add, // .add
        And, // .and
        Xor, // .xor
        Or,  // .or
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        B32, // .b32
        B64, // .b64
        U32, // .u32
        U64, // .u64
        S32, // .s32
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Stsem {
        Relaxed, // .relaxed
        Release, // .release
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Redsem {
        Relaxed, // .relaxed
        Release, // .release
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MultimemLdReduceLdsemScopeSsOpType {
        pub ld_reduce: (),        // .ld_reduce
        pub ldsem: Option<Ldsem>, // {.ldsem}
        pub scope: Option<Scope>, // {.scope}
        pub ss: Option<Ss>,       // {.ss}
        pub op: Op,               // .op
        pub type_: Type,          // .type
        pub d: GeneralOperand,    // d
        pub a: AddressOperand,    // [a]
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MultimemLdReduceWeakSsOpType {
        pub ld_reduce: (),     // .ld_reduce
        pub weak: (),          // .weak
        pub ss: Option<Ss>,    // {.ss}
        pub op: Op,            // .op
        pub type_: Type,       // .type
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MultimemStStsemScopeSsType {
        pub st: (),               // .st
        pub stsem: Option<Stsem>, // {.stsem}
        pub scope: Option<Scope>, // {.scope}
        pub ss: Option<Ss>,       // {.ss}
        pub type_: Type,          // .type
        pub a: AddressOperand,    // [a]
        pub b: GeneralOperand,    // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MultimemStWeakSsType {
        pub st: (),            // .st
        pub weak: (),          // .weak
        pub ss: Option<Ss>,    // {.ss}
        pub type_: Type,       // .type
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MultimemRedRedsemScopeSsOpType {
        pub red: (),                // .red
        pub redsem: Option<Redsem>, // {.redsem}
        pub scope: Option<Scope>,   // {.scope}
        pub ss: Option<Ss>,         // {.ss}
        pub op: Op,                 // .op
        pub type_: Type,            // .type
        pub a: AddressOperand,      // [a]
        pub b: GeneralOperand,      // b
        pub span: Span,
    }
}

pub mod section_1 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Ldsem {
        Relaxed, // .relaxed
        Acquire, // .acquire
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Scope {
        Cluster, // .cluster
        Cta,     // .cta
        Gpu,     // .gpu
        Sys,     // .sys
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Ss {
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Op {
        Min, // .min
        Max, // .max
        Add, // .add
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum AccPrec {
        AccF32, // .acc::f32
        AccF16, // .acc::f16
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Vec {
        V2, // .v2
        V4, // .v4
        V8, // .v8
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        Bf16x2, // .bf16x2
        E5m2x2, // .e5m2x2
        E5m2x4, // .e5m2x4
        E4m3x2, // .e4m3x2
        E4m3x4, // .e4m3x4
        F16x2,  // .f16x2
        Bf16,   // .bf16
        E5m2,   // .e5m2
        E4m3,   // .e4m3
        F16,    // .f16
        F32,    // .f32
        F64,    // .f64
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Stsem {
        Relaxed, // .relaxed
        Release, // .release
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Redsem {
        Relaxed, // .relaxed
        Release, // .release
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Redop {
        Add, // .add
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Redtype {
        Bf16x2, // .bf16x2
        F16x2,  // .f16x2
        Bf16,   // .bf16
        F16,    // .f16
        F32,    // .f32
        F64,    // .f64
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MultimemLdReduceLdsemScopeSsOpAccPrecVecType {
        pub ld_reduce: (),             // .ld_reduce
        pub ldsem: Option<Ldsem>,      // {.ldsem}
        pub scope: Option<Scope>,      // {.scope}
        pub ss: Option<Ss>,            // {.ss}
        pub op: Op,                    // .op
        pub acc_prec: Option<AccPrec>, // {.acc_prec}
        pub vec: Option<Vec>,          // {.vec}
        pub type_: Type,               // .type
        pub d: GeneralOperand,         // d
        pub a: AddressOperand,         // [a]
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MultimemLdReduceWeakSsOpAccPrecVecType {
        pub ld_reduce: (),             // .ld_reduce
        pub weak: (),                  // .weak
        pub ss: Option<Ss>,            // {.ss}
        pub op: Op,                    // .op
        pub acc_prec: Option<AccPrec>, // {.acc_prec}
        pub vec: Option<Vec>,          // {.vec}
        pub type_: Type,               // .type
        pub d: GeneralOperand,         // d
        pub a: AddressOperand,         // [a]
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MultimemStStsemScopeSsVecType {
        pub st: (),               // .st
        pub stsem: Option<Stsem>, // {.stsem}
        pub scope: Option<Scope>, // {.scope}
        pub ss: Option<Ss>,       // {.ss}
        pub vec: Option<Vec>,     // {.vec}
        pub type_: Type,          // .type
        pub a: AddressOperand,    // [a]
        pub b: GeneralOperand,    // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MultimemStWeakSsVecType {
        pub st: (),            // .st
        pub weak: (),          // .weak
        pub ss: Option<Ss>,    // {.ss}
        pub vec: Option<Vec>,  // {.vec}
        pub type_: Type,       // .type
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MultimemRedRedsemScopeSsRedopVecRedtype {
        pub red: (),                // .red
        pub redsem: Option<Redsem>, // {.redsem}
        pub scope: Option<Scope>,   // {.scope}
        pub ss: Option<Ss>,         // {.ss}
        pub redop: Redop,           // .redop
        pub vec: Option<Vec>,       // {.vec}
        pub redtype: Redtype,       // .redtype
        pub a: AddressOperand,      // [a]
        pub b: GeneralOperand,      // b
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Ldsem as Ldsem0;
pub use section_0::MultimemLdReduceLdsemScopeSsOpType;
pub use section_0::MultimemLdReduceWeakSsOpType;
pub use section_0::MultimemRedRedsemScopeSsOpType;
pub use section_0::MultimemStStsemScopeSsType;
pub use section_0::MultimemStWeakSsType;
pub use section_0::Op as Op0;
pub use section_0::Redsem as Redsem0;
pub use section_0::Scope as Scope0;
pub use section_0::Ss as Ss0;
pub use section_0::Stsem as Stsem0;
pub use section_0::Type as Type0;
pub use section_1::AccPrec as AccPrec1;
pub use section_1::Ldsem as Ldsem1;
pub use section_1::MultimemLdReduceLdsemScopeSsOpAccPrecVecType;
pub use section_1::MultimemLdReduceWeakSsOpAccPrecVecType;
pub use section_1::MultimemRedRedsemScopeSsRedopVecRedtype;
pub use section_1::MultimemStStsemScopeSsVecType;
pub use section_1::MultimemStWeakSsVecType;
pub use section_1::Op as Op1;
pub use section_1::Redop as Redop1;
pub use section_1::Redsem as Redsem1;
pub use section_1::Redtype as Redtype1;
pub use section_1::Scope as Scope1;
pub use section_1::Ss as Ss1;
pub use section_1::Stsem as Stsem1;
pub use section_1::Type as Type1;
pub use section_1::Vec as Vec1;
