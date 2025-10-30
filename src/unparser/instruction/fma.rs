use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::fma::*},
    unparser::*,
};

fn push_saturate(tokens: &mut Vec<PtxToken>, saturate: bool) {
    if saturate {
        tokens.push(PtxToken::Directive("sat".to_string()));
    }
}

fn push_operands(
    destination: &RegisterOperand,
    multiplicand_a: &RegisterOperand,
    multiplicand_b: &RegisterOperand,
    addend: &RegisterOperand,
    tokens: &mut Vec<PtxToken>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    multiplicand_a.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    multiplicand_b.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    addend.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
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

impl PtxUnparser for Fma {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("fma".to_string()));

        match self {
            Fma::F32 {
                rounding,
                flush_to_zero,
                saturate,
                destination,
                multiplicand_a,
                multiplicand_b,
                addend,
            } => {
                rounding.unparse_tokens(tokens);
                push_flush_to_zero(tokens, *flush_to_zero);
                push_saturate(tokens, *saturate);
                tokens.push(PtxToken::Directive("f32".to_string()));
                push_operands(destination, multiplicand_a, multiplicand_b, addend, tokens);
            }
            Fma::F32x2 {
                rounding,
                flush_to_zero,
                destination,
                multiplicand_a,
                multiplicand_b,
                addend,
            } => {
                rounding.unparse_tokens(tokens);
                push_flush_to_zero(tokens, *flush_to_zero);
                tokens.push(PtxToken::Directive("f32x2".to_string()));
                push_operands(destination, multiplicand_a, multiplicand_b, addend, tokens);
            }
            Fma::F64 {
                rounding,
                destination,
                multiplicand_a,
                multiplicand_b,
                addend,
            } => {
                rounding.unparse_tokens(tokens);
                tokens.push(PtxToken::Directive("f64".to_string()));
                push_operands(destination, multiplicand_a, multiplicand_b, addend, tokens);
            }
        }
    }
}
