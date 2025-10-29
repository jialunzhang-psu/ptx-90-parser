use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::RegisterOperand,
        instruction::fma::{Fma, Rounding},
    },
};

impl PtxParser for Rounding {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "rn" => Ok(Rounding::Rn),
            "rz" => Ok(Rounding::Rz),
            "rm" => Ok(Rounding::Rm),
            "rp" => Ok(Rounding::Rp),
            other => Err(unexpected_value(
                span,
                &[".rn", ".rz", ".rm", ".rp"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Fma {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, opcode_span) = stream.expect_identifier()?;
        if opcode != "fma" {
            return Err(unexpected_value(opcode_span, &["fma"], opcode));
        }

        let rounding = Rounding::parse(stream)?;

        let mut flush_to_zero = false;
        let mut flush_span = None;
        let mut saturate = false;
        let mut saturate_span = None;

        loop {
            if !flush_to_zero {
                if let Some((_, span)) = stream
                    .consume_if(|token| matches!(token, PtxToken::Directive(name) if name == "ftz"))
                {
                    flush_to_zero = true;
                    flush_span = Some(span.clone());
                    continue;
                }
            }

            if !saturate {
                if let Some((_, span)) = stream
                    .consume_if(|token| matches!(token, PtxToken::Directive(name) if name == "sat"))
                {
                    saturate = true;
                    saturate_span = Some(span.clone());
                    continue;
                }
            }

            break;
        }

        let (data_type, data_span) = stream.expect_directive()?;

        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let multiplicand_a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let multiplicand_b = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let addend = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        match data_type.as_str() {
            "f32" => Ok(Fma::F32 {
                rounding,
                flush_to_zero,
                saturate,
                destination,
                multiplicand_a,
                multiplicand_b,
                addend,
            }),
            "f32x2" => {
                if saturate {
                    let span = saturate_span.unwrap_or_else(|| data_span.clone());
                    return Err(unexpected_value(
                        span,
                        &["omit .sat when using .f32x2"],
                        ".sat",
                    ));
                }

                Ok(Fma::F32x2 {
                    rounding,
                    flush_to_zero,
                    destination,
                    multiplicand_a,
                    multiplicand_b,
                    addend,
                })
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
                if saturate {
                    let span = saturate_span.unwrap_or_else(|| data_span.clone());
                    return Err(unexpected_value(
                        span,
                        &["omit .sat when using .f64"],
                        ".sat",
                    ));
                }

                Ok(Fma::F64 {
                    rounding,
                    destination,
                    multiplicand_a,
                    multiplicand_b,
                    addend,
                })
            }
            other => Err(unexpected_value(
                data_span,
                &[".f32", ".f32x2", ".f64"],
                format!(".{other}"),
            )),
        }
    }
}
