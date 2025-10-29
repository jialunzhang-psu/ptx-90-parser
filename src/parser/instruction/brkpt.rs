#[allow(unused_imports)]
use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::brkpt::*},
};

impl PtxParser for Brkpt {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "brkpt")?;
        stream.expect(&PtxToken::Semicolon)?;
        Ok(Brkpt)
    }
}
