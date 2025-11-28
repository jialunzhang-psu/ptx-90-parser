//! Original PTX specification:
//!
//! set.CmpOp{.ftz}.dtype.stype         d, a, b;
//! set.CmpOp.BoolOp{.ftz}.dtype.stype  d, a, b, {!}c;
//! .CmpOp  = { .eq, .ne, .lt, .le, .gt, .ge, .lo, .ls, .hi, .hs,
//! .equ, .neu, .ltu, .leu, .gtu, .geu, .num, .nan };
//! .BoolOp = { .and, .or, .xor };
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
//! .CmpOp  = { .eq, .ne, .lt, .le, .gt, .ge,
//! .equ, .neu, .ltu, .leu, .gtu, .geu, .num, .nan };
//! .BoolOp = { .and, .or, .xor };
//! .stype  = { .b16, .b32, .b64,
//! .u16, .u32, .u64,
//! .s16, .s32, .s64,
//! .f16, .f32, .f64};

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
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

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Dtype {
        U32, // .u32
        S32, // .s32
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
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

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Boolop {
        And, // .and
        Xor, // .xor
        Or,  // .or
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SetCmpopFtzDtypeStype {
        pub cmpop: Cmpop,      // .CmpOp
        pub ftz: bool,         // {.ftz}
        pub dtype: Dtype,      // .dtype
        pub stype: Stype,      // .stype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SetCmpopBoolopFtzDtypeStype {
        pub cmpop: Cmpop,      // .CmpOp
        pub boolop: Boolop,    // .BoolOp
        pub ftz: bool,         // {.ftz}
        pub dtype: Dtype,      // .dtype
        pub stype: Stype,      // .stype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c_op: bool,        // {!} operator
        pub c: GeneralOperand, // {!}c
        pub span: Span,
    }
}

pub mod section_1 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
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

    #[derive(Debug, Clone, PartialEq, Serialize)]
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

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Boolop {
        And, // .and
        Xor, // .xor
        Or,  // .or
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Dtype {
        U16, // .u16
        S16, // .s16
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SetCmpopFtzF16Stype {
        pub cmpop: Cmpop,      // .CmpOp
        pub ftz: bool,         // {.ftz}
        pub f16: (),           // .f16
        pub stype: Stype,      // .stype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SetCmpopBoolopFtzF16Stype {
        pub cmpop: Cmpop,      // .CmpOp
        pub boolop: Boolop,    // .BoolOp
        pub ftz: bool,         // {.ftz}
        pub f16: (),           // .f16
        pub stype: Stype,      // .stype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c_op: bool,        // {!} operator
        pub c: GeneralOperand, // {!}c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SetCmpopBf16Stype {
        pub cmpop: Cmpop,      // .CmpOp
        pub bf16: (),          // .bf16
        pub stype: Stype,      // .stype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SetCmpopBoolopBf16Stype {
        pub cmpop: Cmpop,      // .CmpOp
        pub boolop: Boolop,    // .BoolOp
        pub bf16: (),          // .bf16
        pub stype: Stype,      // .stype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c_op: bool,        // {!} operator
        pub c: GeneralOperand, // {!}c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SetCmpopFtzDtypeF16 {
        pub cmpop: Cmpop,      // .CmpOp
        pub ftz: bool,         // {.ftz}
        pub dtype: Dtype,      // .dtype
        pub f16: (),           // .f16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SetCmpopBoolopFtzDtypeF16 {
        pub cmpop: Cmpop,      // .CmpOp
        pub boolop: Boolop,    // .BoolOp
        pub ftz: bool,         // {.ftz}
        pub dtype: Dtype,      // .dtype
        pub f16: (),           // .f16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c_op: bool,        // {!} operator
        pub c: GeneralOperand, // {!}c
        pub span: Span,
    }
}

pub mod section_2 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
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

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Dtype {
        U16, // .u16
        S16, // .s16
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Boolop {
        And, // .and
        Xor, // .xor
        Or,  // .or
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SetCmpopDtypeBf16 {
        pub cmpop: Cmpop,      // .CmpOp
        pub dtype: Dtype,      // .dtype
        pub bf16: (),          // .bf16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SetCmpopBoolopDtypeBf16 {
        pub cmpop: Cmpop,      // .CmpOp
        pub boolop: Boolop,    // .BoolOp
        pub dtype: Dtype,      // .dtype
        pub bf16: (),          // .bf16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c_op: bool,        // {!} operator
        pub c: GeneralOperand, // {!}c
        pub span: Span,
    }
}

pub mod section_3 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
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

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Dtype {
        F16x2, // .f16x2
        U32,   // .u32
        S32,   // .s32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Boolop {
        And, // .and
        Xor, // .xor
        Or,  // .or
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SetCmpopFtzDtypeF16x2 {
        pub cmpop: Cmpop,      // .CmpOp
        pub ftz: bool,         // {.ftz}
        pub dtype: Dtype,      // .dtype
        pub f16x2: (),         // .f16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SetCmpopBoolopFtzDtypeF16x2 {
        pub cmpop: Cmpop,      // .CmpOp
        pub boolop: Boolop,    // .BoolOp
        pub ftz: bool,         // {.ftz}
        pub dtype: Dtype,      // .dtype
        pub f16x2: (),         // .f16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c_op: bool,        // {!} operator
        pub c: GeneralOperand, // {!}c
        pub span: Span,
    }
}

pub mod section_4 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
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

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Dtype {
        Bf16x2, // .bf16x2
        U32,    // .u32
        S32,    // .s32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Boolop {
        And, // .and
        Xor, // .xor
        Or,  // .or
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SetCmpopDtypeBf16x2 {
        pub cmpop: Cmpop,      // .CmpOp
        pub dtype: Dtype,      // .dtype
        pub bf16x2: (),        // .bf16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SetCmpopBoolopDtypeBf16x2 {
        pub cmpop: Cmpop,      // .CmpOp
        pub boolop: Boolop,    // .BoolOp
        pub dtype: Dtype,      // .dtype
        pub bf16x2: (),        // .bf16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c_op: bool,        // {!} operator
        pub c: GeneralOperand, // {!}c
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Boolop as Boolop0;
pub use section_0::Cmpop as Cmpop0;
pub use section_0::Dtype as Dtype0;
pub use section_0::SetCmpopBoolopFtzDtypeStype;
pub use section_0::SetCmpopFtzDtypeStype;
pub use section_0::Stype as Stype0;
pub use section_1::Boolop as Boolop1;
pub use section_1::Cmpop as Cmpop1;
pub use section_1::Dtype as Dtype1;
pub use section_1::SetCmpopBf16Stype;
pub use section_1::SetCmpopBoolopBf16Stype;
pub use section_1::SetCmpopBoolopFtzDtypeF16;
pub use section_1::SetCmpopBoolopFtzF16Stype;
pub use section_1::SetCmpopFtzDtypeF16;
pub use section_1::SetCmpopFtzF16Stype;
pub use section_1::Stype as Stype1;
pub use section_2::Boolop as Boolop2;
pub use section_2::Cmpop as Cmpop2;
pub use section_2::Dtype as Dtype2;
pub use section_2::SetCmpopBoolopDtypeBf16;
pub use section_2::SetCmpopDtypeBf16;
pub use section_3::Boolop as Boolop3;
pub use section_3::Cmpop as Cmpop3;
pub use section_3::Dtype as Dtype3;
pub use section_3::SetCmpopBoolopFtzDtypeF16x2;
pub use section_3::SetCmpopFtzDtypeF16x2;
pub use section_4::Boolop as Boolop4;
pub use section_4::Cmpop as Cmpop4;
pub use section_4::Dtype as Dtype4;
pub use section_4::SetCmpopBoolopDtypeBf16x2;
pub use section_4::SetCmpopDtypeBf16x2;
