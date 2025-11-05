pub mod common;
pub mod function;
pub mod instruction;
pub mod module;
pub mod variable;

use crate::lexer::PtxToken;

#[allow(unused_imports)]
pub(crate) use common::{
    push_decimal, push_directive, push_identifier, push_opcode, push_register, push_token_from_str,
};

/// Trait that mirrors [`crate::parser::PtxParser`] but for emitting PTX source
/// text from the structured representation.
pub trait PtxUnparser {
    /// Append the PTX token sequence representing `self` to `tokens`.
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>);

    /// Convenience helper that returns the serialized PTX token stream.
    fn to_tokens(&self) -> Vec<PtxToken> {
        let mut tokens = Vec::new();
        self.unparse_tokens(&mut tokens);
        tokens
    }
}
