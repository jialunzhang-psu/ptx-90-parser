use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::mov::*},
    unparser::*,
};

impl PtxUnparser for crate::r#type::instruction::mov::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Self::Pred => "pred",
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

impl PtxUnparser for AddressType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Self::U32 => "u32",
            Self::U64 => "u64",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Destination {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Self::Register(register) => register.unparse_tokens(tokens),
            Self::Predicate(predicate) => predicate.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for RegisterSource {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Self::Register(register) => register.unparse_tokens(tokens),
            Self::Predicate(predicate) => predicate.unparse_tokens(tokens),
            Self::Immediate(immediate) => immediate.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for SpecialRegisterSource {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Self::Register(reg) => reg.unparse_tokens(tokens),
            Self::Predicate(predicate) => predicate.unparse_tokens(tokens),
        }
    }
}

fn push_destination_and_comma(dest: &Destination, tokens: &mut Vec<PtxToken>) {
    dest.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
}

fn push_register_destination_and_comma(dest: &RegisterOperand, tokens: &mut Vec<PtxToken>) {
    dest.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
}

impl PtxUnparser for Mov {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Self::Register(instruction) => {
                push_opcode(tokens, "mov");
                instruction.data_type.unparse_tokens(tokens);
                push_destination_and_comma(&instruction.destination, tokens);
                instruction.source.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
            Self::SpecialRegister(instruction) => {
                push_opcode(tokens, "mov");
                instruction.data_type.unparse_tokens(tokens);
                push_destination_and_comma(&instruction.destination, tokens);
                instruction.source.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
            Self::Variable(instruction) => {
                push_opcode(tokens, "mov");
                instruction.data_type.unparse_tokens(tokens);
                push_register_destination_and_comma(&instruction.destination, tokens);
                instruction.variable.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
            Self::VariableWithImmediate(instruction) => {
                push_opcode(tokens, "mov");
                instruction.data_type.unparse_tokens(tokens);
                push_register_destination_and_comma(&instruction.destination, tokens);
                instruction.variable.unparse_tokens(tokens);
                tokens.push(PtxToken::Plus);
                instruction.immediate.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
            Self::FunctionAddress(instruction) => {
                push_opcode(tokens, "mov");
                instruction.data_type.unparse_tokens(tokens);
                push_register_destination_and_comma(&instruction.destination, tokens);
                instruction.function.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
            Self::KernelAddress(instruction) => {
                push_opcode(tokens, "mov");
                instruction.data_type.unparse_tokens(tokens);
                push_register_destination_and_comma(&instruction.destination, tokens);
                instruction.kernel.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
        }
    }
}
