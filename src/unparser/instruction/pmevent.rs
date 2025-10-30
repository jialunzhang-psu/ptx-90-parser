use crate::{lexer::PtxToken, r#type::instruction::pmevent::*, unparser::*};

impl PtxUnparser for Pmevent {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("pmevent".to_string()));
        match self {
            Pmevent::Single { event } => {
                event.unparse_tokens(tokens);
            }
            Pmevent::Mask { mask } => {
                tokens.push(PtxToken::Directive("mask".to_string()));
                mask.unparse_tokens(tokens);
            }
        }
        tokens.push(PtxToken::Semicolon);
    }
}
