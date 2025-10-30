use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::shr::*},
    unparser::*,
};

fn push_operands(
    destination: &RegisterOperand,
    a: &RegisterOperand,
    b: &Operand,
    tokens: &mut Vec<PtxToken>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    a.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    b.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for crate::r#type::instruction::shr::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Self::B16 => tokens.push(PtxToken::Directive("b16".to_string())),
            Self::B32 => tokens.push(PtxToken::Directive("b32".to_string())),
            Self::B64 => tokens.push(PtxToken::Directive("b64".to_string())),
            Self::U16 => tokens.push(PtxToken::Directive("u16".to_string())),
            Self::U32 => tokens.push(PtxToken::Directive("u32".to_string())),
            Self::U64 => tokens.push(PtxToken::Directive("u64".to_string())),
            Self::S16 => tokens.push(PtxToken::Directive("s16".to_string())),
            Self::S32 => tokens.push(PtxToken::Directive("s32".to_string())),
            Self::S64 => tokens.push(PtxToken::Directive("s64".to_string())),
        }
    }
}

impl PtxUnparser for Shr {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("shr".to_string()));
        self.data_type.unparse_tokens(tokens);
        push_operands(&self.destination, &self.a, &self.b, tokens);
    }
}
