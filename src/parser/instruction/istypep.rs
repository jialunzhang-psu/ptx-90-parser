use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::istypep::*},
};

impl PtxParser for crate::r#type::instruction::istypep::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "texref" => Ok(crate::r#type::instruction::istypep::DataType::TexRef),
            "samplerref" => Ok(crate::r#type::instruction::istypep::DataType::SamplerRef),
            "surfref" => Ok(crate::r#type::instruction::istypep::DataType::SurfRef),
            other => Err(unexpected_value(
                span,
                &[".texref", ".samplerref", ".surfref"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Istypep {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "istypep" {
            return Err(unexpected_value(span, &["istypep"], opcode));
        }

        let data_type = crate::r#type::instruction::istypep::DataType::parse(stream)?;
        let predicate = PredicateRegister::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let address = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Istypep {
            data_type,
            predicate,
            address,
        })
    }
}
