use crate::{
    lexer::PtxToken,
    r#type::instruction::bfi::{Bfi, DataType},
    unparser::*,
};

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            DataType::B32 => "b32",
            DataType::B64 => "b64",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Bfi {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("bfi".to_string()));
        self.data_type.unparse_tokens(tokens);

        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.base.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.position.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.length.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
