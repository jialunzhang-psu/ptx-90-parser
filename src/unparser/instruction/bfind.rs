use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::bfind::*},
    unparser::*,
};

fn push_operands(
    destination: &RegisterOperand,
    source: &RegisterOperand,
    tokens: &mut Vec<PtxToken>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    source.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for crate::r#type::instruction::bfind::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::S32 => "s32",
            Self::S64 => "s64",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Bfind {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("bfind".to_string()));

        match self {
            Bfind::Plain {
                data_type,
                destination,
                source,
            } => {
                data_type.unparse_tokens(tokens);
                push_operands(destination, source, tokens);
            }
            Bfind::ShiftAmount {
                data_type,
                destination,
                source,
            } => {
                tokens.push(PtxToken::Directive("shiftamt".to_string()));
                data_type.unparse_tokens(tokens);
                push_operands(destination, source, tokens);
            }
        }
    }
}
