use ptx_90_parser_construct::func;

use crate::{
    alt, c,
    lexer::PtxToken,
    mapc, ok,
    parser::{
        ParseErrorKind, PtxParseError, PtxParser, PtxTokenStream, Span,
        util::{
            alt, between, colon_p, comma_p, directive_exact_p, directive_p, identifier_p,
            integer_p, langle_p, lbrace_p, lparen_p, many, map, minus_p, optional,
            parse_signed_integer, parse_u32_literal, parse_unsigned_integer, plus_p, pure,
            rangle_p, rbrace_p, register_p, rparen_p, semicolon_p, sep_by, sep_by1, seq, seq5,
            skip_first, skip_second, skip_semicolon, string_literal_p, try_map, u32_p,
        },
    },
    seq_n,
    r#type::{
        AliasFunctionDirective, AttributeDirective, BranchTargetsDirective, CallPrototypeDirective,
        CallPrototypeReturnSpec, CallTargetsDirective, DataType, DwarfDirective, DwarfDirectiveKind,
        EntryFunctionDirective, EntryFunctionHeaderDirective, FuncFunctionDirective,
        FuncFunctionHeaderDirective, FunctionBody, FunctionDim, FunctionStatement, FunctionSymbol,
        Instruction, Label, LocationDirective, LocationInlinedAt, ParameterDirective,
        PragmaDirective, PragmaDirectiveKind, RegisterDirective, RegisterTarget, SectionDirective,
        SectionEntry, StatementDirective, StatementSectionDirectiveLine, VariableDirective,
        VariableSymbol,
    },
};

impl PtxParser for StatementDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        let branch_targets = try_map(
            skip_semicolon(skip_first(
                directive_exact_p("branchtargets"),
                sep_by1(Label::parse(), comma_p()),
            )),
            |labels, span| {
                let directive = BranchTargetsDirective {
                    labels,
                    span: span.clone(),
                };
                ok!(StatementDirective::BranchTargets { directive })
            },
        );

        let call_targets = try_map(
            skip_semicolon(skip_first(
                directive_exact_p("calltargets"),
                sep_by1(FunctionSymbol::parse(), comma_p()),
            )),
            |targets, span| {
                let directive = CallTargetsDirective {
                    targets,
                    span: span.clone(),
                };
                ok!(StatementDirective::CallTargets { directive })
            },
        );

        let call_prototype = try_map(
            skip_semicolon(skip_first(
                directive_exact_p("callprototype"),
                seq5(
                    return_spec_parser(),
                    parameter_list_parser(),
                    noreturn_parser(),
                    abi_preserve_parser(),
                    abi_preserve_control_parser(),
                ),
            )),
            |(return_spec, params, noreturn, abi_preserve, abi_preserve_control), span| {
                let directive = CallPrototypeDirective {
                    return_spec,
                    params,
                    noreturn,
                    abi_preserve,
                    abi_preserve_control,
                    span: span.clone(),
                };
                ok!(StatementDirective::CallPrototype { directive })
            },
        );

        let location = mapc!(location_directive(), StatementDirective::Loc { directive });

        let reg_stmt = mapc!(register_statement(), StatementDirective::Reg { directive });

        let local_stmt = mapc!(
            skip_first(directive_exact_p("local"), VariableDirective::parse()),
            StatementDirective::Local { directive }
        );

        let param_stmt = mapc!(
            skip_first(directive_exact_p("param"), VariableDirective::parse()),
            StatementDirective::Param { directive }
        );

        let shared_stmt = mapc!(
            skip_first(directive_exact_p("shared"), VariableDirective::parse()),
            StatementDirective::Shared { directive }
        );

        alt!(
            location,
            reg_stmt,
            local_stmt,
            param_stmt,
            shared_stmt,
            branch_targets,
            call_targets,
            call_prototype,
            dwarf_directive(),
            pragma_directive(),
            section_directive()
        )
    }
}

