use std::borrow::Cow;

use crate::{
    lexer::PtxToken,
    parser::{ParseErrorKind, PtxParseError, PtxParser, PtxTokenStream, Span},
    r#type::common::*,
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
                // consume the dot and expect an identifier component
                stream.consume()?;
                let (component, component_span) = stream.expect_identifier()?;

                name.push('.');
                name.push_str(&component);

                span.end = component_span.end;
            }
            PtxToken::Directive(s) if s == "x" || s == "y" || s == "z" => {
                // Treat directive ".x/.y/.z" as a lightweight component for special registers
                let (component, component_span) = stream.expect_directive()?;
                name.push('.');
                name.push_str(&component);
                span.end = component_span.end;
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
            "visible" => Ok(CodeLinkage::Visible),
            "extern" => Ok(CodeLinkage::Extern),
            "weak" => Ok(CodeLinkage::Weak),
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
            "visible" => Ok(DataLinkage::Visible),
            "extern" => Ok(DataLinkage::Extern),
            "weak" => Ok(DataLinkage::Weak),
            "common" => Ok(DataLinkage::Common),
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
            "visible" => Ok(CodeOrDataLinkage::Visible),
            "extern" => Ok(CodeOrDataLinkage::Extern),
            "weak" => Ok(CodeOrDataLinkage::Weak),
            "common" => Ok(CodeOrDataLinkage::Common),
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
            "texref" => Ok(TexType::TexRef),
            "samplerref" => Ok(TexType::SamplerRef),
            "surfref" => Ok(TexType::SurfRef),
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
            "global" => Ok(AddressSpace::Global),
            "const" => Ok(AddressSpace::Const),
            "shared" => Ok(AddressSpace::Shared),
            "local" => Ok(AddressSpace::Local),
            "param" => Ok(AddressSpace::Param),
            "reg" => Ok(AddressSpace::Reg),
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
                Ok(AttributeDirective::Unified(uuid1, uuid2))
            }
            "managed" => Ok(AttributeDirective::Managed),
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
            "u8" => Ok(DataType::U8),
            "u16" => Ok(DataType::U16),
            "u32" => Ok(DataType::U32),
            "u64" => Ok(DataType::U64),
            "s8" => Ok(DataType::S8),
            "s16" => Ok(DataType::S16),
            "s32" => Ok(DataType::S32),
            "s64" => Ok(DataType::S64),
            "f16" => Ok(DataType::F16),
            "f16x2" => Ok(DataType::F16x2),
            "f32" => Ok(DataType::F32),
            "f64" => Ok(DataType::F64),
            "b8" => Ok(DataType::B8),
            "b16" => Ok(DataType::B16),
            "b32" => Ok(DataType::B32),
            "b64" => Ok(DataType::B64),
            "b128" => Ok(DataType::B128),
            "pred" => Ok(DataType::Pred),
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
        if stream
            .consume_if(|token| matches!(token, PtxToken::Plus))
            .is_some()
        {
            return Ok(Sign::Positive);
        }
        if stream
            .consume_if(|token| matches!(token, PtxToken::Minus))
            .is_some()
        {
            return Ok(Sign::Negative);
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
        let (token, span) = stream.peek()?;
        let value = numeric_literal(token).cloned();
        match value {
            Some(value) => {
                let literal = value.clone();
                stream.consume()?;
                Ok(Immediate(literal))
            }
            None => Err(unexpected_value(
                span.clone(),
                &["numeric literal"],
                format!("{token:?}"),
            )),
        }
    }
}

