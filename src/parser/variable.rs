use crate::{
    alt, cclosure, err, mapc, ok,
    parser::{
        ParseErrorKind, PtxParseError, PtxParser, PtxTokenStream, Span,
        util::{
            between, comma_p, directive_exact_p, directive_p, equals_p, lbrace_p, lbracket_p, many,
            map, optional, rbrace_p, rbracket_p, semicolon_p, sep_by, seq, skip_first,
            string_literal_p, try_map, u32_p, u64_p,
        },
    },
    seq_n,
    r#type::{
        AttributeDirective, DataType, FunctionSymbol, GlobalInitializer, Immediate,
        InitializerValue, ModuleVariableDirective, ParamStateSpace, ParameterDirective,
        VariableDirective, VariableModifier, VariableSymbol,
    },
};

impl PtxParser for ModuleVariableDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        let tex = mapc!(
            skip_first(directive_exact_p("tex"), VariableDirective::parse()),
            ModuleVariableDirective::Tex { directive }
        );
        let shared = mapc!(
            skip_first(directive_exact_p("shared"), VariableDirective::parse()),
            ModuleVariableDirective::Shared { directive }
        );
        let global = mapc!(
            skip_first(directive_exact_p("global"), VariableDirective::parse()),
            ModuleVariableDirective::Global { directive }
        );
        let konst = mapc!(
            skip_first(directive_exact_p("const"), VariableDirective::parse()),
            ModuleVariableDirective::Const { directive }
        );
        alt!(tex, shared, global, konst)
    }
}

impl PtxParser for VariableDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        mapc!(
            seq_n!(
                many(AttributeDirective::parse()),
                many(VariableModifier::parse()),
                DataType::parse(),
                VariableSymbol::parse(),
                array_dimensions_parser(),
                optional(initializer_assignment()),
                semicolon_p()
            ),
            VariableDirective {
                attributes,
                modifiers,
                ty,
                name,
                array_dims,
                initializer,
                _
            }
        )
    }
}

impl PtxParser for VariableModifier {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        let alignment = try_map(
            skip_first(directive_exact_p("align"), u32_p()),
            |value, span| ok!(VariableModifier::Alignment { value }),
        );
        let ptr = map(
            directive_exact_p("ptr"),
            cclosure!(VariableModifier::Ptr {}),
        );
        let vector = try_map(directive_p(), |name, span| {
            if let Some(width) = name.strip_prefix('v') {
                if width.is_empty() {
                    return err!(ParseErrorKind::InvalidLiteral(
                        "vector modifier requires width (e.g. .v4)".into(),
                    ));
                }
                let value = width.parse::<u32>().map_err(|_| PtxParseError {
                    kind: ParseErrorKind::InvalidLiteral(format!("invalid vector width: {width}")),
                    span,
                })?;
                ok!(VariableModifier::Vector { value })
            } else {
                err!(ParseErrorKind::InvalidLiteral(format!(
                    "unknown variable modifier: .{name}"
                )))
            }
        });

        alt!(alignment, ptr, vector)
    }
}

impl PtxParser for ParameterDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        let register = mapc!(
            seq_n!(
                directive_exact_p("reg"),
                DataType::parse(),
                VariableSymbol::parse()
            ),
            ParameterDirective::Register { _, ty, name }
        );
        let param = mapc!(
            skip_first(directive_exact_p("param"), parameter_spec_parser()),
            ParameterDirective::Parameter {
                align,
                ptr,
                space,
                ty,
                name,
                array,
            }
        );

        alt!(register, param)
    }
}

impl PtxParser for InitializerValue {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        alt!(
            mapc!(
                Immediate::parse(),
                InitializerValue::NumericLiteral { value }
            ),
            mapc!(
                FunctionSymbol::parse(),
                InitializerValue::FunctionSymbol { name }
            ),
            mapc!(
                string_literal_p(),
                InitializerValue::StringLiteral { value }
            ),
        )
    }
}

impl PtxParser for GlobalInitializer {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        let aggregate = move |stream: &mut PtxTokenStream| {
            let inner = GlobalInitializer::parse();
            between(lbrace_p(), rbrace_p(), sep_by(inner, comma_p()))(stream)
        };
        alt!(
            mapc!(aggregate, GlobalInitializer::Aggregate { values }),
            mapc!(
                InitializerValue::parse(),
                GlobalInitializer::Scalar { value }
            ),
        )
    }
}

fn initializer_assignment()
-> impl Fn(&mut PtxTokenStream) -> Result<(GlobalInitializer, Span), PtxParseError> {
    skip_first(equals_p(), GlobalInitializer::parse())
}

fn array_dimensions_parser()
-> impl Fn(&mut PtxTokenStream) -> Result<(Vec<Option<u64>>, Span), PtxParseError> {
    many(array_dimension_parser())
}

fn array_dimension_parser()
-> impl Fn(&mut PtxTokenStream) -> Result<(Option<u64>, Span), PtxParseError> {
    try_map(
        between(lbracket_p(), rbracket_p(), optional(u64_p())),
        |maybe_value, _span| Ok(maybe_value),
    )
}

fn parse_alignment_modifier() -> impl Fn(&mut PtxTokenStream) -> Result<(u32, Span), PtxParseError>
{
    try_map(
        seq(directive_exact_p("align"), u32_p()),
        |(_, value), _span| Ok(value),
    )
}

fn param_modifier() -> impl Fn(
    &mut PtxTokenStream,
) -> Result<
    ((Option<u32>, bool, Option<ParamStateSpace>), Span),
    PtxParseError,
> {
    alt!(
        map(parse_alignment_modifier(), |value, _| (
            Some(value),
            false,
            None
        )),
        map(directive_exact_p("ptr"), |_, _| (None, true, None)),
        map(ParamStateSpace::parse(), |space, _| (
            None,
            false,
            Some(space)
        ))
    )
}

fn apply_param_mods(
    mods: impl IntoIterator<Item = (Option<u32>, bool, Option<ParamStateSpace>)>,
    align: &mut Option<u32>,
    ptr: &mut bool,
    space: &mut Option<ParamStateSpace>,
) {
    for (a, p, s) in mods {
        if let Some(v) = a {
            *align = Some(v);
        }
        if p {
            *ptr = true;
        }
        if let Some(ss) = s {
            *space = Some(ss);
        }
    }
}

fn parameter_spec_parser() -> impl Fn(
    &mut PtxTokenStream,
) -> Result<
    (
        (
            Option<u32>,
            bool,
            Option<ParamStateSpace>,
            DataType,
            VariableSymbol,
            Vec<Option<u64>>,
        ),
        Span,
    ),
    PtxParseError,
> {
    move |stream| {
        stream.try_with_span(|stream| {
            let mut align = None;
            let mut ptr = false;
            let mut space = None;

            let (mods_before, _) = many(param_modifier())(stream)?;
            apply_param_mods(mods_before, &mut align, &mut ptr, &mut space);

            let (ty, _) = DataType::parse()(stream)?;

            let (mods_after, _) = many(param_modifier())(stream)?;
            apply_param_mods(mods_after, &mut align, &mut ptr, &mut space);

            let (name, _) = VariableSymbol::parse()(stream)?;
            let (array_dims, _) = map(array_dimensions_parser(), |dims, _| dims)(stream)?;

            Ok((align, ptr, space, ty, name, array_dims))
        })
    }
}
