use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::{Immediate, RegisterOperand},
        instruction::alloca::{Alloca, DataType},
    },
};

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "u32" => Ok(DataType::U32),
            "u64" => Ok(DataType::U64),
            other => Err(unexpected_value(
                span,
                &[".u32", ".u64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Alloca {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "alloca" {
            return Err(unexpected_value(span, &["alloca"], opcode));
        }

        let data_type = DataType::parse(stream)?;
        let pointer = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let size = RegisterOperand::parse(stream)?;

        if stream
            .consume_if(|token| matches!(token, PtxToken::Comma))
            .is_some()
        {
            let alignment = Immediate::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;
            return Ok(Alloca::Aligned {
                data_type,
                pointer,
                size,
                alignment,
            });
        }

        stream.expect(&PtxToken::Semicolon)?;
        Ok(Alloca::Default {
            data_type,
            pointer,
            size,
        })
    }
}
