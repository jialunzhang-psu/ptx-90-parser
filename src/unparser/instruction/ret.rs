#![allow(unused_imports)]

use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::ret::*},
    unparser::*,
};

impl PtxUnparser for Ret {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "ret");
        if matches!(self, Ret::Uniform) {
            tokens.push(PtxToken::Directive("uni".to_string()));
        }
        tokens.push(PtxToken::Semicolon);
    }
}
