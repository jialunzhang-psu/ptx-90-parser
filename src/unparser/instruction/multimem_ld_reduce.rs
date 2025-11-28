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

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::multimem_ld_reduce::section_0::*;

    impl PtxUnparser for MultimemLdReduceLdsemScopeSsOpType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "multimem");
            push_directive(tokens, "ld_reduce");
            if let Some(ldsem_0) = self.ldsem.as_ref() {
                match ldsem_0 {
                    Ldsem::Relaxed => {
                        push_directive(tokens, "relaxed");
                    }
                    Ldsem::Acquire => {
                        push_directive(tokens, "acquire");
                    }
                }
            }
            if let Some(scope_1) = self.scope.as_ref() {
                match scope_1 {
                    Scope::Cluster => {
                        push_directive(tokens, "cluster");
                    }
                    Scope::Cta => {
                        push_directive(tokens, "cta");
                    }
                    Scope::Gpu => {
                        push_directive(tokens, "gpu");
                    }
                    Scope::Sys => {
                        push_directive(tokens, "sys");
                    }
                }
            }
            if let Some(ss_2) = self.ss.as_ref() {
                match ss_2 {
                    Ss::Global => {
                        push_directive(tokens, "global");
                    }
                }
            }
            match &self.op {
                Op::Min => {
                    push_directive(tokens, "min");
                }
                Op::Max => {
                    push_directive(tokens, "max");
                }
                Op::Add => {
                    push_directive(tokens, "add");
                }
                Op::And => {
                    push_directive(tokens, "and");
                }
                Op::Xor => {
                    push_directive(tokens, "xor");
                }
                Op::Or => {
                    push_directive(tokens, "or");
                }
            }
            match &self.type_ {
                Type::B32 => {
                    push_directive(tokens, "b32");
                }
                Type::B64 => {
                    push_directive(tokens, "b64");
                }
                Type::U32 => {
                    push_directive(tokens, "u32");
                }
                Type::U64 => {
                    push_directive(tokens, "u64");
                }
                Type::S32 => {
                    push_directive(tokens, "s32");
                }
                Type::S64 => {
                    push_directive(tokens, "s64");
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for MultimemLdReduceWeakSsOpType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "multimem");
            push_directive(tokens, "ld_reduce");
            push_directive(tokens, "weak");
            if let Some(ss_3) = self.ss.as_ref() {
                match ss_3 {
                    Ss::Global => {
                        push_directive(tokens, "global");
                    }
                }
            }
            match &self.op {
                Op::Min => {
                    push_directive(tokens, "min");
                }
                Op::Max => {
                    push_directive(tokens, "max");
                }
                Op::Add => {
                    push_directive(tokens, "add");
                }
                Op::And => {
                    push_directive(tokens, "and");
                }
                Op::Xor => {
                    push_directive(tokens, "xor");
                }
                Op::Or => {
                    push_directive(tokens, "or");
                }
            }
            match &self.type_ {
                Type::B32 => {
                    push_directive(tokens, "b32");
                }
                Type::B64 => {
                    push_directive(tokens, "b64");
                }
                Type::U32 => {
                    push_directive(tokens, "u32");
                }
                Type::U64 => {
                    push_directive(tokens, "u64");
                }
                Type::S32 => {
                    push_directive(tokens, "s32");
                }
                Type::S64 => {
                    push_directive(tokens, "s64");
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for MultimemStStsemScopeSsType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "multimem");
            push_directive(tokens, "st");
            if let Some(stsem_4) = self.stsem.as_ref() {
                match stsem_4 {
                    Stsem::Relaxed => {
                        push_directive(tokens, "relaxed");
                    }
                    Stsem::Release => {
                        push_directive(tokens, "release");
                    }
                }
            }
            if let Some(scope_5) = self.scope.as_ref() {
                match scope_5 {
                    Scope::Cluster => {
                        push_directive(tokens, "cluster");
                    }
                    Scope::Cta => {
                        push_directive(tokens, "cta");
                    }
                    Scope::Gpu => {
                        push_directive(tokens, "gpu");
                    }
                    Scope::Sys => {
                        push_directive(tokens, "sys");
                    }
                }
            }
            if let Some(ss_6) = self.ss.as_ref() {
                match ss_6 {
                    Ss::Global => {
                        push_directive(tokens, "global");
                    }
                }
            }
            match &self.type_ {
                Type::B32 => {
                    push_directive(tokens, "b32");
                }
                Type::B64 => {
                    push_directive(tokens, "b64");
                }
                Type::U32 => {
                    push_directive(tokens, "u32");
                }
                Type::U64 => {
                    push_directive(tokens, "u64");
                }
                Type::S32 => {
                    push_directive(tokens, "s32");
                }
                Type::S64 => {
                    push_directive(tokens, "s64");
                }
            }
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

    impl PtxUnparser for MultimemStWeakSsType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "multimem");
            push_directive(tokens, "st");
            push_directive(tokens, "weak");
            if let Some(ss_7) = self.ss.as_ref() {
                match ss_7 {
                    Ss::Global => {
                        push_directive(tokens, "global");
                    }
                }
            }
            match &self.type_ {
                Type::B32 => {
                    push_directive(tokens, "b32");
                }
                Type::B64 => {
                    push_directive(tokens, "b64");
                }
                Type::U32 => {
                    push_directive(tokens, "u32");
                }
                Type::U64 => {
                    push_directive(tokens, "u64");
                }
                Type::S32 => {
                    push_directive(tokens, "s32");
                }
                Type::S64 => {
                    push_directive(tokens, "s64");
                }
            }
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

    impl PtxUnparser for MultimemRedRedsemScopeSsOpType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "multimem");
            push_directive(tokens, "red");
            if let Some(redsem_8) = self.redsem.as_ref() {
                match redsem_8 {
                    Redsem::Relaxed => {
                        push_directive(tokens, "relaxed");
                    }
                    Redsem::Release => {
                        push_directive(tokens, "release");
                    }
                }
            }
            if let Some(scope_9) = self.scope.as_ref() {
                match scope_9 {
                    Scope::Cluster => {
                        push_directive(tokens, "cluster");
                    }
                    Scope::Cta => {
                        push_directive(tokens, "cta");
                    }
                    Scope::Gpu => {
                        push_directive(tokens, "gpu");
                    }
                    Scope::Sys => {
                        push_directive(tokens, "sys");
                    }
                }
            }
            if let Some(ss_10) = self.ss.as_ref() {
                match ss_10 {
                    Ss::Global => {
                        push_directive(tokens, "global");
                    }
                }
            }
            match &self.op {
                Op::Min => {
                    push_directive(tokens, "min");
                }
                Op::Max => {
                    push_directive(tokens, "max");
                }
                Op::Add => {
                    push_directive(tokens, "add");
                }
                Op::And => {
                    push_directive(tokens, "and");
                }
                Op::Xor => {
                    push_directive(tokens, "xor");
                }
                Op::Or => {
                    push_directive(tokens, "or");
                }
            }
            match &self.type_ {
                Type::B32 => {
                    push_directive(tokens, "b32");
                }
                Type::B64 => {
                    push_directive(tokens, "b64");
                }
                Type::U32 => {
                    push_directive(tokens, "u32");
                }
                Type::U64 => {
                    push_directive(tokens, "u64");
                }
                Type::S32 => {
                    push_directive(tokens, "s32");
                }
                Type::S64 => {
                    push_directive(tokens, "s64");
                }
            }
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
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::multimem_ld_reduce::section_1::*;

    impl PtxUnparser for MultimemLdReduceLdsemScopeSsOpAccPrecVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "multimem");
            push_directive(tokens, "ld_reduce");
            if let Some(ldsem_11) = self.ldsem.as_ref() {
                match ldsem_11 {
                    Ldsem::Relaxed => {
                        push_directive(tokens, "relaxed");
                    }
                    Ldsem::Acquire => {
                        push_directive(tokens, "acquire");
                    }
                }
            }
            if let Some(scope_12) = self.scope.as_ref() {
                match scope_12 {
                    Scope::Cluster => {
                        push_directive(tokens, "cluster");
                    }
                    Scope::Cta => {
                        push_directive(tokens, "cta");
                    }
                    Scope::Gpu => {
                        push_directive(tokens, "gpu");
                    }
                    Scope::Sys => {
                        push_directive(tokens, "sys");
                    }
                }
            }
            if let Some(ss_13) = self.ss.as_ref() {
                match ss_13 {
                    Ss::Global => {
                        push_directive(tokens, "global");
                    }
                }
            }
            match &self.op {
                Op::Min => {
                    push_directive(tokens, "min");
                }
                Op::Max => {
                    push_directive(tokens, "max");
                }
                Op::Add => {
                    push_directive(tokens, "add");
                }
            }
            if let Some(acc_prec_14) = self.acc_prec.as_ref() {
                match acc_prec_14 {
                    AccPrec::AccF32 => {
                        push_directive(tokens, "acc::f32");
                    }
                    AccPrec::AccF16 => {
                        push_directive(tokens, "acc::f16");
                    }
                }
            }
            if let Some(vec_15) = self.vec.as_ref() {
                match vec_15 {
                    Vec::V2 => {
                        push_directive(tokens, "v2");
                    }
                    Vec::V4 => {
                        push_directive(tokens, "v4");
                    }
                    Vec::V8 => {
                        push_directive(tokens, "v8");
                    }
                }
            }
            match &self.type_ {
                Type::Bf16x2 => {
                    push_directive(tokens, "bf16x2");
                }
                Type::E5m2x2 => {
                    push_directive(tokens, "e5m2x2");
                }
                Type::E5m2x4 => {
                    push_directive(tokens, "e5m2x4");
                }
                Type::E4m3x2 => {
                    push_directive(tokens, "e4m3x2");
                }
                Type::E4m3x4 => {
                    push_directive(tokens, "e4m3x4");
                }
                Type::F16x2 => {
                    push_directive(tokens, "f16x2");
                }
                Type::Bf16 => {
                    push_directive(tokens, "bf16");
                }
                Type::E5m2 => {
                    push_directive(tokens, "e5m2");
                }
                Type::E4m3 => {
                    push_directive(tokens, "e4m3");
                }
                Type::F16 => {
                    push_directive(tokens, "f16");
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
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for MultimemLdReduceWeakSsOpAccPrecVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "multimem");
            push_directive(tokens, "ld_reduce");
            push_directive(tokens, "weak");
            if let Some(ss_16) = self.ss.as_ref() {
                match ss_16 {
                    Ss::Global => {
                        push_directive(tokens, "global");
                    }
                }
            }
            match &self.op {
                Op::Min => {
                    push_directive(tokens, "min");
                }
                Op::Max => {
                    push_directive(tokens, "max");
                }
                Op::Add => {
                    push_directive(tokens, "add");
                }
            }
            if let Some(acc_prec_17) = self.acc_prec.as_ref() {
                match acc_prec_17 {
                    AccPrec::AccF32 => {
                        push_directive(tokens, "acc::f32");
                    }
                    AccPrec::AccF16 => {
                        push_directive(tokens, "acc::f16");
                    }
                }
            }
            if let Some(vec_18) = self.vec.as_ref() {
                match vec_18 {
                    Vec::V2 => {
                        push_directive(tokens, "v2");
                    }
                    Vec::V4 => {
                        push_directive(tokens, "v4");
                    }
                    Vec::V8 => {
                        push_directive(tokens, "v8");
                    }
                }
            }
            match &self.type_ {
                Type::Bf16x2 => {
                    push_directive(tokens, "bf16x2");
                }
                Type::E5m2x2 => {
                    push_directive(tokens, "e5m2x2");
                }
                Type::E5m2x4 => {
                    push_directive(tokens, "e5m2x4");
                }
                Type::E4m3x2 => {
                    push_directive(tokens, "e4m3x2");
                }
                Type::E4m3x4 => {
                    push_directive(tokens, "e4m3x4");
                }
                Type::F16x2 => {
                    push_directive(tokens, "f16x2");
                }
                Type::Bf16 => {
                    push_directive(tokens, "bf16");
                }
                Type::E5m2 => {
                    push_directive(tokens, "e5m2");
                }
                Type::E4m3 => {
                    push_directive(tokens, "e4m3");
                }
                Type::F16 => {
                    push_directive(tokens, "f16");
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
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for MultimemStStsemScopeSsVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "multimem");
            push_directive(tokens, "st");
            if let Some(stsem_19) = self.stsem.as_ref() {
                match stsem_19 {
                    Stsem::Relaxed => {
                        push_directive(tokens, "relaxed");
                    }
                    Stsem::Release => {
                        push_directive(tokens, "release");
                    }
                }
            }
            if let Some(scope_20) = self.scope.as_ref() {
                match scope_20 {
                    Scope::Cluster => {
                        push_directive(tokens, "cluster");
                    }
                    Scope::Cta => {
                        push_directive(tokens, "cta");
                    }
                    Scope::Gpu => {
                        push_directive(tokens, "gpu");
                    }
                    Scope::Sys => {
                        push_directive(tokens, "sys");
                    }
                }
            }
            if let Some(ss_21) = self.ss.as_ref() {
                match ss_21 {
                    Ss::Global => {
                        push_directive(tokens, "global");
                    }
                }
            }
            if let Some(vec_22) = self.vec.as_ref() {
                match vec_22 {
                    Vec::V2 => {
                        push_directive(tokens, "v2");
                    }
                    Vec::V4 => {
                        push_directive(tokens, "v4");
                    }
                    Vec::V8 => {
                        push_directive(tokens, "v8");
                    }
                }
            }
            match &self.type_ {
                Type::Bf16x2 => {
                    push_directive(tokens, "bf16x2");
                }
                Type::E5m2x2 => {
                    push_directive(tokens, "e5m2x2");
                }
                Type::E5m2x4 => {
                    push_directive(tokens, "e5m2x4");
                }
                Type::E4m3x2 => {
                    push_directive(tokens, "e4m3x2");
                }
                Type::E4m3x4 => {
                    push_directive(tokens, "e4m3x4");
                }
                Type::F16x2 => {
                    push_directive(tokens, "f16x2");
                }
                Type::Bf16 => {
                    push_directive(tokens, "bf16");
                }
                Type::E5m2 => {
                    push_directive(tokens, "e5m2");
                }
                Type::E4m3 => {
                    push_directive(tokens, "e4m3");
                }
                Type::F16 => {
                    push_directive(tokens, "f16");
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

    impl PtxUnparser for MultimemStWeakSsVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "multimem");
            push_directive(tokens, "st");
            push_directive(tokens, "weak");
            if let Some(ss_23) = self.ss.as_ref() {
                match ss_23 {
                    Ss::Global => {
                        push_directive(tokens, "global");
                    }
                }
            }
            if let Some(vec_24) = self.vec.as_ref() {
                match vec_24 {
                    Vec::V2 => {
                        push_directive(tokens, "v2");
                    }
                    Vec::V4 => {
                        push_directive(tokens, "v4");
                    }
                    Vec::V8 => {
                        push_directive(tokens, "v8");
                    }
                }
            }
            match &self.type_ {
                Type::Bf16x2 => {
                    push_directive(tokens, "bf16x2");
                }
                Type::E5m2x2 => {
                    push_directive(tokens, "e5m2x2");
                }
                Type::E5m2x4 => {
                    push_directive(tokens, "e5m2x4");
                }
                Type::E4m3x2 => {
                    push_directive(tokens, "e4m3x2");
                }
                Type::E4m3x4 => {
                    push_directive(tokens, "e4m3x4");
                }
                Type::F16x2 => {
                    push_directive(tokens, "f16x2");
                }
                Type::Bf16 => {
                    push_directive(tokens, "bf16");
                }
                Type::E5m2 => {
                    push_directive(tokens, "e5m2");
                }
                Type::E4m3 => {
                    push_directive(tokens, "e4m3");
                }
                Type::F16 => {
                    push_directive(tokens, "f16");
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

    impl PtxUnparser for MultimemRedRedsemScopeSsRedopVecRedtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "multimem");
            push_directive(tokens, "red");
            if let Some(redsem_25) = self.redsem.as_ref() {
                match redsem_25 {
                    Redsem::Relaxed => {
                        push_directive(tokens, "relaxed");
                    }
                    Redsem::Release => {
                        push_directive(tokens, "release");
                    }
                }
            }
            if let Some(scope_26) = self.scope.as_ref() {
                match scope_26 {
                    Scope::Cluster => {
                        push_directive(tokens, "cluster");
                    }
                    Scope::Cta => {
                        push_directive(tokens, "cta");
                    }
                    Scope::Gpu => {
                        push_directive(tokens, "gpu");
                    }
                    Scope::Sys => {
                        push_directive(tokens, "sys");
                    }
                }
            }
            if let Some(ss_27) = self.ss.as_ref() {
                match ss_27 {
                    Ss::Global => {
                        push_directive(tokens, "global");
                    }
                }
            }
            match &self.redop {
                Redop::Add => {
                    push_directive(tokens, "add");
                }
            }
            if let Some(vec_28) = self.vec.as_ref() {
                match vec_28 {
                    Vec::V2 => {
                        push_directive(tokens, "v2");
                    }
                    Vec::V4 => {
                        push_directive(tokens, "v4");
                    }
                    Vec::V8 => {
                        push_directive(tokens, "v8");
                    }
                }
            }
            match &self.redtype {
                Redtype::Bf16x2 => {
                    push_directive(tokens, "bf16x2");
                }
                Redtype::F16x2 => {
                    push_directive(tokens, "f16x2");
                }
                Redtype::Bf16 => {
                    push_directive(tokens, "bf16");
                }
                Redtype::F16 => {
                    push_directive(tokens, "f16");
                }
                Redtype::F32 => {
                    push_directive(tokens, "f32");
                }
                Redtype::F64 => {
                    push_directive(tokens, "f64");
                }
            }
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
}
