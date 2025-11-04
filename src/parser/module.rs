use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::CodeOrDataLinkage,
        function::FunctionKernelDirective,
        module::{
            AddressSizeDirective, FileDirective, LinkingDirective, Module, ModuleDebugDirective,
            ModuleDirective, ModuleInfoDirectiveKind, SectionDirective, TargetDirective,
            VersionDirective,
        },
        variable::ModuleVariableDirective,
    },
};

fn is_module_directive_start(token: &PtxToken) -> bool {
    matches!(token, PtxToken::Dot)
}

fn parse_decimal_u32(
    stream: &mut PtxTokenStream,
) -> Result<(u32, std::ops::Range<usize>), PtxParseError> {
    let (token, span) = stream.consume()?;
    match token {
        PtxToken::DecimalInteger(text) => text
            .parse::<u32>()
            .map(|value| (value, span.clone()))
            .map_err(|_| unexpected_value(span.clone(), &["decimal literal"], text.clone())),
        _ => Err(unexpected_value(
            span.clone(),
            &["decimal literal"],
            format!("{token:?}"),
        )),
    }
}

fn token_to_string(token: &PtxToken) -> String {
    match token {
        PtxToken::Dot => ".".into(),
        PtxToken::Identifier(name) => name.clone(),
        PtxToken::DecimalInteger(value) => value.clone(),
        PtxToken::StringLiteral(value) => format!("\"{value}\""),
        PtxToken::LBrace => "{".into(),
        PtxToken::RBrace => "}".into(),
        PtxToken::Comma => ",".into(),
        PtxToken::Colon => ":".into(),
        PtxToken::Star => "*".into(),
        PtxToken::Plus => "+".into(),
        PtxToken::Minus => "-".into(),
        PtxToken::Slash => "/".into(),
        PtxToken::Percent => "%".into(),
        PtxToken::Equals => "=".into(),
        other => format!("{other:?}"),
    }
}

fn classify_next_directive(stream: &PtxTokenStream) -> Option<String> {
    let mut iter = stream.remaining().iter();
    while let Some((token, _)) = iter.next() {
        match token {
            PtxToken::Dot => {
                // Check if next token is an identifier
                if let Some((PtxToken::Identifier(name), _)) = iter.next() {
                    match name.as_str() {
                        "visible" | "extern" | "weak" | "common" => continue,
                        other => return Some(other.to_string()),
                    }
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }
    None
}

impl PtxParser for VersionDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (token, span) = stream.consume()?;
        match token {
            PtxToken::DecimalInteger(text) => {
                let major = text.parse::<u32>().map_err(|_| {
                    unexpected_value(span.clone(), &["decimal literal"], text.clone())
                })?;
                stream.expect(&PtxToken::Dot)?;
                let (minor_token, minor_span) = stream.consume()?;
                let minor = match minor_token {
                    PtxToken::DecimalInteger(value) => value.parse::<u32>().map_err(|_| {
                        unexpected_value(minor_span.clone(), &["decimal literal"], value.clone())
                    })?,
                    _ => {
                        return Err(unexpected_value(
                            minor_span.clone(),
                            &["decimal literal"],
                            format!("{minor_token:?}"),
                        ));
                    }
                };
                Ok(VersionDirective { major, minor })
            }
            PtxToken::Float(value) | PtxToken::FloatExponent(value) => {
                let mut parts = value.split('.');
                let major_str = parts.next().unwrap_or("");
                let minor_part = parts.next().unwrap_or("");
                if parts.next().is_some() || major_str.is_empty() || minor_part.is_empty() {
                    return Err(unexpected_value(
                        span.clone(),
                        &["major.minor"],
                        value.clone(),
                    ));
                }
                let major = major_str.parse::<u32>().map_err(|_| {
                    unexpected_value(span.clone(), &["decimal literal"], value.clone())
                })?;
                let minor = minor_part.parse::<u32>().map_err(|_| {
                    unexpected_value(span.clone(), &["decimal literal"], value.clone())
                })?;
                Ok(VersionDirective { major, minor })
            }
            _ => Err(unexpected_value(
                span.clone(),
                &["decimal literal"],
                format!("{token:?}"),
            )),
        }
    }
}

impl PtxParser for TargetDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let mut entries = Vec::new();
        loop {
            let next = stream.peek();
            let Ok((token, _span)) = next else {
                break;
            };
            match token {
                PtxToken::Identifier(name) => {
                    entries.push(name.clone());
                    stream.consume()?;
                }
                PtxToken::Dot => {
                    stream.consume()?;
                    let (name, _) = stream.expect_identifier()?;
                    entries.push(format!(".{name}"));
                }
                _ => break,
            }
            if stream
                .consume_if(|token| matches!(token, PtxToken::Comma))
                .is_none()
            {
                break;
            }
        }
        if entries.is_empty() {
            let span = stream.peek().map(|(_, span)| span.clone()).unwrap_or(0..0);
            return Err(unexpected_value(
                span,
                &["sm arch or target modifier"],
                "".to_string(),
            ));
        }
        Ok(TargetDirective {
            entries: entries.clone(),
            raw: entries.join(", "),
        })
    }
}

impl PtxParser for AddressSizeDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (size, _) = parse_decimal_u32(stream)?;
        Ok(AddressSizeDirective { size })
    }
}

