use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::AddressOperand,
        instruction::applypriority::{Applypriority, EvictionPriority, Size},
    },
};

impl PtxParser for EvictionPriority {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "L2" => {
                stream.expect_double_colon()?;
                let (identifier, identifier_span) = stream.expect_identifier()?;
                match identifier.as_str() {
                    "evict_normal" => Ok(EvictionPriority::L2EvictNormal),
                    other => Err(unexpected_value(identifier_span, &["evict_normal"], other)),
                }
            }
            other => Err(unexpected_value(span, &[".L2"], format!(".{other}"))),
        }
    }
}

impl PtxParser for Size {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (token, span) = stream.consume()?;
        match token {
            PtxToken::DecimalInteger(value) if value == "128" => Ok(Size::B128),
            other => Err(unexpected_value(
                span.clone(),
                &["128"],
                format!("{other:?}"),
            )),
        }
    }
}

impl PtxParser for Applypriority {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "applypriority" {
            return Err(unexpected_value(span, &["applypriority"], opcode));
        }

        let global = stream
            .consume_if(|token| {
                if let PtxToken::Directive(name) = token {
                    name == "global"
                } else {
                    false
                }
            })
            .is_some();

        if let Ok((token, span)) = stream.peek() {
            if matches!(token, PtxToken::Directive(name) if name == "level") {
                return Err(unexpected_value(
                    span.clone(),
                    &[".L2"],
                    ".level".to_string(),
                ));
            }
        }

        let eviction_priority = EvictionPriority::parse(stream)?;
        let address = AddressOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let size = Size::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Applypriority {
            global,
            eviction_priority,
            address,
            size,
        })
    }
}
