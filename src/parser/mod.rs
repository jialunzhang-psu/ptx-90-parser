use crate::lexer::{PtxToken, tokenize};
use thiserror::Error;

pub(crate) mod common;
pub(crate) mod function;
pub(crate) mod instruction;
pub(crate) mod module;
pub(crate) mod variable;

pub type Span = std::ops::Range<usize>;

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
}

/// PTX parsing error with location information.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("parsing error at {span:?}: {kind}")]
pub struct PtxParseError {
    pub kind: ParseErrorKind,
    pub span: Span,
}

/// Represents a position in the token stream, including both token index and character offset within a token
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamPosition {
    pub index: usize,
    pub char_offset: usize,
}

/// Token stream wrapper for parsing PTX tokens.
///
/// This struct provides methods for consuming and inspecting tokens during parsing.
pub struct PtxTokenStream<'a> {
    tokens: &'a [(PtxToken, Span)],
    /// Current position (index) in the tokens list
    index: usize,
    /// Position within the current token's string content (for parsing multi-char identifiers/numbers)
    pub(crate) char_offset: usize,
}

impl<'a> PtxTokenStream<'a> {
    pub fn new(tokens: &'a [(PtxToken, Span)]) -> Self {
        Self {
            tokens,
            index: 0,
            char_offset: 0,
        }
    }

