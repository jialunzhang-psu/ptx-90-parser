#[allow(unused_imports)]
use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::trap::*},
};

impl PtxParser for Trap {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "trap")?;
        stream.expect(&PtxToken::Semicolon)?;
        Ok(Trap)
    }
}
