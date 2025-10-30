use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::sad::*},
    unparser::*,
};

fn push_operands(
    destination: &RegisterOperand,
    a: &RegisterOperand,
    b: &RegisterOperand,
    c: &RegisterOperand,
    tokens: &mut Vec<PtxToken>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    a.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    b.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    c.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for crate::r#type::instruction::sad::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::S16 => "s16",
            Self::S32 => "s32",
            Self::S64 => "s64",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Sad {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("sad".to_string()));
        self.data_type.unparse_tokens(tokens);
        push_operands(&self.destination, &self.a, &self.b, &self.c, tokens);
    }
}
