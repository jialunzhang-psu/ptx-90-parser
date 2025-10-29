use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::{
            Immediate, PredicateRegister, RegisterOperand, SpecialRegister as CoreSpecialRegister,
            VariableSymbol,
        },
        instruction::mov::{
            AddressType as MovAddressType, DataType as MovDataType, Destination as MovDestination,
            FunctionAddress as MovFunctionAddress, Mov, Register as MovRegister,
            RegisterSource as MovRegisterSource, SpecialRegister as MovSpecialRegister,
            SpecialRegisterSource as MovSpecialRegisterSource, Variable as MovVariable,
            VariableWithImmediate as MovVariableWithImmediate,
        },
    },
};

fn is_numeric_token(token: &PtxToken) -> bool {
    matches!(
        token,
        PtxToken::DecimalInteger(_)
            | PtxToken::HexInteger(_)
            | PtxToken::BinaryInteger(_)
            | PtxToken::OctalInteger(_)
            | PtxToken::FloatExponent(_)
            | PtxToken::Float(_)
            | PtxToken::HexFloat(_)
    )
}

fn parse_destination(
    stream: &mut PtxTokenStream,
    data_type: MovDataType,
) -> Result<MovDestination, PtxParseError> {
    if data_type == MovDataType::Pred {
        Ok(MovDestination::Predicate(PredicateRegister::parse(stream)?))
    } else {
        Ok(MovDestination::Register(RegisterOperand::parse(stream)?))
    }
}

fn expect_register_destination(
    destination: MovDestination,
) -> Result<RegisterOperand, PtxParseError> {
    match destination {
        MovDestination::Register(dest) => Ok(dest),
        MovDestination::Predicate(_) => Err(PtxParseError {
            kind: crate::parser::ParseErrorKind::UnexpectedToken {
                expected: vec!["register destination".into()],
                found: "predicate destination".into(),
            },
            span: 0..0,
        }),
    }
}

impl PtxParser for MovDataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "pred" => Ok(MovDataType::Pred),
            "b16" => Ok(MovDataType::B16),
            "b32" => Ok(MovDataType::B32),
            "b64" => Ok(MovDataType::B64),
            "u16" => Ok(MovDataType::U16),
            "u32" => Ok(MovDataType::U32),
            "u64" => Ok(MovDataType::U64),
            "s16" => Ok(MovDataType::S16),
            "s32" => Ok(MovDataType::S32),
            "s64" => Ok(MovDataType::S64),
            "f32" => Ok(MovDataType::F32),
            "f64" => Ok(MovDataType::F64),
            other => Err(unexpected_value(
                span,
                &[
                    ".pred", ".b16", ".b32", ".b64", ".u16", ".u32", ".u64", ".s16", ".s32",
                    ".s64", ".f32", ".f64",
                ],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for MovAddressType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "u32" => Ok(MovAddressType::U32),
            "u64" => Ok(MovAddressType::U64),
            other => Err(unexpected_value(
                span,
                &[".u32", ".u64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Mov {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "mov" {
            return Err(unexpected_value(span, &["mov"], opcode));
        }

        let type_position = stream.position();
        let data_type = MovDataType::parse(stream)?;
        let destination = parse_destination(stream, data_type)?;
        stream.expect(&PtxToken::Comma)?;

        if stream.check(|token| matches!(token, PtxToken::Register(_) | PtxToken::LBrace)) {
            let saved = stream.position();
            if let Ok(source) = CoreSpecialRegister::parse(stream) {
                stream.expect(&PtxToken::Semicolon)?;
                return Ok(Mov::SpecialRegister(MovSpecialRegister {
                    data_type,
                    destination: destination.clone(),
                    source: MovSpecialRegisterSource::Register(source),
                }));
            } else {
                stream.set_position(saved);
            }

            if data_type == MovDataType::Pred {
                let source = PredicateRegister::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;
                return Ok(Mov::Register(MovRegister {
                    data_type,
                    destination,
                    source: MovRegisterSource::Predicate(source),
                }));
            }

            let source = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;
            return Ok(Mov::Register(MovRegister {
                data_type,
                destination,
                source: MovRegisterSource::Register(source),
            }));
        }

        if stream.check(is_numeric_token) {
            if data_type == MovDataType::Pred {
                let (token, span) = stream.peek()?;
                return Err(unexpected_value(
                    span.clone(),
                    &["predicate register"],
                    format!("{token:?}"),
                ));
            }

            let immediate = Immediate::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;
            return Ok(Mov::Register(MovRegister {
                data_type,
                destination,
                source: MovRegisterSource::Immediate(immediate),
            }));
        }

        if stream.check(|token| matches!(token, PtxToken::Identifier(_))) {
            let destination_register = expect_register_destination(destination)?;

            let (name, _) = stream.expect_identifier()?;
            let variable = VariableSymbol(name);

            if stream
                .consume_if(|token| matches!(token, PtxToken::Plus))
                .is_some()
            {
                let immediate = Immediate::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;
                return Ok(Mov::VariableWithImmediate(MovVariableWithImmediate {
                    data_type,
                    destination: destination_register,
                    variable,
                    immediate,
                }));
            }

            stream.expect(&PtxToken::Semicolon)?;
            return Ok(Mov::Variable(MovVariable {
                data_type,
                destination: destination_register,
                variable,
            }));
        }

        // Attempt to parse address-sized modifiers for function/kernel address forms
        stream.set_position(type_position);
        let address_type = MovAddressType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;

        let (name, _) = stream.expect_identifier()?;
        stream.expect(&PtxToken::Semicolon)?;

        // Without additional semantic context we default to FunctionAddress.
        let function = crate::r#type::common::FunctionSymbol(name);
        Ok(Mov::FunctionAddress(MovFunctionAddress {
            data_type: address_type,
            destination,
            function,
        }))
    }
}
