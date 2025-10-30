use crate::{
    lexer::PtxToken,
    r#type::instruction::sin::{DataType as SinDataType, Sin},
    unparser::*,
};

impl PtxUnparser for SinDataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            SinDataType::F32 => tokens.push(PtxToken::Directive("f32".to_string())),
        }
    }
}

impl PtxUnparser for Sin {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("sin".to_string()));
        tokens.push(PtxToken::Directive("approx".to_string()));
        push_flush_to_zero(tokens, self.flush_to_zero);
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
