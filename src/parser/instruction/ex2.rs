use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream},
    r#type::{
        common::RegisterOperand,
        instruction::ex2::{DataType, Ex2},
    },
};

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "f32" => Ok(DataType::F32),
            other => Err(crate::parser::unexpected_value(
                span,
                &[".f32"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Ex2 {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, opcode_span) = stream.expect_identifier()?;
        if opcode != "ex2" {
            return Err(crate::parser::unexpected_value(
                opcode_span,
                &["ex2"],
                opcode,
            ));
        }

        let (modifier, modifier_span) = stream.expect_modifier()?;
        if modifier != "approx" {
            return Err(crate::parser::unexpected_value(
                modifier_span,
                &[".approx"],
                format!(".{modifier}"),
            ));
        }

        let flush_to_zero = if stream
            .check(|token| matches!(token, PtxToken::Directive(value) if value == "ftz"))
        {
            stream.consume()?;
            true
        } else {
            false
        };

        let data_type = DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Ex2 {
            flush_to_zero,
            data_type,
            destination,
            source,
        })
    }
}
