//! Original PTX specification:
//!
//! set.CmpOp{.ftz}.dtype.stype         d, a, b;
//! set.CmpOp.BoolOp{.ftz}.dtype.stype  d, a, b, {!}c;
//! .CmpOp  = { eq, ne, lt, le, gt, ge, lo, ls, hi, hs,
//! equ, neu, ltu, leu, gtu, geu, num, nan };
//! .BoolOp = { and, or, xor };
//! .dtype  = { .u32, .s32, .f32 };
//! .stype  = { .b16, .b32, .b64,
//! .u16, .u32, .u64,
//! .s16, .s32, .s64,
//! .f32, .f64 };
//! -------------------------------------------------------------
//! set.CmpOp{.ftz}.f16.stype            d, a, b;
//! set.CmpOp.BoolOp{.ftz}.f16.stype     d, a, b, {!}c;
//! set.CmpOp.bf16.stype                 d, a, b;
//! set.CmpOp.BoolOp.bf16.stype          d, a, b, {!}c;
//! set.CmpOp{.ftz}.dtype.f16            d, a, b;
//! set.CmpOp.BoolOp{.ftz}.dtype.f16     d, a, b, {!}c;
//! .dtype  = { .u16, .s16, .u32, .s32};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! set.CmpOp.dtype.bf16                 d, a, b;
//! set.CmpOp.BoolOp.dtype.bf16          d, a, b, {!}c;
//! .dtype  = { .u16, .s16, .u32, .s32};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! set.CmpOp{.ftz}.dtype.f16x2          d, a, b;
//! set.CmpOp.BoolOp{.ftz}.dtype.f16x2   d, a, b, {!}c;
//! .dtype  = { .f16x2, .u32, .s32};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! set.CmpOp.dtype.bf16x2               d, a, b;
//! set.CmpOp.BoolOp.dtype.bf16x2        d, a, b, {!}c;
//! .dtype  = { .bf16x2, .u32, .s32};
//! .CmpOp  = { eq, ne, lt, le, gt, ge,
//! equ, neu, ltu, leu, gtu, geu, num, nan };
//! .BoolOp = { and, or, xor };
//! .stype  = { .b16, .b32, .b64,
//! .u16, .u32, .u64,
//! .s16, .s32, .s64,
//! .f16, .f32, .f64};

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Cmpop {
        Eq, // eq
        Ne, // ne
        Lt, // lt
        Le, // le
        Gt, // gt
        Ge, // ge
        Lo, // lo
        Ls, // ls
        Hi, // hi
        Hs, // hs
        Equ, // equ
        Neu, // neu
        Ltu, // ltu
        Leu, // leu
        Gtu, // gtu
        Geu, // geu
        Num, // num
        Nan, // nan
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dtype {
        U32, // .u32
        S32, // .s32
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Stype {
        B16, // .b16
        B32, // .b32
        B64, // .b64
        U16, // .u16
        U32, // .u32
        U64, // .u64
        S16, // .s16
        S32, // .s32
        S64, // .s64
        F32, // .f32
        F64, // .f64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Boolop {
        And, // and
        Or, // or
        Xor, // xor
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetCmpopFtzDtypeStype {
        pub cmpop: Cmpop, // .CmpOp
        pub ftz: bool, // {.ftz}
        pub dtype: Dtype, // .dtype
        pub stype: Stype, // .stype
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetCmpopBoolopFtzDtypeStype {
        pub cmpop: Cmpop, // .CmpOp
        pub boolop: Boolop, // .BoolOp
        pub ftz: bool, // {.ftz}
        pub dtype: Dtype, // .dtype
        pub stype: Stype, // .stype
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c_op: bool, // {!} operator
        pub c: Operand, // {!}c
    }

}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Cmpop {
        Eq, // eq
        Ne, // ne
        Lt, // lt
        Le, // le
        Gt, // gt
        Ge, // ge
        Lo, // lo
        Ls, // ls
        Hi, // hi
        Hs, // hs
        Equ, // equ
        Neu, // neu
        Ltu, // ltu
        Leu, // leu
        Gtu, // gtu
        Geu, // geu
        Num, // num
        Nan, // nan
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Stype {
        B16, // .b16
        B32, // .b32
        B64, // .b64
        U16, // .u16
        U32, // .u32
        U64, // .u64
        S16, // .s16
        S32, // .s32
        S64, // .s64
        F32, // .f32
        F64, // .f64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Boolop {
        And, // and
        Or, // or
        Xor, // xor
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dtype {
        U16, // .u16
        S16, // .s16
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetCmpopFtzF16Stype {
        pub cmpop: Cmpop, // .CmpOp
        pub ftz: bool, // {.ftz}
        pub f16: (), // .f16
        pub stype: Stype, // .stype
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetCmpopBoolopFtzF16Stype {
        pub cmpop: Cmpop, // .CmpOp
        pub boolop: Boolop, // .BoolOp
        pub ftz: bool, // {.ftz}
        pub f16: (), // .f16
        pub stype: Stype, // .stype
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c_op: bool, // {!} operator
        pub c: Operand, // {!}c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetCmpopBf16Stype {
        pub cmpop: Cmpop, // .CmpOp
        pub bf16: (), // .bf16
        pub stype: Stype, // .stype
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetCmpopBoolopBf16Stype {
        pub cmpop: Cmpop, // .CmpOp
        pub boolop: Boolop, // .BoolOp
        pub bf16: (), // .bf16
        pub stype: Stype, // .stype
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c_op: bool, // {!} operator
        pub c: Operand, // {!}c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetCmpopFtzDtypeF16 {
        pub cmpop: Cmpop, // .CmpOp
        pub ftz: bool, // {.ftz}
        pub dtype: Dtype, // .dtype
        pub f16: (), // .f16
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetCmpopBoolopFtzDtypeF16 {
        pub cmpop: Cmpop, // .CmpOp
        pub boolop: Boolop, // .BoolOp
        pub ftz: bool, // {.ftz}
        pub dtype: Dtype, // .dtype
        pub f16: (), // .f16
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c_op: bool, // {!} operator
        pub c: Operand, // {!}c
    }

}

pub mod section_2 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Cmpop {
        Eq, // eq
        Ne, // ne
        Lt, // lt
        Le, // le
        Gt, // gt
        Ge, // ge
        Lo, // lo
        Ls, // ls
        Hi, // hi
        Hs, // hs
        Equ, // equ
        Neu, // neu
        Ltu, // ltu
        Leu, // leu
        Gtu, // gtu
        Geu, // geu
        Num, // num
        Nan, // nan
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dtype {
        U16, // .u16
        S16, // .s16
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Boolop {
        And, // and
        Or, // or
        Xor, // xor
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetCmpopDtypeBf16 {
        pub cmpop: Cmpop, // .CmpOp
        pub dtype: Dtype, // .dtype
        pub bf16: (), // .bf16
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetCmpopBoolopDtypeBf16 {
        pub cmpop: Cmpop, // .CmpOp
        pub boolop: Boolop, // .BoolOp
        pub dtype: Dtype, // .dtype
        pub bf16: (), // .bf16
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c_op: bool, // {!} operator
        pub c: Operand, // {!}c
    }

}

pub mod section_3 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Cmpop {
        Eq, // eq
        Ne, // ne
        Lt, // lt
        Le, // le
        Gt, // gt
        Ge, // ge
        Lo, // lo
        Ls, // ls
        Hi, // hi
        Hs, // hs
        Equ, // equ
        Neu, // neu
        Ltu, // ltu
        Leu, // leu
        Gtu, // gtu
        Geu, // geu
        Num, // num
        Nan, // nan
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dtype {
        F16x2, // .f16x2
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Boolop {
        And, // and
        Or, // or
        Xor, // xor
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetCmpopFtzDtypeF16x2 {
        pub cmpop: Cmpop, // .CmpOp
        pub ftz: bool, // {.ftz}
        pub dtype: Dtype, // .dtype
        pub f16x2: (), // .f16x2
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetCmpopBoolopFtzDtypeF16x2 {
        pub cmpop: Cmpop, // .CmpOp
        pub boolop: Boolop, // .BoolOp
        pub ftz: bool, // {.ftz}
        pub dtype: Dtype, // .dtype
        pub f16x2: (), // .f16x2
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c_op: bool, // {!} operator
        pub c: Operand, // {!}c
    }

}

pub mod section_4 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Cmpop {
        Eq, // eq
        Ne, // ne
        Lt, // lt
        Le, // le
        Gt, // gt
        Ge, // ge
        Equ, // equ
        Neu, // neu
        Ltu, // ltu
        Leu, // leu
        Gtu, // gtu
        Geu, // geu
        Num, // num
        Nan, // nan
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dtype {
        Bf16x2, // .bf16x2
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Boolop {
        And, // and
        Or, // or
        Xor, // xor
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetCmpopDtypeBf16x2 {
        pub cmpop: Cmpop, // .CmpOp
        pub dtype: Dtype, // .dtype
        pub bf16x2: (), // .bf16x2
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetCmpopBoolopDtypeBf16x2 {
        pub cmpop: Cmpop, // .CmpOp
        pub boolop: Boolop, // .BoolOp
        pub dtype: Dtype, // .dtype
        pub bf16x2: (), // .bf16x2
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c_op: bool, // {!} operator
        pub c: Operand, // {!}c
    }

}
