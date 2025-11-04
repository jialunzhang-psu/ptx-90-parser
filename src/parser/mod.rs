use crate::lexer::{PtxToken, tokenize};
use crate::r#type::instruction::Instruction;
use thiserror::Error;

mod common;
mod function;
pub mod instruction;
mod module;
mod variable;

pub type Span = std::ops::Range<usize>;

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

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("parsing error at {span:?}: {kind}")]
pub struct PtxParseError {
    pub kind: ParseErrorKind,
    pub span: Span,
}

pub struct PtxTokenStream<'a> {
    tokens: &'a [(PtxToken, Span)],
    /// Current position (index) in the tokens list
    index: usize,
}

#[derive(Debug, Clone)]
pub struct InstructionParseFailure {
    pub index: usize,
    pub statement: String,
    pub error: PtxParseError,
}

#[derive(Debug, Clone, Default)]
pub struct InstructionParseSummary {
    pub parsed: usize,
    pub failures: Vec<InstructionParseFailure>,
}

impl<'a> PtxTokenStream<'a> {
    pub fn new(tokens: &'a [(PtxToken, Span)]) -> Self {
        Self { tokens, index: 0 }
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
    pub fn expect(
        &mut self,
        expected: &PtxToken,
    ) -> Result<&'a (PtxToken, Span), PtxParseError> {
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

    /// Try to match a string pattern by consuming tokens until the entire pattern is matched.
    ///
    /// # Behavior
    /// This function attempts to match a string pattern against the token stream by:
    /// 1. Saving the current position in the stream for potential rollback
    /// 2. Tokenizing the pattern string to get expected tokens
    /// 3. Iterating through expected tokens and matching them against the stream by value
    /// 4. If all tokens match exactly, leaving the stream advanced and returning `true`
    /// 5. If any token fails to match, resetting the stream position and returning `false`
    ///
    /// Tokens are matched by exact equality. This means:
    /// - `Identifier("foo")` matches only `Identifier("foo")`, not `Identifier("bar")`
    /// - `StringLiteral("x")` matches only `StringLiteral("x")`
    /// - `Dot` matches only `Dot`
    ///
    /// # Returns
    /// - `true` if the entire pattern was successfully matched (tokens consumed)
    /// - `false` if matching failed at any point (stream position restored)
    ///
    /// # Examples
    /// - Pattern ".shared" matches tokens [Dot, Identifier("shared")]
    /// - Pattern ".to::cluster" matches tokens [Dot, Identifier("to"), Colon, Colon, Identifier("cluster")]
    /// - Pattern "async" matches token [Identifier("async")]
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

        // Try to match each expected token by exact value
        for (expected_token, _) in expected_tokens {
            match self.peek() {
                Ok((actual_token, _)) => {
                    if actual_token != &expected_token {
                        self.set_position(start_pos);
                        return false;
                    }
                    // Token matches, consume it
                    self.index += 1;
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
    pub fn position(&self) -> usize {
        self.index
    }

    /// Reset the stream to an old position, for backtracking.
    pub fn set_position(&mut self, index: usize) {
        self.index = index;
    }

    /// Check if we've reached the end of the token stream.
    pub fn is_at_end(&self) -> bool {
        self.index >= self.tokens.len()
    }

    /// Get the remaining tokens.
    pub fn remaining(&self) -> &'a [(PtxToken, Span)] {
        &self.tokens[self.index..]
    }
}

pub trait PtxParser
where
    Self: Sized,
{
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError>;
}

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

pub fn analyze_instruction_stream(source: &str) -> InstructionParseSummary {
    let mut summary = InstructionParseSummary::default();

    for (idx, statement) in extract_instruction_statements(source)
        .into_iter()
        .enumerate()
    {
        let trimmed = statement.trim();

        if trimmed.is_empty()
            || trimmed.starts_with('.')
            || trimmed.starts_with('{')
            || trimmed.starts_with('}')
            || trimmed.ends_with(':')
        {
            continue;
        }

        // Strip trailing semicolon for instruction parsing
        let trimmed = trimmed.strip_suffix(';').unwrap_or(trimmed);

        let tokens = match tokenize(trimmed) {
            Ok(tokens) => tokens,
            Err(err) => {
                summary.failures.push(InstructionParseFailure {
                    index: idx + 1,
                    statement,
                    error: PtxParseError {
                        kind: ParseErrorKind::InvalidLiteral("lexical error".to_string()),
                        span: err.span,
                    },
                });
                continue;
            }
        };

        if tokens.is_empty() {
            continue;
        }

        let mut stream = PtxTokenStream::new(&tokens);
        match <Instruction as PtxParser>::parse(&mut stream) {
            Ok(_) if stream.is_at_end() => summary.parsed += 1,
            Ok(_) => summary.failures.push(InstructionParseFailure {
                index: idx + 1,
                statement,
                error: PtxParseError {
                    kind: ParseErrorKind::UnexpectedToken {
                        expected: vec!["end of statement".to_string()],
                        found: format!("remaining tokens at position {}", stream.position()),
                    },
                    span: tokens
                        .get(stream.position())
                        .map_or(0..0, |(_, span)| span.clone()),
                },
            }),
            Err(err) => summary.failures.push(InstructionParseFailure {
                index: idx + 1,
                statement,
                error: err,
            }),
        }
    }

    summary
}

pub fn extract_instruction_statements(source: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();

    for line in source.lines() {
        let mut rest = line;

        while let Some(idx) = rest.find(';') {
            current.push_str(rest[..=idx].trim());
            if !current.trim().is_empty() {
                statements.push(current.trim().to_string());
            }
            current.clear();
            rest = &rest[idx + 1..];
        }

        if rest.trim_end().ends_with(':') {
            let label = rest.trim();
            if !label.is_empty() {
                statements.push(label.to_string());
            }
            current.clear();
        } else if !rest.trim().is_empty() {
            current.push_str(rest.trim());
            current.push(' ');
        }
    }

    if !current.trim().is_empty() {
        statements.push(current.trim().to_string());
    }

    statements
}
