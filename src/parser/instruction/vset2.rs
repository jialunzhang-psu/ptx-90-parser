use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::vset2::*},
};

impl PtxParser for Vset2 {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "vset2")?;

        let a_type = <crate::r#type::instruction::vset2::DataType as PtxParser>::parse(stream)?;
        let b_type = <crate::r#type::instruction::vset2::DataType as PtxParser>::parse(stream)?;
        let comparison = CompareOp::parse(stream)?;

        let is_accumulate = consume_directive_if(stream, "add");

        let destination = Destination::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = ASource::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = BSource::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let c = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        if is_accumulate {
            Ok(Vset2::Accumulate(Accumulate {
                a_type,
                b_type,
                comparison,
                destination,
                a,
                b,
                c,
            }))
        } else {
            Ok(Vset2::SimdMerge(SimdMerge {
                a_type,
                b_type,
                comparison,
                destination,
                a,
                b,
                c,
            }))
        }
    }
}

impl PtxParser for crate::r#type::instruction::vset2::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_modifier()?;
        match modifier.as_str() {
            "u32" => Ok(crate::r#type::instruction::vset2::DataType::U32),
            "s32" => Ok(crate::r#type::instruction::vset2::DataType::S32),
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

impl PtxParser for crate::r#type::instruction::vset2::Half {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_modifier()?;
        if modifier.len() != 2 || !modifier.starts_with('h') {
            return Err(unexpected_value(
                span.clone(),
                &[".h0", ".h1", ".h2", ".h3"],
                format!(".{modifier}"),
            ));
        }

        let digit = modifier.chars().nth(1).unwrap();
        parse_half_char(digit, span)
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
        let mask = if stream.check(is_mask_token) {
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
        let selector = if stream.check(is_selector_token) {
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
        let selector = if stream.check(is_selector_token) {
            Some(Selector::parse(stream)?)
        } else {
            None
        };

        Ok(BSource { register, selector })
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
    if digits.len() != 2 {
        return Err(unexpected_value(
            span,
            &[".hxy where x and y are in 0..3"],
            format!(".{name}"),
        ));
    }

    let mut chars = digits.chars();
    let first_half = parse_half_char(chars.next().unwrap(), span.clone())?;
    let second_half = parse_half_char(chars.next().unwrap(), span.clone())?;

    Ok(Selector {
        halves: [first_half, second_half],
    })
}

fn parse_half_char(ch: char, span: Span) -> Result<Half, PtxParseError> {
    match ch {
        '0' => Ok(crate::r#type::instruction::vset2::Half::H0),
        '1' => Ok(crate::r#type::instruction::vset2::Half::H1),
        '2' => Ok(crate::r#type::instruction::vset2::Half::H2),
        '3' => Ok(crate::r#type::instruction::vset2::Half::H3),
        other => Err(unexpected_value(
            span,
            &["digits 0 through 3"],
            other.to_string(),
        )),
    }
}

fn is_mask_token(token: &PtxToken) -> bool {
    matches!(
        token,
        PtxToken::Directive(name) if matches!(name.as_str(), "h0" | "h1" | "h10")
    )
}

fn is_selector_token(token: &PtxToken) -> bool {
    matches!(token, PtxToken::Directive(name) if selector_name(name))
}

fn selector_name(name: &str) -> bool {
    name.starts_with('h')
        && name.len() == 3
        && name.as_bytes()[1].is_ascii_digit()
        && name.as_bytes()[2].is_ascii_digit()
}
