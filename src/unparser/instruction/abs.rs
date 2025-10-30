use crate::{
    lexer::PtxToken,
    r#type::instruction::abs::{Abs, DataType as AbsDataType},
    unparser::PtxUnparser,
};

impl PtxUnparser for AbsDataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            AbsDataType::S16 => "s16",
            AbsDataType::S32 => "s32",
            AbsDataType::S64 => "s64",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Abs {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("abs".to_string()));
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
