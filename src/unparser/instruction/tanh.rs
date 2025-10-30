use crate::{lexer::PtxToken, r#type::instruction::tanh::*, unparser::*};

impl PtxUnparser for Approximation {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Approximation::Approx => tokens.push(PtxToken::Directive("approx".to_string())),
        }
    }
}

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            DataType::F32 => tokens.push(PtxToken::Directive("f32".to_string())),
        }
    }
}

impl PtxUnparser for Tanh {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("tanh".to_string()));
        self.approximation.unparse_tokens(tokens);
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
