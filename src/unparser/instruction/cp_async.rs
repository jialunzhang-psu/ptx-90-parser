//! Original PTX specification:
//!
//! cp.async.ca.state.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], cp-size{, src-size}{, cache-policy};
//! cp.async.cg.state.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], 16{, src-size}{, cache-policy};
//! cp.async.ca.state.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], cp-size{, ignore-src}{, cache-policy} ;
//! cp.async.cg.state.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], 16{, ignore-src}{, cache-policy} ;
//! .level::cache_hint =     { .L2::cache_hint };
//! .level::prefetch_size =  { .L2::64B, .L2::128B, .L2::256B };
//! cp-size = { 4, 8, 16 };
//! .state = { .shared, .shared::cta}

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_async::section_0::*;

    impl PtxUnparser for CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cp");
            push_directive(tokens, "async");
            push_directive(tokens, "ca");
            match &self.state {
                State::SharedCta => {
                    push_directive(tokens, "shared::cta");
                }
                State::Shared => {
                    push_directive(tokens, "shared");
                }
            }
            push_directive(tokens, "global");
            if let Some(level_cache_hint_0) = self.level_cache_hint.as_ref() {
                match level_cache_hint_0 {
                    LevelCacheHint::L2CacheHint => {
                        push_directive(tokens, "L2::cache_hint");
                    }
                }
            }
            if let Some(level_prefetch_size_1) = self.level_prefetch_size.as_ref() {
                match level_prefetch_size_1 {
                    LevelPrefetchSize::L2128b => {
                        push_directive(tokens, "L2::128B");
                    }
                    LevelPrefetchSize::L2256b => {
                        push_directive(tokens, "L2::256B");
                    }
                    LevelPrefetchSize::L264b => {
                        push_directive(tokens, "L2::64B");
                    }
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.dst.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.src.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            match &self.cp_size {
                CpSize::_16 => {
                    push_token_from_str(tokens, "16");
                }
                CpSize::_4 => {
                    push_token_from_str(tokens, "4");
                }
                CpSize::_8 => {
                    push_token_from_str(tokens, "8");
                }
            }
            if self.src_size.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_2) = self.src_size.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_2.unparse_tokens_mode(tokens, spaced);
            }
            if self.cache_policy.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_3) = self.cache_policy.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_3.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cp");
            push_directive(tokens, "async");
            push_directive(tokens, "cg");
            match &self.state {
                State::SharedCta => {
                    push_directive(tokens, "shared::cta");
                }
                State::Shared => {
                    push_directive(tokens, "shared");
                }
            }
            push_directive(tokens, "global");
            if let Some(level_cache_hint_4) = self.level_cache_hint.as_ref() {
                match level_cache_hint_4 {
                    LevelCacheHint::L2CacheHint => {
                        push_directive(tokens, "L2::cache_hint");
                    }
                }
            }
            if let Some(level_prefetch_size_5) = self.level_prefetch_size.as_ref() {
                match level_prefetch_size_5 {
                    LevelPrefetchSize::L2128b => {
                        push_directive(tokens, "L2::128B");
                    }
                    LevelPrefetchSize::L2256b => {
                        push_directive(tokens, "L2::256B");
                    }
                    LevelPrefetchSize::L264b => {
                        push_directive(tokens, "L2::64B");
                    }
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.dst.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.src.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            push_token_from_str(tokens, "16");
            if self.src_size.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_6) = self.src_size.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_6.unparse_tokens_mode(tokens, spaced);
            }
            if self.cache_policy.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_7) = self.cache_policy.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_7.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cp");
            push_directive(tokens, "async");
            push_directive(tokens, "ca");
            match &self.state {
                State::SharedCta => {
                    push_directive(tokens, "shared::cta");
                }
                State::Shared => {
                    push_directive(tokens, "shared");
                }
            }
            push_directive(tokens, "global");
            if let Some(level_cache_hint_8) = self.level_cache_hint.as_ref() {
                match level_cache_hint_8 {
                    LevelCacheHint::L2CacheHint => {
                        push_directive(tokens, "L2::cache_hint");
                    }
                }
            }
            if let Some(level_prefetch_size_9) = self.level_prefetch_size.as_ref() {
                match level_prefetch_size_9 {
                    LevelPrefetchSize::L2128b => {
                        push_directive(tokens, "L2::128B");
                    }
                    LevelPrefetchSize::L2256b => {
                        push_directive(tokens, "L2::256B");
                    }
                    LevelPrefetchSize::L264b => {
                        push_directive(tokens, "L2::64B");
                    }
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.dst.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.src.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            match &self.cp_size {
                CpSize::_16 => {
                    push_token_from_str(tokens, "16");
                }
                CpSize::_4 => {
                    push_token_from_str(tokens, "4");
                }
                CpSize::_8 => {
                    push_token_from_str(tokens, "8");
                }
            }
            if self.ignore_src.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_10) = self.ignore_src.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_10.unparse_tokens_mode(tokens, spaced);
            }
            if self.cache_policy.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_11) = self.cache_policy.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_11.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cp");
            push_directive(tokens, "async");
            push_directive(tokens, "cg");
            match &self.state {
                State::SharedCta => {
                    push_directive(tokens, "shared::cta");
                }
                State::Shared => {
                    push_directive(tokens, "shared");
                }
            }
            push_directive(tokens, "global");
            if let Some(level_cache_hint_12) = self.level_cache_hint.as_ref() {
                match level_cache_hint_12 {
                    LevelCacheHint::L2CacheHint => {
                        push_directive(tokens, "L2::cache_hint");
                    }
                }
            }
            if let Some(level_prefetch_size_13) = self.level_prefetch_size.as_ref() {
                match level_prefetch_size_13 {
                    LevelPrefetchSize::L2128b => {
                        push_directive(tokens, "L2::128B");
                    }
                    LevelPrefetchSize::L2256b => {
                        push_directive(tokens, "L2::256B");
                    }
                    LevelPrefetchSize::L264b => {
                        push_directive(tokens, "L2::64B");
                    }
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.dst.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.src.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            push_token_from_str(tokens, "16");
            if self.ignore_src.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_14) = self.ignore_src.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_14.unparse_tokens_mode(tokens, spaced);
            }
            if self.cache_policy.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_15) = self.cache_policy.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_15.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
