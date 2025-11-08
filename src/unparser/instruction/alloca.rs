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
            push_opcode(tokens, "alloca");
            match &self.type_ {
                Type::U32 => {
                    push_directive(tokens, "u32");
                }
                Type::U64 => {
                    push_directive(tokens, "u64");
                }
            }
            self.ptr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.size.unparse_tokens(tokens);
            if self.immalign.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_0) = self.immalign.as_ref() {
                opt_0.unparse_tokens(tokens);
            }
            tokens.push(PtxToken::Semicolon);
        }
    }
}
