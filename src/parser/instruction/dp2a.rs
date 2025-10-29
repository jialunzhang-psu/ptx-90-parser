use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::RegisterOperand,
        instruction::dp2a::{DataType, Dp2a, Mode},
    },
};

impl PtxParser for Mode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "lo" => Ok(Mode::Lo),
            "hi" => Ok(Mode::Hi),
            other => Err(unexpected_value(span, &[".lo", ".hi"], format!(".{other}"))),
        }
    }
}

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "u32" => Ok(DataType::U32),
            "s32" => Ok(DataType::S32),
            other => Err(unexpected_value(
                span,
                &[".u32", ".s32"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Dp2a {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "dp2a" {
            return Err(unexpected_value(span, &["dp2a"], opcode));
        }

        let mode = Mode::parse(stream)?;
        let atype = DataType::parse(stream)?;
        let btype = DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let c = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Dp2a {
            mode,
            atype,
            btype,
            destination,
            a,
            b,
            c,
        })
    }
}
