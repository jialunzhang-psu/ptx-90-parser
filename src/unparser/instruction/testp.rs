use crate::{lexer::PtxToken, r#type::instruction::testp::*, unparser::*};

impl PtxUnparser for PredicateTest {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            PredicateTest::Finite => "finite",
            PredicateTest::Infinite => "infinite",
            PredicateTest::Number => "number",
            PredicateTest::NotANumber => "notanumber",
            PredicateTest::Normal => "normal",
            PredicateTest::Subnormal => "subnormal",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            DataType::F32 => "f32",
            DataType::F64 => "f64",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Testp {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("testp".to_string()));
        self.test.unparse_tokens(tokens);
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
