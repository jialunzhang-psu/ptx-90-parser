use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::getctarank::*},
};

impl PtxParser for crate::r#type::instruction::getctarank::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "u32" => Ok(crate::r#type::instruction::getctarank::DataType::U32),
            "u64" => Ok(crate::r#type::instruction::getctarank::DataType::U64),
            other => Err(unexpected_value(
                span,
                &[".u32", ".u64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Getctarank {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "getctarank" {
            return Err(unexpected_value(span, &["getctarank"], opcode));
        }

        let shared_cluster = if consume_directive_if(stream, "shared") {
            stream.expect_double_colon()?;
            let (modifier, modifier_span) = stream.expect_identifier()?;
            if modifier != "cluster" {
                return Err(unexpected_value(modifier_span, &["cluster"], modifier));
            }
            true
        } else {
            false
        };

        let data_type = crate::r#type::instruction::getctarank::DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;

        if shared_cluster {
            if stream.check(|token| matches!(token, PtxToken::Register(_) | PtxToken::LBrace)) {
                let source = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;
                return Ok(Getctarank::SharedRegister {
                    data_type,
                    destination,
                    source,
                });
            }

            let symbol = VariableSymbol::parse(stream)?;
            if stream
                .consume_if(|token| matches!(token, PtxToken::Plus))
                .is_some()
            {
                let immediate = Immediate::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;
                Ok(Getctarank::SharedVariableWithImmediate {
                    data_type,
                    destination,
                    symbol,
                    immediate,
                })
            } else {
                stream.expect(&PtxToken::Semicolon)?;
                Ok(Getctarank::SharedVariable {
                    data_type,
                    destination,
                    symbol,
                })
            }
        } else {
            let source = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Getctarank::Generic {
                data_type,
                destination,
                source,
            })
        }
    }
}
