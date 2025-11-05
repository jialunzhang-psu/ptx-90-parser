//! Original PTX specification:
//!
//! sust.b.dim{.cop}.vec.ctype.mode [a, b], c;  // unformatted
//! sust.p.dim.vec.b32.mode       [a, b], c;  // formatted
//! sust.b.adim{.cop}.vec.ctype.mode   [a, b], c;  // unformatted
//! .cop   = { .wb, .cg, .cs, .wt };                     // cache operation
//! .vec   = { none, .v2, .v4 };
//! .ctype = { .b8 , .b16, .b32, .b64 };
//! .mode  = { .trap, .clamp, .zero };
//! .dim   = { .1d, .2d, .3d };
//! .adim  = { .a1d, .a2d };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::sust::section_0::*;

    impl PtxUnparser for SustBDimCopVecCtypeMode {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "sust");
                    push_directive(tokens, "b");
                    match &self.dim {
                            Dim::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Dim::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Dim::_3d => {
                                    push_directive(tokens, "3d");
                            }
                    }
                    if let Some(cop_0) = self.cop.as_ref() {
                            match cop_0 {
                                    Cop::Wb => {
                                            push_directive(tokens, "wb");
                                    }
                                    Cop::Cg => {
                                            push_directive(tokens, "cg");
                                    }
                                    Cop::Cs => {
                                            push_directive(tokens, "cs");
                                    }
                                    Cop::Wt => {
                                            push_directive(tokens, "wt");
                                    }
                            }
                    }
                    match &self.vec {
                            Vec::None => {
                                    push_token_from_str(tokens, "none");
                            }
                            Vec::V2 => {
                                    push_directive(tokens, "v2");
                            }
                            Vec::V4 => {
                                    push_directive(tokens, "v4");
                            }
                    }
                    match &self.ctype {
                            Ctype::B8 => {
                                    push_directive(tokens, "b8");
                            }
                            Ctype::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Ctype::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Ctype::B64 => {
                                    push_directive(tokens, "b64");
                            }
                    }
                    match &self.mode {
                            Mode::Trap => {
                                    push_directive(tokens, "trap");
                            }
                            Mode::Clamp => {
                                    push_directive(tokens, "clamp");
                            }
                            Mode::Zero => {
                                    push_directive(tokens, "zero");
                            }
                    }
                    tokens.push(PtxToken::LBracket);
                    let &( ref group_1_0, ref group_1_1) = &self.a;
                    group_1_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_1_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBracket);
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for SustPDimVecB32Mode {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "sust");
                    push_directive(tokens, "p");
                    match &self.dim {
                            Dim::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Dim::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Dim::_3d => {
                                    push_directive(tokens, "3d");
                            }
                    }
                    match &self.vec {
                            Vec::None => {
                                    push_token_from_str(tokens, "none");
                            }
                            Vec::V2 => {
                                    push_directive(tokens, "v2");
                            }
                            Vec::V4 => {
                                    push_directive(tokens, "v4");
                            }
                    }
                    push_directive(tokens, "b32");
                    match &self.mode {
                            Mode::Trap => {
                                    push_directive(tokens, "trap");
                            }
                            Mode::Clamp => {
                                    push_directive(tokens, "clamp");
                            }
                            Mode::Zero => {
                                    push_directive(tokens, "zero");
                            }
                    }
                    tokens.push(PtxToken::LBracket);
                    let &( ref group_2_0, ref group_2_1) = &self.a;
                    group_2_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_2_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBracket);
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for SustBAdimCopVecCtypeMode {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "sust");
                    push_directive(tokens, "b");
                    match &self.adim {
                            Adim::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Adim::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                    }
                    if let Some(cop_3) = self.cop.as_ref() {
                            match cop_3 {
                                    Cop::Wb => {
                                            push_directive(tokens, "wb");
                                    }
                                    Cop::Cg => {
                                            push_directive(tokens, "cg");
                                    }
                                    Cop::Cs => {
                                            push_directive(tokens, "cs");
                                    }
                                    Cop::Wt => {
                                            push_directive(tokens, "wt");
                                    }
                            }
                    }
                    match &self.vec {
                            Vec::None => {
                                    push_token_from_str(tokens, "none");
                            }
                            Vec::V2 => {
                                    push_directive(tokens, "v2");
                            }
                            Vec::V4 => {
                                    push_directive(tokens, "v4");
                            }
                    }
                    match &self.ctype {
                            Ctype::B8 => {
                                    push_directive(tokens, "b8");
                            }
                            Ctype::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Ctype::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Ctype::B64 => {
                                    push_directive(tokens, "b64");
                            }
                    }
                    match &self.mode {
                            Mode::Trap => {
                                    push_directive(tokens, "trap");
                            }
                            Mode::Clamp => {
                                    push_directive(tokens, "clamp");
                            }
                            Mode::Zero => {
                                    push_directive(tokens, "zero");
                            }
                    }
                    tokens.push(PtxToken::LBracket);
                    let &( ref group_4_0, ref group_4_1) = &self.a;
                    group_4_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_4_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBracket);
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