impl PtxParser for SectionDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        map(
            skip_first(
                directive_exact_p("section"),
                seq(section_name_parser(), section_body_parser()),
            ),
            |(name, entries), span| {
                c!(SectionDirective {
                    name = name,
                    entries,
                })
            },
        )
    }
}

impl PtxParser for DwarfDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        skip_first(directive_exact_p("dwarf"), dwarf_kind_parser())
    }
}

impl PtxParser for FunctionStatement {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        let label_stmt = map(seq(Label::parse(), colon_p()), |(label, _), span| {
            c!(FunctionStatement::Label { label })
        });

        let block_stmt = move |stream: &mut PtxTokenStream| {
            map(
                between(lbrace_p(), rbrace_p(), many(FunctionStatement::parse())),
                |statements, span| c!(FunctionStatement::Block { statements }),
            )(stream)
        };

        let directive_stmt = mapc!(
            StatementDirective::parse(),
            FunctionStatement::Directive { directive }
        );

        let instruction_stmt = mapc!(
            Instruction::parse(),
            FunctionStatement::Instruction { instruction }
        );

        alt!(label_stmt, block_stmt, directive_stmt, instruction_stmt)
    }
}

fn return_spec_parser()
-> impl Fn(&mut PtxTokenStream) -> Result<(CallPrototypeReturnSpec, Span), PtxParseError> {
    // Return spec can be:
    // 1. (.param .type name) _ - parenthesized parameter with function ident placeholder
    // 2. (_) _ - parenthesized underscore with function ident placeholder
    // 3. .param .type name - bare parameter (no function ident)
    // 4. _ - bare underscore placeholder (no function ident)
    //
    // When return spec is parenthesized, there's an explicit function identifier `_` after it.
    // When return spec is bare (either _ or .param...), there's no explicit function ident.
    let paren_param = map(
        skip_second(
            between(lparen_p(), rparen_p(), ParameterDirective::parse()),
            underscore_placeholder(),
        ),
        |param, _| CallPrototypeReturnSpec::ParenParam(param),
    );
    let paren_underscore = map(
        skip_second(
            between(lparen_p(), rparen_p(), underscore_placeholder()),
            underscore_placeholder(),
        ),
        |_, _| CallPrototypeReturnSpec::ParenUnderscore,
    );
    let bare_param = map(
        ParameterDirective::parse(),
        |param, _| CallPrototypeReturnSpec::BareParam(param),
    );
    let bare_underscore = map(
        underscore_placeholder(),
        |_, _| CallPrototypeReturnSpec::BareUnderscore,
    );

    alt!(paren_param, paren_underscore, bare_param, bare_underscore)
}

fn underscore_placeholder() -> impl Fn(&mut PtxTokenStream) -> Result<((), Span), PtxParseError> {
    try_map(identifier_p(), |name, span| {
        if name == "_" {
            Ok(())
        } else {
            Err(PtxParseError {
                kind: ParseErrorKind::UnexpectedToken {
                    expected: vec!["identifier `_`".into()],
                    found: name,
                },
                span,
            })
        }
    })
}

fn parameter_list_parser()
-> impl Fn(&mut PtxTokenStream) -> Result<(Vec<ParameterDirective>, Span), PtxParseError> {
    map(
        between(
            lparen_p(),
            rparen_p(),
            sep_by(ParameterDirective::parse(), comma_p()),
        ),
        |params, _| params,
    )
}

fn noreturn_parser() -> impl Fn(&mut PtxTokenStream) -> Result<(bool, Span), PtxParseError> {
    map(optional(directive_exact_p("noreturn")), |flag, _| {
        flag.is_some()
    })
}

fn abi_preserve_parser()
-> impl Fn(&mut PtxTokenStream) -> Result<(Option<u32>, Span), PtxParseError> {
    map(
        optional(skip_first(directive_exact_p("abi_preserve"), u32_p())),
        |value, _span| value,
    )
}

