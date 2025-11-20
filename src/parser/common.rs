use crate::{
    alt, c, err, mapc, ok,
    parser::{
        ParseErrorKind, PtxParseError, PtxParser, PtxTokenStream, Span,
        util::{
            alt, at_p, between, comma_p, directive_exact_p, directive_p, exclamation_p,
            identifier_p, lbrace_p, lbracket_p, literal_p, lparen_p, map, minus_p, optional,
            parse_index_suffix, plus_p, rbrace_p, rbracket_p, register_p, rparen_p, sep_by1, seq,
            seq3, skip_first, try_map, u64_p,
        },
    },
    seq_n, span,
    span::Spanned,
    r#type::{
        AddressBase, AddressOffset, AddressOperand, AttributeDirective, Axis, CodeLinkage,
        DataLinkage, DataType, FunctionSymbol, GeneralOperand, Immediate, Instruction, Label,
        Operand, ParamStateSpace, Predicate, PredicateRegister, RegisterOperand, Sign,
        SpecialRegister, TexHandler2, TexHandler3, TexHandler3Optional, VariableSymbol,
        VectorOperand,
    },
};

const CODE_LINKAGE_EXPECTED: [&str; 3] = [".visible", ".extern", ".weak"];

impl PtxParser for CodeLinkage {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        try_map(directive_p(), |name, span| {
            let node = match name.as_str() {
                "visible" => c!(CodeLinkage::Visible),
                "extern" => c!(CodeLinkage::Extern),
                "weak" => c!(CodeLinkage::Weak),
                _ => {
                    return err!(ParseErrorKind::UnexpectedToken {
                        expected: CODE_LINKAGE_EXPECTED
                            .iter()
                            .map(|s| s.to_string())
                            .collect(),
                        found: format!(".{name}"),
                    });
                }
            };
            Ok(node)
        })
    }
}

const DATA_LINKAGE_EXPECTED: [&str; 4] = [".visible", ".extern", ".weak", ".common"];

impl PtxParser for DataLinkage {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        try_map(directive_p(), |name, span| {
            let node = match name.as_str() {
                "visible" => c!(DataLinkage::Visible),
                "extern" => c!(DataLinkage::Extern),
                "weak" => c!(DataLinkage::Weak),
                "common" => c!(DataLinkage::Common),
                _ => {
                    return err!(ParseErrorKind::UnexpectedToken {
                        expected: DATA_LINKAGE_EXPECTED
                            .iter()
                            .map(|s| s.to_string())
                            .collect(),
                        found: format!(".{name}"),
                    });
                }
            };
            Ok(node)
        })
    }
}

const DATA_TYPE_EXPECTED: [&str; 21] = [
    ".u8",
    ".u16",
    ".u32",
    ".u64",
    ".s8",
    ".s16",
    ".s32",
    ".s64",
    ".f16",
    ".f16x2",
    ".f32",
    ".f64",
    ".b8",
    ".b16",
    ".b32",
    ".b64",
    ".b128",
    ".pred",
    ".texref",
    ".samplerref",
    ".surfref",
];

impl PtxParser for DataType {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        try_map(directive_p(), |name, span| {
            let node = match name.as_str() {
                "u8" => c!(DataType::U8),
                "u16" => c!(DataType::U16),
                "u32" => c!(DataType::U32),
                "u64" => c!(DataType::U64),
                "s8" => c!(DataType::S8),
                "s16" => c!(DataType::S16),
                "s32" => c!(DataType::S32),
                "s64" => c!(DataType::S64),
                "f16" => c!(DataType::F16),
                "f16x2" => c!(DataType::F16x2),
                "f32" => c!(DataType::F32),
                "f64" => c!(DataType::F64),
                "b8" => c!(DataType::B8),
                "b16" => c!(DataType::B16),
                "b32" => c!(DataType::B32),
                "b64" => c!(DataType::B64),
                "b128" => c!(DataType::B128),
                "pred" => c!(DataType::Pred),
                "texref" => c!(DataType::TexRef),
                "samplerref" => c!(DataType::SamplerRef),
                "surfref" => c!(DataType::SurfRef),
                _ => {
                    return err!(ParseErrorKind::UnexpectedToken {
                        expected: DATA_TYPE_EXPECTED.iter().map(|s| s.to_string()).collect(),
                        found: format!(".{name}"),
                    });
                }
            };
            Ok(node)
        })
    }
}

