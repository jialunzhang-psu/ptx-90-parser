use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::{PredicateRegister, RegisterOperand},
        instruction::setp::{
            BoolOp, Compare, CompareBool, CompareOp, DataType, Destination, Predicate,
            PredicateTarget, Setp,
        },
    },
};

impl PtxParser for CompareOp {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "eq" => Ok(CompareOp::Eq),
            "ne" => Ok(CompareOp::Ne),
            "lt" => Ok(CompareOp::Lt),
            "le" => Ok(CompareOp::Le),
            "gt" => Ok(CompareOp::Gt),
            "ge" => Ok(CompareOp::Ge),
            "lo" => Ok(CompareOp::Lo),
            "ls" => Ok(CompareOp::Ls),
            "hi" => Ok(CompareOp::Hi),
            "hs" => Ok(CompareOp::Hs),
            "equ" => Ok(CompareOp::Equ),
            "neu" => Ok(CompareOp::Neu),
            "ltu" => Ok(CompareOp::Ltu),
            "leu" => Ok(CompareOp::Leu),
            "gtu" => Ok(CompareOp::Gtu),
            "geu" => Ok(CompareOp::Geu),
            "num" => Ok(CompareOp::Num),
            "nan" => Ok(CompareOp::Nan),
            other => Err(unexpected_value(
                span,
                &[
                    ".eq", ".ne", ".lt", ".le", ".gt", ".ge", ".lo", ".ls", ".hi", ".hs", ".equ",
                    ".neu", ".ltu", ".leu", ".gtu", ".geu", ".num", ".nan",
                ],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for BoolOp {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "and" => Ok(BoolOp::And),
            "or" => Ok(BoolOp::Or),
            "xor" => Ok(BoolOp::Xor),
            other => Err(unexpected_value(
                span,
                &[".and", ".or", ".xor"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "b16" => Ok(DataType::B16),
            "b32" => Ok(DataType::B32),
            "b64" => Ok(DataType::B64),
            "u16" => Ok(DataType::U16),
            "u32" => Ok(DataType::U32),
            "u64" => Ok(DataType::U64),
            "s16" => Ok(DataType::S16),
            "s32" => Ok(DataType::S32),
            "s64" => Ok(DataType::S64),
            "f32" => Ok(DataType::F32),
            "f64" => Ok(DataType::F64),
            other => Err(unexpected_value(
                span,
                &[
                    ".b16", ".b32", ".b64", ".u16", ".u32", ".u64", ".s16", ".s32", ".s64", ".f32",
                    ".f64",
                ],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for PredicateTarget {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if stream
            .consume_if(|token| matches!(token, PtxToken::Identifier(name) if name == "_"))
            .is_some()
        {
            return Ok(PredicateTarget::Sink);
        }

        Ok(PredicateTarget::Register(PredicateRegister::parse(stream)?))
    }
}

impl PtxParser for Destination {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let predicate = PredicateTarget::parse(stream)?;
        let complement = if stream
            .consume_if(|token| matches!(token, PtxToken::Pipe))
            .is_some()
        {
            Some(PredicateTarget::parse(stream)?)
        } else {
            None
        };

        Ok(Destination {
            predicate,
            complement,
        })
    }
}

impl PtxParser for Predicate {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let negated = stream
            .consume_if(|token| matches!(token, PtxToken::Exclaim))
            .is_some();
        let register = PredicateRegister::parse(stream)?;

        Ok(Predicate { register, negated })
    }
}

impl PtxParser for Compare {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let compare_op = CompareOp::parse(stream)?;
        let flush_to_zero = stream
            .consume_if(|token| matches!(token, PtxToken::Directive(name) if name == "ftz"))
            .is_some();
        let data_type = DataType::parse(stream)?;
        let destination = Destination::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Compare {
            compare_op,
            flush_to_zero,
            data_type,
            destination,
            a,
            b,
        })
    }
}

impl PtxParser for CompareBool {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let compare_op = CompareOp::parse(stream)?;
        let bool_op = BoolOp::parse(stream)?;
        let flush_to_zero = stream
            .consume_if(|token| matches!(token, PtxToken::Directive(name) if name == "ftz"))
            .is_some();
        let data_type = DataType::parse(stream)?;
        let destination = Destination::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let predicate = Predicate::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(CompareBool {
            compare_op,
            bool_op,
            flush_to_zero,
            data_type,
            destination,
            a,
            b,
            predicate,
        })
    }
}

impl PtxParser for Setp {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "setp" {
            return Err(unexpected_value(span, &["setp"], opcode));
        }

        let position = stream.position();
        match CompareBool::parse(stream) {
            Ok(compare_bool) => Ok(Setp::CompareBool(compare_bool)),
            Err(_) => {
                stream.set_position(position);
                match Compare::parse(stream) {
                    Ok(compare) => Ok(Setp::Compare(compare)),
                    Err(compare_err) => Err(compare_err),
                }
            }
        }
    }
}
