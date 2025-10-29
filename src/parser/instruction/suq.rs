use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::{RegisterOperand, VariableSymbol},
        instruction::suq::{Operand, Query, Suq},
    },
};

impl PtxParser for Query {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "width" => Ok(Query::Width),
            "height" => Ok(Query::Height),
            "depth" => Ok(Query::Depth),
            "channel_data_type" => Ok(Query::ChannelDataType),
            "channel_order" => Ok(Query::ChannelOrder),
            "array_size" => Ok(Query::ArraySize),
            "memory_layout" => Ok(Query::MemoryLayout),
            other => Err(unexpected_value(
                span,
                &[
                    ".width",
                    ".height",
                    ".depth",
                    ".channel_data_type",
                    ".channel_order",
                    ".array_size",
                    ".memory_layout",
                ],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Operand {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        stream.expect(&PtxToken::LBracket)?;

        let operand = if stream.check(|token| matches!(token, PtxToken::Identifier(_))) {
            Operand::Surface(VariableSymbol::parse(stream)?)
        } else if stream.check(|token| matches!(token, PtxToken::Register(_) | PtxToken::LBrace)) {
            Operand::Register(RegisterOperand::parse(stream)?)
        } else {
            let (token, span) = stream.peek()?;
            return Err(unexpected_value(
                span.clone(),
                &["surface identifier", "register operand"],
                format!("{token:?}"),
            ));
        };

        stream.expect(&PtxToken::RBracket)?;
        Ok(operand)
    }
}

impl PtxParser for Suq {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "suq" {
            return Err(unexpected_value(span, &["suq"], opcode));
        }

        let query = Query::parse(stream)?;

        let (data_type, data_span) = stream.expect_directive()?;
        if data_type.as_str() != "b32" {
            return Err(unexpected_value(
                data_span,
                &[".b32"],
                format!(".{data_type}"),
            ));
        }

        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;

        let address = Operand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Suq {
            query,
            destination,
            address,
        })
    }
}
