use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::cp::*},
    unparser::*,
};

fn push_cp_prefix(
    tokens: &mut Vec<PtxToken>,
    cache_operator: &str,
    shared_space: &CpSharedSpace,
    cache_hint: Option<&CpCacheHint>,
    prefetch_size: Option<&CpPrefetchSize>,
) {
    push_identifier(tokens, "cp");
    push_directive(tokens, "async");
    push_directive(tokens, cache_operator);
    shared_space.unparse_tokens(tokens);
    push_directive(tokens, "global");
    if let Some(cache_hint) = cache_hint {
        cache_hint.unparse_tokens(tokens);
    }
    if let Some(prefetch_size) = prefetch_size {
        prefetch_size.unparse_tokens(tokens);
    }
}

fn push_operands_start(
    tokens: &mut Vec<PtxToken>,
    destination: &AddressOperand,
    source: &AddressOperand,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    source.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
}

impl PtxUnparser for CpSharedSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "shared");
        if matches!(self, CpSharedSpace::Cta) {
            push_double_colon(tokens);
            push_identifier(tokens, "cta");
        }
    }
}

impl PtxUnparser for CpCacheHint {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            CpCacheHint::L2CacheHint => {
                push_directive(tokens, "L2");
                push_double_colon(tokens);
                push_identifier(tokens, "cache_hint");
            }
        }
    }
}

impl PtxUnparser for CpPrefetchSize {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let (value, suffix) = match self {
            CpPrefetchSize::L264B => ("64", "B"),
            CpPrefetchSize::L2128B => ("128", "B"),
            CpPrefetchSize::L2256B => ("256", "B"),
        };
        push_directive(tokens, "L2");
        push_double_colon(tokens);
        push_decimal(tokens, value);
        push_identifier(tokens, suffix);
    }
}

impl PtxUnparser for CpCopySize {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let value = match self {
            CpCopySize::Bytes4 => "4",
            CpCopySize::Bytes8 => "8",
            CpCopySize::Bytes16 => "16",
        };
        push_decimal(tokens, value);
    }
}

impl PtxUnparser for CpAsyncCaWithSrcSize {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_cp_prefix(
            tokens,
            "ca",
            &self.shared_space,
            self.cache_hint.as_ref(),
            self.prefetch_size.as_ref(),
        );
        push_operands_start(tokens, &self.destination, &self.source);
        self.copy_size.unparse_tokens(tokens);
        if let Some(source_size) = &self.source_size {
            tokens.push(PtxToken::Comma);
            source_size.unparse_tokens(tokens);
        }
        if let Some(cache_policy) = &self.cache_policy {
            tokens.push(PtxToken::Comma);
            cache_policy.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for CpAsyncCgWithSrcSize {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_cp_prefix(
            tokens,
            "cg",
            &self.shared_space,
            self.cache_hint.as_ref(),
            self.prefetch_size.as_ref(),
        );
        push_operands_start(tokens, &self.destination, &self.source);
        CpCopySize::Bytes16.unparse_tokens(tokens);
        if let Some(source_size) = &self.source_size {
            tokens.push(PtxToken::Comma);
            source_size.unparse_tokens(tokens);
        }
        if let Some(cache_policy) = &self.cache_policy {
            tokens.push(PtxToken::Comma);
            cache_policy.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for CpAsyncCaIgnoreSrc {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_cp_prefix(
            tokens,
            "ca",
            &self.shared_space,
            self.cache_hint.as_ref(),
            self.prefetch_size.as_ref(),
        );
        push_operands_start(tokens, &self.destination, &self.source);
        self.copy_size.unparse_tokens(tokens);
        if let Some(ignore_src) = &self.ignore_src {
            tokens.push(PtxToken::Comma);
            ignore_src.unparse_tokens(tokens);
        }
        if let Some(cache_policy) = &self.cache_policy {
            tokens.push(PtxToken::Comma);
            cache_policy.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for CpAsyncCgIgnoreSrc {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_cp_prefix(
            tokens,
            "cg",
            &self.shared_space,
            self.cache_hint.as_ref(),
            self.prefetch_size.as_ref(),
        );
        push_operands_start(tokens, &self.destination, &self.source);
        CpCopySize::Bytes16.unparse_tokens(tokens);
        if let Some(ignore_src) = &self.ignore_src {
            tokens.push(PtxToken::Comma);
            ignore_src.unparse_tokens(tokens);
        }
        if let Some(cache_policy) = &self.cache_policy {
            tokens.push(PtxToken::Comma);
            cache_policy.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for CpOpcode {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            CpOpcode::AsyncCaWithSrcSize(instruction) => instruction.unparse_tokens(tokens),
            CpOpcode::AsyncCgWithSrcSize(instruction) => instruction.unparse_tokens(tokens),
            CpOpcode::AsyncCaIgnoreSrc(instruction) => instruction.unparse_tokens(tokens),
            CpOpcode::AsyncCgIgnoreSrc(instruction) => instruction.unparse_tokens(tokens),
        }
    }
}
