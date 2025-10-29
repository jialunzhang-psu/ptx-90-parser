use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::mbarrier::*},
};

impl PtxParser for SharedSpace {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if consume_directive_if(stream, "shared") {
            if stream.check(|token| matches!(token, PtxToken::Colon)) {
                stream.expect_double_colon()?;
                expect_identifier_value(stream, "cta")?;
                Ok(SharedSpace::SharedCta)
            } else {
                Ok(SharedSpace::Shared)
            }
        } else {
            Ok(SharedSpace::Generic)
        }
    }
}

impl PtxParser for crate::r#type::instruction::mbarrier::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "b64" => Ok(Self::B64),
            other => Err(unexpected_value(span, &[".b64"], format!(".{other}"))),
        }
    }
}

impl PtxParser for Init {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let shared_space = SharedSpace::parse(stream)?;
        let data_type =
            <crate::r#type::instruction::mbarrier::DataType as PtxParser>::parse(stream)?;
        let address = AddressOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let count = Operand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Init {
            shared_space,
            data_type,
            address,
            count,
        })
    }
}

impl PtxParser for Mbarrier {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "mbarrier" {
            return Err(unexpected_value(span, &["mbarrier"], opcode));
        }

        let (modifier, modifier_span) = stream.expect_directive()?;
        match modifier.as_str() {
            "init" => Ok(Mbarrier::Init(Init::parse(stream)?)),
            other => Err(unexpected_value(
                modifier_span,
                &[".init"],
                format!(".{other}"),
            )),
        }
    }
}
