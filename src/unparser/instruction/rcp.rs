//! Original PTX specification:
//!
//! rcp.approx{.ftz}.f32  d, a;  // fast, approximate reciprocal
//! rcp.rnd{.ftz}.f32     d, a;  // IEEE 754 compliant rounding
//! rcp.rnd.f64           d, a;  // IEEE 754 compliant rounding
//! .rnd = { .rn, .rz, .rm, .rp };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::rcp::section_0::*;

    impl PtxUnparser for RcpApproxFtzF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "rcp");
            push_directive(tokens, "approx");
            if self.ftz {
                push_directive(tokens, "ftz");
            }
            push_directive(tokens, "f32");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for RcpRndFtzF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "rcp");
            match &self.rnd {
                Rnd::Rn => {
                    push_directive(tokens, "rn");
                }
                Rnd::Rz => {
                    push_directive(tokens, "rz");
                }
                Rnd::Rm => {
                    push_directive(tokens, "rm");
                }
                Rnd::Rp => {
                    push_directive(tokens, "rp");
                }
            }
            if self.ftz {
                push_directive(tokens, "ftz");
            }
            push_directive(tokens, "f32");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for RcpRndF64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "rcp");
            match &self.rnd {
                Rnd::Rn => {
                    push_directive(tokens, "rn");
                }
                Rnd::Rz => {
                    push_directive(tokens, "rz");
                }
                Rnd::Rm => {
                    push_directive(tokens, "rm");
                }
                Rnd::Rp => {
                    push_directive(tokens, "rp");
                }
            }
            push_directive(tokens, "f64");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