impl PtxParser for AttributeDirective {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        alt(
            map(directive_exact_p("managed"), |_, span| {
                c!(AttributeDirective::Managed)
            }),
            mapc!(
                skip_first(
                    directive_exact_p("unified"),
                    between(
                        lparen_p(),
                        rparen_p(),
                        seq_n!(u64_p(), skip_first(comma_p(), u64_p()))
                    ),
                ),
                AttributeDirective::Unified { uuid1, uuid2 }
            ),
        )
    }
}

impl PtxParser for Sign {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        alt(
            map(plus_p(), |_, span| c!(Sign::Positive)),
            map(minus_p(), |_, span| c!(Sign::Negative)),
        )
    }
}

impl PtxParser for RegisterOperand {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        try_map(register_p(), |mut raw, span| {
            let component = if let Some(idx) = raw.rfind('.') {
                if idx + 1 >= raw.len() {
                    return err!(ParseErrorKind::InvalidLiteral(
                        "register component missing after '.'".into(),
                    ));
                }
                let suffix = raw[idx + 1..].to_string();
                raw.truncate(idx);
                Some(suffix)
            } else {
                None
            };
            let name = raw;
            ok!(RegisterOperand { name, component })
        })
    }
}

impl PtxParser for VariableSymbol {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        map(identifier_p(), |val, span| c!(VariableSymbol { val }))
    }
}

impl PtxParser for FunctionSymbol {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        map(identifier_p(), |val, span| c!(FunctionSymbol { val }))
    }
}

impl PtxParser for Label {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        map(identifier_p(), |val, span| c!(Label { val }))
    }
}

impl PtxParser for PredicateRegister {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        try_map(register_p(), |name, span| {
            if name.starts_with("%p") {
                ok!(PredicateRegister { name })
            } else {
                err!(ParseErrorKind::InvalidLiteral(
                    "expected predicate register (%pX)".into(),
                ))
            }
        })
    }
}

impl PtxParser for Predicate {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        try_map(
            seq_n!(at_p(), optional(exclamation_p()), Operand::parse()),
            |(_, negation, operand), span| {
                let negated = negation.is_some();
                ok!(Predicate { negated, operand })
            },
        )
    }
}

impl PtxParser for Instruction {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        use crate::r#type::instruction::Inst;

        try_map(
            seq(optional(Predicate::parse()), Inst::parse()),
            |(predicate, inst), span| ok!(Instruction { predicate, inst }),
        )
    }
}

impl PtxParser for ParamStateSpace {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        alt!(
            map(directive_exact_p("const"), |_, span| {
                c!(ParamStateSpace::Const)
            }),
            map(directive_exact_p("global"), |_, span| {
                c!(ParamStateSpace::Global)
            }),
            map(directive_exact_p("local"), |_, span| {
                c!(ParamStateSpace::Local)
            }),
            map(directive_exact_p("shared"), |_, span| {
                c!(ParamStateSpace::Shared)
            }),
        )
    }
}

impl PtxParser for Operand {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        let register = mapc!(RegisterOperand::parse(), Operand::Register { operand });
        let immediate = map(
            seq(optional(Sign::parse()), Immediate::parse()),
            |(sign, mut operand), span| {
                if matches!(sign, Some(Sign::Negative { .. })) && !operand.value.starts_with('-') {
                    operand.value = format!("-{}", operand.value);
                }
                c!(Operand::Immediate { operand })
            },
        );
        let symbol_offset = map(
            seq_n!(identifier_p(), plus_p(), Immediate::parse()),
            |(symbol, _, offset), span| c!(Operand::SymbolOffset { symbol, offset }),
        );
        let symbol = mapc!(identifier_p(), Operand::Symbol { name });
        alt!(register, immediate, symbol_offset, symbol)
    }
}

