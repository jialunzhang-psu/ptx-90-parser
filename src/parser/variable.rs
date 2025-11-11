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
                    Ok(NumericLiteral::Signed { value: signed as i64, span })
                } else {
                    if value > u64::MAX as u128 {
                        return Err(invalid_literal(span.clone(), "unsigned integer overflow"));
                    }
                    Ok(NumericLiteral::Unsigned { value: value as u64, span })
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
                    Ok(NumericLiteral::Signed { value: signed as i64, span })
                } else {
                    if value > u64::MAX as u128 {
                        return Err(invalid_literal(span.clone(), "unsigned integer overflow"));
                    }
                    Ok(NumericLiteral::Unsigned { value: value as u64, span })
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
                    Ok(NumericLiteral::Signed { value: signed as i64, span })
                } else {
                    if value > u64::MAX as u128 {
                        return Err(invalid_literal(span.clone(), "unsigned integer overflow"));
                    }
                    Ok(NumericLiteral::Unsigned { value: value as u64, span })
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
                    Ok(NumericLiteral::Signed { value: signed as i64, span })
                } else {
                    if value > u64::MAX as u128 {
                        return Err(invalid_literal(span.clone(), "unsigned integer overflow"));
                    }
                    Ok(NumericLiteral::Unsigned { value: value as u64, span })
                }
            }
            PtxToken::Float(text) | PtxToken::FloatExponent(text) => {
                let mut value = text
                    .parse::<f64>()
                    .map_err(|_| invalid_literal(span.clone(), "invalid floating-point literal"))?;
                if negative {
                    value = -value;
                }
                Ok(NumericLiteral::Float64 { value: value.to_bits(), span })
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
                        Ok(NumericLiteral::Float32 { value: bits, span })
                    }
                    "0d" => {
                        let mut bits = u64::from_str_radix(digits, 16)
                            .map_err(|_| invalid_literal(span.clone(), "invalid float literal"))?;
                        if negative {
                            bits ^= 0x8000_0000_0000_0000;
                        }
                        Ok(NumericLiteral::Float64 { value: bits, span })
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
        if let Some((token, span)) = stream.peek().ok() {
            let span = span.clone();
            match token {
                PtxToken::StringLiteral(value) => {
                    let value = value.clone();
                    stream.consume()?;
                    return Ok(InitializerValue::StringLiteral { value, span });
                }
                PtxToken::Identifier(_) => {
                    let (name, span) = stream.expect_identifier()?;
                    return Ok(InitializerValue::Symbol { name, span: span.clone() });
                }
                PtxToken::Plus | PtxToken::Minus => {
                    let literal = NumericLiteral::parse(stream)?;
                    let span = literal.span();
                    return Ok(InitializerValue::Numeric { value: literal, span });
                }
                PtxToken::DecimalInteger(_)
                | PtxToken::HexInteger(_)
                | PtxToken::BinaryInteger(_)
                | PtxToken::OctalInteger(_)
                | PtxToken::Float(_)
                | PtxToken::FloatExponent(_)
                | PtxToken::HexFloat(_) => {
                    let literal = NumericLiteral::parse(stream)?;
                    let span = literal.span();
                    return Ok(InitializerValue::Numeric { value: literal, span });
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
        let start_span = stream.peek()?.1.clone();
        if stream
            .consume_if(|token| matches!(token, PtxToken::LBrace))
            .is_some()
        {
            let mut children = Vec::new();
            if !stream.check(|token| matches!(token, PtxToken::RBrace)) {
                loop {
                    let initializer = GlobalInitializer::parse(stream)?;
                    children.push(initializer);
                    if !(stream
                        .consume_if(|token| matches!(token, PtxToken::Comma))
                        .is_some())
                    {
                        break;
                    }
                }
            }
            let (_, end_span) = stream.expect(&PtxToken::RBrace)?;
            let span = start_span.start..end_span.end;
            Ok(GlobalInitializer::Aggregate { values: children, span })
        } else {
            let value = InitializerValue::parse(stream)?;
            let span = value.span();
            Ok(GlobalInitializer::Scalar { value, span })
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
                Ok(VariableModifier::Alignment { value, span })
            }
            "ptr" => Ok(VariableModifier::Ptr { span }),
            "visible" => Ok(VariableModifier::Linkage {
                linkage: DataLinkage::Visible { span: span.clone() },
                span
            }),
            "extern" => Ok(VariableModifier::Linkage {
                linkage: DataLinkage::Extern { span: span.clone() },
                span
            }),
            "weak" => Ok(VariableModifier::Linkage {
                linkage: DataLinkage::Weak { span: span.clone() },
                span
            }),
            "common" => Ok(VariableModifier::Linkage {
                linkage: DataLinkage::Common { span: span.clone() },
                span
            }),
            other if is_vector_modifier(other) => {
                let digits = &other[1..];
                let value = digits
                    .parse::<u32>()
                    .map_err(|_| invalid_literal(span.clone(), "invalid vector width"))?;
                Ok(VariableModifier::Vector { value, span })
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

impl VariableDirective {
    fn parse_with_kind(
        stream: &mut PtxTokenStream,
    ) -> Result<(VariableDirective, VariableDirectiveKind, Option<Span>), PtxParseError> {
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
                    match space {
                        AddressSpace::Global { .. } => {
                            kind = VariableDirectiveKind::Global;
                            kind_span = Some(directive_span.clone());
                        }
                        AddressSpace::Const { .. } => {
                            kind = VariableDirectiveKind::Const;
                            kind_span = Some(directive_span.clone());
                        }
                        AddressSpace::Shared { .. } => {
                            kind = VariableDirectiveKind::Shared;
                            kind_span = Some(directive_span.clone());
                        }
                        _ => {}
                    }
                    address_space = Some(space);
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

        let (name, _) = stream.expect_identifier()?;

        loop {
            if stream
                .consume_if(|token| matches!(token, PtxToken::LBracket))
                .is_none()
            {
                break;
            }

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
                NumericLiteral::Unsigned { value, .. } => value,
                NumericLiteral::Signed { value, .. } if value >= 0 => value as u64,
                _ => {
                    return Err(invalid_literal(
                        size_span.clone(),
                        "array size must be a non-negative integer",
                    ));
                }
            };

            stream.expect(&PtxToken::RBracket)?;
            array.push(Some(size));
        }

        if stream
            .consume_if(|token| matches!(token, PtxToken::Equals))
            .is_some()
        {
            initializer = Some(GlobalInitializer::parse(stream)?);
        }

        stream.expect(&PtxToken::Semicolon)?;

        let mut final_kind = kind;
        if seen_tex {
            final_kind = VariableDirectiveKind::Tex;
        } else if matches!(final_kind, VariableDirectiveKind::Other) {
            final_kind = match address_space {
                Some(AddressSpace::Shared { .. }) => VariableDirectiveKind::Shared,
                Some(AddressSpace::Global { .. }) => VariableDirectiveKind::Global,
                Some(AddressSpace::Const { .. }) => VariableDirectiveKind::Const,
                _ => VariableDirectiveKind::Other,
            };
        }

        let end_span = stream.peek().ok().map(|(_, s)| s.clone()).unwrap_or(0..0);
        let span = first_span.map(|s| s.start..end_span.end).unwrap_or(0..0);

        let directive = VariableDirective {
            address_space,
            attributes,
            ty,
            modifiers,
            name,
            array,
            initializer,
            span: span.clone(),
        };

        Ok((directive, final_kind, kind_span.or(Some(span))))
    }
}

impl PtxParser for VariableDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, _, _) = VariableDirective::parse_with_kind(stream)?;
        Ok(directive)
    }
}

impl PtxParser for ModuleVariableDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, kind, span_opt) = VariableDirective::parse_with_kind(stream)?;
        let span = span_opt.unwrap_or(0..0);
        match kind {
            VariableDirectiveKind::Tex => Ok(ModuleVariableDirective::Tex {
                directive,
                span
            }),
            VariableDirectiveKind::Shared => Ok(ModuleVariableDirective::Shared {
                directive,
                span
            }),
            VariableDirectiveKind::Global => Ok(ModuleVariableDirective::Global {
                directive,
                span
            }),
            VariableDirectiveKind::Const => Ok(ModuleVariableDirective::Const {
                directive,
                span: span.clone()
            }),
            VariableDirectiveKind::Other => Err(unexpected_value(
                span,
                &[".tex", ".shared", ".global", ".const"],
                "variable directive".to_string(),
            )),
        }
    }
}
