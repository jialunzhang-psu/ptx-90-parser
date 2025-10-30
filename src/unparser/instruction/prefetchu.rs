use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::prefetchu::*},
    unparser::*,
};

fn unparse_address(tokens: &mut Vec<PtxToken>, address: &AddressOperand) {
    address.unparse_tokens(tokens);
}

impl PtxUnparser for PrefetchuLevel {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            PrefetchuLevel::L1 => tokens.push(PtxToken::Directive("L1".to_string())),
        }
    }
}

impl PtxUnparser for Prefetchu {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("prefetchu".to_string()));
        self.level.unparse_tokens(tokens);
        unparse_address(tokens, &self.address);
        tokens.push(PtxToken::Semicolon);
    }
}
