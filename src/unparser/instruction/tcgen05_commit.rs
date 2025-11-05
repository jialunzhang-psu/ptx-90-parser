//! Original PTX specification:
//!
//! tcgen05.commit.cta_group.completion_mechanism{.shared::cluster}{.multicast}.b64
//! [mbar] {, ctaMask};
//! .completion_mechanism = { .mbarrier::arrive::one };
//! .cta_group            = { .cta_group::1, .cta_group::2 };
//! .multicast            = { .multicast::cluster };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_commit::section_0::*;

    impl PtxUnparser for Tcgen05CommitCtaGroupCompletionMechanismSharedClusterMulticastB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "commit");
                    match &self.cta_group {
                            CtaGroup::CtaGroup1 => {
                                    push_directive(tokens, "cta_group::1");
                            }
                            CtaGroup::CtaGroup2 => {
                                    push_directive(tokens, "cta_group::2");
                            }
                    }
                    match &self.completion_mechanism {
                            CompletionMechanism::MbarrierArriveOne => {
                                    push_directive(tokens, "mbarrier::arrive::one");
                            }
                    }
                    if self.shared_cluster {
                            push_directive(tokens, "shared::cluster");
                    }
                    if let Some(multicast_0) = self.multicast.as_ref() {
                            match multicast_0 {
                                    Multicast::MulticastCluster => {
                                            push_directive(tokens, "multicast::cluster");
                                    }
                            }
                    }
                    push_directive(tokens, "b64");
                    self.mbar.unparse_tokens(tokens);
            if self.ctamask.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_1) = self.ctamask.as_ref() {
                        opt_1.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

