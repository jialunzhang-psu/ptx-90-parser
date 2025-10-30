use crate::{lexer::PtxToken, r#type::instruction::stackrestore::*, unparser::*};

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Self::U32 => "u32",
            Self::U64 => "u64",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Stackrestore {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("stackrestore".to_string()));
        self.data_type.unparse_tokens(tokens);
        self.register.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
