use crate::{
    lexer::PtxToken,
    r#type::instruction::szext::{self, *},
    unparser::*,
};

impl PtxUnparser for Mode {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            Mode::Clamp => "clamp",
            Mode::Wrap => "wrap",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for szext::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            szext::DataType::U32 => "u32",
            szext::DataType::S32 => "s32",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for Szext {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("szext".to_string()));
        self.mode.unparse_tokens(tokens);
        self.data_type.unparse_tokens(tokens);

        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
