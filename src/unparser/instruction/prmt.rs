use crate::{lexer::PtxToken, r#type::instruction::prmt::*, unparser::*};

impl PtxUnparser for Mode {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Mode::F4e => "f4e",
            Mode::B4e => "b4e",
            Mode::Rc8 => "rc8",
            Mode::Ecl => "ecl",
            Mode::Ecr => "ecr",
            Mode::Rc16 => "rc16",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Prmt {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("prmt".to_string()));
        tokens.push(PtxToken::Directive("b32".to_string()));
        if let Some(mode) = self.mode {
            mode.unparse_tokens(tokens);
        }
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.c.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
