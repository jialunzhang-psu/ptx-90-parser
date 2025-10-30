use crate::{
    lexer::PtxToken,
    r#type::{
        common::{PredicateRegister, RegisterOperand},
        instruction::r#match::{All, Any, DataType, Match},
    },
    unparser::*,
};

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            DataType::B32 => "b32",
            DataType::B64 => "b64",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Any {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Directive("any".to_string()));
        tokens.push(PtxToken::Directive("sync".to_string()));
        self.data_type.unparse_tokens(tokens);
        let destination: &RegisterOperand = &self.destination;
        destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        let source: &RegisterOperand = &self.source;
        source.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.member_mask.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for All {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Directive("all".to_string()));
        tokens.push(PtxToken::Directive("sync".to_string()));
        self.data_type.unparse_tokens(tokens);
        let destination: &RegisterOperand = &self.destination;
        destination.unparse_tokens(tokens);
        if let Some(predicate) = &self.predicate {
            tokens.push(PtxToken::Pipe);
            let predicate: &PredicateRegister = predicate;
            predicate.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Comma);
        let source: &RegisterOperand = &self.source;
        source.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.member_mask.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Match {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("match".to_string()));
        match self {
            Match::Any(any) => any.unparse_tokens(tokens),
            Match::All(all) => all.unparse_tokens(tokens),
        }
    }
}
