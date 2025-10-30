use crate::{
    lexer::PtxToken,
    r#type::{
        common::{PredicateRegister, RegisterOperand},
        instruction::istypep::{DataType, Istypep},
    },
    unparser::*,
};

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            DataType::TexRef => push_directive(tokens, "texref"),
            DataType::SamplerRef => push_directive(tokens, "samplerref"),
            DataType::SurfRef => push_directive(tokens, "surfref"),
        }
    }
}

impl PtxUnparser for Istypep {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("istypep".to_string()));
        self.data_type.unparse_tokens(tokens);
        let predicate: &PredicateRegister = &self.predicate;
        predicate.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        let address: &RegisterOperand = &self.address;
        address.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
