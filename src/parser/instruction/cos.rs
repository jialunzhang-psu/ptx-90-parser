use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::cos::*},
};

impl PtxParser for crate::r#type::instruction::cos::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "f32" => Ok(crate::r#type::instruction::cos::DataType::F32),
            other => Err(unexpected_value(span, &[".f32"], format!(".{other}"))),
        }
    }
}

impl PtxParser for Cos {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, opcode_span) = stream.expect_identifier()?;
        if opcode != "cos" {
            return Err(unexpected_value(opcode_span, &["cos"], opcode));
        }

        let (modifier, modifier_span) = stream.expect_modifier()?;
        if modifier != "approx" {
            return Err(unexpected_value(
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

        let data_type = <crate::r#type::instruction::cos::DataType as PtxParser>::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Cos {
            flush_to_zero,
            data_type,
            destination,
            source,
        })
    }
}