fn abi_preserve_control_parser()
-> impl Fn(&mut PtxTokenStream) -> Result<(Option<u32>, Span), PtxParseError> {
    map(
        optional(skip_first(
            directive_exact_p("abi_preserve_control"),
            u32_p(),
        )),
        |value, _span| value,
    )
}
fn dwarf_directive()
-> impl Fn(&mut PtxTokenStream) -> Result<(StatementDirective, Span), PtxParseError> {
    mapc!(
        skip_semicolon(DwarfDirective::parse()),
        StatementDirective::Dwarf { directive }
    )
}

fn dwarf_kind_parser()
-> impl Fn(&mut PtxTokenStream) -> Result<(DwarfDirective, Span), PtxParseError> {
    let byte_values = try_map(
        seq(
            directive_exact_p("byte"),
            sep_by1(unsigned_integer_literal(), comma_p()),
        ),
        |(_, values), span| {
            let mut parsed = Vec::new();
            for (text, value_span) in values {
                let value = parse_unsigned_integer(&text, value_span, 0, u8::MAX as u128)?;
                parsed.push(value as u8);
            }
            ok!(DwarfDirective {
                kind = DwarfDirectiveKind::ByteValues(parsed)
            })
        },
    );

    let four_byte_values = try_map(
        seq(
            four_byte_keyword(),
            sep_by1(unsigned_integer_literal(), comma_p()),
        ),
        |(_, values), span| {
            let mut parsed = Vec::new();
            for (text, value_span) in values {
                let value = parse_unsigned_integer(&text, value_span, 0, u32::MAX as u128)?;
                parsed.push(value as u32);
            }
            ok!(DwarfDirective {
                kind = DwarfDirectiveKind::FourByteValues(parsed)
            })
        },
    );

    let four_byte_label = try_map(
        seq(four_byte_keyword(), Label::parse()),
        |(_, label), span| {
            ok!(DwarfDirective {
                kind = DwarfDirectiveKind::FourByteLabel(label)
            })
        },
    );

    let quad_values = try_map(
        seq(
            directive_exact_p("quad"),
            sep_by1(unsigned_integer_literal(), comma_p()),
        ),
        |(_, values), span| {
            let mut parsed = Vec::new();
            for (text, value_span) in values {
                let value = parse_unsigned_integer(&text, value_span, 0, u64::MAX as u128)?;
                parsed.push(value as u64);
            }
            ok!(DwarfDirective {
                kind = DwarfDirectiveKind::QuadValues(parsed)
            })
        },
    );

    let quad_label = try_map(
        seq(directive_exact_p("quad"), Label::parse()),
        |(_, label), span| {
            ok!(DwarfDirective {
                kind = DwarfDirectiveKind::QuadLabel(label)
            })
        },
    );

    alt!(
        byte_values,
        four_byte_label,
        quad_label,
        four_byte_values,
        quad_values
    )
}

fn pragma_directive()
-> impl Fn(&mut PtxTokenStream) -> Result<(StatementDirective, Span), PtxParseError> {
    try_map(
        skip_semicolon(seq(directive_exact_p("pragma"), string_literal_p())),
        |(_, text), span| {
            let kind = match text.trim() {
                "nounroll" => PragmaDirectiveKind::Nounroll,
                "enable_smem_spilling" => PragmaDirectiveKind::EnableSmemSpilling,
                other if other.starts_with("used_bytes_mask") => {
                    let mask = other["used_bytes_mask".len()..].trim().to_string();
                    PragmaDirectiveKind::UsedBytesMask { mask }
                }
                other if other.starts_with("frequency") => {
                    let value_str = other["frequency".len()..].trim();
                    let value = parse_u32_literal(value_str, span)?;
                    PragmaDirectiveKind::Frequency { value }
                }
                other => PragmaDirectiveKind::Raw(other.to_string()),
            };
            let directive = c!(PragmaDirective { kind });
            ok!(StatementDirective::Pragma { directive })
        },
    )
}

fn section_directive()
-> impl Fn(&mut PtxTokenStream) -> Result<(StatementDirective, Span), PtxParseError> {
    mapc!(
        SectionDirective::parse(),
        StatementDirective::Section { directive }
    )
}

