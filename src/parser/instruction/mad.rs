use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, expect_identifier_value, unexpected_value},
    r#type::{common::RegisterOperand, instruction::mad::*},
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

impl PtxParser for Mad {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "mad")?;

        let mode = Mode::parse(stream)?;

        if matches!(mode, Mode::Hi)
            && stream.check(|token| matches!(token, PtxToken::Directive(name) if name == "sat"))
        {
            stream.expect_directive()?; // consume .sat

            let (data, data_span) = stream.expect_directive()?;
            if data.as_str() != "s32" {
                return Err(unexpected_value(data_span, &[".s32"], format!(".{data}")));
            }

            let destination = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let c = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;

            return Ok(Mad::HiSatS32 {
                destination,
                a,
                b,
                c,
            });
        }

        let data_type = DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let c = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Mad::Mode {
            mode,
            data_type,
            destination,
            a,
            b,
            c,
        })
    }
}
