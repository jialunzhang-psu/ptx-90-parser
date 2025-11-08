//! Original PTX specification:
//!
//! tcgen05.alloc.cta_group.sync.aligned{.shared::cta}.b32  [dst], nCols;
//! tcgen05.dealloc.cta_group.sync.aligned.b32              taddr, nCols;
//! tcgen05.relinquish_alloc_permit.cta_group.sync.aligned;
//! .cta_group = { .cta_group::1, .cta_group::2 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_alloc::section_0::*;

    impl PtxUnparser for Tcgen05AllocCtaGroupSyncAlignedSharedCtaB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
            push_directive(tokens, "alloc");
            match &self.cta_group {
                CtaGroup::CtaGroup1 => {
                    push_directive(tokens, "cta_group::1");
                }
                CtaGroup::CtaGroup2 => {
                    push_directive(tokens, "cta_group::2");
                }
            }
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            if self.shared_cta {
                push_directive(tokens, "shared::cta");
            }
            push_directive(tokens, "b32");
            self.dst.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.ncols.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05DeallocCtaGroupSyncAlignedB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
            push_directive(tokens, "dealloc");
            match &self.cta_group {
                CtaGroup::CtaGroup1 => {
                    push_directive(tokens, "cta_group::1");
                }
                CtaGroup::CtaGroup2 => {
                    push_directive(tokens, "cta_group::2");
                }
            }
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            push_directive(tokens, "b32");
            self.taddr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.ncols.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05RelinquishAllocPermitCtaGroupSyncAligned {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
            push_directive(tokens, "relinquish_alloc_permit");
            match &self.cta_group {
                CtaGroup::CtaGroup1 => {
                    push_directive(tokens, "cta_group::1");
                }
                CtaGroup::CtaGroup2 => {
                    push_directive(tokens, "cta_group::2");
                }
            }
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            tokens.push(PtxToken::Semicolon);
        }
    }
}
