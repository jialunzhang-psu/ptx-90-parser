use crate::{
    lexer::PtxToken,
    parser::{
        PtxParseError, PtxParser, PtxTokenStream, Span, common::parse_u64_literal, invalid_literal,
        peek_directive, unexpected_value,
    },
    r#type::{
        common::{AddressSpace, AttributeDirective, DataLinkage, DataType},
        variable::{
            GlobalInitializer, InitializerValue, ModuleVariableDirective, NumericLiteral,
            VariableDirective, VariableModifier,
        },
    },
};

const DATA_TYPE_NAMES: &[&str] = &[
    "u8", "u16", "u32", "u64", "s8", "s16", "s32", "s64", "f16", "f16x2", "f32", "f64", "b8",
    "b16", "b32", "b64", "b128", "pred",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VariableDirectiveKind {
    Tex,
    Shared,
    Global,
    Const,
    Other,
}

struct ParsedVariableDirective {
    directive: VariableDirective,
    kind: VariableDirectiveKind,
    leading_span: Option<Span>,
}

fn consume_newlines(stream: &mut PtxTokenStream) {
    while stream
        .consume_if(|token| matches!(token, PtxToken::Newline))
        .is_some()
    {}
}

fn is_data_type_directive(name: &str) -> bool {
    DATA_TYPE_NAMES.iter().any(|candidate| candidate == &name)
}

fn is_vector_modifier(name: &str) -> bool {
    let mut chars = name.chars();
    match (chars.next(), chars.next()) {
        (Some('v'), Some(digit)) if digit.is_ascii_digit() => chars.all(|ch| ch.is_ascii_digit()),
        _ => false,
    }
}

fn parse_alignment_value(stream: &mut PtxTokenStream) -> Result<u32, PtxParseError> {
    consume_newlines(stream);
    let (value, value_span) = parse_u64_literal(stream)?;
    if value > u32::MAX as u64 {
        return Err(invalid_literal(
            value_span,
            "alignment value exceeds u32 range",
        ));
    }
    Ok(value as u32)
}

fn parse_numeric_string(text: &str, span: Span) -> Result<u128, PtxParseError> {
    text.parse::<u128>()
        .map_err(|_| invalid_literal(span, "invalid integer literal"))
}

impl PtxParser for NumericLiteral {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        consume_newlines(stream);

        let negative = stream
            .consume_if(|token| matches!(token, PtxToken::Minus))
            .is_some();
        let positive = stream
            .consume_if(|token| matches!(token, PtxToken::Plus))
            .is_some();

        if negative && positive {
            let (_, span) = stream.peek()?;
            return Err(invalid_literal(
                span.clone(),
                "cannot have both '+' and '-' signs",
            ));
        }

        let (token, span_ref) = stream.consume()?;
        let span = span_ref.clone();
        match token {
            PtxToken::DecimalInteger(text) => {
                let value = parse_numeric_string(text.as_str(), span.clone())?;
                if negative {
                    if value > (i64::MAX as u128) + 1 {
                        return Err(invalid_literal(span.clone(), "signed integer underflow"));
                    }
                    let signed = -(value as i128);
                    Ok(NumericLiteral::Signed(signed as i64))
                } else {
                    if value > u64::MAX as u128 {
                        return Err(invalid_literal(span.clone(), "unsigned integer overflow"));
                    }
                    Ok(NumericLiteral::Unsigned(value as u64))
                }
            }
            PtxToken::HexInteger(text) => {
                let stripped = text
                    .strip_prefix("0x")
                    .or_else(|| text.strip_prefix("0X"))
                    .unwrap_or(text.as_str());
                let value = u128::from_str_radix(stripped, 16)
                    .map_err(|_| invalid_literal(span.clone(), "invalid hex literal"))?;
                if negative {
                    if value > (i64::MAX as u128) + 1 {
                        return Err(invalid_literal(span.clone(), "signed integer underflow"));
                    }
                    let signed = -(value as i128);
                    Ok(NumericLiteral::Signed(signed as i64))
                } else {
                    if value > u64::MAX as u128 {
                        return Err(invalid_literal(span.clone(), "unsigned integer overflow"));
                    }
                    Ok(NumericLiteral::Unsigned(value as u64))
                }
            }
            PtxToken::BinaryInteger(text) => {
                let stripped = text
                    .strip_prefix("0b")
                    .or_else(|| text.strip_prefix("0B"))
                    .unwrap_or(text.as_str());
                let value = u128::from_str_radix(stripped, 2)
                    .map_err(|_| invalid_literal(span.clone(), "invalid binary literal"))?;
                if negative {
                    if value > (i64::MAX as u128) + 1 {
                        return Err(invalid_literal(span.clone(), "signed integer underflow"));
                    }
                    let signed = -(value as i128);
                    Ok(NumericLiteral::Signed(signed as i64))
                } else {
                    if value > u64::MAX as u128 {
                        return Err(invalid_literal(span.clone(), "unsigned integer overflow"));
                    }
                    Ok(NumericLiteral::Unsigned(value as u64))
                }
            }
            PtxToken::OctalInteger(text) => {
                let stripped = &text.as_str()[1..];
                let value = u128::from_str_radix(stripped, 8)
                    .map_err(|_| invalid_literal(span.clone(), "invalid octal literal"))?;
                if negative {
                    if value > (i64::MAX as u128) + 1 {
                        return Err(invalid_literal(span.clone(), "signed integer underflow"));
                    }
                    let signed = -(value as i128);
                    Ok(NumericLiteral::Signed(signed as i64))
                } else {
                    if value > u64::MAX as u128 {
                        return Err(invalid_literal(span.clone(), "unsigned integer overflow"));
                    }
                    Ok(NumericLiteral::Unsigned(value as u64))
                }
            }
            PtxToken::Float(text) | PtxToken::FloatExponent(text) => {
                let mut value = text
                    .parse::<f64>()
                    .map_err(|_| invalid_literal(span.clone(), "invalid floating-point literal"))?;
                if negative {
                    value = -value;
                }
                Ok(NumericLiteral::Float64(value.to_bits()))
            }
            PtxToken::HexFloat(text) => {
                if text.len() < 3 {
                    return Err(invalid_literal(
                        span.clone(),
                        "invalid hexadecimal float literal",
                    ));
                }
                let (prefix, digits) = text.split_at(2);
                match prefix.to_ascii_lowercase().as_str() {
                    "0f" => {
                        let mut bits = u32::from_str_radix(digits, 16)
                            .map_err(|_| invalid_literal(span.clone(), "invalid float literal"))?;
                        if negative {
                            bits ^= 0x8000_0000;
                        }
                        Ok(NumericLiteral::Float32(bits))
                    }
                    "0d" => {
                        let mut bits = u64::from_str_radix(digits, 16)
                            .map_err(|_| invalid_literal(span.clone(), "invalid float literal"))?;
                        if negative {
                            bits ^= 0x8000_0000_0000_0000;
                        }
                        Ok(NumericLiteral::Float64(bits))
                    }
                    _ => Err(invalid_literal(
                        span.clone(),
                        "hexadecimal float must start with 0f or 0d",
                    )),
                }
            }
            _ => Err(unexpected_value(
                span.clone(),
                &["numeric literal"],
                format!("{token:?}"),
            )),
        }
    }
}

