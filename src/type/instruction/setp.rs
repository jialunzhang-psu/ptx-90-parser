//! Original PTX specification:
//!
//! setp.CmpOp{.ftz}.type         p{|q}, a, b;
//! setp.CmpOp.BoolOp{.ftz}.type  p{|q}, a, b, {!}c;
//! .CmpOp  = { .eq, .ne, .lt, .le, .gt, .ge, .lo, .ls, .hi, .hs, .equ, .neu, .ltu, .leu, .gtu, .geu, .num, .nan };
//! .BoolOp = { .and, .or, .xor };
//! .type   = { .b16, .b32, .b64, .u16, .u32, .u64, .s16, .s32, .s64, .f32, .f64 };
//! --------------------------------------------------------------
//! setp.CmpOp{.ftz}.f16           p, a, b;
//! setp.CmpOp.BoolOp{.ftz}.f16    p, a, b, {!}c;
//! setp.CmpOp{.ftz}.f16x2         p|q, a, b;
//! setp.CmpOp.BoolOp{.ftz}.f16x2  p|q, a, b, {!}c;
//! setp.CmpOp.bf16                p, a, b;
//! setp.CmpOp.BoolOp.bf16         p, a, b, {!}c;
//! setp.CmpOp.bf16x2              p|q, a, b;
//! setp.CmpOp.BoolOp.bf16x2       p|q, a, b, {!}c;
//! .CmpOp  = { .eq, .ne, .lt, .le, .gt, .ge, .equ, .neu, .ltu, .leu, .gtu, .geu, .num, .nan };
//! .BoolOp = { .and, .or, .xor };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Cmpop {
        Eq, // .eq
        Ne, // .ne
        Lt, // .lt
        Le, // .le
        Gt, // .gt
        Ge, // .ge
        Lo, // .lo
        Ls, // .ls
        Hi, // .hi
        Hs, // .hs
        Equ, // .equ
        Neu, // .neu
        Ltu, // .ltu
        Leu, // .leu
        Gtu, // .gtu
        Geu, // .geu
        Num, // .num
        Nan, // .nan
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
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
        And, // .and
        Or, // .or
        Xor, // .xor
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopFtzType {
        pub cmpop: Cmpop, // .CmpOp
        pub ftz: bool, // {.ftz}
        pub type_: Type, // .type
        pub p: Operand, // first operand of p{|q}
        pub q: Option<Operand>, // optional second operand of p{|q}
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopBoolopFtzType {
        pub cmpop: Cmpop, // .CmpOp
        pub boolop: Boolop, // .BoolOp
        pub ftz: bool, // {.ftz}
        pub type_: Type, // .type
        pub p: Operand, // first operand of p{|q}
        pub q: Option<Operand>, // optional second operand of p{|q}
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
        Eq, // .eq
        Ne, // .ne
        Lt, // .lt
        Le, // .le
        Gt, // .gt
        Ge, // .ge
        Equ, // .equ
        Neu, // .neu
        Ltu, // .ltu
        Leu, // .leu
        Gtu, // .gtu
        Geu, // .geu
        Num, // .num
        Nan, // .nan
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Boolop {
        And, // .and
        Or, // .or
        Xor, // .xor
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopFtzF16 {
        pub cmpop: Cmpop, // .CmpOp
        pub ftz: bool, // {.ftz}
        pub f16: (), // .f16
        pub p: Operand, // p
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopBoolopFtzF16 {
        pub cmpop: Cmpop, // .CmpOp
        pub boolop: Boolop, // .BoolOp
        pub ftz: bool, // {.ftz}
        pub f16: (), // .f16
        pub p: Operand, // p
        pub a: Operand, // a
        pub b: Operand, // b
        pub c_op: bool, // {!} operator
        pub c: Operand, // {!}c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopFtzF16x2 {
        pub cmpop: Cmpop, // .CmpOp
        pub ftz: bool, // {.ftz}
        pub f16x2: (), // .f16x2
        pub p: Operand, // first operand of p|q
        pub q: Operand, // second operand of p|q
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopBoolopFtzF16x2 {
        pub cmpop: Cmpop, // .CmpOp
        pub boolop: Boolop, // .BoolOp
        pub ftz: bool, // {.ftz}
        pub f16x2: (), // .f16x2
        pub p: Operand, // first operand of p|q
        pub q: Operand, // second operand of p|q
        pub a: Operand, // a
        pub b: Operand, // b
        pub c_op: bool, // {!} operator
        pub c: Operand, // {!}c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopBf16 {
        pub cmpop: Cmpop, // .CmpOp
        pub bf16: (), // .bf16
        pub p: Operand, // p
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopBoolopBf16 {
        pub cmpop: Cmpop, // .CmpOp
        pub boolop: Boolop, // .BoolOp
        pub bf16: (), // .bf16
        pub p: Operand, // p
        pub a: Operand, // a
        pub b: Operand, // b
        pub c_op: bool, // {!} operator
        pub c: Operand, // {!}c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopBf16x2 {
        pub cmpop: Cmpop, // .CmpOp
        pub bf16x2: (), // .bf16x2
        pub p: Operand, // first operand of p|q
        pub q: Operand, // second operand of p|q
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopBoolopBf16x2 {
        pub cmpop: Cmpop, // .CmpOp
        pub boolop: Boolop, // .BoolOp
        pub bf16x2: (), // .bf16x2
        pub p: Operand, // first operand of p|q
        pub q: Operand, // second operand of p|q
        pub a: Operand, // a
        pub b: Operand, // b
        pub c_op: bool, // {!} operator
        pub c: Operand, // {!}c
    }

}
