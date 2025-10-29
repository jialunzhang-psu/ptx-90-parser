use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::tanh::*},
};

impl PtxParser for Approximation {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_modifier()?;
        match modifier.as_str() {
            "approx" => Ok(Approximation::Approx),
            other => Err(unexpected_value(span, &[".approx"], format!(".{other}"))),
        }
    }
}

impl PtxParser for crate::r#type::instruction::tanh::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "f32" => Ok(crate::r#type::instruction::tanh::DataType::F32),
            other => Err(unexpected_value(span, &[".f32"], format!(".{other}"))),
        }
    }
}

impl PtxParser for Tanh {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, opcode_span) = stream.expect_identifier()?;
        if opcode != "tanh" {
            return Err(unexpected_value(opcode_span, &["tanh"], opcode));
        }

        let approximation = Approximation::parse(stream)?;
        let data_type = crate::r#type::instruction::tanh::DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Tanh {
            approximation,
            data_type,
            destination,
            source,
        })
    }
}
