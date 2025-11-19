use crate::{
    alt, err, func,
    lexer::PtxToken,
    mapc, ok,
    parser::{
        ParseErrorKind, PtxParseError, PtxParser, PtxTokenStream, Span,
        util::{
            comma_p, directive_exact_p, identifier_p, integer_p, many, optional, parse_u32_literal,
            parse_u64_literal, sep_by, seq, skip_first, skip_semicolon, string_literal_p, try_map,
        },
    },
    seq_n,
    r#type::{
        AliasFunctionDirective, CodeLinkage, DataLinkage, DwarfDirective, EntryFunctionDirective,
        FuncFunctionDirective, SectionDirective, module::*, variable::ModuleVariableDirective,
    },
};

impl PtxParser for Module {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        mapc!(many(ModuleDirective::parse()), Module { directives })
    }
}

impl PtxParser for ModuleDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        alt!(
            parse_module_variable(),
            parse_entry_function(),
            parse_func_function(),
            parse_alias_function(),
            parse_module_info(),
            parse_module_debug()
        )
    }
}

fn parse_module_variable()
-> impl Fn(&mut PtxTokenStream) -> Result<(ModuleDirective, Span), PtxParseError> {
    mapc!(
        seq(
            optional(DataLinkage::parse()),
            ModuleVariableDirective::parse(),
        ),
        ModuleDirective::ModuleVariable { linkage, directive }
    )
}

fn parse_entry_function()
-> impl Fn(&mut PtxTokenStream) -> Result<(ModuleDirective, Span), PtxParseError> {
    mapc!(
        seq(
            optional(CodeLinkage::parse()),
            EntryFunctionDirective::parse(),
        ),
        ModuleDirective::EntryFunction { linkage, directive }
    )
}

fn parse_func_function()
-> impl Fn(&mut PtxTokenStream) -> Result<(ModuleDirective, Span), PtxParseError> {
    mapc!(
        seq(
            optional(CodeLinkage::parse()),
            FuncFunctionDirective::parse(),
        ),
        ModuleDirective::FuncFunction { linkage, directive }
    )
}

fn parse_alias_function()
-> impl Fn(&mut PtxTokenStream) -> Result<(ModuleDirective, Span), PtxParseError> {
    mapc!(
        AliasFunctionDirective::parse(),
        ModuleDirective::AliasFunction { directive }
    )
}

fn parse_module_info()
-> impl Fn(&mut PtxTokenStream) -> Result<(ModuleDirective, Span), PtxParseError> {
    mapc!(
        ModuleInfoDirectiveKind::parse(),
        ModuleDirective::ModuleInfo { directive }
    )
}

fn parse_module_debug()
-> impl Fn(&mut PtxTokenStream) -> Result<(ModuleDirective, Span), PtxParseError> {
    mapc!(
        ModuleDebugDirective::parse(),
        ModuleDirective::Debug { directive }
    )
}

impl PtxParser for ModuleInfoDirectiveKind {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        alt!(
            mapc!(
                VersionDirective::parse(),
                ModuleInfoDirectiveKind::Version { directive }
            ),
            mapc!(
                TargetDirective::parse(),
                ModuleInfoDirectiveKind::Target { directive }
            ),
            mapc!(
                AddressSizeDirective::parse(),
                ModuleInfoDirectiveKind::AddressSize { directive }
            )
        )
    }
}

impl PtxParser for VersionDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        try_map(
            skip_first(directive_exact_p("version"), version_number_p()),
            func!(|(major, minor)| { ok!(VersionDirective { major, minor }) }),
        )
    }
}

impl PtxParser for TargetDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        mapc!(
            skip_first(
                directive_exact_p("target"),
                sep_by(TargetString::parse(), comma_p()),
            ),
            TargetDirective { entries }
        )
    }
}

