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
            push_opcode(tokens, "prmt");
                    push_directive(tokens, "b32");
                    if let Some(mode_0) = self.mode.as_ref() {
                            match mode_0 {
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
                                    Mode::Rc16 => {
                                            push_directive(tokens, "rc16");
                                    }
                            }
                    }
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

