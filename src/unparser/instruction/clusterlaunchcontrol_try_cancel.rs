//! Original PTX specification:
//!
//! clusterlaunchcontrol.try_cancel.async{.space}.completion_mechanism{.multicast::cluster::all}.b128 [addr], [mbar];
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .space = { .shared::cta };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::clusterlaunchcontrol_try_cancel::section_0::*;

    impl PtxUnparser
        for ClusterlaunchcontrolTryCancelAsyncSpaceCompletionMechanismMulticastClusterAllB128
    {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "clusterlaunchcontrol");
            push_directive(tokens, "try_cancel");
            push_directive(tokens, "async");
            if let Some(space_0) = self.space.as_ref() {
                match space_0 {
                    Space::SharedCta => {
                        push_directive(tokens, "shared::cta");
                    }
                }
            }
            match &self.completion_mechanism {
                CompletionMechanism::MbarrierCompleteTxBytes => {
                    push_directive(tokens, "mbarrier::complete_tx::bytes");
                }
            }
            if self.multicast_cluster_all {
                push_directive(tokens, "multicast::cluster::all");
            }
            push_directive(tokens, "b128");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.addr.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.mbar.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
