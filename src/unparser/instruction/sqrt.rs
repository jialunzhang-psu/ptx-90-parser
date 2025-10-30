use crate::{lexer::PtxToken, r#type::instruction::sqrt::*, unparser::*};

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

impl PtxUnparser for Sqrt {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("sqrt".to_string()));
        match self {
            Sqrt::ApproxF32 {
                flush_to_zero,
                destination,
                source,
            } => {
                tokens.push(PtxToken::Directive("approx".to_string()));
                push_flush_to_zero(tokens, *flush_to_zero);
                tokens.push(PtxToken::Directive("f32".to_string()));
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                source.unparse_tokens(tokens);
            }
            Sqrt::RndF32 {
                rounding,
                flush_to_zero,
                destination,
                source,
            } => {
                tokens.push(PtxToken::Directive("rnd".to_string()));
                rounding.unparse_tokens(tokens);
                push_flush_to_zero(tokens, *flush_to_zero);
                tokens.push(PtxToken::Directive("f32".to_string()));
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                source.unparse_tokens(tokens);
            }
            Sqrt::RndF64 {
                rounding,
                destination,
                source,
            } => {
                tokens.push(PtxToken::Directive("rnd".to_string()));
                rounding.unparse_tokens(tokens);
                tokens.push(PtxToken::Directive("f64".to_string()));
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                source.unparse_tokens(tokens);
            }
        }
        tokens.push(PtxToken::Semicolon);
    }
}
