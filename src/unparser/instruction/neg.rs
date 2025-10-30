use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::neg::*},
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

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            DataType::S16 => "s16",
            DataType::S32 => "s32",
            DataType::S64 => "s64",
        };

        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Neg {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("neg".to_string()));
        self.data_type.unparse_tokens(tokens);
        push_operands(&self.destination, &self.source, tokens);
    }
}
