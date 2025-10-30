use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::atom::*},
    unparser::*,
};

fn push_atom_base(tokens: &mut Vec<PtxToken>, semantics: Option<Semantics>, scope: Option<Scope>) {
    push_identifier(tokens, "atom");
    if let Some(semantics) = semantics {
        semantics.unparse_tokens(tokens);
    }
    if let Some(scope) = scope {
        scope.unparse_tokens(tokens);
    }
}

fn push_state_space(tokens: &mut Vec<PtxToken>, state_space: Option<StateSpace>) {
    if let Some(space) = state_space {
        space.unparse_tokens(tokens);
    }
}

fn push_vector_state_space(tokens: &mut Vec<PtxToken>, state_space: Option<VectorStateSpace>) {
    if let Some(space) = state_space {
        space.unparse_tokens(tokens);
    }
}

fn push_scalar_operands(
    tokens: &mut Vec<PtxToken>,
    destination: &RegisterOperand,
    address: &AddressOperand,
    source: &RegisterOperand,
    cache_policy: &Option<RegisterOperand>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    address.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    source.unparse_tokens(tokens);
    if let Some(policy) = cache_policy {
        tokens.push(PtxToken::Comma);
        policy.unparse_tokens(tokens);
    }
    tokens.push(PtxToken::Semicolon);
}

fn push_compare_swap_operands(
    tokens: &mut Vec<PtxToken>,
    destination: &RegisterOperand,
    address: &AddressOperand,
    compare: &RegisterOperand,
    new_value: &RegisterOperand,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    address.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    compare.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    new_value.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

fn push_register_group(tokens: &mut Vec<PtxToken>, registers: &[RegisterOperand]) {
    tokens.push(PtxToken::LBrace);
    for (index, register) in registers.iter().enumerate() {
        register.unparse_tokens(tokens);
        if index + 1 != registers.len() {
            tokens.push(PtxToken::Comma);
        }
    }
    tokens.push(PtxToken::RBrace);
}

fn push_vector_operands32(
    tokens: &mut Vec<PtxToken>,
    destination: &Vec32Registers,
    address: &AddressOperand,
    source: &Vec32Registers,
    cache_policy: &Option<RegisterOperand>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    address.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    source.unparse_tokens(tokens);
    if let Some(policy) = cache_policy {
        tokens.push(PtxToken::Comma);
        policy.unparse_tokens(tokens);
    }
    tokens.push(PtxToken::Semicolon);
}

fn push_vector_operands16(
    tokens: &mut Vec<PtxToken>,
    destination: &Vec16Registers,
    address: &AddressOperand,
    source: &Vec16Registers,
    cache_policy: &Option<RegisterOperand>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    address.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    source.unparse_tokens(tokens);
    if let Some(policy) = cache_policy {
        tokens.push(PtxToken::Comma);
        policy.unparse_tokens(tokens);
    }
    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for Semantics {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Semantics::Relaxed => "relaxed",
            Semantics::Acquire => "acquire",
            Semantics::Release => "release",
            Semantics::AcqRel => "acq_rel",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Scope {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Scope::Cta => "cta",
            Scope::Cluster => "cluster",
            Scope::Gpu => "gpu",
            Scope::Sys => "sys",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for StateSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            StateSpace::Global => push_directive(tokens, "global"),
            StateSpace::Shared(SharedSpace::Cta) => {
                push_directive(tokens, "shared");
                push_double_colon(tokens);
                push_identifier(tokens, "cta");
            }
            StateSpace::Shared(SharedSpace::Cluster) => {
                push_directive(tokens, "shared");
                push_double_colon(tokens);
                push_identifier(tokens, "cluster");
            }
        }
    }
}

impl PtxUnparser for VectorStateSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            VectorStateSpace::Global => push_directive(tokens, "global"),
        }
    }
}

impl PtxUnparser for CacheHint {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            CacheHint::L2CacheHint => {
                push_directive(tokens, "L2");
                push_double_colon(tokens);
                push_identifier(tokens, "cache_hint");
            }
        }
    }
}

impl PtxUnparser for crate::r#type::instruction::atom::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Self::B32 => "b32",
            Self::B64 => "b64",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::S32 => "s32",
            Self::S64 => "s64",
            Self::F32 => "f32",
            Self::F64 => "f64",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for ScalarOperation {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            ScalarOperation::And => "and",
            ScalarOperation::Or => "or",
            ScalarOperation::Xor => "xor",
            ScalarOperation::Exch => "exch",
            ScalarOperation::Add => "add",
            ScalarOperation::Inc => "inc",
            ScalarOperation::Dec => "dec",
            ScalarOperation::Min => "min",
            ScalarOperation::Max => "max",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for CompareSwapVariant {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            CompareSwapVariant::Typed(data_type) => data_type.unparse_tokens(tokens),
            CompareSwapVariant::B16 => push_directive(tokens, "b16"),
            CompareSwapVariant::B128 => push_directive(tokens, "b128"),
        }
    }
}

