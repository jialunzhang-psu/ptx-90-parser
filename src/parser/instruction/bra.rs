use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::bra::*},
};

impl PtxParser for Bra {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "bra")?;
        let uniform = consume_directive_if(stream, "uni");
        let target = Label::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;
        Ok(Bra { uniform, target })
    }
}
