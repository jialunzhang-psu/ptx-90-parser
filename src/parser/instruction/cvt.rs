use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::cvt::*},
};

use crate::parser::common::parse_register_name;

fn parse_optional_rounding(stream: &mut PtxTokenStream) -> Result<Option<Rounding>, PtxParseError> {
    match peek_directive(stream)? {
        Some((directive, _))
            if matches!(
                directive.as_str(),
                "rni" | "rzi" | "rmi" | "rpi" | "rn" | "rz" | "rm" | "rp"
            ) =>
        {
            Rounding::parse(stream).map(Some)
        }
        _ => Ok(None),
    }
}

fn parse_optional_flags(
    stream: &mut PtxTokenStream,
    flags: &mut [(&str, &mut bool)],
) -> Result<(), PtxParseError> {
    loop {
        let mut consumed = false;
        for (name, value) in flags.iter_mut() {
            if !**value && consume_directive_if(stream, name) {
                **value = true;
                consumed = true;
            }
        }
        if !consumed {
            break;
        }
    }
    Ok(())
}

fn parse_register_vector4(
    stream: &mut PtxTokenStream,
) -> Result<[RegisterOperand; 4], PtxParseError> {
    let (_, start_span) = stream.expect(&PtxToken::LBrace)?;
    let mut registers = Vec::with_capacity(4);

    loop {
        let (name, _) = parse_register_name(stream)?;
        registers.push(RegisterOperand::Single(name));
        if stream
            .consume_if(|token| matches!(token, PtxToken::Comma))
            .is_some()
        {
            continue;
        }
        break;
    }

    let (_, end_span) = stream.expect(&PtxToken::RBrace)?;

    if registers.len() != 4 {
        let mut span = start_span.clone();
        span.end = end_span.end;
        return Err(invalid_literal(
            span,
            format!(
                "expected register vector of length 4, found {}",
                registers.len()
            ),
        ));
    }

    let mut iter = registers.into_iter();
    Ok([
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ])
}