impl PtxParser for VectorOperand {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        try_map(
            between(lbrace_p(), rbrace_p(), sep_by1(Operand::parse(), comma_p())),
            |operands, span| match operands.len() {
                1 => {
                    let mut iter = operands.into_iter();
                    let operand = iter.next().unwrap();
                    ok!(VectorOperand::Vector1 { operand })
                }
                2 => {
                    let mut iter = operands.into_iter();
                    let operands = [iter.next().unwrap(), iter.next().unwrap()];
                    ok!(VectorOperand::Vector2 { operands })
                }
                3 => {
                    let mut iter = operands.into_iter();
                    let operands = [
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                    ];
                    ok!(VectorOperand::Vector3 { operands })
                }
                4 => {
                    let mut iter = operands.into_iter();
                    let operands = [
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                    ];
                    ok!(VectorOperand::Vector4 { operands })
                }
                8 => {
                    let mut iter = operands.into_iter();
                    let operands = [
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                    ];
                    ok!(VectorOperand::Vector8 { operands })
                }
                _ => {
                    let span = operands.first().map(Spanned::span).unwrap_or(span!(0..0));
                    Err(PtxParseError {
                        kind: ParseErrorKind::UnexpectedToken {
                            expected: vec!["vector with 1,2,3,4, or 8 operands".into()],
                            found: format!("vector with {} operands", operands.len()),
                        },
                        span,
                    })
                }
            },
        )
    }
}

impl PtxParser for GeneralOperand {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        alt(
            mapc!(VectorOperand::parse(), GeneralOperand::Vec { operand }),
            mapc!(Operand::parse(), GeneralOperand::Single { operand }),
        )
    }
}

fn handler_len_error(ty: &str, expected: &str, found: usize) -> PtxParseError {
    PtxParseError {
        kind: ParseErrorKind::InvalidLiteral(format!("{ty} expects {expected}, found {found}")),
        span: Span::new(0, 0),
    }
}

fn tex_operands()
-> impl Fn(&mut PtxTokenStream) -> Result<(Vec<GeneralOperand>, Span), PtxParseError> {
    between(
        lbracket_p(),
        rbracket_p(),
        sep_by1(GeneralOperand::parse(), comma_p()),
    )
}

impl PtxParser for TexHandler2 {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        try_map(tex_operands(), |operands, span| {
            if operands.len() != 2 {
                return Err(handler_len_error(
                    "TexHandler2",
                    "exactly 2 operands",
                    operands.len(),
                ));
            }
            let mut iter = operands.into_iter();
            let first = iter.next().unwrap();
            let second = iter.next().unwrap();
            let operands = [first, second];
            ok!(TexHandler2 { operands })
        })
    }
}

impl PtxParser for TexHandler3 {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        try_map(tex_operands(), |operands, span| {
            if operands.len() != 3 {
                return Err(handler_len_error(
                    "TexHandler3",
                    "exactly 3 operands",
                    operands.len(),
                ));
            }
            let mut iter = operands.into_iter();
            let handle = iter.next().unwrap();
            let sampler = iter.next().unwrap();
            let coords = iter.next().unwrap();
            ok!(TexHandler3 {
                handle,
                sampler,
                coords
            })
        })
    }
}

impl PtxParser for TexHandler3Optional {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        try_map(tex_operands(), |operands, span| match operands.len() {
            2 => {
                let mut iter = operands.into_iter();
                let handle = iter.next().unwrap();
                let coords = iter.next().unwrap();
                let sampler = None;
                ok!(TexHandler3Optional {
                    handle,
                    sampler,
                    coords
                })
            }
            3 => {
                let mut iter = operands.into_iter();
                let handle = iter.next().unwrap();
                let sampler = Some(iter.next().unwrap());
                let coords = iter.next().unwrap();
                ok!(TexHandler3Optional {
                    handle,
                    sampler,
                    coords
                })
            }
            other => Err(handler_len_error(
                "TexHandler3Optional",
                "2 or 3 operands",
                other,
            )),
        })
    }
}

