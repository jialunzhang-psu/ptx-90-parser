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

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::set::section_0::*;

    impl PtxUnparser for SetCmpopFtzDtypeStype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "set");
                    match &self.cmpop {
                            Cmpop::Eq => {
                                    push_directive(tokens, "eq");
                            }
                            Cmpop::Ne => {
                                    push_directive(tokens, "ne");
                            }
                            Cmpop::Lt => {
                                    push_directive(tokens, "lt");
                            }
                            Cmpop::Le => {
                                    push_directive(tokens, "le");
                            }
                            Cmpop::Gt => {
                                    push_directive(tokens, "gt");
                            }
                            Cmpop::Ge => {
                                    push_directive(tokens, "ge");
                            }
                            Cmpop::Lo => {
                                    push_directive(tokens, "lo");
                            }
                            Cmpop::Ls => {
                                    push_directive(tokens, "ls");
                            }
                            Cmpop::Hi => {
                                    push_directive(tokens, "hi");
                            }
                            Cmpop::Hs => {
                                    push_directive(tokens, "hs");
                            }
                            Cmpop::Equ => {
                                    push_directive(tokens, "equ");
                            }
                            Cmpop::Neu => {
                                    push_directive(tokens, "neu");
                            }
                            Cmpop::Ltu => {
                                    push_directive(tokens, "ltu");
                            }
                            Cmpop::Leu => {
                                    push_directive(tokens, "leu");
                            }
                            Cmpop::Gtu => {
                                    push_directive(tokens, "gtu");
                            }
                            Cmpop::Geu => {
                                    push_directive(tokens, "geu");
                            }
                            Cmpop::Num => {
                                    push_directive(tokens, "num");
                            }
                            Cmpop::Nan => {
                                    push_directive(tokens, "nan");
                            }
                    }
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    match &self.dtype {
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    match &self.stype {
                            Stype::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Stype::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Stype::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Stype::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Stype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Stype::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Stype::S16 => {
                                    push_directive(tokens, "s16");
                            }
                            Stype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Stype::S64 => {
                                    push_directive(tokens, "s64");
                            }
                            Stype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Stype::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for SetCmpopBoolopFtzDtypeStype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "set");
                    match &self.cmpop {
                            Cmpop::Eq => {
                                    push_directive(tokens, "eq");
                            }
                            Cmpop::Ne => {
                                    push_directive(tokens, "ne");
                            }
                            Cmpop::Lt => {
                                    push_directive(tokens, "lt");
                            }
                            Cmpop::Le => {
                                    push_directive(tokens, "le");
                            }
                            Cmpop::Gt => {
                                    push_directive(tokens, "gt");
                            }
                            Cmpop::Ge => {
                                    push_directive(tokens, "ge");
                            }
                            Cmpop::Lo => {
                                    push_directive(tokens, "lo");
                            }
                            Cmpop::Ls => {
                                    push_directive(tokens, "ls");
                            }
                            Cmpop::Hi => {
                                    push_directive(tokens, "hi");
                            }
                            Cmpop::Hs => {
                                    push_directive(tokens, "hs");
                            }
                            Cmpop::Equ => {
                                    push_directive(tokens, "equ");
                            }
                            Cmpop::Neu => {
                                    push_directive(tokens, "neu");
                            }
                            Cmpop::Ltu => {
                                    push_directive(tokens, "ltu");
                            }
                            Cmpop::Leu => {
                                    push_directive(tokens, "leu");
                            }
                            Cmpop::Gtu => {
                                    push_directive(tokens, "gtu");
                            }
                            Cmpop::Geu => {
                                    push_directive(tokens, "geu");
                            }
                            Cmpop::Num => {
                                    push_directive(tokens, "num");
                            }
                            Cmpop::Nan => {
                                    push_directive(tokens, "nan");
                            }
                    }
                    match &self.boolop {
                            Boolop::And => {
                                    push_directive(tokens, "and");
                            }
                            Boolop::Or => {
                                    push_directive(tokens, "or");
                            }
                            Boolop::Xor => {
                                    push_directive(tokens, "xor");
                            }
                    }
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    match &self.dtype {
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    match &self.stype {
                            Stype::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Stype::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Stype::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Stype::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Stype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Stype::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Stype::S16 => {
                                    push_directive(tokens, "s16");
                            }
                            Stype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Stype::S64 => {
                                    push_directive(tokens, "s64");
                            }
                            Stype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Stype::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            if self.c_op { tokens.push(PtxToken::Exclaim); }
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::set::section_1::*;

    impl PtxUnparser for SetCmpopFtzF16Stype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "set");
                    match &self.cmpop {
                            Cmpop::Eq => {
                                    push_directive(tokens, "eq");
                            }
                            Cmpop::Ne => {
                                    push_directive(tokens, "ne");
                            }
                            Cmpop::Lt => {
                                    push_directive(tokens, "lt");
                            }
                            Cmpop::Le => {
                                    push_directive(tokens, "le");
                            }
                            Cmpop::Gt => {
                                    push_directive(tokens, "gt");
                            }
                            Cmpop::Ge => {
                                    push_directive(tokens, "ge");
                            }
                            Cmpop::Lo => {
                                    push_directive(tokens, "lo");
                            }
                            Cmpop::Ls => {
                                    push_directive(tokens, "ls");
                            }
                            Cmpop::Hi => {
                                    push_directive(tokens, "hi");
                            }
                            Cmpop::Hs => {
                                    push_directive(tokens, "hs");
                            }
                            Cmpop::Equ => {
                                    push_directive(tokens, "equ");
                            }
                            Cmpop::Neu => {
                                    push_directive(tokens, "neu");
                            }
                            Cmpop::Ltu => {
                                    push_directive(tokens, "ltu");
                            }
                            Cmpop::Leu => {
                                    push_directive(tokens, "leu");
                            }
                            Cmpop::Gtu => {
                                    push_directive(tokens, "gtu");
                            }
                            Cmpop::Geu => {
                                    push_directive(tokens, "geu");
                            }
                            Cmpop::Num => {
                                    push_directive(tokens, "num");
                            }
                            Cmpop::Nan => {
                                    push_directive(tokens, "nan");
                            }
                    }
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    push_directive(tokens, "f16");
                    match &self.stype {
                            Stype::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Stype::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Stype::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Stype::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Stype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Stype::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Stype::S16 => {
                                    push_directive(tokens, "s16");
                            }
                            Stype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Stype::S64 => {
                                    push_directive(tokens, "s64");
                            }
                            Stype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Stype::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for SetCmpopBoolopFtzF16Stype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "set");
                    match &self.cmpop {
                            Cmpop::Eq => {
                                    push_directive(tokens, "eq");
                            }
                            Cmpop::Ne => {
                                    push_directive(tokens, "ne");
                            }
                            Cmpop::Lt => {
                                    push_directive(tokens, "lt");
                            }
                            Cmpop::Le => {
                                    push_directive(tokens, "le");
                            }
                            Cmpop::Gt => {
                                    push_directive(tokens, "gt");
                            }
                            Cmpop::Ge => {
                                    push_directive(tokens, "ge");
                            }
                            Cmpop::Lo => {
                                    push_directive(tokens, "lo");
                            }
                            Cmpop::Ls => {
                                    push_directive(tokens, "ls");
                            }
                            Cmpop::Hi => {
                                    push_directive(tokens, "hi");
                            }
                            Cmpop::Hs => {
                                    push_directive(tokens, "hs");
                            }
                            Cmpop::Equ => {
                                    push_directive(tokens, "equ");
                            }
                            Cmpop::Neu => {
                                    push_directive(tokens, "neu");
                            }
                            Cmpop::Ltu => {
                                    push_directive(tokens, "ltu");
                            }
                            Cmpop::Leu => {
                                    push_directive(tokens, "leu");
                            }
                            Cmpop::Gtu => {
                                    push_directive(tokens, "gtu");
                            }
                            Cmpop::Geu => {
                                    push_directive(tokens, "geu");
                            }
                            Cmpop::Num => {
                                    push_directive(tokens, "num");
                            }
                            Cmpop::Nan => {
                                    push_directive(tokens, "nan");
                            }
                    }
                    match &self.boolop {
                            Boolop::And => {
                                    push_directive(tokens, "and");
                            }
                            Boolop::Or => {
                                    push_directive(tokens, "or");
                            }
                            Boolop::Xor => {
                                    push_directive(tokens, "xor");
                            }
                    }
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    push_directive(tokens, "f16");
                    match &self.stype {
                            Stype::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Stype::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Stype::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Stype::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Stype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Stype::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Stype::S16 => {
                                    push_directive(tokens, "s16");
                            }
                            Stype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Stype::S64 => {
                                    push_directive(tokens, "s64");
                            }
                            Stype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Stype::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            if self.c_op { tokens.push(PtxToken::Exclaim); }
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for SetCmpopBf16Stype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "set");
                    match &self.cmpop {
                            Cmpop::Eq => {
                                    push_directive(tokens, "eq");
                            }
                            Cmpop::Ne => {
                                    push_directive(tokens, "ne");
                            }
                            Cmpop::Lt => {
                                    push_directive(tokens, "lt");
                            }
                            Cmpop::Le => {
                                    push_directive(tokens, "le");
                            }
                            Cmpop::Gt => {
                                    push_directive(tokens, "gt");
                            }
                            Cmpop::Ge => {
                                    push_directive(tokens, "ge");
                            }
                            Cmpop::Lo => {
                                    push_directive(tokens, "lo");
                            }
                            Cmpop::Ls => {
                                    push_directive(tokens, "ls");
                            }
                            Cmpop::Hi => {
                                    push_directive(tokens, "hi");
                            }
                            Cmpop::Hs => {
                                    push_directive(tokens, "hs");
                            }
                            Cmpop::Equ => {
                                    push_directive(tokens, "equ");
                            }
                            Cmpop::Neu => {
                                    push_directive(tokens, "neu");
                            }
                            Cmpop::Ltu => {
                                    push_directive(tokens, "ltu");
                            }
                            Cmpop::Leu => {
                                    push_directive(tokens, "leu");
                            }
                            Cmpop::Gtu => {
                                    push_directive(tokens, "gtu");
                            }
                            Cmpop::Geu => {
                                    push_directive(tokens, "geu");
                            }
                            Cmpop::Num => {
                                    push_directive(tokens, "num");
                            }
                            Cmpop::Nan => {
                                    push_directive(tokens, "nan");
                            }
                    }
                    push_directive(tokens, "bf16");
                    match &self.stype {
                            Stype::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Stype::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Stype::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Stype::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Stype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Stype::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Stype::S16 => {
                                    push_directive(tokens, "s16");
                            }
                            Stype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Stype::S64 => {
                                    push_directive(tokens, "s64");
                            }
                            Stype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Stype::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for SetCmpopBoolopBf16Stype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "set");
                    match &self.cmpop {
                            Cmpop::Eq => {
                                    push_directive(tokens, "eq");
                            }
                            Cmpop::Ne => {
                                    push_directive(tokens, "ne");
                            }
                            Cmpop::Lt => {
                                    push_directive(tokens, "lt");
                            }
                            Cmpop::Le => {
                                    push_directive(tokens, "le");
                            }
                            Cmpop::Gt => {
                                    push_directive(tokens, "gt");
                            }
                            Cmpop::Ge => {
                                    push_directive(tokens, "ge");
                            }
                            Cmpop::Lo => {
                                    push_directive(tokens, "lo");
                            }
                            Cmpop::Ls => {
                                    push_directive(tokens, "ls");
                            }
                            Cmpop::Hi => {
                                    push_directive(tokens, "hi");
                            }
                            Cmpop::Hs => {
                                    push_directive(tokens, "hs");
                            }
                            Cmpop::Equ => {
                                    push_directive(tokens, "equ");
                            }
                            Cmpop::Neu => {
                                    push_directive(tokens, "neu");
                            }
                            Cmpop::Ltu => {
                                    push_directive(tokens, "ltu");
                            }
                            Cmpop::Leu => {
                                    push_directive(tokens, "leu");
                            }
                            Cmpop::Gtu => {
                                    push_directive(tokens, "gtu");
                            }
                            Cmpop::Geu => {
                                    push_directive(tokens, "geu");
                            }
                            Cmpop::Num => {
                                    push_directive(tokens, "num");
                            }
                            Cmpop::Nan => {
                                    push_directive(tokens, "nan");
                            }
                    }
                    match &self.boolop {
                            Boolop::And => {
                                    push_directive(tokens, "and");
                            }
                            Boolop::Or => {
                                    push_directive(tokens, "or");
                            }
                            Boolop::Xor => {
                                    push_directive(tokens, "xor");
                            }
                    }
                    push_directive(tokens, "bf16");
                    match &self.stype {
                            Stype::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Stype::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Stype::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Stype::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Stype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Stype::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Stype::S16 => {
                                    push_directive(tokens, "s16");
                            }
                            Stype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Stype::S64 => {
                                    push_directive(tokens, "s64");
                            }
                            Stype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Stype::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            if self.c_op { tokens.push(PtxToken::Exclaim); }
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for SetCmpopFtzDtypeF16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "set");
                    match &self.cmpop {
                            Cmpop::Eq => {
                                    push_directive(tokens, "eq");
                            }
                            Cmpop::Ne => {
                                    push_directive(tokens, "ne");
                            }
                            Cmpop::Lt => {
                                    push_directive(tokens, "lt");
                            }
                            Cmpop::Le => {
                                    push_directive(tokens, "le");
                            }
                            Cmpop::Gt => {
                                    push_directive(tokens, "gt");
                            }
                            Cmpop::Ge => {
                                    push_directive(tokens, "ge");
                            }
                            Cmpop::Lo => {
                                    push_directive(tokens, "lo");
                            }
                            Cmpop::Ls => {
                                    push_directive(tokens, "ls");
                            }
                            Cmpop::Hi => {
                                    push_directive(tokens, "hi");
                            }
                            Cmpop::Hs => {
                                    push_directive(tokens, "hs");
                            }
                            Cmpop::Equ => {
                                    push_directive(tokens, "equ");
                            }
                            Cmpop::Neu => {
                                    push_directive(tokens, "neu");
                            }
                            Cmpop::Ltu => {
                                    push_directive(tokens, "ltu");
                            }
                            Cmpop::Leu => {
                                    push_directive(tokens, "leu");
                            }
                            Cmpop::Gtu => {
                                    push_directive(tokens, "gtu");
                            }
                            Cmpop::Geu => {
                                    push_directive(tokens, "geu");
                            }
                            Cmpop::Num => {
                                    push_directive(tokens, "num");
                            }
                            Cmpop::Nan => {
                                    push_directive(tokens, "nan");
                            }
                    }
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    match &self.dtype {
                            Dtype::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Dtype::S16 => {
                                    push_directive(tokens, "s16");
                            }
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    push_directive(tokens, "f16");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for SetCmpopBoolopFtzDtypeF16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "set");
                    match &self.cmpop {
                            Cmpop::Eq => {
                                    push_directive(tokens, "eq");
                            }
                            Cmpop::Ne => {
                                    push_directive(tokens, "ne");
                            }
                            Cmpop::Lt => {
                                    push_directive(tokens, "lt");
                            }
                            Cmpop::Le => {
                                    push_directive(tokens, "le");
                            }
                            Cmpop::Gt => {
                                    push_directive(tokens, "gt");
                            }
                            Cmpop::Ge => {
                                    push_directive(tokens, "ge");
                            }
                            Cmpop::Lo => {
                                    push_directive(tokens, "lo");
                            }
                            Cmpop::Ls => {
                                    push_directive(tokens, "ls");
                            }
                            Cmpop::Hi => {
                                    push_directive(tokens, "hi");
                            }
                            Cmpop::Hs => {
                                    push_directive(tokens, "hs");
                            }
                            Cmpop::Equ => {
                                    push_directive(tokens, "equ");
                            }
                            Cmpop::Neu => {
                                    push_directive(tokens, "neu");
                            }
                            Cmpop::Ltu => {
                                    push_directive(tokens, "ltu");
                            }
                            Cmpop::Leu => {
                                    push_directive(tokens, "leu");
                            }
                            Cmpop::Gtu => {
                                    push_directive(tokens, "gtu");
                            }
                            Cmpop::Geu => {
                                    push_directive(tokens, "geu");
                            }
                            Cmpop::Num => {
                                    push_directive(tokens, "num");
                            }
                            Cmpop::Nan => {
                                    push_directive(tokens, "nan");
                            }
                    }
                    match &self.boolop {
                            Boolop::And => {
                                    push_directive(tokens, "and");
                            }
                            Boolop::Or => {
                                    push_directive(tokens, "or");
                            }
                            Boolop::Xor => {
                                    push_directive(tokens, "xor");
                            }
                    }
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    match &self.dtype {
                            Dtype::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Dtype::S16 => {
                                    push_directive(tokens, "s16");
                            }
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    push_directive(tokens, "f16");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            if self.c_op { tokens.push(PtxToken::Exclaim); }
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::set::section_2::*;

    impl PtxUnparser for SetCmpopDtypeBf16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "set");
                    match &self.cmpop {
                            Cmpop::Eq => {
                                    push_directive(tokens, "eq");
                            }
                            Cmpop::Ne => {
                                    push_directive(tokens, "ne");
                            }
                            Cmpop::Lt => {
                                    push_directive(tokens, "lt");
                            }
                            Cmpop::Le => {
                                    push_directive(tokens, "le");
                            }
                            Cmpop::Gt => {
                                    push_directive(tokens, "gt");
                            }
                            Cmpop::Ge => {
                                    push_directive(tokens, "ge");
                            }
                            Cmpop::Lo => {
                                    push_directive(tokens, "lo");
                            }
                            Cmpop::Ls => {
                                    push_directive(tokens, "ls");
                            }
                            Cmpop::Hi => {
                                    push_directive(tokens, "hi");
                            }
                            Cmpop::Hs => {
                                    push_directive(tokens, "hs");
                            }
                            Cmpop::Equ => {
                                    push_directive(tokens, "equ");
                            }
                            Cmpop::Neu => {
                                    push_directive(tokens, "neu");
                            }
                            Cmpop::Ltu => {
                                    push_directive(tokens, "ltu");
                            }
                            Cmpop::Leu => {
                                    push_directive(tokens, "leu");
                            }
                            Cmpop::Gtu => {
                                    push_directive(tokens, "gtu");
                            }
                            Cmpop::Geu => {
                                    push_directive(tokens, "geu");
                            }
                            Cmpop::Num => {
                                    push_directive(tokens, "num");
                            }
                            Cmpop::Nan => {
                                    push_directive(tokens, "nan");
                            }
                    }
                    match &self.dtype {
                            Dtype::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Dtype::S16 => {
                                    push_directive(tokens, "s16");
                            }
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    push_directive(tokens, "bf16");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for SetCmpopBoolopDtypeBf16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "set");
                    match &self.cmpop {
                            Cmpop::Eq => {
                                    push_directive(tokens, "eq");
                            }
                            Cmpop::Ne => {
                                    push_directive(tokens, "ne");
                            }
                            Cmpop::Lt => {
                                    push_directive(tokens, "lt");
                            }
                            Cmpop::Le => {
                                    push_directive(tokens, "le");
                            }
                            Cmpop::Gt => {
                                    push_directive(tokens, "gt");
                            }
                            Cmpop::Ge => {
                                    push_directive(tokens, "ge");
                            }
                            Cmpop::Lo => {
                                    push_directive(tokens, "lo");
                            }
                            Cmpop::Ls => {
                                    push_directive(tokens, "ls");
                            }
                            Cmpop::Hi => {
                                    push_directive(tokens, "hi");
                            }
                            Cmpop::Hs => {
                                    push_directive(tokens, "hs");
                            }
                            Cmpop::Equ => {
                                    push_directive(tokens, "equ");
                            }
                            Cmpop::Neu => {
                                    push_directive(tokens, "neu");
                            }
                            Cmpop::Ltu => {
                                    push_directive(tokens, "ltu");
                            }
                            Cmpop::Leu => {
                                    push_directive(tokens, "leu");
                            }
                            Cmpop::Gtu => {
                                    push_directive(tokens, "gtu");
                            }
                            Cmpop::Geu => {
                                    push_directive(tokens, "geu");
                            }
                            Cmpop::Num => {
                                    push_directive(tokens, "num");
                            }
                            Cmpop::Nan => {
                                    push_directive(tokens, "nan");
                            }
                    }
                    match &self.boolop {
                            Boolop::And => {
                                    push_directive(tokens, "and");
                            }
                            Boolop::Or => {
                                    push_directive(tokens, "or");
                            }
                            Boolop::Xor => {
                                    push_directive(tokens, "xor");
                            }
                    }
                    match &self.dtype {
                            Dtype::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Dtype::S16 => {
                                    push_directive(tokens, "s16");
                            }
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    push_directive(tokens, "bf16");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            if self.c_op { tokens.push(PtxToken::Exclaim); }
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::set::section_3::*;

    impl PtxUnparser for SetCmpopFtzDtypeF16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "set");
                    match &self.cmpop {
                            Cmpop::Eq => {
                                    push_directive(tokens, "eq");
                            }
                            Cmpop::Ne => {
                                    push_directive(tokens, "ne");
                            }
                            Cmpop::Lt => {
                                    push_directive(tokens, "lt");
                            }
                            Cmpop::Le => {
                                    push_directive(tokens, "le");
                            }
                            Cmpop::Gt => {
                                    push_directive(tokens, "gt");
                            }
                            Cmpop::Ge => {
                                    push_directive(tokens, "ge");
                            }
                            Cmpop::Lo => {
                                    push_directive(tokens, "lo");
                            }
                            Cmpop::Ls => {
                                    push_directive(tokens, "ls");
                            }
                            Cmpop::Hi => {
                                    push_directive(tokens, "hi");
                            }
                            Cmpop::Hs => {
                                    push_directive(tokens, "hs");
                            }
                            Cmpop::Equ => {
                                    push_directive(tokens, "equ");
                            }
                            Cmpop::Neu => {
                                    push_directive(tokens, "neu");
                            }
                            Cmpop::Ltu => {
                                    push_directive(tokens, "ltu");
                            }
                            Cmpop::Leu => {
                                    push_directive(tokens, "leu");
                            }
                            Cmpop::Gtu => {
                                    push_directive(tokens, "gtu");
                            }
                            Cmpop::Geu => {
                                    push_directive(tokens, "geu");
                            }
                            Cmpop::Num => {
                                    push_directive(tokens, "num");
                            }
                            Cmpop::Nan => {
                                    push_directive(tokens, "nan");
                            }
                    }
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    match &self.dtype {
                            Dtype::F16x2 => {
                                    push_directive(tokens, "f16x2");
                            }
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    push_directive(tokens, "f16x2");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for SetCmpopBoolopFtzDtypeF16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "set");
                    match &self.cmpop {
                            Cmpop::Eq => {
                                    push_directive(tokens, "eq");
                            }
                            Cmpop::Ne => {
                                    push_directive(tokens, "ne");
                            }
                            Cmpop::Lt => {
                                    push_directive(tokens, "lt");
                            }
                            Cmpop::Le => {
                                    push_directive(tokens, "le");
                            }
                            Cmpop::Gt => {
                                    push_directive(tokens, "gt");
                            }
                            Cmpop::Ge => {
                                    push_directive(tokens, "ge");
                            }
                            Cmpop::Lo => {
                                    push_directive(tokens, "lo");
                            }
                            Cmpop::Ls => {
                                    push_directive(tokens, "ls");
                            }
                            Cmpop::Hi => {
                                    push_directive(tokens, "hi");
                            }
                            Cmpop::Hs => {
                                    push_directive(tokens, "hs");
                            }
                            Cmpop::Equ => {
                                    push_directive(tokens, "equ");
                            }
                            Cmpop::Neu => {
                                    push_directive(tokens, "neu");
                            }
                            Cmpop::Ltu => {
                                    push_directive(tokens, "ltu");
                            }
                            Cmpop::Leu => {
                                    push_directive(tokens, "leu");
                            }
                            Cmpop::Gtu => {
                                    push_directive(tokens, "gtu");
                            }
                            Cmpop::Geu => {
                                    push_directive(tokens, "geu");
                            }
                            Cmpop::Num => {
                                    push_directive(tokens, "num");
                            }
                            Cmpop::Nan => {
                                    push_directive(tokens, "nan");
                            }
                    }
                    match &self.boolop {
                            Boolop::And => {
                                    push_directive(tokens, "and");
                            }
                            Boolop::Or => {
                                    push_directive(tokens, "or");
                            }
                            Boolop::Xor => {
                                    push_directive(tokens, "xor");
                            }
                    }
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    match &self.dtype {
                            Dtype::F16x2 => {
                                    push_directive(tokens, "f16x2");
                            }
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    push_directive(tokens, "f16x2");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            if self.c_op { tokens.push(PtxToken::Exclaim); }
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::set::section_4::*;

    impl PtxUnparser for SetCmpopDtypeBf16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "set");
                    match &self.cmpop {
                            Cmpop::Eq => {
                                    push_directive(tokens, "eq");
                            }
                            Cmpop::Ne => {
                                    push_directive(tokens, "ne");
                            }
                            Cmpop::Lt => {
                                    push_directive(tokens, "lt");
                            }
                            Cmpop::Le => {
                                    push_directive(tokens, "le");
                            }
                            Cmpop::Gt => {
                                    push_directive(tokens, "gt");
                            }
                            Cmpop::Ge => {
                                    push_directive(tokens, "ge");
                            }
                            Cmpop::Equ => {
                                    push_directive(tokens, "equ");
                            }
                            Cmpop::Neu => {
                                    push_directive(tokens, "neu");
                            }
                            Cmpop::Ltu => {
                                    push_directive(tokens, "ltu");
                            }
                            Cmpop::Leu => {
                                    push_directive(tokens, "leu");
                            }
                            Cmpop::Gtu => {
                                    push_directive(tokens, "gtu");
                            }
                            Cmpop::Geu => {
                                    push_directive(tokens, "geu");
                            }
                            Cmpop::Num => {
                                    push_directive(tokens, "num");
                            }
                            Cmpop::Nan => {
                                    push_directive(tokens, "nan");
                            }
                    }
                    match &self.dtype {
                            Dtype::Bf16x2 => {
                                    push_directive(tokens, "bf16x2");
                            }
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    push_directive(tokens, "bf16x2");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for SetCmpopBoolopDtypeBf16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "set");
                    match &self.cmpop {
                            Cmpop::Eq => {
                                    push_directive(tokens, "eq");
                            }
                            Cmpop::Ne => {
                                    push_directive(tokens, "ne");
                            }
                            Cmpop::Lt => {
                                    push_directive(tokens, "lt");
                            }
                            Cmpop::Le => {
                                    push_directive(tokens, "le");
                            }
                            Cmpop::Gt => {
                                    push_directive(tokens, "gt");
                            }
                            Cmpop::Ge => {
                                    push_directive(tokens, "ge");
                            }
                            Cmpop::Equ => {
                                    push_directive(tokens, "equ");
                            }
                            Cmpop::Neu => {
                                    push_directive(tokens, "neu");
                            }
                            Cmpop::Ltu => {
                                    push_directive(tokens, "ltu");
                            }
                            Cmpop::Leu => {
                                    push_directive(tokens, "leu");
                            }
                            Cmpop::Gtu => {
                                    push_directive(tokens, "gtu");
                            }
                            Cmpop::Geu => {
                                    push_directive(tokens, "geu");
                            }
                            Cmpop::Num => {
                                    push_directive(tokens, "num");
                            }
                            Cmpop::Nan => {
                                    push_directive(tokens, "nan");
                            }
                    }
                    match &self.boolop {
                            Boolop::And => {
                                    push_directive(tokens, "and");
                            }
                            Boolop::Or => {
                                    push_directive(tokens, "or");
                            }
                            Boolop::Xor => {
                                    push_directive(tokens, "xor");
                            }
                    }
                    match &self.dtype {
                            Dtype::Bf16x2 => {
                                    push_directive(tokens, "bf16x2");
                            }
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    push_directive(tokens, "bf16x2");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            if self.c_op { tokens.push(PtxToken::Exclaim); }
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

