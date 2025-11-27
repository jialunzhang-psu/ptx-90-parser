//! Original PTX specification:
//!
//! // Range-based policy
//! createpolicy.range{.global}.level::primary_priority{.level::secondary_priority}.b64
//! cache-policy, [a], primary-size, total-size;
//! // Fraction-based policy
//! createpolicy.fractional.level::primary_priority{.level::secondary_priority}.b64
//! cache-policy{, fraction};
//! // Converting the access property from CUDA APIs
//! createpolicy.cvt.L2.b64            cache-policy, access-property;
//! .level::primary_priority =   { .L2::evict_last, .L2::evict_normal,
//! .L2::evict_first, .L2::evict_unchanged };
//! .level::secondary_priority = { .L2::evict_first, .L2::evict_unchanged };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::createpolicy::section_0::*;

    impl PtxUnparser for CreatepolicyRangeGlobalLevelPrimaryPriorityLevelSecondaryPriorityB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "createpolicy");
                    push_directive(tokens, "range");
                    if self.global {
                            push_directive(tokens, "global");
                    }
                    match &self.level_primary_priority {
                            LevelPrimaryPriority::L2EvictUnchanged => {
                                    push_directive(tokens, "L2::evict_unchanged");
                            }
                            LevelPrimaryPriority::L2EvictNormal => {
                                    push_directive(tokens, "L2::evict_normal");
                            }
                            LevelPrimaryPriority::L2EvictFirst => {
                                    push_directive(tokens, "L2::evict_first");
                            }
                            LevelPrimaryPriority::L2EvictLast => {
                                    push_directive(tokens, "L2::evict_last");
                            }
                    }
                    if let Some(level_secondary_priority_0) = self.level_secondary_priority.as_ref() {
                            match level_secondary_priority_0 {
                                    LevelSecondaryPriority::L2EvictUnchanged => {
                                            push_directive(tokens, "L2::evict_unchanged");
                                    }
                                    LevelSecondaryPriority::L2EvictFirst => {
                                            push_directive(tokens, "L2::evict_first");
                                    }
                            }
                    }
                    push_directive(tokens, "b64");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.cache_policy.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.primary_size.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.total_size.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for CreatepolicyFractionalLevelPrimaryPriorityLevelSecondaryPriorityB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "createpolicy");
                    push_directive(tokens, "fractional");
                    match &self.level_primary_priority {
                            LevelPrimaryPriority::L2EvictUnchanged => {
                                    push_directive(tokens, "L2::evict_unchanged");
                            }
                            LevelPrimaryPriority::L2EvictNormal => {
                                    push_directive(tokens, "L2::evict_normal");
                            }
                            LevelPrimaryPriority::L2EvictFirst => {
                                    push_directive(tokens, "L2::evict_first");
                            }
                            LevelPrimaryPriority::L2EvictLast => {
                                    push_directive(tokens, "L2::evict_last");
                            }
                    }
                    if let Some(level_secondary_priority_1) = self.level_secondary_priority.as_ref() {
                            match level_secondary_priority_1 {
                                    LevelSecondaryPriority::L2EvictUnchanged => {
                                            push_directive(tokens, "L2::evict_unchanged");
                                    }
                                    LevelSecondaryPriority::L2EvictFirst => {
                                            push_directive(tokens, "L2::evict_first");
                                    }
                            }
                    }
                    push_directive(tokens, "b64");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.cache_policy.unparse_tokens_mode(tokens, spaced);
            if self.fraction.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_2) = self.fraction.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_2.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for CreatepolicyCvtL2B64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "createpolicy");
                    push_directive(tokens, "cvt");
                    push_directive(tokens, "L2");
                    push_directive(tokens, "b64");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.cache_policy.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.access_property.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

