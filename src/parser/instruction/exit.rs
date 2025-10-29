#[allow(unused_imports)]
use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::exit::*},
};

impl PtxParser for Exit {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "exit")?;
        stream.expect(&PtxToken::Semicolon)?;
        Ok(Exit)
    }
}
