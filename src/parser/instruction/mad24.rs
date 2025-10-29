use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::mad24::*},
};

impl PtxParser for crate::r#type::instruction::mad24::Mode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if !stream.check(|token| {
            matches!(
                token,
                PtxToken::Directive(name) if matches!(name.as_str(), "hi" | "lo")
            )
        }) {
            return Ok(Mode::Lo);
        }

        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "hi" => Ok(Mode::Hi),
            "lo" => Ok(Mode::Lo),
            other => Err(unexpected_value(span, &[".hi", ".lo"], format!(".{other}"))),
        }
    }
}

impl PtxParser for crate::r#type::instruction::mad24::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "u32" => Ok(Self::U32),
            "s32" => Ok(Self::S32),
            other => Err(unexpected_value(
                span,
                &[".u32", ".s32"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for crate::r#type::instruction::mad24::Mad24 {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "mad24")?;

        let mode = crate::r#type::instruction::mad24::Mode::parse(stream)?;

        if matches!(mode, Mode::Hi)
            && stream.check(|token| matches!(token, PtxToken::Directive(name) if name == "sat"))
        {
            stream.expect_directive()?; // consume .sat

            let (data_type, data_span) = stream.expect_directive()?;
            if data_type.as_str() != "s32" {
                return Err(unexpected_value(
                    data_span,
                    &[".s32"],
                    format!(".{data_type}"),
                ));
            }

            let destination = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let c = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;

            return Ok(Mad24::HiSatS32 {
                destination,
                a,
                b,
                c,
            });
        }

        let data_type = crate::r#type::instruction::mad24::DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let c = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Mad24::Mode {
            mode,
            data_type,
            destination,
            a,
            b,
            c,
        })
    }
}