fn register_statement()
-> impl Fn(&mut PtxTokenStream) -> Result<(RegisterDirective, Span), PtxParseError> {
    mapc!(
        skip_semicolon(seq(
            skip_first(directive_exact_p("reg"), DataType::parse()),
            register_targets_parser(),
        )),
        RegisterDirective { ty, registers }
    )
}

fn register_targets_parser()
-> impl Fn(&mut PtxTokenStream) -> Result<(Vec<RegisterTarget>, Span), PtxParseError> {
    map(
        sep_by1(
            seq(register_symbol(), optional(register_count())),
            comma_p(),
        ),
        |entries, _span| {
            let registers = entries
                .into_iter()
                .map(|(symbol, range)| {
                    let symbol_span = symbol.span;
                    RegisterTarget {
                        name: symbol,
                        range,
                        span: symbol_span,
                    }
                })
                .collect();
            registers
        },
    )
}

fn register_symbol() -> impl Fn(&mut PtxTokenStream) -> Result<(VariableSymbol, Span), PtxParseError>
{
    alt(
        map(register_p(), |name, span| VariableSymbol {
            val: name,
            span,
        }),
        map(identifier_p(), |val, span| VariableSymbol { val, span }),
    )
}

fn register_count() -> impl Fn(&mut PtxTokenStream) -> Result<(u32, Span), PtxParseError> {
    between(langle_p(), rangle_p(), u32_p())
}

fn location_directive()
-> impl Fn(&mut PtxTokenStream) -> Result<(LocationDirective, Span), PtxParseError> {
    mapc!(
        seq_n!(
            skip_first(directive_exact_p("loc"), u32_p()),
            u32_p(),
            u32_p(),
            pure(Option::<LocationInlinedAt>::None)
        ),
        LocationDirective {
            file_index,
            line,
            column,
            inlined_at,
        }
    )
}

fn section_name_parser() -> impl Fn(&mut PtxTokenStream) -> Result<(String, Span), PtxParseError> {
    alt(
        map(directive_p(), func!(|name| format!(".{name}"))),
        identifier_p(),
    )
}

fn section_body_parser()
-> impl Fn(&mut PtxTokenStream) -> Result<(Vec<SectionEntry>, Span), PtxParseError> {
    between(lbrace_p(), rbrace_p(), many(section_entry_parser()))
}

fn skip_optional_semicolon<T, P>(
    parser: P,
) -> impl Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>
where
    P: Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>,
{
    move |stream| {
        let (value, span) = parser(stream)?;
        let _ = optional(semicolon_p())(stream)?;
        Ok((value, span))
    }
}

fn section_entry_parser()
-> impl Fn(&mut PtxTokenStream) -> Result<(SectionEntry, Span), PtxParseError> {
    alt(
        label_entry(),
        map(
            section_directive_line(),
            func!(|line| SectionEntry::Directive(line)),
        ),
    )
}

fn label_entry() -> impl Fn(&mut PtxTokenStream) -> Result<(SectionEntry, Span), PtxParseError> {
    map(seq(Label::parse(), colon_p()), |(label, _), span| {
        SectionEntry::Label { label, span }
    })
}

