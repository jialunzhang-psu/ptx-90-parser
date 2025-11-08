//! Original PTX specification:
//!
//! match.any.sync.type  d, a, membermask;
//! match.all.sync.type  d{|p}, a, membermask;
//! .type = { .b32, .b64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::match_sync::section_0::*;

    impl PtxUnparser for MatchAnySyncType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "match");
            push_directive(tokens, "any");
            push_directive(tokens, "sync");
            match &self.type_ {
                Type::B32 => {
                    push_directive(tokens, "b32");
                }
                Type::B64 => {
                    push_directive(tokens, "b64");
                }
            }
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.membermask.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MatchAllSyncType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "match");
            push_directive(tokens, "all");
            push_directive(tokens, "sync");
            match &self.type_ {
                Type::B32 => {
                    push_directive(tokens, "b32");
                }
                Type::B64 => {
                    push_directive(tokens, "b64");
                }
            }
            self.d.unparse_tokens(tokens);
            if let Some(p_0) = self.p.as_ref() {
                tokens.push(PtxToken::Pipe);
                p_0.unparse_tokens(tokens);
            }
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.membermask.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
