//! Original PTX specification:
//!
//! applypriority{.global}.level::eviction_priority  [a], size;
//! .level::eviction_priority = { .L2::evict_normal };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::applypriority::section_0::*;

    impl PtxUnparser for ApplypriorityGlobalLevelEvictionPriority {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "applypriority");
            if self.global {
                push_directive(tokens, "global");
            }
            match &self.level_eviction_priority {
                LevelEvictionPriority::L2EvictNormal => {
                    push_directive(tokens, "L2::evict_normal");
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.size.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