fn section_directive_line()
-> impl Fn(&mut PtxTokenStream) -> Result<(StatementSectionDirectiveLine, Span), PtxParseError> {
    let b8 = try_map(
        skip_optional_semicolon(skip_first(
            directive_exact_p("b8"),
            sep_by1(signed_integer_literal(), comma_p()),
        )),
        func!(|values| {
            let mut out = Vec::new();
            for (text, value_span) in values {
                let value = parse_signed_integer(&text, value_span, -128, 255)?;
                out.push(value as i16);
            }
            ok!(StatementSectionDirectiveLine::B8 { values = out })
        }),
    );

    let b16 = try_map(
        skip_optional_semicolon(skip_first(
            directive_exact_p("b16"),
            sep_by1(signed_integer_literal(), comma_p()),
        )),
        |values, span| {
            let mut out = Vec::new();
            for (text, value_span) in values {
                let value = parse_signed_integer(&text, value_span, -32_768, 65_535)?;
                out.push(value as i32);
            }
            ok!(StatementSectionDirectiveLine::B16 { values = out })
        },
    );

    let b32 = try_map(
        skip_optional_semicolon(skip_first(directive_exact_p("b32"), b32_section_suffix())),
        |line, span| Ok(line.with_span(span)),
    );

    let b64 = try_map(
        skip_optional_semicolon(skip_first(directive_exact_p("b64"), b64_section_suffix())),
        |line, span| Ok(line.with_span(span)),
    );

    alt!(b8, b16, b32, b64)
}

fn b32_section_suffix()
-> impl Fn(&mut PtxTokenStream) -> Result<(StatementSectionDirectiveLine, Span), PtxParseError> {
    let immediate = try_map(
        sep_by1(signed_integer_literal(), comma_p()),
        |values, span| {
            let mut out = Vec::new();
            for (text, value_span) in values {
                let value =
                    parse_signed_integer(&text, value_span, i64::MIN as i128, i64::MAX as i128)?;
                out.push(value as i64);
            }
            ok!(StatementSectionDirectiveLine::B32Immediate { values = out })
        },
    );

    let label_diff = try_map(
        seq_n!(Label::parse(), minus_p(), Label::parse()),
        |(left, _, right), span| {
            ok!(StatementSectionDirectiveLine::B32LabelDiff {
                entries = (left, right)
            })
        },
    );

    let label_plus = try_map(
        seq_n!(
            Label::parse(),
            alt(map(plus_p(), |_, _| 1i32), map(minus_p(), |_, _| -1i32)),
            integer_p(),
        ),
        |(label, sign, digits), span| {
            let limit = if sign < 0 {
                (i32::MAX as u128) + 1
            } else {
                i32::MAX as u128
            };
            let magnitude = parse_unsigned_integer(&digits, span, 0, limit)? as i128;
            let value = if sign < 0 { -magnitude } else { magnitude };
            ok!(StatementSectionDirectiveLine::B32LabelPlusImm {
                entries = (label, value as i32)
            })
        },
    );

    let label_only = map(
        Label::parse(),
        |label, span| c!(StatementSectionDirectiveLine::B32Label { labels = label }),
    );

    alt!(immediate, label_diff, label_plus, label_only)
}

fn b64_section_suffix()
-> impl Fn(&mut PtxTokenStream) -> Result<(StatementSectionDirectiveLine, Span), PtxParseError> {
    let immediate = try_map(
        sep_by1(signed_integer_literal(), comma_p()),
        |values, span| {
            let mut out = Vec::new();
            for (text, value_span) in values {
                let value = parse_signed_integer(&text, value_span, i128::MIN, i128::MAX)?;
                out.push(value);
            }
            ok!(StatementSectionDirectiveLine::B64Immediate { values = out })
        },
    );

    let label_diff = try_map(
        seq_n!(Label::parse(), minus_p(), Label::parse()),
        |(left, _, right), span| {
            ok!(StatementSectionDirectiveLine::B64LabelDiff {
                entries = (left, right)
            })
        },
    );

    let label_plus = try_map(
        seq_n!(
            Label::parse(),
            alt(map(plus_p(), |_, _| 1i32), map(minus_p(), |_, _| -1i32)),
            integer_p(),
        ),
        |(label, sign, digits), span| {
            let limit = if sign < 0 {
                (i64::MAX as u128) + 1
            } else {
                i64::MAX as u128
            };
            let magnitude = parse_unsigned_integer(&digits, span, 0, limit)? as i128;
            let value = if sign < 0 { -magnitude } else { magnitude };
            ok!(StatementSectionDirectiveLine::B64LabelPlusImm {
                entries = (label, value as i64)
            })
        },
    );

    let label_only = map(
        Label::parse(),
        |label, span| c!(StatementSectionDirectiveLine::B64Label { labels = label }),
    );

    alt!(immediate, label_diff, label_plus, label_only)
}

