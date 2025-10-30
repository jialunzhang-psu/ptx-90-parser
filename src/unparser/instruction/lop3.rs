use crate::{
    lexer::PtxToken,
    r#type::instruction::lop3::{BoolOp, Boolean, DataType, Destination, Lop3, Plain},
    unparser::*,
};

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Directive("b32".to_string()));
    }
}

impl PtxUnparser for BoolOp {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Self::Or => "or",
            Self::And => "and",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Destination {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Destination::Register(register) => register.unparse_tokens(tokens),
            Destination::Sink => tokens.push(PtxToken::Identifier("_".to_string())),
        }
    }
}

impl PtxUnparser for Plain {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.c.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.lut.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Boolean {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.operator.unparse_tokens(tokens);
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Pipe);
        self.predicate.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.c.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.lut.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.predicate_input.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Lop3 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("lop3".to_string()));
        match self {
            Lop3::Plain(plain) => plain.unparse_tokens(tokens),
            Lop3::Boolean(boolean) => boolean.unparse_tokens(tokens),
        }
    }
}