impl PtxParser for SpecialRegister {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        try_map(register_p(), |name, span| {
            let raw = name.strip_prefix('%').ok_or_else(|| PtxParseError {
                kind: ParseErrorKind::InvalidLiteral("expected % prefix".into()),
                span,
            })?;

            let (base, axis_suffix) = split_axis_suffix(raw).map_err(|msg| PtxParseError {
                kind: ParseErrorKind::InvalidLiteral(msg.into()),
                span,
            })?;
            let base_lower = base.to_ascii_lowercase();

            let mut node = match base_lower.as_str() {
                "aggr_smem_size" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::AggrSmemSize)
                }
                "dynamic_smem_size" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::DynamicSmemSize)
                }
                "lanemask_gt" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::LanemaskGt)
                }
                "reserved_smem_offset_begin" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::ReservedSmemOffsetBegin)
                }
                "clock" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::Clock)
                }
                "lanemask_le" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::LanemaskLe)
                }
                "reserved_smem_offset_cap" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::ReservedSmemOffsetCap)
                }
                "clock64" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::Clock64)
                }
                "globaltimer" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::Globaltimer)
                }
                "lanemask_lt" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::LanemaskLt)
                }
                "reserved_smem_offset_end" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::ReservedSmemOffsetEnd)
                }
                "globaltimer_hi" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::GlobaltimerHi)
                }
                "nclusterid" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::Nclusterid)
                }
                "smid" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::Smid)
                }
                "globaltimer_lo" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::GlobaltimerLo)
                }
                "gridid" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::Gridid)
                }
                "nsmid" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::Nsmid)
                }
                "total_smem_size" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::TotalSmemSize)
                }
                "is_explicit_cluster" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::IsExplicitCluster)
                }
                "warpid" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::Warpid)
                }
                "clusterid" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::Clusterid)
                }
                "laneid" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::Laneid)
                }
                "nwarpid" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::Nwarpid)
                }
                "warpsz" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::WARPSZ)
                }
                "lanemask_eq" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::LanemaskEq)
                }
                "current_graph_exec" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::CurrentGraphExec)
                }
                "lanemask_ge" => {
                    ensure_no_axis(axis_suffix, span)?;
                    c!(SpecialRegister::LanemaskGe)
                }
                "cluster_ctaid" => {
                    let axis = axis_with_span(axis_suffix, &span)?;
                    c!(SpecialRegister::ClusterCtaid { axis })
                }
                "cluster_ctarank" => {
                    let axis = axis_with_span(axis_suffix, &span)?;
                    c!(SpecialRegister::ClusterCtarank { axis })
                }
                "nctaid" => {
                    let axis = axis_with_span(axis_suffix, &span)?;
                    c!(SpecialRegister::Nctaid { axis })
                }
                "tid" => {
                    let axis = axis_with_span(axis_suffix, &span)?;
                    c!(SpecialRegister::Tid { axis })
                }
                "cluster_nctaid" => {
                    let axis = axis_with_span(axis_suffix, &span)?;
                    c!(SpecialRegister::ClusterNctaid { axis })
                }
                "cluster_nctarank" => {
                    let axis = axis_with_span(axis_suffix, &span)?;
                    c!(SpecialRegister::ClusterNctarank { axis })
                }
                "ntid" => {
                    let axis = axis_with_span(axis_suffix, &span)?;
                    c!(SpecialRegister::Ntid { axis })
                }
                "ctaid" => {
                    let axis = axis_with_span(axis_suffix, &span)?;
                    c!(SpecialRegister::Ctaid { axis })
                }
                base if base.starts_with("envreg") => {
                    ensure_no_axis(axis_suffix, span)?;
                    let digits = base.strip_prefix("envreg").unwrap();
                    let index = parse_index_suffix(digits, 31, "envreg", span)?;
                    c!(SpecialRegister::Envreg { index })
                }
                base if base.starts_with("pm") && base.ends_with("_64") => {
                    ensure_no_axis(axis_suffix, span)?;
                    let digits = base
                        .strip_prefix("pm")
                        .and_then(|rest| rest.strip_suffix("_64"))
                        .unwrap();
                    let index = parse_index_suffix(digits, 7, "pm64", span)?;
                    c!(SpecialRegister::Pm64 { index })
                }
                base if base.starts_with("pm") => {
                    ensure_no_axis(axis_suffix, span)?;
                    let digits = base.strip_prefix("pm").unwrap();
                    let index = parse_index_suffix(digits, 7, "pm", span)?;
                    c!(SpecialRegister::Pm { index })
                }
                base if base.starts_with("reserved_smem_offset_") => {
                    ensure_no_axis(axis_suffix, span)?;
                    let digits = base.strip_prefix("reserved_smem_offset_").unwrap();
                    let index = parse_index_suffix(digits, 1, "reserved_smem_offset", span)?;
                    c!(SpecialRegister::ReservedSmemOffset { index })
                }
                _ => {
                    return err!(ParseErrorKind::InvalidLiteral(format!(
                        "unknown special register: {name}"
                    )));
                }
            };

            node.set_span(span);
            Ok(node)
        })
    }
}

