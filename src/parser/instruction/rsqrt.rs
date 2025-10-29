use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::rsqrt::*},
};

impl PtxParser for Rsqrt {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, opcode_span) = stream.expect_identifier()?;
        if opcode != "rsqrt" {
            return Err(unexpected_value(opcode_span, &["rsqrt"], opcode));
        }

        let (modifier, modifier_span) = stream.expect_modifier()?;
        if modifier.as_str() != "approx" {
            return Err(unexpected_value(
                modifier_span,
                &[".approx"],
                format!(".{modifier}"),
            ));
        }

        let mut flush_to_zero = false;
        let mut flush_span = None;
        if let Some((_, span)) =
            stream.consume_if(|token| matches!(token, PtxToken::Directive(value) if value == "ftz"))
        {
            flush_to_zero = true;
            flush_span = Some(span.clone());
        }

        let (data_type, data_span) = stream.expect_directive()?;
        match data_type.as_str() {
            "f32" => {
                let destination = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let source = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;

                Ok(Rsqrt::ApproxF32(ApproxF32 {
                    flush_to_zero,
                    destination,
                    source,
                }))
            }
            "f64" => {
                if flush_to_zero {
                    let span = flush_span.unwrap_or_else(|| data_span.clone());
                    return Err(unexpected_value(
                        span,
                        &["omit .ftz when using .f64"],
                        ".ftz",
                    ));
                }

                let destination = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let source = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;

                Ok(Rsqrt::ApproxF64(ApproxF64 {
                    destination,
                    source,
                }))
            }
            other => Err(unexpected_value(
                data_span,
                &[".f32", ".f64"],
                format!(".{other}"),
            )),
        }
    }
}
