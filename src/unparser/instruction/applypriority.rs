use crate::{lexer::PtxToken, r#type::instruction::applypriority::*, unparser::*};

impl PtxUnparser for EvictionPriority {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            EvictionPriority::L2EvictNormal => {
                push_directive(tokens, "L2");
                push_double_colon(tokens);
                push_identifier(tokens, "evict_normal");
            }
        }
    }
}

impl PtxUnparser for Size {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Size::B128 => tokens.push(PtxToken::DecimalInteger("128".to_string())),
        }
    }
}

impl PtxUnparser for Applypriority {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "applypriority");
        if self.global {
            push_directive(tokens, "global");
        }
        self.eviction_priority.unparse_tokens(tokens);
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.size.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
