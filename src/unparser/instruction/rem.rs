use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::rem::Rem},
    unparser::*,
};

fn push_operands(
    destination: &RegisterOperand,
    lhs: &RegisterOperand,
    rhs: &RegisterOperand,
    tokens: &mut Vec<PtxToken>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    lhs.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    rhs.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for crate::r#type::instruction::rem::DataType {
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

impl PtxUnparser for Rem {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("rem".to_string()));
        self.data_type.unparse_tokens(tokens);
        push_operands(&self.destination, &self.lhs, &self.rhs, tokens);
    }
}
