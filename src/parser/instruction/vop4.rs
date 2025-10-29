use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, Span, unexpected_value},
    r#type::{
        common::RegisterOperand,
        instruction::vop4::{
            ASource, Accumulate, BSource, DataType, Destination, Lane, Mask, Merge, Operation,
            Selector, Vop4,
        },
    },
};

impl PtxParser for Operation {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        match opcode.as_str() {
            "vadd4" => Ok(Operation::Vadd4),
            "vsub4" => Ok(Operation::Vsub4),
            "vavrg4" => Ok(Operation::Vavrg4),
            "vabsdiff4" => Ok(Operation::Vabsdiff4),
            "vmin4" => Ok(Operation::Vmin4),
            "vmax4" => Ok(Operation::Vmax4),
            other => Err(unexpected_value(
                span,
                &["vadd4", "vsub4", "vavrg4", "vabsdiff4", "vmin4", "vmax4"],
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
            "b0" => Ok(Mask::B0),
            "b1" => Ok(Mask::B1),
            "b10" => Ok(Mask::B10),
            "b2" => Ok(Mask::B2),
            "b20" => Ok(Mask::B20),
            "b21" => Ok(Mask::B21),
            "b210" => Ok(Mask::B210),
            "b3" => Ok(Mask::B3),
            "b30" => Ok(Mask::B30),
            "b31" => Ok(Mask::B31),
            "b310" => Ok(Mask::B310),
            "b32" => Ok(Mask::B32),
            "b320" => Ok(Mask::B320),
            "b321" => Ok(Mask::B321),
            "b3210" => Ok(Mask::B3210),
            other => Err(unexpected_value(
                span,
                &[
                    ".b0", ".b1", ".b10", ".b2", ".b20", ".b21", ".b210", ".b3", ".b30", ".b31",
                    ".b310", ".b32", ".b320", ".b321", ".b3210",
                ],
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
            .check(|token| matches!(token, PtxToken::Directive(name) if name.starts_with('b')))
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
            .check(|token| matches!(token, PtxToken::Directive(name) if name.starts_with('b')))
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
            .check(|token| matches!(token, PtxToken::Directive(name) if name.starts_with('b')))
        {
            Some(Selector::parse(stream)?)
        } else {
            None
        };

        Ok(BSource { register, selector })
    }
}

impl PtxParser for Vop4 {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let operation = Operation::parse(stream)?;
        let data_type = DataType::parse(stream)?;
        let a_type = DataType::parse(stream)?;
        let b_type = DataType::parse(stream)?;

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
            Ok(Vop4::Accumulate(Accumulate {
                operation,
                data_type,
                a_type,
                b_type,
                destination,
                a,
                b,
                c,
            }))
        } else {
            Ok(Vop4::Merge(Merge {
                operation,
                data_type,
                a_type,
                b_type,
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
    if !name.starts_with('b') {
        return Err(unexpected_value(
            span,
            &[".bxyzw where x, y, z, w are in 0..7"],
            format!(".{name}"),
        ));
    }

    let digits = &name[1..];
    let mut chars = digits.chars();
    let lanes = [
        parse_lane(chars.next(), &span)?,
        parse_lane(chars.next(), &span)?,
        parse_lane(chars.next(), &span)?,
        parse_lane(chars.next(), &span)?,
    ];

    if chars.next().is_some() {
        return Err(unexpected_value(
            span,
            &[".bxyzw where x, y, z, w are in 0..7"],
            format!(".{name}"),
        ));
    }

    Ok(Selector { lanes })
}

fn parse_lane(ch: Option<char>, span: &Span) -> Result<Lane, PtxParseError> {
    match ch {
        Some('0') => Ok(Lane::B0),
        Some('1') => Ok(Lane::B1),
        Some('2') => Ok(Lane::B2),
        Some('3') => Ok(Lane::B3),
        Some('4') => Ok(Lane::B4),
        Some('5') => Ok(Lane::B5),
        Some('6') => Ok(Lane::B6),
        Some('7') => Ok(Lane::B7),
        _ => Err(unexpected_value(
            span.clone(),
            &["digit between 0 and 7"],
            ch.map(|c| c.to_string())
                .unwrap_or_else(|| "missing digit".into()),
        )),
    }
}
