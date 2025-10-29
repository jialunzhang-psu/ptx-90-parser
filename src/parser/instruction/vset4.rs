use crate::{
    lexer::PtxToken,
    parser::{
        PtxParseError, PtxParser, PtxTokenStream, Span, consume_directive_if,
        expect_identifier_value, unexpected_value,
    },
    r#type::{
        common::RegisterOperand,
        instruction::vset4::{
            CompareOp, Destination, Lane, Mask, OperandType, Selector, SourceA, SourceB, Vset4,
        },
    },
};

impl PtxParser for OperandType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_modifier()?;
        match modifier.as_str() {
            "u32" => Ok(OperandType::U32),
            "s32" => Ok(OperandType::S32),
            other => Err(unexpected_value(
                span,
                &[".u32", ".s32"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for CompareOp {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_modifier()?;
        match modifier.as_str() {
            "eq" => Ok(CompareOp::Eq),
            "ne" => Ok(CompareOp::Ne),
            "lt" => Ok(CompareOp::Lt),
            "le" => Ok(CompareOp::Le),
            "gt" => Ok(CompareOp::Gt),
            "ge" => Ok(CompareOp::Ge),
            other => Err(unexpected_value(
                span,
                &[".eq", ".ne", ".lt", ".le", ".gt", ".ge"],
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

impl PtxParser for SourceA {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let register = RegisterOperand::parse(stream)?;
        let selector = parse_optional_selector(stream)?;

        Ok(SourceA { register, selector })
    }
}

impl PtxParser for SourceB {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let register = RegisterOperand::parse(stream)?;
        let selector = parse_optional_selector(stream)?;

        Ok(SourceB { register, selector })
    }
}

impl PtxParser for Vset4 {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "vset4")?;

        let a_type = OperandType::parse(stream)?;
        let b_type = OperandType::parse(stream)?;
        let compare_op = CompareOp::parse(stream)?;

        let is_accumulate = consume_directive_if(stream, "add");

        let destination = Destination::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = SourceA::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = SourceB::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let c = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        if is_accumulate {
            Ok(Vset4::Accumulate {
                a_type,
                b_type,
                compare_op,
                destination,
                a,
                b,
                c,
            })
        } else {
            Ok(Vset4::SimdMerge {
                a_type,
                b_type,
                compare_op,
                destination,
                a,
                b,
                c,
            })
        }
    }
}

fn parse_optional_selector(stream: &mut PtxTokenStream) -> Result<Option<Selector>, PtxParseError> {
    if stream.check(|token| matches!(token, PtxToken::Directive(name) if name.starts_with('b'))) {
        Selector::parse(stream).map(Some)
    } else {
        Ok(None)
    }
}

impl PtxParser for Selector {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_modifier()?;
        parse_selector(&modifier, span)
    }
}

fn parse_selector(name: &str, span: Span) -> Result<Selector, PtxParseError> {
    if !name.starts_with('b') {
        return Err(unexpected_value(
            span,
            &[".bxyzw where x, y, z, w are in 0..7"],
            format!(".{name}"),
        ));
    }

    let mut digits = name[1..].chars();
    let x = parse_lane(digits.next(), &span)?;
    let y = parse_lane(digits.next(), &span)?;
    let z = parse_lane(digits.next(), &span)?;
    let w = parse_lane(digits.next(), &span)?;

    if digits.next().is_some() {
        return Err(unexpected_value(
            span,
            &[".bxyzw where x, y, z, w are in 0..7"],
            format!(".{name}"),
        ));
    }

    Ok(Selector { x, y, z, w })
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
        other => Err(unexpected_value(
            span.clone(),
            &["digit between 0 and 7"],
            other
                .map(|c| c.to_string())
                .unwrap_or_else(|| "missing digit".into()),
        )),
    }
}
