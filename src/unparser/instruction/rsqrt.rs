use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::rsqrt::*},
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
}

impl PtxUnparser for Rsqrt {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("rsqrt".to_string()));
        match self {
            Rsqrt::ApproxF32(ApproxF32 {
                flush_to_zero,
                destination,
                source,
            }) => {
                tokens.push(PtxToken::Directive("approx".to_string()));
                push_flush_to_zero(tokens, *flush_to_zero);
                tokens.push(PtxToken::Directive("f32".to_string()));
                push_operands(destination, source, tokens);
            }
            Rsqrt::ApproxF64(ApproxF64 {
                destination,
                source,
            }) => {
                tokens.push(PtxToken::Directive("approx".to_string()));
                tokens.push(PtxToken::Directive("f64".to_string()));
                push_operands(destination, source, tokens);
            }
        }
        tokens.push(PtxToken::Semicolon);
    }
}
