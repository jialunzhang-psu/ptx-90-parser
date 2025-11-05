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
            push_opcode(tokens, "applypriority");
                    if self.global {
                            push_directive(tokens, "global");
                    }
                    match &self.level_eviction_priority {
                            LevelEvictionPriority::L2EvictNormal => {
                                    push_directive(tokens, "L2::evict_normal");
                            }
                    }
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.size.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

