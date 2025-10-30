use crate::{
    lexer::PtxToken,
    r#type::{
        common::RegisterOperand,
        instruction::mad24::{DataType, Mad24, Mode},
    },
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
            Mode::Hi => "hi",
            Mode::Lo => "lo",
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

impl PtxUnparser for Mad24 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("mad24".to_string()));

        match self {
            Mad24::Mode {
                mode,
                data_type,
                destination,
                a,
                b,
                c,
            } => {
                mode.unparse_tokens(tokens);
                data_type.unparse_tokens(tokens);
                push_operands(destination, a, b, c, tokens);
            }
            Mad24::HiSatS32 {
                destination,
                a,
                b,
                c,
            } => {
                Mode::Hi.unparse_tokens(tokens);
                tokens.push(PtxToken::Directive("sat".to_string()));
                tokens.push(PtxToken::Directive("s32".to_string()));
                push_operands(destination, a, b, c, tokens);
            }
        }
    }
}