impl PtxParser for Immediate {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        mapc!(literal_p(), Immediate { value })
    }
}

impl PtxParser for AddressBase {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        alt(
            mapc!(RegisterOperand::parse(), AddressBase::Register { operand }),
            mapc!(VariableSymbol::parse(), AddressBase::Variable { symbol }),
        )
    }
}

impl PtxParser for AddressOffset {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        alt(
            mapc!(
                seq(plus_p(), RegisterOperand::parse()),
                AddressOffset::Register { _, operand }
            ),
            mapc!(
                seq(Sign::parse(), Immediate::parse()),
                AddressOffset::Immediate { sign, value }
            ),
        )
    }
}

impl PtxParser for AddressOperand {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
        alt!(
            mapc!(
                seq(
                    VariableSymbol::parse(),
                    between(lbracket_p(), rbracket_p(), Immediate::parse()),
                ),
                AddressOperand::Array { base, index }
            ),
            mapc!(
                between(
                    lbracket_p(),
                    rbracket_p(),
                    seq(AddressBase::parse(), optional(AddressOffset::parse())),
                ),
                AddressOperand::Offset { base, offset }
            ),
            map(
                between(
                    lbracket_p(),
                    rbracket_p(),
                    seq(optional(Sign::parse()), Immediate::parse()),
                ),
                |(sign, mut addr), span| {
                    if matches!(sign, Some(Sign::Negative { .. })) && !addr.value.starts_with('-') {
                        addr.value = format!("-{}", addr.value);
                    }
                    c!(AddressOperand::ImmediateAddress { addr })
                },
            ),
        )
    }
}

fn split_axis_suffix(raw: &str) -> Result<(&str, Option<char>), &'static str> {
    if let Some(idx) = raw.rfind('.') {
        let (base, suffix) = raw.split_at(idx);
        if suffix.len() != 2 {
            return Err("invalid axis suffix");
        }
        let axis_char = suffix.chars().nth(1).unwrap();
        Ok((base, Some(axis_char)))
    } else {
        Ok((raw, None))
    }
}

fn axis_from_suffix(axis: Option<char>, span: Span) -> Result<Axis, PtxParseError> {
    match axis.map(|c| c.to_ascii_lowercase()) {
        None => Ok(Axis::None { span }),
        Some('x') => Ok(Axis::X { span }),
        Some('y') => Ok(Axis::Y { span }),
        Some('z') => Ok(Axis::Z { span }),
        Some(other) => Err(PtxParseError {
            kind: ParseErrorKind::InvalidLiteral(format!(
                "invalid axis '.{}' for special register",
                other
            )),
            span,
        }),
    }
}

fn axis_with_span(axis: Option<char>, span: &Span) -> Result<Axis, PtxParseError> {
    axis_from_suffix(axis, *span)
}

fn ensure_no_axis(axis: Option<char>, span: Span) -> Result<(), PtxParseError> {
    if let Some(ch) = axis {
        Err(PtxParseError {
            kind: ParseErrorKind::InvalidLiteral(format!(
                "register does not accept axis '.{}'",
                ch
            )),
            span,
        })
    } else {
        Ok(())
    }
}
