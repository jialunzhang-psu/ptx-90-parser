pub mod common;

use crate::lexer::PtxToken;

/// Trait that mirrors [`crate::parser::PtxParser`] but for emitting PTX source
/// text from the structured representation.
///
/// Types that implement this trait can serialize themselves back into PTX by
/// emitting a stream of [`PtxToken`] values. Implementations should push tokens
/// in the same order they would be produced by the lexer to ensure
/// round-trippable parsing.
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
