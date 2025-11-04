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
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ldsem {
        Relaxed, // .relaxed
        Acquire, // .acquire
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scope {
        Cta, // .cta
        Cluster, // .cluster
        Gpu, // .gpu
        Sys, // .sys
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        Min, // .min
        Max, // .max
        Add, // .add
        And, // .and
        Or, // .or
        Xor, // .xor
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B32, // .b32
        B64, // .b64
        U32, // .u32
        U64, // .u64
        S32, // .s32
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Stsem {
        Relaxed, // .relaxed
        Release, // .release
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Redsem {
        Relaxed, // .relaxed
        Release, // .release
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MultimemLdReduceLdsemScopeSsOpType {
        pub ld_reduce: (), // .ld_reduce
        pub ldsem: Option<Ldsem>, // {.ldsem}
        pub scope: Option<Scope>, // {.scope}
        pub ss: Option<Ss>, // {.ss}
        pub op: Op, // .op
        pub type_: Type, // .type
        pub d: Operand, // d
        pub a: AddressOperand, // [a]
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MultimemLdReduceWeakSsOpType {
        pub ld_reduce: (), // .ld_reduce
        pub weak: (), // .weak
        pub ss: Option<Ss>, // {.ss}
        pub op: Op, // .op
        pub type_: Type, // .type
        pub d: Operand, // d
        pub a: AddressOperand, // [a]
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MultimemStStsemScopeSsType {
        pub st: (), // .st
        pub stsem: Option<Stsem>, // {.stsem}
        pub scope: Option<Scope>, // {.scope}
        pub ss: Option<Ss>, // {.ss}
        pub type_: Type, // .type
        pub a: AddressOperand, // [a]
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MultimemStWeakSsType {
        pub st: (), // .st
        pub weak: (), // .weak
        pub ss: Option<Ss>, // {.ss}
        pub type_: Type, // .type
        pub a: AddressOperand, // [a]
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MultimemRedRedsemScopeSsOpType {
        pub red: (), // .red
        pub redsem: Option<Redsem>, // {.redsem}
        pub scope: Option<Scope>, // {.scope}
        pub ss: Option<Ss>, // {.ss}
        pub op: Op, // .op
        pub type_: Type, // .type
        pub a: AddressOperand, // [a]
        pub b: Operand, // b
    }

}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ldsem {
        Relaxed, // .relaxed
        Acquire, // .acquire
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scope {
        Cta, // .cta
        Cluster, // .cluster
        Gpu, // .gpu
        Sys, // .sys
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        Min, // .min
        Max, // .max
        Add, // .add
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum AccPrec {
        AccF32, // .acc::f32
        AccF16, // .acc::f16
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Vec {
        V2, // .v2
        V4, // .v4
        V8, // .v8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        F16, // .f16
        F16x2, // .f16x2
        Bf16, // .bf16
        Bf16x2, // .bf16x2
        F32, // .f32
        F64, // .f64
        E5m2, // .e5m2
        E5m2x2, // .e5m2x2
        E5m2x4, // .e5m2x4
        E4m3, // .e4m3
        E4m3x2, // .e4m3x2
        E4m3x4, // .e4m3x4
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Stsem {
        Relaxed, // .relaxed
        Release, // .release
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Redsem {
        Relaxed, // .relaxed
        Release, // .release
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Redop {
        Add, // .add
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Redtype {
        F16, // .f16
        F16x2, // .f16x2
        Bf16, // .bf16
        Bf16x2, // .bf16x2
        F32, // .f32
        F64, // .f64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MultimemLdReduceLdsemScopeSsOpAccPrecVecType {
        pub ld_reduce: (), // .ld_reduce
        pub ldsem: Option<Ldsem>, // {.ldsem}
        pub scope: Option<Scope>, // {.scope}
        pub ss: Option<Ss>, // {.ss}
        pub op: Op, // .op
        pub acc_prec: Option<AccPrec>, // {.acc_prec}
        pub vec: Option<Vec>, // {.vec}
        pub type_: Type, // .type
        pub d: Operand, // d
        pub a: AddressOperand, // [a]
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MultimemLdReduceWeakSsOpAccPrecVecType {
        pub ld_reduce: (), // .ld_reduce
        pub weak: (), // .weak
        pub ss: Option<Ss>, // {.ss}
        pub op: Op, // .op
        pub acc_prec: Option<AccPrec>, // {.acc_prec}
        pub vec: Option<Vec>, // {.vec}
        pub type_: Type, // .type
        pub d: Operand, // d
        pub a: AddressOperand, // [a]
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MultimemStStsemScopeSsVecType {
        pub st: (), // .st
        pub stsem: Option<Stsem>, // {.stsem}
        pub scope: Option<Scope>, // {.scope}
        pub ss: Option<Ss>, // {.ss}
        pub vec: Option<Vec>, // {.vec}
        pub type_: Type, // .type
        pub a: AddressOperand, // [a]
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MultimemStWeakSsVecType {
        pub st: (), // .st
        pub weak: (), // .weak
        pub ss: Option<Ss>, // {.ss}
        pub vec: Option<Vec>, // {.vec}
        pub type_: Type, // .type
        pub a: AddressOperand, // [a]
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MultimemRedRedsemScopeSsRedopVecRedtype {
        pub red: (), // .red
        pub redsem: Option<Redsem>, // {.redsem}
        pub scope: Option<Scope>, // {.scope}
        pub ss: Option<Ss>, // {.ss}
        pub redop: Redop, // .redop
        pub vec: Option<Vec>, // {.vec}
        pub redtype: Redtype, // .redtype
        pub a: AddressOperand, // [a]
        pub b: Operand, // b
    }

}
