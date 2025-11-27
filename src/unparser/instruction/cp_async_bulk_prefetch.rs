//! Original PTX specification:
//!
//! cp.async.bulk.prefetch.L2.src{.level::cache_hint}   [srcMem], size {, cache-policy};
//! .src =                { .global };
//! .level::cache_hint =  { .L2::cache_hint };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk_prefetch::section_0::*;

    impl PtxUnparser for CpAsyncBulkPrefetchL2SrcLevelCacheHint {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cp");
                    push_directive(tokens, "async");
                    push_directive(tokens, "bulk");
                    push_directive(tokens, "prefetch");
                    push_directive(tokens, "L2");
                    match &self.src {
                            Src::Global => {
                                    push_directive(tokens, "global");
                            }
                    }
                    if let Some(level_cache_hint_0) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_0 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.srcmem.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.size.unparse_tokens_mode(tokens, spaced);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_1) = self.cache_policy.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_1.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

