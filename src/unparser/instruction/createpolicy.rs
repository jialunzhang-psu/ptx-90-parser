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
            push_opcode(tokens, "createpolicy");
                    push_directive(tokens, "range");
                    if self.global {
                            push_directive(tokens, "global");
                    }
                    match &self.level_primary_priority {
                            LevelPrimaryPriority::L2EvictLast => {
                                    push_directive(tokens, "L2::evict_last");
                            }
                            LevelPrimaryPriority::L2EvictNormal => {
                                    push_directive(tokens, "L2::evict_normal");
                            }
                            LevelPrimaryPriority::L2EvictFirst => {
                                    push_directive(tokens, "L2::evict_first");
                            }
                            LevelPrimaryPriority::L2EvictUnchanged => {
                                    push_directive(tokens, "L2::evict_unchanged");
                            }
                    }
                    if let Some(level_secondary_priority_0) = self.level_secondary_priority.as_ref() {
                            match level_secondary_priority_0 {
                                    LevelSecondaryPriority::L2EvictFirst => {
                                            push_directive(tokens, "L2::evict_first");
                                    }
                                    LevelSecondaryPriority::L2EvictUnchanged => {
                                            push_directive(tokens, "L2::evict_unchanged");
                                    }
                            }
                    }
                    push_directive(tokens, "b64");
                    self.cache_policy.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.primary_size.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.total_size.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CreatepolicyFractionalLevelPrimaryPriorityLevelSecondaryPriorityB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "createpolicy");
                    push_directive(tokens, "fractional");
                    match &self.level_primary_priority {
                            LevelPrimaryPriority::L2EvictLast => {
                                    push_directive(tokens, "L2::evict_last");
                            }
                            LevelPrimaryPriority::L2EvictNormal => {
                                    push_directive(tokens, "L2::evict_normal");
                            }
                            LevelPrimaryPriority::L2EvictFirst => {
                                    push_directive(tokens, "L2::evict_first");
                            }
                            LevelPrimaryPriority::L2EvictUnchanged => {
                                    push_directive(tokens, "L2::evict_unchanged");
                            }
                    }
                    if let Some(level_secondary_priority_1) = self.level_secondary_priority.as_ref() {
                            match level_secondary_priority_1 {
                                    LevelSecondaryPriority::L2EvictFirst => {
                                            push_directive(tokens, "L2::evict_first");
                                    }
                                    LevelSecondaryPriority::L2EvictUnchanged => {
                                            push_directive(tokens, "L2::evict_unchanged");
                                    }
                            }
                    }
                    push_directive(tokens, "b64");
                    self.cache_policy.unparse_tokens(tokens);
            if self.fraction.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_2) = self.fraction.as_ref() {
                        opt_2.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CreatepolicyCvtL2B64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "createpolicy");
                    push_directive(tokens, "cvt");
                    push_directive(tokens, "L2");
                    push_directive(tokens, "b64");
                    self.cache_policy.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.access_property.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

