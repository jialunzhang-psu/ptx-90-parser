//! Original PTX specification:
//!
//! lop3.b32 d, a, b, c, immLut;
//! lop3.BoolOp.b32 d|p, a, b, c, immLut, q;
//! .BoolOp   = { .or , .and };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::lop3::section_0::*;

    impl PtxUnparser for Lop3B32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "lop3");
            push_directive(tokens, "b32");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.immlut.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Lop3BoolopB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "lop3");
            match &self.boolop {
                Boolop::And => {
                    push_directive(tokens, "and");
                }
                Boolop::Or => {
                    push_directive(tokens, "or");
                }
            }
            push_directive(tokens, "b32");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Pipe);
            self.p.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.immlut.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.q.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
