//! Original PTX specification:
//!
//! // global -> shared::cta
//! cp.async.bulk.dst.src.completion_mechanism{.level::cache_hint} [dstMem], [srcMem], size, [mbar] {, cache-policy};
//! .dst =                  { .shared::cta };
//! .src =                  { .global };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .level::cache_hint =    { .L2::cache_hint };
//! ----------------------------------------------------------------
//! // global -> shared::cluster;
//! cp.async.bulk.dst.src.completion_mechanism{.multicast}{.level::cache_hint} [dstMem], [srcMem], size, [mbar] {, ctaMask} {, cache-policy};
//! .dst =                  { .shared::cluster };
//! .src =                  { .global };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .level::cache_hint =    { .L2::cache_hint };
//! .multicast =            { .multicast::cluster  };
//! ----------------------------------------------------------------
//! // shared::cta -> shared::cluster
//! cp.async.bulk.dst.src.completion_mechanism [dstMem], [srcMem], size, [mbar];
//! .dst =                  { .shared::cluster };
//! .src =                  { .shared::cta };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ----------------------------------------------------------------
//! // shared::cta -> global
//! cp.async.bulk.dst.src.completion_mechanism{.level::cache_hint}{.cp_mask} [dstMem], [srcMem], size {, cache-policy} {, byteMask};
//! .dst =                  { .global };
//! .src =                  { .shared::cta };
//! .completion_mechanism = { .bulk_group };
//! .level::cache_hint =    { .L2::cache_hint };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk::section_0::*;

    impl PtxUnparser for CpAsyncBulkDstSrcCompletionMechanismLevelCacheHint {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cp");
                    push_directive(tokens, "async");
                    push_directive(tokens, "bulk");
                    match &self.dst {
                            Dst::SharedCta => {
                                    push_directive(tokens, "shared::cta");
                            }
                    }
                    match &self.src {
                            Src::Global => {
                                    push_directive(tokens, "global");
                            }
                    }
                    match &self.completion_mechanism {
                            CompletionMechanism::MbarrierCompleteTxBytes => {
                                    push_directive(tokens, "mbarrier::complete_tx::bytes");
                            }
                    }
                    if let Some(level_cache_hint_0) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_0 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    self.dstmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.srcmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.size.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.mbar.unparse_tokens(tokens);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_1) = self.cache_policy.as_ref() {
                        opt_1.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk::section_1::*;

    impl PtxUnparser for CpAsyncBulkDstSrcCompletionMechanismMulticastLevelCacheHint {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cp");
                    push_directive(tokens, "async");
                    push_directive(tokens, "bulk");
                    match &self.dst {
                            Dst::SharedCluster => {
                                    push_directive(tokens, "shared::cluster");
                            }
                    }
                    match &self.src {
                            Src::Global => {
                                    push_directive(tokens, "global");
                            }
                    }
                    match &self.completion_mechanism {
                            CompletionMechanism::MbarrierCompleteTxBytes => {
                                    push_directive(tokens, "mbarrier::complete_tx::bytes");
                            }
                    }
                    if let Some(multicast_2) = self.multicast.as_ref() {
                            match multicast_2 {
                                    Multicast::MulticastCluster => {
                                            push_directive(tokens, "multicast::cluster");
                                    }
                            }
                    }
                    if let Some(level_cache_hint_3) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_3 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    self.dstmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.srcmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.size.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.mbar.unparse_tokens(tokens);
            if self.ctamask.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_4) = self.ctamask.as_ref() {
                        opt_4.unparse_tokens(tokens);
                    }
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_5) = self.cache_policy.as_ref() {
                        opt_5.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk::section_2::*;

    impl PtxUnparser for CpAsyncBulkDstSrcCompletionMechanism {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cp");
                    push_directive(tokens, "async");
                    push_directive(tokens, "bulk");
                    match &self.dst {
                            Dst::SharedCluster => {
                                    push_directive(tokens, "shared::cluster");
                            }
                    }
                    match &self.src {
                            Src::SharedCta => {
                                    push_directive(tokens, "shared::cta");
                            }
                    }
                    match &self.completion_mechanism {
                            CompletionMechanism::MbarrierCompleteTxBytes => {
                                    push_directive(tokens, "mbarrier::complete_tx::bytes");
                            }
                    }
                    self.dstmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.srcmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.size.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.mbar.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk::section_3::*;

    impl PtxUnparser for CpAsyncBulkDstSrcCompletionMechanismLevelCacheHintCpMask {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cp");
                    push_directive(tokens, "async");
                    push_directive(tokens, "bulk");
                    match &self.dst {
                            Dst::Global => {
                                    push_directive(tokens, "global");
                            }
                    }
                    match &self.src {
                            Src::SharedCta => {
                                    push_directive(tokens, "shared::cta");
                            }
                    }
                    match &self.completion_mechanism {
                            CompletionMechanism::BulkGroup => {
                                    push_directive(tokens, "bulk_group");
                            }
                    }
                    if let Some(level_cache_hint_6) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_6 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    if self.cp_mask {
                            push_directive(tokens, "cp_mask");
                    }
                    self.dstmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.srcmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.size.unparse_tokens(tokens);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_7) = self.cache_policy.as_ref() {
                        opt_7.unparse_tokens(tokens);
                    }
            if self.bytemask.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_8) = self.bytemask.as_ref() {
                        opt_8.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

