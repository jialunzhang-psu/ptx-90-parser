use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{common::RegisterOperand, instruction::prmt::*},
};

impl PtxParser for Mode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;

        match modifier.as_str() {
            "f4e" => Ok(Mode::F4e),
            "b4e" => Ok(Mode::B4e),
            "rc8" => Ok(Mode::Rc8),
            "ecl" => Ok(Mode::Ecl),
            "ecr" => Ok(Mode::Ecr),
            "rc16" => Ok(Mode::Rc16),
            other => Err(unexpected_value(
                span,
                &[".f4e", ".b4e", ".rc8", ".ecl", ".ecr", ".rc16"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Prmt {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "prmt" {
            return Err(unexpected_value(span, &["prmt"], opcode));
        }

        let (data_type, data_type_span) = stream.expect_directive()?;
        if data_type != "b32" {
            return Err(unexpected_value(
                data_type_span,
                &[".b32"],
                format!(".{data_type}"),
            ));
        }

        let mode = if stream.check(|token| matches!(token, PtxToken::Directive(_))) {
            Some(Mode::parse(stream)?)
        } else {
            None
        };

        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let c = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Prmt {
            mode,
            destination,
            a,
            b,
            c,
        })
    }
}
