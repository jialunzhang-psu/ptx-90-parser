use crate::{
    lexer::PtxToken,
    r#type::instruction::bfe::{Bfe, DataType},
    unparser::*,
};

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            DataType::U32 => "u32",
            DataType::U64 => "u64",
            DataType::S32 => "s32",
            DataType::S64 => "s64",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Bfe {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("bfe".to_string()));
        self.data_type.unparse_tokens(tokens);

        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.bit_position.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.field_length.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
