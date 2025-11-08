use logos::Logos;

pub use logos::Span;

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
    #[regex(r"0[fFdD][0-9a-fA-F]{8}", |lex| lex.slice().to_string())]
    #[regex(r"0[fFdD][0-9a-fA-F]{16}", |lex| lex.slice().to_string())]
    HexFloat(String),
    #[regex(r"0[xX][0-9a-fA-F]+", |lex| lex.slice().to_string())]
    HexInteger(String),
    #[regex(r"0[bB][01]+", |lex| lex.slice().to_string())]
    BinaryInteger(String),
    #[regex(r"0[0-7]+", |lex| lex.slice().to_string())]
    OctalInteger(String),
    #[regex(r"[0-9]+\.[0-9]+[eE][+-]?[0-9]+", |lex| lex.slice().to_string())]
    #[regex(r"[0-9]+[eE][+-]?[0-9]+", |lex| lex.slice().to_string())]
    FloatExponent(String),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().to_string())]
    Float(String),
    #[regex(r"[0-9]+", |lex| lex.slice().to_string())]
    DecimalInteger(String),
    #[regex(r"%[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Register(String),
    #[regex(r"[a-zA-Z_$][a-zA-Z0-9_$-]*", |lex| lex.slice().to_string())]
    Identifier(String),
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
            | PtxToken::HexFloat(s)
            | PtxToken::Register(s)
            | PtxToken::StringLiteral(s) => s.as_str(),
            _ => "",
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
            Ok(token) => tokens.push((token, lexer.span())),
            Err(_) => return Err(LexError { span: lexer.span() }),
        }
    }

    Ok(tokens)
}
