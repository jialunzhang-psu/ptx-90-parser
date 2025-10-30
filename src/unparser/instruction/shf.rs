use crate::{
    lexer::PtxToken,
    r#type::instruction::shf::{DataType, Direction, Mode, Shf},
    unparser::*,
};

impl PtxUnparser for Direction {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Direction::Left => "l",
            Direction::Right => "r",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Mode {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Mode::Clamp => "clamp",
            Mode::Wrap => "wrap",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Directive("b32".to_string()));
    }
}

impl PtxUnparser for Shf {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("shf".to_string()));
        self.direction.unparse_tokens(tokens);
        self.mode.unparse_tokens(tokens);
        self.data_type.unparse_tokens(tokens);
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
