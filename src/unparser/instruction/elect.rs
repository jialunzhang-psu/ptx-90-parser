use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::elect::*},
    unparser::*,
};

impl PtxUnparser for Destination {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Destination::Register(register) => {
                let register: &RegisterOperand = register;
                register.unparse_tokens(tokens);
            }
            Destination::Sink => tokens.push(PtxToken::Identifier("_".to_string())),
        }
    }
}

impl PtxUnparser for Elect {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("elect".to_string()));
        tokens.push(PtxToken::Directive("sync".to_string()));
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Pipe);
        let predicate: &PredicateRegister = &self.predicate;
        predicate.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        let member_mask: &Operand = &self.member_mask;
        member_mask.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
