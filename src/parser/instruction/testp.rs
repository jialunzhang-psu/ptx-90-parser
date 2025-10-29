use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::testp::*},
};

impl PtxParser for PredicateTest {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "finite" => Ok(Self::Finite),
            "infinite" => Ok(Self::Infinite),
            "number" => Ok(Self::Number),
            "notanumber" => Ok(Self::NotANumber),
            "normal" => Ok(Self::Normal),
            "subnormal" => Ok(Self::Subnormal),
            other => Err(unexpected_value(
                span,
                &[
                    ".finite",
                    ".infinite",
                    ".number",
                    ".notanumber",
                    ".normal",
                    ".subnormal",
                ],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for crate::r#type::instruction::testp::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            other => Err(unexpected_value(
                span,
                &[".f32", ".f64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Testp {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "testp" {
            return Err(unexpected_value(span, &["testp"], opcode));
        }

        let test = crate::r#type::instruction::testp::PredicateTest::parse(stream)?;
        let data_type = crate::r#type::instruction::testp::DataType::parse(stream)?;
        let destination = PredicateRegister::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Testp {
            test,
            data_type,
            destination,
            source,
        })
    }
}
