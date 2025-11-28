//! Original PTX specification:
//!
//! vote.mode.pred  d, {!}a;
//! vote.ballot.b32 d, {!}a;  // 'ballot' form, returns bitmask
//! .mode = { .all, .any, .uni };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::vote::section_0::*;

    impl PtxUnparser for VoteModePred {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vote");
            match &self.mode {
                Mode::All => {
                    push_directive(tokens, "all");
                }
                Mode::Any => {
                    push_directive(tokens, "any");
                }
                Mode::Uni => {
                    push_directive(tokens, "uni");
                }
            }
            push_directive(tokens, "pred");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if self.a_op {
                tokens.push(PtxToken::Exclaim);
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for VoteBallotB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vote");
            push_directive(tokens, "ballot");
            push_directive(tokens, "b32");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if self.a_op {
                tokens.push(PtxToken::Exclaim);
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