impl PtxUnparser for NoFtzType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            NoFtzType::F16 => "f16",
            NoFtzType::F16x2 => "f16x2",
            NoFtzType::Bf16 => "bf16",
            NoFtzType::Bf16x2 => "bf16x2",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for VectorOperation {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            VectorOperation::Add => "add",
            VectorOperation::Min => "min",
            VectorOperation::Max => "max",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for HalfWordType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            HalfWordType::F16 => "f16",
            HalfWordType::Bf16 => "bf16",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for PackedType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            PackedType::F16x2 => "f16x2",
            PackedType::Bf16x2 => "bf16x2",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Vec16Registers {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Vec16Registers::V2(registers) => push_register_group(tokens, registers),
            Vec16Registers::V4(registers) => push_register_group(tokens, registers),
            Vec16Registers::V8(registers) => push_register_group(tokens, registers),
        }
    }
}

impl PtxUnparser for Vec32Registers {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Vec32Registers::V2(registers) => push_register_group(tokens, registers),
            Vec32Registers::V4(registers) => push_register_group(tokens, registers),
        }
    }
}

impl PtxUnparser for Scalar {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_atom_base(tokens, self.semantics, self.scope);
        push_state_space(tokens, self.state_space);
        self.operation.unparse_tokens(tokens);
        if let Some(cache_hint) = &self.cache_hint {
            cache_hint.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        push_scalar_operands(
            tokens,
            &self.destination,
            &self.address,
            &self.source,
            &self.cache_policy,
        );
    }
}

impl PtxUnparser for CompareSwap {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_atom_base(tokens, self.semantics, self.scope);
        push_state_space(tokens, self.state_space);
        push_directive(tokens, "cas");
        self.variant.unparse_tokens(tokens);
        push_compare_swap_operands(
            tokens,
            &self.destination,
            &self.address,
            &self.compare,
            &self.new_value,
        );
    }
}

impl PtxUnparser for Exchange128 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_atom_base(tokens, self.semantics, self.scope);
        push_state_space(tokens, self.state_space);
        push_directive(tokens, "exch");
        if let Some(cache_hint) = &self.cache_hint {
            cache_hint.unparse_tokens(tokens);
        }
        push_directive(tokens, "b128");
        push_scalar_operands(
            tokens,
            &self.destination,
            &self.address,
            &self.source,
            &self.cache_policy,
        );
    }
}

impl PtxUnparser for AddNoFtz {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_atom_base(tokens, self.semantics, self.scope);
        push_state_space(tokens, self.state_space);
        push_directive(tokens, "add");
        push_directive(tokens, "noftz");
        if let Some(cache_hint) = &self.cache_hint {
            cache_hint.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        push_scalar_operands(
            tokens,
            &self.destination,
            &self.address,
            &self.source,
            &self.cache_policy,
        );
    }
}

impl PtxUnparser for VectorAdd32 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_atom_base(tokens, self.semantics, self.scope);
        push_vector_state_space(tokens, self.state_space);
        push_directive(tokens, "add");
        if let Some(cache_hint) = &self.cache_hint {
            cache_hint.unparse_tokens(tokens);
        }
        push_directive(tokens, "vec_32_bit");
        push_directive(tokens, "f32");
        push_vector_operands32(
            tokens,
            &self.destination,
            &self.address,
            &self.source,
            &self.cache_policy,
        );
    }
}

impl PtxUnparser for VectorHalf {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_atom_base(tokens, self.semantics, self.scope);
        push_vector_state_space(tokens, self.state_space);
        self.operation.unparse_tokens(tokens);
        push_directive(tokens, "noftz");
        if let Some(cache_hint) = &self.cache_hint {
            cache_hint.unparse_tokens(tokens);
        }
        push_directive(tokens, "vec_16_bit");
        self.element_type.unparse_tokens(tokens);
        push_vector_operands16(
            tokens,
            &self.destination,
            &self.address,
            &self.source,
            &self.cache_policy,
        );
    }
}

impl PtxUnparser for VectorPacked {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_atom_base(tokens, self.semantics, self.scope);
        push_vector_state_space(tokens, self.state_space);
        self.operation.unparse_tokens(tokens);
        push_directive(tokens, "noftz");
        if let Some(cache_hint) = &self.cache_hint {
            cache_hint.unparse_tokens(tokens);
        }
        push_directive(tokens, "vec_32_bit");
        self.element_type.unparse_tokens(tokens);
        push_vector_operands32(
            tokens,
            &self.destination,
            &self.address,
            &self.source,
            &self.cache_policy,
        );
    }
}

impl PtxUnparser for Atom {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Atom::Scalar(instruction) => instruction.unparse_tokens(tokens),
            Atom::CompareSwap(instruction) => instruction.unparse_tokens(tokens),
            Atom::Exchange128(instruction) => instruction.unparse_tokens(tokens),
            Atom::AddNoFtz(instruction) => instruction.unparse_tokens(tokens),
            Atom::VectorAdd32(instruction) => instruction.unparse_tokens(tokens),
            Atom::VectorHalf(instruction) => instruction.unparse_tokens(tokens),
            Atom::VectorPacked(instruction) => instruction.unparse_tokens(tokens),
        }
    }
}
