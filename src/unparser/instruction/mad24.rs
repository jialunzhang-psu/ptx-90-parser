//! Original PTX specification:
//!
//! mad24.mode.type  d, a, b, c;
//! mad24.hi.sat.s32 d, a, b, c;
//! .mode = { .hi, .lo };
//! .type = { .u32, .s32 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mad24::section_0::*;

    impl PtxUnparser for Mad24ModeType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mad24");
                    match &self.mode {
                            Mode::Hi => {
                                    push_directive(tokens, "hi");
                            }
                            Mode::Lo => {
                                    push_directive(tokens, "lo");
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
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Mad24HiSatS32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mad24");
                    push_directive(tokens, "hi");
                    push_directive(tokens, "sat");
                    push_directive(tokens, "s32");
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

