use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::not::*},
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

impl PtxUnparser for crate::r#type::instruction::not::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            DataType::Pred => "pred",
            DataType::B16 => "b16",
            DataType::B32 => "b32",
            DataType::B64 => "b64",
        };

        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Not {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("not".to_string()));
        self.data_type.unparse_tokens(tokens);
        push_operands(&self.destination, &self.source, tokens);
    }
}
