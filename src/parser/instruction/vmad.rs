use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::RegisterOperand, instruction::vmad::*},
};

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
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

impl PtxParser for Scale {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "shr7" => Ok(Scale::Shr7),
            "shr15" => Ok(Scale::Shr15),
            other => Err(unexpected_value(
                span,
                &[".shr7", ".shr15"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for ComponentSelect {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "b0" => Ok(ComponentSelect::B0),
            "b1" => Ok(ComponentSelect::B1),
            "b2" => Ok(ComponentSelect::B2),
            "b3" => Ok(ComponentSelect::B3),
            "h0" => Ok(ComponentSelect::H0),
            "h1" => Ok(ComponentSelect::H1),
            other => Err(unexpected_value(
                span,
                &[".b0", ".b1", ".b2", ".b3", ".h0", ".h1"],
                format!(".{other}"),
            )),
        }
    }
}

fn parse_optional_component_select(
    stream: &mut PtxTokenStream,
) -> Result<Option<ComponentSelect>, PtxParseError> {
    if stream.check(|token| {
        matches!(
            token,
            PtxToken::Directive(name)
                if matches!(
                    name.as_str(),
                    "b0" | "b1" | "b2" | "b3" | "h0" | "h1"
                )
        )
    }) {
        ComponentSelect::parse(stream).map(Some)
    } else {
        Ok(None)
    }
}

fn parse_optional_scale(stream: &mut PtxTokenStream) -> Result<Option<Scale>, PtxParseError> {
    if stream.check(|token| {
        matches!(
            token,
            PtxToken::Directive(name) if matches!(name.as_str(), "shr7" | "shr15")
        )
    }) {
        Scale::parse(stream).map(Some)
    } else {
        Ok(None)
    }
}

fn parse_standard_operands(
    stream: &mut PtxTokenStream,
) -> Result<
    (
        RegisterOperand,
        bool,
        RegisterOperand,
        Option<ComponentSelect>,
        bool,
        RegisterOperand,
        Option<ComponentSelect>,
        bool,
        RegisterOperand,
    ),
    PtxParseError,
> {
    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;

    let a_negated = stream
        .consume_if(|token| matches!(token, PtxToken::Minus))
        .is_some();
    let a = RegisterOperand::parse(stream)?;
    let asel = parse_optional_component_select(stream)?;
    stream.expect(&PtxToken::Comma)?;

    let b_negated = stream
        .consume_if(|token| matches!(token, PtxToken::Minus))
        .is_some();
    let b = RegisterOperand::parse(stream)?;
    let bsel = parse_optional_component_select(stream)?;
    stream.expect(&PtxToken::Comma)?;

    let c_negated = stream
        .consume_if(|token| matches!(token, PtxToken::Minus))
        .is_some();
    let c = RegisterOperand::parse(stream)?;

    Ok((
        destination,
        a_negated,
        a,
        asel,
        b_negated,
        b,
        bsel,
        c_negated,
        c,
    ))
}

fn parse_plus_one_operands(
    stream: &mut PtxTokenStream,
) -> Result<
    (
        RegisterOperand,
        RegisterOperand,
        Option<ComponentSelect>,
        RegisterOperand,
        Option<ComponentSelect>,
        RegisterOperand,
    ),
    PtxParseError,
> {
    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;

    let a = RegisterOperand::parse(stream)?;
    let asel = parse_optional_component_select(stream)?;
    stream.expect(&PtxToken::Comma)?;

    let b = RegisterOperand::parse(stream)?;
    let bsel = parse_optional_component_select(stream)?;
    stream.expect(&PtxToken::Comma)?;

    let c = RegisterOperand::parse(stream)?;

    Ok((destination, a, asel, b, bsel, c))
}

impl PtxParser for Vmad {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "vmad")?;

        let dtype = DataType::parse(stream)?;
        let atype = DataType::parse(stream)?;
        let btype = DataType::parse(stream)?;

        let is_plus_one = consume_directive_if(stream, "po");

        let mut saturate = false;
        let mut scale = None;
        loop {
            if !saturate && consume_directive_if(stream, "sat") {
                saturate = true;
                continue;
            }

            if scale.is_none() {
                if let Some(value) = parse_optional_scale(stream)? {
                    scale = Some(value);
                    continue;
                }
            }

            break;
        }

        if is_plus_one {
            let (destination, a, asel, b, bsel, c) = parse_plus_one_operands(stream)?;
            stream.expect(&PtxToken::Semicolon)?;

            Ok(Vmad::PlusOne(PlusOne {
                dtype,
                atype,
                btype,
                saturate,
                scale,
                destination,
                a,
                asel,
                b,
                bsel,
                c,
            }))
        } else {
            let (destination, a_negated, a, asel, b_negated, b, bsel, c_negated, c) =
                parse_standard_operands(stream)?;
            stream.expect(&PtxToken::Semicolon)?;

            Ok(Vmad::Standard(Standard {
                dtype,
                atype,
                btype,
                saturate,
                scale,
                destination,
                a_negated,
                a,
                asel,
                b_negated,
                b,
                bsel,
                c_negated,
                c,
            }))
        }
    }
}
