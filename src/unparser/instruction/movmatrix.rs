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
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

