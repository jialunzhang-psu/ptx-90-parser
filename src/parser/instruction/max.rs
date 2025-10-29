use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::RegisterOperand,
        instruction::max::{AType, BType, Max, Relu},
    },
};

impl AType {
    fn from_modifier(modifier: &str) -> Option<Self> {
        match modifier {
            "u16" => Some(AType::U16),
            "u32" => Some(AType::U32),
            "u64" => Some(AType::U64),
            "u16x2" => Some(AType::U16x2),
            "s16" => Some(AType::S16),
            "s64" => Some(AType::S64),
            _ => None,
        }
    }
}

impl BType {
    fn from_modifier(modifier: &str) -> Option<Self> {
        match modifier {
            "s16x2" => Some(BType::S16x2),
            "s32" => Some(BType::S32),
            _ => None,
        }
    }
}

impl PtxParser for AType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        AType::from_modifier(&modifier).ok_or_else(|| {
            unexpected_value(
                span,
                &[".u16", ".u32", ".u64", ".u16x2", ".s16", ".s64"],
                format!(".{modifier}"),
            )
        })
    }
}

impl PtxParser for BType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        BType::from_modifier(&modifier)
            .ok_or_else(|| unexpected_value(span, &[".s16x2", ".s32"], format!(".{modifier}")))
    }
}

impl PtxParser for Max {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "max" {
            return Err(unexpected_value(span, &["max"], opcode));
        }

        let relu_token =
            stream.consume_if(|token| matches!(token, PtxToken::Directive(name) if name == "relu"));
        let relu = if relu_token.is_some() {
            Relu::Relu
        } else {
            Relu::Default
        };
        let relu_span = relu_token.map(|(_, span)| span.clone());

        let (modifier, span) = stream.expect_directive()?;

        if let Some(data_type) = AType::from_modifier(&modifier) {
            if matches!(relu, Relu::Relu) {
                return Err(unexpected_value(
                    relu_span.expect("relu span available"),
                    &[".s16x2", ".s32"],
                    ".relu".to_string(),
                ));
            }

            let destination = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;

            Ok(Max::AType {
                data_type,
                destination,
                a,
                b,
            })
        } else if let Some(data_type) = BType::from_modifier(&modifier) {
            let destination = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;

            Ok(Max::BType {
                relu,
                data_type,
                destination,
                a,
                b,
            })
        } else {
            Err(unexpected_value(
                span,
                &[
                    ".u16", ".u32", ".u64", ".u16x2", ".s16", ".s64", ".s16x2", ".s32",
                ],
                format!(".{modifier}"),
            ))
        }
    }
}