impl PtxParser for ModuleInfoDirectiveKind {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "version" => Ok(ModuleInfoDirectiveKind::Version(VersionDirective::parse(
                stream,
            )?)),
            "target" => Ok(ModuleInfoDirectiveKind::Target(TargetDirective::parse(
                stream,
            )?)),
            "address_size" => Ok(ModuleInfoDirectiveKind::AddressSize(
                AddressSizeDirective::parse(stream)?,
            )),
            other => Err(unexpected_value(
                span,
                &[".version", ".target", ".address_size"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for FileDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (index, _) = parse_decimal_u32(stream)?;
        let (token, span) = stream.consume()?;
        let path = match token {
            PtxToken::StringLiteral(content) => content.clone(),
            _ => {
                return Err(unexpected_value(
                    span.clone(),
                    &["string literal"],
                    format!("{token:?}"),
                ));
            }
        };
        Ok(FileDirective { index, path })
    }
}

impl PtxParser for SectionDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (token, span) = stream.consume()?;
        let name = match token {
            PtxToken::Identifier(value) => value.clone(),
            PtxToken::Dot => {
                let (value, _) = stream.expect_identifier()?;
                format!(".{value}")
            }
            _ => {
                return Err(unexpected_value(
                    span.clone(),
                    &["section name"],
                    format!("{token:?}"),
                ));
            }
        };

        let mut attributes = Vec::new();
        loop {
            let next = stream.peek();
            let Ok((token, _)) = next else { break };
            if is_module_directive_start(token) || matches!(token, PtxToken::Semicolon) {
                break;
            }
            let (tok, _) = stream.consume()?;
            attributes.push(token_to_string(tok));
        }

        Ok(SectionDirective { name, attributes })
    }
}

impl PtxParser for ModuleDebugDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "file" => Ok(ModuleDebugDirective::File(FileDirective::parse(stream)?)),
            "section" => Ok(ModuleDebugDirective::Section(SectionDirective::parse(
                stream,
            )?)),
            other => Err(unexpected_value(
                span,
                &[".file", ".section"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for LinkingDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (linkage, _) = parse_code_or_data_linkage(stream)?;
        let mut prototype = String::new();
        loop {
            let next = stream.peek();
            let Ok((token, _span)) = next else { break };
            if is_module_directive_start(token) {
                break;
            }
            match token {
                PtxToken::Semicolon => {
                    stream.consume()?;
                    break;
                }
                _ => {
                    let (tok, _) = stream.consume()?;
                    if !prototype.is_empty() {
                        prototype.push(' ');
                    }
                    prototype.push_str(&token_to_string(tok));
                }
            }
        }
        Ok(LinkingDirective {
            kind: linkage,
            prototype: prototype.clone(),
            raw: prototype,
        })
    }
}

fn parse_code_or_data_linkage(
    stream: &mut PtxTokenStream,
) -> Result<(CodeOrDataLinkage, std::ops::Range<usize>), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    match directive.as_str() {
        "visible" => Ok((CodeOrDataLinkage::Visible, span.clone())),
        "extern" => Ok((CodeOrDataLinkage::Extern, span.clone())),
        "weak" => Ok((CodeOrDataLinkage::Weak, span.clone())),
        "common" => Ok((CodeOrDataLinkage::Common, span.clone())),
        _ => Err(unexpected_value(
            span.clone(),
            &[".visible", ".extern", ".weak", ".common"],
            format!(".{directive}"),
        )),
    }
}

impl PtxParser for ModuleDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let Some(name) = classify_next_directive(stream) else {
            let span = stream.peek()?.1.clone();
            return Err(unexpected_value(span, &["directive"], "".to_string()));
        };

        match name.as_str() {
            "version" | "target" | "address_size" => {
                let info = ModuleInfoDirectiveKind::parse(stream)?;
                Ok(ModuleDirective::ModuleInfo(info))
            }
            "file" | "section" => {
                let debug = ModuleDebugDirective::parse(stream)?;
                Ok(ModuleDirective::Debug(debug))
            }
            "entry" | "func" | "alias" => {
                let function = FunctionKernelDirective::parse(stream)?;
                Ok(ModuleDirective::FunctionKernel(function))
            }
            "global" | "const" | "shared" | "tex" => {
                let variable = ModuleVariableDirective::parse(stream)?;
                Ok(ModuleDirective::ModuleVariable(variable))
            }
            _ => {
                let position = stream.position();
                if let Ok(variable) = ModuleVariableDirective::parse(stream) {
                    return Ok(ModuleDirective::ModuleVariable(variable));
                }
                stream.set_position(position);
                if let Ok(function) = FunctionKernelDirective::parse(stream) {
                    return Ok(ModuleDirective::FunctionKernel(function));
                }
                stream.set_position(position);
                let span = stream
                    .peek()
                    .map(|(_, span)| span.clone())
                    .unwrap_or(position..position);
                Err(unexpected_value(
                    span,
                    &["module directive"],
                    "unrecognised directive".to_string(),
                ))
            }
        }
    }
}

impl PtxParser for Module {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let mut directives = Vec::new();
        while !stream.is_at_end() {
            if stream.is_at_end() {
                break;
            }
            let directive = ModuleDirective::parse(stream)?;
            directives.push(directive);
        }
        Ok(Module { directives })
    }
}
