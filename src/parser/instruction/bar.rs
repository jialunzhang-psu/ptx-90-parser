use crate::{
    lexer::PtxToken,
    parser::{
        PtxParseError, PtxParser, PtxTokenStream, consume_directive_if, expect_directive_value,
        unexpected_value,
    },
    r#type::{
        common::{Operand, PredicateRegister, RegisterOperand},
        instruction::bar::{
            Bar, BarArrive, BarReductionLogical, BarReductionPopc, BarSync, BarrierArrive,
            BarrierReductionLogical, BarrierReductionPopc, BarrierSync, DataType, LogicalOperation,
            PredicateInput, Scope,
        },
    },
};

impl PtxParser for Scope {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if consume_directive_if(stream, "cta") {
            Ok(Scope::Cta)
        } else {
            Ok(Scope::None)
        }
    }
}

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_directive_value(stream, "u32")?;
        Ok(DataType::U32)
    }
}

impl PtxParser for LogicalOperation {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "and" => Ok(LogicalOperation::And),
            "or" => Ok(LogicalOperation::Or),
            other => Err(unexpected_value(
                span,
                &[".and", ".or"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for PredicateInput {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let negated = stream
            .consume_if(|token| matches!(token, PtxToken::Exclaim))
            .is_some();
        let predicate = PredicateRegister::parse(stream)?;
        Ok(PredicateInput { negated, predicate })
    }
}

impl PtxParser for Bar {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        match opcode.as_str() {
            "barrier" => parse_barrier_instruction(stream),
            "bar" => parse_bar_instruction(stream),
            other => Err(unexpected_value(span, &["barrier", "bar"], other)),
        }
    }
}

fn parse_barrier_instruction(stream: &mut PtxTokenStream) -> Result<Bar, PtxParseError> {
    let scope = Scope::parse(stream)?;
    let (modifier, span) = stream.expect_directive()?;
    match modifier.as_str() {
        "sync" => parse_barrier_sync(stream, scope),
        "arrive" => parse_barrier_arrive(stream, scope),
        "red" => parse_barrier_reduction(stream, scope),
        other => Err(unexpected_value(
            span,
            &[".sync", ".arrive", ".red"],
            format!(".{other}"),
        )),
    }
}

fn parse_bar_instruction(stream: &mut PtxTokenStream) -> Result<Bar, PtxParseError> {
    let scope = Scope::parse(stream)?;
    let (modifier, span) = stream.expect_directive()?;
    match modifier.as_str() {
        "sync" => parse_bar_sync(stream, scope),
        "arrive" => parse_bar_arrive(stream, scope),
        "red" => parse_bar_reduction(stream, scope),
        other => Err(unexpected_value(
            span,
            &[".sync", ".arrive", ".red"],
            format!(".{other}"),
        )),
    }
}

fn parse_barrier_sync(stream: &mut PtxTokenStream, scope: Scope) -> Result<Bar, PtxParseError> {
    let aligned = consume_directive_if(stream, "aligned");
    let barrier = Operand::parse(stream)?;
    let expected_count = if stream
        .consume_if(|token| matches!(token, PtxToken::Comma))
        .is_some()
    {
        Some(Operand::parse(stream)?)
    } else {
        None
    };
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Bar::BarrierSync(BarrierSync {
        scope,
        aligned,
        barrier,
        expected_count,
    }))
}

fn parse_barrier_arrive(stream: &mut PtxTokenStream, scope: Scope) -> Result<Bar, PtxParseError> {
    let aligned = consume_directive_if(stream, "aligned");
    let barrier = Operand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let expected_count = Operand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Bar::BarrierArrive(BarrierArrive {
        scope,
        aligned,
        barrier,
        expected_count,
    }))
}

fn parse_barrier_reduction(
    stream: &mut PtxTokenStream,
    scope: Scope,
) -> Result<Bar, PtxParseError> {
    let (kind, span) = stream.expect_directive()?;
    match kind.as_str() {
        "popc" => parse_barrier_reduction_popc(stream, scope),
        "and" | "or" => parse_barrier_reduction_logical(stream, scope, kind),
        other => Err(unexpected_value(
            span,
            &[".popc", ".and", ".or"],
            format!(".{other}"),
        )),
    }
}

