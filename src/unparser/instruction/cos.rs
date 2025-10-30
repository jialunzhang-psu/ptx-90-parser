use crate::{lexer::PtxToken, r#type::instruction::cos, unparser::*};

impl PtxUnparser for cos::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            cos::DataType::F32 => tokens.push(PtxToken::Directive("f32".to_string())),
        }
    }
}

impl PtxUnparser for cos::Cos {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("cos".to_string()));
        tokens.push(PtxToken::Directive("approx".to_string()));
        push_flush_to_zero(tokens, self.flush_to_zero);
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
