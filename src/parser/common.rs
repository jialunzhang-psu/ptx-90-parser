use std::borrow::Cow;

use crate::{
    lexer::PtxToken,
    parser::{ParseErrorKind, PtxParseError, PtxParser, PtxTokenStream, Span},
    r#type::common::*,
    r#type::instruction::Inst,
};

pub(crate) fn unexpected_value(
    span: Span,
    expected: &[&str],
    found: impl Into<Cow<'static, str>>,
) -> PtxParseError {
    PtxParseError {
        kind: ParseErrorKind::UnexpectedToken {
            expected: expected.iter().map(|s| s.to_string()).collect(),
            found: found.into().to_string(),
        },
        span,
    }
}

pub(crate) fn invalid_literal(span: Span, literal: impl Into<Cow<'static, str>>) -> PtxParseError {
    PtxParseError {
        kind: ParseErrorKind::InvalidLiteral(literal.into().to_string()),
        span,
    }
}

pub(crate) fn parse_register_name(
    stream: &mut PtxTokenStream,
) -> Result<(String, Span), PtxParseError> {
    let (mut name, mut span) = stream.expect_register()?;

    loop {
        // Peek to decide whether the next token should be treated as a component.
        let next = match stream.peek() {
            Ok((token, _)) => token,
            Err(_) => break,
        };

        match next {
            PtxToken::Dot => {
                // Peek ahead to see if this is a valid register component
                if let Some((PtxToken::Identifier(component_name), _)) =
                    stream.tokens.get(stream.index + 1)
                {
                    // Only consume if it's a valid single-character register component
                    // Exclude multi-character .b* patterns (e.g., .b0, .b3210) which are instruction-specific modifiers
                    if matches!(
                        component_name.as_str(),
                        "x" | "y" | "z" | "w" | "r" | "g" | "b" | "a"
                    ) {
                        // consume the dot and identifier
                        stream.consume()?;
                        let (component, component_span) = stream.expect_identifier()?;

                        name.push('.');
                        name.push_str(&component);

                        span.end = component_span.end;
                    } else {
                        // Not a valid register component, stop parsing
                        break;
                    }
                } else {
                    break;
                }
            }
            _ => break,
        }
    }

    Ok((name, span))
}

pub(crate) fn numeric_literal(token: &PtxToken) -> Option<&String> {
    match token {
        PtxToken::DecimalInteger(value)
        | PtxToken::HexInteger(value)
        | PtxToken::BinaryInteger(value)
        | PtxToken::OctalInteger(value)
        | PtxToken::FloatExponent(value)
        | PtxToken::Float(value)
        | PtxToken::HexFloat(value) => Some(value),
        _ => None,
    }
}

pub(crate) fn is_numeric_token(token: &PtxToken) -> bool {
    numeric_literal(token).is_some()
}

pub(crate) fn parse_u64_literal(stream: &mut PtxTokenStream) -> Result<(u64, Span), PtxParseError> {
    let (token, span) = stream.consume()?;
    let span = span.clone();

    let value = match token {
        PtxToken::DecimalInteger(text) => text
            .parse::<u64>()
            .map_err(|_| invalid_literal(span.clone(), text.clone()))?,
        PtxToken::HexInteger(text) => {
            let stripped = text
                .strip_prefix("0x")
                .or_else(|| text.strip_prefix("0X"))
                .ok_or_else(|| invalid_literal(span.clone(), text.clone()))?;
            u64::from_str_radix(stripped, 16)
                .map_err(|_| invalid_literal(span.clone(), text.clone()))?
        }
        PtxToken::BinaryInteger(text) => {
            let stripped = text
                .strip_prefix("0b")
                .or_else(|| text.strip_prefix("0B"))
                .ok_or_else(|| invalid_literal(span.clone(), text.clone()))?;
            u64::from_str_radix(stripped, 2)
                .map_err(|_| invalid_literal(span.clone(), text.clone()))?
        }
        PtxToken::OctalInteger(text) => {
            let stripped = &text[1..];
            u64::from_str_radix(stripped, 8)
                .map_err(|_| invalid_literal(span.clone(), text.clone()))?
        }
        _ => {
            return Err(unexpected_value(
                span,
                &["unsigned integer literal"],
                format!("{token:?}"),
            ));
        }
    };

    Ok((value, span))
}

