#[allow(unused_imports)]
use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::ret::*},
};

impl PtxParser for Ret {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "ret")?;
        let instruction = if consume_directive_if(stream, "uni") {
            Ret::Uniform
        } else {
            Ret::Default
        };
        stream.expect(&PtxToken::Semicolon)?;
        Ok(instruction)
    }
}
