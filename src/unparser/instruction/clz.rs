use crate::{
    lexer::PtxToken,
    r#type::instruction::clz::{Clz, DataType},
    unparser::*,
};

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Self::B32 => tokens.push(PtxToken::Directive("b32".into())),
            Self::B64 => tokens.push(PtxToken::Directive("b64".into())),
        }
    }
}

impl PtxUnparser for Clz {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("clz".into()));
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
