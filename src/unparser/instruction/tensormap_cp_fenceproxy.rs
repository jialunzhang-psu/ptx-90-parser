//! Original PTX specification:
//!
//! tensormap.cp_fenceproxy.cp_qualifiers.fence_qualifiers.sync.aligned  [dst], [src], size;
//! .cp_qualifiers    = { .global.shared::cta };
//! .fence_qualifiers = { .to_proxy::from_proxy.release.scope };
//! .to_proxy::from_proxy  = { .tensormap::generic };
//! .scope            = { .cta, .cluster, .gpu , .sys };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tensormap_cp_fenceproxy::section_0::*;

    impl PtxUnparser for TensormapCpFenceproxyCpQualifiersFenceQualifiersSyncAligned {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "tensormap");
            push_directive(tokens, "cp_fenceproxy");
            match &self.cp_qualifiers {
                CpQualifiers::GlobalSharedCta => {
                    push_directive(tokens, "global.shared::cta");
                }
            }
            match &self.fence_qualifiers {
                FenceQualifiers::ToProxyFromProxyReleaseScope(_, _, _) => {
                    push_directive(tokens, "tensormap::generic");
                    push_directive(tokens, "release");
                    push_directive(tokens, "cluster");
                    push_directive(tokens, "cta");
                    push_directive(tokens, "gpu");
                    push_directive(tokens, "sys");
                }
            }
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
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
            self.size.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
