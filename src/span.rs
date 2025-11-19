use crate::parser::Span;

/// Trait implemented by AST nodes that carry a source span.
pub trait Spanned: Sized {
    /// Returns the stored span for this node.
    fn span(&self) -> Span;

    /// Mutates the stored span in-place.
    fn set_span(&mut self, span: Span);

    /// Convenience helper that sets the span and returns `self`.
    fn with_span(mut self, span: Span) -> Self {
        self.set_span(span);
        self
    }
}

#[macro_export]
macro_rules! span {
    ($range:expr) => {{
        let r = $range;
        $crate::Span::new(r.start, r.end)
    }};
}
