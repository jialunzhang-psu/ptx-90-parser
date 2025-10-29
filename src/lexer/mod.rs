use logos::Logos;
pub(crate) use logos::Span;

/// PTX Token types for lexical analysis
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(error = LexError)]
#[logos(skip r"[ \t\r]+")] // Skip whitespace (but not newlines in some contexts)
pub enum PtxToken {
    // Comments - skip both C++ and C style
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/", logos::skip)]
    // Whitespace and newlines
    #[token("\n")]
    Newline,

    // Punctuation and operators
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

    // Directives (start with dot followed by identifier)
    // This must come before Dot token to match properly
    #[regex(r"\.[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice()[1..].to_string())]
    Directive(String),

    // Numbers - order matters! More specific patterns first

    // Hexadecimal floating point (0F... or 0D...)
    #[regex(r"0[fFdD][0-9a-fA-F]{8}", |lex| lex.slice().to_string())]
    #[regex(r"0[fFdD][0-9a-fA-F]{16}", |lex| lex.slice().to_string())]
    HexFloat(String),

    // Hexadecimal integer
    #[regex(r"0[xX][0-9a-fA-F]+", |lex| lex.slice().to_string())]
    HexInteger(String),

    // Binary integer
    #[regex(r"0[bB][01]+", |lex| lex.slice().to_string())]
    BinaryInteger(String),

    // Octal integer
    #[regex(r"0[0-7]+", |lex| lex.slice().to_string())]
    OctalInteger(String),

    // Floating point with exponent
    #[regex(r"[0-9]+\.[0-9]+[eE][+-]?[0-9]+", |lex| lex.slice().to_string())]
    #[regex(r"[0-9]+[eE][+-]?[0-9]+", |lex| lex.slice().to_string())]
    FloatExponent(String),

    // Regular floating point
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().to_string())]
    Float(String),

    // Decimal integer (including negative)
    #[regex(r"[0-9]+", |lex| lex.slice().to_string())]
    DecimalInteger(String),

    // Identifiers and special registers
    // Register names starting with %
    #[regex(r"%[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Register(String),

    // Regular identifiers (variables, labels, function names)
    #[regex(r"[a-zA-Z_$][a-zA-Z0-9_$]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // String literals
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    StringLiteral(String),
}

/// Custom error type for lexer
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LexError {
    pub span: Span,
}

impl From<Span> for LexError {
    fn from(span: Span) -> Self {
        LexError { span }
    }
}

/// Utility function to tokenize PTX source code
pub fn tokenize(source: &str) -> Result<Vec<(PtxToken, Span)>, LexError> {
    let mut tokens = Vec::new();
    let mut lexer = PtxToken::lexer(source);

    while let Some(token_result) = lexer.next() {
        match token_result {
            Ok(token) => {
                tokens.push((token, lexer.span()));
            }
            Err(_) => {
                return Err(LexError { span: lexer.span() });
            }
        }
    }

    Ok(tokens)
}
