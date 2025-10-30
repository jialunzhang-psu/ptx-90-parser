use crate::{lexer::PtxToken, r#type::instruction::bra::*, unparser::*};

fn push_optional_uniform(tokens: &mut Vec<PtxToken>, uniform: bool) {
    if uniform {
        tokens.push(PtxToken::Directive("uni".to_string()));
    }
}

impl PtxUnparser for Bra {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "bra");
        push_optional_uniform(tokens, self.uniform);
        self.target.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