fn parse_barrier_reduction_popc(
    stream: &mut PtxTokenStream,
    scope: Scope,
) -> Result<Bar, PtxParseError> {
    let aligned = consume_directive_if(stream, "aligned");
    let data_type = DataType::parse(stream)?;
    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let barrier = Operand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let (expected_count, predicate) = parse_optional_expected_count_and_predicate(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Bar::BarrierReductionPopc(BarrierReductionPopc {
        scope,
        aligned,
        data_type,
        destination,
        barrier,
        expected_count,
        predicate,
    }))
}

fn parse_barrier_reduction_logical(
    stream: &mut PtxTokenStream,
    scope: Scope,
    operation_directive: String,
) -> Result<Bar, PtxParseError> {
    let operation = match operation_directive.as_str() {
        "and" => LogicalOperation::And,
        "or" => LogicalOperation::Or,
        _ => unreachable!(),
    };

    let aligned = consume_directive_if(stream, "aligned");
    expect_directive_value(stream, "pred")?;
    let destination = PredicateRegister::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let barrier = Operand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let (expected_count, predicate) = parse_optional_expected_count_and_predicate(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Bar::BarrierReductionLogical(BarrierReductionLogical {
        scope,
        aligned,
        destination,
        barrier,
        expected_count,
        predicate,
        operation,
    }))
}

fn parse_bar_sync(stream: &mut PtxTokenStream, scope: Scope) -> Result<Bar, PtxParseError> {
    let barrier = Operand::parse(stream)?;
    let expected_count = if stream
        .consume_if(|token| matches!(token, PtxToken::Comma))
        .is_some()
    {
        Some(Operand::parse(stream)?)
    } else {
        None
    };
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Bar::BarSync(BarSync {
        scope,
        barrier,
        expected_count,
    }))
}

fn parse_bar_arrive(stream: &mut PtxTokenStream, scope: Scope) -> Result<Bar, PtxParseError> {
    let barrier = Operand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let expected_count = Operand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Bar::BarArrive(BarArrive {
        scope,
        barrier,
        expected_count,
    }))
}

fn parse_bar_reduction(stream: &mut PtxTokenStream, scope: Scope) -> Result<Bar, PtxParseError> {
    let (kind, span) = stream.expect_directive()?;
    match kind.as_str() {
        "popc" => parse_bar_reduction_popc(stream, scope),
        "and" | "or" => parse_bar_reduction_logical(stream, scope, kind),
        other => Err(unexpected_value(
            span,
            &[".popc", ".and", ".or"],
            format!(".{other}"),
        )),
    }
}

fn parse_bar_reduction_popc(
    stream: &mut PtxTokenStream,
    scope: Scope,
) -> Result<Bar, PtxParseError> {
    let data_type = DataType::parse(stream)?;
    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let barrier = Operand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let (expected_count, predicate) = parse_optional_expected_count_and_predicate(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Bar::BarReductionPopc(BarReductionPopc {
        scope,
        data_type,
        destination,
        barrier,
        expected_count,
        predicate,
    }))
}

fn parse_bar_reduction_logical(
    stream: &mut PtxTokenStream,
    scope: Scope,
    operation_directive: String,
) -> Result<Bar, PtxParseError> {
    let operation = match operation_directive.as_str() {
        "and" => LogicalOperation::And,
        "or" => LogicalOperation::Or,
        _ => unreachable!(),
    };

    expect_directive_value(stream, "pred")?;
    let destination = PredicateRegister::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let barrier = Operand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let (expected_count, predicate) = parse_optional_expected_count_and_predicate(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Bar::BarReductionLogical(BarReductionLogical {
        scope,
        destination,
        barrier,
        expected_count,
        predicate,
        operation,
    }))
}

fn parse_optional_expected_count_and_predicate(
    stream: &mut PtxTokenStream,
) -> Result<(Option<Operand>, PredicateInput), PtxParseError> {
    let predicate_next = next_is_predicate_input(stream)?;
    if predicate_next {
        let predicate = PredicateInput::parse(stream)?;
        Ok((None, predicate))
    } else {
        let expected_count = Operand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let predicate = PredicateInput::parse(stream)?;
        Ok((Some(expected_count), predicate))
    }
}

fn next_is_predicate_input(stream: &mut PtxTokenStream) -> Result<bool, PtxParseError> {
    let (token, _) = stream.peek()?;
    match token {
        PtxToken::Exclaim => Ok(true),
        PtxToken::Register(name) => Ok(name.starts_with("%p")),
        _ => Ok(false),
    }
}
