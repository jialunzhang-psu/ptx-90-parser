use crate::{lexer::PtxToken, r#type::instruction::exit::Exit, unparser::PtxUnparser};

impl PtxUnparser for Exit {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("exit".to_string()));
        tokens.push(PtxToken::Semicolon);
    }
}
