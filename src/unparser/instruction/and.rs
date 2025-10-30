use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::and::*},
    unparser::*,
};

fn push_operands(
    destination: &RegisterOperand,
    a: &RegisterOperand,
    b: &RegisterOperand,
    tokens: &mut Vec<PtxToken>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    a.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    b.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for crate::r#type::instruction::and::DataType {
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

impl PtxUnparser for And {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("and".to_string()));
        self.data_type.unparse_tokens(tokens);
        push_operands(&self.destination, &self.a, &self.b, tokens);
    }
}
