use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::bmsk::*},
};

impl PtxParser for Mode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "clamp" => Ok(Mode::Clamp),
            "wrap" => Ok(Mode::Wrap),
            other => Err(unexpected_value(
                span,
                &[".clamp", ".wrap"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Bmsk {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "bmsk" {
            return Err(unexpected_value(span, &["bmsk"], opcode));
        }

        let mode = Mode::parse(stream)?;
        let (data_type, span) = stream.expect_directive()?;
        if data_type != "b32" {
            return Err(unexpected_value(span, &[".b32"], format!(".{data_type}")));
        }

        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = Operand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = Operand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Bmsk {
            mode,
            destination,
            a,
            b,
        })
    }
}
