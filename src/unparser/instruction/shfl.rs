//! Original PTX specification:
//!
//! shfl.mode.b32  d{|p}, a, b, c;
//! .mode = { .up, .down, .bfly, .idx };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::shfl::section_0::*;

    impl PtxUnparser for ShflModeB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "shfl");
                    match &self.mode {
                            Mode::Down => {
                                    push_directive(tokens, "down");
                            }
                            Mode::Bfly => {
                                    push_directive(tokens, "bfly");
                            }
                            Mode::Idx => {
                                    push_directive(tokens, "idx");
                            }
                            Mode::Up => {
                                    push_directive(tokens, "up");
                            }
                    }
                    push_directive(tokens, "b32");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(p_0) = self.p.as_ref() {
                        tokens.push(PtxToken::Pipe);
                        p_0.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