impl PtxParser for IntegerRounding {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "rni" => Ok(IntegerRounding::Rni),
            "rzi" => Ok(IntegerRounding::Rzi),
            "rmi" => Ok(IntegerRounding::Rmi),
            "rpi" => Ok(IntegerRounding::Rpi),
            other => Err(unexpected_value(
                span,
                &[".rni", ".rzi", ".rmi", ".rpi"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for FloatRounding {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "rn" => Ok(FloatRounding::Rn),
            "rz" => Ok(FloatRounding::Rz),
            "rm" => Ok(FloatRounding::Rm),
            "rp" => Ok(FloatRounding::Rp),
            other => Err(unexpected_value(
                span,
                &[".rn", ".rz", ".rm", ".rp"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Rounding {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        match peek_directive(stream)? {
            Some((directive, _)) if matches!(directive.as_str(), "rni" | "rzi" | "rmi" | "rpi") => {
                IntegerRounding::parse(stream).map(Rounding::Integer)
            }
            Some((directive, _)) if matches!(directive.as_str(), "rn" | "rz" | "rm" | "rp") => {
                FloatRounding::parse(stream).map(Rounding::Float)
            }
            _ => {
                let (_, span) = stream.expect_directive()?;
                Err(unexpected_value(
                    span,
                    &["rounding directive"],
                    "invalid rounding".to_string(),
                ))
            }
        }
    }
}

impl PtxParser for ScalarType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "u8" => Ok(ScalarType::U8),
            "u16" => Ok(ScalarType::U16),
            "u32" => Ok(ScalarType::U32),
            "u64" => Ok(ScalarType::U64),
            "s8" => Ok(ScalarType::S8),
            "s16" => Ok(ScalarType::S16),
            "s32" => Ok(ScalarType::S32),
            "s64" => Ok(ScalarType::S64),
            "bf16" => Ok(ScalarType::Bf16),
            "f16" => Ok(ScalarType::F16),
            "f32" => Ok(ScalarType::F32),
            "f64" => Ok(ScalarType::F64),
            other => Err(unexpected_value(
                span,
                &[
                    ".u8", ".u16", ".u32", ".u64", ".s8", ".s16", ".s32", ".s64", ".bf16", ".f16",
                    ".f32", ".f64",
                ],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Basic {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let rounding = parse_optional_rounding(stream)?;

        let mut flush_to_zero = false;
        let mut saturate = false;
        parse_optional_flags(
            stream,
            &mut [("ftz", &mut flush_to_zero), ("sat", &mut saturate)],
        )?;

        let destination_type = ScalarType::parse(stream)?;
        let source_type = ScalarType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Basic {
            rounding,
            flush_to_zero,
            saturate,
            destination_type,
            source_type,
            destination,
            source,
        })
    }
}

impl PtxParser for Frnd2Rounding {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "rn" => Ok(Frnd2Rounding::Rn),
            "rz" => Ok(Frnd2Rounding::Rz),
            other => Err(unexpected_value(span, &[".rn", ".rz"], format!(".{other}"))),
        }
    }
}

impl PtxParser for Frnd2Kind {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "f16" => {
                expect_directive_value(stream, "f32")?;
                Ok(Frnd2Kind::F16FromF32)
            }
            "f16x2" => {
                expect_directive_value(stream, "f32")?;
                Ok(Frnd2Kind::F16x2FromF32)
            }
            "bf16" => {
                expect_directive_value(stream, "f32")?;
                Ok(Frnd2Kind::Bf16FromF32)
            }
            "bf16x2" => {
                expect_directive_value(stream, "f32")?;
                Ok(Frnd2Kind::Bf16x2FromF32)
            }
            "tf32" => {
                expect_directive_value(stream, "f32")?;
                Ok(Frnd2Kind::Tf32FromF32)
            }
            other => Err(unexpected_value(
                span,
                &[".f16", ".f16x2", ".bf16", ".bf16x2", ".tf32"],
                format!(".{other}"),
            )),
        }
    }
}

fn frnd2_kind_requires_second_source(kind: &Frnd2Kind) -> bool {
    matches!(kind, Frnd2Kind::F16x2FromF32 | Frnd2Kind::Bf16x2FromF32)
}

impl PtxParser for Frnd2 {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_directive_value(stream, "frnd2")?;
        let rounding = Frnd2Rounding::parse(stream)?;
        let mut relu = false;
        let mut satfinite = false;
        parse_optional_flags(
            stream,
            &mut [("relu", &mut relu), ("satfinite", &mut satfinite)],
        )?;

        let kind = Frnd2Kind::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        let mut b = None;
        if frnd2_kind_requires_second_source(&kind) {
            stream.expect(&PtxToken::Comma)?;
            b = Some(RegisterOperand::parse(stream)?);
        }
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Frnd2 {
            rounding,
            relu,
            satfinite,
            kind,
            destination,
            a,
            b,
        })
    }
}

impl PtxParser for F8x2Type {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "e4m3x2" => Ok(F8x2Type::E4m3x2),
            "e5m2x2" => Ok(F8x2Type::E5m2x2),
            other => Err(unexpected_value(
                span,
                &[".e4m3x2", ".e5m2x2"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for F4x2Type {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "e2m1x2" => Ok(F4x2Type::E2m1x2),
            other => Err(unexpected_value(span, &[".e2m1x2"], format!(".{other}"))),
        }
    }
}

impl PtxParser for F6x2Type {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "e2m3x2" => Ok(F6x2Type::E2m3x2),
            "e3m2x2" => Ok(F6x2Type::E3m2x2),
            other => Err(unexpected_value(
                span,
                &[".e2m3x2", ".e3m2x2"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for F8x4Type {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "e4m3x4" => Ok(F8x4Type::E4m3x4),
            "e5m2x4" => Ok(F8x4Type::E5m2x4),
            other => Err(unexpected_value(
                span,
                &[".e4m3x4", ".e5m2x4"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for F4x4Type {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "e2m1x4" => Ok(F4x4Type::E2m1x4),
            other => Err(unexpected_value(span, &[".e2m1x4"], format!(".{other}"))),
        }
    }
}

impl PtxParser for F6x4Type {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "e2m3x4" => Ok(F6x4Type::E2m3x4),
            "e3m2x4" => Ok(F6x4Type::E3m2x4),
            other => Err(unexpected_value(
                span,
                &[".e2m3x4", ".e3m2x4"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Rs {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_directive_value(stream, "rs")?;
        let mut relu = false;
        let mut satfinite = false;
        parse_optional_flags(
            stream,
            &mut [("relu", &mut relu), ("satfinite", &mut satfinite)],
        )?;

        let (kind_directive, span) = stream.expect_directive()?;
        match kind_directive.as_str() {
            "f16x2" => {
                expect_directive_value(stream, "f32")?;
                let destination = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let a = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let b = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let rbits = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;
                Ok(Rs {
                    relu,
                    satfinite,
                    kind: RsKind::F16x2FromF32 { a, b },
                    destination,
                    rbits,
                })
            }
            "bf16x2" => {
                expect_directive_value(stream, "f32")?;
                let destination = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let a = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let b = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let rbits = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;
                Ok(Rs {
                    relu,
                    satfinite,
                    kind: RsKind::Bf16x2FromF32 { a, b },
                    destination,
                    rbits,
                })
            }
            "f8x4type" => {
                let data_type = F8x4Type::parse(stream)?;
                expect_directive_value(stream, "f32")?;
                let destination = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let [a, b, e, f] = parse_register_vector4(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let rbits = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;
                Ok(Rs {
                    relu,
                    satfinite,
                    kind: RsKind::F8x4FromF32 {
                        data_type,
                        a,
                        b,
                        e,
                        f,
                    },
                    destination,
                    rbits,
                })
            }
            "f4x4type" => {
                let data_type = F4x4Type::parse(stream)?;
                expect_directive_value(stream, "f32")?;
                let destination = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let [a, b, e, f] = parse_register_vector4(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let rbits = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;
                Ok(Rs {
                    relu,
                    satfinite,
                    kind: RsKind::F4x4FromF32 {
                        data_type,
                        a,
                        b,
                        e,
                        f,
                    },
                    destination,
                    rbits,
                })
            }
            "f6x4type" => {
                let data_type = F6x4Type::parse(stream)?;
                expect_directive_value(stream, "f32")?;
                let destination = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let [a, b, e, f] = parse_register_vector4(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let rbits = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;
                Ok(Rs {
                    relu,
                    satfinite,
                    kind: RsKind::F6x4FromF32 {
                        data_type,
                        a,
                        b,
                        e,
                        f,
                    },
                    destination,
                    rbits,
                })
            }
            other => Err(unexpected_value(
                span,
                &[".f16x2", ".bf16x2", ".f8x4type", ".f4x4type", ".f6x4type"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Rna {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_directive_value(stream, "rna")?;
        let mut satfinite = false;
        parse_optional_flags(stream, &mut [("satfinite", &mut satfinite)])?;
        expect_directive_value(stream, "tf32")?;
        expect_directive_value(stream, "f32")?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;
        Ok(Rna {
            satfinite,
            destination,
            source,
        })
    }
}

fn is_rn_special(stream: &mut PtxTokenStream) -> Result<bool, PtxParseError> {
    let checkpoint = stream.position();
    if expect_directive_value(stream, "rn").is_err() {
        return Ok(false);
    }

    let mut satfinite = false;
    let mut relu = false;
    parse_optional_flags(
        stream,
        &mut [("satfinite", &mut satfinite), ("relu", &mut relu)],
    )?;

    let result = matches!(
        peek_directive(stream)?,
        Some((directive, _))
            if matches!(
                directive.as_str(),
                "e4m3x2"
                    | "e5m2x2"
                    | "f16x2"
                    | "e2m1x2"
                    | "e2m3x2"
                    | "e3m2x2"
                    | "bf16x2"
            )
    );

    stream.set_position(checkpoint);
    Ok(result)
}

impl PtxParser for Rn {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_directive_value(stream, "rn")?;
        let mut satfinite = false;
        let mut relu = false;
        parse_optional_flags(
            stream,
            &mut [("satfinite", &mut satfinite), ("relu", &mut relu)],
        )?;

        let (directive, span) = stream.expect_directive()?;
        let kind = match directive.as_str() {
            "e4m3x2" | "e5m2x2" => {
                let data_type = if directive == "e4m3x2" {
                    F8x2Type::E4m3x2
                } else {
                    F8x2Type::E5m2x2
                };
                let (source_type, source_span) = stream.expect_directive()?;
                match source_type.as_str() {
                    "f32" => {
                        let destination = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let a = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let b = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Semicolon)?;
                        return Ok(Rn {
                            satfinite,
                            relu,
                            kind: RnKind::F8x2FromF32 {
                                data_type,
                                destination,
                                a,
                                b,
                            },
                        });
                    }
                    "f16x2" => {
                        let destination = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let a = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Semicolon)?;
                        return Ok(Rn {
                            satfinite,
                            relu,
                            kind: RnKind::F8x2FromF16x2 {
                                data_type,
                                destination,
                                a,
                            },
                        });
                    }
                    other => {
                        return Err(unexpected_value(
                            source_span,
                            &[".f32", ".f16x2"],
                            format!(".{other}"),
                        ));
                    }
                }
            }
            "f16x2" => {
                let (next, next_span) = stream.expect_directive()?;
                match next.as_str() {
                    "e4m3x2" | "e5m2x2" => {
                        let data_type = if next == "e4m3x2" {
                            F8x2Type::E4m3x2
                        } else {
                            F8x2Type::E5m2x2
                        };
                        let destination = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let a = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Semicolon)?;
                        RnKind::F16x2FromF8x2 {
                            data_type,
                            destination,
                            a,
                        }
                    }
                    "e2m1x2" => {
                        let destination = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let a = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Semicolon)?;
                        RnKind::F16x2FromF4x2 {
                            data_type: F4x2Type::E2m1x2,
                            destination,
                            a,
                        }
                    }
                    "e2m3x2" | "e3m2x2" => {
                        let data_type = if next == "e2m3x2" {
                            F6x2Type::E2m3x2
                        } else {
                            F6x2Type::E3m2x2
                        };
                        let destination = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let a = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Semicolon)?;
                        RnKind::F16x2FromF6x2 {
                            data_type,
                            destination,
                            a,
                        }
                    }
                    other => {
                        return Err(unexpected_value(
                            next_span,
                            &[".e4m3x2", ".e5m2x2", ".e2m1x2", ".e2m3x2", ".e3m2x2"],
                            format!(".{other}"),
                        ));
                    }
                }
            }
            "e2m1x2" => {
                let data_type = F4x2Type::E2m1x2;
                expect_directive_value(stream, "f32")?;
                let destination = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let a = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let b = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;
                RnKind::F4x2FromF32 {
                    data_type,
                    destination,
                    a,
                    b,
                }
            }
            "e2m3x2" | "e3m2x2" => {
                let data_type = if directive == "e2m3x2" {
                    F6x2Type::E2m3x2
                } else {
                    F6x2Type::E3m2x2
                };
                expect_directive_value(stream, "f32")?;
                let destination = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let a = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let b = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;
                RnKind::F6x2FromF32 {
                    data_type,
                    destination,
                    a,
                    b,
                }
            }
            "bf16x2" => {
                expect_directive_value(stream, "ue8m0x2")?;
                let destination = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let a = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;
                RnKind::Bf16x2FromUe8m0x2 { destination, a }
            }
            other => {
                return Err(unexpected_value(
                    span,
                    &[
                        ".e4m3x2", ".e5m2x2", ".f16x2", ".e2m1x2", ".e2m3x2", ".e3m2x2", ".bf16x2",
                    ],
                    format!(".{other}"),
                ));
            }
        };

        Ok(Rn {
            satfinite,
            relu,
            kind,
        })
    }
}

impl PtxParser for Frnd3Rounding {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "rz" => Ok(Frnd3Rounding::Rz),
            "rp" => Ok(Frnd3Rounding::Rp),
            other => Err(unexpected_value(span, &[".rz", ".rp"], format!(".{other}"))),
        }
    }
}

impl PtxParser for Frnd3Kind {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "ue8m0x2" => {
                let (source, source_span) = stream.expect_directive()?;
                match source.as_str() {
                    "f32" => Ok(Frnd3Kind::Ue8m0x2FromF32),
                    "bf16x2" => Ok(Frnd3Kind::Ue8m0x2FromBf16x2),
                    other => Err(unexpected_value(
                        source_span,
                        &[".f32", ".bf16x2"],
                        format!(".{other}"),
                    )),
                }
            }
            other => Err(unexpected_value(span, &[".ue8m0x2"], format!(".{other}"))),
        }
    }
}

fn frnd3_kind_requires_second_source(kind: &Frnd3Kind) -> bool {
    matches!(kind, Frnd3Kind::Ue8m0x2FromF32)
}

impl PtxParser for Frnd3 {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_directive_value(stream, "frnd3")?;
        let rounding = Frnd3Rounding::parse(stream)?;
        let mut satfinite = false;
        parse_optional_flags(stream, &mut [("satfinite", &mut satfinite)])?;

        let kind = Frnd3Kind::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        let mut b = None;
        if frnd3_kind_requires_second_source(&kind) {
            stream.expect(&PtxToken::Comma)?;
            b = Some(RegisterOperand::parse(stream)?);
        }
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Frnd3 {
            rounding,
            satfinite,
            kind,
            destination,
            a,
            b,
        })
    }
}

impl PtxParser for Cvt {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "cvt")?;
        let variant = match peek_directive(stream)? {
            Some((directive, _)) if directive == "frnd2" => Cvt::Frnd2(Frnd2::parse(stream)?),
            Some((directive, _)) if directive == "rs" => Cvt::Rs(Rs::parse(stream)?),
            Some((directive, _)) if directive == "rna" => Cvt::Rna(Rna::parse(stream)?),
            Some((directive, _)) if directive == "frnd3" => Cvt::Frnd3(Frnd3::parse(stream)?),
            Some((directive, _)) if directive == "rn" && is_rn_special(stream)? => {
                Cvt::Rn(Rn::parse(stream)?)
            }
            _ => Cvt::Basic(Basic::parse(stream)?),
        };
        Ok(variant)
    }
}
