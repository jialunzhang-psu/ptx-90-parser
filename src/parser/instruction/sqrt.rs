use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::RegisterOperand,
        instruction::sqrt::{Rounding, Sqrt},
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

impl PtxParser for Sqrt {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, opcode_span) = stream.expect_identifier()?;
        if opcode != "sqrt" {
            return Err(unexpected_value(opcode_span, &["sqrt"], opcode));
        }

        let (modifier, modifier_span) = stream.expect_modifier()?;
        match modifier.as_str() {
            "approx" => parse_approx(stream),
            "rnd" => parse_rnd(stream),
            other => Err(unexpected_value(
                modifier_span,
                &[".approx", ".rnd"],
                format!(".{other}"),
            )),
        }
    }
}

fn parse_approx(stream: &mut PtxTokenStream) -> Result<Sqrt, PtxParseError> {
    let flush_to_zero =
        if stream.check(|token| matches!(token, PtxToken::Directive(value) if value == "ftz")) {
            stream.consume()?;
            true
        } else {
            false
        };

    let (data_type, data_span) = stream.expect_directive()?;
    if data_type.as_str() != "f32" {
        return Err(unexpected_value(
            data_span,
            &[".f32"],
            format!(".{data_type}"),
        ));
    }

    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let source = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Sqrt::ApproxF32 {
        flush_to_zero,
        destination,
        source,
    })
}

fn parse_rnd(stream: &mut PtxTokenStream) -> Result<Sqrt, PtxParseError> {
    let rounding = Rounding::parse(stream)?;

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

            Ok(Sqrt::RndF32 {
                rounding,
                flush_to_zero,
                destination,
                source,
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

            let destination = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let source = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;

            Ok(Sqrt::RndF64 {
                rounding,
                destination,
                source,
            })
        }
        other => Err(unexpected_value(
            data_span,
            &[".f32", ".f64"],
            format!(".{other}"),
        )),
    }
}
