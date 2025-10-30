use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::nanosleep::*},
    unparser::*,
};

impl PtxUnparser for crate::r#type::instruction::nanosleep::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Self::U32 => "u32",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Nanosleep {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("nanosleep".to_string()));
        self.data_type.unparse_tokens(tokens);
        let delay: &Operand = &self.delay;
        delay.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
