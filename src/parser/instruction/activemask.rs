use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{common::RegisterOperand, instruction::activemask::Activemask},
};

impl PtxParser for Activemask {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "activemask" {
            return Err(unexpected_value(span, &["activemask"], opcode));
        }

        let (modifier, span) = stream.expect_directive()?;
        if modifier != "b32" {
            return Err(unexpected_value(span, &[".b32"], format!(".{modifier}")));
        }

        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Activemask { destination })
    }
}
