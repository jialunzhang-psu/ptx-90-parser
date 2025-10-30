use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::rcp::*},
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

impl PtxUnparser for Rounding {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Rounding::Rn => "rn",
            Rounding::Rz => "rz",
            Rounding::Rm => "rm",
            Rounding::Rp => "rp",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Rcp {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("rcp".to_string()));
        match self {
            Rcp::ApproxF32(ApproxF32 {
                flush_to_zero,
                destination,
                source,
            }) => {
                tokens.push(PtxToken::Directive("approx".to_string()));
                push_flush_to_zero(tokens, *flush_to_zero);
                tokens.push(PtxToken::Directive("f32".to_string()));
                push_operands(destination, source, tokens);
            }
            Rcp::RndF32(RndF32 {
                rounding,
                flush_to_zero,
                destination,
                source,
            }) => {
                tokens.push(PtxToken::Directive("rnd".to_string()));
                rounding.unparse_tokens(tokens);
                push_flush_to_zero(tokens, *flush_to_zero);
                tokens.push(PtxToken::Directive("f32".to_string()));
                push_operands(destination, source, tokens);
            }
            Rcp::RndF64(RndF64 {
                rounding,
                destination,
                source,
            }) => {
                tokens.push(PtxToken::Directive("rnd".to_string()));
                rounding.unparse_tokens(tokens);
                tokens.push(PtxToken::Directive("f64".to_string()));
                push_operands(destination, source, tokens);
            }
        };
        tokens.push(PtxToken::Semicolon);
    }
}