    /// Peek at the next token without consuming it.
    pub fn peek(&self) -> Result<&'a (PtxToken, Span), PtxParseError> {
        self.tokens.get(self.index).ok_or_else(|| {
            // If the stream is empty, return an EOF error
            let span = self.tokens.last().map_or(0..0, |(_, s)| s.clone());
            PtxParseError {
                kind: ParseErrorKind::UnexpectedEof,
                span,
            }
        })
    }

    /// Consume and return the next token.
    pub fn consume(&mut self) -> Result<&'a (PtxToken, Span), PtxParseError> {
        let token = self.peek()?;
        self.index += 1;
        Ok(token)
    }

    /// Check if the next token is the expected type, and if so, consume it.
    /// Otherwise, return an error and do NOT consume the token.
    pub fn expect(&mut self, expected: &PtxToken) -> Result<&'a (PtxToken, Span), PtxParseError> {
        let token_pair = self.peek()?;
        let (token, span) = token_pair;
        if std::mem::discriminant(token) == std::mem::discriminant(expected) {
            self.index += 1;
            Ok(token_pair)
        } else {
            Err(PtxParseError {
                kind: ParseErrorKind::UnexpectedToken {
                    expected: vec![format!("{:?}", expected)],
                    found: format!("{:?}", token),
                },
                span: span.clone(),
            })
        }
    }

    /// Generic helper to extract a String value from a token variant.
    /// Returns the extracted string and span if the pattern matches, otherwise returns an error.
    fn expect_token_with_string<F>(
        &mut self,
        expected_name: &str,
        extractor: F,
    ) -> Result<(String, Span), PtxParseError>
    where
        F: FnOnce(&PtxToken) -> Option<String>,
    {
        let (token, span) = self.peek()?;
        if let Some(value) = extractor(token) {
            let span = span.clone();
            self.index += 1;
            Ok((value, span))
        } else {
            Err(PtxParseError {
                kind: ParseErrorKind::UnexpectedToken {
                    expected: vec![expected_name.to_string()],
                    found: format!("{:?}", token),
                },
                span: span.clone(),
            })
        }
    }

    /// Check if the next token is an identifier, and if so, consume it and return the String.
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
    pub fn expect_directive(&mut self) -> Result<(String, Span), PtxParseError> {
        let (_, dot_span) = self.expect(&PtxToken::Dot)?;
        let (name, id_span) = self.expect_identifier()?;
        let span = dot_span.start..id_span.end;
        Ok((name, span))
    }

    /// Check if the next token is a directive that represents a modifier (type, state space, etc.).
    /// This is an alias for expect_directive for semantic clarity when parsing modifiers.
    pub fn expect_modifier(&mut self) -> Result<(String, Span), PtxParseError> {
        self.expect_directive()
    }

    /// Expect and consume a double colon (::) token sequence.
    pub fn expect_double_colon(&mut self) -> Result<(), PtxParseError> {
        self.expect(&PtxToken::Colon)?;
        self.expect(&PtxToken::Colon)?;
        Ok(())
    }

    /// Try to match and consume a sequence of tokens that matches one of the candidate strings.
    /// Returns the index of the matched candidate.
    ///
    /// This is used for parsing modifiers that may contain :: sequences like ".to::cluster"
    /// The candidates should include the leading dot (e.g., [".to::cluster", ".to::cta"])
    pub fn expect_strings(&mut self, candidates: &[&str]) -> Result<usize, PtxParseError> {
        let start_pos = self.position();

        for (idx, candidate) in candidates.iter().enumerate() {
            self.set_position(start_pos);

            // Try to match this candidate
            if self.try_match_string(candidate) {
                return Ok(idx);
            }
        }

        // None matched, create error
        let (token, span) = self.peek()?;
        Err(PtxParseError {
            kind: ParseErrorKind::UnexpectedToken {
                expected: candidates.iter().map(|s| s.to_string()).collect(),
                found: format!("{:?}", token),
            },
            span: span.clone(),
        })
    }

    pub fn expect_string(&mut self, expected: &str) -> Result<(), PtxParseError> {
        if self.try_match_string(expected) {
            Ok(())
        } else {
            let (token, span) = self.peek()?;
            Err(PtxParseError {
                kind: ParseErrorKind::UnexpectedToken {
                    expected: vec![expected.to_string()],
                    found: format!("{:?}", token),
                },
                span: span.clone(),
            })
        }
    }

    /// Try to match a string pattern by consuming characters from the stream.
    ///
    /// # Behavior
    /// Matches the pattern character-by-character against the token stream.
    /// Tokens are converted to their string representation and matched from char_offset.
    /// If all characters match, the stream is advanced and returns true.
    /// If any character fails to match, the stream is reset and returns false.
    ///
    /// # Returns
    /// - `true` if the entire pattern was successfully matched (chars consumed)
    /// - `false` if matching failed at any point (stream position restored)
    pub fn try_match_string(&mut self, pattern: &str) -> bool {
        let start_pos = self.position();

        // Tokenize the pattern to get expected tokens
        let expected_tokens = match tokenize(pattern) {
            Ok(tokens) => tokens,
            Err(_) => {
                // If pattern can't be tokenized, it can't match
                return false;
            }
        };

        // Try to match each expected token
        for (expected_token, _) in expected_tokens {
            match self.peek() {
                Ok((actual_token, _)) => {
                    // Check if we can do a partial match for Identifier tokens
                    // This handles cases like matching ".b3210" as ".b" + "3" + "2" + "1" + "0"
                    if let (PtxToken::Identifier(actual_id), expected_str) =
                        (actual_token, expected_token.as_str())
                    {
                        // Check if the expected string matches from the current char_offset
                        let remaining = &actual_id[self.char_offset..];
                        if remaining.starts_with(expected_str) {
                            let new_offset = self.char_offset + expected_str.len();
                            if new_offset == actual_id.len() {
                                // Exactly consumed the entire identifier - advance to next token
                                self.index += 1;
                                self.char_offset = 0;
                            } else {
                                // Partial match! Advance char_offset but DON'T advance index
                                self.char_offset = new_offset;
                            }
                            continue;
                        }
                    }

                    // Normal exact match
                    if actual_token != &expected_token {
                        self.set_position(start_pos);
                        return false;
                    }
                    // Token matches, consume it
                    self.index += 1;
                    self.char_offset = 0;
                }
                Err(_) => {
                    // Unexpected EOF
                    self.set_position(start_pos);
                    return false;
                }
            }
        }

        // Successfully matched all tokens
        true
    }

    /// Check if the next token matches a specific pattern.
    pub fn check<F>(&self, predicate: F) -> bool
    where
        F: FnOnce(&PtxToken) -> bool,
    {
        self.tokens
            .get(self.index)
            .map_or(false, |(token, _)| predicate(token))
    }

    /// Expect that we've consumed a complete token (not stopped in the middle).
    /// This should be called at the end of each struct parser to verify that
    /// character-level parsing has consumed all characters from the current token.
    ///
    /// # Returns
    /// - `Ok(())` if `char_offset == 0` (no partial token consumption)
    /// - `Err(PtxParseError)` if `char_offset > 0` (stopped in middle of token)
    pub fn expect_complete(&self) -> Result<(), PtxParseError> {
        if self.char_offset > 0 {
            // We're in the middle of a token - this is an error
            let span = self
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            Err(unexpected_value(
                span,
                &["complete token"],
                format!("partial token at char offset {}", self.char_offset),
            ))
        } else {
            Ok(())
        }
    }

    /// Consume the next token if it matches the predicate.
    pub fn consume_if<F>(&mut self, predicate: F) -> Option<&'a (PtxToken, Span)>
    where
        F: FnOnce(&PtxToken) -> bool,
    {
        if self.check(predicate) {
            self.index += 1;
            self.tokens.get(self.index - 1)
        } else {
            None
        }
    }

    /// Get the current position in the stream, for backtracking.
    pub fn position(&self) -> StreamPosition {
        StreamPosition {
            index: self.index,
            char_offset: self.char_offset,
        }
    }

    /// Reset the stream to an old position, for backtracking.
    pub fn set_position(&mut self, pos: StreamPosition) {
        self.index = pos.index;
        self.char_offset = pos.char_offset;
    }

    /// Check if we've reached the end of the token stream.
    pub fn is_at_end(&self) -> bool {
        self.index >= self.tokens.len()
    }

    /// Get the remaining tokens.
    pub fn remaining(&self) -> &'a [(PtxToken, Span)] {
        &self.tokens[self.index..]
    }

    /// Peek at the character at the current char_offset within the current token's string.
    /// Returns None if we're at the end of the current token's string or if the token has no string content.
    pub fn peek_char_in_token(&self) -> Option<char> {
        if self.index >= self.tokens.len() {
            return None;
        }

        let (token, _) = &self.tokens[self.index];
        let string = match token {
            PtxToken::Identifier(s)
            | PtxToken::DecimalInteger(s)
            | PtxToken::HexInteger(s)
            | PtxToken::BinaryInteger(s)
            | PtxToken::OctalInteger(s) => s,
            _ => return None,
        };

        string.chars().nth(self.char_offset)
    }

    /// Consume one character from the current token by advancing char_offset.
    /// If we reach the end of the token's string, advance to the next token and reset char_offset.
    /// Returns the consumed character.
    pub fn consume_char_in_token(&mut self) -> Option<char> {
        let ch = self.peek_char_in_token()?;
        self.char_offset += 1;

        // Check if we've consumed the entire string of this token
        if self.index < self.tokens.len() {
            let (token, _) = &self.tokens[self.index];
            let string = match token {
                PtxToken::Identifier(s)
                | PtxToken::DecimalInteger(s)
                | PtxToken::HexInteger(s)
                | PtxToken::BinaryInteger(s)
                | PtxToken::OctalInteger(s) => s,
                _ => "",
            };

            if self.char_offset >= string.len() {
                // Move to next token and reset char_offset
                self.index += 1;
                self.char_offset = 0;
            }
        }

        Some(ch)
    }

    /// Match a specific character at the current position within the token.
    /// Consumes the character if it matches.
    pub fn expect_char_in_token(&mut self, expected: char) -> Result<char, PtxParseError> {
        match self.peek_char_in_token() {
            Some(ch) if ch == expected => {
                self.consume_char_in_token();
                Ok(ch)
            }
            Some(ch) => {
                let span = if self.index < self.tokens.len() {
                    self.tokens[self.index].1.clone()
                } else {
                    0..0
                };
                Err(PtxParseError {
                    kind: ParseErrorKind::UnexpectedToken {
                        expected: vec![format!("'{}'", expected)],
                        found: format!("'{}'", ch),
                    },
                    span,
                })
            }
            None => {
                let span = if self.index < self.tokens.len() {
                    self.tokens[self.index].1.clone()
                } else {
                    0..0
                };
                Err(PtxParseError {
                    kind: ParseErrorKind::UnexpectedEof,
                    span,
                })
            }
        }
    }
}

