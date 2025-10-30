use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::ldu::*},
    unparser::*,
};

impl PtxUnparser for StateSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            StateSpace::Global => push_directive(tokens, "global"),
        }
    }
}

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            DataType::B8 => "b8",
            DataType::B16 => "b16",
            DataType::B32 => "b32",
            DataType::B64 => "b64",
            DataType::B128 => "b128",
            DataType::U8 => "u8",
            DataType::U16 => "u16",
            DataType::U32 => "u32",
            DataType::U64 => "u64",
            DataType::S8 => "s8",
            DataType::S16 => "s16",
            DataType::S32 => "s32",
            DataType::S64 => "s64",
            DataType::F32 => "f32",
            DataType::F64 => "f64",
        };
        push_directive(tokens, name);
    }
}

fn push_vector_registers(tokens: &mut Vec<PtxToken>, registers: &[RegisterOperand]) {
    tokens.push(PtxToken::LBrace);
    for (index, register) in registers.iter().enumerate() {
        if index > 0 {
            tokens.push(PtxToken::Comma);
        }
        register.unparse_tokens(tokens);
    }
    tokens.push(PtxToken::RBrace);
}

impl PtxUnparser for VectorDestination {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            VectorDestination::V2(registers) => push_vector_registers(tokens, registers),
            VectorDestination::V4(registers) => push_vector_registers(tokens, registers),
        }
    }
}

fn push_vector_width(tokens: &mut Vec<PtxToken>, destination: &VectorDestination) {
    let modifier = match destination {
        VectorDestination::V2(_) => "v2",
        VectorDestination::V4(_) => "v4",
    };
    push_directive(tokens, modifier);
}

impl PtxUnparser for Scalar {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "ldu");
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Vector {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "ldu");
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        push_vector_width(tokens, &self.destination);
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Ldu {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Ldu::Scalar(instruction) => instruction.unparse_tokens(tokens),
            Ldu::Vector(instruction) => instruction.unparse_tokens(tokens),
        }
    }
}