impl PtxParser for RegisterOperand {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        // Vector register
        if stream.check(|token| matches!(token, PtxToken::LBrace)) {
            let (_, brace_span) = stream.consume()?;
            let mut span = brace_span.clone();

            let mut registers = Vec::new();
            loop {
                let (name, reg_span) = parse_register_name(stream)?;
                registers.push(name);
                span.end = reg_span.end;

                if stream
                    .consume_if(|token| matches!(token, PtxToken::Comma))
                    .is_some()
                {
                    continue;
                }
                break;
            }

            let (_, closing_span) = stream.expect(&PtxToken::RBrace)?;
            span.end = closing_span.end;

            match registers.len() {
                2 => {
                    let mut iter = registers.into_iter();
                    Ok(RegisterOperand::Vector2([
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                    ]))
                }
                4 => {
                    let mut iter = registers.into_iter();
                    Ok(RegisterOperand::Vector4([
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                    ]))
                }
                other => Err(invalid_literal(
                    span,
                    format!("expected register vector of length 2 or 4, found {other}"),
                )),
            }
        } else {
            let (name, _) = parse_register_name(stream)?;
            Ok(RegisterOperand::Single(name))
        }
    }
}

impl PtxParser for PredicateRegister {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (name, span) = parse_register_name(stream)?;
        if name.starts_with("%p") {
            Ok(PredicateRegister(name))
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
        let (name, _) = stream.expect_identifier()?;
        Ok(Label(name))
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
                return Ok(SpecialRegister::ClusterCtaid(Axis::None));
            } else if rest == ".x" {
                return Ok(SpecialRegister::ClusterCtaid(Axis::X));
            } else if rest == ".y" {
                return Ok(SpecialRegister::ClusterCtaid(Axis::Y));
            } else if rest == ".z" {
                return Ok(SpecialRegister::ClusterCtaid(Axis::Z));
            }
        }
        if let Some(rest) = name_str.strip_prefix("%cluster_ctarank") {
            if rest.is_empty() {
                return Ok(SpecialRegister::ClusterCtarank(Axis::None));
            } else if rest == ".x" {
                return Ok(SpecialRegister::ClusterCtarank(Axis::X));
            } else if rest == ".y" {
                return Ok(SpecialRegister::ClusterCtarank(Axis::Y));
            } else if rest == ".z" {
                return Ok(SpecialRegister::ClusterCtarank(Axis::Z));
            }
        }
        if let Some(rest) = name_str.strip_prefix("%nctaid") {
            if rest.is_empty() {
                return Ok(SpecialRegister::Nctaid(Axis::None));
            } else if rest == ".x" {
                return Ok(SpecialRegister::Nctaid(Axis::X));
            } else if rest == ".y" {
                return Ok(SpecialRegister::Nctaid(Axis::Y));
            } else if rest == ".z" {
                return Ok(SpecialRegister::Nctaid(Axis::Z));
            }
        }
        if let Some(rest) = name_str.strip_prefix("%tid") {
            if rest.is_empty() {
                return Ok(SpecialRegister::Tid(Axis::None));
            } else if rest == ".x" {
                return Ok(SpecialRegister::Tid(Axis::X));
            } else if rest == ".y" {
                return Ok(SpecialRegister::Tid(Axis::Y));
            } else if rest == ".z" {
                return Ok(SpecialRegister::Tid(Axis::Z));
            }
        }
        if let Some(rest) = name_str.strip_prefix("%cluster_nctaid") {
            if rest.is_empty() {
                return Ok(SpecialRegister::ClusterNctaid(Axis::None));
            } else if rest == ".x" {
                return Ok(SpecialRegister::ClusterNctaid(Axis::X));
            } else if rest == ".y" {
                return Ok(SpecialRegister::ClusterNctaid(Axis::Y));
            } else if rest == ".z" {
                return Ok(SpecialRegister::ClusterNctaid(Axis::Z));
            }
        }
        if let Some(rest) = name_str.strip_prefix("%cluster_nctarank") {
            if rest.is_empty() {
                return Ok(SpecialRegister::ClusterNctarank(Axis::None));
            } else if rest == ".x" {
                return Ok(SpecialRegister::ClusterNctarank(Axis::X));
            } else if rest == ".y" {
                return Ok(SpecialRegister::ClusterNctarank(Axis::Y));
            } else if rest == ".z" {
                return Ok(SpecialRegister::ClusterNctarank(Axis::Z));
            }
        }
        if let Some(rest) = name_str.strip_prefix("%ntid") {
            if rest.is_empty() {
                return Ok(SpecialRegister::Ntid(Axis::None));
            } else if rest == ".x" {
                return Ok(SpecialRegister::Ntid(Axis::X));
            } else if rest == ".y" {
                return Ok(SpecialRegister::Ntid(Axis::Y));
            } else if rest == ".z" {
                return Ok(SpecialRegister::Ntid(Axis::Z));
            }
        }
        if let Some(rest) = name_str.strip_prefix("%ctaid") {
            if rest.is_empty() {
                return Ok(SpecialRegister::Ctaid(Axis::None));
            } else if rest == ".x" {
                return Ok(SpecialRegister::Ctaid(Axis::X));
            } else if rest == ".y" {
                return Ok(SpecialRegister::Ctaid(Axis::Y));
            } else if rest == ".z" {
                return Ok(SpecialRegister::Ctaid(Axis::Z));
            }
        }

        match name.as_str() {
            "%aggr_smem_size" => Ok(SpecialRegister::AggrSmemSize),
            "%dynamic_smem_size" => Ok(SpecialRegister::DynamicSmemSize),
            "%lanemask_gt" => Ok(SpecialRegister::LanemaskGt),
            "%reserved_smem_offset_begin" => Ok(SpecialRegister::ReservedSmemOffsetBegin),
            "%clock" => Ok(SpecialRegister::Clock),
            "%lanemask_le" => Ok(SpecialRegister::LanemaskLe),
            "%reserved_smem_offset_cap" => Ok(SpecialRegister::ReservedSmemOffsetCap),
            "%clock64" => Ok(SpecialRegister::Clock64),
            "%globaltimer" => Ok(SpecialRegister::Globaltimer),
            "%lanemask_lt" => Ok(SpecialRegister::LanemaskLt),
            "%reserved_smem_offset_end" => Ok(SpecialRegister::ReservedSmemOffsetEnd),
            "%cluster_ctaid" | "%cluster_ctaid.x" | "%cluster_ctaid.y" | "%cluster_ctaid.z" => {
                Ok(SpecialRegister::ClusterCtaid(Axis::None))
            }
            "%globaltimer_hi" => Ok(SpecialRegister::GlobaltimerHi),
            "%nclusterid" => Ok(SpecialRegister::Nclusterid),
            "%smid" => Ok(SpecialRegister::Smid),
            "%cluster_ctarank" | "%cluster_ctarank.x" | "%cluster_ctarank.y"
            | "%cluster_ctarank.z" => Ok(SpecialRegister::ClusterCtarank(Axis::None)),
            "%globaltimer_lo" => Ok(SpecialRegister::GlobaltimerLo),
            "%nctaid" | "%nctaid.x" | "%nctaid.y" | "%nctaid.z" => {
                Ok(SpecialRegister::Nctaid(Axis::None))
            }
            "%tid" | "%tid.x" | "%tid.y" | "%tid.z" => Ok(SpecialRegister::Tid(Axis::None)),
            "%cluster_nctaid" | "%cluster_nctaid.x" | "%cluster_nctaid.y" | "%cluster_nctaid.z" => {
                Ok(SpecialRegister::ClusterNctaid(Axis::None))
            }
            "%gridid" => Ok(SpecialRegister::Gridid),
            "%nsmid" => Ok(SpecialRegister::Nsmid),
            "%total_smem_size" => Ok(SpecialRegister::TotalSmemSize),
            "%cluster_nctarank"
            | "%cluster_nctarank.x"
            | "%cluster_nctarank.y"
            | "%cluster_nctarank.z" => Ok(SpecialRegister::ClusterNctarank(Axis::None)),
            "%is_explicit_cluster" => Ok(SpecialRegister::IsExplicitCluster),
            "%ntid" | "%ntid.x" | "%ntid.y" | "%ntid.z" => Ok(SpecialRegister::Ntid(Axis::None)),
            "%warpid" => Ok(SpecialRegister::Warpid),
            "%clusterid" => Ok(SpecialRegister::Clusterid),
            "%laneid" => Ok(SpecialRegister::Laneid),
            "%nwarpid" => Ok(SpecialRegister::Nwarpid),
            "%WARPSZ" => Ok(SpecialRegister::WARPSZ),
            "%ctaid" | "%ctaid.x" | "%ctaid.y" | "%ctaid.z" => {
                Ok(SpecialRegister::Ctaid(Axis::None))
            }
            "%lanemask_eq" => Ok(SpecialRegister::LanemaskEq),
            "%current_graph_exec" => Ok(SpecialRegister::CurrentGraphExec),
            "%lanemask_ge" => Ok(SpecialRegister::LanemaskGe),
            other => {
                if let Some(num) = other.strip_prefix("%envreg") {
                    let value = num
                        .parse::<u8>()
                        .map_err(|_| invalid_literal(span.clone(), name.clone()))?;
                    if value <= 31 {
                        return Ok(SpecialRegister::Envreg(value));
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
                            return Ok(SpecialRegister::Pm64(value));
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
                        return Ok(SpecialRegister::Pm(value));
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
                        return Ok(SpecialRegister::ReservedSmemOffset(value));
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
        if stream.check(|token| matches!(token, PtxToken::Register(_) | PtxToken::LBrace)) {
            Ok(Operand::Register(RegisterOperand::parse(stream)?))
        } else {
            Ok(Operand::Immediate(Immediate::parse(stream)?))
        }
    }
}

impl PtxParser for AddressBase {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if stream.check(|token| matches!(token, PtxToken::Register(_))) {
            Ok(AddressBase::Register(RegisterOperand::parse(stream)?))
        } else if stream.check(|token| matches!(token, PtxToken::Identifier(_))) {
            Ok(AddressBase::Variable(VariableSymbol::parse(stream)?))
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
        if stream
            .consume_if(|token| matches!(token, PtxToken::Plus))
            .is_some()
        {
            if stream.check(|token| matches!(token, PtxToken::Register(_))) {
                Ok(AddressOffset::Register(RegisterOperand::parse(stream)?))
            } else {
                Ok(AddressOffset::Immediate(
                    Sign::Positive,
                    Immediate::parse(stream)?,
                ))
            }
        } else if stream
            .consume_if(|token| matches!(token, PtxToken::Minus))
            .is_some()
        {
            Ok(AddressOffset::Immediate(
                Sign::Negative,
                Immediate::parse(stream)?,
            ))
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
            let (identifier, _) = stream.expect_identifier()?;
            if stream
                .consume_if(|token| matches!(token, PtxToken::LBracket))
                .is_some()
            {
                let immediate = Immediate::parse(stream)?;
                stream.expect(&PtxToken::RBracket)?;
                return Ok(AddressOperand::Array(VariableSymbol(identifier), immediate));
            } else {
                stream.set_position(saved);
            }
        }

        stream.expect(&PtxToken::LBracket)?;

        if stream.check(|token| matches!(token, PtxToken::Minus)) {
            let pos = stream.position();
            stream.consume()?;
            if stream.check(|token| is_numeric_token(token)) {
                let mut immediate = Immediate::parse(stream)?;
                immediate.0.insert(0, '-');
                stream.expect(&PtxToken::RBracket)?;
                return Ok(AddressOperand::ImmediateAddress(immediate));
            } else {
                stream.set_position(pos);
            }
        }

        if stream.check(|token| is_numeric_token(token)) {
            let immediate = Immediate::parse(stream)?;
            stream.expect(&PtxToken::RBracket)?;
            return Ok(AddressOperand::ImmediateAddress(immediate));
        }

        let base = AddressBase::parse(stream)?;
        let offset = if stream.check(|token| matches!(token, PtxToken::Plus | PtxToken::Minus)) {
            Some(AddressOffset::parse(stream)?)
        } else {
            None
        };
        stream.expect(&PtxToken::RBracket)?;

        Ok(AddressOperand::Offset(base, offset))
    }
}

impl PtxParser for FunctionSymbol {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (name, _) = stream.expect_identifier()?;
        Ok(FunctionSymbol(name))
    }
}

impl PtxParser for VariableSymbol {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (name, _) = stream.expect_identifier()?;
        Ok(VariableSymbol(name))
    }
}
