use crate::{lexer::PtxToken, r#type::instruction::fns::*, unparser::*};

fn push_directive(directive: &str, tokens: &mut Vec<PtxToken>) {
    tokens.push(PtxToken::Directive(directive.to_string()));
}

impl PtxUnparser for Mask {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Self::B32(register) => {
                push_directive("b32", tokens);
                register.unparse_tokens(tokens);
            }
            Self::U32(register) => {
                push_directive("u32", tokens);
                register.unparse_tokens(tokens);
            }
            Self::S32(register) => {
                push_directive("s32", tokens);
                register.unparse_tokens(tokens);
            }
        }
    }
}

impl PtxUnparser for Base {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Self::B32(register) => {
                push_directive("b32", tokens);
                register.unparse_tokens(tokens);
            }
            Self::U32(register) => {
                push_directive("u32", tokens);
                register.unparse_tokens(tokens);
            }
            Self::S32(register) => {
                push_directive("s32", tokens);
                register.unparse_tokens(tokens);
            }
        }
    }
}

impl PtxUnparser for Fns {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("fns".to_string()));
        push_directive("b32", tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.mask.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.base.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.offset.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
