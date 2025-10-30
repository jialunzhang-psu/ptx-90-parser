use crate::{lexer::PtxToken, r#type::instruction::ld::*, unparser::*};

impl PtxUnparser for CacheOperator {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            CacheOperator::Ca => "ca",
            CacheOperator::Cg => "cg",
            CacheOperator::Cs => "cs",
            CacheOperator::Lu => "lu",
            CacheOperator::Cv => "cv",
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

impl PtxUnparser for PrefetchSize {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let (value, suffix) = match self {
            PrefetchSize::L264B => ("64", "B"),
            PrefetchSize::L2128B => ("128", "B"),
            PrefetchSize::L2256B => ("256", "B"),
        };
        push_directive(tokens, "L2");
        push_double_colon(tokens);
        tokens.push(PtxToken::DecimalInteger(value.to_string()));
        push_identifier(tokens, suffix);
    }
}

impl PtxUnparser for Level1EvictionPriority {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let value = match self {
            Level1EvictionPriority::EvictNormal => "evict_normal",
            Level1EvictionPriority::EvictUnchanged => "evict_unchanged",
            Level1EvictionPriority::EvictFirst => "evict_first",
            Level1EvictionPriority::EvictLast => "evict_last",
            Level1EvictionPriority::NoAllocate => "no_allocate",
        };
        push_directive(tokens, "L1");
        push_double_colon(tokens);
        push_identifier(tokens, value);
    }
}

impl PtxUnparser for Level2EvictionPriority {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let value = match self {
            Level2EvictionPriority::EvictNormal => "evict_normal",
            Level2EvictionPriority::EvictFirst => "evict_first",
            Level2EvictionPriority::EvictLast => "evict_last",
        };
        push_directive(tokens, "L2");
        push_double_colon(tokens);
        push_identifier(tokens, value);
    }
}

impl PtxUnparser for ParamState {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            ParamState::Entry => "entry",
            ParamState::Func => "func",
        };
        push_identifier(tokens, name);
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
            StateSpace::Const => push_directive(tokens, "const"),
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

impl PtxUnparser for MmioStateSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            MmioStateSpace::Global => push_directive(tokens, "global"),
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

impl PtxUnparser for Vector {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Vector::V2 => "v2",
            Vector::V4 => "v4",
            Vector::V8 => "v8",
        };
        push_directive(tokens, directive);
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

impl PtxUnparser for DestinationElement {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            DestinationElement::Register(register) => register.unparse_tokens(tokens),
            DestinationElement::Sink => push_identifier(tokens, "_"),
        }
    }
}

fn push_destination_elements(tokens: &mut Vec<PtxToken>, elements: &[DestinationElement]) {
    tokens.push(PtxToken::LBrace);
    for (index, element) in elements.iter().enumerate() {
        if index > 0 {
            tokens.push(PtxToken::Comma);
        }
        element.unparse_tokens(tokens);
    }
    tokens.push(PtxToken::RBrace);
}

impl PtxUnparser for DestinationElements {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            DestinationElements::V2(elements) => push_destination_elements(tokens, elements),
            DestinationElements::V4(elements) => push_destination_elements(tokens, elements),
            DestinationElements::V8(elements) => push_destination_elements(tokens, elements),
        }
    }
}

impl PtxUnparser for Destination {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Destination::Scalar(register) => register.unparse_tokens(tokens),
            Destination::Vector(elements) => elements.unparse_tokens(tokens),
        }
    }
}

fn unparse_common_vector(tokens: &mut Vec<PtxToken>, vector: &Option<Vector>) {
    if let Some(vector) = vector {
        vector.unparse_tokens(tokens);
    }
}

fn unparse_common_cache_hint(tokens: &mut Vec<PtxToken>, cache_hint: &Option<CacheHint>) {
    if let Some(cache_hint) = cache_hint {
        cache_hint.unparse_tokens(tokens);
    }
}

fn unparse_common_prefetch(tokens: &mut Vec<PtxToken>, prefetch_size: &Option<PrefetchSize>) {
    if let Some(prefetch_size) = prefetch_size {
        prefetch_size.unparse_tokens(tokens);
    }
}

impl PtxUnparser for Generic {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "ld");
        if self.weak {
            push_directive(tokens, "weak");
        }
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        if let Some(cache_operator) = &self.cache_operator {
            cache_operator.unparse_tokens(tokens);
        }
        unparse_common_cache_hint(tokens, &self.cache_hint);
        unparse_common_prefetch(tokens, &self.prefetch_size);
        unparse_common_vector(tokens, &self.vector);
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.address.unparse_tokens(tokens);
        if self.unified {
            push_directive(tokens, "unified");
        }
        if let Some(cache_policy) = &self.cache_policy {
            tokens.push(PtxToken::Comma);
            cache_policy.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Eviction {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "ld");
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
        unparse_common_cache_hint(tokens, &self.cache_hint);
        unparse_common_prefetch(tokens, &self.prefetch_size);
        unparse_common_vector(tokens, &self.vector);
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.address.unparse_tokens(tokens);
        if self.unified {
            push_directive(tokens, "unified");
        }
        if let Some(cache_policy) = &self.cache_policy {
            tokens.push(PtxToken::Comma);
            cache_policy.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Volatile {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "ld");
        push_directive(tokens, "volatile");
        if let Some(state_space) = &self.state_space {
            state_space.unparse_tokens(tokens);
        }
        unparse_common_prefetch(tokens, &self.prefetch_size);
        unparse_common_vector(tokens, &self.vector);
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

fn unparse_scoped(tokens: &mut Vec<PtxToken>, directive: &str, instruction: &Scoped) {
    push_opcode(tokens, "ld");
    push_directive(tokens, directive);
    instruction.scope.unparse_tokens(tokens);
    if let Some(state_space) = &instruction.state_space {
        state_space.unparse_tokens(tokens);
    }
    if let Some(level1) = &instruction.level1 {
        level1.unparse_tokens(tokens);
    }
    if let Some(level2) = &instruction.level2 {
        level2.unparse_tokens(tokens);
    }
    unparse_common_cache_hint(tokens, &instruction.cache_hint);
    unparse_common_prefetch(tokens, &instruction.prefetch_size);
    unparse_common_vector(tokens, &instruction.vector);
    instruction.data_type.unparse_tokens(tokens);
    instruction.destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    instruction.address.unparse_tokens(tokens);
    if let Some(cache_policy) = &instruction.cache_policy {
        tokens.push(PtxToken::Comma);
        cache_policy.unparse_tokens(tokens);
    }
    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for Mmio {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "ld");
        push_directive(tokens, "mmio");
        push_directive(tokens, "relaxed");
        push_directive(tokens, "sys");
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

impl PtxUnparser for Ld {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Ld::Generic(instruction) => instruction.unparse_tokens(tokens),
            Ld::Eviction(instruction) => instruction.unparse_tokens(tokens),
            Ld::Volatile(instruction) => instruction.unparse_tokens(tokens),
            Ld::Relaxed(instruction) => unparse_scoped(tokens, "relaxed", instruction),
            Ld::Acquire(instruction) => unparse_scoped(tokens, "acquire", instruction),
            Ld::Mmio(instruction) => instruction.unparse_tokens(tokens),
        }
    }
}
