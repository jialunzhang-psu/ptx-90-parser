//! Original PTX specification:
//!
//! bar.warp.sync      membermask;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::bar_warp_sync::section_0::*;

    impl PtxUnparser for BarWarpSync {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "bar");
                    push_directive(tokens, "warp");
                    push_directive(tokens, "sync");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.membermask.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

