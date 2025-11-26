use crate::{LexError, lexer::PtxToken, span};
use thiserror::Error;
#[cfg(debug_assertions)]
use stacker;

pub(crate) mod common;
pub(crate) mod function;
pub(crate) mod instruction;
pub(crate) mod module;
pub(crate) mod util;
pub(crate) mod variable;

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl From<std::ops::Range<usize>> for Span {
    fn from(range: std::ops::Range<usize>) -> Self {
        Span::new(range.start, range.end)
    }
}

impl From<Span> for std::ops::Range<usize> {
    fn from(span: Span) -> Self {
        span.start..span.end
    }
}

/// Macro to create an UnexpectedToken error with expected and found values.
///
/// # Usage
/// ```ignore
/// unexpected_token!(span, ["expected1", "expected2"], "found_value")
/// unexpected_token!(span, vec!["expected1".to_string()], format!("{:?}", token))
/// ```
#[macro_export]
macro_rules! unexpected_token {
    ($span:expr, $expected:expr, $found:expr) => {
        $crate::parser::PtxParseError {
            kind: $crate::parser::ParseErrorKind::UnexpectedToken {
                expected: $expected.iter().map(|s| s.to_string()).collect(),
                found: $found,
            },
            span: $span,
        }
    };
}

/// Macro to check if in partial mode and return error if so.
/// Use this in token-based methods that should only work in complete mode.
///
/// # Usage
/// ```ignore
/// reject_partial_mode!(self);
/// ```
macro_rules! reject_partial_mode {
    ($self:expr) => {
        if $self.index.1.is_some() {
            let span = $self
                .tokens
                .get($self.index.0)
                .map_or(span!(0..0), |(_, s)| *s);
            return Err($crate::parser::PtxParseError {
                kind: $crate::parser::ParseErrorKind::InvalidModeForTokenMethod,
                span,
            });
        }
    };
}

/// Macro to create an UnexpectedToken error when no candidates match.
///
/// # Usage
/// ```ignore
/// no_candidate_match!(self, candidates)
/// ```
macro_rules! no_candidate_match {
    ($self:expr, $candidates:expr) => {{
        let span = $self
            .tokens
            .get($self.index.0)
            .map_or(span!(0..0), |(_, s)| *s);
        $crate::parser::PtxParseError {
            kind: $crate::parser::ParseErrorKind::UnexpectedToken {
                expected: $candidates.iter().map(|s| s.to_string()).collect(),
                found: "no match".to_string(),
            },
            span,
        }
    }};
}

/// Macro to build a standard unexpected-value parse error.
#[macro_export]
macro_rules! unexpected_value {
    ($span:expr, $expected:expr, $found:expr) => {
        $crate::parser::PtxParseError {
            kind: $crate::parser::ParseErrorKind::UnexpectedToken {
                expected: $expected.iter().map(|s| s.to_string()).collect(),
                found: $found.into(),
            },
            span: $span,
        }
    };
}

/// Kinds of parse errors that can occur during PTX parsing.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ParseErrorKind {
    #[error("unexpected token: expected one of {expected:?}, found {found}")]
    UnexpectedToken {
        expected: Vec<String>,
        found: String,
    },
    #[error("unexpected end of input")]
    UnexpectedEof,
    #[error("invalid literal: {0}")]
    InvalidLiteral(String),
    #[error("cannot use token-based methods in partial mode")]
    InvalidModeForTokenMethod,
}

/// PTX parsing error with location information.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("parsing error at {span:?}: {kind}")]
pub struct PtxParseError {
    pub kind: ParseErrorKind,
    pub span: Span,
}

impl From<LexError> for PtxParseError {
    fn from(err: LexError) -> Self {
        PtxParseError {
            kind: ParseErrorKind::InvalidLiteral("lexing failed".into()),
            span: err.span,
        }
    }
}

/// Represents a position in the token stream,
/// index of the token and optional char offset within the token.
pub type StreamPosition = (usize, Option<usize>);

/// Token stream wrapper for parsing PTX tokens.
///
/// This struct provides methods for consuming and inspecting tokens during parsing.
pub struct PtxTokenStream<'a> {
    tokens: &'a [(PtxToken, Span)],
    /// Current position (index) in the tokens list
    index: StreamPosition,
}

impl<'a> PtxTokenStream<'a> {
    pub fn new(tokens: &'a [(PtxToken, Span)]) -> Self {
        Self {
            tokens,
            index: (0, None),
        }
    }

