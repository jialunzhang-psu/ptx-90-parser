use crate::{lexer::PtxToken, r#type::instruction::barrier::*, unparser::*};

fn push_optional_aligned(tokens: &mut Vec<PtxToken>, aligned: bool) {
    if aligned {
        push_directive(tokens, "aligned");
    }
}

impl PtxUnparser for Scope {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        if let Scope::Cta = self {
            push_directive(tokens, "cta");
        }
    }
}

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "u32");
    }
}

impl PtxUnparser for LogicalOperation {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            LogicalOperation::And => "and",
            LogicalOperation::Or => "or",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for PredicateInput {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        if self.negated {
            tokens.push(PtxToken::Exclaim);
        }
        self.predicate.unparse_tokens(tokens);
    }
}

impl PtxUnparser for BarrierSync {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "barrier");
        self.scope.unparse_tokens(tokens);
        push_directive(tokens, "sync");
        push_optional_aligned(tokens, self.aligned);
        self.barrier.unparse_tokens(tokens);
        if let Some(expected_count) = &self.expected_count {
            tokens.push(PtxToken::Comma);
            expected_count.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for BarrierArrive {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "barrier");
        self.scope.unparse_tokens(tokens);
        push_directive(tokens, "arrive");
        push_optional_aligned(tokens, self.aligned);
        self.barrier.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.expected_count.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for BarrierReductionPopc {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "barrier");
        self.scope.unparse_tokens(tokens);
        push_directive(tokens, "red");
        push_directive(tokens, "popc");
        push_optional_aligned(tokens, self.aligned);
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.barrier.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        if let Some(expected_count) = &self.expected_count {
            expected_count.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
        }
        self.predicate.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for BarrierReductionLogical {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "barrier");
        self.scope.unparse_tokens(tokens);
        push_directive(tokens, "red");
        self.operation.unparse_tokens(tokens);
        push_optional_aligned(tokens, self.aligned);
        push_directive(tokens, "pred");
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.barrier.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        if let Some(expected_count) = &self.expected_count {
            expected_count.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
        }
        self.predicate.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for BarSync {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "bar");
        self.scope.unparse_tokens(tokens);
        push_directive(tokens, "sync");
        self.barrier.unparse_tokens(tokens);
        if let Some(expected_count) = &self.expected_count {
            tokens.push(PtxToken::Comma);
            expected_count.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for BarArrive {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "bar");
        self.scope.unparse_tokens(tokens);
        push_directive(tokens, "arrive");
        self.barrier.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.expected_count.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for BarReductionPopc {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "bar");
        self.scope.unparse_tokens(tokens);
        push_directive(tokens, "red");
        push_directive(tokens, "popc");
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.barrier.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        if let Some(expected_count) = &self.expected_count {
            expected_count.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
        }
        self.predicate.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for BarReductionLogical {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "bar");
        self.scope.unparse_tokens(tokens);
        push_directive(tokens, "red");
        self.operation.unparse_tokens(tokens);
        push_directive(tokens, "pred");
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.barrier.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        if let Some(expected_count) = &self.expected_count {
            expected_count.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
        }
        self.predicate.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Barrier {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Barrier::Sync(inst) => inst.unparse_tokens(tokens),
            Barrier::Arrive(inst) => inst.unparse_tokens(tokens),
            Barrier::ReductionPopc(inst) => inst.unparse_tokens(tokens),
            Barrier::ReductionLogical(inst) => inst.unparse_tokens(tokens),
            Barrier::BarSync(inst) => inst.unparse_tokens(tokens),
            Barrier::BarArrive(inst) => inst.unparse_tokens(tokens),
            Barrier::BarReductionPopc(inst) => inst.unparse_tokens(tokens),
            Barrier::BarReductionLogical(inst) => inst.unparse_tokens(tokens),
        }
    }
}
