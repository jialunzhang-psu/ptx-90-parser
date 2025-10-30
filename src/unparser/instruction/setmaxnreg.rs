use crate::{lexer::PtxToken, r#type::instruction::setmaxnreg::*, unparser::*};

impl PtxUnparser for Action {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            Action::Inc => "inc",
            Action::Dec => "dec",
        };
        tokens.push(PtxToken::Directive(name.to_string()));
    }
}

impl PtxUnparser for Setmaxnreg {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("setmaxnreg".to_string()));
        self.action.unparse_tokens(tokens);
        tokens.push(PtxToken::Directive("sync".to_string()));
        tokens.push(PtxToken::Directive("aligned".to_string()));
        tokens.push(PtxToken::Directive("u32".to_string()));
        self.register_count.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
