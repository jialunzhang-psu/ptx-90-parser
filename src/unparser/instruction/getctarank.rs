use crate::{
    lexer::PtxToken,
    r#type::instruction::getctarank::{self, Getctarank},
    unparser::*,
};

impl PtxUnparser for getctarank::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            getctarank::DataType::U32 => push_directive(tokens, "u32"),
            getctarank::DataType::U64 => push_directive(tokens, "u64"),
        }
    }
}

impl PtxUnparser for Getctarank {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "getctarank");

        match self {
            Getctarank::Generic {
                data_type,
                destination,
                source,
            } => {
                data_type.unparse_tokens(tokens);
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                source.unparse_tokens(tokens);
            }
            Getctarank::SharedRegister {
                data_type,
                destination,
                source,
            } => {
                push_directive(tokens, "shared");
                tokens.push(PtxToken::Colon);
                tokens.push(PtxToken::Colon);
                push_identifier(tokens, "cluster");
                data_type.unparse_tokens(tokens);
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                source.unparse_tokens(tokens);
            }
            Getctarank::SharedVariable {
                data_type,
                destination,
                symbol,
            } => {
                push_directive(tokens, "shared");
                tokens.push(PtxToken::Colon);
                tokens.push(PtxToken::Colon);
                push_identifier(tokens, "cluster");
                data_type.unparse_tokens(tokens);
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                symbol.unparse_tokens(tokens);
            }
            Getctarank::SharedVariableWithImmediate {
                data_type,
                destination,
                symbol,
                immediate,
            } => {
                push_directive(tokens, "shared");
                tokens.push(PtxToken::Colon);
                tokens.push(PtxToken::Colon);
                push_identifier(tokens, "cluster");
                data_type.unparse_tokens(tokens);
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                symbol.unparse_tokens(tokens);
                tokens.push(PtxToken::Plus);
                immediate.unparse_tokens(tokens);
            }
        }

        tokens.push(PtxToken::Semicolon);
    }
}
