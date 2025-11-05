#![allow(dead_code)]
use ptx_parser::{
    lexer::tokenize,
    parser::{PtxParseError, PtxParser, PtxTokenStream},
    unparser::PtxUnparser,
};

pub fn parse_result<T: PtxParser>(source: &str) -> Result<T, PtxParseError> {
    let tokens = tokenize(source).expect("tokenization should succeed");
    let mut stream = PtxTokenStream::new(&tokens);
    T::parse(&mut stream)
}

pub fn parse<T: PtxParser>(source: &str) -> T {
    parse_result(source).expect("parse should succeed")
}

pub fn tokenize_only(source: &str) -> Vec<ptx_parser::lexer::PtxToken> {
    let tokens = tokenize(source).expect("tokenization should succeed");
    tokens.into_iter().map(|(token, _)| token).collect()
}

pub fn assert_roundtrip<T>(source: &str)
where
    T: PtxParser + PtxUnparser,
{
    let original_tokens = tokenize_only(source);
    let parsed = parse::<T>(source);
    assert_eq!(parsed.to_tokens(), original_tokens);
}
