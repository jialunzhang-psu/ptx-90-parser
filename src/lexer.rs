use crate::parser::Span;
use logos::Logos;

/// PTX specification token types for lexical analysis.
///
/// This enum represents all token types that can appear in PTX assembly code,
/// including keywords, operators, literals, identifiers, and special markers.
#[derive(Logos, Debug, Clone, PartialEq, Eq)]
#[logos(error = LexError)]
#[logos(skip r"[ \t\r\n]+")]
pub enum PtxToken {
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/", logos::skip)]
    #[token("::")]
    DoubleColon,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("<")]
    LAngle,
    #[token(">")]
    RAngle,
    #[token("=")]
    Equals,
    #[token("%")]
    Percent,
    #[token("!")]
    Exclaim,
    #[token("|")]
    Pipe,
    #[token("&")]
    Ampersand,
    #[token("^")]
    Caret,
    #[token("~")]
    Tilde,
    #[token("@")]
    At,
    // Single-precision hex float: 0f12345678
    #[regex(r"0[fF][0-9a-fA-F]{8}", |lex| lex.slice().to_string())]
    HexFloatSingle(String),
    // Double-precision hex float: 0d1234567890abcdef
    #[regex(r"0[dD][0-9a-fA-F]{16}", |lex| lex.slice().to_string())]
    HexFloatDouble(String),
    #[regex(r"0[xX][0-9a-fA-F]+U?", |lex| lex.slice().to_string())]
    HexInteger(String),
    #[regex(r"0[bB][01]+U?", |lex| lex.slice().to_string())]
    BinaryInteger(String),
    #[regex(r"0[0-7]+U?", |lex| lex.slice().to_string())]
    OctalInteger(String),
    #[regex(r"[0-9]+\.[0-9]+[eE][+-]?[0-9]+", |lex| lex.slice().to_string())]
    #[regex(r"[0-9]+[eE][+-]?[0-9]+", |lex| lex.slice().to_string())]
    FloatExponent(String),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().to_string())]
    Float(String),
    #[regex(r"[1-9][0-9]*U?", priority = 2, callback = |lex| lex.slice().to_string())]
    #[regex(r"0U?", |lex| lex.slice().to_string())]
    DecimalInteger(String),
    #[regex(r"%[a-zA-Z_][a-zA-Z0-9_]*", priority = 2, callback = |lex| lex.slice().to_string())]
    Register(String),
    #[regex(r"[a-zA-Z][a-zA-Z0-9_$]*", |lex| lex.slice().to_string())]
    #[regex(r"[_$%][a-zA-Z0-9_$]*", priority = 1, callback = |lex| lex.slice().to_string())]
    Identifier(String),
    /// Whitespace tokens emitted by the unparser only (never produced by the lexer).
    #[token("\u{0000}")]
    Space,
    #[token("\u{0001}")]
    Newline,
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let slice = lex.slice();
        slice[1..slice.len() - 1].to_string()
    })]
    StringLiteral(String),
}

impl PtxToken {
    /// Extract the string value from a token, if it has one
    pub fn as_str(&self) -> &str {
        match self {
            PtxToken::Identifier(s)
            | PtxToken::DecimalInteger(s)
            | PtxToken::HexInteger(s)
            | PtxToken::BinaryInteger(s)
            | PtxToken::OctalInteger(s)
            | PtxToken::Float(s)
            | PtxToken::FloatExponent(s)
        | PtxToken::HexFloatSingle(s)
        | PtxToken::HexFloatDouble(s)
        | PtxToken::Register(s)
        | PtxToken::StringLiteral(s) => s.as_str(),
        PtxToken::DoubleColon => "::",
        PtxToken::Dot => ".",
        PtxToken::Comma => ",",
        PtxToken::Semicolon => ";",
        PtxToken::Colon => ":",
            PtxToken::LParen => "(",
            PtxToken::RParen => ")",
            PtxToken::LBracket => "[",
            PtxToken::RBracket => "]",
            PtxToken::LBrace => "{",
            PtxToken::RBrace => "}",
            PtxToken::Plus => "+",
            PtxToken::Minus => "-",
            PtxToken::Star => "*",
            PtxToken::Slash => "/",
            PtxToken::LAngle => "<",
            PtxToken::RAngle => ">",
            PtxToken::Equals => "=",
            PtxToken::Percent => "%",
            PtxToken::Exclaim => "!",
            PtxToken::Pipe => "|",
            PtxToken::Ampersand => "&",
            PtxToken::Caret => "^",
            PtxToken::Tilde => "~",
            PtxToken::At => "@",
            PtxToken::Space => " ",
            PtxToken::Newline => "\n",
        }
    }

    pub fn len(&self) -> usize {
        match self {
            PtxToken::Identifier(s)
            | PtxToken::DecimalInteger(s)
            | PtxToken::HexInteger(s)
            | PtxToken::BinaryInteger(s)
            | PtxToken::OctalInteger(s)
            | PtxToken::Float(s)
            | PtxToken::FloatExponent(s)
            | PtxToken::HexFloatSingle(s)
            | PtxToken::HexFloatDouble(s)
            | PtxToken::Register(s)
            | PtxToken::StringLiteral(s) => s.len(),
            PtxToken::DoubleColon => 2,
            PtxToken::Dot
            | PtxToken::Comma
            | PtxToken::Semicolon
            | PtxToken::Colon
            | PtxToken::LParen
            | PtxToken::RParen
            | PtxToken::LBracket
            | PtxToken::RBracket
            | PtxToken::LBrace
            | PtxToken::RBrace
            | PtxToken::Plus
            | PtxToken::Minus
            | PtxToken::Star
            | PtxToken::Slash
            | PtxToken::LAngle
            | PtxToken::RAngle
            | PtxToken::Equals
            | PtxToken::Percent
            | PtxToken::Exclaim
            | PtxToken::Pipe
            | PtxToken::Ampersand
            | PtxToken::Caret
            | PtxToken::Tilde
            | PtxToken::At
            | PtxToken::Space
            | PtxToken::Newline => 1,
        }
    }
}

/// Lexical analysis error type.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LexError {
    /// The span in the source code where the error occurred
    pub span: Span,
}

impl From<Span> for LexError {
    fn from(span: Span) -> Self {
        LexError { span }
    }
}

/// Tokenize a PTX source string into a sequence of tokens with their spans.
///
/// This is the main entry point for lexical analysis. It converts raw PTX
/// source code into a vector of tokens that can be parsed.
///
/// # Arguments
///
/// * `source` - The PTX source code as a string slice
///
/// # Returns
///
/// Returns a vector of tuples containing each token and its span in the source,
/// or a `LexError` if tokenization fails.
///
/// # Example
///
/// ```no_run
/// use ptx_parser::tokenize;
///
/// let source = ".version 8.5\n.target sm_90";
/// let tokens = tokenize(source).expect("Failed to tokenize");
/// ```
pub fn tokenize(source: &str) -> Result<Vec<(PtxToken, Span)>, LexError> {
    let mut lexer = PtxToken::lexer(source);
    let mut tokens = Vec::new();

    while let Some(item) = lexer.next() {
        match item {
            Ok(token) => tokens.push((token, Span::from(lexer.span()))),
            Err(_) => {
                return Err(LexError {
                    span: Span::from(lexer.span()),
                });
            }
        }
    }

    Ok(tokens)
}
