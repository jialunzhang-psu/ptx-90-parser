//! Original PTX specification:
//!
//! tcgen05.wait_operation.sync.aligned;
//! .wait_operation = { .wait::ld, .wait::st }

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_wait::section_0::*;

    impl PtxUnparser for Tcgen05WaitOperationSyncAligned {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "tcgen05");
            match &self.wait_operation {
                WaitOperation::WaitLd => {
                    push_directive(tokens, "wait::ld");
                }
                WaitOperation::WaitSt => {
                    push_directive(tokens, "wait::st");
                }
            }
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
