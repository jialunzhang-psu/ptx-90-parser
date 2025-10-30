use crate::{lexer::PtxToken, r#type::instruction::brkpt::Brkpt, unparser::PtxUnparser};

impl PtxUnparser for Brkpt {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("brkpt".to_string()));
        tokens.push(PtxToken::Semicolon);
    }
}