impl PtxParser for InitializerValue {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        consume_newlines(stream);
        if let Some((token, span)) = stream.peek().ok() {
            match token {
                PtxToken::StringLiteral(value) => {
                    let value = value.clone();
                    stream.consume()?;
                    return Ok(InitializerValue::StringLiteral(value));
                }
                PtxToken::Identifier(_) => {
                    let (symbol, _) = stream.expect_identifier()?;
                    return Ok(InitializerValue::Symbol(symbol));
                }
                PtxToken::Plus | PtxToken::Minus => {
                    let literal = NumericLiteral::parse(stream)?;
                    return Ok(InitializerValue::Numeric(literal));
                }
                PtxToken::DecimalInteger(_)
                | PtxToken::HexInteger(_)
                | PtxToken::BinaryInteger(_)
                | PtxToken::OctalInteger(_)
                | PtxToken::Float(_)
                | PtxToken::FloatExponent(_)
                | PtxToken::HexFloat(_) => {
                    let literal = NumericLiteral::parse(stream)?;
                    return Ok(InitializerValue::Numeric(literal));
                }
                _ => {
                    return Err(unexpected_value(
                        span.clone(),
                        &["numeric literal", "symbol", "string literal"],
                        format!("{token:?}"),
                    ));
                }
            }
        }
        let span = stream.peek()?.1.clone();
        Err(unexpected_value(
            span,
            &["numeric literal", "symbol", "string literal"],
            "end of input".to_string(),
        ))
    }
}

