use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{common::RegisterOperand, instruction::madc::*},
};

impl PtxParser for ResultPart {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "hi" => Ok(ResultPart::Hi),
            "lo" => Ok(ResultPart::Lo),
            other => Err(unexpected_value(span, &[".hi", ".lo"], format!(".{other}"))),
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

impl PtxParser for Madc {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "madc" {
            return Err(unexpected_value(span, &["madc"], opcode));
        }

        let result_part = if stream.check(|token| {
            matches!(
                token,
                PtxToken::Directive(name) if matches!(name.as_str(), "hi" | "lo")
            )
        }) {
            Some(ResultPart::parse(stream)?)
        } else {
            None
        };

        let condition_code =
            if stream.check(|token| matches!(token, PtxToken::Directive(name) if name == "cc")) {
                stream.consume()?;
                true
            } else {
                false
            };

        let data_type = DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let multiplicand = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let multiplier = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let addend = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Madc {
            result_part,
            condition_code,
            data_type,
            destination,
            multiplicand,
            multiplier,
            addend,
        })
    }
}
