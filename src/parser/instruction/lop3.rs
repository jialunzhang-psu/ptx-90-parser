use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::lop3::*},
};

impl PtxParser for crate::r#type::instruction::lop3::BoolOp {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;

        match modifier.as_str() {
            "or" => Ok(BoolOp::Or),
            "and" => Ok(BoolOp::And),
            other => Err(unexpected_value(
                span,
                &[".or", ".and"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for crate::r#type::instruction::lop3::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;

        match modifier.as_str() {
            "b32" => Ok(crate::r#type::instruction::lop3::DataType::B32),
            other => Err(unexpected_value(span, &[".b32"], format!(".{other}"))),
        }
    }
}

impl PtxParser for crate::r#type::instruction::lop3::Destination {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if stream
            .consume_if(|token| matches!(token, PtxToken::Identifier(name) if name == "_"))
            .is_some()
        {
            return Ok(Destination::Sink);
        }

        if stream.check(|token| matches!(token, PtxToken::Register(_) | PtxToken::LBrace)) {
            return Ok(Destination::Register(RegisterOperand::parse(stream)?));
        }

        let (token, span) = stream.peek()?;
        Err(unexpected_value(
            span.clone(),
            &["register operand", "_"],
            format!("{token:?}"),
        ))
    }
}

impl PtxParser for crate::r#type::instruction::lop3::Lop3 {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "lop3")?;

        if is_bool_op_modifier(stream) {
            let operator = BoolOp::parse(stream)?;
            let data_type = crate::r#type::instruction::lop3::DataType::parse(stream)?;
            let destination = Destination::parse(stream)?;
            stream.expect(&PtxToken::Pipe)?;
            let predicate = PredicateRegister::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let c = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let lut = Immediate::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let predicate_input = PredicateRegister::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;

            Ok(Lop3::Boolean(Boolean {
                operator,
                data_type,
                destination,
                predicate,
                a,
                b,
                c,
                lut,
                predicate_input,
            }))
        } else {
            let data_type = crate::r#type::instruction::lop3::DataType::parse(stream)?;
            let destination = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let c = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let lut = Immediate::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;

            Ok(Lop3::Plain(Plain {
                data_type,
                destination,
                a,
                b,
                c,
                lut,
            }))
        }
    }
}

fn is_bool_op_modifier(stream: &mut PtxTokenStream) -> bool {
    stream.check(|token| {
        matches!(
            token,
            PtxToken::Directive(name) if matches!(name.as_str(), "or" | "and")
        )
    })
}
