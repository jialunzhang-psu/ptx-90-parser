use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::brx::*},
};

impl PtxParser for Brx {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "brx")?;
        expect_directive_value(stream, "idx")?;
        let uniform = consume_directive_if(stream, "uni");
        let index = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let targets = Label::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;
        Ok(Brx {
            uniform,
            index,
            targets,
        })
    }
}