impl PtxParser for TargetString {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        // Parse target specifiers like "sm_80", "texmode_unified", etc.
        try_map(
            identifier_p(),
            func!(|name| match name.as_str() {
                "sm_120a" => ok!(TargetString::Sm120a),
                "sm_120f" => ok!(TargetString::Sm120f),
                "sm_120" => ok!(TargetString::Sm120),
                "sm_121a" => ok!(TargetString::Sm121a),
                "sm_121f" => ok!(TargetString::Sm121f),
                "sm_121" => ok!(TargetString::Sm121),
                "sm_110a" => ok!(TargetString::Sm110a),
                "sm_110f" => ok!(TargetString::Sm110f),
                "sm_110" => ok!(TargetString::Sm110),
                "sm_100a" => ok!(TargetString::Sm100a),
                "sm_100f" => ok!(TargetString::Sm100f),
                "sm_100" => ok!(TargetString::Sm100),
                "sm_101a" => ok!(TargetString::Sm101a),
                "sm_101f" => ok!(TargetString::Sm101f),
                "sm_101" => ok!(TargetString::Sm101),
                "sm_103a" => ok!(TargetString::Sm103a),
                "sm_103f" => ok!(TargetString::Sm103f),
                "sm_103" => ok!(TargetString::Sm103),
                "sm_90a" => ok!(TargetString::Sm90a),
                "sm_90" => ok!(TargetString::Sm90),
                "sm_80" => ok!(TargetString::Sm80),
                "sm_86" => ok!(TargetString::Sm86),
                "sm_87" => ok!(TargetString::Sm87),
                "sm_88" => ok!(TargetString::Sm88),
                "sm_89" => ok!(TargetString::Sm89),
                "sm_70" => ok!(TargetString::Sm70),
                "sm_72" => ok!(TargetString::Sm72),
                "sm_75" => ok!(TargetString::Sm75),
                "sm_60" => ok!(TargetString::Sm60),
                "sm_61" => ok!(TargetString::Sm61),
                "sm_62" => ok!(TargetString::Sm62),
                "sm_50" => ok!(TargetString::Sm50),
                "sm_52" => ok!(TargetString::Sm52),
                "sm_53" => ok!(TargetString::Sm53),
                "sm_30" => ok!(TargetString::Sm30),
                "sm_32" => ok!(TargetString::Sm32),
                "sm_35" => ok!(TargetString::Sm35),
                "sm_37" => ok!(TargetString::Sm37),
                "sm_20" => ok!(TargetString::Sm20),
                "sm_10" => ok!(TargetString::Sm10),
                "sm_11" => ok!(TargetString::Sm11),
                "sm_12" => ok!(TargetString::Sm12),
                "sm_13" => ok!(TargetString::Sm13),
                "texmode_unified" => ok!(TargetString::TexmodeUnified),
                "texmode_independent" => ok!(TargetString::TexmodeIndependent),
                "debug" => ok!(TargetString::Debug),
                "map_f64_to_f32" => ok!(TargetString::MapF64ToF32),
                _ => err!(ParseErrorKind::InvalidLiteral(format!(
                    "unknown target specifier: {}",
                    name
                ))),
            }),
        )
    }
}

impl PtxParser for AddressSizeDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        mapc!(
            skip_first(directive_exact_p("address_size"), AddressSize::parse()),
            AddressSizeDirective { size }
        )
    }
}

impl PtxParser for ModuleDebugDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        alt!(
            mapc!(
                FileDirective::parse(),
                ModuleDebugDirective::File { directive }
            ),
            mapc!(
                SectionDirective::parse(),
                ModuleDebugDirective::Section { directive }
            ),
            mapc!(
                skip_semicolon(DwarfDirective::parse()),
                ModuleDebugDirective::Dwarf { directive }
            )
        )
    }
}

impl PtxParser for FileDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        try_map(
            skip_first(
                directive_exact_p("file"),
                seq_n!(
                    integer_p(),
                    string_literal_p(),
                    optional(skip_first(
                        comma_p(),
                        seq(integer_p(), skip_first(comma_p(), integer_p())),
                    )),
                ),
            ),
            |(index_str, path, maybe_timestamps), span| {
                let index = parse_u32_literal(&index_str, span)?;
                let (timestamp, file_size) = if let Some((ts_str, size_str)) = maybe_timestamps {
                    let ts = parse_u64_literal(&ts_str, span)?;
                    let size = parse_u64_literal(&size_str, span)?;
                    (Some(ts), Some(size))
                } else {
                    (None, None)
                };
                ok!(FileDirective {
                    index,
                    path,
                    timestamp,
                    file_size,
                })
            },
        )
    }
}

impl PtxParser for AddressSize {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        try_map(
            integer_p(),
            func!(|value| match value.as_str() {
                "32" => ok!(AddressSize::Size32),
                "64" => ok!(AddressSize::Size64),
                _ => err!(ParseErrorKind::InvalidLiteral(format!(
                    "invalid address size: {} (expected 32 or 64)",
                    value
                ))),
            }),
        )
    }
}

/// Parser for version numbers - handles both Float("8.5") and separate tokens (8 . 5)
fn version_number_p() -> impl Fn(&mut PtxTokenStream) -> Result<((u32, u32), Span), PtxParseError> {
    |stream| {
        let start_pos = stream.position().0;

        // Try to parse as float first
        if let Ok((token, span)) = stream.peek() {
            if let PtxToken::Float(f) = token {
                let version_str = f.clone();
                stream.consume()?;
                let end_pos = stream.position().0;
                let full_span = Span::new(start_pos, end_pos);
                let parts: Vec<&str> = version_str.split('.').collect();
                let span = span.clone();
                if parts.len() != 2 {
                    return err!(ParseErrorKind::InvalidLiteral(format!(
                        "expected version in format X.Y, got {}",
                        version_str
                    )));
                }
                let major = parse_u32_literal(parts[0], span)?;
                let minor = parse_u32_literal(parts[1], span)?;
                return Ok(((major, minor), full_span));
            }
        }

        // Otherwise parse as integer.integer
        let (major_str, major_span) = integer_p()(stream)?;
        stream.expect(&PtxToken::Dot)?;
        let (minor_str, minor_span) = integer_p()(stream)?;

        let end_pos = stream.position().0;
        let span = Span::new(start_pos, end_pos);
        let major = parse_u32_literal(&major_str, major_span)?;
        let minor = parse_u32_literal(&minor_str, minor_span)?;
        Ok(((major, minor), span))
    }
}
