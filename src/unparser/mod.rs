pub(crate) mod common;
pub(crate) mod function;
pub(crate) mod instruction;
pub(crate) mod module;
pub(crate) mod variable;

use crate::lexer::PtxToken;

#[allow(unused_imports)]
pub(crate) use common::{
    push_decimal, push_directive, push_identifier, push_opcode, push_register, push_token_from_str,
};

pub(crate) fn push_space(tokens: &mut Vec<PtxToken>, spaced: bool) {
    if spaced {
        tokens.push(PtxToken::Space);
    }
}

pub(crate) fn push_newline(tokens: &mut Vec<PtxToken>, spaced: bool) {
    if spaced {
        tokens.push(PtxToken::Newline);
    }
}

/// Trait that mirrors [`crate::parser::PtxParser`] but for emitting PTX source
/// text from the structured representation.
pub trait PtxUnparser {
    /// Append the PTX token sequence representing `self` to `tokens`.
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>);

    /// Append tokens, optionally inserting spacing tokens for readability.
    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        let _ = spaced;
        self.unparse_tokens(tokens);
    }

    /// Convenience helper that returns the serialized PTX token stream.
    fn to_tokens(&self) -> Vec<PtxToken> {
        let mut tokens = Vec::new();
        self.unparse_tokens_mode(&mut tokens, false);
        tokens
    }

    /// Convenience helper that returns the serialized PTX token stream with
    /// spacing/newlines inserted for readability.
    fn to_tokens_spaced(&self) -> Vec<PtxToken> {
        let mut tokens = Vec::new();
        self.unparse_tokens_mode(&mut tokens, true);
        tokens
    }
}