    /// Peek at the next token without consuming it.
    ///
    /// # Behavior for complete mode
    ///
    /// Returns the token at the current stream position without advancing the position.
    /// This is a simple array lookup at `index.0`.
    ///
    /// # Behavior for partial mode
    ///
    /// Returns an error (InvalidModeForTokenMethod). This method only operates on whole tokens
    /// and cannot be used during partial (character-by-character) matching mode.
    ///
    /// # Returns
    ///
    /// - `Ok(&(PtxToken, Span))` - The token and its span
    /// - `Err(PtxParseError)` - If at end of stream (UnexpectedEof) or in partial mode (InvalidModeForTokenMethod)
    pub fn peek(&self) -> Result<&'a (PtxToken, Span), PtxParseError> {
        reject_partial_mode!(self);
        self.tokens.get(self.index.0).ok_or_else(|| {
            // If the stream is empty, return an EOF error
            let span = self.tokens.last().map_or(span!(0..0), |(_, s)| *s);
            PtxParseError {
                kind: ParseErrorKind::UnexpectedEof,
                span,
            }
        })
    }

    /// Peek at the token `offset` positions ahead without consuming it.
    ///
    /// Behaves like `peek()` but allows inspecting future tokens in complete mode.
    pub fn peek_n(&self, offset: usize) -> Result<&'a (PtxToken, Span), PtxParseError> {
        reject_partial_mode!(self);
        self.tokens.get(self.index.0 + offset).ok_or_else(|| {
            let span = self.tokens.last().map_or(span!(0..0), |(_, s)| *s);
            PtxParseError {
                kind: ParseErrorKind::UnexpectedEof,
                span,
            }
        })
    }

    /// Consume and return the next token.
    ///
    /// # Behavior for complete mode
    ///
    /// Advances the stream position by one token (increments `index.0`).
    /// Returns the token that was at the current position before advancing.
    ///
    /// # Behavior for partial mode
    ///
    /// Returns an error (InvalidModeForTokenMethod). This method only operates on whole tokens
    /// and cannot be used during partial (character-by-character) matching mode.
    ///
    /// # Returns
    ///
    /// - `Ok(&(PtxToken, Span))` - The consumed token and its span
    /// - `Err(PtxParseError)` - If at end of stream (UnexpectedEof) or in partial mode (InvalidModeForTokenMethod)
    pub fn consume(&mut self) -> Result<&'a (PtxToken, Span), PtxParseError> {
        reject_partial_mode!(self);
        let token = self.peek()?;
        self.index.0 += 1;
        Ok(token)
    }

    /// Conditionally consume the next token if it matches the predicate.
    ///
    /// # Returns
    ///
    /// - `Some(&(PtxToken, Span))` - If the predicate returns true, consumes and returns the token
    /// - `None` - If the predicate returns false or if at end of stream
    pub fn consume_if<F>(&mut self, predicate: F) -> Option<&'a (PtxToken, Span)>
    where
        F: FnOnce(&PtxToken) -> bool,
    {
        if self.index.1.is_some() {
            return None; // In partial mode
        }
        if let Ok((token, _)) = self.peek() {
            if predicate(token) {
                self.index.0 += 1;
                return self.tokens.get(self.index.0 - 1);
            }
        }
        None
    }

    /// Check if the next token is the expected type, and if so, consume it.
    /// Otherwise, return an error and do NOT consume the token.
    ///
    /// # Behavior for complete mode
    ///
    /// Peeks at the current token and checks if its discriminant (variant type) matches
    /// the expected token discriminant. If it matches, advances the stream by one token
    /// and returns the token. If it doesn't match, returns an UnexpectedToken error
    /// without consuming anything.
    ///
    /// # Behavior for partial mode
    ///
    /// Returns an error (InvalidModeForTokenMethod). This method only operates on whole tokens
    /// and cannot be used during partial (character-by-character) matching mode.
    ///
    /// # Returns
    ///
    /// - `Ok(&(PtxToken, Span))` - The matched and consumed token
    /// - `Err(PtxParseError)` - If token doesn't match (UnexpectedToken) or in partial mode (InvalidModeForTokenMethod)
    pub fn expect(&mut self, expected: &PtxToken) -> Result<&'a (PtxToken, Span), PtxParseError> {
        reject_partial_mode!(self);
        let token_pair = self.peek()?;
        let (token, span) = token_pair;
        if std::mem::discriminant(token) == std::mem::discriminant(expected) {
            self.index.0 += 1;
            Ok(token_pair)
        } else {
            Err(unexpected_token!(
                *span,
                &[format!("{:?}", expected)],
                format!("{:?}", token)
            ))
        }
    }

    /// Generic helper to extract a String value from a token variant.
    /// Returns the extracted string and span if the pattern matches, otherwise returns an error.
    ///
    /// # Behavior for complete mode
    ///
    /// Peeks at the current token and attempts to extract a string value using the provided
    /// extractor function. If extraction succeeds, advances the stream by one token and returns
    /// the extracted string with its span. If extraction fails, returns an UnexpectedToken error.
    ///
    /// # Behavior for partial mode
    ///
    /// Returns an error (InvalidModeForTokenMethod). This method only operates on whole tokens
    /// and cannot be used during partial (character-by-character) matching mode.
    ///
    /// # Returns
    ///
    /// - `Ok((String, Span))` - The extracted string value and its span
    /// - `Err(PtxParseError)` - If extraction fails (UnexpectedToken) or in partial mode (InvalidModeForTokenMethod)
    fn expect_token_with_string<F>(
        &mut self,
        expected_name: &str,
        extractor: F,
    ) -> Result<(String, Span), PtxParseError>
    where
        F: FnOnce(&PtxToken) -> Option<String>,
    {
        reject_partial_mode!(self);
        let (token, span_ref) = self.peek()?;
        if let Some(value) = extractor(token) {
            let span = *span_ref;
            self.index.0 += 1;
            Ok((value, span))
        } else {
            Err(unexpected_token!(
                *span_ref,
                &[expected_name.to_string()],
                format!("{:?}", token)
            ))
        }
    }

    /// Check if the next token is an identifier, and if so, consume it and return the String.
    ///
    /// # Behavior for complete mode
    ///
    /// Expects the current token to be an Identifier, consumes it, and returns its string value.
    ///
    /// # Behavior for partial mode
    ///
    /// Returns an error (InvalidModeForTokenMethod). This is a token-based method.
    pub fn expect_identifier(&mut self) -> Result<(String, Span), PtxParseError> {
        self.expect_token_with_string("Identifier", |token| {
            if let PtxToken::Identifier(name) = token {
                Some(name.clone())
            } else {
                None
            }
        })
    }

    /// Check if the next token is a register, and if so, consume it and return the String.
    ///
    /// # Behavior for complete mode
    ///
    /// Expects the current token to be a Register, consumes it, and returns its string value.
    ///
    /// # Behavior for partial mode
    ///
    /// Returns an error (InvalidModeForTokenMethod). This is a token-based method.
    pub fn expect_register(&mut self) -> Result<(String, Span), PtxParseError> {
        self.expect_token_with_string("Register", |token| {
            if let PtxToken::Register(name) = token {
                Some(name.clone())
            } else {
                None
            }
        })
    }

    /// Check if the next token is a directive (Dot + Identifier), and if so, consume them and return the String.
    ///
    /// # Behavior for complete mode
    ///
    /// Expects a Dot token followed by an Identifier token, consumes both, and returns the
    /// identifier string with a combined span covering both tokens.
    ///
    /// # Behavior for partial mode
    ///
    /// Returns an error (InvalidModeForTokenMethod). This is a token-based method.
    pub fn expect_directive(&mut self) -> Result<(String, Span), PtxParseError> {
        let (_, dot_span) = self.expect(&PtxToken::Dot)?;
        let (name, id_span) = self.expect_identifier()?;
        let span = Span::new(dot_span.start, id_span.end);
        Ok((name, span))
    }

    /// Internal helper to match a string pattern against the token stream.
    /// Returns true if the entire pattern matches and consumes the matched portion.
    /// Returns false if matching fails (does not modify stream state on failure).
    ///
    /// Supports both complete mode (whole token matching) and partial mode (char-by-char).
    fn match_string_internal(&mut self, pattern: &str) -> bool {
        let start_pos = self.position();
        let mut pattern_chars = pattern.chars().peekable();

        loop {
            // Check if we've consumed the entire pattern
            if pattern_chars.peek().is_none() {
                return true; // Successfully matched
            }

            // Check if we've run out of tokens
            if self.index.0 >= self.tokens.len() {
                self.set_position(start_pos);
                return false;
            }

            let (token, _span) = &self.tokens[self.index.0];
            let token_str = token.as_str();

            if let Some(char_offset) = self.index.1 {
                // Partial mode: match character-by-character
                let token_chars: Vec<char> = token_str.chars().collect();

                if char_offset >= token_chars.len() {
                    // Consumed entire token, advance to next
                    self.index.0 += 1;
                    self.index.1 = Some(0);
                    continue;
                }

                // Try to match remaining pattern chars against remaining token chars
                let mut offset = char_offset;
                while offset < token_chars.len() && pattern_chars.peek().is_some() {
                    if Some(&token_chars[offset]) == pattern_chars.peek() {
                        pattern_chars.next();
                        offset += 1;
                    } else {
                        // Mismatch
                        self.set_position(start_pos);
                        return false;
                    }
                }
                self.index.1 = Some(offset);
            } else {
                // Complete mode: match whole token string representation
                let token_chars: Vec<char> = token_str.chars().collect();
                let mut token_idx = 0;

                while token_idx < token_chars.len() && pattern_chars.peek().is_some() {
                    if Some(&token_chars[token_idx]) == pattern_chars.peek() {
                        pattern_chars.next();
                        token_idx += 1;
                    } else {
                        // Mismatch
                        self.set_position(start_pos);
                        return false;
                    }
                }

                // Check if we consumed the entire token
                if token_idx == token_chars.len() {
                    self.index.0 += 1;
                } else if pattern_chars.peek().is_none() {
                    // Pattern matched but didn't consume entire token - this is an error in complete mode
                    self.set_position(start_pos);
                    return false;
                }
            }
        }
    }

    /// Try to match and consume a sequence of tokens that matches one of the candidate strings.
    /// Returns the index of the matched candidate.
    ///
    /// This is used for parsing modifiers that may contain :: sequences like ".to::cluster"
    /// The candidates should include the leading dot (e.g., [".to::cluster", ".to::cta"])
    ///
    /// # Behavior for complete mode
    ///
    /// Tries to match each candidate string against the token stream by consuming whole tokens.
    /// Returns the index of the first candidate that matches. Uses backtracking (position/set_position)
    /// to try each candidate without consuming tokens on failed attempts.
    ///
    /// # Behavior for partial mode
    ///
    /// Supports character-by-character matching within tokens using the char offset.
    /// This allows matching patterns that span across token boundaries or within tokens.
    /// Uses backtracking to restore position when a candidate fails to match.
    pub fn expect_strings(&mut self, candidates: &[&str]) -> Result<usize, PtxParseError> {
        let start_pos = self.position();

        for (idx, candidate) in candidates.iter().enumerate() {
            self.set_position(start_pos);

            // Try to match this candidate
            if self.match_string_internal(candidate) {
                return Ok(idx);
            }
        }

        // None matched, restore position and create error
        self.set_position(start_pos);
        Err(no_candidate_match!(self, candidates))
    }

    /// Expect that the next sequence of tokens matches the given string pattern.
    ///
    /// # Behavior for complete mode
    ///
    /// Matches the pattern against the token stream by consuming whole tokens.
    /// Each token's string representation must match consecutive characters in the pattern.
    /// The match succeeds only if the entire pattern is consumed and tokens are fully consumed.
    ///
    /// # Behavior for partial mode
    ///
    /// Matches the pattern character-by-character against the token stream using the
    /// character offset for partial token matching. This allows matching patterns that
    /// don't align with token boundaries. If all characters match, the stream advances.
    /// If any character fails to match, the stream position is restored.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the entire pattern was successfully matched (consumed)
    /// - `Err(PtxParseError)` if matching failed (UnexpectedToken)
    pub fn expect_string(&mut self, expected: &str) -> Result<(), PtxParseError> {
        let start_pos = self.position();
        if self.match_string_internal(expected) {
            Ok(())
        } else {
            self.set_position(start_pos);
            Err(no_candidate_match!(self, &[expected]))
        }
    }

    /// Ensure we're in complete mode (not in partial token mode).
    /// This is a no-op in complete mode, and succeeds as long as we're not mid-token.
    /// Used by generated parsers to enforce token boundaries.
    pub fn expect_complete(&mut self) -> Result<(), PtxParseError> {
        if self.index.1.is_some() {
            let span = self
                .tokens
                .get(self.index.0)
                .map_or(span!(0..0), |(_, s)| *s);
            return Err(PtxParseError {
                kind: ParseErrorKind::InvalidModeForTokenMethod,
                span,
            });
        }
        Ok(())
    }

    /// Execute a function in partial token mode, enabling character-by-character matching.
    ///
    /// # Behavior
    ///
    /// This method switches the stream from complete mode to partial mode by setting the
    /// character offset to `Some(0)`. While in partial mode, string-based methods like
    /// `expect_string()` can match patterns character-by-character within tokens.
    ///
    /// After the closure completes:
    /// - If the char offset is non-zero, validates that the current token was fully consumed
    /// - If not fully consumed, reverts to the starting position and returns an error
    /// - Always resets the mode back to complete mode (sets `index.1` to `None`)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The closure returns an error
    /// - The token was partially consumed but not completely consumed (incomplete match)
    ///
    /// # Panics
    ///
    /// Panics if already in partial mode (char offset is already `Some`).
    pub fn with_partial_token_mode<F, R>(&mut self, f: F) -> Result<R, PtxParseError>
    where
        F: FnOnce(&mut PtxTokenStream) -> Result<R, PtxParseError>,
    {
        let start_index = self.index;
        assert!(self.index.1.is_none(), "Already in partial mode");
        self.index.1 = Some(0);
        let result = f(self);

        // Check if char offset has consumed the entire token
        if let Some(char_offset) = self.index.1 {
            if char_offset != 0 {
                // if consumed entire token, ok; else, reset position and error
                if let Some((token, span)) = self.tokens.get(self.index.0) {
                    if token.len() != char_offset {
                        self.index = start_index;
                        return Err(unexpected_token!(
                            *span,
                            &["fully consumed token".to_string()],
                            format!("partially consumed {:?}", token)
                        ));
                    } else {
                        // Token was fully consumed, advance to next token
                        self.index.0 += 1;
                    }
                }
            }
        }
        self.index.1 = None;
        result
    }

    /// Execute a closure with automatic backtracking and span tracking.
    ///
    /// Saves the current stream position before running `f`. If `f` returns an
    /// error, the stream position (including partial-mode offsets) is restored.
    /// When `f` succeeds, this returns the closure result together with the span
    /// covering the consumed source range.
    pub fn try_with_span<F, R>(&mut self, f: F) -> Result<(R, Span), PtxParseError>
    where
        F: FnOnce(&mut PtxTokenStream) -> Result<R, PtxParseError>,
    {
        let start_pos = self.position();
        match f(self) {
            Ok(value) => {
                let end_pos = self.position();
                let span_start = self.offset_from_start(start_pos);
                let span_end = self.offset_from_end(start_pos, end_pos).max(span_start);
                Ok((value, Span::new(span_start, span_end)))
            }
            Err(err) => {
                self.set_position(start_pos);
                Err(err)
            }
        }
    }

    /// Get the current position in the stream, for backtracking.
    ///
    /// # Behavior for complete mode
    ///
    /// Returns a StreamPosition containing the token index (index.0).
    /// The char offset (index.1) will be `None`.
    ///
    /// # Behavior for partial mode
    ///
    /// Returns a StreamPosition containing both the token index (index.0) and
    /// the character offset within that token (index.1 = Some(offset)).
    ///
    /// This position can be used with `set_position()` to restore the exact state,
    /// including the parsing mode and character offset.
    pub fn position(&self) -> StreamPosition {
        self.index
    }

    /// Reset the stream to a previously saved position, for backtracking.
    ///
    /// # Behavior for complete mode
    ///
    /// Restores the token index to the saved position. If the saved position
    /// was in complete mode (char offset = None), stays in complete mode.
    ///
    /// # Behavior for partial mode
    ///
    /// Can restore to either complete or partial mode depending on the saved position.
    /// If the saved position was in partial mode (char offset = Some(n)), switches
    /// to partial mode at that exact character offset. This allows proper backtracking
    /// during character-by-character matching attempts.
    pub fn set_position(&mut self, pos: StreamPosition) {
        self.index = pos;
    }

    /// Check if we've reached the end of the token stream.
    ///
    /// # Behavior for complete mode
    ///
    /// Returns `true` if the token index is at or past the end of the tokens array
    /// and we're in complete mode (char offset is `None`).
    ///
    /// # Behavior for partial mode
    ///
    /// Always returns `false` while in partial mode (char offset is `Some`), even if
    /// positioned at the last token. This is because partial mode implies we're still
    /// potentially consuming characters from the current token.
    pub fn is_at_end(&self) -> bool {
        self.index.0 >= self.tokens.len() && self.index.1.is_none()
    }

    /// Create a zero-length span at the current stream position.
    pub fn current_span(&self) -> Span {
        let offset = self.offset_from_start(self.index);
        Span::new(offset, offset)
    }

    /// Convert a `StreamPosition` into an absolute start offset in source bytes.
    ///
    /// Uses the lexer-supplied span of the token at `pos.0` and the character
    /// offset stored in `pos.1` (if any) to compute the precise byte position,
    /// preserving partial-mode progress within the token.
    fn offset_from_start(&self, pos: StreamPosition) -> usize {
        if let Some((_, span)) = self.tokens.get(pos.0) {
            let token_offset = pos.1.unwrap_or(0);
            return (span.start + token_offset).min(span.end);
        }
        self.tokens.last().map(|(_, span)| span.end).unwrap_or(0)
    }

    /// Convert a pair of positions into the absolute end offset of the parsed span.
    ///
    /// Handles both complete mode (token-level) and partial mode (character-level)
    /// states and gracefully falls back to the closest known span when the stream
    /// is at the very beginning or end.
    fn offset_from_end(&self, start: StreamPosition, end: StreamPosition) -> usize {
        if start == end {
            return self.offset_from_start(start);
        }

        if let Some(char_offset) = end.1 {
            if let Some((_, span)) = self.tokens.get(end.0) {
                return (span.start + char_offset).min(span.end);
            }
        } else if end.0 == 0 {
            if let Some((_, span)) = self.tokens.get(0) {
                return span.start;
            }
        } else if let Some((_, span)) = self.tokens.get(end.0 - 1) {
            return span.end;
        }

        self.tokens
            .last()
            .map(|(_, span)| span.end)
            .unwrap_or_else(|| self.offset_from_start(start))
    }
}

