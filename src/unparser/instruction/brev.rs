use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::brev::*},
    unparser::*,
};

fn push_operands(
    destination: &RegisterOperand,
    source: &RegisterOperand,
    tokens: &mut Vec<PtxToken>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    source.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for crate::r#type::instruction::brev::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Self::B32 => "b32",
            Self::B64 => "b64",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Brev {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("brev".to_string()));
        self.data_type.unparse_tokens(tokens);
        push_operands(&self.destination, &self.source, tokens);
    }
}
