use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::dp2a::*},
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

impl PtxUnparser for Mode {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Mode::Lo => "lo",
            Mode::Hi => "hi",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            DataType::U32 => "u32",
            DataType::S32 => "s32",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Dp2a {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("dp2a".to_string()));
        self.mode.unparse_tokens(tokens);
        self.atype.unparse_tokens(tokens);
        self.btype.unparse_tokens(tokens);
        push_operands(&self.destination, &self.a, &self.b, &self.c, tokens);
    }
}
