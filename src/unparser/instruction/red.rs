use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::red::*},
    unparser::*,
};

fn unparse_register_list(tokens: &mut Vec<PtxToken>, registers: &[RegisterOperand]) {
    tokens.push(PtxToken::LBrace);
    for (index, register) in registers.iter().enumerate() {
        if index > 0 {
            tokens.push(PtxToken::Comma);
        }
        register.unparse_tokens(tokens);
    }
    tokens.push(PtxToken::RBrace);
}

fn push_common_prefix(
    tokens: &mut Vec<PtxToken>,
    semantics: Option<Semantics>,
    scope: Option<Scope>,
) {
    if let Some(semantics) = semantics {
        semantics.unparse_tokens(tokens);
    }
    if let Some(scope) = scope {
        scope.unparse_tokens(tokens);
    }
}

fn push_scalar_state_space(tokens: &mut Vec<PtxToken>, state_space: Option<StateSpace>) {
    if let Some(state_space) = state_space {
        state_space.unparse_tokens(tokens);
    }
}

fn push_vector_state_space(tokens: &mut Vec<PtxToken>, state_space: Option<VectorStateSpace>) {
    if let Some(state_space) = state_space {
        state_space.unparse_tokens(tokens);
    }
}

fn push_operands<T: PtxUnparser>(
    tokens: &mut Vec<PtxToken>,
    address: &AddressOperand,
    source: &T,
    cache_policy: Option<&RegisterOperand>,
) {
    address.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    source.unparse_tokens(tokens);
    if let Some(cache_policy) = cache_policy {
        tokens.push(PtxToken::Comma);
        cache_policy.unparse_tokens(tokens);
    }
    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for Semantics {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Semantics::Relaxed => "relaxed",
            Semantics::Release => "release",
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
            StateSpace::Shared(SharedSpace::Default) => push_directive(tokens, "shared"),
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

impl PtxUnparser for ScalarOperation {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            ScalarOperation::And => "and",
            ScalarOperation::Or => "or",
            ScalarOperation::Xor => "xor",
            ScalarOperation::Add => "add",
            ScalarOperation::Inc => "inc",
            ScalarOperation::Dec => "dec",
            ScalarOperation::Min => "min",
            ScalarOperation::Max => "max",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for ScalarType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            ScalarType::B32 => "b32",
            ScalarType::B64 => "b64",
            ScalarType::U32 => "u32",
            ScalarType::U64 => "u64",
            ScalarType::S32 => "s32",
            ScalarType::S64 => "s64",
            ScalarType::F32 => "f32",
            ScalarType::F64 => "f64",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for ScalarAddNoFtzType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            ScalarAddNoFtzType::F16 => "f16",
            ScalarAddNoFtzType::F16x2 => "f16x2",
            ScalarAddNoFtzType::Bf16 => "bf16",
            ScalarAddNoFtzType::Bf16x2 => "bf16x2",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Vec32ElementType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Vec32ElementType::F32 => push_directive(tokens, "f32"),
        }
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
            Vec16Registers::V2(registers) => unparse_register_list(tokens, registers),
            Vec16Registers::V4(registers) => unparse_register_list(tokens, registers),
            Vec16Registers::V8(registers) => unparse_register_list(tokens, registers),
        }
    }
}

impl PtxUnparser for Vec32Registers {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Vec32Registers::V2(registers) => unparse_register_list(tokens, registers),
            Vec32Registers::V4(registers) => unparse_register_list(tokens, registers),
        }
    }
}

impl PtxUnparser for Scalar {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_common_prefix(tokens, self.semantics, self.scope);
        push_scalar_state_space(tokens, self.state_space);
        self.operation.unparse_tokens(tokens);
        if let Some(cache_hint) = self.cache_hint {
            cache_hint.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        push_operands(
            tokens,
            &self.address,
            &self.source,
            self.cache_policy.as_ref(),
        );
    }
}

impl PtxUnparser for ScalarAddNoFtz {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_common_prefix(tokens, self.semantics, self.scope);
        push_scalar_state_space(tokens, self.state_space);
        push_directive(tokens, "add");
        push_directive(tokens, "noftz");
        if let Some(cache_hint) = self.cache_hint {
            cache_hint.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        push_operands(
            tokens,
            &self.address,
            &self.source,
            self.cache_policy.as_ref(),
        );
    }
}

impl PtxUnparser for VectorAdd32 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_common_prefix(tokens, self.semantics, self.scope);
        push_vector_state_space(tokens, self.state_space);
        push_directive(tokens, "add");
        if let Some(cache_hint) = self.cache_hint {
            cache_hint.unparse_tokens(tokens);
        }
        push_directive(tokens, "vec_32_bit");
        self.element_type.unparse_tokens(tokens);
        push_operands(
            tokens,
            &self.address,
            &self.source,
            self.cache_policy.as_ref(),
        );
    }
}

impl PtxUnparser for VectorHalf {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_common_prefix(tokens, self.semantics, self.scope);
        push_vector_state_space(tokens, self.state_space);
        self.operation.unparse_tokens(tokens);
        push_directive(tokens, "noftz");
        if let Some(cache_hint) = self.cache_hint {
            cache_hint.unparse_tokens(tokens);
        }
        push_directive(tokens, "vec_16_bit");
        self.element_type.unparse_tokens(tokens);
        push_operands(
            tokens,
            &self.address,
            &self.source,
            self.cache_policy.as_ref(),
        );
    }
}

impl PtxUnparser for VectorPacked {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_common_prefix(tokens, self.semantics, self.scope);
        push_vector_state_space(tokens, self.state_space);
        self.operation.unparse_tokens(tokens);
        push_directive(tokens, "noftz");
        if let Some(cache_hint) = self.cache_hint {
            cache_hint.unparse_tokens(tokens);
        }
        push_directive(tokens, "vec_32_bit");
        self.element_type.unparse_tokens(tokens);
        push_operands(
            tokens,
            &self.address,
            &self.source,
            self.cache_policy.as_ref(),
        );
    }
}

impl PtxUnparser for RedOpcode {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "red");
        match self {
            RedOpcode::Scalar(scalar) => scalar.unparse_tokens(tokens),
            RedOpcode::ScalarAddNoFtz(scalar) => scalar.unparse_tokens(tokens),
            RedOpcode::VectorAdd32(vector) => vector.unparse_tokens(tokens),
            RedOpcode::VectorHalf(vector) => vector.unparse_tokens(tokens),
            RedOpcode::VectorPacked(vector) => vector.unparse_tokens(tokens),
        }
    }
}
