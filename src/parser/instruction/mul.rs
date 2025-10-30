use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::RegisterOperand,
        instruction::mul::{DataType, Mode, Mul},
    },
};

impl PtxParser for Mode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "hi" => Ok(Mode::Hi),
            "lo" => Ok(Mode::Lo),
            "wide" => Ok(Mode::Wide),
            other => Err(unexpected_value(
                span,
                &[".hi", ".lo", ".wide"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "u16" => Ok(DataType::U16),
            "u32" => Ok(DataType::U32),
            "u64" => Ok(DataType::U64),
            "s16" => Ok(DataType::S16),
            "s32" => Ok(DataType::S32),
            "s64" => Ok(DataType::S64),
            other => Err(unexpected_value(
                span,
                &[".u16", ".u32", ".u64", ".s16", ".s32", ".s64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Mul {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "mul" {
            return Err(unexpected_value(span, &["mul"], opcode));
        }

        let mode = Mode::parse(stream)?;
        let data_type = DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let lhs = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let rhs = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Mul {
            mode,
            data_type,
            destination,
            lhs,
            rhs,
        })
    }
}