impl PtxParser for GlobalInitializer {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        consume_newlines(stream);
        if stream
            .consume_if(|token| matches!(token, PtxToken::LBrace))
            .is_some()
        {
            let mut children = Vec::new();
            consume_newlines(stream);
            if !stream.check(|token| matches!(token, PtxToken::RBrace)) {
                loop {
                    let initializer = GlobalInitializer::parse(stream)?;
                    children.push(initializer);
                    consume_newlines(stream);
                    if stream
                        .consume_if(|token| matches!(token, PtxToken::Comma))
                        .is_some()
                    {
                        consume_newlines(stream);
                        continue;
                    }
                    break;
                }
            }
            stream.expect(&PtxToken::RBrace)?;
            Ok(GlobalInitializer::Aggregate(children))
        } else {
            let value = InitializerValue::parse(stream)?;
            Ok(GlobalInitializer::Scalar(value))
        }
    }
}

impl PtxParser for VariableModifier {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span_ref) = stream.expect_directive()?;
        let span = span_ref.clone();
        match directive.as_str() {
            "align" => {
                let value = parse_alignment_value(stream)?;
                Ok(VariableModifier::Alignment(value))
            }
            "ptr" => Ok(VariableModifier::Ptr),
            "visible" => Ok(VariableModifier::Linkage(DataLinkage::Visible)),
            "extern" => Ok(VariableModifier::Linkage(DataLinkage::Extern)),
            "weak" => Ok(VariableModifier::Linkage(DataLinkage::Weak)),
            "common" => Ok(VariableModifier::Linkage(DataLinkage::Common)),
            other if is_vector_modifier(other) => {
                let digits = &other[1..];
                let value = digits
                    .parse::<u32>()
                    .map_err(|_| invalid_literal(span.clone(), "invalid vector width"))?;
                Ok(VariableModifier::Vector(value))
            }
            other => Err(unexpected_value(
                span.clone(),
                &[
                    ".align", ".ptr", ".visible", ".extern", ".weak", ".common", ".vN",
                ],
                format!(".{other}"),
            )),
        }
    }
}

