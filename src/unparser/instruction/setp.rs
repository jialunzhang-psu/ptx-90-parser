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

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::setp::section_0::*;

    impl PtxUnparser for SetpCmpopFtzType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "setp");
            match &self.cmpop {
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
            }
            if self.ftz {
                push_directive(tokens, "ftz");
            }
            match &self.type_ {
                Type::B16 => {
                    push_directive(tokens, "b16");
                }
                Type::B32 => {
                    push_directive(tokens, "b32");
                }
                Type::B64 => {
                    push_directive(tokens, "b64");
                }
                Type::U16 => {
                    push_directive(tokens, "u16");
                }
                Type::U32 => {
                    push_directive(tokens, "u32");
                }
                Type::U64 => {
                    push_directive(tokens, "u64");
                }
                Type::S16 => {
                    push_directive(tokens, "s16");
                }
                Type::S32 => {
                    push_directive(tokens, "s32");
                }
                Type::S64 => {
                    push_directive(tokens, "s64");
                }
                Type::F32 => {
                    push_directive(tokens, "f32");
                }
                Type::F64 => {
                    push_directive(tokens, "f64");
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.p.unparse_tokens_mode(tokens, spaced);
            if let Some(q_0) = self.q.as_ref() {
                tokens.push(PtxToken::Pipe);
                q_0.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for SetpCmpopBoolopFtzType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "setp");
            match &self.cmpop {
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
            }
            match &self.boolop {
                Boolop::And => {
                    push_directive(tokens, "and");
                }
                Boolop::Xor => {
                    push_directive(tokens, "xor");
                }
                Boolop::Or => {
                    push_directive(tokens, "or");
                }
            }
            if self.ftz {
                push_directive(tokens, "ftz");
            }
            match &self.type_ {
                Type::B16 => {
                    push_directive(tokens, "b16");
                }
                Type::B32 => {
                    push_directive(tokens, "b32");
                }
                Type::B64 => {
                    push_directive(tokens, "b64");
                }
                Type::U16 => {
                    push_directive(tokens, "u16");
                }
                Type::U32 => {
                    push_directive(tokens, "u32");
                }
                Type::U64 => {
                    push_directive(tokens, "u64");
                }
                Type::S16 => {
                    push_directive(tokens, "s16");
                }
                Type::S32 => {
                    push_directive(tokens, "s32");
                }
                Type::S64 => {
                    push_directive(tokens, "s64");
                }
                Type::F32 => {
                    push_directive(tokens, "f32");
                }
                Type::F64 => {
                    push_directive(tokens, "f64");
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.p.unparse_tokens_mode(tokens, spaced);
            if let Some(q_1) = self.q.as_ref() {
                tokens.push(PtxToken::Pipe);
                q_1.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if self.c_op {
                tokens.push(PtxToken::Exclaim);
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::setp::section_1::*;

    impl PtxUnparser for SetpCmpopFtzF16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "setp");
            match &self.cmpop {
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
            }
            if self.ftz {
                push_directive(tokens, "ftz");
            }
            push_directive(tokens, "f16");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.p.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for SetpCmpopBoolopFtzF16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "setp");
            match &self.cmpop {
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
            }
            match &self.boolop {
                Boolop::And => {
                    push_directive(tokens, "and");
                }
                Boolop::Xor => {
                    push_directive(tokens, "xor");
                }
                Boolop::Or => {
                    push_directive(tokens, "or");
                }
            }
            if self.ftz {
                push_directive(tokens, "ftz");
            }
            push_directive(tokens, "f16");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.p.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if self.c_op {
                tokens.push(PtxToken::Exclaim);
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for SetpCmpopFtzF16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "setp");
            match &self.cmpop {
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
            }
            if self.ftz {
                push_directive(tokens, "ftz");
            }
            push_directive(tokens, "f16x2");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.p.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Pipe);
            self.q.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for SetpCmpopBoolopFtzF16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "setp");
            match &self.cmpop {
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
            }
            match &self.boolop {
                Boolop::And => {
                    push_directive(tokens, "and");
                }
                Boolop::Xor => {
                    push_directive(tokens, "xor");
                }
                Boolop::Or => {
                    push_directive(tokens, "or");
                }
            }
            if self.ftz {
                push_directive(tokens, "ftz");
            }
            push_directive(tokens, "f16x2");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.p.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Pipe);
            self.q.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if self.c_op {
                tokens.push(PtxToken::Exclaim);
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for SetpCmpopBf16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "setp");
            match &self.cmpop {
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
            }
            push_directive(tokens, "bf16");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.p.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for SetpCmpopBoolopBf16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "setp");
            match &self.cmpop {
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
            }
            match &self.boolop {
                Boolop::And => {
                    push_directive(tokens, "and");
                }
                Boolop::Xor => {
                    push_directive(tokens, "xor");
                }
                Boolop::Or => {
                    push_directive(tokens, "or");
                }
            }
            push_directive(tokens, "bf16");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.p.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if self.c_op {
                tokens.push(PtxToken::Exclaim);
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for SetpCmpopBf16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "setp");
            match &self.cmpop {
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
            }
            push_directive(tokens, "bf16x2");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.p.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Pipe);
            self.q.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for SetpCmpopBoolopBf16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "setp");
            match &self.cmpop {
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
            }
            match &self.boolop {
                Boolop::And => {
                    push_directive(tokens, "and");
                }
                Boolop::Xor => {
                    push_directive(tokens, "xor");
                }
                Boolop::Or => {
                    push_directive(tokens, "or");
                }
            }
            push_directive(tokens, "bf16x2");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.p.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Pipe);
            self.q.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if self.c_op {
                tokens.push(PtxToken::Exclaim);
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
