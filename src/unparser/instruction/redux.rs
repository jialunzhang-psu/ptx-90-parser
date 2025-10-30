use crate::{
    lexer::PtxToken,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::redux::{
            Bitwise, BitwiseOperator, Float, FloatOperator, Integer, IntegerOperator, Redux,
        },
    },
    unparser::*,
};

fn push_operands(
    destination: &RegisterOperand,
    source: &RegisterOperand,
    member_mask: &Operand,
    tokens: &mut Vec<PtxToken>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    source.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    member_mask.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for IntegerOperator {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            IntegerOperator::Add => "add",
            IntegerOperator::Min => "min",
            IntegerOperator::Max => "max",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Integer {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.operator.unparse_tokens(tokens);
        let directive = match self.data_type {
            crate::r#type::instruction::redux::DataType::U32 => "u32",
            crate::r#type::instruction::redux::DataType::S32 => "s32",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
        push_operands(&self.destination, &self.source, &self.member_mask, tokens);
    }
}

impl PtxUnparser for BitwiseOperator {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            BitwiseOperator::And => "and",
            BitwiseOperator::Or => "or",
            BitwiseOperator::Xor => "xor",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Bitwise {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.operator.unparse_tokens(tokens);
        tokens.push(PtxToken::Directive("b32".to_string()));
        push_operands(&self.destination, &self.source, &self.member_mask, tokens);
    }
}

impl PtxUnparser for FloatOperator {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            FloatOperator::Min => "min",
            FloatOperator::Max => "max",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Float {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.operator.unparse_tokens(tokens);
        if self.absolute {
            tokens.push(PtxToken::Directive("abs".to_string()));
        }
        if self.propagate_nan {
            tokens.push(PtxToken::Directive("NaN".to_string()));
        }
        tokens.push(PtxToken::Directive("f32".to_string()));
        push_operands(&self.destination, &self.source, &self.member_mask, tokens);
    }
}

impl PtxUnparser for Redux {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("redux".to_string()));
        tokens.push(PtxToken::Directive("sync".to_string()));
        match self {
            Redux::Integer(integer) => integer.unparse_tokens(tokens),
            Redux::Bitwise(bitwise) => bitwise.unparse_tokens(tokens),
            Redux::Float(float) => float.unparse_tokens(tokens),
        }
    }
}
