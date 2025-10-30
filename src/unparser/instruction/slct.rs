use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::slct::*},
    unparser::*,
};

fn push_operands(
    destination: &RegisterOperand,
    on_true: &RegisterOperand,
    on_false: &RegisterOperand,
    selector: &RegisterOperand,
    tokens: &mut Vec<PtxToken>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    on_true.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    on_false.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    selector.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for crate::r#type::instruction::slct::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Self::B16 => "b16",
            Self::B32 => "b32",
            Self::B64 => "b64",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::S16 => "s16",
            Self::S32 => "s32",
            Self::S64 => "s64",
            Self::F32 => "f32",
            Self::F64 => "f64",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Slct {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "slct");
        match self {
            Slct::S32 {
                data_type,
                destination,
                on_true,
                on_false,
                selector,
            } => {
                data_type.unparse_tokens(tokens);
                tokens.push(PtxToken::Directive("s32".to_string()));
                push_operands(destination, on_true, on_false, selector, tokens);
            }
            Slct::F32 {
                flush_to_zero,
                data_type,
                destination,
                on_true,
                on_false,
                selector,
            } => {
                if *flush_to_zero {
                    tokens.push(PtxToken::Directive("ftz".to_string()));
                }
                data_type.unparse_tokens(tokens);
                tokens.push(PtxToken::Directive("f32".to_string()));
                push_operands(destination, on_true, on_false, selector, tokens);
            }
        }
    }
}
