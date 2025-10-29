use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::r#match::*},
};

impl PtxParser for crate::r#type::instruction::r#match::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "b32" => Ok(crate::r#type::instruction::r#match::DataType::B32),
            "b64" => Ok(crate::r#type::instruction::r#match::DataType::B64),
            other => Err(unexpected_value(
                span,
                &[".b32", ".b64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Match {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "match" {
            return Err(unexpected_value(span, &["match"], opcode));
        }

        let (mode, mode_span) = stream.expect_directive()?;
        match mode.as_str() {
            "any" => {
                expect_directive_value(stream, "sync")?;
                consume_directive_if(stream, "type");
                let data_type = crate::r#type::instruction::r#match::DataType::parse(stream)?;
                let destination = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let source = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let member_mask = Operand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;

                Ok(Match::Any(Any {
                    data_type,
                    destination,
                    source,
                    member_mask,
                }))
            }
            "all" => {
                expect_directive_value(stream, "sync")?;
                consume_directive_if(stream, "type");
                let data_type = crate::r#type::instruction::r#match::DataType::parse(stream)?;
                let destination = RegisterOperand::parse(stream)?;
                let predicate = if stream
                    .consume_if(|token| matches!(token, PtxToken::Pipe))
                    .is_some()
                {
                    Some(PredicateRegister::parse(stream)?)
                } else {
                    None
                };
                stream.expect(&PtxToken::Comma)?;
                let source = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let member_mask = Operand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;

                Ok(Match::All(All {
                    data_type,
                    destination,
                    predicate,
                    source,
                    member_mask,
                }))
            }
            other => Err(unexpected_value(
                mode_span,
                &[".any", ".all"],
                format!(".{other}"),
            )),
        }
    }
}
