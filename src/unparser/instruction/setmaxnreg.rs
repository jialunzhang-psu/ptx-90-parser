//! Original PTX specification:
//!
//! setmaxnreg.action.sync.aligned.u32 imm-reg-count;
//! .action = { .inc, .dec };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::setmaxnreg::section_0::*;

    impl PtxUnparser for SetmaxnregActionSyncAlignedU32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "setmaxnreg");
            match &self.action {
                Action::Inc => {
                    push_directive(tokens, "inc");
                }
                Action::Dec => {
                    push_directive(tokens, "dec");
                }
            }
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            push_directive(tokens, "u32");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.imm_reg_count.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
