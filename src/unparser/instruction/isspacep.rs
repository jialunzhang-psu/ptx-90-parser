use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::isspacep::*},
    unparser::*,
};

impl PtxUnparser for Space {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Space::Const => push_directive(tokens, "const"),
            Space::Global => push_directive(tokens, "global"),
            Space::Local => push_directive(tokens, "local"),
            Space::Shared => push_directive(tokens, "shared"),
            Space::SharedCta => {
                push_directive(tokens, "shared");
                push_double_colon(tokens);
                push_identifier(tokens, "cta");
            }
            Space::SharedCluster => {
                push_directive(tokens, "shared");
                push_double_colon(tokens);
                push_identifier(tokens, "cluster");
            }
            Space::Param => push_directive(tokens, "param"),
            Space::ParamEntry => {
                push_directive(tokens, "param");
                push_double_colon(tokens);
                push_identifier(tokens, "entry");
            }
        }
    }
}

impl PtxUnparser for Isspacep {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("isspacep".to_string()));
        self.space.unparse_tokens(tokens);
        let predicate: &PredicateRegister = &self.predicate;
        predicate.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        let address: &RegisterOperand = &self.address;
        address.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
