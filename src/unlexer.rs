use crate::lexer::PtxToken;
use std::fmt::{self, Write};

/// Utility that performs the inverse of the lexer: it writes textual PTX for a
/// sequence of [`PtxSpecToken`] values.
pub struct PtxUnlexer;

impl PtxUnlexer {
    /// Write the textual PTX representation of `tokens` to `writer`.
    pub fn write_tokens<W>(writer: &mut W, tokens: &[PtxToken]) -> fmt::Result
    where
        W: Write,
    {
        let mut prev: Option<&PtxToken> = None;
        for token in tokens {
            if let Some(previous) = prev {
                if needs_space(previous, token) {
                    writer.write_char(' ')?;
                }
            }
            write_token(writer, token)?;
            prev = Some(token);
        }
        Ok(())
    }

    /// Convenience helper that produces a PTX string for `tokens`.
    pub fn to_string(tokens: &[PtxToken]) -> Result<String, fmt::Error> {
        let mut buffer = String::new();
        Self::write_tokens(&mut buffer, tokens)?;
        Ok(buffer)
    }
}

fn write_token<W: Write>(writer: &mut W, token: &PtxToken) -> fmt::Result {
    match token {
        PtxToken::Identifier(name)
        | PtxToken::DecimalInteger(name)
        | PtxToken::HexInteger(name)
        | PtxToken::BinaryInteger(name)
        | PtxToken::OctalInteger(name)
        | PtxToken::Float(name)
        | PtxToken::FloatExponent(name)
        | PtxToken::HexFloatSingle(name)
        | PtxToken::HexFloatDouble(name)
        | PtxToken::Register(name) => writer.write_str(name),
        PtxToken::StringLiteral(name) => {
            writer.write_char('"')?;
            writer.write_str(name)?;
            writer.write_char('"')
        }
        PtxToken::Dot => writer.write_char('.'),
        PtxToken::Comma => writer.write_char(','),
        PtxToken::Semicolon => writer.write_char(';'),
        PtxToken::Colon => writer.write_char(':'),
        PtxToken::DoubleColon => writer.write_str("::"),
        PtxToken::LParen => writer.write_char('('),
        PtxToken::RParen => writer.write_char(')'),
        PtxToken::LBracket => writer.write_char('['),
        PtxToken::RBracket => writer.write_char(']'),
        PtxToken::LBrace => writer.write_char('{'),
        PtxToken::RBrace => writer.write_char('}'),
        PtxToken::Plus => writer.write_char('+'),
        PtxToken::Minus => writer.write_char('-'),
        PtxToken::Star => writer.write_char('*'),
        PtxToken::Slash => writer.write_char('/'),
        PtxToken::LAngle => writer.write_char('<'),
        PtxToken::RAngle => writer.write_char('>'),
        PtxToken::Equals => writer.write_char('='),
        PtxToken::Percent => writer.write_char('%'),
        PtxToken::Exclaim => writer.write_char('!'),
        PtxToken::Pipe => writer.write_char('|'),
        PtxToken::Ampersand => writer.write_char('&'),
        PtxToken::Caret => writer.write_char('^'),
        PtxToken::Tilde => writer.write_char('~'),
        PtxToken::At => writer.write_char('@'),
    }
}

fn needs_space(prev: &PtxToken, curr: &PtxToken) -> bool {
    if no_space_after(prev) || no_space_before(curr) {
        return false;
    }
    true
}

fn no_space_after(token: &PtxToken) -> bool {
    matches!(
        token,
        PtxToken::LParen | PtxToken::LBracket | PtxToken::LBrace | PtxToken::Dot
    )
}

fn no_space_before(token: &PtxToken) -> bool {
    matches!(
        token,
        PtxToken::RParen
            | PtxToken::RBracket
            | PtxToken::RBrace
            | PtxToken::Comma
            | PtxToken::Semicolon
            | PtxToken::Colon
            | PtxToken::Dot
    )
}
