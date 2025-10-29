use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::{Operand, PredicateRegister, RegisterOperand},
        instruction::shfl::{DataType, Destination, Mode, Shfl},
    },
};

impl PtxParser for Mode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;

        match modifier.as_str() {
            "up" => Ok(Mode::Up),
            "down" => Ok(Mode::Down),
            "bfly" => Ok(Mode::Bfly),
            "idx" => Ok(Mode::Idx),
            other => Err(unexpected_value(
                span,
                &[".up", ".down", ".bfly", ".idx"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;

        match modifier.as_str() {
            "b32" => Ok(DataType::B32),
            other => Err(unexpected_value(span, &[".b32"], format!(".{other}"))),
        }
    }
}

impl PtxParser for Destination {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let register = RegisterOperand::parse(stream)?;
        let predicate = if stream
            .consume_if(|token| matches!(token, PtxToken::Pipe))
            .is_some()
        {
            Some(PredicateRegister::parse(stream)?)
        } else {
            None
        };

        Ok(Destination {
            register,
            predicate,
        })
    }
}

impl PtxParser for Shfl {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "shfl" {
            return Err(unexpected_value(span, &["shfl"], opcode));
        }

        let mode = Mode::parse(stream)?;
        let data_type = DataType::parse(stream)?;
        let destination = Destination::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let lane = Operand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let clamp = Operand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Shfl {
            mode,
            data_type,
            destination,
            source,
            lane,
            clamp,
        })
    }
}