fn signed_integer_literal()
-> impl Fn(&mut PtxTokenStream) -> Result<((String, Span), Span), PtxParseError> {
    map(
        seq(
            optional(alt(map(minus_p(), |_, _| '-'), map(plus_p(), |_, _| '+'))),
            integer_p(),
        ),
        |(sign, digits), span| {
            let mut value = String::new();
            if let Some(ch) = sign {
                if ch == '-' {
                    value.push('-');
                }
            }
            value.push_str(&digits);
            (value, span)
        },
    )
}

fn unsigned_integer_literal()
-> impl Fn(&mut PtxTokenStream) -> Result<((String, Span), Span), PtxParseError> {
    map(integer_p(), |digits, span| (digits, span))
}

fn four_byte_keyword() -> impl Fn(&mut PtxTokenStream) -> Result<((), Span), PtxParseError> {
    move |stream| {
        stream.try_with_span(|stream| {
            stream.expect(&PtxToken::Dot)?;
            let (value, value_span) = integer_p()(stream)?;
            if value != "4" {
                return Err(crate::unexpected_value!(value_span, &["4"], value));
            }
            let (name, name_span) = identifier_p()(stream)?;
            if name != "byte" {
                return Err(crate::unexpected_value!(name_span, &["byte"], name));
            }
            Ok(())
        })
    }
}

impl PtxParser for AliasFunctionDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        use crate::parser::util::{comma_p, directive_exact_p, semicolon_p, skip_first};

        try_map(
            seq_n!(
                skip_first(directive_exact_p("alias"), FunctionSymbol::parse()),
                skip_first(comma_p(), FunctionSymbol::parse()),
                semicolon_p()
            ),
            |(alias, target, _), span| ok!(AliasFunctionDirective { alias, target }),
        )
    }
}

impl PtxParser for FunctionBody {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        try_map(
            between(lbrace_p(), rbrace_p(), many(FunctionStatement::parse())),
            |statements, span| ok!(FunctionBody { statements }),
        )
    }
}

/// Parser for pre-body declarations (.reg, .local, .shared, .param) that appear
/// between the function header and the opening brace.
fn pre_body_declaration()
-> impl Fn(&mut PtxTokenStream) -> Result<(StatementDirective, Span), PtxParseError> {
    let reg_stmt = mapc!(register_statement(), StatementDirective::Reg { directive });

    let local_stmt = mapc!(
        skip_first(directive_exact_p("local"), VariableDirective::parse()),
        StatementDirective::Local { directive }
    );

    let param_stmt = mapc!(
        skip_first(directive_exact_p("param"), VariableDirective::parse()),
        StatementDirective::Param { directive }
    );

    let shared_stmt = mapc!(
        skip_first(directive_exact_p("shared"), VariableDirective::parse()),
        StatementDirective::Shared { directive }
    );

    alt!(reg_stmt, local_stmt, param_stmt, shared_stmt)
}

impl PtxParser for FuncFunctionDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        let return_spec = alt(
            map(
                between(
                    lparen_p(),
                    rparen_p(),
                    optional(ParameterDirective::parse()),
                ),
                |param, _| param,
            ),
            map(optional(ParameterDirective::parse()), |param, _| param),
        );

        let body_or_prototype = alt(
            map(FunctionBody::parse(), |body, _| Some(body)),
            map(semicolon_p(), |_, _| None),
        );

        mapc!(
            seq_n!(
                skip_first(directive_exact_p("func"), many(AttributeDirective::parse())),
                return_spec,
                FunctionSymbol::parse(),
                between(
                    lparen_p(),
                    rparen_p(),
                    sep_by(ParameterDirective::parse(), comma_p()),
                ),
                many(FuncFunctionHeaderDirective::parse()),
                many(pre_body_declaration()),
                body_or_prototype,
            ),
            FuncFunctionDirective {
                attributes,
                return_param,
                name,
                params,
                directives,
                pre_body_declarations,
                body
            }
        )
    }
}

