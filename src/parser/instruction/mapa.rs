use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{
        common::{Immediate, Operand, RegisterOperand, VariableSymbol},
        instruction::mapa::{DataType, Mapa},
    },
};

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "u32" => Ok(DataType::U32),
            "u64" => Ok(DataType::U64),
            other => Err(unexpected_value(
                span,
                &[".u32", ".u64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Mapa {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "mapa" {
            return Err(unexpected_value(span, &["mapa"], opcode));
        }

        let mut shared_cluster = false;
        if consume_directive_if(stream, "shared") {
            stream.expect_double_colon()?;
            expect_identifier_value(stream, "cluster")?;
            shared_cluster = true;
        }

        let data_type = DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;

        if !shared_cluster {
            let address = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let cta = Operand::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;

            return Ok(Mapa::Generic {
                data_type,
                destination,
                address,
                cta,
            });
        }

        if stream.check(|token| matches!(token, PtxToken::Register(_) | PtxToken::LBrace)) {
            let address = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let cta = Operand::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;

            return Ok(Mapa::SharedRegister {
                data_type,
                destination,
                address,
                cta,
            });
        }

        let variable = VariableSymbol::parse(stream)?;
        let immediate = if stream
            .consume_if(|token| matches!(token, PtxToken::Plus))
            .is_some()
        {
            Some(Immediate::parse(stream)?)
        } else {
            None
        };
        stream.expect(&PtxToken::Comma)?;
        let cta = Operand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(match immediate {
            Some(immediate) => Mapa::SharedVariableWithImmediate {
                data_type,
                destination,
                variable,
                immediate,
                cta,
            },
            None => Mapa::SharedVariable {
                data_type,
                destination,
                variable,
                cta,
            },
        })
    }
}
