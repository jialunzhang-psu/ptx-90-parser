//! Original PTX specification:
//!
//! barrier.cluster.arrive{.sem}{.aligned};
//! barrier.cluster.wait{.acquire}{.aligned};
//! .sem = {.release, .relaxed};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::barrier_cluster::section_0::*;

    impl PtxUnparser for BarrierClusterArriveSemAligned {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "barrier");
            push_directive(tokens, "cluster");
            push_directive(tokens, "arrive");
            if let Some(sem_0) = self.sem.as_ref() {
                match sem_0 {
                    Sem::Release => {
                        push_directive(tokens, "release");
                    }
                    Sem::Relaxed => {
                        push_directive(tokens, "relaxed");
                    }
                }
            }
            if self.aligned {
                push_directive(tokens, "aligned");
            }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for BarrierClusterWaitAcquireAligned {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "barrier");
            push_directive(tokens, "cluster");
            push_directive(tokens, "wait");
            if self.acquire {
                push_directive(tokens, "acquire");
            }
            if self.aligned {
                push_directive(tokens, "aligned");
            }
            tokens.push(PtxToken::Semicolon);
        }
    }
}
