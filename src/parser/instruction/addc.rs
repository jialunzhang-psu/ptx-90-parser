use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::RegisterOperand,
        instruction::addc::{Addc, ConditionCode, DataType},
    },
};

impl PtxParser for ConditionCode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if stream.check(|token| matches!(token, PtxToken::Directive(name) if name == "cc")) {
            stream.consume()?;
            Ok(ConditionCode::Cc)
        } else {
            Ok(ConditionCode::None)
        }
    }
}

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "u32" => Ok(DataType::U32),
            "s32" => Ok(DataType::S32),
            "u64" => Ok(DataType::U64),
            "s64" => Ok(DataType::S64),
            other => Err(unexpected_value(
                span,
                &[".u32", ".s32", ".u64", ".s64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Addc {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "addc" {
            return Err(unexpected_value(span, &["addc"], opcode));
        }

        let condition_code = ConditionCode::parse(stream)?;
        let data_type = DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let augend = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let addend = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Addc {
            condition_code,
            data_type,
            destination,
            augend,
            addend,
        })
    }
}
