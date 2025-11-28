//! Original PTX specification:
//!
//! prefetch{.space}.level                    [a];   // prefetch to data cache
//! prefetch.global.level::eviction_priority  [a];   // prefetch to data cache
//! prefetchu.L1  [a];             // prefetch to uniform cache
//! prefetch{.tensormap_space}.tensormap [a];  // prefetch the tensormap
//! .space =                    { .global, .local };
//! .level =                    { .L1, .L2 };
//! .level::eviction_priority = { .L2::evict_last, .L2::evict_normal };
//! .tensormap_space =          { .const, .param };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::prefetch::section_0::*;

    impl PtxUnparser for PrefetchSpaceLevel {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "prefetch");
            if let Some(space_0) = self.space.as_ref() {
                match space_0 {
                    Space::Global => {
                        push_directive(tokens, "global");
                    }
                    Space::Local => {
                        push_directive(tokens, "local");
                    }
                }
            }
            match &self.level {
                Level::L1 => {
                    push_directive(tokens, "L1");
                }
                Level::L2 => {
                    push_directive(tokens, "L2");
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for PrefetchGlobalLevelEvictionPriority {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "prefetch");
            push_directive(tokens, "global");
            match &self.level_eviction_priority {
                LevelEvictionPriority::L2EvictNormal => {
                    push_directive(tokens, "L2::evict_normal");
                }
                LevelEvictionPriority::L2EvictLast => {
                    push_directive(tokens, "L2::evict_last");
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for PrefetchuL1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "prefetchu");
            push_directive(tokens, "L1");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for PrefetchTensormapSpaceTensormap {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "prefetch");
            if let Some(tensormap_space_1) = self.tensormap_space.as_ref() {
                match tensormap_space_1 {
                    TensormapSpace::Const => {
                        push_directive(tokens, "const");
                    }
                    TensormapSpace::Param => {
                        push_directive(tokens, "param");
                    }
                }
            }
            push_directive(tokens, "tensormap");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