impl PtxParser for EntryFunctionDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        // Entry functions can be either:
        // 1. Forward declarations (prototypes): .entry name();
        // 2. Definitions: .entry name() { ... }
        let body_or_prototype = alt(
            map(FunctionBody::parse(), |body, _| Some(body)),
            map(semicolon_p(), |_, _| None),
        );

        mapc!(
            seq_n!(
                skip_first(directive_exact_p("entry"), FunctionSymbol::parse()),
                between(
                    lparen_p(),
                    rparen_p(),
                    sep_by(ParameterDirective::parse(), comma_p()),
                ),
                many(EntryFunctionHeaderDirective::parse()),
                body_or_prototype,
            ),
            EntryFunctionDirective {
                name,
                params,
                directives,
                body,
            }
        )
    }
}

impl PtxParser for FuncFunctionHeaderDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        alt!(
            mapc!(
                directive_exact_p("noreturn"),
                FuncFunctionHeaderDirective::NoReturn {}
            ),
            mapc!(
                skip_first(
                    directive_exact_p("pragma"),
                    seq(sep_by1(string_literal_p(), comma_p()), semicolon_p())
                ),
                FuncFunctionHeaderDirective::Pragma { args, _ }
            ),
            mapc!(
                skip_first(directive_exact_p("abi_preserve"), u32_p()),
                FuncFunctionHeaderDirective::AbiPreserve { value }
            ),
            mapc!(
                skip_first(directive_exact_p("abi_preserve_control"), u32_p()),
                FuncFunctionHeaderDirective::AbiPreserveControl { value }
            )
        )
    }
}

impl PtxParser for EntryFunctionHeaderDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        alt!(
            mapc!(
                skip_first(directive_exact_p("maxnreg"), u32_p()),
                EntryFunctionHeaderDirective::MaxNReg { value }
            ),
            try_map(
                skip_first(directive_exact_p("maxntid"), sep_by1(u32_p(), comma_p())),
                |dim_strs, span| {
                    let dim = parse_function_dim(&dim_strs, span)?;
                    ok!(EntryFunctionHeaderDirective::MaxNTid { dim })
                }
            ),
            try_map(
                skip_first(directive_exact_p("reqntid"), sep_by1(u32_p(), comma_p())),
                |dim_strs, span| {
                    let dim = parse_function_dim(&dim_strs, span)?;
                    ok!(EntryFunctionHeaderDirective::ReqNTid { dim })
                }
            ),
            mapc!(
                skip_first(directive_exact_p("minnctapersm"), u32_p()),
                EntryFunctionHeaderDirective::MinNCtaPerSm { value }
            ),
            mapc!(
                skip_first(directive_exact_p("maxnctapersm"), u32_p()),
                EntryFunctionHeaderDirective::MaxNCtaPerSm { value }
            ),
            mapc!(
                skip_first(
                    directive_exact_p("pragma"),
                    skip_second(sep_by1(string_literal_p(), comma_p()), semicolon_p())
                ),
                EntryFunctionHeaderDirective::Pragma { args }
            )
        )
    }
}

fn parse_function_dim(dims: &[u32], span: Span) -> Result<FunctionDim, PtxParseError> {
    match dims.len() {
        1 => {
            let x = dims[0];
            Ok(FunctionDim::X { x, span })
        }
        2 => {
            let x = dims[0];
            let y = dims[1];
            Ok(FunctionDim::XY { x, y, span })
        }
        3 => {
            let x = dims[0];
            let y = dims[1];
            let z = dims[2];
            Ok(FunctionDim::XYZ { x, y, z, span })
        }
        _ => Err(PtxParseError {
            kind: ParseErrorKind::InvalidLiteral(format!(
                "expected 1-3 dimensions, got {}",
                dims.len()
            )),
            span,
        }),
    }
}
