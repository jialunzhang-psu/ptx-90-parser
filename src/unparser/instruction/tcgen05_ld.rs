//! Original PTX specification:
//!
//! // Base load instruction:
//! tcgen05.ld.sync.aligned.shape1.num{.pack}.b32    r, [taddr];
//! tcgen05.ld.sync.aligned.shape2.num{.pack}.b32    r, [taddr], immHalfSplitoff;
//! .shape1 = { .16x64b, .16x128b, .16x256b, .32x32b };
//! .shape2 = { .16x32bx2 };
//! .num    = { .x1, .x2, .x4, .x8, .x16, .x32, .x64, .x128 };
//! .pack   = { .pack::16b };
//! // Floating point type load along with reduction :
//! tcgen05.ld.red.sync.aligned.shape3.num.redOp{.abs}{.NaN}.f32 r, redval, [taddr];
//! tcgen05.ld.red.sync.aligned.shape4.num.redOp{.abs}{.NaN}.f32 r, redval, [taddr], immHalfSplitoff;
//! // Integer type load along with reduction :
//! tcgen05.ld.red.sync.aligned.shape3.num.redOp.type r, redval, [taddr];
//! tcgen05.ld.red.sync.aligned.shape4.num.redOp.type r, redval, [taddr], immHalfSplitoff;
//! .shape3 = { .32x32b   };
//! .shape4 = { .16x32bx2 };
//! .redOp  = { .min, .max };
//! .type   = { .u32, .s32 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_ld::section_0::*;

    impl PtxUnparser for Tcgen05LdSyncAlignedShape1NumPackB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
            push_directive(tokens, "ld");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            match &self.shape1 {
                Shape1::_16x128b => {
                    push_directive(tokens, "16x128b");
                }
                Shape1::_16x256b => {
                    push_directive(tokens, "16x256b");
                }
                Shape1::_16x64b => {
                    push_directive(tokens, "16x64b");
                }
                Shape1::_32x32b => {
                    push_directive(tokens, "32x32b");
                }
            }
            match &self.num {
                Num::X128 => {
                    push_directive(tokens, "x128");
                }
                Num::X16 => {
                    push_directive(tokens, "x16");
                }
                Num::X32 => {
                    push_directive(tokens, "x32");
                }
                Num::X64 => {
                    push_directive(tokens, "x64");
                }
                Num::X1 => {
                    push_directive(tokens, "x1");
                }
                Num::X2 => {
                    push_directive(tokens, "x2");
                }
                Num::X4 => {
                    push_directive(tokens, "x4");
                }
                Num::X8 => {
                    push_directive(tokens, "x8");
                }
            }
            if let Some(pack_0) = self.pack.as_ref() {
                match pack_0 {
                    Pack::Pack16b => {
                        push_directive(tokens, "pack::16b");
                    }
                }
            }
            push_directive(tokens, "b32");
            self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.taddr.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05LdSyncAlignedShape2NumPackB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
            push_directive(tokens, "ld");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            match &self.shape2 {
                Shape2::_16x32bx2 => {
                    push_directive(tokens, "16x32bx2");
                }
            }
            match &self.num {
                Num::X128 => {
                    push_directive(tokens, "x128");
                }
                Num::X16 => {
                    push_directive(tokens, "x16");
                }
                Num::X32 => {
                    push_directive(tokens, "x32");
                }
                Num::X64 => {
                    push_directive(tokens, "x64");
                }
                Num::X1 => {
                    push_directive(tokens, "x1");
                }
                Num::X2 => {
                    push_directive(tokens, "x2");
                }
                Num::X4 => {
                    push_directive(tokens, "x4");
                }
                Num::X8 => {
                    push_directive(tokens, "x8");
                }
            }
            if let Some(pack_1) = self.pack.as_ref() {
                match pack_1 {
                    Pack::Pack16b => {
                        push_directive(tokens, "pack::16b");
                    }
                }
            }
            push_directive(tokens, "b32");
            self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.taddr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.immhalfsplitoff.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05LdRedSyncAlignedShape3NumRedopAbsNanF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
            push_directive(tokens, "ld");
            push_directive(tokens, "red");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            match &self.shape3 {
                Shape3::_32x32b => {
                    push_directive(tokens, "32x32b");
                }
            }
            match &self.num {
                Num::X128 => {
                    push_directive(tokens, "x128");
                }
                Num::X16 => {
                    push_directive(tokens, "x16");
                }
                Num::X32 => {
                    push_directive(tokens, "x32");
                }
                Num::X64 => {
                    push_directive(tokens, "x64");
                }
                Num::X1 => {
                    push_directive(tokens, "x1");
                }
                Num::X2 => {
                    push_directive(tokens, "x2");
                }
                Num::X4 => {
                    push_directive(tokens, "x4");
                }
                Num::X8 => {
                    push_directive(tokens, "x8");
                }
            }
            match &self.redop {
                Redop::Min => {
                    push_directive(tokens, "min");
                }
                Redop::Max => {
                    push_directive(tokens, "max");
                }
            }
            if self.abs {
                push_directive(tokens, "abs");
            }
            if self.nan {
                push_directive(tokens, "NaN");
            }
            push_directive(tokens, "f32");
            self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.redval.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.taddr.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05LdRedSyncAlignedShape4NumRedopAbsNanF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
            push_directive(tokens, "ld");
            push_directive(tokens, "red");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            match &self.shape4 {
                Shape4::_16x32bx2 => {
                    push_directive(tokens, "16x32bx2");
                }
            }
            match &self.num {
                Num::X128 => {
                    push_directive(tokens, "x128");
                }
                Num::X16 => {
                    push_directive(tokens, "x16");
                }
                Num::X32 => {
                    push_directive(tokens, "x32");
                }
                Num::X64 => {
                    push_directive(tokens, "x64");
                }
                Num::X1 => {
                    push_directive(tokens, "x1");
                }
                Num::X2 => {
                    push_directive(tokens, "x2");
                }
                Num::X4 => {
                    push_directive(tokens, "x4");
                }
                Num::X8 => {
                    push_directive(tokens, "x8");
                }
            }
            match &self.redop {
                Redop::Min => {
                    push_directive(tokens, "min");
                }
                Redop::Max => {
                    push_directive(tokens, "max");
                }
            }
            if self.abs {
                push_directive(tokens, "abs");
            }
            if self.nan {
                push_directive(tokens, "NaN");
            }
            push_directive(tokens, "f32");
            self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.redval.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.taddr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.immhalfsplitoff.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05LdRedSyncAlignedShape3NumRedopType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
            push_directive(tokens, "ld");
            push_directive(tokens, "red");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            match &self.shape3 {
                Shape3::_32x32b => {
                    push_directive(tokens, "32x32b");
                }
            }
            match &self.num {
                Num::X128 => {
                    push_directive(tokens, "x128");
                }
                Num::X16 => {
                    push_directive(tokens, "x16");
                }
                Num::X32 => {
                    push_directive(tokens, "x32");
                }
                Num::X64 => {
                    push_directive(tokens, "x64");
                }
                Num::X1 => {
                    push_directive(tokens, "x1");
                }
                Num::X2 => {
                    push_directive(tokens, "x2");
                }
                Num::X4 => {
                    push_directive(tokens, "x4");
                }
                Num::X8 => {
                    push_directive(tokens, "x8");
                }
            }
            match &self.redop {
                Redop::Min => {
                    push_directive(tokens, "min");
                }
                Redop::Max => {
                    push_directive(tokens, "max");
                }
            }
            match &self.type_ {
                Type::U32 => {
                    push_directive(tokens, "u32");
                }
                Type::S32 => {
                    push_directive(tokens, "s32");
                }
            }
            self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.redval.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.taddr.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05LdRedSyncAlignedShape4NumRedopType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
            push_directive(tokens, "ld");
            push_directive(tokens, "red");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            match &self.shape4 {
                Shape4::_16x32bx2 => {
                    push_directive(tokens, "16x32bx2");
                }
            }
            match &self.num {
                Num::X128 => {
                    push_directive(tokens, "x128");
                }
                Num::X16 => {
                    push_directive(tokens, "x16");
                }
                Num::X32 => {
                    push_directive(tokens, "x32");
                }
                Num::X64 => {
                    push_directive(tokens, "x64");
                }
                Num::X1 => {
                    push_directive(tokens, "x1");
                }
                Num::X2 => {
                    push_directive(tokens, "x2");
                }
                Num::X4 => {
                    push_directive(tokens, "x4");
                }
                Num::X8 => {
                    push_directive(tokens, "x8");
                }
            }
            match &self.redop {
                Redop::Min => {
                    push_directive(tokens, "min");
                }
                Redop::Max => {
                    push_directive(tokens, "max");
                }
            }
            match &self.type_ {
                Type::U32 => {
                    push_directive(tokens, "u32");
                }
                Type::S32 => {
                    push_directive(tokens, "s32");
                }
            }
            self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.redval.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.taddr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.immhalfsplitoff.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