/// Trait for types that can be parsed from a PTX token stream.
///
/// This trait is implemented for all PTX AST node types to enable
/// recursive descent parsing.
pub trait PtxParser
where
    Self: Sized,
{
    /// Parse an instance of `Self` from the token stream.
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError>;
}

/// Parse PTX source code into a structured Module representation.
///
/// This is the main entry point for parsing PTX code. It performs lexical
/// analysis followed by syntactic parsing.
///
/// # Arguments
///
/// * `source` - The PTX source code as a string slice
///
/// # Returns
///
/// Returns a parsed `Module` AST node, or a `PtxParseError` if parsing fails.
///
/// # Example
///
/// ```no_run
/// use ptx_parser::parse_ptx;
///
/// let source = r#"
///     .version 8.5
///     .target sm_90
///     .address_size 64
///     
///     .entry kernel() {
///         ret;
///     }
/// "#;
///
/// let module = parse_ptx(source).expect("Failed to parse PTX");
/// println!("Parsed {} directives", module.directives.len());
/// ```
pub fn parse_ptx(source: &str) -> Result<crate::r#type::module::Module, PtxParseError> {
    let tokens = crate::lexer::tokenize(source).map_err(|err| PtxParseError {
        kind: ParseErrorKind::InvalidLiteral("lexical error".into()),
        span: err.span,
    })?;
    let mut stream = PtxTokenStream::new(&tokens);
    let module = crate::r#type::module::Module::parse(&mut stream)?;
    if !stream.is_at_end() {
        let (token, span) = stream.peek()?;
        return Err(unexpected_value(
            span.clone(),
            &["end of input"],
            format!("{token:?}"),
        ));
    }
    Ok(module)
}

