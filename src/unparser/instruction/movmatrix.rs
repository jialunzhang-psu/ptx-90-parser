//! Original PTX specification:
//!
//! movmatrix.sync.aligned.shape.trans.type d, a;
//! .shape  = {.m8n8};
//! .type   = {.b16};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::movmatrix::section_0::*;

    impl PtxUnparser for MovmatrixSyncAlignedShapeTransType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "movmatrix");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            match &self.shape {
                Shape::M8n8 => {
                    push_directive(tokens, "m8n8");
                }
            }
            push_directive(tokens, "trans");
            match &self.type_ {
                Type::B16 => {
                    push_directive(tokens, "b16");
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
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
