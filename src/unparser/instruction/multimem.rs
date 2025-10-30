use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::multimem::*},
    unparser::*,
};

impl PtxUnparser for LoadSemantics {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            LoadSemantics::Relaxed => "relaxed",
            LoadSemantics::Acquire => "acquire",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for StoreSemantics {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            StoreSemantics::Relaxed => "relaxed",
            StoreSemantics::Release => "release",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for ReductionSemantics {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            ReductionSemantics::Relaxed => "relaxed",
            ReductionSemantics::Release => "release",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for Scope {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            Scope::Cta => "cta",
            Scope::Cluster => "cluster",
            Scope::Gpu => "gpu",
            Scope::Sys => "sys",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for StateSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            StateSpace::Global => "global",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for IntegerOp {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            IntegerOp::Min => "min",
            IntegerOp::Max => "max",
            IntegerOp::Add => "add",
            IntegerOp::And => "and",
            IntegerOp::Or => "or",
            IntegerOp::Xor => "xor",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for FloatOp {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            FloatOp::Min => "min",
            FloatOp::Max => "max",
            FloatOp::Add => "add",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for FloatRedOp {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            FloatRedOp::Add => "add",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for AccumulatorPrecision {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            AccumulatorPrecision::AccF32 => "acc_f32",
            AccumulatorPrecision::AccF16 => "acc_f16",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for VectorWidth {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            VectorWidth::V2 => "v2",
            VectorWidth::V4 => "v4",
            VectorWidth::V8 => "v8",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for IntegerType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            IntegerType::B32 => "b32",
            IntegerType::B64 => "b64",
            IntegerType::U32 => "u32",
            IntegerType::U64 => "u64",
            IntegerType::S32 => "s32",
            IntegerType::S64 => "s64",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for FloatType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            FloatType::F16 => "f16",
            FloatType::F16x2 => "f16x2",
            FloatType::Bf16 => "bf16",
            FloatType::Bf16x2 => "bf16x2",
            FloatType::F32 => "f32",
            FloatType::F64 => "f64",
            FloatType::E5m2 => "e5m2",
            FloatType::E5m2x2 => "e5m2x2",
            FloatType::E5m2x4 => "e5m2x4",
            FloatType::E4m3 => "e4m3",
            FloatType::E4m3x2 => "e4m3x2",
            FloatType::E4m3x4 => "e4m3x4",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for FloatReductionType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            FloatReductionType::F16 => "f16",
            FloatReductionType::F16x2 => "f16x2",
            FloatReductionType::Bf16 => "bf16",
            FloatReductionType::Bf16x2 => "bf16x2",
            FloatReductionType::F32 => "f32",
            FloatReductionType::F64 => "f64",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for VectorDestination {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            VectorDestination::Scalar(register) => register.unparse_tokens(tokens),
            VectorDestination::Vector2(registers) => {
                tokens.push(PtxToken::LBrace);
                registers[0].unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                registers[1].unparse_tokens(tokens);
                tokens.push(PtxToken::RBrace);
            }
            VectorDestination::Vector4(registers) => {
                tokens.push(PtxToken::LBrace);
                for (index, register) in registers.iter().enumerate() {
                    if index > 0 {
                        tokens.push(PtxToken::Comma);
                    }
                    register.unparse_tokens(tokens);
                }
                tokens.push(PtxToken::RBrace);
            }
            VectorDestination::Vector8(registers) => {
                tokens.push(PtxToken::LBrace);
                for (index, register) in registers.iter().enumerate() {
                    if index > 0 {
                        tokens.push(PtxToken::Comma);
                    }
                    register.unparse_tokens(tokens);
                }
                tokens.push(PtxToken::RBrace);
            }
        }
    }
}

impl PtxUnparser for VectorValue {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            VectorValue::Scalar(register) => register.unparse_tokens(tokens),
            VectorValue::Vector2(registers) => {
                tokens.push(PtxToken::LBrace);
                registers[0].unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                registers[1].unparse_tokens(tokens);
                tokens.push(PtxToken::RBrace);
            }
            VectorValue::Vector4(registers) => {
                tokens.push(PtxToken::LBrace);
                for (index, register) in registers.iter().enumerate() {
                    if index > 0 {
                        tokens.push(PtxToken::Comma);
                    }
                    register.unparse_tokens(tokens);
                }
                tokens.push(PtxToken::RBrace);
            }
            VectorValue::Vector8(registers) => {
                tokens.push(PtxToken::LBrace);
                for (index, register) in registers.iter().enumerate() {
                    if index > 0 {
                        tokens.push(PtxToken::Comma);
                    }
                    register.unparse_tokens(tokens);
                }
                tokens.push(PtxToken::RBrace);
            }
        }
    }
}

fn push_ld_reduce_prefix(tokens: &mut Vec<PtxToken>) {
    push_identifier(tokens, "multimem");
    push_directive(tokens, "ld_reduce");
}

fn push_store_prefix(tokens: &mut Vec<PtxToken>) {
    push_identifier(tokens, "multimem");
    push_directive(tokens, "st");
}

fn push_red_prefix(tokens: &mut Vec<PtxToken>) {
    push_identifier(tokens, "multimem");
    push_directive(tokens, "red");
}

fn push_destination_and_address(
    destination: &RegisterOperand,
    address: &AddressOperand,
    tokens: &mut Vec<PtxToken>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    address.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

fn push_vector_destination_and_address(
    destination: &VectorDestination,
    address: &AddressOperand,
    tokens: &mut Vec<PtxToken>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    address.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

fn push_address_and_value(
    address: &AddressOperand,
    value: &RegisterOperand,
    tokens: &mut Vec<PtxToken>,
) {
    address.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    value.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

fn push_address_and_vector_value(
    address: &AddressOperand,
    value: &VectorValue,
    tokens: &mut Vec<PtxToken>,
) {
    address.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    value.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for LdReduceInt {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_ld_reduce_prefix(tokens);
        if let Some(semantics) = &self.semantics {
            semantics.unparse_tokens(tokens);
        }
        if let Some(scope) = &self.scope {
            scope.unparse_tokens(tokens);
        }
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        self.operation.unparse_tokens(tokens);
        self.data_type.unparse_tokens(tokens);
        push_destination_and_address(&self.destination, &self.address, tokens);
    }
}

impl PtxUnparser for LdReduceFloat {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_ld_reduce_prefix(tokens);
        if let Some(semantics) = &self.semantics {
            semantics.unparse_tokens(tokens);
        }
        if let Some(scope) = &self.scope {
            scope.unparse_tokens(tokens);
        }
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        self.operation.unparse_tokens(tokens);
        if let Some(precision) = &self.accumulator_precision {
            precision.unparse_tokens(tokens);
        }
        if let Some(vector) = &self.vector {
            vector.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        push_vector_destination_and_address(&self.destination, &self.address, tokens);
    }
}

impl PtxUnparser for LdReduceWeakInt {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_ld_reduce_prefix(tokens);
        push_directive(tokens, "weak");
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        self.operation.unparse_tokens(tokens);
        self.data_type.unparse_tokens(tokens);
        push_destination_and_address(&self.destination, &self.address, tokens);
    }
}

impl PtxUnparser for LdReduceWeakFloat {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_ld_reduce_prefix(tokens);
        push_directive(tokens, "weak");
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        self.operation.unparse_tokens(tokens);
        if let Some(precision) = &self.accumulator_precision {
            precision.unparse_tokens(tokens);
        }
        if let Some(vector) = &self.vector {
            vector.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        push_vector_destination_and_address(&self.destination, &self.address, tokens);
    }
}

impl PtxUnparser for StoreInt {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_store_prefix(tokens);
        if let Some(semantics) = &self.semantics {
            semantics.unparse_tokens(tokens);
        }
        if let Some(scope) = &self.scope {
            scope.unparse_tokens(tokens);
        }
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        push_address_and_value(&self.address, &self.value, tokens);
    }
}

impl PtxUnparser for StoreFloat {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_store_prefix(tokens);
        if let Some(semantics) = &self.semantics {
            semantics.unparse_tokens(tokens);
        }
        if let Some(scope) = &self.scope {
            scope.unparse_tokens(tokens);
        }
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        if let Some(vector) = &self.vector {
            vector.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        push_address_and_vector_value(&self.address, &self.value, tokens);
    }
}

impl PtxUnparser for StoreWeakInt {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_store_prefix(tokens);
        push_directive(tokens, "weak");
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        push_address_and_value(&self.address, &self.value, tokens);
    }
}

impl PtxUnparser for StoreWeakFloat {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_store_prefix(tokens);
        push_directive(tokens, "weak");
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        if let Some(vector) = &self.vector {
            vector.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        push_address_and_vector_value(&self.address, &self.value, tokens);
    }
}

impl PtxUnparser for RedInt {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_red_prefix(tokens);
        if let Some(semantics) = &self.semantics {
            semantics.unparse_tokens(tokens);
        }
        if let Some(scope) = &self.scope {
            scope.unparse_tokens(tokens);
        }
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        self.operation.unparse_tokens(tokens);
        self.data_type.unparse_tokens(tokens);
        push_address_and_value(&self.address, &self.value, tokens);
    }
}

impl PtxUnparser for RedFloat {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_red_prefix(tokens);
        if let Some(semantics) = &self.semantics {
            semantics.unparse_tokens(tokens);
        }
        if let Some(scope) = &self.scope {
            scope.unparse_tokens(tokens);
        }
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        self.operation.unparse_tokens(tokens);
        if let Some(vector) = &self.vector {
            vector.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        push_address_and_vector_value(&self.address, &self.value, tokens);
    }
}

impl PtxUnparser for LdReduce {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            LdReduce::Int(instruction) => instruction.unparse_tokens(tokens),
            LdReduce::Float(instruction) => instruction.unparse_tokens(tokens),
            LdReduce::WeakInt(instruction) => instruction.unparse_tokens(tokens),
            LdReduce::WeakFloat(instruction) => instruction.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for Store {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Store::Int(instruction) => instruction.unparse_tokens(tokens),
            Store::Float(instruction) => instruction.unparse_tokens(tokens),
            Store::WeakInt(instruction) => instruction.unparse_tokens(tokens),
            Store::WeakFloat(instruction) => instruction.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for Red {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Red::Int(instruction) => instruction.unparse_tokens(tokens),
            Red::Float(instruction) => instruction.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for Instruction {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Instruction::LdReduce(instruction) => instruction.unparse_tokens(tokens),
            Instruction::Store(instruction) => instruction.unparse_tokens(tokens),
            Instruction::Red(instruction) => instruction.unparse_tokens(tokens),
        }
    }
}
