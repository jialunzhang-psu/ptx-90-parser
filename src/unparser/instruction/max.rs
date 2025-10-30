use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::max::*},
    unparser::*,
};

fn push_operands(
    destination: &RegisterOperand,
    a: &RegisterOperand,
    b: &RegisterOperand,
    tokens: &mut Vec<PtxToken>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    a.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    b.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for Relu {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        if matches!(self, Relu::Relu) {
            tokens.push(PtxToken::Directive("relu".to_string()));
        }
    }
}

impl PtxUnparser for AType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            AType::U16 => "u16",
            AType::U32 => "u32",
            AType::U64 => "u64",
            AType::U16x2 => "u16x2",
            AType::S16 => "s16",
            AType::S64 => "s64",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for BType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            BType::S16x2 => "s16x2",
            BType::S32 => "s32",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Max {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("max".to_string()));
        match self {
            Max::AType {
                data_type,
                destination,
                a,
                b,
            } => {
                data_type.unparse_tokens(tokens);
                push_operands(destination, a, b, tokens);
            }
            Max::BType {
                relu,
                data_type,
                destination,
                a,
                b,
            } => {
                relu.unparse_tokens(tokens);
                data_type.unparse_tokens(tokens);
                push_operands(destination, a, b, tokens);
            }
        }
    }
}
