//! Original PTX specification:
//!
//! ldmatrix.sync.aligned.shape.num{.trans}{.ss}.type r, [p];
//! ldmatrix.sync.aligned.m8n16.num{.ss}.dst_fmt.src_fmt        r, [p];
//! ldmatrix.sync.aligned.m16n16.num.trans{.ss}.dst_fmt.src_fmt r, [p];
//! .shape   = {.m8n8, .m16n16};
//! .num     = {.x1, .x2, .x4};
//! .ss      = {.shared, .shared::cta};
//! .type    = {.b16, .b8};
//! .dst_fmt = { .b8x16 };
//! .src_fmt = { .b6x16_p32, .b4x16_p64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::ldmatrix::section_0::*;

    impl PtxUnparser for LdmatrixSyncAlignedShapeNumTransSsType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "ldmatrix");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.shape {
                            Shape::M16n16 => {
                                    push_directive(tokens, "m16n16");
                            }
                            Shape::M8n8 => {
                                    push_directive(tokens, "m8n8");
                            }
                    }
                    match &self.num {
                            Num::X1 => {
                                    push_directive(tokens, "x1");
                            }
                            Num::X2 => {
                                    push_directive(tokens, "x2");
                            }
                            Num::X4 => {
                                    push_directive(tokens, "x4");
                            }
                    }
                    if self.trans {
                            push_directive(tokens, "trans");
                    }
                    if let Some(ss_0) = self.ss.as_ref() {
                            match ss_0 {
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    match &self.type_ {
                            Type::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Type::B8 => {
                                    push_directive(tokens, "b8");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.r.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.p.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for LdmatrixSyncAlignedM8n16NumSsDstFmtSrcFmt {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "ldmatrix");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    push_directive(tokens, "m8n16");
                    match &self.num {
                            Num::X1 => {
                                    push_directive(tokens, "x1");
                            }
                            Num::X2 => {
                                    push_directive(tokens, "x2");
                            }
                            Num::X4 => {
                                    push_directive(tokens, "x4");
                            }
                    }
                    if let Some(ss_1) = self.ss.as_ref() {
                            match ss_1 {
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    match &self.dst_fmt {
                            DstFmt::B8x16 => {
                                    push_directive(tokens, "b8x16");
                            }
                    }
                    match &self.src_fmt {
                            SrcFmt::B6x16P32 => {
                                    push_directive(tokens, "b6x16_p32");
                            }
                            SrcFmt::B4x16P64 => {
                                    push_directive(tokens, "b4x16_p64");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.r.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.p.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for LdmatrixSyncAlignedM16n16NumTransSsDstFmtSrcFmt {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "ldmatrix");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    push_directive(tokens, "m16n16");
                    match &self.num {
                            Num::X1 => {
                                    push_directive(tokens, "x1");
                            }
                            Num::X2 => {
                                    push_directive(tokens, "x2");
                            }
                            Num::X4 => {
                                    push_directive(tokens, "x4");
                            }
                    }
                    push_directive(tokens, "trans");
                    if let Some(ss_2) = self.ss.as_ref() {
                            match ss_2 {
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    match &self.dst_fmt {
                            DstFmt::B8x16 => {
                                    push_directive(tokens, "b8x16");
                            }
                    }
                    match &self.src_fmt {
                            SrcFmt::B6x16P32 => {
                                    push_directive(tokens, "b6x16_p32");
                            }
                            SrcFmt::B4x16P64 => {
                                    push_directive(tokens, "b4x16_p64");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.r.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.p.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

