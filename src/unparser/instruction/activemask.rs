use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::activemask::*},
    unparser::*,
};

impl PtxUnparser for Activemask {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("activemask".to_string()));
        tokens.push(PtxToken::Directive("b32".to_string()));
        let destination: &RegisterOperand = &self.destination;
        destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
