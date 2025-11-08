//! Original PTX specification:
//!
//! st.bulk{.weak}{.shared::cta}  [a], size, initval; // initval must be zero

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::st_bulk::section_0::*;

    impl PtxUnparser for StBulkWeakSharedCta {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "st");
            push_directive(tokens, "bulk");
            if self.weak {
                push_directive(tokens, "weak");
            }
            if self.shared_cta {
                push_directive(tokens, "shared::cta");
            }
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.size.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.initval.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
