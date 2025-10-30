use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::selp::*},
    unparser::*,
};

fn push_operands(
    destination: &RegisterOperand,
    true_value: &RegisterOperand,
    false_value: &RegisterOperand,
    predicate: &PredicateRegister,
    tokens: &mut Vec<PtxToken>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    true_value.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    false_value.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    predicate.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for crate::r#type::instruction::selp::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Self::B16 => "b16",
            Self::B32 => "b32",
            Self::B64 => "b64",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::S16 => "s16",
            Self::S32 => "s32",
            Self::S64 => "s64",
            Self::F32 => "f32",
            Self::F64 => "f64",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Selp {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "selp");
        self.data_type.unparse_tokens(tokens);
        push_operands(
            &self.destination,
            &self.true_value,
            &self.false_value,
            &self.predicate,
            tokens,
        );
    }
}
