//! Original PTX specification:
//!
//! prmt.b32{.mode}  d, a, b, c;
//! .mode = { .f4e, .b4e, .rc8, .ecl, .ecr, .rc16 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::prmt::section_0::*;

    impl PtxUnparser for PrmtB32Mode {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "prmt");
            push_directive(tokens, "b32");
            if let Some(mode_0) = self.mode.as_ref() {
                match mode_0 {
                    Mode::Rc16 => {
                        push_directive(tokens, "rc16");
                    }
                    Mode::F4e => {
                        push_directive(tokens, "f4e");
                    }
                    Mode::B4e => {
                        push_directive(tokens, "b4e");
                    }
                    Mode::Rc8 => {
                        push_directive(tokens, "rc8");
                    }
                    Mode::Ecl => {
                        push_directive(tokens, "ecl");
                    }
                    Mode::Ecr => {
                        push_directive(tokens, "ecr");
                    }
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
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
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
