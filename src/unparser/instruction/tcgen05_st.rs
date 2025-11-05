//! Original PTX specification:
//!
//! tcgen05.st.sync.aligned.shape1.num{.unpack}.b32    [taddr], r;
//! tcgen05.st.sync.aligned.shape2.num{.unpack}.b32    [taddr], immHalfSplitoff, r;
//! .shape1 = { .16x64b, .16x128b, .16x256b, .32x32b };
//! .shape2 = { .16x32bx2 };
//! .num    = { .x1, .x2, .x4, .x8, .x16, .x32, .x64, .x128 };
//! .unpack = { .unpack::16b };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_st::section_0::*;

    impl PtxUnparser for Tcgen05StSyncAlignedShape1NumUnpackB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "st");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.shape1 {
                            Shape1::_16x64b => {
                                    push_directive(tokens, "16x64b");
                            }
                            Shape1::_16x128b => {
                                    push_directive(tokens, "16x128b");
                            }
                            Shape1::_16x256b => {
                                    push_directive(tokens, "16x256b");
                            }
                            Shape1::_32x32b => {
                                    push_directive(tokens, "32x32b");
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
                            Num::X8 => {
                                    push_directive(tokens, "x8");
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
                            Num::X128 => {
                                    push_directive(tokens, "x128");
                            }
                    }
                    if let Some(unpack_0) = self.unpack.as_ref() {
                            match unpack_0 {
                                    Unpack::Unpack16b => {
                                            push_directive(tokens, "unpack::16b");
                                    }
                            }
                    }
                    push_directive(tokens, "b32");
                    self.taddr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05StSyncAlignedShape2NumUnpackB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "st");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.shape2 {
                            Shape2::_16x32bx2 => {
                                    push_directive(tokens, "16x32bx2");
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
                            Num::X8 => {
                                    push_directive(tokens, "x8");
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
                            Num::X128 => {
                                    push_directive(tokens, "x128");
                            }
                    }
                    if let Some(unpack_1) = self.unpack.as_ref() {
                            match unpack_1 {
                                    Unpack::Unpack16b => {
                                            push_directive(tokens, "unpack::16b");
                                    }
                            }
                    }
                    push_directive(tokens, "b32");
                    self.taddr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.immhalfsplitoff.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

