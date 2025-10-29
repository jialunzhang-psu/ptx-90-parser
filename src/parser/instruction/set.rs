use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::set::*},
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

impl PtxParser for DestinationType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "u32" => Ok(DestinationType::U32),
            "s32" => Ok(DestinationType::S32),
            "f32" => Ok(DestinationType::F32),
            other => Err(unexpected_value(
                span,
                &[".u32", ".s32", ".f32"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for SourceType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "b16" => Ok(SourceType::B16),
            "b32" => Ok(SourceType::B32),
            "b64" => Ok(SourceType::B64),
            "u16" => Ok(SourceType::U16),
            "u32" => Ok(SourceType::U32),
            "u64" => Ok(SourceType::U64),
            "s16" => Ok(SourceType::S16),
            "s32" => Ok(SourceType::S32),
            "s64" => Ok(SourceType::S64),
            "f32" => Ok(SourceType::F32),
            "f64" => Ok(SourceType::F64),
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
        let destination_type = DestinationType::parse(stream)?;
        let source_type = SourceType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Compare {
            compare_op,
            flush_to_zero,
            destination_type,
            source_type,
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
        let destination_type = DestinationType::parse(stream)?;
        let source_type = SourceType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
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
            destination_type,
            source_type,
            destination,
            a,
            b,
            predicate,
        })
    }
}

impl PtxParser for Set {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "set" {
            return Err(unexpected_value(span, &["set"], opcode));
        }

        let position = stream.position();
        match CompareBool::parse(stream) {
            Ok(compare_bool) => Ok(Set::CompareBool(compare_bool)),
            Err(_) => {
                stream.set_position(position);
                let compare = Compare::parse(stream)?;
                Ok(Set::Compare(compare))
            }
        }
    }
}
