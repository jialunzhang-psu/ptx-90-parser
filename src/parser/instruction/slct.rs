use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::RegisterOperand,
        instruction::slct::{DataType, Slct},
    },
};

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "b16" => Ok(DataType::B16),
            "b32" => Ok(DataType::B32),
            "b64" => Ok(DataType::B64),
            "u16" => Ok(DataType::U16),
            "u32" => Ok(DataType::U32),
            "u64" => Ok(DataType::U64),
            "s16" => Ok(DataType::S16),
            "s32" => Ok(DataType::S32),
            "s64" => Ok(DataType::S64),
            "f32" => Ok(DataType::F32),
            "f64" => Ok(DataType::F64),
            other => Err(unexpected_value(
                span,
                &[
                    ".b16", ".b32", ".b64", ".u16", ".u32", ".u64", ".s16", ".s32", ".s64", ".f32",
                    ".f64",
                ],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Slct {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "slct" {
            return Err(unexpected_value(span, &["slct"], opcode));
        }

        let ftz_token =
            stream.consume_if(|token| matches!(token, PtxToken::Directive(name) if name == "ftz"));
        let flush_to_zero = ftz_token.is_some();
        let ftz_span = ftz_token.map(|(_, span)| span.clone());

        let data_type = DataType::parse(stream)?;
        let (selector_modifier, selector_span) = stream.expect_directive()?;

        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let on_true = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let on_false = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let selector = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        match selector_modifier.as_str() {
            "s32" => {
                if flush_to_zero {
                    return Err(unexpected_value(
                        ftz_span.expect("ftz span must be present when ftz is set"),
                        &[".f32"],
                        ".ftz",
                    ));
                }

                Ok(Slct::S32 {
                    data_type,
                    destination,
                    on_true,
                    on_false,
                    selector,
                })
            }
            "f32" => Ok(Slct::F32 {
                flush_to_zero,
                data_type,
                destination,
                on_true,
                on_false,
                selector,
            }),
            other => Err(unexpected_value(
                selector_span,
                &[".s32", ".f32"],
                format!(".{other}"),
            )),
        }
    }
}
