use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::prefetch::*},
    unparser::*,
};

fn unparse_address(tokens: &mut Vec<PtxToken>, address: &AddressOperand) {
    address.unparse_tokens(tokens);
}

impl PtxUnparser for Space {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Space::Global => push_directive(tokens, "global"),
            Space::Local => push_directive(tokens, "local"),
        }
    }
}

impl PtxUnparser for Level {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Level::L1 => push_directive(tokens, "L1"),
            Level::L2 => push_directive(tokens, "L2"),
        }
    }
}

impl PtxUnparser for Eviction {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Eviction::L2EvictLast => push_identifier(tokens, "evict_last"),
            Eviction::L2EvictNormal => push_identifier(tokens, "evict_normal"),
        }
    }
}

impl PtxUnparser for TensorMapSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            TensorMapSpace::Const => push_directive(tokens, "const"),
            TensorMapSpace::Param => push_directive(tokens, "param"),
        }
    }
}

impl PtxUnparser for Prefetch {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Prefetch::DataCache {
                space,
                level,
                address,
            } => {
                push_identifier(tokens, "prefetch");
                if let Some(space) = space {
                    space.unparse_tokens(tokens);
                }
                level.unparse_tokens(tokens);
                unparse_address(tokens, address);
                tokens.push(PtxToken::Semicolon);
            }
            Prefetch::GlobalEviction { eviction, address } => {
                push_identifier(tokens, "prefetch");
                push_directive(tokens, "global");
                push_directive(tokens, "L2");
                push_double_colon(tokens);
                eviction.unparse_tokens(tokens);
                unparse_address(tokens, address);
                tokens.push(PtxToken::Semicolon);
            }
            Prefetch::Uniform { address } => {
                push_identifier(tokens, "prefetchu");
                Level::L1.unparse_tokens(tokens);
                unparse_address(tokens, address);
                tokens.push(PtxToken::Semicolon);
            }
            Prefetch::TensorMap { space, address } => {
                push_identifier(tokens, "prefetch");
                if let Some(space) = space {
                    space.unparse_tokens(tokens);
                }
                push_directive(tokens, "tensormap");
                unparse_address(tokens, address);
                tokens.push(PtxToken::Semicolon);
            }
        }
    }
}
