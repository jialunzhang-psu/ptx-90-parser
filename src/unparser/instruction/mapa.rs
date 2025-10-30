use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::mapa},
    unparser::*,
};

fn push_shared_cluster_prefix(tokens: &mut Vec<PtxToken>) {
    push_directive(tokens, "shared");
    push_double_colon(tokens);
    push_identifier(tokens, "cluster");
}

fn push_destination(tokens: &mut Vec<PtxToken>, destination: &RegisterOperand) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
}

fn push_cta_operand(tokens: &mut Vec<PtxToken>, cta: &Operand) {
    cta.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for mapa::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            mapa::DataType::U32 => {
                push_directive(tokens, "u32");
            }
            mapa::DataType::U64 => {
                push_directive(tokens, "u64");
            }
        }
    }
}

impl PtxUnparser for mapa::Mapa {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "mapa");
        match self {
            mapa::Mapa::Generic {
                data_type,
                destination,
                address,
                cta,
            } => {
                data_type.unparse_tokens(tokens);
                push_destination(tokens, destination);
                address.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                push_cta_operand(tokens, cta);
            }
            mapa::Mapa::SharedRegister {
                data_type,
                destination,
                address,
                cta,
            } => {
                push_shared_cluster_prefix(tokens);
                data_type.unparse_tokens(tokens);
                push_destination(tokens, destination);
                address.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                push_cta_operand(tokens, cta);
            }
            mapa::Mapa::SharedVariable {
                data_type,
                destination,
                variable,
                cta,
            } => {
                push_shared_cluster_prefix(tokens);
                data_type.unparse_tokens(tokens);
                push_destination(tokens, destination);
                variable.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                push_cta_operand(tokens, cta);
            }
            mapa::Mapa::SharedVariableWithImmediate {
                data_type,
                destination,
                variable,
                immediate,
                cta,
            } => {
                push_shared_cluster_prefix(tokens);
                data_type.unparse_tokens(tokens);
                push_destination(tokens, destination);
                variable.unparse_tokens(tokens);
                tokens.push(PtxToken::Plus);
                immediate.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                push_cta_operand(tokens, cta);
            }
        }
    }
}
