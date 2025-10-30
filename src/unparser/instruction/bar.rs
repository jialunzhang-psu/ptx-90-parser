use crate::{lexer::PtxToken, r#type::instruction::bar::*, unparser::*};

fn push_optional_aligned(tokens: &mut Vec<PtxToken>, aligned: bool) {
    if aligned {
        push_directive(tokens, "aligned");
    }
}

impl PtxUnparser for crate::r#type::instruction::bar::Scope {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        if let Scope::Cta = self {
            push_directive(tokens, "cta");
        }
    }
}

impl PtxUnparser for crate::r#type::instruction::bar::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "u32");
    }
}

impl PtxUnparser for crate::r#type::instruction::bar::LogicalOperation {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            LogicalOperation::And => "and",
            LogicalOperation::Or => "or",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for crate::r#type::instruction::bar::PredicateInput {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        if self.negated {
            tokens.push(PtxToken::Exclaim);
        }
        self.predicate.unparse_tokens(tokens);
    }
}

impl PtxUnparser for crate::r#type::instruction::bar::BarrierSync {
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

impl PtxUnparser for crate::r#type::instruction::bar::BarrierArrive {
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

impl PtxUnparser for crate::r#type::instruction::bar::BarrierReductionPopc {
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

impl PtxUnparser for crate::r#type::instruction::bar::BarrierReductionLogical {
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

impl PtxUnparser for crate::r#type::instruction::bar::BarSync {
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

impl PtxUnparser for crate::r#type::instruction::bar::BarArrive {
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

impl PtxUnparser for crate::r#type::instruction::bar::BarReductionPopc {
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

impl PtxUnparser for crate::r#type::instruction::bar::BarReductionLogical {
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

impl PtxUnparser for crate::r#type::instruction::bar::Bar {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Bar::BarrierSync(inst) => inst.unparse_tokens(tokens),
            Bar::BarrierArrive(inst) => inst.unparse_tokens(tokens),
            Bar::BarrierReductionPopc(inst) => inst.unparse_tokens(tokens),
            Bar::BarrierReductionLogical(inst) => inst.unparse_tokens(tokens),
            Bar::BarSync(inst) => inst.unparse_tokens(tokens),
            Bar::BarArrive(inst) => inst.unparse_tokens(tokens),
            Bar::BarReductionPopc(inst) => inst.unparse_tokens(tokens),
            Bar::BarReductionLogical(inst) => inst.unparse_tokens(tokens),
        }
    }
}
