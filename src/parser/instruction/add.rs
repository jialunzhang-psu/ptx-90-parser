use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::add::{Add, DataType},
    },
};

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let saturate = stream
            .consume_if(|token| matches!(token, PtxToken::Directive(name) if name == "sat"))
            .is_some();

        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "u16" => {
                if saturate {
                    return Err(unexpected_value(span, &[".s32"], format!(".{modifier}")));
                }
                Ok(DataType::U16)
            }
            "u32" => {
                if saturate {
                    return Err(unexpected_value(span, &[".s32"], format!(".{modifier}")));
                }
                Ok(DataType::U32)
            }
            "u64" => {
                if saturate {
                    return Err(unexpected_value(span, &[".s32"], format!(".{modifier}")));
                }
                Ok(DataType::U64)
            }
            "s16" => {
                if saturate {
                    return Err(unexpected_value(span, &[".s32"], format!(".{modifier}")));
                }
                Ok(DataType::S16)
            }
            "s32" => Ok(DataType::S32 { saturate }),
            "s64" => {
                if saturate {
                    return Err(unexpected_value(span, &[".s32"], format!(".{modifier}")));
                }
                Ok(DataType::S64)
            }
            "u16x2" => {
                if saturate {
                    return Err(unexpected_value(span, &[".s32"], format!(".{modifier}")));
                }
                Ok(DataType::U16x2)
            }
            "s16x2" => {
                if saturate {
                    return Err(unexpected_value(span, &[".s32"], format!(".{modifier}")));
                }
                Ok(DataType::S16x2)
            }
            other => Err(unexpected_value(
                span,
                &[
                    ".u16", ".u32", ".u64", ".s16", ".s32", ".s64", ".u16x2", ".s16x2",
                ],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Add {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "add" {
            return Err(unexpected_value(span, &["add"], opcode));
        }

        let data_type = DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = Operand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Add {
            data_type,
            destination,
            a,
            b,
        })
    }
}
