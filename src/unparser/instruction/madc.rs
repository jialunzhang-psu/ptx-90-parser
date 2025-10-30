use crate::{
    lexer::PtxToken,
    r#type::instruction::madc::{DataType, Madc, ResultPart},
    unparser::*,
};

impl PtxUnparser for ResultPart {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Self::Hi => "hi",
            Self::Lo => "lo",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
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

impl PtxUnparser for Madc {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("madc".to_string()));
        if let Some(result_part) = self.result_part {
            result_part.unparse_tokens(tokens);
        }
        if self.condition_code {
            tokens.push(PtxToken::Directive("cc".to_string()));
        }
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.multiplicand.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.multiplier.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.addend.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
