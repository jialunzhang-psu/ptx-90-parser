//! Original PTX specification:
//!
//! shfl.sync.mode.b32  d{|p}, a, b, c, membermask;
//! .mode = { .up, .down, .bfly, .idx };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::shfl_sync::section_0::*;

    impl PtxUnparser for ShflSyncModeB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "shfl");
            push_directive(tokens, "sync");
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
            self.d.unparse_tokens(tokens);
            if let Some(p_0) = self.p.as_ref() {
                tokens.push(PtxToken::Pipe);
                p_0.unparse_tokens(tokens);
            }
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.membermask.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
