use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, Span, unexpected_value},
    r#type::{
        common::RegisterOperand,
        instruction::vop2::{
            ASource, Accumulate, BSource, DataType, Destination, Half, Mask, Merge, Operation,
            Selector, Vop2,
        },
    },
};

impl PtxParser for Operation {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        match opcode.as_str() {
            "vadd2" => Ok(Operation::Vadd2),
            "vsub2" => Ok(Operation::Vsub2),
            "vavrg2" => Ok(Operation::Vavrg2),
            "vabsdiff2" => Ok(Operation::Vabsdiff2),
            "vmin2" => Ok(Operation::Vmin2),
            "vmax2" => Ok(Operation::Vmax2),
            other => Err(unexpected_value(
                span,
                &["vadd2", "vsub2", "vavrg2", "vabsdiff2", "vmin2", "vmax2"],
                other,
            )),
        }
    }
}

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_modifier()?;
        match modifier.as_str() {
            "u32" => Ok(DataType::U32),
            "s32" => Ok(DataType::S32),
            other => Err(unexpected_value(
                span,
                &[".u32", ".s32"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Mask {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_modifier()?;
        match modifier.as_str() {
            "h0" => Ok(Mask::H0),
            "h1" => Ok(Mask::H1),
            "h10" => Ok(Mask::H10),
            other => Err(unexpected_value(
                span,
                &[".h0", ".h1", ".h10"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Selector {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_modifier()?;
        parse_selector_modifier(&modifier, span)
    }
}

impl PtxParser for Destination {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let register = RegisterOperand::parse(stream)?;
        let mask = if stream
            .check(|token| matches!(token, PtxToken::Directive(name) if name.starts_with('h')))
        {
            Some(Mask::parse(stream)?)
        } else {
            None
        };

        Ok(Destination { register, mask })
    }
}

impl PtxParser for ASource {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let register = RegisterOperand::parse(stream)?;
        let selector = if stream
            .check(|token| matches!(token, PtxToken::Directive(name) if name.starts_with('h')))
        {
            Some(Selector::parse(stream)?)
        } else {
            None
        };

        Ok(ASource { register, selector })
    }
}

impl PtxParser for BSource {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let register = RegisterOperand::parse(stream)?;
        let selector = if stream
            .check(|token| matches!(token, PtxToken::Directive(name) if name.starts_with('h')))
        {
            Some(Selector::parse(stream)?)
        } else {
            None
        };

        Ok(BSource { register, selector })
    }
}

impl PtxParser for Vop2 {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let operation = Operation::parse(stream)?;
        let dtype = DataType::parse(stream)?;
        let atype = DataType::parse(stream)?;
        let btype = DataType::parse(stream)?;

        let mut saturate = false;
        let mut sat_span: Option<Span> = None;
        if let Some((_, span)) =
            stream.consume_if(|token| matches!(token, PtxToken::Directive(name) if name == "sat"))
        {
            saturate = true;
            sat_span = Some(span.clone());
        }

        let mut is_accumulate = false;
        if stream
            .consume_if(|token| matches!(token, PtxToken::Directive(name) if name == "add"))
            .is_some()
        {
            is_accumulate = true;
        }

        if is_accumulate && saturate {
            let span = sat_span.unwrap();
            return Err(unexpected_value(
                span,
                &["omit .sat when using .add"],
                ".sat",
            ));
        }

        let destination = Destination::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = ASource::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = BSource::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let c = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        if is_accumulate {
            Ok(Vop2::Accumulate(Accumulate {
                operation,
                dtype,
                atype,
                btype,
                destination,
                a,
                b,
                c,
            }))
        } else {
            Ok(Vop2::Merge(Merge {
                operation,
                dtype,
                atype,
                btype,
                saturate,
                destination,
                a,
                b,
                c,
            }))
        }
    }
}

fn parse_selector_modifier(name: &str, span: Span) -> Result<Selector, PtxParseError> {
    if !name.starts_with('h') {
        return Err(unexpected_value(
            span,
            &[".hxy where x and y are in 0..3"],
            format!(".{name}"),
        ));
    }

    let digits = &name[1..];
    let mut chars = digits.chars();
    let first = chars.next();
    let second = chars.next();

    if first.is_none() || second.is_none() || chars.next().is_some() {
        return Err(unexpected_value(
            span,
            &[".hxy where x and y are in 0..3"],
            format!(".{name}"),
        ));
    }

    let first_half = parse_half_char(first.unwrap(), span.clone())?;
    let second_half = parse_half_char(second.unwrap(), span)?;

    Ok(Selector {
        halves: [first_half, second_half],
    })
}

fn parse_half_char(ch: char, span: Span) -> Result<Half, PtxParseError> {
    match ch {
        '0' => Ok(Half::H0),
        '1' => Ok(Half::H1),
        '2' => Ok(Half::H2),
        '3' => Ok(Half::H3),
        other => Err(unexpected_value(
            span,
            &["digits 0 through 3"],
            other.to_string(),
        )),
    }
}
