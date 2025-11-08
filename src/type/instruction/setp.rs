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
        Equ, // .equ
        Neu, // .neu
        Ltu, // .ltu
        Leu, // .leu
        Gtu, // .gtu
        Geu, // .geu
        Num, // .num
        Nan, // .nan
        Eq,  // .eq
        Ne,  // .ne
        Lt,  // .lt
        Le,  // .le
        Gt,  // .gt
        Ge,  // .ge
        Lo,  // .lo
        Ls,  // .ls
        Hi,  // .hi
        Hs,  // .hs
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
        Xor, // .xor
        Or,  // .or
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopFtzType {
        pub cmpop: Cmpop,              // .CmpOp
        pub ftz: bool,                 // {.ftz}
        pub type_: Type,               // .type
        pub p: GeneralOperand,         // first operand of p{|q}
        pub q: Option<GeneralOperand>, // optional second operand of p{|q}
        pub a: GeneralOperand,         // a
        pub b: GeneralOperand,         // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopBoolopFtzType {
        pub cmpop: Cmpop,              // .CmpOp
        pub boolop: Boolop,            // .BoolOp
        pub ftz: bool,                 // {.ftz}
        pub type_: Type,               // .type
        pub p: GeneralOperand,         // first operand of p{|q}
        pub q: Option<GeneralOperand>, // optional second operand of p{|q}
        pub a: GeneralOperand,         // a
        pub b: GeneralOperand,         // b
        pub c_op: bool,                // {!} operator
        pub c: GeneralOperand,         // {!}c
    }
}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Cmpop {
        Equ, // .equ
        Neu, // .neu
        Ltu, // .ltu
        Leu, // .leu
        Gtu, // .gtu
        Geu, // .geu
        Num, // .num
        Nan, // .nan
        Eq,  // .eq
        Ne,  // .ne
        Lt,  // .lt
        Le,  // .le
        Gt,  // .gt
        Ge,  // .ge
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Boolop {
        And, // .and
        Xor, // .xor
        Or,  // .or
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopFtzF16 {
        pub cmpop: Cmpop,      // .CmpOp
        pub ftz: bool,         // {.ftz}
        pub f16: (),           // .f16
        pub p: GeneralOperand, // p
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopBoolopFtzF16 {
        pub cmpop: Cmpop,      // .CmpOp
        pub boolop: Boolop,    // .BoolOp
        pub ftz: bool,         // {.ftz}
        pub f16: (),           // .f16
        pub p: GeneralOperand, // p
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c_op: bool,        // {!} operator
        pub c: GeneralOperand, // {!}c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopFtzF16x2 {
        pub cmpop: Cmpop,      // .CmpOp
        pub ftz: bool,         // {.ftz}
        pub f16x2: (),         // .f16x2
        pub p: GeneralOperand, // first operand of p|q
        pub q: GeneralOperand, // second operand of p|q
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopBoolopFtzF16x2 {
        pub cmpop: Cmpop,      // .CmpOp
        pub boolop: Boolop,    // .BoolOp
        pub ftz: bool,         // {.ftz}
        pub f16x2: (),         // .f16x2
        pub p: GeneralOperand, // first operand of p|q
        pub q: GeneralOperand, // second operand of p|q
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c_op: bool,        // {!} operator
        pub c: GeneralOperand, // {!}c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopBf16 {
        pub cmpop: Cmpop,      // .CmpOp
        pub bf16: (),          // .bf16
        pub p: GeneralOperand, // p
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopBoolopBf16 {
        pub cmpop: Cmpop,      // .CmpOp
        pub boolop: Boolop,    // .BoolOp
        pub bf16: (),          // .bf16
        pub p: GeneralOperand, // p
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c_op: bool,        // {!} operator
        pub c: GeneralOperand, // {!}c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopBf16x2 {
        pub cmpop: Cmpop,      // .CmpOp
        pub bf16x2: (),        // .bf16x2
        pub p: GeneralOperand, // first operand of p|q
        pub q: GeneralOperand, // second operand of p|q
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SetpCmpopBoolopBf16x2 {
        pub cmpop: Cmpop,      // .CmpOp
        pub boolop: Boolop,    // .BoolOp
        pub bf16x2: (),        // .bf16x2
        pub p: GeneralOperand, // first operand of p|q
        pub q: GeneralOperand, // second operand of p|q
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c_op: bool,        // {!} operator
        pub c: GeneralOperand, // {!}c
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Boolop as Boolop0;
pub use section_0::Cmpop as Cmpop0;
pub use section_0::SetpCmpopBoolopFtzType;
pub use section_0::SetpCmpopFtzType;
pub use section_0::Type as Type0;
pub use section_1::Boolop as Boolop1;
pub use section_1::Cmpop as Cmpop1;
pub use section_1::SetpCmpopBf16;
pub use section_1::SetpCmpopBf16x2;
pub use section_1::SetpCmpopBoolopBf16;
pub use section_1::SetpCmpopBoolopBf16x2;
pub use section_1::SetpCmpopBoolopFtzF16;
pub use section_1::SetpCmpopBoolopFtzF16x2;
pub use section_1::SetpCmpopFtzF16;
pub use section_1::SetpCmpopFtzF16x2;
