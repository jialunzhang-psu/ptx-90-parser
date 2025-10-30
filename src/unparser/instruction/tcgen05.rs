use crate::{lexer::PtxToken, r#type::instruction::tcgen05::*, unparser::*};

impl PtxUnparser for CtaGroup {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "cta_group");
        push_double_colon(tokens);
        let value = match self {
            CtaGroup::One => 1,
            CtaGroup::Two => 2,
        };
        push_decimal(tokens, value);
    }
}

impl PtxUnparser for StateSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            StateSpace::SharedCta => {
                push_directive(tokens, "shared");
                push_double_colon(tokens);
                push_identifier(tokens, "cta");
            }
        }
    }
}

impl PtxUnparser for Alloc {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "tcgen05");
        push_directive(tokens, "alloc");
        self.cta_group.unparse_tokens(tokens);
        push_directive(tokens, "sync");
        push_directive(tokens, "aligned");
        if let Some(state_space) = self.state_space {
            state_space.unparse_tokens(tokens);
        }
        push_directive(tokens, "b32");
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.column_count.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Dealloc {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "tcgen05");
        push_directive(tokens, "dealloc");
        self.cta_group.unparse_tokens(tokens);
        push_directive(tokens, "sync");
        push_directive(tokens, "aligned");
        push_directive(tokens, "b32");
        self.tensor_address.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.column_count.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for RelinquishAllocPermit {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "tcgen05");
        push_directive(tokens, "relinquish_alloc_permit");
        self.cta_group.unparse_tokens(tokens);
        push_directive(tokens, "sync");
        push_directive(tokens, "aligned");
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Tcgen05 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Tcgen05::Alloc(instruction) => instruction.unparse_tokens(tokens),
            Tcgen05::Dealloc(instruction) => instruction.unparse_tokens(tokens),
            Tcgen05::RelinquishAllocPermit(instruction) => instruction.unparse_tokens(tokens),
        }
    }
}
