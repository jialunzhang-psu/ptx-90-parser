use crate::{
    lexer::PtxToken,
    r#type::instruction::subc::{ConditionCode, DataType, Subc},
    unparser::*,
};

impl PtxUnparser for ConditionCode {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        if let Self::Cc = self {
            tokens.push(PtxToken::Directive("cc".to_string()));
        }
    }
}

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Self::U32 => "u32",
            Self::S32 => "s32",
            Self::U64 => "u64",
            Self::S64 => "s64",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Subc {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("subc".to_string()));
        self.condition_code.unparse_tokens(tokens);
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.minuend.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.subtrahend.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
