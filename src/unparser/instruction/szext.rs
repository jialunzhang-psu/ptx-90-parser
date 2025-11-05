//! Original PTX specification:
//!
//! szext.mode.type  d, a, b;
//! .mode = { .clamp, .wrap };
//! .type = { .u32, .s32 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::szext::section_0::*;

    impl PtxUnparser for SzextModeType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "szext");
                    match &self.mode {
                            Mode::Clamp => {
                                    push_directive(tokens, "clamp");
                            }
                            Mode::Wrap => {
                                    push_directive(tokens, "wrap");
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
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