/// Trait for types that can be parsed from a PTX token stream.
///
/// This trait is implemented for all PTX AST node types to enable
/// recursive descent parsing.
///
/// Following the combinator architecture, parse() returns a parser function
/// rather than directly taking a stream parameter.
pub trait PtxParser
where
    Self: Sized,
{
    /// Returns a parser function that can parse an instance of `Self`.
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError>;
}

// Parse PTX source code into a structured Module representation.
//
// This is the main entry point for parsing PTX code. It performs lexical
// analysis followed by syntactic parsing.
//
// # Arguments
//
// * `source` - The PTX source code as a string slice
//
// # Returns
//
// Returns a parsed `Module` AST node, or a `PtxParseError` if parsing fails.
//
// # Example
//
// ```no_run
// use ptx_parser::parse_ptx;
//
// let source = r#"
//     .version 8.5
//     .target sm_90
//     .address_size 64
//
//     .entry kernel() {
//         ret;
//     }
// "#;
//
// let module = parse_ptx(source).expect("Failed to parse PTX");
// println!("Parsed {} directives", module.directives.len());
// ```
pub fn parse_ptx(source: &str) -> Result<crate::r#type::module::Module, PtxParseError> {
    #[cfg(debug_assertions)]
    {
        // Debug builds can have very deep combinator stacks; force a large stack for parsing.
        return stacker::grow(256 * 1024 * 1024, || parse_ptx_inner(source));
    }

    #[cfg(not(debug_assertions))]
    {
        parse_ptx_inner(source)
    }
}

fn parse_ptx_inner(source: &str) -> Result<crate::r#type::module::Module, PtxParseError> {
    use crate::{PtxTokenStream, tokenize, r#type::Module};

    let tokens = tokenize(source)?;
    let mut stream = PtxTokenStream::new(&tokens);
    let (module, _) = Module::parse()(&mut stream)?;
    if !stream.is_at_end() {
        let pos = stream.position();
        let remaining = tokens
            .get(pos.0)
            .map(|(tok, _)| format!("{:?}", tok))
            .unwrap_or_else(|| "EOF".into());
        return Err(PtxParseError {
            kind: ParseErrorKind::UnexpectedToken {
                expected: vec!["end of file".into()],
                found: remaining,
            },
            span: stream.current_span(),
        });
    }
    Ok(module)
}