pub fn unexpected_value(span: Span, expected: &[&str], found: impl Into<String>) -> PtxParseError {
    PtxParseError {
        kind: ParseErrorKind::UnexpectedToken {
            expected: expected.iter().map(|s| s.to_string()).collect(),
            found: found.into(),
        },
        span,
    }
}

pub(crate) fn invalid_literal(span: Span, message: impl Into<String>) -> PtxParseError {
    PtxParseError {
        kind: ParseErrorKind::InvalidLiteral(message.into()),
        span,
    }
}

pub(crate) fn expect_directive_value(
    stream: &mut PtxTokenStream,
    expected: &str,
) -> Result<(), PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    if value == expected {
        Ok(())
    } else {
        Err(unexpected_value(
            span,
            &[&format!(".{expected}")],
            format!(".{value}"),
        ))
    }
}

pub(crate) fn peek_directive(
    stream: &mut PtxTokenStream,
) -> Result<Option<(String, Span)>, PtxParseError> {
    // Check if we have Dot followed by Identifier
    if let Some((PtxToken::Dot, dot_span)) = stream.tokens.get(stream.index) {
        if let Some((PtxToken::Identifier(value), id_span)) = stream.tokens.get(stream.index + 1) {
            let span = dot_span.start..id_span.end;
            return Ok(Some((value.clone(), span)));
        }
    }
    Ok(None)
}
