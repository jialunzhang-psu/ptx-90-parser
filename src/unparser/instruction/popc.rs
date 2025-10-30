use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::popc::Popc},
    unparser::PtxUnparser,
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

impl PtxUnparser for crate::r#type::instruction::popc::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Self::B32 => tokens.push(PtxToken::Directive("b32".to_string())),
            Self::B64 => tokens.push(PtxToken::Directive("b64".to_string())),
        }
    }
}

impl PtxUnparser for Popc {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("popc".to_string()));
        self.data_type.unparse_tokens(tokens);
        push_operands(&self.destination, &self.source, tokens);
    }
}
