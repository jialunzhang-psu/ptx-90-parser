use crate::{lexer::PtxToken, r#type::instruction::bmsk::*, unparser::*};

impl PtxUnparser for Mode {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Mode::Clamp => "clamp",
            Mode::Wrap => "wrap",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Bmsk {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("bmsk".to_string()));
        self.mode.unparse_tokens(tokens);
        tokens.push(PtxToken::Directive("b32".to_string()));

        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