fn parse_variable_directive_internal(
    stream: &mut PtxTokenStream,
) -> Result<ParsedVariableDirective, PtxParseError> {
    let first_span = stream.peek().ok().map(|(_, span)| span.clone());

    let mut address_space: Option<AddressSpace> = None;
    let mut attributes = Vec::new();
    let mut modifiers = Vec::new();
    let mut ty: Option<DataType> = None;
    let mut array = Vec::new();
    let mut initializer = None;
    let mut seen_tex = false;
    let mut kind = VariableDirectiveKind::Other;
    let mut kind_span = None;

    loop {
        consume_newlines(stream);
        let Some((directive, directive_span)) = peek_directive(stream)? else {
            break;
        };
        match directive.as_str() {
            "tex" => {
                stream.expect_directive()?;
                if !seen_tex {
                    seen_tex = true;
                    kind = VariableDirectiveKind::Tex;
                    kind_span = Some(directive_span);
                }
            }
            "global" | "const" | "shared" | "local" | "param" | "reg" => {
                if address_space.is_some() {
                    return Err(unexpected_value(
                        directive_span.clone(),
                        &["single address space qualifier"],
                        format!(".{directive}"),
                    ));
                }
                let space = AddressSpace::parse(stream)?;
                address_space = Some(space);
                match space {
                    AddressSpace::Global => {
                        kind = VariableDirectiveKind::Global;
                        kind_span = Some(directive_span);
                    }
                    AddressSpace::Const => {
                        kind = VariableDirectiveKind::Const;
                        kind_span = Some(directive_span);
                    }
                    AddressSpace::Shared => {
                        kind = VariableDirectiveKind::Shared;
                        kind_span = Some(directive_span);
                    }
                    _ => {}
                }
            }
            "managed" | "unified" => {
                attributes.push(AttributeDirective::parse(stream)?);
            }
            "align" | "ptr" | "visible" | "extern" | "weak" | "common" => {
                modifiers.push(VariableModifier::parse(stream)?);
            }
            other if is_vector_modifier(other) => {
                modifiers.push(VariableModifier::parse(stream)?);
            }
            other if is_data_type_directive(other) => {
                if ty.is_some() {
                    return Err(unexpected_value(
                        directive_span.clone(),
                        &["single data type qualifier"],
                        format!(".{other}"),
                    ));
                }
                ty = Some(DataType::parse(stream)?);
            }
            _ => break,
        }
    }

    consume_newlines(stream);
    let (name, _) = stream.expect_identifier()?;

    loop {
        consume_newlines(stream);
        if stream
            .consume_if(|token| matches!(token, PtxToken::LBracket))
            .is_none()
        {
            break;
        }

        consume_newlines(stream);

        if stream
            .consume_if(|token| matches!(token, PtxToken::RBracket))
            .is_some()
        {
            array.push(None);
            continue;
        }

        let size_span = stream.peek()?.1.clone();
        let literal = NumericLiteral::parse(stream)?;
        let size = match literal {
            NumericLiteral::Unsigned(value) => value,
            NumericLiteral::Signed(value) if value >= 0 => value as u64,
            _ => {
                return Err(invalid_literal(
                    size_span.clone(),
                    "array size must be a non-negative integer",
                ));
            }
        };

        consume_newlines(stream);
        stream.expect(&PtxToken::RBracket)?;
        array.push(Some(size));
    }

    consume_newlines(stream);
    if stream
        .consume_if(|token| matches!(token, PtxToken::Equals))
        .is_some()
    {
        consume_newlines(stream);
        initializer = Some(GlobalInitializer::parse(stream)?);
    }

    consume_newlines(stream);
    stream.expect(&PtxToken::Semicolon)?;

    let mut final_kind = kind;
    if seen_tex {
        final_kind = VariableDirectiveKind::Tex;
    } else if matches!(final_kind, VariableDirectiveKind::Other) {
        final_kind = match address_space {
            Some(AddressSpace::Shared) => VariableDirectiveKind::Shared,
            Some(AddressSpace::Global) => VariableDirectiveKind::Global,
            Some(AddressSpace::Const) => VariableDirectiveKind::Const,
            _ => VariableDirectiveKind::Other,
        };
    }

    let directive = VariableDirective {
        address_space,
        attributes,
        ty,
        modifiers,
        name,
        array,
        initializer,
        raw: String::new(),
    };

    Ok(ParsedVariableDirective {
        directive,
        kind: final_kind,
        leading_span: kind_span.or(first_span),
    })
}

impl VariableDirective {
    fn parse_with_kind(
        stream: &mut PtxTokenStream,
    ) -> Result<(VariableDirective, VariableDirectiveKind, Option<Span>), PtxParseError> {
        let parsed = parse_variable_directive_internal(stream)?;
        Ok((parsed.directive, parsed.kind, parsed.leading_span))
    }
}

impl PtxParser for VariableDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let parsed = parse_variable_directive_internal(stream)?;
        Ok(parsed.directive)
    }
}

impl PtxParser for ModuleVariableDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, kind, span) = VariableDirective::parse_with_kind(stream)?;
        match kind {
            VariableDirectiveKind::Tex => Ok(ModuleVariableDirective::Tex(directive)),
            VariableDirectiveKind::Shared => Ok(ModuleVariableDirective::Shared(directive)),
            VariableDirectiveKind::Global => Ok(ModuleVariableDirective::Global(directive)),
            VariableDirectiveKind::Const => Ok(ModuleVariableDirective::Const(directive)),
            VariableDirectiveKind::Other => Err(unexpected_value(
                span.unwrap_or(0..0),
                &[".tex", ".shared", ".global", ".const"],
                "variable directive".to_string(),
            )),
        }
    }
}
