use crate::{
    lexer::PtxToken,
    r#type::instruction::st::{
        CacheHint, CacheOperator, DataType, Eviction, Generic, Level1EvictionPriority,
        Level2EvictionPriority, Mmio, MmioStateSpace, ParamState, Relaxed, Release, Scope,
        ScopedStateSpace, SharedState, Source, St, StateSpace, Vector, VectorElement,
        VectorElements, Volatile,
    },
    unparser::*,
};

impl PtxUnparser for CacheOperator {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            CacheOperator::Wb => "wb",
            CacheOperator::Cg => "cg",
            CacheOperator::Cs => "cs",
            CacheOperator::Wt => "wt",
        };
        push_directive(tokens, modifier);
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

impl PtxUnparser for Level1EvictionPriority {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            Level1EvictionPriority::EvictNormal => "evict_normal",
            Level1EvictionPriority::EvictUnchanged => "evict_unchanged",
            Level1EvictionPriority::EvictFirst => "evict_first",
            Level1EvictionPriority::EvictLast => "evict_last",
            Level1EvictionPriority::NoAllocate => "no_allocate",
        };
        push_directive(tokens, "L1");
        push_double_colon(tokens);
        push_identifier(tokens, name);
    }
}

impl PtxUnparser for Level2EvictionPriority {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            Level2EvictionPriority::EvictNormal => "evict_normal",
            Level2EvictionPriority::EvictFirst => "evict_first",
            Level2EvictionPriority::EvictLast => "evict_last",
        };
        push_directive(tokens, "L2");
        push_double_colon(tokens);
        push_identifier(tokens, name);
    }
}

impl PtxUnparser for ParamState {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            ParamState::Func => push_identifier(tokens, "func"),
        }
    }
}

impl PtxUnparser for SharedState {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            SharedState::Cta => "cta",
            SharedState::Cluster => "cluster",
        };
        push_identifier(tokens, name);
    }
}

impl PtxUnparser for StateSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            StateSpace::Global => push_directive(tokens, "global"),
            StateSpace::Local => push_directive(tokens, "local"),
            StateSpace::Param(param) => {
                push_directive(tokens, "param");
                push_double_colon(tokens);
                param.unparse_tokens(tokens);
            }
            StateSpace::Shared(shared) => {
                push_directive(tokens, "shared");
                push_double_colon(tokens);
                shared.unparse_tokens(tokens);
            }
        }
    }
}

impl PtxUnparser for ScopedStateSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            ScopedStateSpace::Global => push_directive(tokens, "global"),
            ScopedStateSpace::Shared(shared) => {
                push_directive(tokens, "shared");
                push_double_colon(tokens);
                shared.unparse_tokens(tokens);
            }
        }
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

impl PtxUnparser for MmioStateSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            MmioStateSpace::Global => push_directive(tokens, "global"),
        }
    }
}

impl PtxUnparser for Vector {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            Vector::V2 => "v2",
            Vector::V4 => "v4",
            Vector::V8 => "v8",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for VectorElement {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            VectorElement::Register(register) => register.unparse_tokens(tokens),
            VectorElement::Sink => push_identifier(tokens, "_"),
        }
    }
}

impl PtxUnparser for VectorElements {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::LBrace);
        match self {
            VectorElements::V2(elements) => {
                for (index, element) in elements.iter().enumerate() {
                    if index > 0 {
                        tokens.push(PtxToken::Comma);
                    }
                    element.unparse_tokens(tokens);
                }
            }
            VectorElements::V4(elements) => {
                for (index, element) in elements.iter().enumerate() {
                    if index > 0 {
                        tokens.push(PtxToken::Comma);
                    }
                    element.unparse_tokens(tokens);
                }
            }
            VectorElements::V8(elements) => {
                for (index, element) in elements.iter().enumerate() {
                    if index > 0 {
                        tokens.push(PtxToken::Comma);
                    }
                    element.unparse_tokens(tokens);
                }
            }
        }
        tokens.push(PtxToken::RBrace);
    }
}

impl PtxUnparser for Source {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Source::Register(register) => register.unparse_tokens(tokens),
            Source::Vector(elements) => elements.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for Generic {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "st");
        if self.weak {
            push_directive(tokens, "weak");
        }
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        if let Some(cache_operator) = &self.cache_operator {
            cache_operator.unparse_tokens(tokens);
        }
        if let Some(cache_hint) = &self.cache_hint {
            cache_hint.unparse_tokens(tokens);
        }
        if let Some(vector) = &self.vector {
            vector.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        if let Some(cache_policy) = &self.cache_policy {
            tokens.push(PtxToken::Comma);
            cache_policy.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Eviction {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "st");
        if self.weak {
            push_directive(tokens, "weak");
        }
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        if let Some(level1) = &self.level1 {
            level1.unparse_tokens(tokens);
        }
        if let Some(level2) = &self.level2 {
            level2.unparse_tokens(tokens);
        }
        if let Some(cache_hint) = &self.cache_hint {
            cache_hint.unparse_tokens(tokens);
        }
        if let Some(vector) = &self.vector {
            vector.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        if let Some(cache_policy) = &self.cache_policy {
            tokens.push(PtxToken::Comma);
            cache_policy.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Volatile {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "st");
        push_directive(tokens, "volatile");
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        if let Some(vector) = &self.vector {
            vector.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Relaxed {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "st");
        push_directive(tokens, "relaxed");
        self.scope.unparse_tokens(tokens);
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        if let Some(level1) = &self.level1 {
            level1.unparse_tokens(tokens);
        }
        if let Some(level2) = &self.level2 {
            level2.unparse_tokens(tokens);
        }
        if let Some(cache_hint) = &self.cache_hint {
            cache_hint.unparse_tokens(tokens);
        }
        if let Some(vector) = &self.vector {
            vector.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        if let Some(cache_policy) = &self.cache_policy {
            tokens.push(PtxToken::Comma);
            cache_policy.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Release {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "st");
        push_directive(tokens, "release");
        self.scope.unparse_tokens(tokens);
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        if let Some(level1) = &self.level1 {
            level1.unparse_tokens(tokens);
        }
        if let Some(level2) = &self.level2 {
            level2.unparse_tokens(tokens);
        }
        if let Some(cache_hint) = &self.cache_hint {
            cache_hint.unparse_tokens(tokens);
        }
        if let Some(vector) = &self.vector {
            vector.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        if let Some(cache_policy) = &self.cache_policy {
            tokens.push(PtxToken::Comma);
            cache_policy.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Mmio {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "st");
        push_directive(tokens, "mmio");
        push_directive(tokens, "relaxed");
        push_directive(tokens, "sys");
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for St {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            St::Generic(instruction) => instruction.unparse_tokens(tokens),
            St::Eviction(instruction) => instruction.unparse_tokens(tokens),
            St::Volatile(instruction) => instruction.unparse_tokens(tokens),
            St::Relaxed(instruction) => instruction.unparse_tokens(tokens),
            St::Release(instruction) => instruction.unparse_tokens(tokens),
            St::Mmio(instruction) => instruction.unparse_tokens(tokens),
        }
    }
}
