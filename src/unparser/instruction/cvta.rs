use crate::{lexer::PtxToken, r#type::instruction::cvta::*, unparser::*};

impl PtxUnparser for Space {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Space::Const => push_directive(tokens, "const"),
            Space::Global => push_directive(tokens, "global"),
            Space::Local => push_directive(tokens, "local"),
            Space::Shared => push_directive(tokens, "shared"),
            Space::SharedCta => {
                push_directive(tokens, "shared");
                tokens.push(PtxToken::Colon);
                tokens.push(PtxToken::Colon);
                push_identifier(tokens, "cta");
            }
            Space::SharedCluster => {
                push_directive(tokens, "shared");
                tokens.push(PtxToken::Colon);
                tokens.push(PtxToken::Colon);
                push_identifier(tokens, "cluster");
            }
            Space::Param => push_directive(tokens, "param"),
            Space::ParamEntry => {
                push_directive(tokens, "param");
                tokens.push(PtxToken::Colon);
                tokens.push(PtxToken::Colon);
                push_identifier(tokens, "entry");
            }
        }
    }
}

impl PtxUnparser for Size {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Size::U32 => push_directive(tokens, "u32"),
            Size::U64 => push_directive(tokens, "u64"),
        }
    }
}

impl PtxUnparser for GenericSource {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            GenericSource::Register(register) => register.unparse_tokens(tokens),
            GenericSource::Variable(variable) => variable.unparse_tokens(tokens),
            GenericSource::VariableWithImmediate {
                variable,
                immediate,
            } => {
                variable.unparse_tokens(tokens);
                tokens.push(PtxToken::Plus);
                immediate.unparse_tokens(tokens);
            }
        }
    }
}

impl PtxUnparser for ToGeneric {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "cvta");
        self.space.unparse_tokens(tokens);
        self.size.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for ToAddressSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "cvta");
        push_directive(tokens, "to");
        self.space.unparse_tokens(tokens);
        self.size.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Cvta {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Cvta::ToGeneric(instruction) => instruction.unparse_tokens(tokens),
            Cvta::ToAddressSpace(instruction) => instruction.unparse_tokens(tokens),
        }
    }
}
