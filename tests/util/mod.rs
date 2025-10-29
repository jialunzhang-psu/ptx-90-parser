use ptx_parser::{
    lexer::tokenize,
    parser::{PtxParseError, PtxParser, PtxTokenStream},
};

pub fn parse_result<T: PtxParser>(source: &str) -> Result<T, PtxParseError> {
    let tokens = tokenize(source).expect("tokenization should succeed");
    let mut stream = PtxTokenStream::new(&tokens);
    T::parse(&mut stream)
}

pub fn parse<T: PtxParser>(source: &str) -> T {
    parse_result(source).expect("parse should succeed")
}
