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

impl PtxParser for VersionDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let start_pos = stream.position().index;
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
                let end_pos = stream.position().index;
                Ok(VersionDirective { major, minor, span: start_pos..end_pos })
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
                let end_pos = stream.position().index;
                Ok(VersionDirective { major, minor, span: start_pos..end_pos })
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
        let start_pos = stream.position().index;
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
        let end_pos = stream.position().index;
        Ok(TargetDirective {
            entries,
            span: start_pos..end_pos,
        })
    }
}

impl PtxParser for AddressSizeDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let start_pos = stream.position().index;
        let (size, _) = parse_decimal_u32(stream)?;
        let end_pos = stream.position().index;
        Ok(AddressSizeDirective { size, span: start_pos..end_pos })
    }
}

impl PtxParser for ModuleInfoDirectiveKind {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let start_pos = stream.position().index;
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "version" => {
                let directive = VersionDirective::parse(stream)?;
                let end_pos = stream.position().index;
                Ok(ModuleInfoDirectiveKind::Version { directive, span: start_pos..end_pos })
            }
            "target" => {
                let directive = TargetDirective::parse(stream)?;
                let end_pos = stream.position().index;
                Ok(ModuleInfoDirectiveKind::Target { directive, span: start_pos..end_pos })
            }
            "address_size" => {
                let directive = AddressSizeDirective::parse(stream)?;
                let end_pos = stream.position().index;
                Ok(ModuleInfoDirectiveKind::AddressSize { directive, span: start_pos..end_pos })
            }
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
        let start_pos = stream.position().index;
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
        let end_pos = stream.position().index;
        Ok(FileDirective { index, path, span: start_pos..end_pos })
    }
}

impl PtxParser for SectionDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let start_pos = stream.position().index;
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

        let end_pos = stream.position().index;
        Ok(SectionDirective { name, attributes, span: start_pos..end_pos })
    }
}

impl PtxParser for ModuleDebugDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let start_pos = stream.position().index;
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "file" => {
                let directive = FileDirective::parse(stream)?;
                let end_pos = stream.position().index;
                Ok(ModuleDebugDirective::File { directive, span: start_pos..end_pos })
            }
            "section" => {
                let directive = SectionDirective::parse(stream)?;
                let end_pos = stream.position().index;
                Ok(ModuleDebugDirective::Section { directive, span: start_pos..end_pos })
            }
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
        let start_pos = stream.position().index;
        let linkage = CodeOrDataLinkage::parse(stream)?;
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
        let end_pos = stream.position().index;
        Ok(LinkingDirective {
            kind: linkage,
            prototype,
            span: start_pos..end_pos,
        })
    }
}

impl PtxParser for ModuleDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let position = stream.position();
        let start_pos = position.index;

        if let Ok(info) = ModuleInfoDirectiveKind::parse(stream) {
            let end_pos = stream.position().index;
            return Ok(ModuleDirective::ModuleInfo { directive: info, span: start_pos..end_pos });
        }
        stream.set_position(position);

        if let Ok(debug) = ModuleDebugDirective::parse(stream) {
            let end_pos = stream.position().index;
            return Ok(ModuleDirective::Debug { directive: debug, span: start_pos..end_pos });
        }
        stream.set_position(position);

        if let Ok(function) = FunctionKernelDirective::parse(stream) {
            let end_pos = stream.position().index;
            return Ok(ModuleDirective::FunctionKernel { directive: function, span: start_pos..end_pos });
        }
        stream.set_position(position);

        if let Ok(variable) = ModuleVariableDirective::parse(stream) {
            let end_pos = stream.position().index;
            return Ok(ModuleDirective::ModuleVariable { directive: variable, span: start_pos..end_pos });
        }
        stream.set_position(position);

        if let Ok(linking) = LinkingDirective::parse(stream) {
            let end_pos = stream.position().index;
            return Ok(ModuleDirective::Linking { directive: linking, span: start_pos..end_pos });
        }
        stream.set_position(position);

        let span = stream
            .peek()
            .map(|(_, span)| span.clone())
            .unwrap_or(position.index..position.index);
        Err(unexpected_value(
            span,
            &["module directive"],
            "unrecognised directive".to_string(),
        ))
    }
}

impl PtxParser for Module {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let start_pos = stream.position().index;
        let mut directives = Vec::new();
        while !stream.is_at_end() {
            if stream.is_at_end() {
                break;
            }
            let directive = ModuleDirective::parse(stream)?;
            directives.push(directive);
        }
        let end_pos = stream.position().index;
        Ok(Module { directives, span: start_pos..end_pos })
    }
}
