//! Original PTX specification:
//!
//! shf.l.mode.b32  d, a, b, c;  // left shift
//! shf.r.mode.b32  d, a, b, c;  // right shift
//! .mode = { .clamp, .wrap };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::shf::section_0::*;

    impl PtxUnparser for ShfLModeB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "shf");
                    push_directive(tokens, "l");
                    match &self.mode {
                            Mode::Clamp => {
                                    push_directive(tokens, "clamp");
                            }
                            Mode::Wrap => {
                                    push_directive(tokens, "wrap");
                            }
                    }
                    push_directive(tokens, "b32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for ShfRModeB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "shf");
                    push_directive(tokens, "r");
                    match &self.mode {
                            Mode::Clamp => {
                                    push_directive(tokens, "clamp");
                            }
                            Mode::Wrap => {
                                    push_directive(tokens, "wrap");
                            }
                    }
                    push_directive(tokens, "b32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

