#![allow(unused_imports)]

use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::trap::*},
    unparser::*,
};

impl PtxUnparser for Trap {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("trap".to_string()));
        tokens.push(PtxToken::Semicolon);
    }
}
