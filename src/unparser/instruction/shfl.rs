use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::shfl::*},
    unparser::*,
};

impl PtxUnparser for Mode {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Mode::Up => "up",
            Mode::Down => "down",
            Mode::Bfly => "bfly",
            Mode::Idx => "idx",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for crate::r#type::instruction::shfl::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Directive("b32".to_string()));
    }
}

impl PtxUnparser for Destination {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let register: &RegisterOperand = &self.register;
        register.unparse_tokens(tokens);
        if let Some(predicate) = &self.predicate {
            tokens.push(PtxToken::Pipe);
            let predicate: &PredicateRegister = predicate;
            predicate.unparse_tokens(tokens);
        }
    }
}

impl PtxUnparser for Shfl {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("shfl".to_string()));
        self.mode.unparse_tokens(tokens);
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.lane.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.clamp.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
