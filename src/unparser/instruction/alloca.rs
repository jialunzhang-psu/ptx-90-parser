//! Original PTX specification:
//!
//! alloca.type  ptr, size{, immAlign};
//! .type = { .u32, .u64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::alloca::section_0::*;

    impl PtxUnparser for AllocaType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "alloca");
                    match &self.type_ {
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.ptr.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.size.unparse_tokens_mode(tokens, spaced);
            if self.immalign.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_0) = self.immalign.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_0.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

