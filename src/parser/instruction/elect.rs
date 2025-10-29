use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::{Operand, PredicateRegister, RegisterOperand},
        instruction::elect::{Destination, Elect},
    },
};

impl PtxParser for Destination {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if stream
            .consume_if(|token| matches!(token, PtxToken::Identifier(name) if name == "_"))
            .is_some()
        {
            return Ok(Destination::Sink);
        }

        if stream.check(|token| matches!(token, PtxToken::Register(_) | PtxToken::LBrace)) {
            return Ok(Destination::Register(RegisterOperand::parse(stream)?));
        }

        let (token, span) = stream.peek()?;
        Err(unexpected_value(
            span.clone(),
            &["register operand", "_"],
            format!("{token:?}"),
        ))
    }
}

impl PtxParser for Elect {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "elect" {
            return Err(unexpected_value(span, &["elect"], opcode));
        }

        let (modifier, span) = stream.expect_directive()?;
        if modifier != "sync" {
            return Err(unexpected_value(span, &[".sync"], format!(".{modifier}")));
        }

        let destination = Destination::parse(stream)?;
        stream.expect(&PtxToken::Pipe)?;
        let predicate = PredicateRegister::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let member_mask = Operand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Elect {
            destination,
            predicate,
            member_mask,
        })
    }
}
