use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::mad::*},
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

impl PtxUnparser for crate::r#type::instruction::mad::Mode {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Self::Hi => "hi",
            Self::Lo => "lo",
            Self::Wide => "wide",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for crate::r#type::instruction::mad::DataType {
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

impl PtxUnparser for crate::r#type::instruction::mad::Mad {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("mad".to_string()));

        match self {
            Self::Mode {
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
            Self::HiSatS32 {
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
