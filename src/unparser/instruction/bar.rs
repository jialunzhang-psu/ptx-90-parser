//! Original PTX specification:
//!
//! barrier{.cta}.sync{.aligned}      a{, b};
//! barrier{.cta}.arrive{.aligned}    a, b;
//! barrier{.cta}.red.popc{.aligned}.u32  d, a{, b}, {!}c;
//! barrier{.cta}.red.op{.aligned}.pred   p, a{, b}, {!}c;
//! bar{.cta}.sync      a{, b};
//! bar{.cta}.arrive    a, b;
//! bar{.cta}.red.popc.u32  d, a{, b}, {!}c;
//! bar{.cta}.red.op.pred   p, a{, b}, {!}c;
//! .op = { .and, .or };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::bar::section_0::*;

    impl PtxUnparser for BarrierCtaSyncAligned {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "barrier");
            if self.cta {
                push_directive(tokens, "cta");
            }
            push_directive(tokens, "sync");
            if self.aligned {
                push_directive(tokens, "aligned");
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            if self.b.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_0) = self.b.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_0.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for BarrierCtaArriveAligned {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "barrier");
            if self.cta {
                push_directive(tokens, "cta");
            }
            push_directive(tokens, "arrive");
            if self.aligned {
                push_directive(tokens, "aligned");
            }
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

    impl PtxUnparser for BarrierCtaRedPopcAlignedU32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "barrier");
            if self.cta {
                push_directive(tokens, "cta");
            }
            push_directive(tokens, "red");
            push_directive(tokens, "popc");
            if self.aligned {
                push_directive(tokens, "aligned");
            }
            push_directive(tokens, "u32");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            if self.b.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_1) = self.b.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_1.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Comma);
            if self.c_op {
                tokens.push(PtxToken::Exclaim);
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for BarrierCtaRedOpAlignedPred {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "barrier");
            if self.cta {
                push_directive(tokens, "cta");
            }
            push_directive(tokens, "red");
            match &self.op {
                Op::And => {
                    push_directive(tokens, "and");
                }
                Op::Or => {
                    push_directive(tokens, "or");
                }
            }
            if self.aligned {
                push_directive(tokens, "aligned");
            }
            push_directive(tokens, "pred");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.p.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            if self.b.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_2) = self.b.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_2.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Comma);
            if self.c_op {
                tokens.push(PtxToken::Exclaim);
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for BarCtaSync {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "bar");
            if self.cta {
                push_directive(tokens, "cta");
            }
            push_directive(tokens, "sync");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            if self.b.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_3) = self.b.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_3.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for BarCtaArrive {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "bar");
            if self.cta {
                push_directive(tokens, "cta");
            }
            push_directive(tokens, "arrive");
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

    impl PtxUnparser for BarCtaRedPopcU32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "bar");
            if self.cta {
                push_directive(tokens, "cta");
            }
            push_directive(tokens, "red");
            push_directive(tokens, "popc");
            push_directive(tokens, "u32");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            if self.b.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_4) = self.b.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_4.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Comma);
            if self.c_op {
                tokens.push(PtxToken::Exclaim);
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for BarCtaRedOpPred {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "bar");
            if self.cta {
                push_directive(tokens, "cta");
            }
            push_directive(tokens, "red");
            match &self.op {
                Op::And => {
                    push_directive(tokens, "and");
                }
                Op::Or => {
                    push_directive(tokens, "or");
                }
            }
            push_directive(tokens, "pred");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.p.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            if self.b.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_5) = self.b.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_5.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Comma);
            if self.c_op {
                tokens.push(PtxToken::Exclaim);
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
