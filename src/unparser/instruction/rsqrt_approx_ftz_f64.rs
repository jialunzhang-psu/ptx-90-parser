//! Original PTX specification:
//!
//! rsqrt.approx.ftz.f64 d, a;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::rsqrt_approx_ftz_f64::section_0::*;

    impl PtxUnparser for RsqrtApproxFtzF64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "rsqrt");
            push_directive(tokens, "approx");
            push_directive(tokens, "ftz");
            push_directive(tokens, "f64");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
