use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::vote::*},
    unparser::*,
};

impl PtxUnparser for Mode {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            Mode::All => "all",
            Mode::Any => "any",
            Mode::Uni => "uni",
        };

        tokens.push(PtxToken::Directive(modifier.to_string()));
    }
}

impl PtxUnparser for PredicateOperand {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        if self.negated {
            tokens.push(PtxToken::Exclaim);
        }
        let register: &PredicateRegister = &self.register;
        register.unparse_tokens(tokens);
    }
}

impl PtxUnparser for Predicate {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.mode.unparse_tokens(tokens);
        tokens.push(PtxToken::Directive("pred".to_string()));
        let destination: &PredicateRegister = &self.destination;
        destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Ballot {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Directive("ballot".to_string()));
        tokens.push(PtxToken::Directive("b32".to_string()));
        let destination: &RegisterOperand = &self.destination;
        destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Vote {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("vote".to_string()));
        match self {
            Vote::Predicate(predicate) => predicate.unparse_tokens(tokens),
            Vote::Ballot(ballot) => ballot.unparse_tokens(tokens),
        }
    }
}