impl PtxParser for CodeLinkage {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "visible" => Ok(CodeLinkage::Visible { span }),
            "extern" => Ok(CodeLinkage::Extern { span }),
            "weak" => Ok(CodeLinkage::Weak { span }),
            other => Err(unexpected_value(
                span,
                &[".visible", ".extern", ".weak"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for DataLinkage {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "visible" => Ok(DataLinkage::Visible { span }),
            "extern" => Ok(DataLinkage::Extern { span }),
            "weak" => Ok(DataLinkage::Weak { span }),
            "common" => Ok(DataLinkage::Common { span }),
            other => Err(unexpected_value(
                span,
                &[".visible", ".extern", ".weak", ".common"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for CodeOrDataLinkage {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "visible" => Ok(CodeOrDataLinkage::Visible { span }),
            "extern" => Ok(CodeOrDataLinkage::Extern { span }),
            "weak" => Ok(CodeOrDataLinkage::Weak { span }),
            "common" => Ok(CodeOrDataLinkage::Common { span }),
            other => Err(unexpected_value(
                span,
                &[".visible", ".extern", ".weak", ".common"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for TexType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "texref" => Ok(TexType::TexRef { span }),
            "samplerref" => Ok(TexType::SamplerRef { span }),
            "surfref" => Ok(TexType::SurfRef { span }),
            other => Err(unexpected_value(
                span,
                &[".texref", ".samplerref", ".surfref"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for AddressSpace {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "global" => Ok(AddressSpace::Global { span }),
            "const" => Ok(AddressSpace::Const { span }),
            "shared" => Ok(AddressSpace::Shared { span }),
            "local" => Ok(AddressSpace::Local { span }),
            "param" => Ok(AddressSpace::Param { span }),
            "reg" => Ok(AddressSpace::Reg { span }),
            other => Err(unexpected_value(
                span,
                &[".global", ".const", ".shared", ".local", ".param", ".reg"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for AttributeDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "unified" => {
                stream.expect(&PtxToken::LParen)?;
                let (uuid1, _) = parse_u64_literal(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let (uuid2, _) = parse_u64_literal(stream)?;
                stream.expect(&PtxToken::RParen)?;
                Ok(AttributeDirective::Unified { uuid1, uuid2, span })
            }
            "managed" => Ok(AttributeDirective::Managed { span }),
            other => Err(unexpected_value(
                span,
                &[".unified", ".managed"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "u8" => Ok(DataType::U8 { span }),
            "u16" => Ok(DataType::U16 { span }),
            "u32" => Ok(DataType::U32 { span }),
            "u64" => Ok(DataType::U64 { span }),
            "s8" => Ok(DataType::S8 { span }),
            "s16" => Ok(DataType::S16 { span }),
            "s32" => Ok(DataType::S32 { span }),
            "s64" => Ok(DataType::S64 { span }),
            "f16" => Ok(DataType::F16 { span }),
            "f16x2" => Ok(DataType::F16x2 { span }),
            "f32" => Ok(DataType::F32 { span }),
            "f64" => Ok(DataType::F64 { span }),
            "b8" => Ok(DataType::B8 { span }),
            "b16" => Ok(DataType::B16 { span }),
            "b32" => Ok(DataType::B32 { span }),
            "b64" => Ok(DataType::B64 { span }),
            "b128" => Ok(DataType::B128 { span }),
            "pred" => Ok(DataType::Pred { span }),
            other => Err(unexpected_value(
                span,
                &[
                    ".u8", ".u16", ".u32", ".u64", ".s8", ".s16", ".s32", ".s64", ".f16", ".f16x2",
                    ".f32", ".f64", ".b8", ".b16", ".b32", ".b64", ".b128", ".pred",
                ],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Sign {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if let Some((_, span)) = stream
            .consume_if(|token| matches!(token, PtxToken::Plus))
        {
            return Ok(Sign::Positive { span: span.clone() });
        }
        if let Some((_, span)) = stream
            .consume_if(|token| matches!(token, PtxToken::Minus))
        {
            return Ok(Sign::Negative { span: span.clone() });
        }

        let (token, span) = stream.peek()?;
        Err(unexpected_value(
            span.clone(),
            &["+", "-"],
            format!("{token:?}"),
        ))
    }
}

impl PtxParser for Immediate {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        // Check for optional minus sign
        let minus_span = stream
            .consume_if(|token| matches!(token, PtxToken::Minus))
            .map(|(_, span)| span.clone());

        let (token, span) = stream.peek()?;
        let value = numeric_literal(token).cloned();
        match value {
            Some(value) => {
                let literal = if minus_span.is_some() {
                    format!("-{}", value)
                } else {
                    value.clone()
                };
                let (_, value_span) = stream.consume()?;
                let full_span = if let Some(ref ms) = minus_span {
                    Span { start: ms.start, end: value_span.end }
                } else {
                    value_span.clone()
                };
                Ok(Immediate { value: literal, span: full_span })
            }
            None => {
                // If we consumed a minus, we need to restore position by going back one token
                if minus_span.is_some() {
                    let mut current_pos = stream.position();
                    if current_pos.index > 0 {
                        current_pos.index -= 1;
                        current_pos.char_offset = 0;
                        stream.set_position(current_pos);
                    }
                }
                Err(unexpected_value(
                    span.clone(),
                    &["numeric literal"],
                    format!("{token:?}"),
                ))
            }
        }
    }
}

impl PtxParser for RegisterOperand {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if !stream.check(|token| matches!(token, PtxToken::Register(_))) {
            let (token, span) = stream.peek()?;
            return Err(unexpected_value(
                span.clone(),
                &["register"],
                format!("{token:?}"),
            ));
        }
        let (name, span) = parse_register_name(stream)?;
        Ok(RegisterOperand { name, span })
    }
}

impl PtxParser for PredicateRegister {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (name, span) = parse_register_name(stream)?;
        if name.starts_with("%p") {
            Ok(PredicateRegister { name, span })
        } else {
            Err(invalid_literal(
                span,
                format!("expected predicate register starting with %p, found {name}"),
            ))
        }
    }
}

impl PtxParser for Label {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (name, span) = stream.expect_identifier()?;
        Ok(Label { name, span })
    }
}

impl PtxParser for SpecialRegister {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (name, span) = parse_register_name(stream)?;
        // Preserve component information (.x/.y/.z) for certain special registers.
        // If a component is present, return the axis-aware variant; otherwise fall through
        // to the general match below.
        let name_str = name.as_str();
        if let Some(rest) = name_str.strip_prefix("%cluster_ctaid") {
            if rest.is_empty() {
                return Ok(SpecialRegister::ClusterCtaid { axis: Axis::None { span: span.clone() }, span });
            } else if rest == ".x" {
                return Ok(SpecialRegister::ClusterCtaid { axis: Axis::X { span: span.clone() }, span });
            } else if rest == ".y" {
                return Ok(SpecialRegister::ClusterCtaid { axis: Axis::Y { span: span.clone() }, span });
            } else if rest == ".z" {
                return Ok(SpecialRegister::ClusterCtaid { axis: Axis::Z { span: span.clone() }, span });
            }
        }
        if let Some(rest) = name_str.strip_prefix("%cluster_ctarank") {
            if rest.is_empty() {
                return Ok(SpecialRegister::ClusterCtarank { axis: Axis::None { span: span.clone() }, span });
            } else if rest == ".x" {
                return Ok(SpecialRegister::ClusterCtarank { axis: Axis::X { span: span.clone() }, span });
            } else if rest == ".y" {
                return Ok(SpecialRegister::ClusterCtarank { axis: Axis::Y { span: span.clone() }, span });
            } else if rest == ".z" {
                return Ok(SpecialRegister::ClusterCtarank { axis: Axis::Z { span: span.clone() }, span });
            }
        }
        if let Some(rest) = name_str.strip_prefix("%nctaid") {
            if rest.is_empty() {
                return Ok(SpecialRegister::Nctaid { axis: Axis::None { span: span.clone() }, span });
            } else if rest == ".x" {
                return Ok(SpecialRegister::Nctaid { axis: Axis::X { span: span.clone() }, span });
            } else if rest == ".y" {
                return Ok(SpecialRegister::Nctaid { axis: Axis::Y { span: span.clone() }, span });
            } else if rest == ".z" {
                return Ok(SpecialRegister::Nctaid { axis: Axis::Z { span: span.clone() }, span });
            }
        }
        if let Some(rest) = name_str.strip_prefix("%tid") {
            if rest.is_empty() {
                return Ok(SpecialRegister::Tid { axis: Axis::None { span: span.clone() }, span });
            } else if rest == ".x" {
                return Ok(SpecialRegister::Tid { axis: Axis::X { span: span.clone() }, span });
            } else if rest == ".y" {
                return Ok(SpecialRegister::Tid { axis: Axis::Y { span: span.clone() }, span });
            } else if rest == ".z" {
                return Ok(SpecialRegister::Tid { axis: Axis::Z { span: span.clone() }, span });
            }
        }
        if let Some(rest) = name_str.strip_prefix("%cluster_nctaid") {
            if rest.is_empty() {
                return Ok(SpecialRegister::ClusterNctaid { axis: Axis::None { span: span.clone() }, span });
            } else if rest == ".x" {
                return Ok(SpecialRegister::ClusterNctaid { axis: Axis::X { span: span.clone() }, span });
            } else if rest == ".y" {
                return Ok(SpecialRegister::ClusterNctaid { axis: Axis::Y { span: span.clone() }, span });
            } else if rest == ".z" {
                return Ok(SpecialRegister::ClusterNctaid { axis: Axis::Z { span: span.clone() }, span });
            }
        }
        if let Some(rest) = name_str.strip_prefix("%cluster_nctarank") {
            if rest.is_empty() {
                return Ok(SpecialRegister::ClusterNctarank { axis: Axis::None { span: span.clone() }, span });
            } else if rest == ".x" {
                return Ok(SpecialRegister::ClusterNctarank { axis: Axis::X { span: span.clone() }, span });
            } else if rest == ".y" {
                return Ok(SpecialRegister::ClusterNctarank { axis: Axis::Y { span: span.clone() }, span });
            } else if rest == ".z" {
                return Ok(SpecialRegister::ClusterNctarank { axis: Axis::Z { span: span.clone() }, span });
            }
        }
        if let Some(rest) = name_str.strip_prefix("%ntid") {
            if rest.is_empty() {
                return Ok(SpecialRegister::Ntid { axis: Axis::None { span: span.clone() }, span });
            } else if rest == ".x" {
                return Ok(SpecialRegister::Ntid { axis: Axis::X { span: span.clone() }, span });
            } else if rest == ".y" {
                return Ok(SpecialRegister::Ntid { axis: Axis::Y { span: span.clone() }, span });
            } else if rest == ".z" {
                return Ok(SpecialRegister::Ntid { axis: Axis::Z { span: span.clone() }, span });
            }
        }
        if let Some(rest) = name_str.strip_prefix("%ctaid") {
            if rest.is_empty() {
                return Ok(SpecialRegister::Ctaid { axis: Axis::None { span: span.clone() }, span });
            } else if rest == ".x" {
                return Ok(SpecialRegister::Ctaid { axis: Axis::X { span: span.clone() }, span });
            } else if rest == ".y" {
                return Ok(SpecialRegister::Ctaid { axis: Axis::Y { span: span.clone() }, span });
            } else if rest == ".z" {
                return Ok(SpecialRegister::Ctaid { axis: Axis::Z { span: span.clone() }, span });
            }
        }

        match name.as_str() {
            "%aggr_smem_size" => Ok(SpecialRegister::AggrSmemSize { span }),
            "%dynamic_smem_size" => Ok(SpecialRegister::DynamicSmemSize { span }),
            "%lanemask_gt" => Ok(SpecialRegister::LanemaskGt { span }),
            "%reserved_smem_offset_begin" => Ok(SpecialRegister::ReservedSmemOffsetBegin { span }),
            "%clock" => Ok(SpecialRegister::Clock { span }),
            "%lanemask_le" => Ok(SpecialRegister::LanemaskLe { span }),
            "%reserved_smem_offset_cap" => Ok(SpecialRegister::ReservedSmemOffsetCap { span }),
            "%clock64" => Ok(SpecialRegister::Clock64 { span }),
            "%globaltimer" => Ok(SpecialRegister::Globaltimer { span }),
            "%lanemask_lt" => Ok(SpecialRegister::LanemaskLt { span }),
            "%reserved_smem_offset_end" => Ok(SpecialRegister::ReservedSmemOffsetEnd { span }),
            "%cluster_ctaid" | "%cluster_ctaid.x" | "%cluster_ctaid.y" | "%cluster_ctaid.z" => {
                Ok(SpecialRegister::ClusterCtaid { axis: Axis::None { span: span.clone() }, span })
            }
            "%globaltimer_hi" => Ok(SpecialRegister::GlobaltimerHi { span }),
            "%nclusterid" => Ok(SpecialRegister::Nclusterid { span }),
            "%smid" => Ok(SpecialRegister::Smid { span }),
            "%cluster_ctarank" | "%cluster_ctarank.x" | "%cluster_ctarank.y"
            | "%cluster_ctarank.z" => Ok(SpecialRegister::ClusterCtarank { axis: Axis::None { span: span.clone() }, span }),
            "%globaltimer_lo" => Ok(SpecialRegister::GlobaltimerLo { span }),
            "%nctaid" | "%nctaid.x" | "%nctaid.y" | "%nctaid.z" => {
                Ok(SpecialRegister::Nctaid { axis: Axis::None { span: span.clone() }, span })
            }
            "%tid" | "%tid.x" | "%tid.y" | "%tid.z" => Ok(SpecialRegister::Tid { axis: Axis::None { span: span.clone() }, span }),
            "%cluster_nctaid" | "%cluster_nctaid.x" | "%cluster_nctaid.y" | "%cluster_nctaid.z" => {
                Ok(SpecialRegister::ClusterNctaid { axis: Axis::None { span: span.clone() }, span })
            }
            "%gridid" => Ok(SpecialRegister::Gridid { span }),
            "%nsmid" => Ok(SpecialRegister::Nsmid { span }),
            "%total_smem_size" => Ok(SpecialRegister::TotalSmemSize { span }),
            "%cluster_nctarank"
            | "%cluster_nctarank.x"
            | "%cluster_nctarank.y"
            | "%cluster_nctarank.z" => Ok(SpecialRegister::ClusterNctarank { axis: Axis::None { span: span.clone() }, span }),
            "%is_explicit_cluster" => Ok(SpecialRegister::IsExplicitCluster { span }),
            "%ntid" | "%ntid.x" | "%ntid.y" | "%ntid.z" => Ok(SpecialRegister::Ntid { axis: Axis::None { span: span.clone() }, span }),
            "%warpid" => Ok(SpecialRegister::Warpid { span }),
            "%clusterid" => Ok(SpecialRegister::Clusterid { span }),
            "%laneid" => Ok(SpecialRegister::Laneid { span }),
            "%nwarpid" => Ok(SpecialRegister::Nwarpid { span }),
            "%WARPSZ" => Ok(SpecialRegister::WARPSZ { span }),
            "%ctaid" | "%ctaid.x" | "%ctaid.y" | "%ctaid.z" => {
                Ok(SpecialRegister::Ctaid { axis: Axis::None { span: span.clone() }, span })
            }
            "%lanemask_eq" => Ok(SpecialRegister::LanemaskEq { span }),
            "%current_graph_exec" => Ok(SpecialRegister::CurrentGraphExec { span }),
            "%lanemask_ge" => Ok(SpecialRegister::LanemaskGe { span }),
            other => {
                if let Some(num) = other.strip_prefix("%envreg") {
                    let value = num
                        .parse::<u8>()
                        .map_err(|_| invalid_literal(span.clone(), name.clone()))?;
                    if value <= 31 {
                        return Ok(SpecialRegister::Envreg { index: value, span });
                    }
                    return Err(invalid_literal(
                        span,
                        format!("envreg index out of range: {value}"),
                    ));
                }

                if let Some(num) = other.strip_prefix("%pm") {
                    if let Some(rest) = num.strip_suffix("_64") {
                        let value = rest
                            .parse::<u8>()
                            .map_err(|_| invalid_literal(span.clone(), name.clone()))?;
                        if value <= 7 {
                            return Ok(SpecialRegister::Pm64 { index: value, span });
                        }
                        return Err(invalid_literal(
                            span,
                            format!("pm index out of range: {value}"),
                        ));
                    }

                    let value = num
                        .parse::<u8>()
                        .map_err(|_| invalid_literal(span.clone(), name.clone()))?;
                    if value <= 7 {
                        return Ok(SpecialRegister::Pm { index: value, span });
                    }
                    return Err(invalid_literal(
                        span,
                        format!("pm index out of range: {value}"),
                    ));
                }

                if let Some(num) = other.strip_prefix("%reserved_smem_offset_") {
                    let value = num
                        .parse::<u8>()
                        .map_err(|_| invalid_literal(span.clone(), name.clone()))?;
                    if value <= 1 {
                        return Ok(SpecialRegister::ReservedSmemOffset { index: value, span });
                    }
                    return Err(invalid_literal(
                        span,
                        format!("reserved_smem_offset index out of range: {value}"),
                    ));
                }

                Err(invalid_literal(
                    span,
                    format!("unknown special register {name}"),
                ))
            }
        }
    }
}

impl PtxParser for Operand {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let saved_pos = stream.position();
        if let Ok(immediate) = Immediate::parse(stream) {
            let span = immediate.span.clone();
            return Ok(Operand::Immediate { operand: immediate, span });
        }
        stream.set_position(saved_pos);

        if stream.check(|token| matches!(token, PtxToken::Register(_))) {
            let register = RegisterOperand::parse(stream)?;
            let span = register.span.clone();
            return Ok(Operand::Register { operand: register, span });
        }

        if stream.check(|token| matches!(token, PtxToken::Identifier(_))) {
            let (identifier, ident_span) = stream.expect_identifier()?;

            // Check for arithmetic expression: identifier + immediate
            let saved_pos_after_ident = stream.position();
            if stream.expect(&PtxToken::Plus).is_ok() {
                if let Ok(offset) = Immediate::parse(stream) {
                    let span = Span { start: ident_span.start, end: offset.span.end };
                    return Ok(Operand::SymbolOffset { symbol: identifier, offset, span });
                }
                // If parsing offset failed, backtrack
                stream.set_position(saved_pos_after_ident);
            }

            return Ok(Operand::Symbol { name: identifier, span: ident_span });
        }

        let (token, span) = stream.peek()?;
        Err(unexpected_value(
            span.clone(),
            &["operand"],
            format!("{token:?}"),
        ))
    }
}

impl PtxParser for VectorOperand {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (_, brace_span) = stream.expect(&PtxToken::LBrace)?;
        let mut operands = Vec::new();

        loop {
            operands.push(Operand::parse(stream)?);
            if stream
                .consume_if(|token| matches!(token, PtxToken::Comma))
                .is_some()
            {
                continue;
            }
            break;
        }

        let (_, end_span) = stream.expect(&PtxToken::RBrace)?;
        let span = Span { start: brace_span.start, end: end_span.end };

        match operands.len() {
            1 => Ok(VectorOperand::Vector1 { operand: operands.remove(0), span }),
            2 => Ok(VectorOperand::Vector2 { operands: [
                operands.remove(0),
                operands.remove(0),
            ], span }),
            3 => Ok(VectorOperand::Vector3 { operands: [
                operands.remove(0),
                operands.remove(0),
                operands.remove(0),
            ], span }),
            4 => Ok(VectorOperand::Vector4 { operands: [
                operands.remove(0),
                operands.remove(0),
                operands.remove(0),
                operands.remove(0),
            ], span }),
            8 => Ok(VectorOperand::Vector8 { operands: [
                operands.remove(0),
                operands.remove(0),
                operands.remove(0),
                operands.remove(0),
                operands.remove(0),
                operands.remove(0),
                operands.remove(0),
                operands.remove(0),
            ], span }),
            other => Err(invalid_literal(
                brace_span.clone(),
                format!("expected operand vector of length 1..=4 or 8, found {other}"),
            )),
        }
    }
}

impl PtxParser for GeneralOperand {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if stream.check(|token| matches!(token, PtxToken::LBrace)) {
            let vec_operand = VectorOperand::parse(stream)?;
            let span = vec_operand.span();
            Ok(GeneralOperand::Vec { operand: vec_operand, span })
        } else {
            let operand = Operand::parse(stream)?;
            let span = operand.span();
            Ok(GeneralOperand::Single { operand, span })
        }
    }
}

impl PtxParser for TexHandler2 {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (_, start_span) = stream.expect(&PtxToken::LBracket)?;
        let first = GeneralOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let second = GeneralOperand::parse(stream)?;
        let (_, end_span) = stream.expect(&PtxToken::RBracket)?;
        let span = Span { start: start_span.start, end: end_span.end };
        Ok(TexHandler2 { operands: [first, second], span })
    }
}

impl PtxParser for TexHandler3 {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (_, start_span) = stream.expect(&PtxToken::LBracket)?;
        let handle = GeneralOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let sampler = GeneralOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let coords = GeneralOperand::parse(stream)?;
        let (_, end_span) = stream.expect(&PtxToken::RBracket)?;
        let span = Span { start: start_span.start, end: end_span.end };

        Ok(TexHandler3 {
            handle,
            sampler,
            coords,
            span,
        })
    }
}

impl PtxParser for TexHandler3Optional {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (_, start_span) = stream.expect(&PtxToken::LBracket)?;
        let handle = GeneralOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let second = GeneralOperand::parse(stream)?;

        let (sampler, coords) = if stream
            .consume_if(|token| matches!(token, PtxToken::Comma))
            .is_some()
        {
            let coords = GeneralOperand::parse(stream)?;
            (Some(second), coords)
        } else {
            (None, second)
        };

        let (_, end_span) = stream.expect(&PtxToken::RBracket)?;
        let span = Span { start: start_span.start, end: end_span.end };

        Ok(TexHandler3Optional {
            handle,
            sampler,
            coords,
            span,
        })
    }
}

impl PtxParser for AddressBase {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if stream.check(|token| matches!(token, PtxToken::Register(_))) {
            let register = RegisterOperand::parse(stream)?;
            let span = register.span.clone();
            Ok(AddressBase::Register { operand: register, span })
        } else if stream.check(|token| matches!(token, PtxToken::Identifier(_))) {
            let variable = VariableSymbol::parse(stream)?;
            let span = variable.span.clone();
            Ok(AddressBase::Variable { symbol: variable, span })
        } else {
            let (token, span) = stream.peek()?;
            Err(unexpected_value(
                span.clone(),
                &["register", "identifier"],
                format!("{token:?}"),
            ))
        }
    }
}

impl PtxParser for AddressOffset {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if let Some((_, plus_span)) = stream
            .consume_if(|token| matches!(token, PtxToken::Plus))
        {
            if stream.check(|token| matches!(token, PtxToken::Register(_))) {
                let register = RegisterOperand::parse(stream)?;
                let span = Span { start: plus_span.start, end: register.span.end };
                Ok(AddressOffset::Register { operand: register, span })
            } else {
                let sign = Sign::Positive { span: plus_span.clone() };
                let value = Immediate::parse(stream)?;
                let span = Span { start: plus_span.start, end: value.span.end };
                Ok(AddressOffset::Immediate { sign, value, span })
            }
        } else if let Some((_, minus_span)) = stream
            .consume_if(|token| matches!(token, PtxToken::Minus))
        {
            let sign = Sign::Negative { span: minus_span.clone() };
            let value = Immediate::parse(stream)?;
            let span = Span { start: minus_span.start, end: value.span.end };
            Ok(AddressOffset::Immediate { sign, value, span })
        } else {
            let (token, span) = stream.peek()?;
            Err(unexpected_value(
                span.clone(),
                &["+", "-"],
                format!("{token:?}"),
            ))
        }
    }
}

impl PtxParser for AddressOperand {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if stream.check(|token| matches!(token, PtxToken::Identifier(_))) {
            let saved = stream.position();
            let (identifier, ident_span) = stream.expect_identifier()?;
            if stream
                .consume_if(|token| matches!(token, PtxToken::LBracket))
                .is_some()
            {
                let immediate = Immediate::parse(stream)?;
                let (_, end_span) = stream.expect(&PtxToken::RBracket)?;
                let span = Span { start: ident_span.start, end: end_span.end };
                return Ok(AddressOperand::Array { base: VariableSymbol { name: identifier, span: ident_span }, index: immediate, span });
            } else {
                stream.set_position(saved);
            }
        }

        let (_, start_span) = stream.expect(&PtxToken::LBracket)?;

        if stream.check(|token| matches!(token, PtxToken::Minus)) {
            let pos = stream.position();
            stream.consume()?;
            if stream.check(|token| is_numeric_token(token)) {
                let mut immediate = Immediate::parse(stream)?;
                immediate.value.insert(0, '-');
                let (_, end_span) = stream.expect(&PtxToken::RBracket)?;
                let span = Span { start: start_span.start, end: end_span.end };
                return Ok(AddressOperand::ImmediateAddress { addr: immediate, span });
            } else {
                stream.set_position(pos);
            }
        }

        if stream.check(|token| is_numeric_token(token)) {
            let immediate = Immediate::parse(stream)?;
            let (_, end_span) = stream.expect(&PtxToken::RBracket)?;
            let span = Span { start: start_span.start, end: end_span.end };
            return Ok(AddressOperand::ImmediateAddress { addr: immediate, span });
        }

        let base = AddressBase::parse(stream)?;
        let offset = if stream.check(|token| matches!(token, PtxToken::Plus | PtxToken::Minus)) {
            Some(AddressOffset::parse(stream)?)
        } else {
            None
        };
        let (_, end_span) = stream.expect(&PtxToken::RBracket)?;
        let span = Span { start: start_span.start, end: end_span.end };

        Ok(AddressOperand::Offset { base, offset, span })
    }
}

impl PtxParser for FunctionSymbol {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (name, span) = stream.expect_identifier()?;
        Ok(FunctionSymbol { name, span })
    }
}

impl PtxParser for VariableSymbol {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (name, span) = stream.expect_identifier()?;
        Ok(VariableSymbol { name, span })
    }
}

/// Try to parse an optional label (identifier followed by colon).
/// Returns `Ok(Some(label))` if a label is found, `Ok(None)` if not,
/// or `Err` if parsing fails.
pub(crate) fn try_parse_label(
    stream: &mut PtxTokenStream,
) -> Result<Option<String>, PtxParseError> {
    if !stream.check(|token| matches!(token, PtxToken::Identifier(_))) {
        return Ok(None);
    }

    let position = stream.position();
    let (name, _) = stream.expect_identifier()?;
    if stream
        .consume_if(|token| matches!(token, PtxToken::Colon))
        .is_some()
    {
        Ok(Some(name))
    } else {
        stream.set_position(position);
        Ok(None)
    }
}

impl PtxParser for Instruction {
    /// Parse a PTX instruction with optional predicate
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let start_pos = stream.position();

        // Optional predicate: @{!}pred or @!pred
        let predicate = if stream.check(|t| matches!(t, PtxToken::At)) {
            let (_, at_span) = stream.consume()?; // consume @

            // Optional negation
            let negated = stream
                .consume_if(|t| matches!(t, PtxToken::Exclaim))
                .is_some();

            // Predicate operand (can be register %p1 or identifier p)
            let operand = Operand::parse(stream)?;
            let pred_span = Span { start: at_span.start, end: operand.span().end };

            Some(Predicate { negated, operand, span: pred_span })
        } else {
            None
        };

        // Parse the actual instruction using the module-level dispatcher
        let inst = crate::parser::instruction::parse_instruction_inner(stream)?;

        // Calculate span from the start to the end of the instruction
        let end_pos = stream.position();
        let span = if let Some(ref pred) = predicate {
            Span { start: pred.span.start, end: end_pos.char_offset as usize }
        } else {
            Span { start: start_pos.char_offset as usize, end: end_pos.char_offset as usize }
        };

        Ok(Instruction { predicate, inst, span })
    }
}

// Backwards compatibility: Inst can still be parsed directly
impl PtxParser for Inst {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        Ok(Instruction::parse(stream)?.inst)
    }
}
