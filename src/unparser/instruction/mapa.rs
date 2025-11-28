//! Original PTX specification:
//!
//! mapa{.space}.type          d, a, b;
//! // Maps shared memory address in register a into CTA b.
//! // mapa.shared::cluster.type  d, a, b;
//! // Maps shared memory variable into CTA b.
//! // mapa.shared::cluster.type  d, sh, b;
//! // Maps shared memory variable into CTA b.
//! // mapa.shared::cluster.type  d, sh + imm, b;
//! // Maps generic address in register a into CTA b.
//! // mapa.type                  d, a, b;
//! .space = { .shared::cluster };
//! .type  = { .u32, .u64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mapa::section_0::*;

    impl PtxUnparser for MapaSpaceType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "mapa");
            if let Some(space_0) = self.space.as_ref() {
                match space_0 {
                    Space::SharedCluster => {
                        push_directive(tokens, "shared::cluster");
                    }
                }
            }
            match &self.type_ {
                Type::U32 => {
                    push_directive(tokens, "u32");
                }
                Type::U64 => {
                    push_directive(tokens, "u64");
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
