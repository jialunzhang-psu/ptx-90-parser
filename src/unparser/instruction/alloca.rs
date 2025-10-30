use crate::{lexer::PtxToken, r#type::instruction::alloca::*, unparser::*};

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Self::U32 => "u32",
            Self::U64 => "u64",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Alloca {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("alloca".to_string()));
        match self {
            Self::Default {
                data_type,
                pointer,
                size,
            } => {
                data_type.unparse_tokens(tokens);
                pointer.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                size.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
            Self::Aligned {
                data_type,
                pointer,
                size,
                alignment,
            } => {
                data_type.unparse_tokens(tokens);
                pointer.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                size.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                alignment.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
        }
    }
}
