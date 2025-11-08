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
            push_opcode(tokens, "bar");
            push_directive(tokens, "warp");
            push_directive(tokens, "sync");
            self.membermask.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
