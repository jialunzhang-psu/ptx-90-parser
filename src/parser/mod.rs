use crate::lexer::PtxToken;
use thiserror::Error;

mod common;
mod function;
mod instruction;
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

    /// Check if the next token is a directive, and if so, consume it and return the String.
    pub fn expect_directive(&mut self) -> Result<(String, Span), PtxParseError> {
        self.expect_token_with_string("Directive", |token| {
            if let PtxToken::Directive(name) = token {
                Some(name.clone())
            } else {
                None
            }
        })
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

pub(crate) fn directive_matches(token: &PtxToken, expected: &str) -> bool {
    matches!(token, PtxToken::Directive(name) if name == expected)
}

pub(crate) fn consume_directive_if(stream: &mut PtxTokenStream, expected: &str) -> bool {
    stream
        .consume_if(|token| directive_matches(token, expected))
        .is_some()
}

pub(crate) fn expect_identifier_value(
    stream: &mut PtxTokenStream,
    expected: &str,
) -> Result<(), PtxParseError> {
    let (value, span) = stream.expect_identifier()?;
    if value == expected {
        Ok(())
    } else {
        Err(unexpected_value(span, &[expected], format!("{value}")))
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
    match stream.peek() {
        Ok((token, span)) => {
            if let PtxToken::Directive(value) = token {
                Ok(Some((value.clone(), span.clone())))
            } else {
                Ok(None)
            }
        }
        Err(err) if matches!(err.kind, ParseErrorKind::UnexpectedEof) => Ok(None),
        Err(err) => Err(err),
    }
}

pub(crate) fn parse_u16_literal(stream: &mut PtxTokenStream) -> Result<(u16, Span), PtxParseError> {
    let (token, span) = stream.consume()?;
    let span = span.clone();
    let value = match token {
        PtxToken::DecimalInteger(text) => text.parse::<u32>(),
        PtxToken::HexInteger(text) => {
            let stripped = text
                .strip_prefix("0x")
                .or_else(|| text.strip_prefix("0X"))
                .ok_or_else(|| invalid_literal(span.clone(), text.clone()))?;
            u32::from_str_radix(stripped, 16)
        }
        PtxToken::BinaryInteger(text) => {
            let stripped = text
                .strip_prefix("0b")
                .or_else(|| text.strip_prefix("0B"))
                .ok_or_else(|| invalid_literal(span.clone(), text.clone()))?;
            u32::from_str_radix(stripped, 2)
        }
        PtxToken::OctalInteger(text) => {
            let stripped = &text[1..];
            u32::from_str_radix(stripped, 8)
        }
        other => {
            return Err(unexpected_value(
                span,
                &["numeric literal"],
                format!("{other:?}"),
            ));
        }
    }
    .map_err(|_| invalid_literal(span.clone(), "invalid integer literal"))?;

    if value > u16::MAX as u32 {
        return Err(invalid_literal(
            span,
            format!("value {value} exceeds u16 range"),
        ));
    }

    Ok((value as u16, span))
}
